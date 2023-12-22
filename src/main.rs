use application::main::configuration::rest_configuration::start_web_backend;
use telnet::main::server::start_telnet_server;

fn main() {
    // let _ = start_telnet_server();
    let _ = start_web_backend();
}
