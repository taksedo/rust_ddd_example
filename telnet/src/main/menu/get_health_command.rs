use std::io::Write;
use std::net::TcpStream;

pub fn get_health_command(mut stream: TcpStream) {
    let _ = write!(stream, "healthy!");
    stream.flush().unwrap();
}
