mod models;
mod state;
mod worker;
use crate::models::{NodeInfo, Message};
use crate::state::AppState;
use crate::worker::receive_gossip;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use std::net::SocketAddr;
use tokio::net::UdpSocket;


#[tokio::main]
async fn main() -> std::io::Result<()>  {
    println!("Hello, world!");
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <bind_address> [seed_address...]", args[0]);
        std::process::exit(1);
    }
    
    // let binder_addr = Box::new(args[1].to_string());
    // let seed_nodes =Box::new(args[2..].to_vec());
    let bind_addr = args[1].clone();
    let seed_nodes = &args[2..];
    
    let socket = Arc::new(UdpSocket::bind(&bind_addr).await.expect("Failed to bind UDP socket"));    println!("Node listening on {}", bind_addr);
    //me here represent this node 
    let me = NodeInfo{
        address: bind_addr,
        heartbeat: 0,
    };

    // create node state 
    let state = Arc::new(Mutex::new(AppState {
        store: HashMap::new(),
        self_node: me,
        node_members: Vec::new(),
        seen_updates: HashSet::new(),
        wal_path: "wal.json".to_string(),
    }));
    
    // handle incoming data from user and from more nodes 
    let (tx_message, rx_message) = mpsc::channel::<(Message, SocketAddr)>(100);    

    {
        let tx_message_clone = tx_message.clone();
        let socket_clone = Arc::clone(&socket);        
        tokio::spawn(async move {
            receive_gossip(socket_clone, tx_message_clone);   
        });
    }


    Ok(())
}
