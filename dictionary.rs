use std::{env, fs};
use std::io::{BufRead, BufReader};

const DICTFILE: &str = "dict.txt";

fn main() {
    // Command Line 의 인수들을 읽어서 벡터에 저장
    let args: Vec<String> = env::args().collect();

    // 찾고자 하는 단어가 입력 되었는지 확인
    if args.len() < 2 {
        println!("[USAGE] dictionary <word>");
        return;
    }

    // 찾고싶은 단어
    let word = &args[1];

    // 사전 파일 열기
    let fp = fs::File::open(DICTFILE).unwrap();

    // BufReader 로 읽어들임
    let reader = BufReader::new(fp);

    for line in reader.lines() {
        // 한줄씩 처리
        let line = line.unwrap();

        // 찾고자 하는 단어가 포함되어 있는 줄인지 확인
        if line.find(word) == None { continue; }
        println!("{}", line);
    }
}