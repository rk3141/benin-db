use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

mod connection;

fn main() {
    let DATA: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    connection::ConnectionHandle {
        host: "127.0.0.1".to_string(),
        port: 5050,
    }
    .start(DATA);
}
