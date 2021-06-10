use std::collections::HashMap;

mod connection;

fn main() {
    let mut kv_map = (HashMap::new());

    connection::ConnectionHandle {
        host: "127.0.0.1".to_string(),
        port: 5050,
    }
    .start(&mut kv_map);
}
