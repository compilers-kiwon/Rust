use std::rc::Rc;

fn main() {
    // 힙 영역에 i32 타입 값 1000 을 저장
    let a_rc = Rc::new(1000);
    {
        // i32 타입을 참조하는 b_rc 도 생성
        let b_rc = Rc::clone(&a_rc);
        println!("{}", b_rc);
        // a_rc 의 참조 카운트 확인
        println!("참조 카운트 = {}", Rc::strong_count(&a_rc));
    } // a_rc 의 참조 카운트가 1 감소
    println!("{}", a_rc);   // Rc 타입이므로 이동 가능
    println!("참조 카운트 = {}", Rc::strong_count(&a_rc));
}