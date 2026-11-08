// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cell::RefCell;
use std::rc::Rc;

use crate::Value;
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

pub trait ValueTuple {
    type Values;

    fn into_values(self) -> Self::Values;
}

pub trait ValueTupleClone: ValueTuple {
    fn from_values(values: &Self::Values) -> Self;
}

pub trait ValueTupleDefault: ValueTuple {
    fn default_values() -> Self::Values;
}

expand_pack_to_concrete_arity_impl! { 32;
    impl<#(T),*> ValueTuple for (#(T,)*) {
        type Values = (#(Value<T>,)*);

        fn into_values(self) -> Self::Values {
            (#(Rc::new(RefCell::new(self.#i)),)*)
        }
    }
}

expand_pack_to_concrete_arity_impl! { 32;
    impl<#(T: Clone),*> ValueTupleClone for (#(T,)*) {
        fn from_values(_values: &Self::Values) -> Self {
            (#(_values.#i.borrow().clone(),)*)
        }
    }
}

expand_pack_to_concrete_arity_impl! { 32;
    impl<#(T: Default),*> ValueTupleDefault for (#(T,)*) {
        fn default_values() -> Self::Values {
            (#(Rc::new(RefCell::new(T::default())),)*)
        }
    }
}
