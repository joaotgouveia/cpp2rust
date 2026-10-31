// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{AsPointer, Ptr};
use std::cell::RefCell;
use std::rc::Rc;

/// An iterator advanced by its prefix ++ (and --), as std::next and
/// std::prev advance it.
pub trait UnsafeCppAdvance {
    /// # Safety
    ///
    /// The iterator must stay within its range.
    unsafe fn cpp_inc(&mut self);

    /// # Safety
    ///
    /// The iterator must stay within its range.
    unsafe fn cpp_dec(&mut self) {
        panic!("iterator is not bidirectional")
    }
}

impl<T> UnsafeCppAdvance for *const T {
    unsafe fn cpp_inc(&mut self) {
        *self = self.wrapping_add(1);
    }
    unsafe fn cpp_dec(&mut self) {
        *self = self.wrapping_sub(1);
    }
}

impl<T> UnsafeCppAdvance for *mut T {
    unsafe fn cpp_inc(&mut self) {
        *self = self.wrapping_add(1);
    }
    unsafe fn cpp_dec(&mut self) {
        *self = self.wrapping_sub(1);
    }
}

/// # Safety
///
/// The iterator must stay within its range.
pub unsafe fn next_unsafe<I: UnsafeCppAdvance>(mut it: I, n: isize) -> I {
    unsafe { advance_unsafe(&mut it, n) };
    it
}

/// # Safety
///
/// The iterator must stay within its range.
pub unsafe fn prev_unsafe<I: UnsafeCppAdvance>(mut it: I, n: isize) -> I {
    unsafe { advance_unsafe(&mut it, -n) };
    it
}

unsafe fn advance_unsafe<I: UnsafeCppAdvance>(it: &mut I, n: isize) {
    for _ in 0..n.max(0) {
        unsafe { it.cpp_inc() };
    }
    for _ in 0..(-n).max(0) {
        unsafe { it.cpp_dec() };
    }
}

/// Refcount counterpart of `UnsafeCppAdvance`: the operators take the
/// iterator through a pointer to it.
pub trait CppAdvance: Sized {
    fn cpp_inc(it: &Ptr<Self>);

    fn cpp_dec(_it: &Ptr<Self>) {
        panic!("iterator is not bidirectional")
    }
}

impl<T: 'static> CppAdvance for Ptr<T> {
    fn cpp_inc(it: &Ptr<Self>) {
        it.with_mut(|p| *p = p.offset(1));
    }
    fn cpp_dec(it: &Ptr<Self>) {
        it.with_mut(|p| *p = p.offset(-1));
    }
}

fn advance_refcount<I: CppAdvance + 'static>(it: I, n: isize) -> I {
    let value = Rc::new(RefCell::new(it));
    {
        let ptr = value.as_pointer();
        for _ in 0..n.max(0) {
            I::cpp_inc(&ptr);
        }
        for _ in 0..(-n).max(0) {
            I::cpp_dec(&ptr);
        }
    }
    match Rc::try_unwrap(value) {
        Ok(cell) => cell.into_inner(),
        Err(_) => panic!("iterator escaped std::next"),
    }
}

pub fn next_refcount<I: CppAdvance + 'static>(it: I, n: isize) -> I {
    advance_refcount(it, n)
}

pub fn prev_refcount<I: CppAdvance + 'static>(it: I, n: isize) -> I {
    advance_refcount(it, -n)
}
