use std::net::{TcpListener, TcpStream};
use std::{thread, sync::Arc};
use crossbeam::atomic::AtomicCell;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::fs;

//got TCP Listener framewrok from documentation
fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:8888")?;
    //keep track of request counters here
    let req_counter = Arc::new(Mutex::new(0));
    let valid_counter = Arc::new(Mutex::new(0));

    // accept connections and process them serially
    for stream in listener.incoming() {
        let req_counter = Arc::clone(&req_counter); //clones go outside thread
        let valid_counter = Arc::clone(&valid_counter); 
        handle_client(stream?, req_counter, valid_counter);
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream, req_counter: Arc<Mutex<i32>>, valid_counter: Arc<Mutex<i32>>) {
    //println!("{}", stream);
    thread::spawn(move||{
        let mut req_num = req_counter.lock().unwrap(); //lock unwraps go inside thread
        *req_num += 1;
        let mut valid_num = valid_counter.lock().unwrap();
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
                //need to find num bytes actually read instead of len
                bytes_counted = buf.len();
                end_char = true;
            }

        }
        let req_file = get_req_file(client_msg.to_owned());
        let result = return_message(req_file);
        if result != "HTTP/1.1 404 Not Found".to_string(){
            //let mut valid_num = valid_counter.lock().unwrap();
            *valid_num += 1;

        }
        //println!("Client IP address: {:?}", stream.peer_addr().unwrap());  
        //println!("Bytes read from client {}", &bytes_counted);      
        //println!("{}", &client_msg[0..64]);
        println!("{}", result);
        println!("{}", req_num);
        println!("{}", valid_num);
        //spit back the file we need to send back to client here
        return result
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
    //println!("{}",path.display());
    if path.exists(){
        //need to return the index.html file from the path directory here
        //not just the path, shouldn't the request be an index.html file though?
        if path.is_dir(){
            //find index.html file
            //got code from the read_dir() rust documentation
            for entry in path.read_dir().expect("read_dir call failed") {
                if let Ok(entry) = entry {
                    println!("{:?}", entry.path());
                    if entry.path().ends_with("index.html"){
                        requested = entry.path().display().to_string();
                    }
                }
            }
        }
        else{
            //find index.html
            if path.ends_with("index.html"){
                requested = path.display().to_string();
            }
        }
    }else{
        requested = "404".to_string();
    }
    return requested;
}

fn return_message(req: String) -> String {
    //check to see if is valid file 
    //set result to error message
    //Ferrer said to not worry about validation but to check for 404 then move on 
    let mut result = " ".to_string();
    /* 
    let mut result = format!("<html>
        <body>
            <h1>Message received</h1>
            Requested file: {req} <br>
        </body>
    </html>");
    */
    if req == "404"{
        result = "HTTP/1.1 404 Not Found".to_string();
    }else{
        let p = PathBuf::from(req);
        let path = p.as_path();
        let result = path.display().to_string();     
    }
    return result.to_string();
}