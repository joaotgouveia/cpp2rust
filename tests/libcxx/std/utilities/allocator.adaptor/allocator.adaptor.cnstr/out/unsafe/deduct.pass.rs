extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_scoped_allocator_adaptor_A1_int____std_scoped_allocator_adaptor_A1_int___
{}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_scoped_allocator_adaptor_A1_int___A2_int____std_scoped_allocator_adaptor_A1_int___A2_int___
{}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_scoped_allocator_adaptor_A1_int___A2_int___A2_float____std_scoped_allocator_adaptor_A1_int___A2_int___A2_float___
{}
pub static mut copy_called_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut move_called_4: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct A1_int_ {
    id_: (),
}
impl A1_int_ {
    pub unsafe fn new(mut id: ()) -> Self {
        let mut this = Self { id_: id };
        this
    }
    pub unsafe fn id(&self) -> () {
        return self.id_;
    }
    pub unsafe fn copy_from(a: *const A1_int_) -> Self {
        let mut this = Self {
            id_: (unsafe { A1_int_::id(&(*a)) }),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_3)) = true;
        this
    }
    pub unsafe fn move_from(a: *mut A1_int_) -> Self {
        let mut this = Self {
            id_: (unsafe { A1_int_::id(&(*a)) }),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_4)) = true;
        this
    }
}
impl Clone for A1_int_ {
    fn clone(&self) -> Self {
        unsafe { A1_int_::copy_from(self as *const A1_int_) }
    }
}
pub static mut copy_called_7: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(VaArg, FnPtrArg, Default)]
pub struct A2_int_ {
    id_: (),
}
impl A2_int_ {
    pub unsafe fn new(mut id: ()) -> Self {
        let mut this = Self { id_: id };
        this
    }
    pub unsafe fn id(&self) -> () {
        return self.id_;
    }
    pub unsafe fn copy_from(a: *const A2_int_) -> Self {
        let mut this = Self {
            id_: (unsafe { A2_int_::id(&(*a)) }),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_7)) = true;
        this
    }
}
impl Clone for A2_int_ {
    fn clone(&self) -> Self {
        unsafe { A2_int_::copy_from(self as *const A2_int_) }
    }
}
pub static mut copy_called_10: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(VaArg, FnPtrArg, Default)]
pub struct A2_float_ {
    id_: (),
}
impl A2_float_ {
    pub unsafe fn new(mut id: ()) -> Self {
        let mut this = Self { id_: id };
        this
    }
    pub unsafe fn id(&self) -> () {
        return self.id_;
    }
    pub unsafe fn copy_from(a: *const A2_float_) -> Self {
        let mut this = Self {
            id_: (unsafe { A2_float_::id(&(*a)) }),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_10)) = true;
        this
    }
}
impl Clone for A2_float_ {
    fn clone(&self) -> Self {
        unsafe { A2_float_::copy_from(self as *const A2_float_) }
    }
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
    {
        let mut outer: A1_int_ = A1_int_::new({ 3_() });
        let mut a: () = std_scoped_allocator_adaptor_A1_int__::new_1({ &mut outer });
        const _: () = assert!(true, "");;
    }
    {
        let mut a: () = A1_int_::new({ 3_() });
        const _: () = assert!(true, "");;
    }
    {
        let mut outer: A1_int_ = A1_int_::new({ 3_() });
        let mut inner: A2_int_ = A2_int_::new({ 4_() });
        let mut a: () =
            std_scoped_allocator_adaptor_A1_int___A2_int__::new_1({ &mut outer }, { &inner });
        const _: () = assert!(true, "");;
    }
    {
        let mut outer: A1_int_ = A1_int_::new({ 3_() });
        let mut inner: A2_int_ = A2_int_::new({ 4_() });
        let mut a: () = {
            let mut __tmp_0: A2_float_ = A2_float_::new({ 5_() });
            std_scoped_allocator_adaptor_A1_int___A2_int___A2_float__::new_1(
                { &mut outer },
                { &inner },
                { &mut __tmp_0 },
            )
        };
        const _: () = assert!(true, "");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copy_called_3);
    std::cell::LazyCell::force(&*&raw const move_called_4);
    std::cell::LazyCell::force(&*&raw const copy_called_7);
    std::cell::LazyCell::force(&*&raw const copy_called_10);
}
