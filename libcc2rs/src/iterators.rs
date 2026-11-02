// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::reinterpret::ByteRepr;
use crate::{
    CChar, IteratorSeam, MapNode, PostfixDec, PostfixInc, PrefixDec, PrefixInc, Ptr, UnsafeMapNode,
    Value,
};
use std::cmp::Ordering;
use std::convert::Infallible;
use std::marker::PhantomData;
use std::rc::Rc;

pub struct CStringIterator<T: CChar = u8> {
    pub(crate) ptr: Ptr<T>,
}

impl<T: CChar> Iterator for CStringIterator<T> {
    type Item = T;

    fn count(self) -> usize {
        self.ptr.c_str_len()
    }

    fn next(&mut self) -> Option<Self::Item> {
        // read until the null terminator
        let ch = self.ptr.read();
        if ch == T::default() {
            return None;
        }
        self.ptr += 1;
        Some(ch)
    }
}

pub trait CppIterator<T>: Clone {
    fn iter_deref(&self) -> Ptr<T>;
    fn iter_inc(&mut self) -> &mut Self;
    fn iter_post_inc(&mut self) -> Self;
}

pub trait CppInputIterator<T>: CppIterator<T> + PartialEq {}

pub trait CppBidirectionalIterator<T>: CppInputIterator<T> {
    fn iter_dec(&mut self) -> &mut Self;
    fn iter_post_dec(&mut self) -> Self;
}

pub trait CppRandomAccessIterator<T>: CppBidirectionalIterator<T> + PartialOrd {
    fn iter_add(&self, n: isize) -> Self;
    fn iter_add_assign(&mut self, n: isize) -> Self;
    fn iter_distance(&self, from: &Self) -> isize;
    fn iter_at(&self, n: isize) -> Ptr<T>;
}

pub trait UnsafeCppIterator<T>: Clone {
    fn iter_deref(&self) -> *mut T;
    fn iter_inc(&mut self) -> &mut Self;
    fn iter_post_inc(&mut self) -> Self;
}

pub trait UnsafeCppInputIterator<T>: UnsafeCppIterator<T> + PartialEq {}

pub trait UnsafeCppBidirectionalIterator<T>: UnsafeCppInputIterator<T> {
    fn iter_dec(&mut self) -> &mut Self;
    fn iter_post_dec(&mut self) -> Self;
}

pub trait UnsafeCppRandomAccessIterator<T>: UnsafeCppBidirectionalIterator<T> + PartialOrd {
    fn iter_add(&self, n: isize) -> Self;
    fn iter_add_assign(&mut self, n: isize) -> Self;
    fn iter_distance(&self, from: &Self) -> isize;
    fn iter_at(&self, n: isize) -> *mut T;
}

impl<T> CppIterator<T> for Ptr<T> {
    fn iter_deref(&self) -> Ptr<T> {
        self.clone()
    }
    fn iter_inc(&mut self) -> &mut Self {
        PrefixInc::prefix_inc(self)
    }
    fn iter_post_inc(&mut self) -> Self {
        PostfixInc::postfix_inc(self)
    }
}

impl<T> CppInputIterator<T> for Ptr<T> {}

impl<T> CppBidirectionalIterator<T> for Ptr<T> {
    fn iter_dec(&mut self) -> &mut Self {
        PrefixDec::prefix_dec(self)
    }
    fn iter_post_dec(&mut self) -> Self {
        PostfixDec::postfix_dec(self)
    }
}

impl<T> CppRandomAccessIterator<T> for Ptr<T> {
    fn iter_add(&self, n: isize) -> Self {
        self.offset(n)
    }
    fn iter_add_assign(&mut self, n: isize) -> Self {
        *self = self.offset(n);
        self.clone()
    }
    fn iter_distance(&self, from: &Self) -> isize {
        self.get_offset() as isize - from.get_offset() as isize
    }
    fn iter_at(&self, n: isize) -> Ptr<T> {
        self.offset(n)
    }
}

macro_rules! raw_pointer_iterator {
    ($ptr:ty, $deref:expr) => {
        impl<T> UnsafeCppIterator<T> for $ptr {
            fn iter_deref(&self) -> *mut T {
                $deref(*self)
            }
            fn iter_inc(&mut self) -> &mut Self {
                *self = self.wrapping_add(1);
                self
            }
            fn iter_post_inc(&mut self) -> Self {
                let old = *self;
                *self = self.wrapping_add(1);
                old
            }
        }

        impl<T> UnsafeCppInputIterator<T> for $ptr {}

        impl<T> UnsafeCppBidirectionalIterator<T> for $ptr {
            fn iter_dec(&mut self) -> &mut Self {
                *self = self.wrapping_sub(1);
                self
            }
            fn iter_post_dec(&mut self) -> Self {
                let old = *self;
                *self = self.wrapping_sub(1);
                old
            }
        }

        impl<T> UnsafeCppRandomAccessIterator<T> for $ptr {
            fn iter_add(&self, n: isize) -> Self {
                self.wrapping_offset(n)
            }
            fn iter_add_assign(&mut self, n: isize) -> Self {
                *self = self.wrapping_offset(n);
                *self
            }
            fn iter_distance(&self, from: &Self) -> isize {
                unsafe { self.offset_from(*from) }
            }
            fn iter_at(&self, n: isize) -> *mut T {
                $deref(self.wrapping_offset(n))
            }
        }
    };
}

raw_pointer_iterator!(*mut T, |p: *mut T| p);
raw_pointer_iterator!(*const T, |p: *const T| p.cast_mut());

macro_rules! uninhabited_iterator {
    ($name:ident, $what:literal) => {
        pub enum $name<T> {
            #[doc(hidden)]
            None(Infallible, PhantomData<T>),
        }

        impl<T> $name<T> {
            pub fn uninitialized() -> Self {
                panic!(concat!("no container provides ", $what, " yet"))
            }

            fn never(&self) -> ! {
                match self {
                    Self::None(never, _) => match *never {},
                }
            }
        }

        impl<T> Clone for $name<T> {
            fn clone(&self) -> Self {
                self.never()
            }
        }

        impl<T: 'static> ByteRepr for $name<T> {}

        impl<T> PartialEq for $name<T> {
            fn eq(&self, _: &Self) -> bool {
                self.never()
            }
        }
    };
}

macro_rules! uninhabited_iterator_ops {
    ($name:ident, $iter:ident, $ptr:ty) => {
        impl<T> $iter<T> for $name<T> {
            fn iter_deref(&self) -> $ptr {
                self.never()
            }
            fn iter_inc(&mut self) -> &mut Self {
                self.never()
            }
            fn iter_post_inc(&mut self) -> Self {
                self.never()
            }
        }
    };
}

fn seam_eq<T>(a: &dyn IteratorSeam<T>, b: &dyn IteratorSeam<T>) -> bool {
    let other = b.as_any();
    assert!(
        a.as_any().type_id() == other.type_id(),
        "comparison of iterators of different types"
    );
    a.eq_seam(other)
}

macro_rules! seam_iterator {
    ($name:ident) => {
        pub enum $name<T> {
            Seam(Box<dyn IteratorSeam<T>>),
        }

        impl<T> $name<T> {
            pub fn seam(seam: impl IteratorSeam<T> + 'static) -> Self {
                Self::Seam(Box::new(seam))
            }

            pub fn uninitialized() -> Self {
                panic!(concat!(
                    stringify!($name),
                    " has no default value, build it with ",
                    stringify!($name),
                    "::seam"
                ))
            }
        }

        impl<T> Clone for $name<T> {
            fn clone(&self) -> Self {
                match self {
                    Self::Seam(seam) => Self::Seam(seam.clone_seam()),
                }
            }
        }

        impl<T: 'static> ByteRepr for $name<T> {}

        impl<T> PartialEq for $name<T> {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (Self::Seam(a), Self::Seam(b)) => seam_eq(&**a, &**b),
                }
            }
        }

        impl<T> CppIterator<T> for $name<T> {
            fn iter_deref(&self) -> Ptr<T> {
                match self {
                    Self::Seam(seam) => seam.deref(),
                }
            }
            fn iter_inc(&mut self) -> &mut Self {
                match self {
                    Self::Seam(seam) => seam.inc(),
                }
                self
            }
            fn iter_post_inc(&mut self) -> Self {
                let old = self.clone();
                self.iter_inc();
                old
            }
        }
    };
}

seam_iterator!(InputIterator);
seam_iterator!(OutputIterator);
seam_iterator!(ForwardIterator);
uninhabited_iterator!(UnsafeInputIterator, "input iterators");
uninhabited_iterator!(UnsafeOutputIterator, "output iterators");
uninhabited_iterator!(UnsafeForwardIterator, "forward iterators");

uninhabited_iterator_ops!(UnsafeInputIterator, UnsafeCppIterator, *mut T);
uninhabited_iterator_ops!(UnsafeOutputIterator, UnsafeCppIterator, *mut T);
uninhabited_iterator_ops!(UnsafeForwardIterator, UnsafeCppIterator, *mut T);

impl<T> CppInputIterator<T> for InputIterator<T> {}
impl<T> CppInputIterator<T> for ForwardIterator<T> {}
impl<T> UnsafeCppInputIterator<T> for UnsafeInputIterator<T> {}
impl<T> UnsafeCppInputIterator<T> for UnsafeForwardIterator<T> {}

pub enum BidirectionalIterator<T> {
    Map(Value<MapNode<T>>),
    Seam(Box<dyn IteratorSeam<T>>),
}

impl<T> BidirectionalIterator<T> {
    pub fn seam(seam: impl IteratorSeam<T> + 'static) -> Self {
        Self::Seam(Box::new(seam))
    }
}

impl<T> Default for BidirectionalIterator<T> {
    fn default() -> Self {
        Self::Map(MapNode::header())
    }
}

impl<T> Clone for BidirectionalIterator<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Map(node) => Self::Map(node.clone()),
            Self::Seam(seam) => Self::Seam(seam.clone_seam()),
        }
    }
}

impl<T: 'static> ByteRepr for BidirectionalIterator<T> {}

impl<T> PartialEq for BidirectionalIterator<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Map(a), Self::Map(b)) => Rc::ptr_eq(a, b),
            (Self::Seam(a), Self::Seam(b)) => seam_eq(&**a, &**b),
            _ => panic!("comparison of iterators of different types"),
        }
    }
}

impl<T> CppIterator<T> for BidirectionalIterator<T> {
    fn iter_deref(&self) -> Ptr<T> {
        match self {
            Self::Map(node) => MapNode::value(node),
            Self::Seam(seam) => seam.deref(),
        }
    }
    fn iter_inc(&mut self) -> &mut Self {
        match self {
            Self::Map(node) => *node = MapNode::next(node),
            Self::Seam(seam) => seam.inc(),
        }
        self
    }
    fn iter_post_inc(&mut self) -> Self {
        let old = self.clone();
        self.iter_inc();
        old
    }
}

impl<T> CppInputIterator<T> for BidirectionalIterator<T> {}

impl<T> CppBidirectionalIterator<T> for BidirectionalIterator<T> {
    fn iter_dec(&mut self) -> &mut Self {
        match self {
            Self::Map(node) => *node = MapNode::prev(node),
            Self::Seam(seam) => seam.dec(),
        }
        self
    }
    fn iter_post_dec(&mut self) -> Self {
        let old = self.clone();
        self.iter_dec();
        old
    }
}

pub enum UnsafeBidirectionalIterator<T> {
    Map(*mut UnsafeMapNode<T>),
}

impl<T> Default for UnsafeBidirectionalIterator<T> {
    fn default() -> Self {
        Self::Map(std::ptr::null_mut())
    }
}

impl<T> Clone for UnsafeBidirectionalIterator<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Map(node) => Self::Map(*node),
        }
    }
}

impl<T: 'static> ByteRepr for UnsafeBidirectionalIterator<T> {}

impl<T> PartialEq for UnsafeBidirectionalIterator<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Map(a), Self::Map(b)) => a == b,
        }
    }
}

impl<T> UnsafeCppIterator<T> for UnsafeBidirectionalIterator<T> {
    fn iter_deref(&self) -> *mut T {
        match self {
            Self::Map(node) => UnsafeMapNode::value(*node),
        }
    }
    fn iter_inc(&mut self) -> &mut Self {
        match self {
            Self::Map(node) => *node = UnsafeMapNode::next(*node),
        }
        self
    }
    fn iter_post_inc(&mut self) -> Self {
        let old = self.clone();
        self.iter_inc();
        old
    }
}

impl<T> UnsafeCppInputIterator<T> for UnsafeBidirectionalIterator<T> {}

impl<T> UnsafeCppBidirectionalIterator<T> for UnsafeBidirectionalIterator<T> {
    fn iter_dec(&mut self) -> &mut Self {
        match self {
            Self::Map(node) => *node = UnsafeMapNode::prev(*node),
        }
        self
    }
    fn iter_post_dec(&mut self) -> Self {
        let old = self.clone();
        self.iter_dec();
        old
    }
}

macro_rules! random_access_iterator {
    ($name:ident, $ptr:ty, $iter:ident, $input:ident, $bidi:ident, $random:ident, $null:expr) => {
        pub enum $name<T> {
            Vec($ptr),
        }

        impl<T> Default for $name<T> {
            fn default() -> Self {
                Self::Vec($null)
            }
        }

        impl<T> Clone for $name<T> {
            fn clone(&self) -> Self {
                match self {
                    Self::Vec(p) => Self::Vec(p.clone()),
                }
            }
        }

        impl<T: 'static> ByteRepr for $name<T> {}

        impl<T> PartialEq for $name<T> {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (Self::Vec(a), Self::Vec(b)) => a == b,
                }
            }
        }

        impl<T> PartialOrd for $name<T> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                match (self, other) {
                    (Self::Vec(a), Self::Vec(b)) => a.partial_cmp(b),
                }
            }
        }

        impl<T> $iter<T> for $name<T> {
            fn iter_deref(&self) -> $ptr {
                match self {
                    Self::Vec(p) => $iter::<T>::iter_deref(p),
                }
            }
            fn iter_inc(&mut self) -> &mut Self {
                match self {
                    Self::Vec(p) => {
                        $iter::<T>::iter_inc(p);
                    }
                }
                self
            }
            fn iter_post_inc(&mut self) -> Self {
                match self {
                    Self::Vec(p) => Self::Vec($iter::<T>::iter_post_inc(p)),
                }
            }
        }

        impl<T> $input<T> for $name<T> {}

        impl<T> $bidi<T> for $name<T> {
            fn iter_dec(&mut self) -> &mut Self {
                match self {
                    Self::Vec(p) => {
                        $bidi::<T>::iter_dec(p);
                    }
                }
                self
            }
            fn iter_post_dec(&mut self) -> Self {
                match self {
                    Self::Vec(p) => Self::Vec($bidi::<T>::iter_post_dec(p)),
                }
            }
        }

        impl<T> $random<T> for $name<T> {
            fn iter_add(&self, n: isize) -> Self {
                match self {
                    Self::Vec(p) => Self::Vec($random::<T>::iter_add(p, n)),
                }
            }
            fn iter_add_assign(&mut self, n: isize) -> Self {
                match self {
                    Self::Vec(p) => Self::Vec($random::<T>::iter_add_assign(p, n)),
                }
            }
            fn iter_distance(&self, from: &Self) -> isize {
                match (self, from) {
                    (Self::Vec(a), Self::Vec(b)) => $random::<T>::iter_distance(a, b),
                }
            }
            fn iter_at(&self, n: isize) -> $ptr {
                match self {
                    Self::Vec(p) => $random::<T>::iter_at(p, n),
                }
            }
        }
    };
}

random_access_iterator!(
    RandomAccessIterator,
    Ptr<T>,
    CppIterator,
    CppInputIterator,
    CppBidirectionalIterator,
    CppRandomAccessIterator,
    Ptr::null()
);

impl<T> RandomAccessIterator<T> {
    pub fn index(&self) -> usize {
        match self {
            Self::Vec(p) => p.get_offset(),
        }
    }
}

random_access_iterator!(
    UnsafeRandomAccessIterator,
    *mut T,
    UnsafeCppIterator,
    UnsafeCppInputIterator,
    UnsafeCppBidirectionalIterator,
    UnsafeCppRandomAccessIterator,
    std::ptr::null_mut()
);

pub trait IterStep: Clone + PartialEq {
    fn iter_step(&mut self);
}

macro_rules! iter_step {
    ($name:ty, $iter:ident) => {
        impl<T> IterStep for $name {
            fn iter_step(&mut self) {
                $iter::<T>::iter_inc(self);
            }
        }
    };
}

iter_step!(Ptr<T>, CppIterator);
iter_step!(*mut T, UnsafeCppIterator);
iter_step!(*const T, UnsafeCppIterator);
iter_step!(InputIterator<T>, CppIterator);
iter_step!(ForwardIterator<T>, CppIterator);
iter_step!(UnsafeInputIterator<T>, UnsafeCppIterator);
iter_step!(UnsafeForwardIterator<T>, UnsafeCppIterator);
iter_step!(BidirectionalIterator<T>, CppIterator);
iter_step!(UnsafeBidirectionalIterator<T>, UnsafeCppIterator);
iter_step!(RandomAccessIterator<T>, CppIterator);
iter_step!(UnsafeRandomAccessIterator<T>, UnsafeCppIterator);

pub struct IterRange<I> {
    cur: I,
    end: I,
}

impl<I: IterStep> IterRange<I> {
    pub fn new(cur: I, end: I) -> Self {
        Self { cur, end }
    }
}

impl<I: IterStep> Iterator for IterRange<I> {
    type Item = I;

    fn next(&mut self) -> Option<I> {
        if self.cur == self.end {
            return None;
        }
        let it = self.cur.clone();
        self.cur.iter_step();
        Some(it)
    }
}

pub trait CppRange {
    type Iter;
    fn begin(&self) -> Self::Iter;
    fn end(&self) -> Self::Iter;
}

impl<T> CppRange for Vec<T> {
    type Iter = UnsafeRandomAccessIterator<T>;
    fn begin(&self) -> Self::Iter {
        UnsafeRandomAccessIterator::Vec(self.as_ptr().cast_mut())
    }
    fn end(&self) -> Self::Iter {
        UnsafeRandomAccessIterator::Vec(self.as_ptr().wrapping_add(self.len()).cast_mut())
    }
}

impl<T> CppRange for Ptr<T> {
    type Iter = RandomAccessIterator<T>;
    fn begin(&self) -> Self::Iter {
        RandomAccessIterator::Vec(self.clone())
    }
    fn end(&self) -> Self::Iter {
        RandomAccessIterator::Vec(self.to_end())
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::{
        BidirectionalIterator, CppBidirectionalIterator, CppIterator, ForwardIterator,
        InputIterator, IterRange, IterStep, IteratorSeam, OutputIterator, Ptr,
    };
    use std::any::Any;

    pub(crate) struct IndexSeam<T> {
        pub(crate) data: Ptr<T>,
        pub(crate) idx: isize,
    }

    impl<T: 'static> IteratorSeam<T> for IndexSeam<T> {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn clone_seam(&self) -> Box<dyn IteratorSeam<T>> {
            Box::new(IndexSeam {
                data: self.data.clone(),
                idx: self.idx,
            })
        }
        fn eq_seam(&self, other: &dyn Any) -> bool {
            other
                .downcast_ref::<Self>()
                .is_some_and(|other| self.idx == other.idx)
        }
        fn deref(&self) -> Ptr<T> {
            self.data.offset(self.idx)
        }
        fn inc(&mut self) {
            self.idx += 1;
        }
        fn dec(&mut self) {
            self.idx -= 1;
        }
    }

    pub(crate) struct NoDecSeam<T>(pub(crate) IndexSeam<T>);

    impl<T: 'static> IteratorSeam<T> for NoDecSeam<T> {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn clone_seam(&self) -> Box<dyn IteratorSeam<T>> {
            Box::new(NoDecSeam(IndexSeam {
                data: self.0.data.clone(),
                idx: self.0.idx,
            }))
        }
        fn eq_seam(&self, other: &dyn Any) -> bool {
            other
                .downcast_ref::<Self>()
                .is_some_and(|other| self.0.idx == other.0.idx)
        }
        fn deref(&self) -> Ptr<T> {
            self.0.deref()
        }
        fn inc(&mut self) {
            self.0.inc();
        }
    }

    fn data() -> Ptr<i32> {
        Ptr::alloc_array(vec![10, 20, 30].into_boxed_slice())
    }

    fn at(data: &Ptr<i32>, idx: isize) -> IndexSeam<i32> {
        IndexSeam {
            data: data.clone(),
            idx,
        }
    }

    fn collect<I: IterStep + CppIterator<i32>>(begin: I, end: I) -> Vec<i32> {
        IterRange::new(begin, end)
            .map(|it| it.iter_deref().read())
            .collect()
    }

    macro_rules! seam_tests {
        ($module:ident, $name:ident) => {
            mod $module {
                use super::{NoDecSeam, at, data};
                use crate::{CppIterator, $name};

                #[test]
                fn deref_and_inc() {
                    let data = data();
                    let mut it = $name::seam(at(&data, 0));
                    assert_eq!(it.iter_deref().read(), 10);
                    assert_eq!(it.iter_inc().iter_deref().read(), 20);
                    let old = it.iter_post_inc();
                    assert_eq!(old.iter_deref().read(), 20);
                    assert_eq!(it.iter_deref().read(), 30);
                }

                #[test]
                fn clone_is_independent() {
                    let data = data();
                    let it = $name::seam(at(&data, 0));
                    let mut copy = it.clone();
                    copy.iter_inc();
                    assert_eq!(it.iter_deref().read(), 10);
                    assert_eq!(copy.iter_deref().read(), 20);
                }

                #[test]
                fn eq_same_type() {
                    let data = data();
                    assert!($name::seam(at(&data, 1)) == $name::seam(at(&data, 1)));
                    assert!($name::seam(at(&data, 1)) != $name::seam(at(&data, 2)));
                }

                #[test]
                #[should_panic(expected = "comparison of iterators of different types")]
                fn eq_different_types_panics() {
                    let data = data();
                    let _ = $name::seam(at(&data, 0)) == $name::seam(NoDecSeam(at(&data, 0)));
                }
            }
        };
    }

    seam_tests!(input, InputIterator);
    seam_tests!(output, OutputIterator);
    seam_tests!(forward, ForwardIterator);
    seam_tests!(bidirectional, BidirectionalIterator);

    #[test]
    fn input_iterator_range() {
        let data = data();
        let values = collect(
            InputIterator::seam(at(&data, 0)),
            InputIterator::seam(at(&data, 3)),
        );
        assert_eq!(values, [10, 20, 30]);
    }

    #[test]
    fn forward_iterator_range() {
        let data = data();
        let values = collect(
            ForwardIterator::seam(at(&data, 0)),
            ForwardIterator::seam(at(&data, 3)),
        );
        assert_eq!(values, [10, 20, 30]);
    }

    #[test]
    fn bidirectional_iterator_range() {
        let data = data();
        let values = collect(
            BidirectionalIterator::seam(at(&data, 0)),
            BidirectionalIterator::seam(at(&data, 3)),
        );
        assert_eq!(values, [10, 20, 30]);
    }

    #[test]
    fn output_iterator_writes() {
        let data = data();
        let mut it = OutputIterator::seam(at(&data, 0));
        it.iter_post_inc().iter_deref().write(1);
        it.iter_deref().write(2);
        assert_eq!(data.read(), 1);
        assert_eq!(data.offset(1).read(), 2);
        assert_eq!(data.offset(2).read(), 30);
    }

    #[test]
    fn bidirectional_seam_dec() {
        let data = data();
        let mut it = BidirectionalIterator::seam(at(&data, 2));
        assert_eq!(it.iter_dec().iter_deref().read(), 20);
        let old = it.iter_post_dec();
        assert_eq!(old.iter_deref().read(), 20);
        assert_eq!(it.iter_deref().read(), 10);
    }

    #[test]
    #[should_panic(expected = "dec is not supported by this seam")]
    fn bidirectional_seam_without_dec_panics() {
        let data = data();
        BidirectionalIterator::seam(NoDecSeam(at(&data, 1))).iter_dec();
    }

    #[test]
    #[should_panic(expected = "comparison of iterators of different types")]
    fn bidirectional_map_against_seam_panics() {
        let data = data();
        let _ = BidirectionalIterator::default() == BidirectionalIterator::seam(at(&data, 0));
    }

    #[test]
    #[should_panic(
        expected = "InputIterator has no default value, build it with InputIterator::seam"
    )]
    fn input_iterator_uninitialized_panics() {
        let _ = InputIterator::<i32>::uninitialized();
    }

    #[test]
    #[should_panic(
        expected = "OutputIterator has no default value, build it with OutputIterator::seam"
    )]
    fn output_iterator_uninitialized_panics() {
        let _ = OutputIterator::<i32>::uninitialized();
    }

    #[test]
    #[should_panic(
        expected = "ForwardIterator has no default value, build it with ForwardIterator::seam"
    )]
    fn forward_iterator_uninitialized_panics() {
        let _ = ForwardIterator::<i32>::uninitialized();
    }
}
