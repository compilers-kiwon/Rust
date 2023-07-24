fn main() {
    let g1 = "실수할 줄 아는 사람이 아름답다".to_string();
    show_message(&g1);
    println!("g1의 소유권이 유효함, {}",g1);
}

fn show_message(message: &String) {
    println!("{}",message);
}