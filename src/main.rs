mod connection;

fn main() {
    connection::ConnectionHandle {
        host: "127.0.0.1".to_string(),
        port: 5050,
    }
    .start();
}
