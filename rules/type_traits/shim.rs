// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct TrueType;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct FalseType;

pub trait BoolConstant {
    const VALUE: bool;
}

impl BoolConstant for TrueType {
    const VALUE: bool = true;
}

impl BoolConstant for FalseType {
    const VALUE: bool = false;
}

pub trait Conjunction {
    const VALUE: bool;
}

pub trait Disjunction {
    const VALUE: bool;
}

macro_rules! impl_logical_traits {
    ($($t:ident),*) => {
        impl<$($t: BoolConstant),*> Conjunction for ($($t,)*) {
            const VALUE: bool = true $(&& $t::VALUE)*;
        }

        impl<$($t: BoolConstant),*> Disjunction for ($($t,)*) {
            const VALUE: bool = false $(|| $t::VALUE)*;
        }
    };
}

impl_logical_traits!();
impl_logical_traits!(T0);
impl_logical_traits!(T0, T1);
impl_logical_traits!(T0, T1, T2);
