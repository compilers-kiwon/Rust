// 2개 이상의 인수를 표시하는 매크로 정의
#[macro_export]
macro_rules! echo_nums {
    ( $( $num:expr ),* ) => {
        $(
            print!("{}, ", $num);
        )*
        println!("");
    }
}

// 매크로 이용
fn main() {
    echo_nums![10, 20, 30, 40, 50];
    echo_nums!(60, 70);
    echo_nums!{80, 90, 100};
}