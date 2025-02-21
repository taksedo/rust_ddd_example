use std::error::Error;

use futures_util::SinkExt;
use nectar::{TelnetCodec, event::TelnetEvent};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

pub async fn get_health_command(
    frame: &mut Framed<TcpStream, TelnetCodec>,
) -> Result<(), Box<dyn Error>> {
    frame
        .send(TelnetEvent::Message("Healthy!\n".to_string()))
        .await?;
    Ok(())
}
