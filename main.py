# -*- coding: utf-8 -*-

from cffi import FFI

ffi = FFI()
ffi.cdef("""
    int double(int);
""")

C = ffi.dlopen("target/release")