use std::{
    collections::HashMap,
    io::Read,
    net::{TcpListener, TcpStream},
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

// type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ConnectionHandle {
    pub port: u16,
    pub host: String,
}

// struct ThreadPool {
//     workers: Vec<Worker>,
//     sender: mpsc::Sender<Job>,
// }

// struct Worker {
//     id: usize,
//     thread: thread::JoinHandle<()>,
// }

// impl Worker {
//     pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
//         let thread = thread::spawn(move || loop {
//             let job = receiver.lock().unwrap().recv().unwrap();

//             println!("Worker {} working", id);

//             job();
//         });

//         Self { id, thread }
//     }
// }

// impl ThreadPool {
//     pub fn new(size: usize) -> Self {
//         assert!(size > 0);

//         let (sender, receiver) = mpsc::channel();

//         let receiver = Arc::new(Mutex::new(receiver));

//         let mut workers = Vec::with_capacity(size);

//         for id in 0..size {
//             workers.push(Worker::new(id, Arc::clone(&receiver)));
//         }

//         Self { workers, sender }
//     }

//     pub fn execute<F>(&self, f: F)
//     where
//         F: FnOnce() + Send + 'static,
//     {
//         let job = Box::new(f);

//         self.sender.send(job).unwrap();
//     }
// }

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
