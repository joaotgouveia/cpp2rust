// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::marker::PhantomData;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct IntegralConstant<T, const V: i128>(pub PhantomData<T>);

pub trait Integral {}

macro_rules! impl_integral {
    ($($t:ty),*) => { $(impl Integral for $t {})* };
}

impl_integral!(
    bool, char, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

pub trait BoolConstant {
    const VALUE: bool;
}

impl<const V: i128> BoolConstant for IntegralConstant<bool, V> {
    const VALUE: bool = V != 0;
}
