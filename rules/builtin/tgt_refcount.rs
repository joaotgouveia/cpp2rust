// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f1(a0: usize, a1: usize) -> usize {
    a0
}

fn f2(a0: u32) -> i32 {
    a0.trailing_zeros() as i32
}

fn f3(a0: u32) -> i32 {
    a0.leading_zeros() as i32
}

fn f7(a0: u64) -> i32 {
    a0.trailing_zeros() as i32
}

fn f8(a0: u64) -> i32 {
    a0.count_ones() as i32
}

fn f9(a0: i64, a1: i64, a2: Ptr<i64>) -> bool {
    let (val, ovf) = a0.overflowing_mul(a1);
    a2.write(val);
    ovf
}

fn f10(a0: i64, a1: i64, a2: Ptr<i64>) -> bool {
    let (val, ovf) = a0.overflowing_mul(a1);
    a2.write(val);
    ovf
}

fn f11() {
    std::hint::spin_loop();
}

fn f12(a0: i64, a1: i64, a2: Ptr<i64>) -> bool {
    let (val, ovf) = a0.overflowing_mul(a1);
    a2.write(val);
    ovf
}

fn f13(a0: i64, a1: i64, a2: Ptr<i64>) -> bool {
    let (val, ovf) = a0.overflowing_mul(a1);
    a2.write(val);
    ovf
}

fn f14(a0: AnyPtr, a1: AnyPtr, a2: usize) -> AnyPtr {
    a0.memcpy(&a1, a2 as usize);
    a0
}

fn f15() {
    std::process::abort()
}

fn f16(a0: u64) -> i32 {
    a0.leading_zeros() as i32
}

fn f17() -> f32 {
    f32::INFINITY
}

fn f19() -> bool {
    false
}
