use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::thread;

fn handle_client(mut stream: UnixStream) {
    println!("== client connected");

    let mut buf = String::new();
    let sock_copy = stream.try_clone().expect("Couldn't clone socket");
    let mut wrapped = BufReader::new(sock_copy);
    wrapped.read_line(&mut buf).unwrap();

    println!("debug: {:?}", buf.to_uppercase());

    stream
        .write_fmt(format_args!("{}", buf.to_uppercase()))
        .unwrap();
}

fn main() {
    let socket = Path::new("/tmp/rust_socket.tmp");

    // Delete socket if necessary
    if socket.exists() {
        fs::remove_file(&socket).unwrap();
    }

    let listener = UnixListener::bind(&socket).unwrap();

    println!("Server started, waiting for clients");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}
