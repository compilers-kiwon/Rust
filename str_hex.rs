fn main () {
    hex_dump("성공하는 사람은 송곳처럼 어느 한 점을 향하여 일한다.");
}

fn hex_dump(s: &str) {
    for (i,c) in s.bytes().enumerate() {
        // 16 바이트 단위로 주소 표시
        if i%16 == 0 {
            print!("{:08x}|", i);
        }

        // 1 바이트 출력
        print!(" {:02x}", c);

        // 4 바이트 출력 후, '|' 표시
        if i%4 == 3 {
            print!("|");
        }

        // 16 바이트마다 줄바꿈
        if i%16 == 15 {
            println!("");
        }
    }

    println!("");
}