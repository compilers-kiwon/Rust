use image::{GenericImage, GenericImageView, Rgba};

fn main() {
    // 명령줄 인수 얻기
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("[USAGE] image_filter.exe <image_file>");
        return;
    }

    // 입력 파일과 출력 파일을 지정
    let infile = args[1].clone();
    let file_name: Vec<&str> = infile.split(".").collect();
    let outfile = format!("{}-out.jpg", file_name[0]);

    println!("infile={}", infile);
    println!("outfile={}", outfile);

    // 이미지 파일 읽기
    let mut img = image::open(infile).expect("파일을 읽을 수 없습니다");

    // 이미지의 가로와 세로 크기(튜플)
    let (w, h) = img.dimensions();

    // 행과 열을 반복하면서
    for y in 0..h {
        for x in 0..w {
            // (x, y) 좌표의 픽셀 데이터 얻기
            let c: Rgba<u8> = img.get_pixel(x, y);

            // 색상 반전 처리
            let c = Rgba([
                255-c[0],   // Red
                255-c[1],   // Green
                255-c[2],   // Blue
                c[3]        // 투명도는 그대로
            ]);

            // 반전된 픽셀을 (x, y) 좌표에 저장
            img.put_pixel(x, y, c);
        }
    }

    // 이미지 저장
    img.save(outfile).unwrap();
}