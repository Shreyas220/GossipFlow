use crate::models::{NodeInfo, ValueVersion, Message};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

pub struct AppState {
    pub store: HashMap<String,ValueVersion>,
    pub self_node: NodeInfo,
    pub node_members: Vec<NodeInfo>,
    pub seen_updates: HashSet<Uuid>,
    pub wal_path: String,

}

// handling update to KV store data

// handle update to members 

