extern crate hyper;

use std::io::Read;

use hyper::Client;
use hyper::header::Connection;

mod config;

fn main() {
    // Create a client.
    let mut client = Client::new();

    // Creating an outgoing request.
    let mut res = client.get("http://www.google.com/")
        // set a header
        .header(Connection::close())
        // let 'er go!
        .send().unwrap();

    //assert_eq!(res.status, hyper::Ok);

    // Read the response.
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
     
    println!("Response: {}", body);

    //let config = include_str!("vastatrix.config");
    //println!("{}", config);
    //config::load();
}
