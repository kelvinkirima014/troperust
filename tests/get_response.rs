#[tokio::test]
async fn get_response_works(){
	spawn_app().await.expect("Failed to spawn our app");

	let client = reqwest::Client::new();

	let response = client
		.get("http://127.0.0.1:8080/get_response")
		.send()
		.await
		.expect("Failed to execute our request");

		assert!(response.status().is_success());
		assert_eq!(Some(0), response.content_length());
}
//launch app in background
async fn spawn_app() -> std::io::Result<()> {
	troperust::run().await
}