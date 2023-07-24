fn main() {
    let mut g1 = "실수할 줄 아는 사람이 아름답다".to_string();
    g1 = show_message(g1);
    println!("g1의 소유권이 유효함, {}",g1);
}

fn show_message(message: String) -> String {
    println!("{}",message);
    return message;
}