// Item 구조체(튜플) 정의
struct Item(String, i64);

fn main() {
    // 튜플 만들기
    let banana = Item("바나나".to_string(), 300);
    let apple = Item("사과".to_string(), 200);
    let mango = Item("망고".to_string(), 500);

    // Item 을 벡터에 추가
    let items = vec![banana, apple, mango];

    // 합계 금액 구하기
    let total = print_and_sum_items(&items);
    println!("합계는 {}원입니다.", total);
}

// 튜플을 표시하는 함수
fn print_tuple(item: &Item) {
    println!("{}를 {}원에 구입", item.0, item.1);
}

// Item 을 순서대로 표시하고 합계 금액 구하기
fn print_and_sum_items(items: &Vec<Item>) -> i64 {
    let mut total = 0;

    for it in items {
        print_tuple(&it);
        total += it.1;
    }

    total // 합계 금액 반환, return total;
}