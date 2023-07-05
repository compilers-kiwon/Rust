fn main() {
    let moon = 384400.0;
    let car = 80.0;
    let btrain = 300.0;

    println!("To Moon by Car : {} days",moon/car/24.0);
    println!("To Moon by KTX : {} days",moon/btrain/24.0);
}