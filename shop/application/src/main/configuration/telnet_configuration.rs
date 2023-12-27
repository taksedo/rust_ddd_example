use std::env;

use log::{error, info};
use tokio::{net::TcpListener, task, task::JoinHandle};

use crate::main::configuration::handle_client::handle_client;

pub fn telnet_backend_startup() -> JoinHandle<()> {
    task::spawn(async move {
        let telnet_host_url = env::var("TELNET_HOST_URL").unwrap();
        let listener = TcpListener::bind(&telnet_host_url).await.unwrap();
        info!("Starting Telnet server at {telnet_host_url}");
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    tokio::spawn(async move {
                        if let Err(e) = handle_client(stream).await {
                            error!("error: {}", e);
                        }
                    });
                }
                Err(e) => println!("couldn't get client: {:?}", e),
            }
        }
    })
}
