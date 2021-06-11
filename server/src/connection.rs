use std::{
    collections::HashMap,
    io::Read,
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

pub struct ConnectionHandle {
    pub port: u16,
    pub host: String,
}

impl ConnectionHandle {
    pub fn start(self, hashmap: Arc<Mutex<HashMap<String, String>>>) {
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port))
            .expect("Couldnt bind to address");

        for req in listener.incoming() {
            let hashmap = Arc::clone(&hashmap);
            thread::spawn(move || {
                let mut hashmap = hashmap.lock().unwrap();
                let mut req = req.unwrap();

                let mut message = String::new();

                req.read_to_string(&mut message).expect("read err");
                message = message.trim().to_string(); // remove \n chars

                println!("Message: {:?}", message);
                let params = message.split(' ').collect::<Vec<&str>>();

                if let Some((&command, rest)) = params.split_first() {
                    match command {
                        "set" => {
                            println!("Seting... {:?}", rest);

                            let mut set_params = rest.into_iter();

                            if let Some(key) = set_params.next() {
                                let mut value = String::new();

                                for val in set_params {
                                    value += (val.to_string() + " ").as_str();
                                }

                                value.pop();
                                (*hashmap).insert(key.to_string(), value.to_string());
                            }
                        }
                        _ => {}
                    }
                }
            });
        }
    }
}
