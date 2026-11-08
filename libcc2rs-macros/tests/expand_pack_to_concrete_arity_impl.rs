// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs_macros::expand_pack_to_concrete_arity_impl;

trait Flag {
    const VALUE: bool;
}

struct Yes;
struct No;

impl Flag for Yes {
    const VALUE: bool = true;
}

impl Flag for No {
    const VALUE: bool = false;
}

trait All {
    const VALUE: bool;
}

expand_pack_to_concrete_arity_impl! { 3;
    impl<#(T: Flag),*> All for (#(T,)*) {
        const VALUE: bool = true #(&& T::VALUE)*;
    }
}

trait Sum {
    fn sum(&self) -> i64;
}

expand_pack_to_concrete_arity_impl! { 3;
    impl<#(T: Copy + Into<i64>),*> Sum for (#(T,)*) {
        fn sum(&self) -> i64 {
            0 #(+ self.#i.into())*
        }
    }
}

trait Convert<S> {
    fn convert(source: S) -> Self;
}

expand_pack_to_concrete_arity_impl! { 3;
    impl<#(T: From<S>, S),*> Convert<(#(S,)*)> for (#(T,)*) {
        fn convert(source: (#(S,)*)) -> Self {
            (#(T::from(source.#i),)*)
        }
    }
}

trait Element<const N: usize> {
    type Type;
    fn element(&self) -> &Self::Type;
}

expand_pack_to_concrete_arity_impl! { 3, each;
    impl<#(T),*> Element<#i> for (#(T,)*) {
        type Type = #T;
        fn element(&self) -> &#T {
            &self.#i
        }
    }
}

#[test]
fn all_over_each_arity() {
    assert!(<() as All>::VALUE);
    assert!(<(Yes,) as All>::VALUE);
    assert!(!<(No,) as All>::VALUE);
    assert!(<(Yes, Yes) as All>::VALUE);
    assert!(!<(Yes, No) as All>::VALUE);
    assert!(!<(Yes, Yes, No) as All>::VALUE);
}

#[test]
fn repetition_with_separator_and_index() {
    assert_eq!(().sum(), 0);
    assert_eq!((1i32,).sum(), 1);
    assert_eq!((1i32, 2i64).sum(), 3);
    assert_eq!((1i32, 2i64, 3u8).sum(), 6);
}

#[test]
fn two_element_lists() {
    let converted: (i64, f64) = Convert::convert((4i32, 1.5f32));
    assert_eq!(converted, (4, 1.5));
    let single: (i64,) = Convert::convert((3i32,));
    assert_eq!(single.0, 3);
    let empty: () = Convert::convert(());
    assert_eq!(empty, ());
}

#[test]
fn each_element() {
    let t = (1i32, 2.5f64, 'c');
    assert_eq!(*<(i32, f64, char) as Element<0>>::element(&t), 1);
    assert_eq!(*<(i32, f64, char) as Element<1>>::element(&t), 2.5);
    assert_eq!(*<(i32, f64, char) as Element<2>>::element(&t), 'c');
    assert_eq!(*<(i32,) as Element<0>>::element(&(7,)), 7);
}
