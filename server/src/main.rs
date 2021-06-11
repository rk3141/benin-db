use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

mod connection;

fn main() {
    let kv_map: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    connection::ConnectionHandle {
        host: "127.0.0.1".to_string(),
        port: 5050,
    }
    .start(kv_map);
}
