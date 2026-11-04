// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn f10<T1: CChar, T2, T3>() -> Vec<T1> {
    vec![T1::default()]
}

fn f12<T1: CChar, T2, T3>(a0: Vec<T1>) -> Vec<T1> {
    a0
}

fn f17<T1: CChar, T2, T3>(a0: &mut Vec<T1>) -> Vec<T1> {
    std::mem::take(&mut *a0)
}

fn f21<T1: CChar, T2, T3>(a0: Ptr<Vec<T1>>, a1: Vec<T1>) {
    a0.write(a1)
}

fn f24<T1: CChar, T2, T3>(a0: Ptr<Vec<T1>>, a1: &mut Vec<T1>) {
    a0.write(std::mem::take(&mut *a1))
}

fn f27<T1: CChar, T2, T3>(a0: Ptr<T1>) -> RandomAccessIterator<T1> {
    RandomAccessIterator::Vec(a0)
}

fn f29<T1: CChar, T2, T3>(a0: Ptr<T1>) -> RandomAccessIterator<T1> {
    RandomAccessIterator::Vec(a0.to_last())
}

fn f39<T1: CChar, T2, T3>(a0: Vec<T1>) -> usize {
    (a0.len() - 1)
}

fn f40<T1: CChar, T2, T3>(a0: Vec<T1>) -> usize {
    (a0.len() - 1)
}

fn f43<T1: CChar, T2, T3>(a0: &mut Vec<T1>, a1: usize) {
    a0.pop();
    a0.resize((a1) as usize, T1::default());
    a0.push(T1::default())
}

fn f44<T1: CChar, T2, T3>(a0: &mut Vec<T1>) {
    a0.shrink_to_fit()
}

fn f48<T1: CChar, T2, T3>(a0: &mut Vec<T1>) {
    a0.clear();
    a0.push(T1::default())
}

fn f49<T1: CChar, T2, T3>(a0: Vec<T1>) -> bool {
    a0.len() <= 1
}

fn f53<T1: CChar, T2, T3>(a0: Ptr<Vec<T1>>, a1: usize) -> Ptr<T1> {
    if a1 as usize >= (*a0.upgrade().deref()).len().saturating_sub(1) {
        panic!("out of bounds access")
    } else {
        a0.decay().offset(a1 as isize)
    }
}

fn f64<T1: CChar, T2, T3>(a0: Ptr<Vec<T1>>, a1: Ptr<T1>, a2: usize) -> Ptr<Vec<T1>> {
    a0.with_mut(|__v: &mut Vec<T1>| {
        __v.pop();
        a1.with_slice(a2 as usize, |__s| __v.extend_from_slice(__s));
        __v.push(T1::default());
    });
    a0
}

fn f66<T1: CChar, T2, T3>(a0: &mut Vec<T1>, a1: usize, a2: T1) -> Vec<T1> {
    a0.pop();
    let __len = a0.len() + (a1) as usize;
    a0.resize(__len, a2);
    a0.push(T1::default());
    a0.clone()
}

fn f90<T1: CChar, T2, T3>(a0: Ptr<Vec<T1>>, a1: usize, a2: usize, a3: Ptr<T1>, a4: usize) -> Ptr<Vec<T1>> {
    let __pos = a1 as usize;
    let __end = std::cmp::min(
        __pos + a2 as usize,
        (*a0.upgrade().deref()).len().saturating_sub(1),
    );
    a0.with_mut(|__v: &mut Vec<T1>| {
        a3.with_slice(a4 as usize, |__s| {
            __v.splice(__pos..__end, __s.iter().copied());
        });
    });
    a0
}

fn f104<T1: CChar, T2, T3>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f105<T1: CChar, T2, T3>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f106<T1: CChar, T2, T3>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f110<T1: CChar, T2, T3>(a0: Vec<T1>, a1: Ptr<T1>, a2: usize) -> usize {
    let __hay = &a0[..a0.len().saturating_sub(1)];
    a1.with_c_str(|__needle| {
        (a2..=__hay.len().saturating_sub(__needle.len()))
            .find(|&__i| __hay[__i..].starts_with(__needle))
            .unwrap_or(usize::MAX)
    })
}

fn f132<T1: CChar, T2, T3>(a0: Ptr<Vec<T1>>, a1: usize, a2: usize) -> Vec<T1> {
    a0.with(|__s: &Vec<T1>| {
        let mut __tmp1 = __s[a1..::std::cmp::min(a1 + a2, __s.len().saturating_sub(1))].to_vec();
        __tmp1.push(T1::default());
        __tmp1
    })
}

fn f189<T1: CChar, T2, T3>(a0: Vec<T1>, a1: Ptr<T1>) -> Vec<T1> {
    let mut __tmp2 = a0;
    __tmp2.pop();
    a1.with_c_str(|__s| __tmp2.extend_from_slice(__s));
    __tmp2.push(T1::default());
    __tmp2
}

fn f193<T1: CChar, T2, T3>(a0: Vec<T1>, a1: Ptr<T1>) -> Vec<T1> {
    let mut __r = a0;
    __r.pop();
    a1.with_c_str(|__s| __r.extend_from_slice(__s));
    __r.push(T1::default());
    __r
}

fn t1<T1: CChar, T2, T3>() -> Vec<T1> {
    vec![T1::default()]
}

fn t3<T1: CChar, T2, T3>() -> T1 {
    T1::default()
}

fn t7<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn t8<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn t9<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn t11<T1>() -> RandomAccessIterator<T1> {
    RandomAccessIterator::default()
}

fn t12<T1>() -> RandomAccessIterator<T1> {
    RandomAccessIterator::default()
}

fn f148<T1: CChar, T2, T3>(a0: Ptr<T1>, a1: T3) -> Vec<T1> {
    let mut __bytes = a0.to_c_bytes();
    __bytes.push(T1::default());
    __bytes
}

fn f16<T1: CChar, T2, T3>(a0: Ptr<T1>, a1: usize, a2: T3) -> Vec<T1> {
    let mut __v = Vec::with_capacity(a1 as usize + 1);
    a0.with_slice(a1 as usize, |__s| __v.extend_from_slice(__s));
    __v.push(T1::default());
    __v
}

fn f149<T1: CChar, T2, T3>(a0: usize, a1: T1, a2: T3) -> Vec<T1> {
    vec![a1; (a0) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(T1::default()))
        .collect()
}

fn f122<T1: CChar, T2, T3>(a0: Vec<T1>, a1: Ptr<T1>, a2: usize) -> usize {
    let __n = a0.len().saturating_sub(1);
    let __end = a2.saturating_add(1).min(__n);
    a1.with_c_str(|__lookup| {
        a0.iter()
            .take(__end)
            .rposition(|__x| __lookup.contains(__x))
            .unwrap_or(usize::MAX)
    })
}

fn f203<T1: CChar, T2, T3>(a0: Vec<T1>, a1: Ptr<T1>) -> bool {
    a1.with_c_str(|__s| a0[..a0.len().saturating_sub(1)] == *__s)
}

fn f51<T1: CChar, T2, T3>(a0: Ptr<T1>, a1: usize) -> Ptr<T1> {
    a0.offset(a1 as isize)
}

fn f200<T1: StreamChar, T2, T3>(a0: Ptr<::std::fs::File>, a1: Vec<T1>) -> Ptr<::std::fs::File> {
    let __s = a0;
    let mut __bytes = Vec::new();
    for &__c in &a1[..a1.len().saturating_sub(1)] {
        __c.push_stream_bytes(&mut __bytes);
    }
    let _ = __s.write_all(&__bytes);
    __s
}

fn f151<T1, T2, T3, T4, T5>(a0: std::fs::File, a1: std::fs::File, a2: T3) -> Vec<i8> {
    use std::io::Read;
    let mut __bytes: Vec<u8> = Vec::new();
    let mut __f = &a0;
    __f.read_to_end(&mut __bytes)
        .expect("couldn't read the file");
    __bytes.push(0);
    CChar::from_byte_vec(__bytes)
}
