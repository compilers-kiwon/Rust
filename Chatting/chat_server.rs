use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Write};
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    // 서버의 주소와 포트 번호 지정
    let server_addr = "127.0.0.1:8888";
    // 스레드 사이의 통신 준비
    let (tx, rx) = mpsc::channel::<String>();
    // 클라이언트 목록 저장용 변수 선언
    let mut clients: Vec<TcpStream> = Vec::new();

    // 서버 시작
    let server = TcpListener::bind(server_addr).expect("서버를 실행할 수 없습니다.");
    server.set_nonblocking(true).expect("알 수 없는 에러가 발생하였습니다.");
    println!("{}에서 서버가 실행 중입니다.", server_addr);

    // 메인 스레드
    loop {
        // 클라이언트 접속 처리
        if let Ok((client, addr)) = server.accept() {
            println!("클라이언트가 접속 하였습니다: {}", addr);
            clients.push(client.try_clone().unwrap());
            start_thread(client, tx.clone());
        }

        // 스레드 사이의 통신 처리
        if let Ok(msg) = rx.try_recv() {
            println!("전원에게 보내기: {}", msg.trim());
            clients = send_all(clients, &msg);
        }

        thread::sleep(Duration::from_millis(100));
    }
}

// 클라이언트가 보내는 메세지 수신 스레드
fn start_thread(client: TcpStream, tx: mpsc::Sender<String>) {
    let mut reader = BufReader::new(client);
    thread::spawn(move || loop {
        // 메세지 수신 대기
        let mut msg = String::new();
        if let Ok(n) = reader.read_line(&mut msg) {
            // 수신한 메세지를 메인 스레드로 전달
            if n > 0 { tx.send(msg).unwrap(); }
        }
        thread::sleep(Duration::from_millis(100));
    });
}

// 모든 클라이언트들에게 메세지 전송
fn send_all(clients: Vec<TcpStream>, s: &str) -> Vec<TcpStream> {
    let mut collector = vec![];

    for mut socket in clients.into_iter() {
        // 문자열을 바이트열로 변환하여 전송
        let bytes = String::from(s).into_bytes();
        if let Err(e) = socket.write_all(&bytes) {
            println!("전송 에러 : {}", e);
            continue;
        }
        collector.push(socket); // 소유권 회수
    }
    collector   // 소유권 반환
}