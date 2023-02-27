use std::net::{TcpListener, TcpStream};
use std::ffi::CString;
use std::env;
use std::io; 
use nix::{sys::wait::waitpid,unistd::{fork, ForkResult, execvp}};
use check::*;

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
    println!("{}", stream);
}