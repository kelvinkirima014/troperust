#[tokio::test]
async fn get_response_works(){
	let address = spawn_app();
	let client = reqwest::Client::new();

	let response = client
		.get(format!("{}/get_response", address))
		.send()
		.await
		.expect("Failed to execute our request");

		assert!(response.status().is_success());
		assert_eq!(Some(0), response.content_length());
}
use std::net::TcpListener;
fn spawn_app() -> String {
	let listener = TcpListener::bind("127.0.0.1:0")
	.expect("Failed to bind random port");
	//retrieve port from OS
	let port = listener.local_addr().unwrap()
	.port();
	let server = troperust::run(listener)
	.expect("Failed to bind address");
	let _ =  tokio::spawn(server);
	//return the application address to caller
	format!("http://127.0.0.1:{}", port)
}