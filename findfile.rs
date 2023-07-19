use std::{env, path};

fn main() {
    //Command Line 인수 확인
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("[USAGE] findfile <start path> <keyword>");
        return;
    }

    let start_path = &args[1];
    let keyword = &args[2];

    // PathBuf 로 변환
    let current = path::PathBuf::from(start_path);
    findfile(&current, keyword);
}

fn findfile(current: &path::PathBuf, keyword: &str) {
    // 현재 디렉토리의 파일/서브 디렉토리 목록 취득
    let files = current.read_dir().expect("존재하지 않는 경로입니다.");

    for dir_entry in files {
        // PathBuf 로 경로 취득
        let path = dir_entry.unwrap().path();

        // 디렉토리라면 recursive call
        if path.is_dir() {
            findfile(&path, keyword);
            continue;
        }

        // 파일 이름을 문자열로 변환
        // to_string_lossy() : OsStr -> Str 변환에 사용
        let filename = path.file_name().unwrap().to_string_lossy();

        // keyword 가 포함되어 있는가?
        if filename.find(keyword) == None { continue; }
        // 포함되어 있다면 출력
        println!("{}",path.to_string_lossy());
    }
}