// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::fn_ptr::FnPtr;
use crate::fn_ptr_arg::FnPtrArg;
use crate::{ByteRepr, Ptr};

macro_rules! callable {
    ($name:ident; $($a:ident: $A:ident),*) => {
        pub trait $name<$($A,)* R> {
            fn call(&self, $($a: $A),*) -> R;
        }

        impl<F, $($A,)* R> $name<$($A,)* R> for F
        where
            F: Fn($($A),*) -> R,
        {
            #[inline]
            fn call(&self, $($a: $A),*) -> R {
                self($($a),*)
            }
        }

        impl<$($A,)* R> $name<$($A,)* R> for unsafe fn($($A),*) -> R {
            #[inline]
            fn call(&self, $($a: $A),*) -> R {
                unsafe { self($($a),*) }
            }
        }

        impl<$($A: FnPtrArg,)* R: FnPtrArg> $name<$($A,)* R>
            for FnPtr<fn($($A),*) -> R>
        {
            #[inline]
            fn call(&self, $($a: $A),*) -> R {
                self.call($($a),*)
            }
        }
    };
}

callable!(Callable0;);
callable!(Callable1; a1: A1);
callable!(Callable2; a1: A1, a2: A2);
callable!(Callable3; a1: A1, a2: A2, a3: A3);

// A comparator handed elements by pointer, whether it takes them by pointer
// (C++ `const T &`) or by value (C++ `T`).
pub trait UnsafeComparator<T> {
    unsafe fn unsafe_less(&self, a: *const T, b: *const T) -> bool;
}

impl<T, C: Callable2<*const T, *const T, bool>> UnsafeComparator<T> for C {
    #[inline]
    unsafe fn unsafe_less(&self, a: *const T, b: *const T) -> bool {
        self.call(a, b)
    }
}

impl<T: Clone + FnPtrArg> UnsafeComparator<T> for FnPtr<fn(T, T) -> bool> {
    #[inline]
    unsafe fn unsafe_less(&self, a: *const T, b: *const T) -> bool {
        unsafe { self.call((*a).clone(), (*b).clone()) }
    }
}

pub trait Comparator<T> {
    fn less(&self, a: Ptr<T>, b: Ptr<T>) -> bool;
}

impl<T, C: Callable2<Ptr<T>, Ptr<T>, bool>> Comparator<T> for C {
    #[inline]
    fn less(&self, a: Ptr<T>, b: Ptr<T>) -> bool {
        self.call(a, b)
    }
}

impl<T: Clone + ByteRepr + FnPtrArg> Comparator<T> for FnPtr<fn(T, T) -> bool> {
    #[inline]
    fn less(&self, a: Ptr<T>, b: Ptr<T>) -> bool {
        self.call(a.read(), b.read())
    }
}
