use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
fn cl_handle( mut stream:  TcpStream) {
     //this is a buffer that read data
    let mut buffer = [0; 1024];
    // this line read data from the link buffer
    stream.read( &mut buffer ).unwrap();
    //this line coverts the data in the buffer into a UTF- 8  enccoded string.
    let request = String::from_utf8_lossy( &buffer [..]);
    println!("Received request: {}", request);
    let response = "Hello , client".as_bytes();
    stream.write( response ).unwrap();

}
//ENTRY POINT
fn main() {
    let listener = TcpListener::bind("127.0.0.1:3030").unwrap();
    println!("Listening on 127.0.0.1:7878");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn( || cl_handle(stream));

            }
            Err(e)=> {
                eprintln!("Error: {}", e);
                // stderr - standard error stream
        }
        }

    }
}