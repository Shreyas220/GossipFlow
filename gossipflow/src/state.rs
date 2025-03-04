use crate::models::{NodeInfo, ValueVersion, Message};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub store: HashMap<String,ValueVersion>,
    pub self_node: NodeInfo,
    pub node_members: Vec<NodeInfo>,
    pub seen_updates: HashSet<Uuid>,
    pub wal_path: String,

}

// handling update to KV store data
pub fn update_store(state: Arc<Mutex<AppState>>,incoming_store: HashMap<String, ValueVersion> ) {
    for (k,v) in incoming_store {
        let mut st = state.lock().unwrap();

        if st.seen_updates.contains(&v.uniquerid) {
            continue;
        }

        st.seen_updates.insert(v.uniquerid);

        // st.store.insert(k,v);
        match st.store.get_mut(&k) {
            Some(existing_value) => {
                // if the version is greater than the existing value, update the value else its an old update
                if existing_value.version < v.version {
                    st.store.insert(k,v);
                }
            }
            None => {
                st.store.insert(k,v);
            }
        }
    }
}

// handle update to members 
pub fn update_members(state: &mut AppState, new_members: Vec<NodeInfo>) {
    for member in new_members {
        if member.id == state.self_node.id {
            //its me 
            continue;
        }

        match state.node_members.iter_mut().find(|m| m.id == member.id) {
            Some(existing_member) => {
                if member.heartbeat > existing_member.heartbeat {
                    existing_member.heartbeat = member.heartbeat;
                }
            }
            None => {
                state.node_members.push(member);
            }
        }
    }
}

