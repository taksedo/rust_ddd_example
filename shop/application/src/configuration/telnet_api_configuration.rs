use std::error::Error;

use futures_lite::StreamExt;
use futures_util::sink::SinkExt;
use log::info;
use nectar::{event::TelnetEvent, TelnetCodec};
use telnet::main::menu::{
    get_health_command::get_health_command, get_menu_command::get_menu_command,
};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

use super::use_case_configuration::GET_MENU_USE_CASE;

pub(super) async fn handle_telnet_client(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    // We construct a 'Frame', which is just a wrapper around the underlying
    // stream that is decoded by the `nectar::TelnetCodec`.
    let mut frame = Framed::new(stream, TelnetCodec::new(1024));

    // Let's send a friendly welcome message to anyone who connects!
    frame
        .send(TelnetEvent::Message(
            "\nWelcome to the nectar telnet server!\nYou can exit by typing \"quit\".\n"
                .to_string(),
        ))
        .await?;

    // In a real application, you would want to handle Some(Err(_)) and None
    // variants, but for this example we'll be succinct for simplicities sake.
    while let Some(Ok(msg)) = frame.next().await {
        match msg {
            TelnetEvent::Message(string) => {
                match string.as_str() {
                    "quit" => {
                        break;
                    }
                    "check health" => {
                        info!("Checking health by Telnet");
                        get_health_command(&mut frame).await?;
                    }
                    "get menu" => {
                        info!("Getting menu by Telnet");
                        get_menu_command(GET_MENU_USE_CASE.clone(), &mut frame).await?;
                    }
                    // // ...or just echo back whatever the user has said!
                    _ => {
                        frame
                            .send(TelnetEvent::Message(format!("You said: {}\n", string)))
                            .await?;
                    }
                }
                // We can check for commands...
            }
            // We break here to close to connection.
            _ => break,
        }
    }

    // When the above loop breaks we'll send a goodbye message before closing.
    frame
        .send(TelnetEvent::Message("Goodbye!\n".to_string()))
        .await?;

    Ok(())
}
