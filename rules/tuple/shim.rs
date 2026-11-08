// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs_macros::expand_pack_to_concrete_arity_impl;

pub trait TupleFrom<S> {
    fn tuple_from(source: S) -> Self;
}

expand_pack_to_concrete_arity_impl! { 32;
    impl<#(T: From<S>, S),*> TupleFrom<(#(S,)*)> for (#(T,)*) {
        fn tuple_from(_source: (#(S,)*)) -> Self {
            (#(T::from(_source.#i),)*)
        }
    }
}
