fn main() {
    let mut msg = "건강한 신체에 건강한 정신이 깃든다.".to_string();
    println!("{}",msg);
    add_quote(&mut msg);
    println!("{}",msg);
}

fn add_quote(msg: &mut String) {
    msg.insert(0,'"');
    msg.push('"');
}