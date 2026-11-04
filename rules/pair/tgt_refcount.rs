// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn f1<T1: Clone, T2: Clone>(a0: (Value<T1>, Value<T2>)) -> (Value<T1>, Value<T2>) {
    (
        Rc::new(RefCell::new(a0.0.borrow().clone())),
        Rc::new(RefCell::new(a0.1.borrow().clone())),
    )
}

fn f2<T1: CppMove + ByteRepr, T2: CppMove + ByteRepr>(a0: Ptr<(Value<T1>, Value<T2>)>) -> (Value<T1>, Value<T2>) {
    CppMove::cpp_move(a0)
}

fn f7<T1: Clone + ByteRepr, T2: Clone + ByteRepr>(a0: Ptr<(Value<T1>, Value<T2>)>, a1: (Value<T1>, Value<T2>)) -> Ptr<(Value<T1>, Value<T2>)> {
    a0.write((
        Rc::new(RefCell::new(a1.0.borrow().clone())),
        Rc::new(RefCell::new(a1.1.borrow().clone())),
    ));
    a0
}

fn f8<T1: CppMove + ByteRepr, T2: CppMove + ByteRepr>(a0: Ptr<(Value<T1>, Value<T2>)>, a1: Ptr<(Value<T1>, Value<T2>)>) -> Ptr<(Value<T1>, Value<T2>)> {
    a0.write(CppMove::cpp_move(a1));
    a0
}

fn f13<T1: CppFrom<T3>, T2: CppFrom<T4>, T3, T4>(
    a0: (Value<T3>, Value<T4>),
) -> (Value<T1>, Value<T2>) {
    (
        Rc::new(RefCell::new(T1::cpp_from(a0.0.as_pointer()))),
        Rc::new(RefCell::new(T2::cpp_from(a0.1.as_pointer()))),
    )
}

fn f14<T1: CppFrom<T3>, T2: CppFrom<T4>, T3, T4>(
    a0: &mut (Value<T3>, Value<T4>),
) -> (Value<T1>, Value<T2>) {
    (
        Rc::new(RefCell::new(T1::cpp_from_move(a0.0.as_pointer()))),
        Rc::new(RefCell::new(T2::cpp_from_move(a0.1.as_pointer()))),
    )
}

fn f18<T1: CppFrom<T3> + ByteRepr, T2: CppFrom<T4> + ByteRepr, T3, T4>(
    a0: Ptr<(Value<T1>, Value<T2>)>,
    a1: (Value<T3>, Value<T4>),
) -> Ptr<(Value<T1>, Value<T2>)> {
    a0.write((
        Rc::new(RefCell::new(T1::cpp_from(a1.0.as_pointer()))),
        Rc::new(RefCell::new(T2::cpp_from(a1.1.as_pointer()))),
    ));
    a0
}

fn f19<T1: CppFrom<T3> + ByteRepr, T2: CppFrom<T4> + ByteRepr, T3, T4>(
    a0: Ptr<(Value<T1>, Value<T2>)>,
    a1: &mut (Value<T3>, Value<T4>),
) -> Ptr<(Value<T1>, Value<T2>)> {
    a0.write((
        Rc::new(RefCell::new(T1::cpp_from_move(a1.0.as_pointer()))),
        Rc::new(RefCell::new(T2::cpp_from_move(a1.1.as_pointer()))),
    ));
    a0
}

fn t1<T1: Default, T2: Default>() -> (Value<T1>, Value<T2>) {
    (
        Rc::new(RefCell::new(T1::default())),
        Rc::new(RefCell::new(T2::default())),
    )
}

fn f12<T1, T2, T3, T4>(a0: T1, a1: T2) -> (Value<T1>, Value<T2>) {
    (Rc::new(RefCell::new(a0)), Rc::new(RefCell::new(a1)))
}

fn f15<T1: CppFrom<T3>, T2: CppFrom<T4>, T3, T4>(a0: &mut (Value<T3>, Value<T4>)) -> (Value<T1>, Value<T2>) {
    (
        Rc::new(RefCell::new(T1::cpp_from(a0.0.as_pointer()))),
        Rc::new(RefCell::new(T2::cpp_from(a0.1.as_pointer()))),
    )
}
