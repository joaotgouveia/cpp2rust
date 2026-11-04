// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

fn f16<T1: CppLess>(a0: Ptr<T1>, a1: Ptr<T1>) -> Ptr<T1> {
    if CppLess::less(&a0, &a1) { a1 } else { a0 }
}

fn f18<T1: CppLess>(a0: Ptr<T1>, a1: Ptr<T1>) -> Ptr<T1> {
    if CppLess::less(&a1, &a0) { a1 } else { a0 }
}

fn f1<
    T1: CppInputIterator<T3>,
    T2: CppIterator<T4>,
    T3: ByteRepr,
    T4: CppFrom<T3> + ByteRepr,
>(
    a0: T1,
    a1: T1,
    a2: T2,
) -> T2 {
    let mut __first = a0;
    let mut __out = a2;
    while __first != a1 {
        __out.iter_deref().write(T4::cpp_from(__first.iter_deref()));
        __first.iter_inc();
        __out.iter_inc();
    }
    __out
}

fn f7<T1: CppInputIterator<T2>, T2: Clone + ByteRepr>(a0: T1, a1: T1, a2: T2) {
    let mut __it = a0;
    while __it != a1 {
        __it.iter_deref().write(Clone::clone(&a2));
        __it.iter_inc();
    }
}
