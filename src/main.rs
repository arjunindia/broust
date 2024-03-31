use ::std::env;

use std::collections::HashMap;
use std::str::from_utf8_unchecked;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

use native_tls::TlsConnector;

pub struct URL {
    scheme: String,
    host: String,
    path: String,
    port: u16,
}

impl URL {
    pub fn new(url: &str) -> Self {
        let (scheme, url) = url.split_once("://").unwrap();
        let scheme = scheme.to_owned();
        assert!(scheme == "http" || scheme == "https");
        let mut url: String = url.to_string();
        if !url.contains("/") {
            url = url.to_string() + "/";
        }
        let (host, url) = url.split_once("/").unwrap();
        let host = host.to_owned();
        let path = "/".to_string() + url;
        let port = if scheme == "http" { 80 } else { 443 };
        let port = if host.contains(":") {
            host.split_once(":").unwrap().1.parse::<u16>().unwrap()
        } else {
            port
        };
        Self {
            scheme,
            host,
            path,
            port,
        }
    }

    pub fn request(&self) -> String {
        let socket = TcpStream::connect(format!("{}:{}", self.host, self.port));
        match socket {
            Ok(mut stream) => {
                if self.scheme == "https" {
                    let connector = TlsConnector::new().unwrap();
                    let mut stream = connector.connect(&self.host, stream).unwrap();
                    let request = format!(
                        "GET {} HTTP/1.0\r\nHost: {}\r\nAccept-Encoding: identity\r\n\r\n",
                        self.path, self.host
                    );
                    let request = request.as_bytes();
                    stream.write(request).unwrap();
                    let mut data: Vec<u8> = Vec::new();
                    let mut header_map: HashMap<String, String> = HashMap::new();
                    return match stream.read_to_end(&mut data) {
                        Ok(_) => {
                            let text = unsafe { from_utf8_unchecked(&data) };
                            let headers: Vec<&str> = text
                                .split("\r\n")
                                .collect::<Vec<&str>>()
                                .split_first()
                                .unwrap()
                                .1
                                .split(|v| v.eq(&""))
                                .next()
                                .unwrap()
                                .to_vec();
                            for header in headers {
                                let (k, v) = header.split_once(":").unwrap();
                                let k = k.trim().to_lowercase();
                                let v = v.trim().to_lowercase();
                                header_map.insert(k, v);
                            }
                            if header_map.contains_key("transfer-encoding")
                                || header_map.contains_key("content-encoding")
                            {
                                return "Error encoding not supported".to_owned();
                            }
                            let text = text.split_once("\r\n\r\n").unwrap().1;
                            text.to_owned()
                        }
                        Err(e) => {
                            eprintln!("Failed to recieve: {}", e);
                            "500 Server Error".to_owned()
                        }
                    };
                }
                let request = format!(
                    "GET {} HTTP/1.0\r\nHost: {}\r\nAccept-Encoding: identity\r\n\r\n",
                    self.path, self.host
                );
                let request = request.as_bytes();
                stream.write(request).unwrap();
                let mut data: Vec<u8> = Vec::new();
                let mut header_map: HashMap<String, String> = HashMap::new();
                match stream.read_to_end(&mut data) {
                    Ok(_) => {
                        let text = unsafe { from_utf8_unchecked(&data) };

                        let headers: Vec<&str> = text
                            .split("\r\n")
                            .collect::<Vec<&str>>()
                            .split_first()
                            .unwrap()
                            .1
                            .split(|v| v.eq(&""))
                            .next()
                            .unwrap()
                            .to_vec();
                        for header in headers {
                            let (k, v) = header.split_once(":").unwrap();
                            let k = k.trim().to_lowercase();
                            let v = v.trim().to_lowercase();
                            header_map.insert(k, v);
                        }
                        if header_map.contains_key("transfer-encoding")
                            || header_map.contains_key("content-encoding")
                        {
                            return "Error encoding not supported".to_owned();
                        }
                        let text = text.split_once("\r\n\r\n").unwrap().1;
                        text.to_owned()
                    }
                    Err(e) => {
                        eprintln!("Failed to recieve: {}", e);
                        "500 Server Error".to_owned()
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed: {}", e);
                "500 Server Error".to_owned()
            }
        }
    }
}

fn show_body(body: String) {
    let mut in_tag = false;
    for c in body.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            print!("{}", c)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Not enough arguments! add a `-- {{url}}` at the end of the CLI");
        return;
    }
    let url = URL::new(&args[1]);
    let response = url.request();
    show_body(response);
    println!("\n\nConnection Scheme: {}", url.scheme);
    println!("Connection Host: {}", url.host);
    println!("Connection Path: {}", url.path);
}
