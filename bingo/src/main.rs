// rand Crate for suffling array
use rand::seq::SliceRandom;

// #define
const MAX_NUM: usize = 75;
const JOKER: usize = 12;

fn main() {
    // 1~75 array
    let mut nums = [0;MAX_NUM];
    for i in 1..75 { nums[i-1] = i;}

    // shuffle
    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);

    // 75 개의 숫자들 중, 앞에서 25개만 사용
    for row in 0..5 {
        for col in 0..5 {
            let idx = row*5+col;

            if idx == JOKER {
                print!("  *,");
            } else {
                print!("{:3},",nums[idx]);
            }
        }
        println!("");
    }
}