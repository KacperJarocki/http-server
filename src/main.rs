use http_server::HttpServer;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
fn main() {
    let server: HttpServer = HttpServer::run(8080);
    let port = server.local_addr().port();
    let address = format!("127.0.0.1:{}", port);
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        let connection = TcpStream::connect(address).expect("Failed to connect");
        println!(
            "Connection established {}, peer address: {}",
            connection.local_addr().unwrap(),
            connection.peer_addr().unwrap()
        );
    });
    loop {
        if let Some((socket, _)) = server.accept() {
            println!("Connection established");
            let _ = socket;
        }
    }
}
