use std::net::{TcpListener, TcpStream};
use std::{thread, sync::Arc};
use crossbeam::atomic::AtomicCell;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;

//got TCP Listener framewrok from documentation
fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:8888")?;
    //keep track of request counters here
    //otherwise no way to update them based on the current code base structure
    let req_counter = Arc::new(Mutex::new(0));
    let valid_counter = Arc::new(Mutex::new(0));

    // accept connections and process them serially
    for stream in listener.incoming() {
        let req_counter = Arc::clone(&req_counter); //goes outside thread
        handle_client(stream?);
        
        let mut req_num = counter.lock().unwrap(); //inside thread
        *req_num += 1;                              //inside thread 
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
            //need 500 byte limiter
            let mut buf = [0; 500];
            let _msg = stream.read(&mut buf);
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
        //spit back the file we need to send back to client here
        //return result
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
    //file validation here
    
    let p = PathBuf::from(format!("{requested}"));
    let path = p.as_path(); 
    //assert_eq!(Path::new("/test"), p.as_path());
    if path.is_file(){
        return requested; 
    }else{
        requested = "404".to_string();
    }
    
    return requested;
}

fn return_message(req: String) -> String {
    //check to see if is valid file 
    //set result to error message
    //Ferrer said to not worry about validation but to check for 404 then move on 
    let mut result = format!("<html>
        <body>
            <h1>Message received</h1>
            Requested file: {req} <br>
        </body>
    </html>");
    if req == "404"{
        result = "404 error message".to_string();
    }
    return result.to_string();
}