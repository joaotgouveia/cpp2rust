// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::io::Write;

fn t1<T1, T2>() -> ::std::fs::File {
    ::std::fs::File::open("").unwrap()
}

unsafe fn f7<T1, T2>(mut a0: &mut ::std::fs::File, a1: bool) -> *mut ::std::fs::File {
    let __r: &mut ::std::fs::File = &mut a0;
    let __s: *mut ::std::fs::File = __r;
    libcc2rs::write_bool(&mut *__s, a1);
    __s
}

unsafe fn f2<T1, T2>(
    mut a0: &mut ::std::fs::File,
    a1: unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File,
) -> *mut ::std::fs::File {
    let __r: &mut ::std::fs::File = &mut a0;
    let __s: *mut ::std::fs::File = __r;
    a1(__s)
}

unsafe fn f4<T1, T2>(
    mut a0: &mut ::std::fs::File,
    a1: unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File,
) -> *mut ::std::fs::File {
    let __r: &mut ::std::fs::File = &mut a0;
    let __s: *mut ::std::fs::File = __r;
    a1(__s)
}

unsafe fn f5<T1, T2>(mut a0: &mut ::std::fs::File, a1: i64) -> *mut ::std::fs::File {
    let __r: &mut ::std::fs::File = &mut a0;
    let __s: *mut ::std::fs::File = __r;
    libcc2rs::write_int(&mut *__s, a1);
    __s
}

unsafe fn f6<T1, T2>(mut a0: &mut ::std::fs::File, a1: u64) -> *mut ::std::fs::File {
    let __r: &mut ::std::fs::File = &mut a0;
    let __s: *mut ::std::fs::File = __r;
    libcc2rs::write_int(&mut *__s, a1);
    __s
}

unsafe fn f8<T1, T2>(mut a0: &mut ::std::fs::File, a1: i16) -> *mut ::std::fs::File {
    let __r: &mut ::std::fs::File = &mut a0;
    let __s: *mut ::std::fs::File = __r;
    libcc2rs::write_int(&mut *__s, a1);
    __s
}

unsafe fn f9<T1, T2>(mut a0: &mut ::std::fs::File, a1: u16) -> *mut ::std::fs::File {
    let __r: &mut ::std::fs::File = &mut a0;
    let __s: *mut ::std::fs::File = __r;
    libcc2rs::write_int(&mut *__s, a1);
    __s
}

unsafe fn f10<T1, T2>(mut a0: &mut ::std::fs::File, a1: i32) -> *mut ::std::fs::File {
    let __r: &mut ::std::fs::File = &mut a0;
    let __s: *mut ::std::fs::File = __r;
    libcc2rs::write_int(&mut *__s, a1);
    __s
}

unsafe fn f11<T1, T2>(mut a0: &mut ::std::fs::File, a1: u32) -> *mut ::std::fs::File {
    let __r: &mut ::std::fs::File = &mut a0;
    let __s: *mut ::std::fs::File = __r;
    libcc2rs::write_int(&mut *__s, a1);
    __s
}

unsafe fn f12<T1, T2>(mut a0: &mut ::std::fs::File, a1: i64) -> *mut ::std::fs::File {
    let __r: &mut ::std::fs::File = &mut a0;
    let __s: *mut ::std::fs::File = __r;
    libcc2rs::write_int(&mut *__s, a1);
    __s
}

unsafe fn f13<T1, T2>(mut a0: &mut ::std::fs::File, a1: u64) -> *mut ::std::fs::File {
    let __r: &mut ::std::fs::File = &mut a0;
    let __s: *mut ::std::fs::File = __r;
    libcc2rs::write_int(&mut *__s, a1);
    __s
}
