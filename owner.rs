fn main() {
    let g1 = 30;
    let g2 = g1;
    println!("g1={}, g2={}",g1,g2);

    let s1 = String::from("온화한 마음은 몸에 좋다");
    let s2 = s1.clone();

    println!("{}",s1);
    println!("{}",s2);
}