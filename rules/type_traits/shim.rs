// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs_macros::expand_pack_to_concrete_arity_impl;

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

expand_pack_to_concrete_arity_impl! { 32;
    impl<#(T: BoolConstant),*> Conjunction for (#(T,)*) {
        const VALUE: bool = true #(&& T::VALUE)*;
    }
}

expand_pack_to_concrete_arity_impl! { 32;
    impl<#(T: BoolConstant),*> Disjunction for (#(T,)*) {
        const VALUE: bool = false #(|| T::VALUE)*;
    }
}
