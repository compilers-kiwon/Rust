use std::env;
use std::fs::{self, File};
use std::io::{Write, BufWriter};

const OUTFILE: &str = "fizzbuzz.out.txt";

fn main() {
    // Command Line 의 인수들을 읽어서 벡터에 저장
    let args: Vec<String> = env::args().collect();

    // 어디까지 FizzBuzz 를 구할 것인지 확인
    if args.len() < 2 {
        println!("[USAGE] fizzbuzz <number>");
        return;
    }

    let max_num = &args[1];
    let max_num = match max_num.parse() {
        Ok(_v) => _v,
        Err(_) => 0,
    };

    if max_num == 0 {
        println!("Please specify what number is upper bound.");
        return;
    }

    {
        // 파일 생성/열기
        let fp = File::create(OUTFILE).unwrap();
        let mut writer = BufWriter::new(fp);

        // FizzBuzz 구하기
        for i in 1..max_num+1 {
            // Default string with just number
            let mut line = format!("{}\n",i);

            if i%15 == 0 {
                line = String::from("FizzBuzz\n");
            } else if i%3 == 0 {
                line = String::from("Fizz\n");
            } else if i%5 == 0 {
                line = String::from("Buzz\n");
            }

            // 파일에 쓰기
            let bytes = line.as_bytes();
            writer.write(bytes).unwrap();
        }
    }

    // 저장한ㅍ파일의 내용을 읽어들여 화면에 출력
    let s = fs::read_to_string(OUTFILE).unwrap();
    println!("{}",s);
}