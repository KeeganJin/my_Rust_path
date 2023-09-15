use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};




fn main() {

    // closure_test();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();


    for stream in listener.incoming(){
        let stream = stream.unwrap();

        //func handle connection to handle stream 
        handle_connection(stream);
        
    }
}




fn handle_connection(mut stream:TcpStream){

    // buf is used to achieve buffer read
    let buf_reader = BufReader::new(&mut stream);
    
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // setup http request 
    // let http_request:Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    if request_line =="GET / HTTP/1.1"{
        
        let status_line = "HTTP/1.1 200 OK";

        let contents = fs::read_to_string("hello.html").unwrap();

//  get content length
        let length = contents.len();

        let response = 
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    } 
    else {


        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response = 
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");    
        stream.write_all(response.as_bytes()).unwrap();
    }
    
}

