use troperust::run;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
	run().await
}
