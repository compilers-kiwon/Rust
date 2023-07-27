use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 난수 초기화
    let mut seed = rand_init();

    // 100 개의 난수 생성
    for _ in 0..100 {
        let r = rand(&mut seed, 1, 6);
        println!("{}", r);
    }
}

fn rand_init() -> u32 {
    // 현재 시간을 이용해 난수의 초기값을 결정
    SystemTime::now()
        .duration_since(UNIX_EPOCH).unwrap()
        .as_millis() as u32
}

fn rand(seed: &mut u32, start: u32, end: u32) -> u32 {
    *seed ^= *seed<<13;
    *seed ^= *seed>>17;
    *seed ^= *seed<<5;

    return  *seed%(end-start+1)+start;
}