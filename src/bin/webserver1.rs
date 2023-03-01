use std::net::{TcpListener, TcpStream};
use std::ffi::CString;
use std::env;
use std::io; 
use nix::{sys::wait::waitpid,unistd::{fork, ForkResult, execvp}};
use check::*;
use std::{thread, sync::Arc};
use crossbeam::atomic::AtomicCell;


//got TCP Listener framewrok from documentation
fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:8888")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

fn handle_client(stream: TcpStream) {
    //println!("{}", stream);
    thread::spawn(move||{
        let mut client_msg: &str = "".to_string();
        let mut end_char = false;
        while !end_char{
            let buf = BufReader::new(stream);
            //need 500 byte limiter
            let from_bytes = str::from_utf8(&buf).unwrap();
            let msg = client_msg.read(from_bytes);
            client_msg.push_str(msg);
            if client_msg.contains("\r\n\r\n") or client_msg.contains("\n\n"){
                end_char = true;
            }
        }
        println!("{}", client_msg);
    })
}