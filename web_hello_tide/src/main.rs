// 서버 주소와 포트를 지정
const SERVER_ADDR: &str = "127.0.0.1:8888";

// 메인 함수
#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("http://{}/", SERVER_ADDR);
    // Tide 객체 생성
    let mut app = tide::new();
    // 라우팅 지정
    app.at("/").get(|_| async { Ok("Hello, World!") });
    // 서버 시작
    app.listen(SERVER_ADDR).await?;
    Ok(())
}