use serde::{ Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct NodeInfo{
    pub id: Uuid,
    pub address: String,
    pub heartbeat: u64,
}

#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct ValueVersion{
    pub value: String,
    pub uniquerid: Uuid,
    pub version : u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Message {
    /// Full gossip state push: membership + store
    GossipState {
        membership: Vec<NodeInfo>,
        store: HashMap<String, ValueVersion>,
    },
    /// Request the remote node's current state
    GossipRequest,
}