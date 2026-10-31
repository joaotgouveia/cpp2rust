// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

unsafe fn f67<T1>(a0: *mut bool) -> bool {
    *a0
}

unsafe fn f115<T1>() -> Vec<T1> {
    Vec::new()
}

unsafe fn f119<T1: Clone>(a0: Vec<T1>) -> Vec<T1> {
    a0.clone()
}

unsafe fn f120<T1>(a0: &mut Vec<T1>) -> Vec<T1> {
    std::mem::take(&mut *a0)
}

unsafe fn f124<T1: Clone>(a0: &mut Vec<T1>, a1: Vec<T1>) {
    *a0 = a1.clone()
}

unsafe fn f125<T1: Clone>(a0: &mut Vec<T1>, a1: &mut Vec<T1>) {
    *a0 = std::mem::take(&mut *a1)
}

unsafe fn f129<T1>(a0: &mut Vec<T1>) -> UnsafeRandomAccessIterator<T1> {
    a0.begin()
}

unsafe fn f130<T1>(a0: Vec<T1>) -> UnsafeRandomAccessIterator<T1> {
    a0.begin()
}

unsafe fn f131<T1>(a0: &mut Vec<T1>) -> UnsafeRandomAccessIterator<T1> {
    a0.end()
}

unsafe fn f132<T1>(a0: Vec<T1>) -> UnsafeRandomAccessIterator<T1> {
    a0.end()
}

unsafe fn f138<T1>(a0: Vec<T1>) -> UnsafeRandomAccessIterator<T1> {
    a0.end()
}

unsafe fn f141<T1>(a0: Vec<T1>) -> usize {
    a0.len()
}

unsafe fn f143<T1: Default>(a0: &mut Vec<T1>, a1: usize) {
    let __a0 = a1 as usize;
    a0.resize_with(__a0, || <T1>::default())
}

unsafe fn f144<T1: Default + Clone>(a0: &mut Vec<T1>, a1: usize, a2: T1) {
    let __a0 = a1 as usize;
    a0.resize(__a0, a2)
}

unsafe fn f145<T1>(a0: &mut Vec<T1>) {
    a0.shrink_to_fit()
}

unsafe fn f146<T1>(a0: Vec<T1>) -> usize {
    a0.capacity()
}

unsafe fn f147<T1>(a0: Vec<T1>) -> bool {
    a0.is_empty()
}

unsafe fn f148<T1>(a0: &mut Vec<T1>, a1: usize) {
    if a1 as usize > a0.capacity() as usize {
        let len_0 = a0.len();
        a0.reserve_exact(a1 as usize - len_0 as usize);
    }
}

unsafe fn f151<T1>(mut a0: &mut Vec<T1>, a1: usize) -> *mut T1 {
    if a1 as usize >= (&mut a0).len() {
        panic!("out of bounds access")
    } else {
        (&mut a0).as_mut_ptr().add(a1 as usize)
    }
}

unsafe fn f152<T1: Copy>(a0: &Vec<T1>, a1: usize) -> *const T1 {
    let __idx = a1;
    if __idx as usize >= (&a0).len() {
        panic!("out of bounds access")
    } else {
        (&a0).as_ptr().add(__idx as usize)
    }
}

unsafe fn f153<T1>(a0: &mut Vec<T1>) -> *mut T1 {
    ((&mut *a0).first_mut().unwrap())
}

unsafe fn f155<T1>(a0: &mut Vec<T1>) -> *mut T1 {
    ((&mut *a0).last_mut().unwrap())
}

unsafe fn f156<T1>(a0: &Vec<T1>) -> &T1 {
    ((&*a0).last().unwrap())
}

unsafe fn f157<T1>(a0: &mut Vec<T1>) -> *mut T1 {
    a0.as_mut_ptr()
}

unsafe fn f158<T1>(a0: &mut Vec<T1>) -> *const T1 {
    a0.as_ptr()
}

unsafe fn f159<T1: Clone>(a0: &mut Vec<T1>, a1: T1) {
    let a0_clone = a1.clone();
    a0.push(a0_clone)
}

unsafe fn f160<T1: Default>(a0: &mut Vec<T1>, a1: &mut T1) {
    let __a1 = std::mem::take(&mut *a1);
    a0.push(__a1)
}

unsafe fn f161<T1>(a0: &mut Vec<T1>) {
    a0.pop();
}

unsafe fn f162<T1>(a0: &mut Vec<T1>, a1: UnsafeRandomAccessIterator<T1>, a2: T1) -> UnsafeRandomAccessIterator<T1> {
    let __it = a1;
    let pos = __it.iter_distance(&UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr())) as usize;
    a0.insert(pos, a2);
    __it
}

unsafe fn f163<T1>(a0: &mut Vec<T1>, a1: UnsafeRandomAccessIterator<T1>, a2: T1) -> UnsafeRandomAccessIterator<T1> {
    let __it = a1;
    let pos = __it.iter_distance(&UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr())) as usize;
    a0.insert(pos, a2);
    __it
}

unsafe fn f164<T1>(a0: &mut Vec<T1>, a1: UnsafeRandomAccessIterator<T1>, a2: Vec<T1>) -> UnsafeRandomAccessIterator<T1> {
    let __off = a1.iter_distance(&UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr())) as usize;
    a0.splice(__off..__off, a2);
    UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr().add(__off))
}

unsafe fn f165<T1: Clone>(a0: &mut Vec<T1>, a1: UnsafeRandomAccessIterator<T1>, a2: usize, a3: T1) -> UnsafeRandomAccessIterator<T1> {
    let __off = a1.iter_distance(&UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr())) as usize;
    a0.splice(__off..__off, std::iter::repeat_n(a3, a2));
    UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr().add(__off))
}

unsafe fn f166<T1>(a0: &mut Vec<T1>, a1: UnsafeRandomAccessIterator<T1>) -> UnsafeRandomAccessIterator<T1> {
    let __it = a1;
    let pos = __it.iter_distance(&UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr())) as usize;
    a0.remove(pos);
    __it
}

unsafe fn f168(a0: &mut Vec<bool>, a1: &mut Vec<bool>) {
    std::mem::swap(&mut *a0, &mut *a1)
}

unsafe fn f169<T1>(a0: &mut Vec<T1>) {
    a0.clear()
}

unsafe fn f174<T1>(a0: &mut Vec<T1>, init: T1) {
    let __init = init;
    a0.push(__init)
}

fn t4() -> usize {
    0_usize
}

fn t8() -> *mut bool {
    ::std::ptr::null_mut()
}

fn t22<T1>() -> Vec<T1> {
    Default::default()
}

fn t23<T1>() -> T1 {
    unimplemented!()
}

fn t2() -> Vec<bool> {
    Vec::new()
}

fn t6<T1>() -> *mut bool {
    ::std::ptr::null_mut()
}

fn t24<T1>() -> *mut T1 {
    ::std::ptr::null_mut()
}

fn t25<T1>() -> *const T1 {
    ::std::ptr::null()
}

fn t26<T1>() -> *mut T1 {
    ::std::ptr::null_mut()
}

fn t27<T1>() -> *const T1 {
    ::std::ptr::null()
}

fn t28<T1>() -> UnsafeRandomAccessIterator<T1> {
    UnsafeRandomAccessIterator::default()
}

fn t29<T1>() -> UnsafeRandomAccessIterator<T1> {
    UnsafeRandomAccessIterator::default()
}

fn t32() -> usize {
    0_usize
}

fn t33() -> isize {
    0_isize
}

unsafe fn f149<T1, T2>(mut a0: &mut Vec<T1>, a1: usize) -> *mut T1 {
    (&mut a0).as_mut_ptr().add(a1 as usize)
}

unsafe fn f150<T1, T2>(a0: &Vec<T1>, a1: usize) -> *const T1 {
    (&a0).as_ptr().add(a1 as usize)
}

unsafe fn f117<T1: Default, T2>(a0: usize, a1: T2) -> Vec<T1> {
    (0..(a0) as usize)
        .map(|_| <T1>::default())
        .collect::<Vec<_>>()
}

unsafe fn f118<T1: Default + Clone, T2>(a0: usize, a1: T1, a2: T2) -> Vec<T1> {
    vec![a1; a0 as usize]
}

unsafe fn f123<T1, T2>(a0: Vec<T1>, a1: T2) -> Vec<T1> {
    a0
}

unsafe fn f170<T1: CppFromUnsafe<T5>, T2, T3: UnsafeCppInputIterator<T5>, T4, T5>(
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

unsafe fn f176<T1: CppFromUnsafe<T5>, T2, T3: UnsafeCppInputIterator<T5>, T4, T5>(
    a0: &mut Vec<T1>,
    a1: UnsafeRandomAccessIterator<T1>,
    a2: T3,
    a3: T3,
) -> UnsafeRandomAccessIterator<T1> {
    let __off = a1.iter_distance(&UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr())) as usize;
    let mut __tmp = Vec::new();
    let mut __it = a2;
    while __it != a3 {
        __tmp.push(T1::cpp_from(__it.iter_deref()));
        __it.iter_inc();
    }
    a0.splice(__off..__off, __tmp);
    UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr().add(__off))
}

unsafe fn f12<T1>(a0: Vec<bool>, a1: T1) -> Vec<bool> {
    a0
}

unsafe fn f34<T1>(mut a0: &mut Vec<bool>, a1: usize) -> *mut bool {
    (&mut a0).as_mut_ptr().add(a1 as usize)
}

unsafe fn f66<T1>(a0: *mut bool) -> *mut bool {
    a0
}

fn t10<T1>() -> UnsafeRandomAccessIterator<bool> {
    UnsafeRandomAccessIterator::default()
}

fn t11<T1>() -> UnsafeRandomAccessIterator<bool> {
    UnsafeRandomAccessIterator::default()
}

fn t15<T1>() -> *mut bool {
    ::std::ptr::null_mut()
}

fn t16<T1>() -> bool {
    false
}

unsafe fn f18<T1>(a0: &mut Vec<bool>) -> UnsafeRandomAccessIterator<bool> {
    a0.begin()
}

unsafe fn f68<T1>(a0: *mut bool, a1: bool) -> *mut bool {
    *a0 = a1;
    a0
}

unsafe fn f69<T1>(a0: *mut bool, a1: bool) -> *mut bool {
    *a0 = a1;
    a0
}

unsafe fn f70<T1>(a0: *mut bool, a1: *mut bool) -> *mut bool {
    *a0 = *a1;
    a0
}

unsafe fn f71<T1>(a0: *mut bool, a1: *mut bool) -> bool {
    *a0 == *a1
}

unsafe fn f72<T1>(a0: *mut bool, a1: *mut bool) -> bool {
    !*a0 & *a1
}

unsafe fn f73<T1>(a0: *mut bool) {
    *a0 = !*a0;
}

unsafe fn f74<T1>(a0: *mut bool, a1: *mut bool) {
    ::std::ptr::swap(a0, a1);
}

unsafe fn f75<T1>(a0: *mut bool, a1: *mut bool) {
    ::std::ptr::swap(a0, a1);
}

unsafe fn f76<T1>(a0: *mut bool, a1: *mut bool) {
    ::std::ptr::swap(a0, a1);
}

unsafe fn f77<T1>() -> UnsafeRandomAccessIterator<bool> {
    UnsafeRandomAccessIterator::default()
}

unsafe fn f79<T1>(a0: &mut UnsafeRandomAccessIterator<bool>) -> &mut UnsafeRandomAccessIterator<bool> {
    a0.iter_inc()
}

unsafe fn f80<T1>(a0: &mut UnsafeRandomAccessIterator<bool>, a1: i32) -> UnsafeRandomAccessIterator<bool> {
    a0.iter_post_inc()
}

unsafe fn f81<T1>(a0: &mut UnsafeRandomAccessIterator<bool>) -> &mut UnsafeRandomAccessIterator<bool> {
    a0.iter_dec()
}

unsafe fn f82<T1>(a0: &mut UnsafeRandomAccessIterator<bool>, a1: i32) -> UnsafeRandomAccessIterator<bool> {
    a0.iter_post_dec()
}

unsafe fn f83<T1>(a0: &mut UnsafeRandomAccessIterator<bool>, a1: isize) -> UnsafeRandomAccessIterator<bool> {
    a0.iter_add_assign(a1)
}

unsafe fn f84<T1>(a0: &mut UnsafeRandomAccessIterator<bool>, a1: isize) -> UnsafeRandomAccessIterator<bool> {
    a0.iter_add_assign(-a1)
}

unsafe fn f85<T1>(a0: UnsafeRandomAccessIterator<bool>) -> UnsafeRandomAccessIterator<bool> {
    a0
}

unsafe fn f86<T1>(a0: UnsafeRandomAccessIterator<bool>) -> UnsafeRandomAccessIterator<bool> {
    a0
}

unsafe fn f87<T1>(a0: &mut UnsafeRandomAccessIterator<bool>, a1: UnsafeRandomAccessIterator<bool>) -> UnsafeRandomAccessIterator<bool> {
    *a0 = a1;
    a0.clone()
}

unsafe fn f88<T1>(a0: &mut UnsafeRandomAccessIterator<bool>, a1: UnsafeRandomAccessIterator<bool>) -> UnsafeRandomAccessIterator<bool> {
    *a0 = a1;
    a0.clone()
}

unsafe fn f89<T1>(a0: UnsafeRandomAccessIterator<bool>, a1: isize) -> UnsafeRandomAccessIterator<bool> {
    a0.iter_add(-a1)
}

unsafe fn f90<T1>(a0: isize, a1: UnsafeRandomAccessIterator<bool>) -> UnsafeRandomAccessIterator<bool> {
    a1.iter_add(a0)
}

unsafe fn f91<T1>(a0: UnsafeRandomAccessIterator<bool>, a1: isize) -> UnsafeRandomAccessIterator<bool> {
    a0.iter_add(a1)
}

unsafe fn f92<T1>(a0: UnsafeRandomAccessIterator<bool>, a1: UnsafeRandomAccessIterator<bool>) -> isize {
    a0.iter_distance(&a1)
}

unsafe fn f19<T1>(a0: &Vec<bool>) -> UnsafeRandomAccessIterator<bool> {
    a0.begin()
}

unsafe fn f20<T1>(a0: &mut Vec<bool>) -> UnsafeRandomAccessIterator<bool> {
    a0.end()
}

unsafe fn f21<T1>(a0: &Vec<bool>) -> UnsafeRandomAccessIterator<bool> {
    a0.end()
}

unsafe fn f26<T1>(a0: &Vec<bool>) -> UnsafeRandomAccessIterator<bool> {
    a0.begin()
}

unsafe fn f27<T1>(a0: &Vec<bool>) -> UnsafeRandomAccessIterator<bool> {
    a0.end()
}

unsafe fn f93<T1>(a0: UnsafeRandomAccessIterator<bool>, a1: UnsafeRandomAccessIterator<bool>) -> std::cmp::Ordering {
    a0.partial_cmp(&a1).unwrap()
}

unsafe fn f94<T1>(a0: UnsafeRandomAccessIterator<bool>, a1: UnsafeRandomAccessIterator<bool>) -> bool {
    a0 == a1
}

unsafe fn f95<T1>() -> UnsafeRandomAccessIterator<bool> {
    UnsafeRandomAccessIterator::default()
}

unsafe fn f97<T1>(a0: UnsafeRandomAccessIterator<bool>) -> bool {
    *a0.iter_deref()
}

unsafe fn f98<T1>(a0: &mut UnsafeRandomAccessIterator<bool>) -> &mut UnsafeRandomAccessIterator<bool> {
    a0.iter_inc()
}

unsafe fn f99<T1>(a0: &mut UnsafeRandomAccessIterator<bool>, a1: i32) -> UnsafeRandomAccessIterator<bool> {
    a0.iter_post_inc()
}

unsafe fn f100<T1>(a0: &mut UnsafeRandomAccessIterator<bool>) -> &mut UnsafeRandomAccessIterator<bool> {
    a0.iter_dec()
}

unsafe fn f101<T1>(a0: &mut UnsafeRandomAccessIterator<bool>, a1: i32) -> UnsafeRandomAccessIterator<bool> {
    a0.iter_post_dec()
}

unsafe fn f102<T1>(a0: &mut UnsafeRandomAccessIterator<bool>, a1: isize) -> UnsafeRandomAccessIterator<bool> {
    a0.iter_add_assign(a1)
}

unsafe fn f103<T1>(a0: &mut UnsafeRandomAccessIterator<bool>, a1: isize) -> UnsafeRandomAccessIterator<bool> {
    a0.iter_add_assign(-a1)
}

unsafe fn f104<T1>(a0: UnsafeRandomAccessIterator<bool>, a1: isize) -> bool {
    *a0.iter_at(a1)
}

unsafe fn f105<T1>(a0: UnsafeRandomAccessIterator<bool>) -> UnsafeRandomAccessIterator<bool> {
    a0
}

unsafe fn f106<T1>(a0: UnsafeRandomAccessIterator<bool>) -> UnsafeRandomAccessIterator<bool> {
    a0
}

unsafe fn f107<T1>(a0: &mut UnsafeRandomAccessIterator<bool>, a1: UnsafeRandomAccessIterator<bool>) -> UnsafeRandomAccessIterator<bool> {
    *a0 = a1;
    a0.clone()
}

unsafe fn f108<T1>(a0: &mut UnsafeRandomAccessIterator<bool>, a1: UnsafeRandomAccessIterator<bool>) -> UnsafeRandomAccessIterator<bool> {
    *a0 = a1;
    a0.clone()
}

unsafe fn f109<T1>(a0: isize, a1: UnsafeRandomAccessIterator<bool>) -> UnsafeRandomAccessIterator<bool> {
    a1.iter_add(a0)
}

unsafe fn f110<T1>(a0: UnsafeRandomAccessIterator<bool>, a1: isize) -> UnsafeRandomAccessIterator<bool> {
    a0.iter_add(-a1)
}

unsafe fn f111<T1>(a0: UnsafeRandomAccessIterator<bool>, a1: isize) -> UnsafeRandomAccessIterator<bool> {
    a0.iter_add(a1)
}

unsafe fn f112<T1>(a0: UnsafeRandomAccessIterator<bool>, a1: UnsafeRandomAccessIterator<bool>) -> isize {
    a0.iter_distance(&a1)
}

unsafe fn f113<T1>(a0: UnsafeRandomAccessIterator<bool>, a1: UnsafeRandomAccessIterator<bool>) -> std::cmp::Ordering {
    a0.partial_cmp(&a1).unwrap()
}

unsafe fn f114<T1>(a0: UnsafeRandomAccessIterator<bool>, a1: UnsafeRandomAccessIterator<bool>) -> bool {
    a0 == a1
}

fn t17<T1>() -> UnsafeRandomAccessIterator<bool> {
    UnsafeRandomAccessIterator::default()
}

fn t18<T1>() -> bool {
    false
}

fn t19<T1>() -> bool {
    false
}

fn t21<T1>() -> UnsafeRandomAccessIterator<bool> {
    UnsafeRandomAccessIterator::default()
}

fn t20<T1>() -> *const bool {
    ::std::ptr::null()
}

fn t3<T1>() -> bool {
    false
}

unsafe fn f6<T1>(a0: usize, a1: T1) -> Vec<bool> {
    vec![false; a0]
}

unsafe fn f30<T1>(a0: &Vec<bool>) -> usize {
    (&a0).len()
}

unsafe fn f57<T1, T2: UnsafeCppInputIterator<T4>, T3, T4: Clone + PartialEq + Default>(a0: T2, a1: T2, a2: T1) -> Vec<bool> {
    let mut __v = Vec::new();
    let mut __it = a0;
    while __it != a1 {
        __v.push((*__it.iter_deref()).clone() != T4::default());
        __it.iter_inc();
    }
    __v
}
