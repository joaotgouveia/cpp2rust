// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t2<T1: Default>() -> T1 {
    T1::default()
}

unsafe fn f6<const T1: usize, T2>(a0: &mut T2) -> *mut <T2 as TupleElement<T1>>::Type
where
    T2: TupleElement<T1> + TupleGet,
{
    &mut *a0.get_mut::<T1>()
}

unsafe fn f12<T1>(a0: T1) -> T1 {
    a0
}

unsafe fn f14<T1: PartialEq<T2>, T2>(a0: T1, a1: T2) -> bool {
    a0 == a1
}

unsafe fn f27<T1: Default>() -> T1 {
    T1::default()
}

unsafe fn f36<T1>(a0: T1) -> T1 {
    a0
}

unsafe fn f37<T1: TupleFrom<T2>, T2>(a0: T2) -> T1 {
    <T1 as libcc2rs::TupleFrom<T2>>::tuple_from(a0)
}

unsafe fn f39<T1: TupleFrom<T2>, T2>(a0: T2) -> T1 {
    <T1 as libcc2rs::TupleFrom<T2>>::tuple_from(a0)
}

unsafe fn f46<T1: TupleFrom<(T2,)>, T2>(a0: T2) -> T1 {
    <T1 as libcc2rs::TupleFrom<(T2,)>>::tuple_from((a0,))
}
