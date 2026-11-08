// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t2<T1: ValueTupleDefault>() -> <T1 as ValueTuple>::Values {
    <T1 as ValueTupleDefault>::default_values()
}

fn f6<const T1: usize, T2>(a0: <T2 as ValueTuple>::Values) -> Ptr<<T2 as TupleElement<T1>>::Type>
where
    T2: ValueTuple + TupleElementValue<T1, Values = <T2 as ValueTuple>::Values>,
    <T2 as ValueTuple>::Values: ValueTupleGet<Plain = T2>,
{
    a0.get_value::<T1>().as_pointer()
}

fn f12<T1: ValueTuple>(a0: T1) -> <T1 as ValueTuple>::Values {
    <T1 as ValueTuple>::into_values(a0)
}

fn f14<T1: ValueTupleClone + PartialEq<T2>, T2: ValueTupleClone>(
    a0: <T1 as ValueTuple>::Values,
    a1: <T2 as ValueTuple>::Values,
) -> bool {
    <T1 as ValueTupleClone>::from_values(&a0) == <T2 as ValueTupleClone>::from_values(&a1)
}

fn f27<T1: ValueTupleDefault>() -> <T1 as ValueTuple>::Values {
    <T1 as ValueTupleDefault>::default_values()
}

fn f36<T1: ValueTuple>(a0: T1) -> <T1 as ValueTuple>::Values {
    <T1 as ValueTuple>::into_values(a0)
}

fn f37<T1: ValueTuple + TupleFrom<T2>, T2>(a0: T2) -> <T1 as ValueTuple>::Values {
    <T1 as ValueTuple>::into_values(<T1 as TupleFrom<T2>>::tuple_from(a0))
}

fn f39<T1: ValueTuple + TupleFrom<T2>, T2: ValueTupleClone>(
    a0: <T2 as ValueTuple>::Values,
) -> <T1 as ValueTuple>::Values {
    <T1 as ValueTuple>::into_values(<T1 as TupleFrom<T2>>::tuple_from(
        <T2 as ValueTupleClone>::from_values(&a0),
    ))
}

fn f46<T1: ValueTuple + TupleFrom<(T2,)>, T2>(a0: T2) -> <T1 as ValueTuple>::Values {
    <T1 as ValueTuple>::into_values(<T1 as TupleFrom<(T2,)>>::tuple_from((a0,)))
}
