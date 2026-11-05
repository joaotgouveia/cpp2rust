// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

pub struct IgnoreRule;

pub trait PairGet<const N: usize> {
    type Out;
    fn pair_get(&self) -> *const Self::Out;
    fn pair_get_mut(&mut self) -> *mut Self::Out;
}

impl<A, B> PairGet<0> for (A, B) {
    type Out = A;
    fn pair_get(&self) -> *const A {
        &self.0
    }
    fn pair_get_mut(&mut self) -> *mut A {
        &mut self.0
    }
}

impl<A, B> PairGet<1> for (A, B) {
    type Out = B;
    fn pair_get(&self) -> *const B {
        &self.1
    }
    fn pair_get_mut(&mut self) -> *mut B {
        &mut self.1
    }
}

pub trait PairGetRef<const N: usize> {
    type Out;
    fn pair_get(&self) -> crate::Ptr<Self::Out>;
}

impl<A, B> PairGetRef<0> for (crate::Value<A>, crate::Value<B>) {
    type Out = A;
    fn pair_get(&self) -> crate::Ptr<A> {
        crate::AsPointer::as_pointer(&self.0)
    }
}

impl<A, B> PairGetRef<1> for (crate::Value<A>, crate::Value<B>) {
    type Out = B;
    fn pair_get(&self) -> crate::Ptr<B> {
        crate::AsPointer::as_pointer(&self.1)
    }
}
