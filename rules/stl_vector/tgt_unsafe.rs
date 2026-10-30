// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

unsafe fn f3<T1, T2>(a0: *mut Vec<T1>, a1: *mut Vec<T1>) {
    std::ptr::swap(a0, a1)
}
