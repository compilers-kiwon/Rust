use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // 힙 영역에 i32 가변성을 가진 i32 타입 값 1000 dmf wjwkd
    let a = Rc::new(RefCell::new(1000));
    // 참조 카운터 증가
    let b = Rc::clone(&a);
    // 값 변경 시도
    *b.borrow_mut() += 100;
    // 원래의 참조 값 확인
    println!("{}", a.borrow());
}