// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::io::{Seek, Write};

unsafe fn f42<T1: UnsafeCppInputIterator<T2>, T2: PartialOrd>(a0: T1, a1: T1) -> T1 {
    let mut __best = a0.clone();
    if a0 != a1 {
        let mut __it = a0;
        __it.iter_inc();
        while __it != a1 {
            if *__best.iter_deref() < *__it.iter_deref() {
                __best = __it.clone();
            }
            __it.iter_inc();
        }
    }
    __best
}

unsafe fn f70<T1: UnsafeCppRandomAccessIterator<T2>, T2>(a0: T1, a1: T1) {
    let __n = a1.iter_distance(&a0);
    for __i in (1..__n).rev() {
        let __j = libc::rand() as isize % (__i + 1);
        cpp_swap_unsafe!(a0.iter_at(__i), a0.iter_at(__j));
    }
}

unsafe fn f97<T1: UnsafeCppRandomAccessIterator<T2>, T2: Ord>(a0: T1, a1: T1) {
    let __n = a1.iter_distance(&a0);
    let mut __order: Vec<isize> = (0..__n).collect();
    __order.sort_by(|&__x, &__y| (*a0.iter_at(__x)).cmp(&*a0.iter_at(__y)));
    let __values: Vec<_> = __order
        .iter()
        .map(|&__i| std::ptr::read(a0.iter_at(__i)))
        .collect();
    for (__i, __v) in (0..__n).zip(__values) {
        std::ptr::write(a0.iter_at(__i), __v);
    }
}

unsafe fn f104<T1: UnsafeCppInputIterator<T2>, T2: PartialEq + Clone>(a0: T1, a1: T1) -> T1 {
    if a0 == a1 {
        a1
    } else {
        let mut __write = a0.clone();
        let mut __it = a0;
        __it.iter_inc();
        while __it != a1 {
            if *__write.iter_deref() != *__it.iter_deref() {
                __write.iter_inc();
                *__write.iter_deref() = (*__it.iter_deref()).clone();
            }
            __it.iter_inc();
        }
        __write.iter_inc();
        __write
    }
}

unsafe fn f16<T1: UnsafeCppInputIterator<T3>, T2, T3: PartialEq<T2>>(a0: T1, a1: T1, a2: T2) -> T1 {
    let mut __it = a0;
    while __it != a1 && *__it.iter_deref() != a2 {
        __it.iter_inc();
    }
    __it
}

unsafe fn f101<T1: UnsafeCppRandomAccessIterator<T3>, T2, T3>(a0: T1, a1: T1, a2: T2)
where
    T2: UnsafeComparator<T3>,
{
    let __n = a1.iter_distance(&a0);
    let mut __order: Vec<isize> = (0..__n).collect();
    __order.sort_by(|&__x, &__y| {
        let __a = a0.iter_at(__x) as *const _;
        let __b = a0.iter_at(__y) as *const _;
        if a2.unsafe_less(__a, __b) {
            std::cmp::Ordering::Less
        } else if a2.unsafe_less(__b, __a) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    let __values: Vec<_> = __order
        .iter()
        .map(|&__i| std::ptr::read(a0.iter_at(__i)))
        .collect();
    for (__i, __v) in (0..__n).zip(__values) {
        std::ptr::write(a0.iter_at(__i), __v);
    }
}
