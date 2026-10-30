// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

unsafe fn f1<T1: Default>(mut a0: &mut ::std::fs::File) -> UnsafeOstreamIterator<T1> {
    let __r: &mut ::std::fs::File = &mut a0;
    UnsafeOstreamIterator::new(__r, ::std::ptr::null())
}

unsafe fn f2<T1: Default>(
    mut a0: &mut ::std::fs::File,
    a1: *const libc::c_char,
) -> UnsafeOstreamIterator<T1> {
    let __r: &mut ::std::fs::File = &mut a0;
    UnsafeOstreamIterator::new(__r, a1)
}

unsafe fn f3<T1>(a0: UnsafeOstreamIterator<T1>) -> UnsafeOstreamIterator<T1> {
    a0.clone()
}

fn t1<T1: Default>() -> UnsafeOstreamIterator<T1> {
    UnsafeOstreamIterator::default()
}

fn t5() -> ::std::fs::File {
    ::std::fs::File::open("").unwrap()
}
