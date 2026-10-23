extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
impl A1_double_ {
    pub unsafe fn new_1(a: *mut A1_int_) -> Self {
        let mut this = Self {
            id_: (unsafe { A1_int_::id(&(*a)) }),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_0)) = true;
        this
    }
}
pub static mut move_called_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct A1_double_ {
    id_: (),
}
impl A1_double_ {
    pub unsafe fn id(&self) -> () {
        return self.id_;
    }
}
impl A1_int_ {
    pub unsafe fn new_1(a: *mut A1_double_) -> Self {
        let mut this = Self {
            id_: (unsafe { A1_double_::id(&(*a)) }),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_4)) = true;
        this
    }
}
pub static mut copy_called_5: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut move_called_4: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct A1_int_ {
    id_: (),
}
impl A1_int_ {
    pub unsafe fn new_2(mut id: ()) -> Self {
        let mut this = Self { id_: id };
        this
    }
    pub unsafe fn id(&self) -> () {
        return self.id_;
    }
}
pub unsafe fn operator_eq_8(x: *const A1_int_, y: *const A1_double_) -> () {
    return ((unsafe { A1_int_::id(&(*x)) }) == (unsafe { A1_double_::id(&(*y)) }));
}
pub static mut copy_called_9: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut move_called_10: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
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
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_9)) = true;
        this
    }
    pub unsafe fn move_from(a: *mut A2_int_) -> Self {
        let mut this = Self {
            id_: (unsafe { A2_int_::id(&(*a)) }),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_10)) = true;
        this
    }
}
impl std::cmp::PartialEq for A2_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_12(self as *const A2_int_, other as *const A2_int_) }
    }
}
impl std::cmp::Eq for A2_int_ {}
impl Clone for A2_int_ {
    fn clone(&self) -> Self {
        unsafe { A2_int_::copy_from(self as *const A2_int_) }
    }
}
pub unsafe fn operator_eq_12(x: *const A2_int_, y: *const A2_int_) -> () {
    return ((unsafe { A2_int_::id(&(*x)) }) == (unsafe { A2_int_::id(&(*y)) }));
}
pub static mut copy_called_13: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut move_called_14: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct A3_int_ {
    id_: (),
}
impl A3_int_ {
    pub unsafe fn new(mut id: ()) -> Self {
        let mut this = Self { id_: id };
        this
    }
    pub unsafe fn id(&self) -> () {
        return self.id_;
    }
    pub unsafe fn copy_from(a: *const A3_int_) -> Self {
        let mut this = Self {
            id_: (unsafe { A3_int_::id(&(*a)) }),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_13)) = true;
        this
    }
    pub unsafe fn move_from(a: *mut A3_int_) -> Self {
        let mut this = Self {
            id_: (unsafe { A3_int_::id(&(*a)) }),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_14)) = true;
        this
    }
}
impl std::cmp::PartialEq for A3_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_17(self as *const A3_int_, other as *const A3_int_) }
    }
}
impl std::cmp::Eq for A3_int_ {}
impl Clone for A3_int_ {
    fn clone(&self) -> Self {
        unsafe { A3_int_::copy_from(self as *const A3_int_) }
    }
}
pub unsafe fn operator_eq_17(x: *const A3_int_, y: *const A3_int_) -> () {
    return ((unsafe { A3_int_::id(&(*x)) }) == (unsafe { A3_int_::id(&(*y)) }));
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
        let mut a1: () = A1_int_::new_2({ 3_() });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_5)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_4)) = false;
        let mut a2: () = a1;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut a1: () = {
            let mut __tmp_0: A1_int_ = A1_int_::new_2({ 4_() });
            let mut __tmp_1: A2_int_ = A2_int_::new({ 5_() });
            std_scoped_allocator_adaptor_A1_double___A2_int__::new_1({ &mut __tmp_0 }, {
                &mut __tmp_1
            })
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_5)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_4)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_9)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_10)) = false;
        let mut a2: () = a1;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut a1: () = {
            let mut __tmp_2: A1_int_ = A1_int_::new_2({ 4_() });
            let mut __tmp_3: A2_int_ = A2_int_::new({ 5_() });
            let mut __tmp_4: A3_int_ = A3_int_::new({ 6_() });
            std_scoped_allocator_adaptor_A1_double___A2_int___A3_int__::new_1(
                { &mut __tmp_2 },
                { &mut __tmp_3 },
                { &mut __tmp_4 },
            )
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_5)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_4)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_9)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_10)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_13)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_14)) = false;
        let mut a2: () = a1;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const move_called_0);
    std::cell::LazyCell::force(&*&raw const copy_called_5);
    std::cell::LazyCell::force(&*&raw const move_called_4);
    std::cell::LazyCell::force(&*&raw const copy_called_9);
    std::cell::LazyCell::force(&*&raw const move_called_10);
    std::cell::LazyCell::force(&*&raw const copy_called_13);
    std::cell::LazyCell::force(&*&raw const move_called_14);
}
