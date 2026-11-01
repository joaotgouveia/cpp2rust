// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::capture::MoveCtor;
use crate::fn_ptr::{FnPtr, FnSig};
use crate::rc::{AsPointer, Ptr, Value};
use crate::reinterpret::ByteRepr;
use crate::Map;

// The C++ move of a value of any type: the moved value, leaving the source in
// its moved-from state. A type without a move constructor is copied, as in
// C++. Records implement it with their move_from method, or by a copy.
pub trait CppMove: Sized {
    fn cpp_move(src: Ptr<Self>) -> Self;
}

macro_rules! impl_cpp_move_by_copy {
    ($($ty:ty),* $(,)?) => {$(
        impl CppMove for $ty {
            fn cpp_move(src: Ptr<Self>) -> Self {
                src.read()
            }
        }
    )*};
}

impl_cpp_move_by_copy!(
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64, bool, ()
);

impl<T: 'static> CppMove for Ptr<T> {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}

impl<T: 'static> CppMove for *const T {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}

impl<T: 'static> CppMove for *mut T {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}

macro_rules! impl_cpp_move_with_ctor {
    ($(<$($param:ident $(: $bound:ident)?),*> $ty:ty),* $(,)?) => {$(
        impl<$($param $(: $bound)?),*> CppMove for $ty
        where
            Self: MoveCtor,
        {
            fn cpp_move(src: Ptr<Self>) -> Self {
                Self::move_ctor(src)
            }
        }
    )*};
}

impl_cpp_move_with_ctor!(
    <T> Vec<T>,
    <K, V> BTreeMap<K, V>,
    <K, T> Map<K, T>,
    <T> Option<Value<T>>,
    <T: FnSig> FnPtr<T>,
);

impl<A: CppMove + ByteRepr, B: CppMove + ByteRepr> CppMove for (Value<A>, Value<B>) {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.with(|pair| {
            (
                Rc::new(RefCell::new(A::cpp_move(pair.0.as_pointer()))),
                Rc::new(RefCell::new(B::cpp_move(pair.1.as_pointer()))),
            )
        })
    }
}

impl<T: CppMove + ByteRepr + 'static> CppMove for Box<[T]> {
    fn cpp_move(src: Ptr<Self>) -> Self {
        let len = src.with(|elems| elems.len());
        let elems: Ptr<T> = src.decay();
        (0..len).map(|i| T::cpp_move(elems.offset(i))).collect()
    }
}

pub trait CppFrom<S>: Sized {
    fn cpp_from(src: Ptr<S>) -> Self;

    fn cpp_from_move(src: Ptr<S>) -> Self {
        Self::cpp_from(src)
    }
}

pub trait CppFromUnsafe<S>: Sized {
    /// # Safety
    /// `src` must point to a live value.
    unsafe fn cpp_from(src: *const S) -> Self;

    /// # Safety
    /// `src` must point to a live value.
    unsafe fn cpp_from_move(src: *mut S) -> Self {
        unsafe { Self::cpp_from(src) }
    }
}

impl<T: Clone + ByteRepr + CppMove> CppFrom<T> for T {
    fn cpp_from(src: Ptr<T>) -> T {
        src.read()
    }

    fn cpp_from_move(src: Ptr<T>) -> T {
        T::cpp_move(src)
    }
}

impl<T: Clone> CppFromUnsafe<T> for T {
    unsafe fn cpp_from(src: *const T) -> T {
        unsafe { (*src).clone() }
    }
}
