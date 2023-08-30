#include    <stdio.h>

// Rust 로 만든 함수를 정의
int rust_mul(int a, int b);

int main(void) {
    // Rust 로 만든 함수를 실행
    printf("%d\n", rust_mul(10, 8));
    printf("%d\n", rust_mul(9, 9));
    return  0;
}