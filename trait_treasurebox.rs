// 보물 상자의 동작을 정의하는 트레잇
trait TreasureBox {
    // 기본 메서드
    fn open(&self, key_no: i32) -> bool {
        self.get_key_no() == key_no   // 지정된 열쇠로만 상자가 열림
    }
    fn check(&self);
    fn get_key_no(&self) -> i32;
}

// 보석이 들어 있는 상자의 구조체를 정의
struct JewelryBox {
    price: i32,     // 들어있는 골드의 양
    key_no: i32,    // 이 상자를 열수 있는 열쇠의 번호
}

impl TreasureBox for JewelryBox {
    fn check(&self) {
        println!("보석 상자였다. {} 골드 입수", self.price);
    }

    fn get_key_no(&self) -> i32 {
        self.key_no
    }
}

// 빈 상자의 구조체 정의
struct EmptyBox {
    key_no: i32
}

impl TreasureBox for EmptyBox {
    fn check(&self) {
        println!("비어있는 상자이다.");
    }

    fn get_key_no(&self) -> i32 {
        self.key_no
    }
}

// 모험가가 상자를 여는 동작
fn open_box(tbox: &impl TreasureBox, key_no: i32){
    if tbox.open(key_no) {
        tbox.check();
    } else {
        println!("맞는 열쇠가 아니다.");
    }
}

fn main() {
    // 다양한 상자를 준비
    let box1 = JewelryBox {
        price: 30,
        key_no: 1
    };

    let box2 = EmptyBox {
        key_no: 1
    };

    let box3 = JewelryBox {
        price: 20,
        key_no: 2
    };

    // 모험가가 가진 열쇠로 상자를 연다.
    open_box(&box1, 1);
    open_box(&box2, 1);
    open_box(&box3, 1);
}