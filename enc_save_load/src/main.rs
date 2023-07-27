use encoding_rs;
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    // 저장할 파일 이름
    let filename = "test-euckr.txt";

    // EUC-KR로 저장
    save_euckr(filename, "맛있게 먹으면 0 칼로리");

    // EUC-KR 파일 읽어오기
    let s = load_euckr(filename);
    println!("{}", s);
}

fn save_euckr(filename: &str, text: &str) {
    // EUC-KR 로 인코딩
    // Return : (Cow<[u8]>, Encoding, bool)
    // into_owned() : Cow data 의 소유권을 가져옴
    let (enc, _, _) = encoding_rs::EUC_KR.encode(text);
    let buf = enc.into_owned();

    // 파일 생성 및 내용 쓰기
    let mut fp = File::create(filename).expect("An error during a file creation!");
    fp.write(&buf[..]).expect("An error while writing the dato into a file!");
}

fn load_euckr(filename: &str) -> String {
    // 파일을 한 번에 읽어들임
    let buf = fs::read(filename).expect("읽기 에러");
    // EUC-KR 로 디코딩
    let (dec, _, _) = encoding_rs::EUC_KR.decode(&buf);
    return  dec.into_owned();
}