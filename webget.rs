use openssl::ssl::{SslConnector, SslMethod};
use std::io;
use std::ffi::CString;
use std::env;
use std::io::Write; 
use std::io::BufReader; 
use std::io::BufRead;
use std::net::TcpStream; 
//Usage: webget url
fn main() {
    let mut url: String = std::env::args().last().unwrap();
    //got all my basic variables; Don't know what's next    
    //parse to get message parts; skip over the https:// in url   
    let mut parts: Vec<&str> = url.split("/").collect();
    //println!("{}", parts.last());
    let mut host: String = parts[2].to_string();
    let mut req = "".to_string();
    for entry in parts.iter().skip(3) {
        req += &("/".to_string() + entry); 
    }
    println!("{}", host);
    println!("{}", req);
    send_message(host.as_str(), 443, &req); 
    //execvp(&cstring_cmd[0], &cstring_cmd);
}

fn send_message(host: &str, port: usize, message: &str) -> io::Result<()> {
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
    for line in buf.lines(){
        //println!("{}", line.unwrap());
        newmsg += &(line.unwrap() + "\n");
        //newmsg.push_str("{}", line);
        // newmsg += &("/".to_string() + &line);
    }
    //write to file once message received
    
    let flags: OFlag = [OFlag::O_CREAT, OFlag::O_WRONLY, OFlag::O_TRUNC].iter().copied().collect();
    let mode: Mode = [Mode::S_IRUSR, Mode::S_IWUSR].iter().copied().collect();
    let file_out = open("tar.rs", flags, mode)?;
    dup2(file_out, 1)?;
    println!("{}", newmsg); 
    Ok(())
}

//test comment