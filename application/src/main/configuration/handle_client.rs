use std::io::{stdout, Read, Write};
use std::net::TcpStream;

use crossterm::style::{Color, Print, SetForegroundColor};
use crossterm::{execute, terminal};

use telnet::main::menu::get_health_command::get_health_command;
use telnet::main::menu::get_menu_command::get_menu_command;

use super::use_case_configuration::GET_MENU_USE_CASE;

pub fn handle_client(stream: &mut TcpStream) {
    write!(stream, "Welcome to the Telnet server!\r\n").unwrap();
    // stream.flush().unwrap();

    // let mut stdout = stdout();
    // execute!(stream, terminal::EnterAlternateScreen).unwrap();

    // execute!(stream, SetForegroundColor(Color::Blue), Print("Hello, ")).unwrap();
    // execute!(stream, SetForegroundColor(Color::Red), Print("Rust!")).unwrap();
    // Ok(())
    // write!(stream, "Enter a command: ").unwrap();
    // stream.flush().unwrap();

    // loop {
    //     let reader = stream.try_clone().unwrap().bytes();
    //     let mut command = "".to_string();

    //     for byte in reader {
    //         if let Ok(byte) = byte {
    //             if let Some(key) = std::char::from_u32(u32::from(byte)) {
    //                 match key {
    //                     '\r' => {
    //                         command = command.trim().to_string();
    //                         dbg!(&command);
    //                         write!(stdout, "{}", &command).unwrap();
    //                         execute_command(&command, stream.try_clone().unwrap());
    //                         stdout.flush().unwrap();
    //                         break;
    //                     }
    //                     _ => {
    //                         command += &key.to_string();
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
}

fn execute_command(command: &str, stream: TcpStream) {
    let stream = stream.try_clone().unwrap();
    match command {
        "check health" => {
            get_health_command(stream);
        }
        "get menu" => {
            get_menu_command(GET_MENU_USE_CASE.clone() as _, stream);
        }
        &_ => {}
    }
}
