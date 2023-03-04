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
        let mut bytes_counted = 0;
        while !end_char{
            //let buf = BufReader::new(stream);
            //need 500 byte limiter
            let mut buf = [0; 500];
            let msg = stream.read(&mut buf);
            let from_bytes = std::str::from_utf8(&buf).unwrap();
            client_msg.push_str(from_bytes);

            if client_msg.contains("\r\n\r\n") || client_msg.contains("\n\n"){
                //need to find num bytes actually read
                bytes_counted = buf.len();
                end_char = true;
            }

        }
        let req_file = get_req_file(client_msg.to_owned());
        let result = return_message(req_file);
        
        println!("Client IP address: {:?}", stream.peer_addr().unwrap());  
        println!("Bytes read from client {}", &bytes_counted);      
        println!("{}", &client_msg[0..64]);
        println!("{}", result);
    });
}

fn get_req_file(message: String) -> String {
    let mut requested = "".to_string();
    let mut counter = 0;
    for line in message.split_whitespace(){
        if line.contains("/") && counter < 2 {
            //println!("{}", line);
            requested = line.to_string();
        }
        counter += 1;
    }
    
    return requested;
    
}

fn return_message(req: String) -> String {
    let mut result = format!("<html>
        <body>
            <h1>Message received</h1>
            Requested file: {req} <br>
        </body>
    </html>");
    return result.to_string();
}