// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::io::Read;

unsafe fn f10<T1: CChar, T2, T3>() -> Vec<T1> {
    vec![T1::default()]
}

unsafe fn f12<T1: CChar, T2, T3>(a0: Vec<T1>) -> Vec<T1> {
    a0.clone()
}

unsafe fn f17<T1: CChar, T2, T3>(a0: &mut Vec<T1>) -> Vec<T1> {
    std::mem::take(&mut *a0)
}

unsafe fn f21<T1: CChar, T2, T3>(a0: &mut Vec<T1>, a1: Vec<T1>) {
    *a0 = a1.clone()
}

unsafe fn f24<T1: CChar, T2, T3>(a0: &mut Vec<T1>, a1: &mut Vec<T1>) {
    *a0 = std::mem::take(&mut *a1)
}

unsafe fn f27<T1: CChar, T2, T3>(a0: &mut Vec<T1>) -> UnsafeRandomAccessIterator<T1> {
    UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr())
}

unsafe fn f29<T1: CChar, T2, T3>(a0: &mut Vec<T1>) -> UnsafeRandomAccessIterator<T1> {
    UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr().add(a0.len() - 1))
}

unsafe fn f39<T1: CChar, T2, T3>(a0: Vec<T1>) -> usize {
    (a0.len() - 1)
}

unsafe fn f40<T1: CChar, T2, T3>(a0: Vec<T1>) -> usize {
    (a0.len() - 1)
}

unsafe fn f43<T1: CChar, T2, T3>(a0: &mut Vec<T1>, a1: usize) {
    a0.pop();
    a0.resize((a1) as usize, T1::default());
    a0.push(T1::default())
}

unsafe fn f44<T1: CChar, T2, T3>(a0: &mut Vec<T1>) {
    a0.shrink_to_fit()
}

unsafe fn f48<T1: CChar, T2, T3>(a0: &mut Vec<T1>) {
    a0.clear();
    a0.push(T1::default())
}

unsafe fn f49<T1: CChar, T2, T3>(a0: Vec<T1>) -> bool {
    Vec::len(&a0) <= 1
}

unsafe fn f53<T1: CChar, T2, T3>(mut a0: &mut Vec<T1>, a1: usize) -> *mut T1 {
    if a1 as usize >= (&mut a0).len() - 1 {
        panic!("out of bounds access")
    } else {
        (&mut a0).as_mut_ptr().add(a1 as usize)
    }
}

unsafe fn f64<T1: CChar, T2, T3>(a0: &mut Vec<T1>, a1: *mut T1, a2: usize) {
    a0.splice(a0.len().saturating_sub(1)..a0.len(), {
        let mut v = ::std::slice::from_raw_parts(a1, a2 as usize).to_vec();
        v.push(T1::default());
        v
    });
}

unsafe fn f66<T1: CChar, T2, T3>(a0: &mut Vec<T1>, a1: usize, a2: T1) {
    a0.splice(
        a0.len() - 1..a0.len() - 1,
        ::std::vec::from_elem(a2, a1 as usize),
    );
}

unsafe fn f90<T1: CChar, T2, T3>(a0: &mut Vec<T1>, a1: usize, a2: usize, a3: *const T1, a4: usize) {
    a0.splice(
        a1 as usize..a1 as usize + a2 as usize,
        ::std::slice::from_raw_parts(a3, a4 as usize).to_vec(),
    );
}

unsafe fn f104<T1: CChar, T2, T3>(a0: Vec<T1>) -> *const T1 {
    a0.as_ptr()
}

unsafe fn f105<T1: CChar, T2, T3>(a0: Vec<T1>) -> *const T1 {
    a0.as_ptr()
}

unsafe fn f106<T1: CChar, T2, T3>(a0: &mut Vec<T1>) -> *const T1 {
    a0.as_mut_ptr()
}

unsafe fn f110<T1: CChar, T2, T3>(a0: Vec<T1>, a1: *const T1, a2: usize) -> usize {
    let __hay = &a0[..a0.len() - 1];
    let __needle = ::std::slice::from_raw_parts(a1, libcc2rs::c_str_len(a1));
    (a2..=__hay.len().saturating_sub(__needle.len()))
        .find(|&__i| __hay[__i..].starts_with(__needle))
        .unwrap_or(usize::MAX)
}

unsafe fn f132<T1: CChar, T2, T3>(a0: *const Vec<T1>, a1: usize, a2: usize) -> Vec<T1> {
    let __range = a1..::std::cmp::min(a1 + a2, (*a0).len() - 1);
    let mut __tmp1 = (&*a0)[__range].to_vec();
    __tmp1.push(T1::default());
    __tmp1
}

unsafe fn f189<T1: CChar, T2, T3>(a0: Vec<T1>, a1: *const T1) -> Vec<T1> {
    let mut __tmp2 = a0.clone();
    __tmp2.pop();
    __tmp2.extend_from_slice(::std::slice::from_raw_parts(a1, libcc2rs::c_str_len(a1)));
    __tmp2.push(T1::default());
    __tmp2
}

unsafe fn f193<T1: CChar, T2, T3>(a0: Vec<T1>, a1: *const T1) -> Vec<T1> {
    let mut __tmp2 = a0.clone();
    __tmp2.pop();
    __tmp2.extend_from_slice(::std::slice::from_raw_parts(a1, libcc2rs::c_str_len(a1)));
    __tmp2.push(T1::default());
    __tmp2
}

fn t1<T1: CChar, T2, T3>() -> Vec<T1> {
    vec![T1::default()]
}

fn t3<T1: CChar, T2, T3>() -> T1 {
    T1::default()
}

fn t5() -> usize {
    0_usize
}

fn t6() -> isize {
    0_isize
}

fn t7<T1>() -> *mut T1 {
    ::std::ptr::null_mut()
}

fn t8<T1>() -> *const T1 {
    ::std::ptr::null()
}

fn t9<T1>() -> *mut T1 {
    ::std::ptr::null_mut()
}

fn t11<T1>() -> UnsafeRandomAccessIterator<T1> {
    UnsafeRandomAccessIterator::default()
}

fn t12<T1>() -> UnsafeRandomAccessIterator<T1> {
    UnsafeRandomAccessIterator::default()
}

unsafe fn f148<T1: CChar, T2, T3>(a0: *const T1, a1: T3) -> Vec<T1> {
    std::slice::from_raw_parts(a0, libcc2rs::c_str_len(a0) + 1).to_vec()
}

unsafe fn f16<T1: CChar, T2, T3>(a0: *const T1, a1: usize, a2: T3) -> Vec<T1> {
    std::slice::from_raw_parts(a0, a1 as usize)
        .to_vec()
        .iter()
        .copied()
        .chain(std::iter::once(T1::default()))
        .collect()
}

unsafe fn f149<T1: CChar, T2, T3>(a0: usize, a1: T1, a2: T3) -> Vec<T1> {
    vec![a1; (a0) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(T1::default()))
        .collect()
}

unsafe fn f122<T1: CChar, T2, T3>(a0: Vec<T1>, a1: *const T1, a2: usize) -> usize {
    let __n = a0.len().saturating_sub(1);
    let __end = a2.saturating_add(1).min(__n);
    let __lookup = ::std::slice::from_raw_parts(a1, libcc2rs::c_str_len(a1));
    match a0.iter().take(__end).rposition(|__c| __lookup.contains(__c)) {
        Some(__idx) => __idx,
        None => usize::MAX,
    }
}

unsafe fn f203<T1: CChar, T2, T3>(a0: Vec<T1>, a1: *const T1) -> bool {
    a0 == std::slice::from_raw_parts(a1, libcc2rs::c_str_len(a1) + 1).to_vec()
}

unsafe fn f51<T1: CChar, T2, T3>(mut a0: &mut Vec<T1>, a1: usize) -> *mut T1 {
    (&mut a0).as_mut_ptr().add(a1 as usize)
}

unsafe fn f183<T1, T2, T3>() -> usize {
    usize::MAX
}

unsafe fn f200<T1: StreamChar, T2, T3>(mut a0: &mut ::std::fs::File, a1: Vec<T1>) -> *mut ::std::fs::File {
    let __r: &mut ::std::fs::File = &mut a0;
    let __s: *mut ::std::fs::File = __r;
    let mut __bytes = Vec::new();
    for &__c in &a1[..a1.len().saturating_sub(1)] {
        __c.push_stream_bytes(&mut __bytes);
    }
    let _ = ::std::io::Write::write_all(&mut *__s, &__bytes);
    __s
}

unsafe fn f151<T1, T2, T3, T4, T5>(a0: std::fs::File, a1: std::fs::File, a2: T3) -> Vec<libc::c_char> {
    let mut __bytes: Vec<u8> = Vec::new();
    let mut __f = &a0;
    __f.read_to_end(&mut __bytes)
        .expect("couldn't read the file");
    let mut __buf: Vec<libc::c_char> = __bytes.iter().map(|&b| b as libc::c_char).collect();
    __buf.push(0);
    __buf
}
