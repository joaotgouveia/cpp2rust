// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f35(a0: Vec<libc::c_char>) -> Vec<libc::c_char> {
    a0
}

fn t2() -> Vec<libc::c_char> {
    vec![0]
}

fn t4() -> Vec<libc::c_char> {
    vec![0]
}

unsafe fn f74<T1>(a0: *const libc::c_char) -> Vec<libc::c_char> {
    let mut __v = ::std::ffi::CStr::from_ptr(a0)
        .to_bytes()
        .iter()
        .map(|&__b| __b as libc::c_char)
        .collect::<Vec<libc::c_char>>();
    __v.push(0);
    __v
}
