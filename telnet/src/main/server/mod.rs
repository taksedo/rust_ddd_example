use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use termion::{event::Key, input::TermRead, raw::IntoRawMode};

fn handle_client(mut stream: TcpStream) {
    let mut reader = stream.try_clone().unwrap();
    let mut writer = stream.try_clone().unwrap();

    write!(writer, "Welcome to the Rust Telnet server!\r\n").unwrap();
    writer.flush().unwrap();

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('\n') => {
                write!(writer, "\r\n").unwrap();
                writer.flush().unwrap();
            }
            Key::Char(c) => {
                write!(writer, "{}", c).unwrap();
                writer.flush().unwrap();
            }
            Key::Ctrl('c') => {
                break;
            }
            _ => {}
        }
    }
}

#[tokio::main]
pub async fn start_telnet_server() {
    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();

    println!("Telnet server listening on port 8081");

    loop {
        let (stream, _) = listener.accept().unwrap();

        thread::spawn(|| {
            handle_client(stream);
        });
    }
}
