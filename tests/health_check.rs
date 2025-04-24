use std::net::TcpListener;

use tokio;

// 该宏是 tokio::main 的等价
// 可以使用 cargo +nightly expand --test health_check 查看展开后的代码
#[tokio::test]
async fn health_check_works() {
    // spawn_app().await.expect("Failed to spawn app");
    let addr = spawn_app(); // 不需要await, expect了
    let client = reqwest::Client::new();
    let response = client
        // 使用动态返回的地址
        .get(&format!("{}/health_check", &addr))
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// async fn spawn_app() -> std::io::Result<()> {
fn spawn_app() -> String {
    // 0 是操作系统提供的特殊端口，将触发os扫描可用端口
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind random port");
    // 检索OS分配的端口
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("failed to bind addr");
    // 启动服务器作为后台任务
    // tokio::spawn 返回一个指向spawned future的handle
    let _handle = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
