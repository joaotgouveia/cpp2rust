// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

unsafe fn f3<T1, T2, T3, T4>(a0: *mut UnsafeMap<T1, (T1, T2)>, a1: *mut UnsafeMap<T1, (T1, T2)>) {
    std::ptr::swap(a0, a1)
}
