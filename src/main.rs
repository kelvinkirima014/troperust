use troperust::startup::run;
use std::net::TcpListener;
use troperust::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration");
    //remove hardcoded `8000` -it's now coming from settings
    let address = TcpListener::bind("127.0.0.1:{}").unwrap();
    run(address)?.await
}