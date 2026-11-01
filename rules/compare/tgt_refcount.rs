// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1() -> std::cmp::Ordering {
    std::cmp::Ordering::Equal
}

fn f1() -> Ptr<std::cmp::Ordering> {
    libcc2rs::strong_ordering_less()
}

fn f2() -> Ptr<std::cmp::Ordering> {
    libcc2rs::strong_ordering_equal()
}

fn f3() -> Ptr<std::cmp::Ordering> {
    libcc2rs::strong_ordering_equivalent()
}

fn f4() -> Ptr<std::cmp::Ordering> {
    libcc2rs::strong_ordering_greater()
}

fn f5(a0: std::cmp::Ordering, a1: std::cmp::Ordering) -> bool {
    a0 == a1
}

fn f6(a0: std::cmp::Ordering, a1: i32) -> bool {
    a0 == std::cmp::Ordering::Equal
}

fn f7(a0: std::cmp::Ordering, a1: i32) -> bool {
    a0 == std::cmp::Ordering::Less
}

fn f8(a0: std::cmp::Ordering, a1: i32) -> bool {
    a0 == std::cmp::Ordering::Greater
}

fn f9(a0: std::cmp::Ordering, a1: i32) -> bool {
    a0 != std::cmp::Ordering::Greater
}

fn f10(a0: std::cmp::Ordering, a1: i32) -> bool {
    a0 != std::cmp::Ordering::Less
}
