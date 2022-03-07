use std::io;
#[tokio::test]
async fn health_check_works (){
	spawn_app();
	//Bring reqwest to perform HTTP requests against our app
	let client = reqwest::Client::new();

	let response = client
		.get("http://127.0.0.1:8000/health_check")
		.send()
		.await
		.expect("Failed to send request");
	
	assert!(response.status().is_success());
	assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
	let server = troperust::run("127.0.0.1:8000").expect("failes to bind address");
	let _ = tokio::spawn(server);
}