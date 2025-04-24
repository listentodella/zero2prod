use actix_web::dev::Server;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};

// pub async fn run() -> std::io::Result<()> {
// 注意这里删除了async, 也没有await了
pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/", web::get().to(greet))
            // 这个要么删除，要么放到最后，因为App会遍历所有注册的端点，放在前面会被优先匹配
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run();

    Ok(server)
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {}!", name)
}

// async fn health_check(req: HttpRequest) -> impl Responder {
async fn health_check() -> impl Responder {
    // HttpResponse::Ok().finish()
    HttpResponse::Ok()
}
