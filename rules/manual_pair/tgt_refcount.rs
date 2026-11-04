// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn f15<T1, T2>(a0: (Value<T1>, Value<T2>)) -> Value<T2> {
    a0.1
}

fn f16<T1, T2>(a0: (Value<T1>, Value<T2>)) -> Value<T1> {
    a0.0
}

fn f17<T1: CppFrom<T3> + ByteRepr, T2: CppFrom<T4> + ByteRepr, T3, T4>(
    a0: Ptr<(Value<T1>, Value<T2>)>,
    a1: (Value<T3>, Value<T4>),
) {
    a0.write((
        Rc::new(RefCell::new(T1::cpp_from(a1.0.as_pointer()))),
        Rc::new(RefCell::new(T2::cpp_from(a1.1.as_pointer()))),
    ))
}
