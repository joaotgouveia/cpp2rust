// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn t5<T1>() -> Vec<T1> {
    Default::default()
}

fn t9() -> usize {
    0_usize
}

fn t10<T1>() -> T1 {
    unimplemented!()
}

fn t11<T1>() -> Vec<T1> {
    Vec::new()
}

fn t12() -> bool {
    false
}

fn t13<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn t14<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn f38(a0: usize, a1: bool) -> Vec<bool> {
    (0..(a0) as usize).map(|_| a1).collect::<Vec<bool>>()
}

fn f40<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0.to_end()
}

fn f47(a0: Vec<bool>) -> Vec<bool> {
    a0
}

fn f60<T1: ByteRepr>(a0: Ptr<Vec<T1>>, a1: RandomAccessIterator<T1>) -> RandomAccessIterator<T1> {
    let __it = a1;
    let __off = __it.index();
    a0.with_mut(|__v: &mut Vec<T1>| __v.remove(__off));
    __it
}

fn f61<T1>(a0: Vec<T1>) -> usize {
    a0.len()
}

fn f62<T1>(a0: Vec<T1>) -> bool {
    a0.is_empty()
}

fn f63<T1>() -> Vec<T1> {
    Vec::new()
}

fn f64<T1>(a0: &mut Vec<T1>) {
    a0.pop();
}

fn f65<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f66<T1>(a0: Ptr<T1>, a1: usize) -> Ptr<T1> {
    a0.offset(a1 as isize)
}

fn f67<T1: Default>(a0: usize) -> Vec<T1> {
    (0..(a0) as usize)
        .map(|_| <T1>::default())
        .collect::<Vec<_>>()
}

fn f68<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f69<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0.to_last()
}

fn f70<T1>(a0: Vec<T1>) -> usize {
    a0.capacity()
}

fn f71<T1>(a0: &mut Vec<T1>, a1: usize) {
    if a1 as usize > a0.capacity() as usize {
        let len_0 = a0.len();
        a0.reserve_exact(a1 as usize - len_0 as usize);
    }
}

fn f72<T1>(a0: Ptr<T1>) -> RandomAccessIterator<T1> {
    a0.begin()
}

fn f73<T1: Default>(a0: &mut Vec<T1>, a1: &mut T1) {
    let __a1 = std::mem::take(&mut *a1);
    a0.push(__a1)
}

fn f74<T1: Default>(a0: &mut Vec<T1>, a1: usize) {
    let __a0 = a1 as usize;
    a0.resize_with(__a0, || <T1>::default())
}

fn f75<T1>(a0: &mut Vec<T1>) {
    a0.clear()
}

fn f76<T1>(a0: Ptr<T1>) -> RandomAccessIterator<T1> {
    a0.end()
}

fn f77<T1: ByteRepr>(
    a0: &mut Vec<T1>,
    a1: RandomAccessIterator<T1>,
    a2: T1,
) -> RandomAccessIterator<T1> {
    let __it = a1;
    let __off = __it.index();
    a0.insert(__off, a2);
    __it
}

fn f78<T1: Default + Clone>(a0: usize, a1: T1) -> Vec<T1> {
    vec![a1; a0 as usize]
}

fn f79<T1: ByteRepr>(
    a0: &mut Vec<T1>,
    a1: RandomAccessIterator<T1>,
    a2: T1,
) -> RandomAccessIterator<T1> {
    let __it = a1;
    let __off = __it.index();
    a0.insert(__off, a2);
    __it
}

fn f80<T1: Clone>(a0: &mut Vec<T1>, a1: T1) {
    let a0_clone = a1.clone();
    a0.push(a0_clone)
}

fn f81<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f82<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f83<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f84<T1>(a0: Ptr<T1>, a1: usize) -> Ptr<T1> {
    a0.offset(a1 as isize)
}

fn f85<T1>(a0: Ptr<T1>, a1: Ptr<T1>) -> bool {
    a0 != a1
}

fn f86<T1>(a0: Ptr<T1>, a1: Ptr<T1>) -> bool {
    a0 == a1
}

fn f87<T1>(a0: &mut Ptr<T1>) -> Ptr<T1> {
    a0.postfix_inc()
}

fn f88<T1>(a0: Ptr<T1>, a1: Ptr<T1>) -> isize {
    ((a0.get_offset() as isize) - (a1.get_offset() as isize))
}

fn f89<T1>(a0: &mut Ptr<T1>) -> Ptr<T1> {
    a0.prefix_inc()
}

fn f91<T1>(a0: Vec<T1>) -> Vec<T1> {
    a0
}

fn f92<T1: TryFrom<T2>, T2: Clone + ByteRepr, T3: CppInputIterator<T2>>(a0: T3, a1: T3) -> Vec<T1> {
    let mut __v = Vec::new();
    let mut __it = a0;
    while __it != a1 {
        __v.push(<T1>::try_from(__it.iter_deref().read()).ok().unwrap());
        __it.iter_inc();
    }
    __v
}

fn f93<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f95<T1>(a0: Ptr<T1>) -> RandomAccessIterator<T1> {
    a0.begin()
}

fn f96<T1>(a0: Ptr<T1>) -> RandomAccessIterator<T1> {
    a0.end()
}

fn f97(a0: &mut Vec<bool>, a1: &mut Vec<bool>) {
    std::mem::swap(&mut *a0, &mut *a1)
}

fn f98<T1: ByteRepr + 'static>(a0: Ptr<Vec<T1>>, a1: usize) -> Ptr<T1> {
    let __idx = a1;
    if __idx as usize >= (*a0.upgrade().deref()).len() {
        panic!("out of bounds access")
    } else {
        a0.decay().offset(__idx as isize)
    }
}

fn f99<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0.to_last()
}

fn f101<T1: Clone + ByteRepr, T2, T3: CppInputIterator<T1>>(
    a0: Ptr<Vec<T1>>,
    a1: RandomAccessIterator<T1>,
    a2: T3,
    a3: T3,
) -> RandomAccessIterator<T1> {
    let __pos = a1;
    let start_idx = __pos.index();
    let mut temp_vec: Vec<T1> = Vec::new();
    let mut __it = a2;
    while __it != a3 {
        temp_vec.push(__it.iter_deref().read());
        __it.iter_inc();
    }
    a0.with_mut(|v: &mut Vec<T1>| {
        v.splice(start_idx..start_idx, temp_vec);
    });
    __pos
}

fn f102<T1: Default + Clone>(a0: &mut Vec<T1>, a1: usize, a2: T1) {
    let __a0 = a1 as usize;
    a0.resize(__a0, a2)
}

fn f103<T1: ByteRepr + Clone>(a0: Ptr<Vec<T1>>, a1: &mut Vec<T1>) {
    a0.write(std::mem::take(&mut *a1))
}

fn f104<T1>(a0: Ptr<T1>) -> RandomAccessIterator<T1> {
    a0.end()
}

fn f105<T1: Clone + ByteRepr>(a0: Ptr<Vec<T1>>, a1: Vec<T1>) {
    a0.write(a1)
}

fn f106<T1>(a0: &mut Vec<T1>) {
    a0.shrink_to_fit()
}

fn f108<T1>(a0: &mut Vec<T1>) -> Vec<T1> {
    std::mem::take(&mut *a0)
}

fn f110<T1: Clone>(a0: Vec<T1>) -> Vec<T1> {
    a0.clone()
}

fn f114<T1>(a0: &mut Vec<T1>, init: T1) {
    let __init = init;
    a0.push(__init)
}

fn f115<T1: Clone + ByteRepr>(
    a0: &mut Vec<T1>,
    a1: RandomAccessIterator<T1>,
    a2: usize,
    a3: T1,
) -> RandomAccessIterator<T1> {
    let __it = a1;
    let __off = __it.index();
    a0.splice(__off..__off, std::iter::repeat_n(a3, a2));
    __it
}

fn f116<T1: ByteRepr>(
    a0: &mut Vec<T1>,
    a1: RandomAccessIterator<T1>,
    a2: Vec<T1>,
) -> RandomAccessIterator<T1> {
    let __it = a1;
    let __off = __it.index();
    let __off = a1.index();
    a0.splice(__off..__off, a2);
    __it
}
