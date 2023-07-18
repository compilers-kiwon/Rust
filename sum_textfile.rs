use std::{env, fs};

fn main() {
    // 명령줄 인수 취득
    let args = env::args();
    let mut total:f64 = 0.0;

    // 각각의 인수 처리
    for (i, filename) in args.enumerate() {
        if i == 0 { continue; }
        // text file 을 읽어 들임
        let text = fs::read_to_string(filename).unwrap();
        // 한 줄씩 분리
        let lines = text.split("\r\n");
        // 각각의 분리된 라인들을 숫자로 변환하여 덧셈
        for line in lines {
            let num: f64 = match line.parse() {
                Ok(_v) => _v,
                Err(_) => 0.0,
            };
            
            total += num;
        }
    }

    println!("{}", total);
}