# -*- coding: utf-8 -*-

from cffi import FFI

ffi = FFI()
ffi.cdef("""
    int double(int);
    void process(int threads);
""")

C = ffi.dlopen(".\\target\\release\\lib.dll")

print C.double(3)
C.process(1000)
