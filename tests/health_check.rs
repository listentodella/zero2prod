use tokio;

// 该宏是 tokio::main 的等价
// 可以使用 cargo +nightly expand --test health_check 查看展开后的代码
#[tokio::test]
async fn health_check_works() {
    spawn_app().await.expect("Failed to spawn app");
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:8080/health_check")
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> std::io::Result<()> {
    todo!()
}
