extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut copy_called_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut move_called_1: std::cell::LazyCell<()> =
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
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_0)) = true;
        this
    }
    pub unsafe fn move_from(a: *mut A1_int_) -> Self {
        let mut this = Self {
            id_: (unsafe { A1_int_::id(&(*a)) }),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_1)) = true;
        this
    }
}
impl Clone for A1_int_ {
    fn clone(&self) -> Self {
        unsafe { A1_int_::copy_from(self as *const A1_int_) }
    }
}
pub static mut copy_called_4: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut move_called_5: std::cell::LazyCell<()> =
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
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_4)) = true;
        this
    }
    pub unsafe fn move_from(a: *mut A2_int_) -> Self {
        let mut this = Self {
            id_: (unsafe { A2_int_::id(&(*a)) }),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_5)) = true;
        this
    }
}
impl Clone for A2_int_ {
    fn clone(&self) -> Self {
        unsafe { A2_int_::copy_from(self as *const A2_int_) }
    }
}
pub static mut copy_called_7: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut move_called_8: std::cell::LazyCell<()> =
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
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_7)) = true;
        this
    }
    pub unsafe fn move_from(a: *mut A3_int_) -> Self {
        let mut this = Self {
            id_: (unsafe { A3_int_::id(&(*a)) }),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_8)) = true;
        this
    }
    pub unsafe fn select_on_container_copy_construction(&self) -> A3_int_ {
        return A3_int_::new({ -1_() });
    }
}
impl Clone for A3_int_ {
    fn clone(&self) -> Self {
        unsafe { A3_int_::copy_from(self as *const A3_int_) }
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
        let mut a1: () = unimplemented!();
        unimplemented!();
        let mut a2: () = (unsafe {
            std_allocator_traits_std_scoped_allocator_adaptor_A1_int___::select_on_container_copy_construction ( & a1 , )
        });
        unimplemented!();
    }
    {
        let mut a1: () = unimplemented!();
        unimplemented!();
        let mut a2: () = (unsafe {
            std_allocator_traits_std_scoped_allocator_adaptor_A3_int___::select_on_container_copy_construction ( & a1 , )
        });
        unimplemented!();
    }
    {
        let mut a1: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut a2: () = (unsafe {
            std_allocator_traits_std_scoped_allocator_adaptor_A1_int___A2_int___A3_int___::select_on_container_copy_construction ( & a1 , )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copy_called_0);
    std::cell::LazyCell::force(&*&raw const move_called_1);
    std::cell::LazyCell::force(&*&raw const copy_called_4);
    std::cell::LazyCell::force(&*&raw const move_called_5);
    std::cell::LazyCell::force(&*&raw const copy_called_7);
    std::cell::LazyCell::force(&*&raw const move_called_8);
}
