// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::BoolConstant;
use libcc2rs_macros::expand_pack_to_concrete_arity_impl;

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
