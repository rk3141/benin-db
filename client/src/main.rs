use std::{
    io::{stdin, stdout, Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    // TcpStream::connect("127.0.0.1:5050").expect("Couldnt Connect");

    // let mut buff = String::new();

    // print!("Enter your message: ");
    // stdout().flush();

    // stdin().read_line(&mut buff);
    // buff = buff.trim().to_string();

    // println!("{:?}", buff);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 2;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
