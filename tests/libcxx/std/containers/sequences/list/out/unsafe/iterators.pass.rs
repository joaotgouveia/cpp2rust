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
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *const (),
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_A__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut A,
}
impl min_pointer_A__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut A) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *mut A {
        return self.ptr_;
    }
    pub unsafe fn pointer_to(t: *mut A) -> min_pointer_A__std_integral_constant_unsigned_long__0__ {
        return min_pointer_A__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_A__std_integral_constant_unsigned_long__0__ {
    ptr_: *const A,
}
impl min_pointer_const_A__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *const A) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *const A {
        return self.ptr_;
    }
    pub unsafe fn pointer_to(
        t: *const A,
    ) -> min_pointer_const_A__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_A__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_A_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A {
    pub first: (),
    pub second: (),
}
pub unsafe fn test_4() -> () {
    {
        let mut c: () = unimplemented!();
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let c: () = unimplemented!();
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut c: () = unimplemented!();
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let t: [(); 10] = [0_(), 1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_(), 9_()];
        let mut c: () =
            std_list_int__std_allocator_int__::new_1({ unimplemented!() }, { unimplemented!() });
        let mut i: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (*unimplemented!()) = 10_();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        &(i);
        &(j);
    }
    {
        let mut c: () = unimplemented!();
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let c: () = unimplemented!();
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut c: () = unimplemented!();
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let t: [(); 10] = [0_(), 1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_(), 9_()];
        let mut c: () =
            std_list_int__min_allocator_int__::new_1({ unimplemented!() }, { unimplemented!() });
        let mut i: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (*unimplemented!()) = 10_();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        &(i);
        &(j);
    }
    {
        let mut c: () = unimplemented!();
        let mut i: () = unimplemented!();
        (*(unsafe {
            min_pointer_A__std_integral_constant_unsigned_long__0__::operator_arrow(&i)
        }))
        .first = 3_();
        let mut j : () = std___list_const_iterator_A__min_pointer_void__std_integral_constant_unsigned_long__0___ :: new_1 ( { & i } , )  ;
        unimplemented!();
    }
    {
        let mut ii1: () = unimplemented!();
        let mut ii2: () = unimplemented!();
        let mut ii4: () = ii1.clone();
        let mut cii: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
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
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
