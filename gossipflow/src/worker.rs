use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use rand::seq::SliceRandom;
use tokio::net::UdpSocket;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::interval;

use crate::models::{Message, NodeInfo, ValueVersion};
use crate::state::{
    AppState, update_store, update_members, update_sender_heartbeat
};
// use crate::util::{send_message, current_unix_timestamp};
// use crate::state::{replay_wal, write_to_wal};


// receieve gossips
pub fn receive_gossip(
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

// send gossips

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

