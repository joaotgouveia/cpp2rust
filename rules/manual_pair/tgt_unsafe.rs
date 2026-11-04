// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

unsafe fn f15<T1, T2>(a0: (T1, T2)) -> T2 {
    a0.1
}

unsafe fn f16<T1, T2>(a0: (T1, T2)) -> T1 {
    a0.0
}

unsafe fn f17<T1: CppFromUnsafe<T3>, T2: CppFromUnsafe<T4>, T3, T4>(a0: &mut (T1, T2), a1: &mut (T3, T4)) {
    *a0 = (T1::cpp_from(&a1.0), T2::cpp_from(&a1.1))
}
