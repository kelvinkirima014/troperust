use std::io;
use std::net::TcpListener;
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

fn spawn_app() -> String{
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
}