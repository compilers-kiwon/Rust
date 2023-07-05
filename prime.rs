fn is_prime(n:i32) -> bool {
    for i in 2..n {
        if n%i == 0 {
            return  false
        }
    }
    return  true
}

fn get_primes(primes: &mut[i32;100]) {
    let mut n = 2;
    let mut cnt = 0;

    while cnt != 100 {
        if is_prime(n) {
            primes[cnt] = n;
            cnt += 1;
        }

        n += 1;
    }
}

fn main() {
    let mut primes = [0;100];
    get_primes(&mut primes);
    println!("{:?}",primes);
}