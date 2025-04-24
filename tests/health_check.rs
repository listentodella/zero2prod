use tokio;

// 该宏是 tokio::main 的等价
// 可以使用 cargo +nightly expand --test health_check 查看展开后的代码
#[tokio::test]
async fn health_check_works() {
    // spawn_app().await.expect("Failed to spawn app");
    spawn_app(); // 不需要await, expect了
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:8000/health_check")
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// async fn spawn_app() -> std::io::Result<()> {
fn spawn_app() {
    let server = zero2prod::run().expect("failed to bind addr");
    // 启动服务器作为后台任务
    // tokio::spawn 返回一个指向spawned future的handle
    let _handle = tokio::spawn(server);
}
