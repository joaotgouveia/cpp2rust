// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t6() -> std::cmp::Ordering {
    std::cmp::Ordering::Equal
}

unsafe fn f35(a0: std::cmp::Ordering) -> std::cmp::Ordering {
    a0
}

unsafe fn f36(a0: std::cmp::Ordering) -> std::cmp::Ordering {
    a0
}

unsafe fn f45<T1>(a0: std::cmp::Ordering, a1: T1) -> bool {
    a0 != std::cmp::Ordering::Less
}

unsafe fn f46<T1>(a0: std::cmp::Ordering, a1: T1) -> bool {
    a0 != std::cmp::Ordering::Greater
}

unsafe fn f47<T1>(a0: std::cmp::Ordering, a1: T1) -> bool {
    a0 == std::cmp::Ordering::Greater
}

unsafe fn f48<T1>(a0: std::cmp::Ordering, a1: T1) -> bool {
    a0 == std::cmp::Ordering::Less
}

unsafe fn f49(a0: std::cmp::Ordering, a1: std::cmp::Ordering) -> bool {
    a0 == a1
}

unsafe fn f50<T1>(a0: std::cmp::Ordering, a1: T1) -> bool {
    a0 == std::cmp::Ordering::Equal
}

unsafe fn f51() -> *const std::cmp::Ordering {
    &std::cmp::Ordering::Less
}

unsafe fn f52() -> *const std::cmp::Ordering {
    &std::cmp::Ordering::Equal
}

unsafe fn f53() -> *const std::cmp::Ordering {
    &std::cmp::Ordering::Equal
}

unsafe fn f54() -> *const std::cmp::Ordering {
    &std::cmp::Ordering::Greater
}
