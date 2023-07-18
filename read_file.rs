use std::env;   // 명령줄 인수 취득용
use std::fs;    // 파일 읽기용

fn main() {
    // 인수를 벡터로 취득
    let args: Vec<String> = env::args().collect();

    // 인수에 파일 이름을 지정했는가?
    if args.len() < 2 {
        println!("There is no file name.");
        return;
    }

    // 두번째 인수 => 파일 이름
    let filename = &args[1];

    // 파일 읽기
    let text = match fs::read_to_string(filename) {
        Ok(_v) => _v,
        Err(_e) => _e.to_string(),
    };

    println!("{}", text);
}