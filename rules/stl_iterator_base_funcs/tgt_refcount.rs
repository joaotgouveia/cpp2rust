// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f3<T1: CppAdvance + 'static>(a0: T1, a1: i64) -> T1 {
    libcc2rs::next_refcount(a0, a1 as isize)
}

fn f4<T1: CppAdvance + 'static>(a0: T1, a1: i64) -> T1 {
    libcc2rs::prev_refcount(a0, a1 as isize)
}
