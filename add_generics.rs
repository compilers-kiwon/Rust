// 제너릭을 이용하여 add() 정의
// std::ops::Add<Output=T> : 덧셈과 관련된 트레잇
fn add <T: std::ops::Add<Output=T>> (a:T, b:T) -> T {
    a+b
}

// 함수 사용하기
fn main() {
    println!("{}", add(10, 25));
    println!("{}", add(10.0, 25.0));
    println!("{}", add::<i32>(10, 25)); // 타입을 명시할 때
    // println!("{}", add('a', 'a')); --- char 타입은 미구현이므로 에러 발생
    //                                    char 타입의 덧셈은 Add 트레잇에 정의되지 않음
}