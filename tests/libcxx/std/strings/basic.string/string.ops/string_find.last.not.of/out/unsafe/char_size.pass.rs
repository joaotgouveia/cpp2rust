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
pub unsafe fn test_4(s: *const (), mut c: (), mut pos: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if ((x) != (unimplemented!())) {
        unimplemented!();
    }
}
pub unsafe fn test_5(s: *const (), mut c: (), mut pos: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if ((x) != (unimplemented!())) {
        unimplemented!();
    }
}
pub unsafe fn test_6(s: *const (), mut c: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if ((x) != (unimplemented!())) {
        unimplemented!();
    }
}
pub unsafe fn test_7(s: *const (), mut c: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if ((x) != (unimplemented!())) {
        unimplemented!();
    }
}
pub unsafe fn test_string_8() {
    (unsafe {
        let mut _s: () = unimplemented!();
        test_4(&mut _s, ('i' as ()), 0_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_4(&mut _s, ('i' as ()), 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_4(&mut _s, ('i' as ()), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_4(&mut _s, ('i' as ()), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_4(&mut _s, ('i' as ()), 2_(), 2_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_4(&mut _s, ('i' as ()), 4_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_4(&mut _s, ('i' as ()), 5_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_4(&mut _s, ('i' as ()), 6_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_4(&mut _s, ('i' as ()), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_4(&mut _s, ('i' as ()), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_4(&mut _s, ('i' as ()), 5_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_4(&mut _s, ('i' as ()), 9_(), 8_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_4(&mut _s, ('i' as ()), 10_(), 9_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_4(&mut _s, ('i' as ()), 11_(), 9_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_4(&mut _s, ('i' as ()), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_4(&mut _s, ('i' as ()), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_4(&mut _s, ('i' as ()), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_4(&mut _s, ('i' as ()), 19_(), 18_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_4(&mut _s, ('i' as ()), 20_(), 18_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_4(&mut _s, ('i' as ()), 21_(), 19_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_6(&mut _s, ('i' as ()), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_6(&mut _s, ('i' as ()), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_6(&mut _s, ('i' as ()), 9_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_6(&mut _s, ('i' as ()), 19_())
    });
}
pub unsafe fn test_string_9() {
    (unsafe {
        let mut _s: () = unimplemented!();
        test_5(&mut _s, ('i' as ()), 0_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_5(&mut _s, ('i' as ()), 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_5(&mut _s, ('i' as ()), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_5(&mut _s, ('i' as ()), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_5(&mut _s, ('i' as ()), 2_(), 2_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_5(&mut _s, ('i' as ()), 4_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_5(&mut _s, ('i' as ()), 5_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_5(&mut _s, ('i' as ()), 6_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_5(&mut _s, ('i' as ()), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_5(&mut _s, ('i' as ()), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_5(&mut _s, ('i' as ()), 5_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_5(&mut _s, ('i' as ()), 9_(), 8_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_5(&mut _s, ('i' as ()), 10_(), 9_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_5(&mut _s, ('i' as ()), 11_(), 9_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_5(&mut _s, ('i' as ()), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_5(&mut _s, ('i' as ()), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_5(&mut _s, ('i' as ()), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_5(&mut _s, ('i' as ()), 19_(), 18_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_5(&mut _s, ('i' as ()), 20_(), 18_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_5(&mut _s, ('i' as ()), 21_(), 19_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_7(&mut _s, ('i' as ()), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_7(&mut _s, ('i' as ()), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_7(&mut _s, ('i' as ()), 9_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_7(&mut _s, ('i' as ()), 19_())
    });
}
pub unsafe fn test_10() -> () {
    (unsafe { test_string_8() });
    (unsafe { test_string_9() });
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
    (unsafe { test_10() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
