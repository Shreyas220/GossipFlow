use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use rand::seq::SliceRandom;
use tokio::net::UdpSocket;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::interval;

use crate::models::{Message, NodeInfo, ValueVersion};
use crate::state::{
    AppState, // merge_membership, merge_store, update_sender_heartbeat, prune_old_data,
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
                let mut st = state.lock().unwrap();

                // merge the member if new 

                // add to kv store

                //update_sender_heartbeat
            }
            
            Message::GossipRequest => {
                // wants our state

            }
        }


    }
}