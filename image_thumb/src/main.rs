use image;

fn main() {
    // re-size 후의 크기 지정
    let size = 128;
    // 명령줄 인수 얻기
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("[USAGE] > image_thumb.exe <image file>");
        return;
    }

    // 입력 파일과 출력 파일 지정
    let infile = String::from(&args[1]);
    let filename: Vec<&str> = infile.split(".").collect();
    let outfile = format!("{}-thumb.png", filename[0]);
    println!("input: {}", infile);
    println!("output: {}", outfile);

    // 이미지 파일 읽기
    let img = image::open(infile)
                    .expect("파일을 읽을 수 없습니다.");
    // 섬네일 만들기
    let thumb = img.thumbnail(size, size);

    // 파일로 저장
    thumb.save(outfile).unwrap();
}