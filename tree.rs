use std::{env, path};

fn main() {
    // Command Line 에 경로가 지정된 경우 해당 경로부터, 아니면 현재 경로부터 탐색
    let args: Vec<String> = env::args().collect();
    let mut target_dir = ".";
    if args.len() >= 2 {
        target_dir = &args[1];
    }

    // PathBuf 로 변환
    let target = path::PathBuf::from(target_dir);
    println!("{}", target_dir);
    tree(&target, 0);
}

// Recursive Function
fn tree(target: &path::PathBuf, level: isize) {
    // 현재 디렉토리의 파일 목록
    let files = target.read_dir().expect("존재하지 않는 경로입니다.");

    // 반복해서 표시
    for ent in files {
        let path = ent.unwrap().path();
        
        // level 만큼 들여쓰기
        for _ in 1..=level { print!("|  "); }

        // 파일 이름
        let filename = path.file_name().unwrap().to_string_lossy();

        // Recursive call for a directory
        if path.is_dir() {
            println!("|-- <{}>", filename);
            tree(&path, level+1);
            continue;
        }

        // 파일 이름 출력
        println!("|-- {}", filename);
    }
}