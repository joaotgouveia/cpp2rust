// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f7(a0: ::std::fs::File) -> ::std::fs::File {
    a0
}

unsafe fn f2<T1, T2>(a0: *const libc::c_char) -> ::std::fs::File {
    ::std::fs::File::open(::std::ffi::CStr::from_ptr(a0).to_str().unwrap()).unwrap()
}
