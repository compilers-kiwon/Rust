// 소수를 반환하는 8비트 반복자
struct PrimeIterator {
    n: u8
}

// 메서드 정의
impl PrimeIterator {
    fn new() -> Self { PrimeIterator{n: 1} }
    // n 이 소수인지 확인
    fn is_prime(&self) -> bool {
        for i in 2..self.n {
            if self.n%i == 0 { return false; }
        }
        return true;
    }
}

// 반복자 구현
impl Iterator for PrimeIterator {
    type Item = u8; // 반복자의 반환 요소 타입
    // 다음 소수 값을 반환
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.n += 1;
            // 8비트를 넘어가면 더이상 찾지 않음
            if std::u8::MAX == self.n {
                return  None
            }
            // self.n 이 소수인지 확인
            if self.is_prime() { return Some(self.n); }
        }
    }
}

fn main() {
    // 반복자 생성
    let prime_iter = PrimeIterator::new();
    // for 문으로 반복
    for n in prime_iter {
        print!("{},", n);
    }
}