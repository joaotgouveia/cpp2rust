// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t5() -> isize {
    0_isize
}

fn t4<T1>() -> T1 {
    unimplemented!()
}

fn t6<T1>() -> *mut T1 {
    ::std::ptr::null_mut()
}

fn t7<T1>() -> *mut T1 {
    ::std::ptr::null_mut()
}
