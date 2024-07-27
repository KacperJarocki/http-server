use std::net::TcpListener;

struct HttpServer {
    port: u128,
    socket: TcpListener,
}
impl HttpServer {
    pub fn new(port: u128) -> Self {
        Self {
            port,
            socket: TcpListener::bind(),
        }
    }
    pub fn get_port(self) -> u128 {
        self.port
    }
}
#[test]
fn test() {
    let server: HttpServer = HttpServer::new(8080);
    assert_eq!(server.get_port(), 8080)
}
