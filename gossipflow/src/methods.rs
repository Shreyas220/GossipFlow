use crate::models::{NodeInfo, ValueVersion, Message};
use crate::state::AppState;
use uuid::Uuid;

// get 
pub fn get(key: &str, state: &AppState) -> Option<String> {
    state.store.get(key).map(|v| v.value.clone())
}

// set 
pub fn set(key: &str, value: &str, state: &mut AppState) {
    state.store.insert(key.to_string(), ValueVersion { value: value.to_string(), uniquerid: Uuid::new_v4(), version: 0 });
}

// delete 
pub fn delete(key: &str, state: &mut AppState) {
    state.store.remove(key);
}

