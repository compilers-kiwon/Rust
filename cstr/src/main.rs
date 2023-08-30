use std::ffi::CString;

#[link(name="c_lib", kind="static")]
extern "C" {
    fn print_str(s: *const i8);
}

fn main() {
    // Rust 문자열 준비
    let msg = "Hello, World!";
    // C 언어 문자열로 변경
    let msg_cstr = CString::new(msg).unwrap();
    // C 라이브러리 호출
    unsafe {
        // 여기서 C 라이브러리 호출
        print_str(msg_cstr.as_ptr());
    }
}