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
use wiremock::{Mock, ResponseTemplate};
use wiremock::matchers::{method, path};
#[tokio::test]
async fn suscribe_returns_a_200_for_valid_form_data(){
	//slet app = spawn_app().await;
	let app_address = spawn_app();
	let client = reqwest::Client::new();

	Mock::given(path("/email"))
	     .and(method("POST"))
	     .respond_with(ResponseTemplate::new(200))
	     .mount(&app.email_server)
	     .await;

	let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
	let response = client
		.post(&format!("{}/subscriptions", &app_address))
		.body(body) 
		.send()
		.await
		.expect("Failed to load request");
	
		assert_eq!(200, response.status().as_u16());
}
#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
	let app_address = spawn_app();
	let client = reqwest::Client::new();
	let test_cases = vec![
		("name=le%20guin", "missing the email"),
		("email=ursula_le_guin%40gmail.com", "missing the name"),
		("", "missing both name and email")
	];
	for (invalid_body, error_message) in test_cases {
		let response = client
			.post(&format!("{}/subscriptions", &app_address))
			.header("Content-Type", "application/x-www-form-urlencoded")
			.body(invalid_body)
			.send()
			.await
			.expect("failed to execute request");

		assert_eq!(
			400,
			response.status().as_u16(),
			//custom error msg on test failure
			"The API did not failed with 400 Bad Request when the payload was {}.",
			error_message

		);
	}
}






