// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t2() -> ::std::fs::File {
    ::std::fs::File::open("").unwrap()
}

unsafe fn f2(mut a0: &mut ::std::fs::File) -> *mut ::std::fs::File {
    let __r: &mut ::std::fs::File = &mut a0;
    libcc2rs::dec_unsafe(__r)
}

unsafe fn f5(mut a0: &mut ::std::fs::File) -> *mut ::std::fs::File {
    let __r: &mut ::std::fs::File = &mut a0;
    libcc2rs::hex_unsafe(__r)
}

unsafe fn f81(mut a0: &mut ::std::fs::File) -> *mut ::std::fs::File {
    let __r: &mut ::std::fs::File = &mut a0;
    libcc2rs::oct_unsafe(__r)
}
