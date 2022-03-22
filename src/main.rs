use troperust::startup::run;
use std::net::TcpListener;
use troperust::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration");
    //remove hardcoded `8000` -it's now coming from settings
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}