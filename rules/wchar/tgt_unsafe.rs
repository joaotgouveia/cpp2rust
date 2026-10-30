// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t1() -> ::libc::mbstate_t {
    unsafe { std::mem::zeroed() }
}

#[cfg(target_os = "linux")]
fn t3() -> ::libc::mbstate_t {
    unsafe { std::mem::zeroed() }
}
