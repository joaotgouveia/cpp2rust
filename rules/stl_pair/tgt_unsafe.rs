// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

unsafe fn f13<T1, T2>(a0: T1, a1: T2) -> (T1, T2) {
    (a0, a1)
}

unsafe fn f9<const T1: usize, T2, T3>(a0: &(T2, T3)) -> *const <(T2, T3) as PairGet<T1>>::Out
where
    (T2, T3): PairGet<T1>,
{
    <(T2, T3) as PairGet<T1>>::pair_get(&a0)
}

unsafe fn f10<const T1: usize, T2, T3>(mut a0: &mut (T2, T3)) -> *mut <(T2, T3) as PairGet<T1>>::Out
where
    (T2, T3): PairGet<T1>,
{
    <(T2, T3) as PairGet<T1>>::pair_get_mut(&mut a0)
}

unsafe fn f11<const T1: usize, T2, T3>(a0: &(T2, T3)) -> *const <(T2, T3) as PairGet<T1>>::Out
where
    (T2, T3): PairGet<T1>,
{
    <(T2, T3) as PairGet<T1>>::pair_get(&a0)
}

unsafe fn f12<const T1: usize, T2, T3>(mut a0: &mut (T2, T3)) -> *mut <(T2, T3) as PairGet<T1>>::Out
where
    (T2, T3): PairGet<T1>,
{
    <(T2, T3) as PairGet<T1>>::pair_get_mut(&mut a0)
}
