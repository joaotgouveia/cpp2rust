// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

unsafe fn f35(a0: Vec<libc::c_char>) -> Vec<libc::c_char> {
    a0
}

fn t2() -> Vec<libc::c_char> {
    vec![0]
}

fn t4() -> Vec<libc::c_char> {
    vec![0]
}

unsafe fn f74<T1: PathSource>(a0: &T1) -> Vec<libc::c_char> {
    let mut __v: Vec<libc::c_char> = a0
        .path_bytes()
        .into_iter()
        .map(|__b| __b as libc::c_char)
        .collect();
    __v.push(0);
    __v
}
