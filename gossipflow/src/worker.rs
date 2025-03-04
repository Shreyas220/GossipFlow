use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use rand::seq::SliceRandom;
use tokio::net::UdpSocket;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::interval;

use crate::utils::{send_message, current_unix_timestamp};
use crate::models::{Message, NodeInfo, ValueVersion};
use crate::state::{
    AppState, update_store, update_members, update_sender_heartbeat
};
// use crate::util::{send_message, current_unix_timestamp};
// use crate::state::{replay_wal, write_to_wal};


// receieve gossips
pub async fn receive_gossip(
    socket: Arc<UdpSocket>,
    tx_message: Sender<(Message, SocketAddr)>,
) {
    let mut buf = vec![0; 1024];
    loop {
        match socket.try_recv_from(&mut buf) {

            Ok((size, addr)) => {
                let data = &buf[..size];
                if let Ok(msg) = serde_json::from_slice::<Message>(&data) {
                    // Remove await since we're in a non-async function
                    if let Err(e) = tx_message.try_send((msg,addr)) {
                        eprintln!("Error sending message: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                
            }
        }
    }
}

// handle add update delete message

pub async fn handle_message(
    mut rx_message: Receiver<(Message, SocketAddr)>,
    state: Arc<Mutex<AppState>>,
) {
    let socket = UdpSocket::bind("0.0.0.0:0").await.expect("Failed to bind to socket");

    while let Some((msg, addr)) = rx_message.recv().await {
        match msg {
            Message::GossipState {membership,store} => {
               
                {
                    let mut st = state.lock().unwrap();
                    // merge the member if new 
                    update_members(&mut st, membership);
                }

                {
                    let mut st = state.lock().unwrap();
                    // add to kv store
                    update_store(&mut st, store);
                }

                //update_sender_heartbeat
                {
                    let mut st = state.lock().unwrap();
                    update_sender_heartbeat(&mut st, &addr);
                }
            }
            
            Message::GossipRequest => {
                // TODO: Implement SWIM protocol
                let mut st = state.lock().unwrap();
                update_sender_heartbeat(&mut st, &addr);

            }
        }


    }
}

// send gossips

pub async fn send_gossip(socket: Arc<UdpSocket>, state: Arc<Mutex<AppState>>) {
    let mut ticker = interval(Duration::from_secs(5));

    loop {
        ticker.tick().await;

        let st = state.lock().unwrap();

        //picking up random node to gossip with
        let random_node = st.node_members.choose(&mut rand::thread_rng()).unwrap();

        //sending gossip state to the random node
        let message = Message::GossipState {
            membership: {
                let mut all: Vec<NodeInfo> = vec![st.self_node.clone()];
                all.extend(st.node_members.clone());
                all
            },
            store: st.store.clone(),
        };
    
        //TODO: implement ACK
        let _ = send_message(&socket, &message, &random_node.address).await;

        // there is really no use of this request, will use SWIM protocol here 
        let req = Message::GossipRequest;
        let _ = send_message(&socket, &req, &random_node.address).await;

    }
}