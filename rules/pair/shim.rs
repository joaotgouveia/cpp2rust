// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::Value;
use libcc2rs_macros::expand_pack_to_concrete_arity_impl;

pub trait TupleElement<const N: usize> {
    type Type;

    fn element(&self) -> &Self::Type;

    fn element_mut(&mut self) -> &mut Self::Type;
}

pub trait TupleGet {
    fn get<const N: usize>(&self) -> &<Self as TupleElement<N>>::Type
    where
        Self: TupleElement<N>,
    {
        TupleElement::<N>::element(self)
    }

    fn get_mut<const N: usize>(&mut self) -> &mut <Self as TupleElement<N>>::Type
    where
        Self: TupleElement<N>,
    {
        TupleElement::<N>::element_mut(self)
    }
}

pub trait TupleElementValue<const N: usize>: TupleElement<N> {
    type Values;

    fn value(values: &Self::Values) -> &Value<Self::Type>;
}

pub trait ValueTupleGet {
    type Plain;

    fn get_value<const N: usize>(&self) -> &Value<<Self::Plain as TupleElement<N>>::Type>
    where
        Self::Plain: TupleElementValue<N, Values = Self>,
    {
        <Self::Plain as TupleElementValue<N>>::value(self)
    }
}

expand_pack_to_concrete_arity_impl! { 32, each;
    impl<#(T),*> TupleElement<#i> for (#(T,)*) {
        type Type = #T;

        fn element(&self) -> &#T {
            &self.#i
        }

        fn element_mut(&mut self) -> &mut #T {
            &mut self.#i
        }
    }
}

expand_pack_to_concrete_arity_impl! { 32, each;
    impl<#(T),*> TupleElementValue<#i> for (#(T,)*) {
        type Values = (#(Value<T>,)*);

        fn value(values: &Self::Values) -> &Value<#T> {
            &values.#i
        }
    }
}

expand_pack_to_concrete_arity_impl! { 32;
    impl<#(T),*> TupleGet for (#(T,)*) {}
}

expand_pack_to_concrete_arity_impl! { 32;
    impl<#(T),*> ValueTupleGet for (#(Value<T>,)*) {
        type Plain = (#(T,)*);
    }
}
