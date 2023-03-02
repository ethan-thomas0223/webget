use std::net::{TcpListener, TcpStream};
use std::{thread, sync::Arc};
use crossbeam::atomic::AtomicCell;
use std::io::BufReader;
use std::io::Read;

//got TCP Listener framewrok from documentation
fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:8888")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    //println!("{}", stream);
    thread::spawn(move||{
        let mut end_char = false;
        let mut client_msg = "".to_string();
        while !end_char{
            //let buf = BufReader::new(stream);
            //need 500 byte limiter
            let mut buf = [0; 500];
            let msg = stream.read(&mut buf);
            let from_bytes = std::str::from_utf8(&buf).unwrap();
            client_msg.push_str(from_bytes);
            
            if client_msg.contains("\r\n\r\n") || client_msg.contains("\n\n"){
                end_char = true;
            }
        }
        println!("Client IP address: {:?}", stream.peer_addr().unwrap());        
        println!("{}", client_msg);
    });
}