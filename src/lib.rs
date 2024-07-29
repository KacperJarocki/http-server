use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream};

pub struct HttpServer {
    socket: TcpListener,
}
impl HttpServer {
    pub fn run(port: u16) -> Self {
        let address = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port);
        Self {
            socket: match TcpListener::bind(address) {
                Ok(listener) => listener,
                Err(e) => {
                    println!("Failed to bind to port: {}", e);
                    let s = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0))
                        .unwrap();
                    println!("bind to port: {}", s.local_addr().unwrap().port());
                    s
                }
            },
        }
    }
    pub fn accept(&self) -> Option<(TcpStream, SocketAddr)> {
        match self.socket.accept() {
            Ok((socket, accept)) => Some((socket, accept)),
            Err(_) => None,
        }
    }
    pub fn local_addr(&self) -> SocketAddr {
        self.socket.local_addr().unwrap()
    }
}
#[test]
fn when_called_will_create_listener_even_if_port_taken() {
    let _server1: HttpServer = HttpServer::run(8080);
    let _server2: HttpServer = HttpServer::run(8080);
}
#[test]
fn when_proper_listener_is_created_should_accept_connection() {
    use std::net::TcpStream;
    use std::thread;
    use std::time::Duration;
    let server: HttpServer = HttpServer::run(23000);
    let port = server.socket.local_addr().unwrap().port();
    let address = format!("127.0.0.1:{}", port);
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        let connection = TcpStream::connect(address).expect("Failed to connect");
        assert_eq!(
            connection.peer_addr().unwrap(),
            SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080))
        );
    });
    let address = format!("127.0.0.1:{}", port);
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        let connection = TcpStream::connect(address).expect("Failed to connect");
        assert_eq!(
            connection.peer_addr().unwrap(),
            SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080))
        );
    });

    let _accept = server.accept().expect("Failed to accept connection");
}
