// 슬라이스의 각 요소를 더하는 함수
fn sum_slice(s: &[i64]) -> i64 {
    let mut ret = 0;
    for i in s { ret += i; }
    ret
}

fn main() {
    let a = [1,2,3,4,5,6,7,8,9,10];
    let b = vec![1,2,3,4,5,6,7,8,9,10];

    println!("a={}",sum_slice(&a[..]));
    println!("b={}",sum_slice(&b[..]));
}