use actix_web::{web, App, HttpServer, HttpRequest};

// 서버 주소와 포트를 지정
const SERVER_ADDR: &str = "127.0.0.1:8888";

// Actix Web 메인 함수
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("[SERVER] http://{}/", SERVER_ADDR);
    // HTTP 서버 시작
    HttpServer::new(|| {
        // 라우팅 지정
        App::new()
            .route("/", web::get().to(index))
    })
    .bind(SERVER_ADDR)?
    .run()
    .await
}

// 실행할 함수
async fn index(req: HttpRequest) -> &'static str {
    println!("request: {:?}", req);
    "Hello, World!"
}