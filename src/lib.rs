use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

struct HttpServer {
    socket: TcpListener,
}
impl HttpServer {
    pub fn new(port: u16) -> Self {
        let address = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port);
        Self {
            socket: match TcpListener::bind(address) {
                Ok(listener) => listener,
                Err(e) => panic!("Failed to bind to port: {}", e),
            },
        }
    }
}
#[test]
fn when_port_is_free_should_create_proper_listener() {
    let server: HttpServer = HttpServer::new(8080);
    assert_eq!(server.socket.local_addr().unwrap().port(), 8080);
}
#[test]
#[should_panic]
fn when_port_is_taken_creating_should_panic() {
    let _server1: HttpServer = HttpServer::new(8080);
    let _server2: HttpServer = HttpServer::new(8080);
}
