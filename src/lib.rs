pub mod http_request {
    use std::collections::HashMap;
    #[derive(Debug, PartialEq)]
    pub enum HttpMethod {
        GET,
        POST,
        PUT,
        DELETE,
        PATCH,
        TRACE,
        OPTIONS,
        CONNECT,
    }
    #[derive(Debug, PartialEq)]
    pub struct HttpRequest {
        method: HttpMethod,
        route: String,
        options: HashMap<String, String>,
    }
    impl HttpRequest {
        pub fn create(method: HttpMethod, route: String, options: HashMap<String, String>) -> Self {
            Self {
                method,
                route,
                options,
            }
        }
        pub fn get_method(&self) -> &HttpMethod {
            &self.method
        }
        pub fn get_route(&self) -> &String {
            &self.route
        }
        pub fn get_options(&self) -> &HashMap<String, String> {
            &self.options
        }
    }
}
pub mod parser {
    use crate::http_request::{HttpMethod, HttpRequest};
    use std::collections::HashMap;
    pub fn parse_http_request(buffer: &[u8]) -> Option<HttpRequest> {
        let buffer: Vec<u8> = buffer.to_vec();
        let request = match String::from_utf8(buffer) {
            Ok(request) => request,
            Err(_) => return None,
        };
        let method = match request.split_whitespace().next() {
            Some("GET") => HttpMethod::GET,
            Some("POST") => HttpMethod::POST,
            Some("PUT") => HttpMethod::PUT,
            Some("DELETE") => HttpMethod::DELETE,
            Some("PATCH") => HttpMethod::PATCH,
            Some("TRACE") => HttpMethod::TRACE,
            Some("OPTIONS") => HttpMethod::OPTIONS,
            Some("CONNECT") => HttpMethod::CONNECT,
            _ => return None,
        };
        let route = request.split_whitespace().nth(1).unwrap();
        let options = parse_options(request.clone());
        Some(HttpRequest::create(method, route.to_string(), options))
    }
    fn parse_options(request: String) -> HashMap<String, String> {
        let mut options: HashMap<String, String> = HashMap::new();
        for line in request.lines().skip(1) {
            let mut parts = line.split(": ");
            let key = match parts.next() {
                Some(key) => key,
                None => continue,
            };
            println!("{key}");
            let value = match parts.next() {
                Some(value) => value,
                None => continue,
            };
            println!("{value}");
            options.insert(key.to_string(), value.to_string());
        }
        options
    }
    #[test]
    fn test() {
        let request = b" GET / HTTP/1.1\nHost: 127.0.0.1:8080\nConnection: keep-alive\nCache-Control: max-age=0\nsec-ch-ua: \"Not)A;Brand\";v=\"99\", \"Brav\";v=\"127\",\"Chromium\";v=\"127\"\nsec-ch-ua-mobile: ?0\nsec-ch-ua-platform: \"Linux\"\nDNT: 1\nUpgrade-Insecure-Requests: 1\nUser-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36\nAccept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8\nSec-GPC: 1\nAccept-Language: pl-PL,pl\nSec-Fetch-Site: cross-site\nSec-Fetch-Mode: navigate\nSec-Fetch-User: ?1\nSec-Fetch-Dest: document\nAccept-Encoding: gzip, deflate, br, zstd";
        let parsed_request = parse_http_request(request).unwrap();
        assert_eq!(parsed_request.get_method(), &HttpMethod::GET);
        assert_eq!(parsed_request.get_route(), &"/".to_string());
    }
    #[test]
    fn test_option_parser() {
        let request = " GET / HTTP/1.1\nHost: 127.0.0.1:8080\nConnection: keep-alive\nCache-Control: max-age=0\nsec-ch-ua: \"Not)A;Brand\";v=\"99\", \"Brav\";v=\"127\",\"Chromium\";v=\"127\"\nsec-ch-ua-mobile: ?0\nsec-ch-ua-platform: \"Linux\"\nDNT: 1\nUpgrade-Insecure-Requests: 1\nUser-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36\nAccept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8\nSec-GPC: 1\nAccept-Language: pl-PL,pl\nSec-Fetch-Site: cross-site\nSec-Fetch-Mode: navigate\nSec-Fetch-User: ?1\nSec-Fetch-Dest: document\nAccept-Encoding: gzip, deflate, br, zstd".to_string();
        let parsed_options: HashMap<String, String> = parse_options(request);
        let mut options = HashMap::new();
        options.insert("Host".to_string(), "127.0.0.1:8080".to_string());
        assert_eq!(
            parsed_options.get_key_value("Host"),
            parsed_options.get_key_value("Host")
        )
    }
}
pub mod server {
    use crate::parser;
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream};
    use std::thread;
    pub struct HttpServer {
        socket: TcpListener,
    }
    impl HttpServer {
        pub fn new(port: u16) -> Self {
            let address = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port);
            Self {
                socket: match TcpListener::bind(address) {
                    Ok(listener) => listener,
                    Err(e) => {
                        println!("Failed to bind to port: {}", e);
                        let s =
                            TcpListener::bind(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0))
                                .unwrap();
                        println!("bind to port: {}", s.local_addr().unwrap().port());
                        s
                    }
                },
            }
        }
        pub fn run(&self) -> std::io::Result<()> {
            for stream in self.socket.incoming() {
                let stream = match stream {
                    Ok(stream) => stream,
                    Err(e) => {
                        println!("Failed to establish connection: {}", e);
                        break;
                    }
                };
                thread::spawn(move || {
                    println!("Connection established");
                    let mut buffer = [0u8; 1024];
                    println!("Connection established");
                    stream.peek(&mut buffer).unwrap();
                    let request = match parser::parse_http_request(&buffer) {
                        Some(request) => request,
                        None => panic!("Failed to parse request"),
                    };
                    let mut x: u128 = 0;
                    for i in 0..500_000_000_000 {
                        x += i;
                    }
                    println!("{:?}\n\r {x}", request);
                });
            }
            Ok(())
        }

        pub fn accept(&self) -> Option<(TcpStream, SocketAddr)> {
            match self.socket.accept() {
                Ok((stream, accept)) => Some((stream, accept)),
                Err(_) => None,
            }
        }
        pub fn local_addr(&self) -> SocketAddr {
            self.socket.local_addr().unwrap()
        }
    }
    #[test]
    fn when_called_will_create_listener_even_if_port_taken() {
        let _server1: HttpServer = HttpServer::new(8080);
        let _server2: HttpServer = HttpServer::new(8080);
    }
    #[test]
    fn when_proper_listener_is_created_should_accept_connection() {
        use std::net::TcpStream;
        use std::thread;
        use std::time::Duration;
        let server: HttpServer = HttpServer::new(23000);
        let port = server.socket.local_addr().unwrap().port();
        let address = format!("127.0.0.1:{}", port);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            let connection = TcpStream::connect(address).expect("Failed to connect");
            assert_eq!(
                connection.peer_addr().unwrap(),
                SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port))
            );
        });
        let address = format!("127.0.0.1:{}", port);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            let connection = TcpStream::connect(address).expect("Failed to connect");
            assert_eq!(
                connection.peer_addr().unwrap(),
                SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port))
            );
        });

        let _accept = server.accept().expect("Failed to accept connection");
    }
}
