use std::{env, fs};

fn main() {
    // 명령어 인수들을 읽어들임
    let args: Vec<String> = env::args().collect();

    // 명령어 인수가 모자를 경우, 예외 처리
    if args.len() < 3 {
        println!("[Usage] cmp_text <file1> <file2>");
        println!("<file1> or <file2> is missed.");
        return;
    }

    let filename1 = &args[1];
    let filename2 = &args[2];

    // 각각의 파일을 문자열로 읽어들임
    let file1_str = fs::read_to_string(filename1).unwrap();
    let file2_str = fs::read_to_string(filename2).unwrap();

    // 불필요한 공백 삭제
    let file1_str = file1_str.trim();
    let file2_str = file2_str.trim();

    // 비교
    if file1_str == file2_str {
        println!("OK");
    } else {
        println!("NG");
    }
}