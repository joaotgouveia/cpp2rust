// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn t1<T1>() -> Vec<T1> {
    Default::default()
}

fn t2() -> usize {
    0_usize
}

fn t3<T1>() -> T1 {
    unimplemented!()
}

fn t4<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn t5<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn f1<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0.to_last()
}

fn f2<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f3<T1>(a0: Vec<T1>) -> bool {
    a0.is_empty()
}

fn f4<T1: Default>(a0: &mut Vec<T1>, a1: &mut T1) {
    a0.push(std::mem::take(&mut *a1))
}

fn f5<T1>(a0: &mut Vec<T1>) -> T1 {
    a0.remove(0)
}

fn f7<T1>(a0: &mut Vec<T1>, a1: T1) {
    a0.push(a1)
}

fn f8<T1: Clone>(a0: Vec<T1>) -> Vec<T1> {
    a0.clone()
}

fn f9<T1>(a0: &mut Vec<T1>) -> Vec<T1> {
    std::mem::take(&mut *a0)
}

fn f10<T1: Clone + ByteRepr>(a0: Ptr<Vec<T1>>, a1: Vec<T1>) {
    a0.write(a1)
}

fn f11<T1: ByteRepr + Clone>(a0: Ptr<Vec<T1>>, a1: &mut Vec<T1>) {
    a0.write(std::mem::take(&mut *a1))
}

fn f12<T1>(a0: &mut Vec<T1>, init: T1) {
    let __init = init;
    a0.push(__init)
}

fn f14<T1>() -> Vec<T1> {
    Vec::new()
}
