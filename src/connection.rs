use std::{io::Read, net::TcpListener};

pub struct ConnectionHandle {
    pub port: u16,
    pub host: String,
}

impl ConnectionHandle {
    pub fn start(self) {
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port))
            .expect("Couldnt bind to address");

        for req in listener.incoming() {
            if let Ok(mut req) = req {
                let mut message = String::new();
                req.read_to_string(&mut message).expect("read err");
                println!("Message: {}", message);

                let params = message.split(' ').collect::<Vec<&str>>();

                if let Some(command) = params.first() {
                    match command {
                        "set" => {}
                        _ => {}
                    }
                }
            }
        }
    }
}
