// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

pub trait TupleFrom<S> {
    fn tuple_from(source: S) -> Self;
}

macro_rules! impl_tuple_from {
    ($(($t:ident $s:ident $n:tt)),*) => {
        impl<$($t: From<$s>, $s),*> TupleFrom<($($s,)*)> for ($($t,)*) {
            fn tuple_from(source: ($($s,)*)) -> Self {
                ($($t::from(source.$n),)*)
            }
        }
    };
}

impl_tuple_from!();
impl_tuple_from!((T0 S0 0));
impl_tuple_from!((T0 S0 0), (T1 S1 1));
impl_tuple_from!((T0 S0 0), (T1 S1 1), (T2 S2 2));
