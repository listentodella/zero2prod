use actix_web::dev::Server;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use std::net::TcpListener;

// pub async fn run() -> std::io::Result<()> {
// 注意这里删除了async, 也没有await了
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            // 为 POST/subscriptions 在请求路由表里添加一个新的条目
            .route("/subscriptions", web::post().to(subscribe))
            .route("/", web::get().to(greet))
            // 这个要么删除，要么放到最后，因为App会遍历所有注册的端点，放在前面会被优先匹配
            .route("/{name}", web::get().to(greet))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {}!", name)
}

// async fn health_check(req: HttpRequest) -> impl Responder {
async fn health_check() -> impl Responder {
    //和直接用Ok()返回是一样的效果
    //为了和其他的风格一致,使用finish()
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
