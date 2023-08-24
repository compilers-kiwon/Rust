// BASIC 문법 for 문 매크로 정의
#[macro_export]
macro_rules! basic_for {
    // for i=1 to 10 과 같은 경우
    (
        for $i:ident = $from:tt to $to:tt
        $block:block
    ) => {{
        for $i in $from..=$to {
            $block
        }
    }};
    // for i=1 to 10 step 2 와 같은 경우
    (
        for $i:ident = $from:tt to $to:tt step $step:tt
        $block:block
    ) => {{
        let mut $i = $from;
        loop {
            if $i > $to { break }
            $block
            $i += $step
        }
    }};
}

fn main() {
    // 매크로를 이용해 1부터 10까지 합 구하기
    let mut total = 0;
    basic_for! {
        for i = 1 to 10 {
            total += i;
        }
    }
    println!("{}", total);
    // 매크로를 이용해 0부터 10까지 3이 증가할 때마다 출력하기
    basic_for! {
        for i = 0 to 10 step 3 {
            println!("i={}", i);
        }
    }
}