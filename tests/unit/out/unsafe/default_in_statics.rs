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
pub struct Inner {
    pub v: (),
    pub name: *const (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Outer {
    pub p1: *mut (),
    pub p2: *const (),
    pub arr: [*mut (); 3],
    pub cp: *const (),
    pub pp: *mut *mut (),
    pub inner: Inner,
    pub x: (),
    pub fn_: Option<unsafe fn(()) -> ()>,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Foo {
    pub s1: *const (),
    pub s2: *const (),
    pub fn1: Option<unsafe fn(()) -> ()>,
    pub fn2: Option<unsafe fn(()) -> ()>,
    pub n: (),
}
pub static mut static_fn_0: std::cell::LazyCell<Option<unsafe fn(()) -> ()>> =
    std::cell::LazyCell::new(|| unsafe { None });
pub static mut static_outer_1: std::cell::LazyCell<Outer> =
    std::cell::LazyCell::new(|| unsafe { <Outer>::default() });
pub static mut static_inner_array_2: std::cell::LazyCell<[Inner; 2]> =
    std::cell::LazyCell::new(|| unsafe { [<Inner>::default(); 2] });
pub static mut static_foo_3: std::cell::LazyCell<Foo> = std::cell::LazyCell::new(|| unsafe {
    Foo {
        s1: c"hello".as_ptr(),
        s2: std::ptr::null(),
        fn1: None,
        fn2: None,
        n: 42_(),
    }
});
pub static mut static_foo_array_4: std::cell::LazyCell<[Foo; 2]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            Foo {
                s1: c"first".as_ptr(),
                s2: std::ptr::null(),
                fn1: None,
                fn2: None,
                n: 1_(),
            },
            Foo {
                s1: c"second".as_ptr(),
                s2: std::ptr::null(),
                fn1: None,
                fn2: None,
                n: 2_(),
            },
        ]
    });
pub unsafe fn check_local_static_5() {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
static mut local_outer_6: std::cell::LazyCell<Outer> =
    std::cell::LazyCell::new(|| unsafe { <Outer>::default() });
static mut local_fn_7: std::cell::LazyCell<Option<unsafe fn(()) -> ()>> =
    std::cell::LazyCell::new(|| unsafe { None });
static mut local_p_8: std::cell::LazyCell<*mut ()> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null_mut() });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (3_())) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (2_())) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (2_())) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    (unsafe { check_local_static_5() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const static_fn_0);
    std::cell::LazyCell::force(&*&raw const static_outer_1);
    std::cell::LazyCell::force(&*&raw const static_inner_array_2);
    std::cell::LazyCell::force(&*&raw const static_foo_3);
    std::cell::LazyCell::force(&*&raw const static_foo_array_4);
}
