// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f17<T1>(a0: Vec<T1>) -> *const T1 {
    a0.as_ptr().add(a0.len())
}
