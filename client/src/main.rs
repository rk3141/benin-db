use std::{
    io::{stdin, stdout, Read, Write},
    net::{Shutdown, TcpStream},
};

fn prompt(prompt: &str) -> String {
    let mut buff = String::new();

    print!("{}", prompt);
    stdout().flush().unwrap();

    stdin().read_line(&mut buff).unwrap();
    buff = buff.trim().to_string();

    buff
}

fn main() {
    while let input = prompt("> ") {
        if input == "exit" {
            break;
        }
        if input == "" {
            continue;
        }
        let mut connection = TcpStream::connect("127.0.0.1:5050").expect("Couldnt Connect");
        connection.write(input.as_bytes()).unwrap();
        connection.shutdown(Shutdown::Both);
    }
}
