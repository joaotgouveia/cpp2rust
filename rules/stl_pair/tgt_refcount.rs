// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn f13<T1, T2>(a0: T1, a1: T2) -> (Value<T1>, Value<T2>) {
    (Rc::new(RefCell::new(a0)), Rc::new(RefCell::new(a1)))
}

fn f9<const T1: usize, T2, T3>(
    a0: (Value<T2>, Value<T3>),
) -> Ptr<<(Value<T2>, Value<T3>) as PairGetRef<T1>>::Out>
where
    (Value<T2>, Value<T3>): PairGetRef<T1>,
{
    <(Value<T2>, Value<T3>) as PairGetRef<T1>>::pair_get(&a0)
}

fn f10<const T1: usize, T2, T3>(
    a0: (Value<T2>, Value<T3>),
) -> Ptr<<(Value<T2>, Value<T3>) as PairGetRef<T1>>::Out>
where
    (Value<T2>, Value<T3>): PairGetRef<T1>,
{
    <(Value<T2>, Value<T3>) as PairGetRef<T1>>::pair_get(&a0)
}

fn f11<const T1: usize, T2, T3>(
    a0: (Value<T2>, Value<T3>),
) -> Ptr<<(Value<T2>, Value<T3>) as PairGetRef<T1>>::Out>
where
    (Value<T2>, Value<T3>): PairGetRef<T1>,
{
    <(Value<T2>, Value<T3>) as PairGetRef<T1>>::pair_get(&a0)
}

fn f12<const T1: usize, T2, T3>(
    a0: (Value<T2>, Value<T3>),
) -> Ptr<<(Value<T2>, Value<T3>) as PairGetRef<T1>>::Out>
where
    (Value<T2>, Value<T3>): PairGetRef<T1>,
{
    <(Value<T2>, Value<T3>) as PairGetRef<T1>>::pair_get(&a0)
}
