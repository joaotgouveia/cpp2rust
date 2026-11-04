// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

unsafe fn f13<T1, T2>(a0: T1, a1: T2) -> (T1, T2) {
    (a0, a1)
}

unsafe fn f9<const T1: usize, T2, T3>(a0: &(T2, T3)) -> *const <(T2, T3) as TupleElement<T1>>::Type
where
    (T2, T3): TupleElement<T1>,
{
    &*a0.get::<T1>()
}

unsafe fn f10<const T1: usize, T2, T3>(a0: &mut (T2, T3)) -> *mut <(T2, T3) as TupleElement<T1>>::Type
where
    (T2, T3): TupleElement<T1>,
{
    &mut *a0.get_mut::<T1>()
}

unsafe fn f11<const T1: usize, T2, T3>(a0: &(T2, T3)) -> *const <(T2, T3) as TupleElement<T1>>::Type
where
    (T2, T3): TupleElement<T1>,
{
    &*a0.get::<T1>()
}

unsafe fn f12<const T1: usize, T2, T3>(a0: &mut (T2, T3)) -> *mut <(T2, T3) as TupleElement<T1>>::Type
where
    (T2, T3): TupleElement<T1>,
{
    &mut *a0.get_mut::<T1>()
}
