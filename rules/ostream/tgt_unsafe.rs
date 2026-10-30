// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::io::Write;

unsafe fn f7<T1>(mut a0: &mut ::std::fs::File, a1: *const libc::c_char) -> *mut ::std::fs::File {
    let __r: &mut ::std::fs::File = &mut a0;
    let __s: *mut ::std::fs::File = __r;
    let _ = (*__s).write_all(::std::ffi::CStr::from_ptr(a1).to_bytes());
    __s
}

unsafe fn f10<T1>(mut a0: &mut ::std::fs::File, a1: libc::c_char) -> *mut ::std::fs::File {
    let __r: &mut ::std::fs::File = &mut a0;
    let __s: *mut ::std::fs::File = __r;
    let _ = (*__s).write_all(&[a1 as u8]);
    __s
}

unsafe fn f11<T1>(mut a0: &mut ::std::fs::File, a1: u8) -> *mut ::std::fs::File {
    let __r: &mut ::std::fs::File = &mut a0;
    let __s: *mut ::std::fs::File = __r;
    let _ = (*__s).write_all(&[a1]);
    __s
}

unsafe fn f2<T1, T2>(mut a0: &mut ::std::fs::File) -> *mut ::std::fs::File {
    let __r: &mut ::std::fs::File = &mut a0;
    libcc2rs::endl_unsafe(__r)
}

unsafe fn f3<T1, T2>(mut a0: &mut ::std::fs::File) -> *mut ::std::fs::File {
    let __r: &mut ::std::fs::File = &mut a0;
    libcc2rs::ends_unsafe(__r)
}

unsafe fn f4<T1, T2>(mut a0: &mut ::std::fs::File) -> *mut ::std::fs::File {
    let __r: &mut ::std::fs::File = &mut a0;
    libcc2rs::flush_unsafe(__r)
}
