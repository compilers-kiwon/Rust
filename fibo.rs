fn main() {
    let mut a = 1;
    let mut b = 1;

    println!("{}",a);
    println!("{}",b);

    for _ in 0..30 {
        let cur = a+b;

        println!("{}",cur);
        a = b;
        b = cur;
    }
}