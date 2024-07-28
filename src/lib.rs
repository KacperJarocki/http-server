use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream};

struct HttpServer {
    socket: TcpListener,
}
impl HttpServer {
    pub fn run(port: u16) -> Self {
        let address = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port);
        Self {
            socket: match TcpListener::bind(address) {
                Ok(listener) => listener,
                Err(e) => panic!("Failed to bind to port: {}", e),
            },
        }
    }
    pub fn accept(&self) -> Option<(TcpStream, SocketAddr)> {
        match self.socket.accept() {
            Ok((socket, accept)) => Some((socket, accept)),
            Err(_) => None,
        }
    }
}
#[test]
fn when_port_is_free_should_create_proper_listener() {
    let server: HttpServer = HttpServer::run(8080);
    assert_eq!(server.socket.local_addr().unwrap().port(), 8080);
}
#[test]
#[should_panic]
fn when_port_is_taken_creating_should_panic() {
    let _server1: HttpServer = HttpServer::run(8080);
    let _server2: HttpServer = HttpServer::run(8080);
}
#[test]
fn when_proper_listener_is_created_should_accept_connection() {
    use std::net::TcpStream;
    use std::thread;
    use std::time::Duration;

    // Załóżmy, że HttpServer implementuje metodę `run` uruchamiającą serwer.
    let server: HttpServer = HttpServer::run(8000);

    // Uruchomienie wątku, który będzie próbował nawiązać połączenie.
    thread::spawn(move || {
        // Czekamy krótko, aby dać czas serwerowi na uruchomienie.
        thread::sleep(Duration::from_millis(100));
        let _connection = TcpStream::connect("127.0.0.1:8000").expect("Failed to connect");
    });

    // Serwer powinien zaakceptować połączenie.
    let _accept = server.accept().expect("Failed to accept connection");
}
