use std::io;
use std::net::TcpListener;

use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
async fn health_check_works (){
	let address = spawn_app();
	//Bring reqwest to perform HTTP requests against our app
	let client = reqwest::Client::new();

	let response = client
		.get(&format!("{}/health_check", &address))
		.send()
		.await
		.expect("Failed to send request");
	
	assert!(response.status().is_success());
	assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
	let listener = TcpListener::bind("127.0.0.1:0")
		.expect("failed to bind random port");
	let port = listener.local_addr().unwrap().port();
	let server = troperust::run(listener).expect("Failed tp bind address");
	let _ = tokio::spawn(server);

	//return app address to the caller
	format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn subsribe_returns_a_200_valid_form_data(){
	let app_address = spawn_app();
	let client = reqwest::Client::new();

	Mock::given(path("/email"))
		.and(method("POST"))
		.respond_with(ResponseTemplate::new(200))
		.mount(&app.email_server)
		.await;

	//Act
	let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
	let response = client
		.post(&format!("{}/subsriptions", &app_address))
		.body(body)
		.send()
		.await
		.expect("Failed to execute post request");
	assert_eq!(200, response.status().as_u16());

}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
	let app_address = spawn_app();
	let client = reqwest::Client::new();
	let test_cases = vec![
		("name=le%20guin", "missing the email"),
		("email=ursula_le_guin%40gmail.com", "missing the name"),
		("", "missing both email and name")
	];

	for (invalid_body, error_message) in test_cases {
		let response = client
			.post(&format!("{}/subsriptions", &app_address))
			.header("Content-Type", "application/x-www-form-urlencoded")
			.body(invalid_body)
			.send()
			.await
			.expect("Failed to execute request");
		
		assert_eq!(
			400,
			response.status().as_u16(),
		  	"The API did not fail with 400 bad Request when the payload was {}",
			error_message
		);
	}
}