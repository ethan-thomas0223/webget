use openssl::ssl::{SslConnector, SslMethod};
use std::io;
use std::io::Write; 
use std::io::BufReader; 
use std::io::BufRead;
use std::net::TcpStream; 
use std::fs::File;
use std::ffi::OsStr;
//Usage: webget url
fn main() {
    let url: String = std::env::args().last().unwrap();
    //got all my basic variables; Don't know what's next    
    //parse to get message parts; skip over the https:// in url   
    let parts: Vec<&str> = url.split("/").collect();
    //println!("{}", parts.last());
    let host: String = parts[2].to_string();
    let mut req = "".to_string();
    for entry in parts.iter().skip(3) {
        req += &("/".to_string() + entry); 
    }
    //println!("{}", host);
    //println!("{}", req);
    let get = format!("GET {req} HTTP/1.1\r\nHost: {host} \r\nConnection: Close \r\n\r\n"); 
    println!("{}", get);
    send_message(host.as_str(), 443, &get).unwrap(); 
}

fn send_message(host: &str, port: usize, message: &str) -> io::Result<()> {
    //println!("here");
    let get_msg_vec: Vec<&str> = message.split_whitespace().collect();
    //println!("{}", file_req[1]);
    let file_req = get_msg_vec[1].to_owned();
    let tcp = TcpStream::connect(format!("{}:{}", host, port))?;
    let connector = SslConnector::builder(SslMethod::tls())?.build();
    let mut stream = connector.connect(host, tcp).unwrap();
    stream.write(message.as_bytes())?;
    //create buff reader on the stream     
    let buf = BufReader::new(stream);
    let mut newmsg = "".to_string();
    //iterate through buf reader using lines    
    //add to sequence of string the we are getting back (print to clarify)   
    //break up string to get header, shave it off, save the rest to local file
    let mut lcount = 0;
    for line in buf.lines(){
        //println!("here");
        //println!("{}", line.unwrap());
        newmsg += &(line.unwrap() + "\n");
        if (newmsg.ends_with("\n\n") || newmsg.ends_with("\r\n\r\n")) && lcount < 8{  
            //println!("here");
            //newmsg += "here";
            newmsg = "".to_string();
        }
        //newmsg.push_str("{}", line);
        // newmsg += &("/".to_string() + &line);
        lcount += 1;
    }
    
    //write to file once message received
    println!("{}", &file_req);
    let f = File::create(format!("{file_req}")); 
    //println!("{}", newmsg); 
    f?.write(newmsg.as_bytes())?;
    Ok(())
}

//test comment