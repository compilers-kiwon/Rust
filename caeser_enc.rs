fn encrypt(text: &str, shift: i16) -> String {
    let a = 'A' as i16;
    let is_upper_alpha = |c| 'A' <= c && c <= 'Z';
    let conv = |c| (((c-a+shift+26)%26+a) as u8) as char;
    let enc = |c| if is_upper_alpha(c) {conv(c as i16)} else {c};
    text.chars().map(|c| enc(c)).collect()
}

fn main() {
    let enc = encrypt("I LOVE RUST.",3);
    let dec = encrypt(&enc,-3);
    println!("{} => {}", enc, dec);
}