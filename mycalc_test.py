# 파이썬에서 동적 라이브러리 이용
import platform, os
from ctypes import *

# 동적 라이브러리 경로 지정
libpath = os.path.join(os.path.dirname(__file__), 'libmycalc.so')
print("lib=", libpath)

# 라이브러리 로드
mycalc = cdll.LoadLibrary(libpath)
# Rust 라이브러리 실행
print(mycalc.rust_mul(100, 8))
print(mycalc.rust_mul(8,9))
