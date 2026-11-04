// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

unsafe fn f3<T1: UnsafeCppAdvance>(a0: T1, a1: i64) -> T1 {
    libcc2rs::next_unsafe(a0, a1 as isize)
}

unsafe fn f4<T1: UnsafeCppAdvance>(a0: T1, a1: i64) -> T1 {
    libcc2rs::prev_unsafe(a0, a1 as isize)
}
