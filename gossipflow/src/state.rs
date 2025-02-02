use crate::model::{NodeInfo, ValueVersion, Message};
use std::collections::{HashMap, HashSet};


pub struct AppState {
    pub store: Hashmap<String,ValueVersion>,
    pub self_node: NodeInfo,
    pub node_members: Vec<NodeInfo>,
    pub seen_updates: HashSet<Uuid>,
    pub wal_path: String,                    

}

// handling update to KV store data

// handle update to members 

