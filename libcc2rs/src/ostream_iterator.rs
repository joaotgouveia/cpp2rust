// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cell::{RefCell, UnsafeCell};
use std::ffi::CStr;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;

use crate::{AsPointer, ByteRepr, CppIterator, Ptr, UnsafeCppIterator, Value};

// The text `os << value` writes for an element of an ostream_iterator.
pub trait StreamInsert {
    fn stream_bytes(&self) -> Vec<u8>;
}

impl StreamInsert for i8 {
    fn stream_bytes(&self) -> Vec<u8> {
        vec![*self as u8]
    }
}

impl StreamInsert for u8 {
    fn stream_bytes(&self) -> Vec<u8> {
        vec![*self]
    }
}

impl StreamInsert for bool {
    fn stream_bytes(&self) -> Vec<u8> {
        (*self as i32).to_string().into_bytes()
    }
}

macro_rules! stream_insert_display {
    ($($t:ty),*) => {
        $(impl StreamInsert for $t {
            fn stream_bytes(&self) -> Vec<u8> {
                self.to_string().into_bytes()
            }
        })*
    };
}

stream_insert_display!(i16, u16, i32, u32, i64, u64, isize, usize);

// std::ostream_iterator: `*it = v` stores v, `++it` writes it and the
// delimiter to the stream.
pub struct UnsafeOstreamIterator<T> {
    stream: *mut File,
    delim: *const libc::c_char,
    slot: Rc<UnsafeCell<T>>,
}

impl<T: Default> UnsafeOstreamIterator<T> {
    pub fn new(stream: *mut File, delim: *const libc::c_char) -> Self {
        Self {
            stream,
            delim,
            slot: Rc::new(UnsafeCell::new(T::default())),
        }
    }
}

impl<T: Default> Default for UnsafeOstreamIterator<T> {
    fn default() -> Self {
        Self::new(std::ptr::null_mut(), std::ptr::null())
    }
}

impl<T> Clone for UnsafeOstreamIterator<T> {
    fn clone(&self) -> Self {
        Self {
            stream: self.stream,
            delim: self.delim,
            slot: self.slot.clone(),
        }
    }
}

impl<T: StreamInsert> UnsafeCppIterator<T> for UnsafeOstreamIterator<T> {
    fn iter_deref(&self) -> *mut T {
        self.slot.get()
    }

    fn iter_inc(&mut self) -> Self {
        unsafe {
            let _ = (*self.stream).write_all(&(*self.slot.get()).stream_bytes());
            if !self.delim.is_null() {
                let _ = (*self.stream).write_all(CStr::from_ptr(self.delim).to_bytes());
            }
        }
        self.clone()
    }

    fn iter_post_inc(&mut self) -> Self {
        self.iter_inc()
    }
}

pub struct OstreamIterator<T> {
    stream: Ptr<File>,
    delim: Ptr<i8>,
    slot: Value<T>,
}

impl<T: Default> OstreamIterator<T> {
    pub fn new(stream: Ptr<File>, delim: Ptr<i8>) -> Self {
        Self {
            stream,
            delim,
            slot: Rc::new(RefCell::new(T::default())),
        }
    }
}

impl<T: Default> Default for OstreamIterator<T> {
    fn default() -> Self {
        Self::new(Ptr::null(), Ptr::null())
    }
}

impl<T> Clone for OstreamIterator<T> {
    fn clone(&self) -> Self {
        Self {
            stream: self.stream.clone(),
            delim: self.delim.clone(),
            slot: self.slot.clone(),
        }
    }
}

impl<T: StreamInsert + ByteRepr> CppIterator<T> for OstreamIterator<T> {
    fn iter_deref(&self) -> Ptr<T> {
        self.slot.as_pointer()
    }

    fn iter_inc(&mut self) -> Self {
        let _ = self.stream.write_all(&self.slot.borrow().stream_bytes());
        if !self.delim.is_null() {
            let _ = self.delim.with_c_str(|__cs| {
                self.stream
                    .write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
            });
        }
        self.clone()
    }

    fn iter_post_inc(&mut self) -> Self {
        self.iter_inc()
    }
}
