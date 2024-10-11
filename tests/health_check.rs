use reqwest::Client;

fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}

#[tokio::test]
async fn health_check_succeeds() {
    spawn_app();

    let client = Client::new();
    let response = client
        .get("http://127.0.0.1:7878/health_check")
        .send()
        .await
        .expect("failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
