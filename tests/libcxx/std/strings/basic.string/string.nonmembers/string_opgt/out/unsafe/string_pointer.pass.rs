extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_3)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_0)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)) = 0_();
    }
}
pub static mut outstanding_bytes_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_char__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_char__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
        return self.ptr_;
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.ptr_.offset((n) as isize));
    }
    pub unsafe fn pointer_to(
        t: *mut (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    ptr_: *const (),
}
impl min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *const ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn new_2(mut p: min_pointer_char__std_integral_constant_unsigned_long__0__) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *const () {
        return self.ptr_;
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self
            as *mut min_pointer_const_char__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_const_char__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_const_char__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_const_char__std_integral_constant_unsigned_long__0__::operator_add_assign(
                &mut tmp, n,
            )
        });
        return tmp;
    }
    pub unsafe fn pointer_to(
        t: *const (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_char_ {}
impl min_allocator_char_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_char__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub unsafe fn test_4(lhs: *const (), mut rhs: *const (), mut x: ()) {
    unimplemented!();
}
pub unsafe fn test_5(lhs: *const (), mut rhs: *const (), mut x: ()) {
    unimplemented!();
}
pub unsafe fn test_string_6() {
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_4(&mut _lhs, c"".as_ptr(), false)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_4(&mut _lhs, c"abcde".as_ptr(), false)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_4(&mut _lhs, c"abcdefghij".as_ptr(), false)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_4(&mut _lhs, c"abcdefghijklmnopqrst".as_ptr(), false)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_4(&mut _lhs, c"".as_ptr(), true)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_4(&mut _lhs, c"abcde".as_ptr(), false)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_4(&mut _lhs, c"abcdefghij".as_ptr(), false)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_4(&mut _lhs, c"abcdefghijklmnopqrst".as_ptr(), false)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_4(&mut _lhs, c"".as_ptr(), true)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_4(&mut _lhs, c"abcde".as_ptr(), true)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_4(&mut _lhs, c"abcdefghij".as_ptr(), false)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_4(&mut _lhs, c"abcdefghijklmnopqrst".as_ptr(), false)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_4(&mut _lhs, c"".as_ptr(), true)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_4(&mut _lhs, c"abcde".as_ptr(), true)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_4(&mut _lhs, c"abcdefghij".as_ptr(), true)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_4(&mut _lhs, c"abcdefghijklmnopqrst".as_ptr(), false)
    });
}
pub unsafe fn test_string_7() {
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_5(&mut _lhs, c"".as_ptr(), false)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_5(&mut _lhs, c"abcde".as_ptr(), false)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_5(&mut _lhs, c"abcdefghij".as_ptr(), false)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_5(&mut _lhs, c"abcdefghijklmnopqrst".as_ptr(), false)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_5(&mut _lhs, c"".as_ptr(), true)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_5(&mut _lhs, c"abcde".as_ptr(), false)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_5(&mut _lhs, c"abcdefghij".as_ptr(), false)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_5(&mut _lhs, c"abcdefghijklmnopqrst".as_ptr(), false)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_5(&mut _lhs, c"".as_ptr(), true)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_5(&mut _lhs, c"abcde".as_ptr(), true)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_5(&mut _lhs, c"abcdefghij".as_ptr(), false)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_5(&mut _lhs, c"abcdefghijklmnopqrst".as_ptr(), false)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_5(&mut _lhs, c"".as_ptr(), true)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_5(&mut _lhs, c"abcde".as_ptr(), true)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_5(&mut _lhs, c"abcdefghij".as_ptr(), true)
    });
    (unsafe {
        let mut _lhs: () = unimplemented!();
        test_5(&mut _lhs, c"abcdefghijklmnopqrst".as_ptr(), false)
    });
}
pub unsafe fn test_8() -> () {
    (unsafe { test_string_6() });
    (unsafe { test_string_7() });
    return true;
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_8() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
