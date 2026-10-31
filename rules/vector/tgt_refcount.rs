// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn f124<T1: Clone + ByteRepr>(a0: Ptr<Vec<T1>>, a1: Vec<T1>) {
    a0.write(a1)
}

fn f125<T1: ByteRepr + Clone>(a0: Ptr<Vec<T1>>, a1: &mut Vec<T1>) {
    a0.write(std::mem::take(&mut *a1))
}

fn f129<T1>(a0: Ptr<T1>) -> RandomAccessIterator<T1> {
    a0.begin()
}

fn f130<T1>(a0: Ptr<T1>) -> RandomAccessIterator<T1> {
    a0.begin()
}

fn f131<T1>(a0: Ptr<T1>) -> RandomAccessIterator<T1> {
    a0.end()
}

fn f132<T1>(a0: Ptr<T1>) -> RandomAccessIterator<T1> {
    a0.end()
}

fn f138<T1>(a0: Ptr<T1>) -> RandomAccessIterator<T1> {
    a0.end()
}

fn f151<T1>(a0: Ptr<T1>, a1: usize) -> Ptr<T1> {
    a0.offset(a1 as isize)
}

fn f152<T1: ByteRepr + 'static>(a0: Ptr<Vec<T1>>, a1: usize) -> Ptr<T1> {
    let __idx = a1;
    if __idx as usize >= (*a0.upgrade().deref()).len() {
        panic!("out of bounds access")
    } else {
        a0.decay().offset(__idx as isize)
    }
}

fn f153<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f155<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0.to_last()
}

fn f156<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0.to_last()
}

fn f157<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f158<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f162<T1: ByteRepr>(a0: &mut Vec<T1>, a1: RandomAccessIterator<T1>, a2: T1) -> RandomAccessIterator<T1> {
    let __it = a1;
    let __off = __it.index();
    a0.insert(__off, a2);
    __it
}

fn f163<T1: ByteRepr>(a0: &mut Vec<T1>, a1: RandomAccessIterator<T1>, a2: T1) -> RandomAccessIterator<T1> {
    let __it = a1;
    let __off = __it.index();
    a0.insert(__off, a2);
    __it
}

fn f164<T1: ByteRepr>(a0: &mut Vec<T1>, a1: RandomAccessIterator<T1>, a2: Vec<T1>) -> RandomAccessIterator<T1> {
    let __it = a1;
    let __off = __it.index();
    a0.splice(__off..__off, a2);
    __it
}

fn f165<T1: Clone + ByteRepr>(a0: &mut Vec<T1>, a1: RandomAccessIterator<T1>, a2: usize, a3: T1) -> RandomAccessIterator<T1> {
    let __it = a1;
    let __off = __it.index();
    a0.splice(__off..__off, std::iter::repeat_n(a3, a2));
    __it
}

fn f166<T1: ByteRepr>(a0: Ptr<Vec<T1>>, a1: RandomAccessIterator<T1>) -> RandomAccessIterator<T1> {
    let __it = a1;
    let __off = __it.index();
    a0.with_mut(|__v: &mut Vec<T1>| __v.remove(__off));
    __it
}

fn t8() -> Ptr<bool> {
    Ptr::null()
}

fn t24<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn t25<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn t26<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn t27<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn t28<T1>() -> RandomAccessIterator<T1> {
    RandomAccessIterator::default()
}

fn t29<T1>() -> RandomAccessIterator<T1> {
    RandomAccessIterator::default()
}

fn f149<T1, T2>(a0: Ptr<T1>, a1: usize) -> Ptr<T1> {
    a0.offset(a1 as isize)
}

fn f150<T1: ByteRepr + 'static, T2>(a0: Ptr<Vec<T1>>, a1: usize) -> Ptr<T1> {
    let __idx = a1;
    if __idx as usize >= (*a0.upgrade().deref()).len() {
        panic!("out of bounds access")
    } else {
        a0.decay().offset(__idx as isize)
    }
}

fn f170<T1: CppFrom<T5>, T2, T3: CppInputIterator<T5>, T4, T5: ByteRepr>(
    a0: T3,
    a1: T3,
    a2: T2,
) -> Vec<T1> {
    let mut __v = Vec::new();
    let mut __it = a0;
    while __it != a1 {
        __v.push(T1::cpp_from(__it.iter_deref()));
        __it.iter_inc();
    }
    __v
}

fn f176<T1: CppFrom<T5> + ByteRepr, T2, T3: CppInputIterator<T5>, T4, T5: ByteRepr>(
    a0: Ptr<Vec<T1>>,
    a1: RandomAccessIterator<T1>,
    a2: T3,
    a3: T3,
) -> RandomAccessIterator<T1> {
    let __pos = a1;
    let __start = __pos.index();
    let mut __tmp: Vec<T1> = Vec::new();
    let mut __it = a2;
    while __it != a3 {
        __tmp.push(T1::cpp_from(__it.iter_deref()));
        __it.iter_inc();
    }
    a0.with_mut(|__v: &mut Vec<T1>| {
        __v.splice(__start..__start, __tmp);
    });
    __pos
}

fn t10<T1>() -> RandomAccessIterator<bool> {
    RandomAccessIterator::default()
}

fn t11<T1>() -> RandomAccessIterator<bool> {
    RandomAccessIterator::default()
}

fn t6<T1>() -> Ptr<bool> {
    Ptr::null()
}

fn t15<T1>() -> Ptr<bool> {
    Ptr::null()
}

fn f34<T1>(a0: Ptr<bool>, a1: usize) -> Ptr<bool> {
    a0.offset(a1 as isize)
}

fn f67<T1>(a0: Ptr<bool>) -> bool {
    a0.read()
}

fn f66<T1>(a0: Ptr<bool>) -> Ptr<bool> {
    a0
}

fn f18<T1>(a0: Ptr<bool>) -> RandomAccessIterator<bool> {
    a0.begin()
}

fn f68<T1>(a0: Ptr<bool>, a1: bool) -> Ptr<bool> {
    a0.write(a1);
    a0
}

fn f69<T1>(a0: Ptr<bool>, a1: bool) -> Ptr<bool> {
    a0.write(a1);
    a0
}

fn f70<T1>(a0: Ptr<bool>, a1: Ptr<bool>) -> Ptr<bool> {
    a0.write(a1.read());
    a0
}

fn f71<T1>(a0: Ptr<bool>, a1: Ptr<bool>) -> bool {
    a0.read() == a1.read()
}

fn f72<T1>(a0: Ptr<bool>, a1: Ptr<bool>) -> bool {
    !a0.read() & a1.read()
}

fn f73<T1>(a0: Ptr<bool>) {
    a0.write(!a0.read());
}

fn f74<T1>(a0: Ptr<bool>, a1: Ptr<bool>) {
    let __tmp = a0.read();
    a0.write(a1.read());
    a1.write(__tmp);
}

fn f75<T1>(a0: Ptr<bool>, a1: Ptr<bool>) {
    let __tmp = a0.read();
    a0.write(a1.read());
    a1.write(__tmp);
}

fn f76<T1>(a0: Ptr<bool>, a1: Ptr<bool>) {
    let __tmp = a0.read();
    a0.write(a1.read());
    a1.write(__tmp);
}

fn f77<T1>() -> RandomAccessIterator<bool> {
    RandomAccessIterator::default()
}

fn f79<T1>(a0: &mut RandomAccessIterator<bool>) -> RandomAccessIterator<bool> {
    a0.iter_inc()
}

fn f80<T1>(a0: &mut RandomAccessIterator<bool>, a1: i32) -> RandomAccessIterator<bool> {
    a0.iter_post_inc()
}

fn f81<T1>(a0: &mut RandomAccessIterator<bool>) -> RandomAccessIterator<bool> {
    a0.iter_dec()
}

fn f82<T1>(a0: &mut RandomAccessIterator<bool>, a1: i32) -> RandomAccessIterator<bool> {
    a0.iter_post_dec()
}

fn f83<T1>(a0: &mut RandomAccessIterator<bool>, a1: isize) -> RandomAccessIterator<bool> {
    a0.iter_add_assign(a1)
}

fn f84<T1>(a0: &mut RandomAccessIterator<bool>, a1: isize) -> RandomAccessIterator<bool> {
    a0.iter_add_assign(-a1)
}

fn f85<T1>(a0: RandomAccessIterator<bool>) -> RandomAccessIterator<bool> {
    a0
}

fn f86<T1>(a0: RandomAccessIterator<bool>) -> RandomAccessIterator<bool> {
    a0
}

fn f87<T1>(a0: &mut RandomAccessIterator<bool>, a1: RandomAccessIterator<bool>) -> RandomAccessIterator<bool> {
    *a0 = a1;
    a0.clone()
}

fn f88<T1>(a0: &mut RandomAccessIterator<bool>, a1: RandomAccessIterator<bool>) -> RandomAccessIterator<bool> {
    *a0 = a1;
    a0.clone()
}

fn f89<T1>(a0: RandomAccessIterator<bool>, a1: isize) -> RandomAccessIterator<bool> {
    a0.iter_add(-a1)
}

fn f90<T1>(a0: isize, a1: RandomAccessIterator<bool>) -> RandomAccessIterator<bool> {
    a1.iter_add(a0)
}

fn f91<T1>(a0: RandomAccessIterator<bool>, a1: isize) -> RandomAccessIterator<bool> {
    a0.iter_add(a1)
}

fn f92<T1>(a0: RandomAccessIterator<bool>, a1: RandomAccessIterator<bool>) -> isize {
    a0.iter_distance(&a1)
}

fn f19<T1>(a0: Ptr<bool>) -> RandomAccessIterator<bool> {
    a0.begin()
}

fn f20<T1>(a0: Ptr<bool>) -> RandomAccessIterator<bool> {
    a0.end()
}

fn f21<T1>(a0: Ptr<bool>) -> RandomAccessIterator<bool> {
    a0.end()
}

fn f26<T1>(a0: Ptr<bool>) -> RandomAccessIterator<bool> {
    a0.begin()
}

fn f27<T1>(a0: Ptr<bool>) -> RandomAccessIterator<bool> {
    a0.end()
}

fn f93<T1>(a0: RandomAccessIterator<bool>, a1: RandomAccessIterator<bool>) -> std::cmp::Ordering {
    a0.partial_cmp(&a1).unwrap()
}

fn f94<T1>(a0: RandomAccessIterator<bool>, a1: RandomAccessIterator<bool>) -> bool {
    a0 == a1
}

fn f95<T1>() -> RandomAccessIterator<bool> {
    RandomAccessIterator::default()
}

fn f97<T1>(a0: RandomAccessIterator<bool>) -> bool {
    a0.iter_deref().read()
}

fn f98<T1>(a0: &mut RandomAccessIterator<bool>) -> RandomAccessIterator<bool> {
    a0.iter_inc()
}

fn f99<T1>(a0: &mut RandomAccessIterator<bool>, a1: i32) -> RandomAccessIterator<bool> {
    a0.iter_post_inc()
}

fn f100<T1>(a0: &mut RandomAccessIterator<bool>) -> RandomAccessIterator<bool> {
    a0.iter_dec()
}

fn f101<T1>(a0: &mut RandomAccessIterator<bool>, a1: i32) -> RandomAccessIterator<bool> {
    a0.iter_post_dec()
}

fn f102<T1>(a0: &mut RandomAccessIterator<bool>, a1: isize) -> RandomAccessIterator<bool> {
    a0.iter_add_assign(a1)
}

fn f103<T1>(a0: &mut RandomAccessIterator<bool>, a1: isize) -> RandomAccessIterator<bool> {
    a0.iter_add_assign(-a1)
}

fn f104<T1>(a0: RandomAccessIterator<bool>, a1: isize) -> bool {
    a0.iter_at(a1).read()
}

fn f105<T1>(a0: RandomAccessIterator<bool>) -> RandomAccessIterator<bool> {
    a0
}

fn f106<T1>(a0: RandomAccessIterator<bool>) -> RandomAccessIterator<bool> {
    a0
}

fn f107<T1>(a0: &mut RandomAccessIterator<bool>, a1: RandomAccessIterator<bool>) -> RandomAccessIterator<bool> {
    *a0 = a1;
    a0.clone()
}

fn f108<T1>(a0: &mut RandomAccessIterator<bool>, a1: RandomAccessIterator<bool>) -> RandomAccessIterator<bool> {
    *a0 = a1;
    a0.clone()
}

fn f109<T1>(a0: isize, a1: RandomAccessIterator<bool>) -> RandomAccessIterator<bool> {
    a1.iter_add(a0)
}

fn f110<T1>(a0: RandomAccessIterator<bool>, a1: isize) -> RandomAccessIterator<bool> {
    a0.iter_add(-a1)
}

fn f111<T1>(a0: RandomAccessIterator<bool>, a1: isize) -> RandomAccessIterator<bool> {
    a0.iter_add(a1)
}

fn f112<T1>(a0: RandomAccessIterator<bool>, a1: RandomAccessIterator<bool>) -> isize {
    a0.iter_distance(&a1)
}

fn f113<T1>(a0: RandomAccessIterator<bool>, a1: RandomAccessIterator<bool>) -> std::cmp::Ordering {
    a0.partial_cmp(&a1).unwrap()
}

fn f114<T1>(a0: RandomAccessIterator<bool>, a1: RandomAccessIterator<bool>) -> bool {
    a0 == a1
}

fn t17<T1>() -> RandomAccessIterator<bool> {
    RandomAccessIterator::default()
}

fn t18<T1>() -> bool {
    false
}

fn t19<T1>() -> bool {
    false
}

fn t21<T1>() -> RandomAccessIterator<bool> {
    RandomAccessIterator::default()
}

fn t20<T1>() -> Ptr<bool> {
    Ptr::null()
}

fn f6<T1>(a0: usize, a1: T1) -> Vec<bool> {
    vec![false; a0]
}

fn f57<T1, T2: CppInputIterator<T4>, T3, T4: ByteRepr + Clone + PartialEq + Default>(a0: T2, a1: T2, a2: T1) -> Vec<bool> {
    let mut __v = Vec::new();
    let mut __it = a0;
    while __it != a1 {
        __v.push(__it.iter_deref().read() != T4::default());
        __it.iter_inc();
    }
    __v
}
