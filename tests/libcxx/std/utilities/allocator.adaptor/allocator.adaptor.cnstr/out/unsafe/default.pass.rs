extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut copy_called_0: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut move_called_1: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(FnPtrArg)]
pub struct A1_int_ {
    id_: i32,
}
impl A1_int_ {
    pub unsafe fn new(mut id: Option<i32>) -> Self {
        let mut id: i32 = id.unwrap_or_else(|| unsafe { 0 });
        let mut this = Self { id_: id };
        this
    }
    pub unsafe fn id(&self) -> i32 {
        return self.id_;
    }
}
impl std::cmp::PartialEq for A1_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_4(self as *const A1_int_, other as *const A1_int_) }
    }
}
impl std::cmp::Eq for A1_int_ {}
impl Default for A1_int_ {
    fn default() -> Self {
        unsafe { A1_int_::new(None) }
    }
}
pub unsafe fn operator_eq_4(x: *const A1_int_, y: *const A1_int_) -> bool {
    return ((unsafe { A1_int_::id(&(*x)) }) == (unsafe { A1_int_::id(&(*y)) }));
}
pub static mut copy_called_5: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut move_called_6: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(FnPtrArg)]
pub struct A2_int_ {
    id_: i32,
}
impl A2_int_ {
    pub unsafe fn new(mut id: Option<i32>) -> Self {
        let mut id: i32 = id.unwrap_or_else(|| unsafe { 0 });
        let mut this = Self { id_: id };
        this
    }
    pub unsafe fn id(&self) -> i32 {
        return self.id_;
    }
}
impl std::cmp::PartialEq for A2_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_8(self as *const A2_int_, other as *const A2_int_) }
    }
}
impl std::cmp::Eq for A2_int_ {}
impl Default for A2_int_ {
    fn default() -> Self {
        unsafe { A2_int_::new(None) }
    }
}
pub unsafe fn operator_eq_8(x: *const A2_int_, y: *const A2_int_) -> bool {
    return ((unsafe { A2_int_::id(&(*x)) }) == (unsafe { A2_int_::id(&(*y)) }));
}
pub static mut copy_called_9: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut move_called_10: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(FnPtrArg)]
pub struct A3_int_ {
    id_: i32,
}
impl A3_int_ {
    pub unsafe fn new(mut id: Option<i32>) -> Self {
        let mut id: i32 = id.unwrap_or_else(|| unsafe { 0 });
        let mut this = Self { id_: id };
        this
    }
    pub unsafe fn id(&self) -> i32 {
        return self.id_;
    }
}
impl std::cmp::PartialEq for A3_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_13(self as *const A3_int_, other as *const A3_int_) }
    }
}
impl std::cmp::Eq for A3_int_ {}
impl Default for A3_int_ {
    fn default() -> Self {
        unsafe { A3_int_::new(None) }
    }
}
pub unsafe fn operator_eq_13(x: *const A3_int_, y: *const A3_int_) -> bool {
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    {
        let mut a: () = unimplemented!();
        assert!(
            (unsafe {
                let _x: *const A1_int_ = unimplemented!();
                let mut _y: A1_int_ = A1_int_::new(None);
                operator_eq_4(_x, &mut _y)
            })
        );
        assert!(unimplemented!());
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_0)) as i32)
                == (false as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_1)) as i32)
                == (false as i32))
        );
    }
    {
        let mut a: () = unimplemented!();
        assert!(
            (unsafe {
                let _x: *const A1_int_ = unimplemented!();
                let mut _y: A1_int_ = A1_int_::new(None);
                operator_eq_4(_x, &mut _y)
            })
        );
        assert!(unimplemented!());
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_0)) as i32)
                == (false as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_1)) as i32)
                == (false as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_5)) as i32)
                == (false as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_6)) as i32)
                == (false as i32))
        );
    }
    {
        let mut a: () = unimplemented!();
        assert!(
            (unsafe {
                let _x: *const A1_int_ = unimplemented!();
                let mut _y: A1_int_ = A1_int_::new(None);
                operator_eq_4(_x, &mut _y)
            })
        );
        assert!((unimplemented!()));
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_0)) as i32)
                == (false as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_1)) as i32)
                == (false as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_5)) as i32)
                == (false as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_6)) as i32)
                == (false as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_9)) as i32)
                == (false as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_10)) as i32)
                == (false as i32))
        );
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copy_called_0);
    std::cell::LazyCell::force(&*&raw const move_called_1);
    std::cell::LazyCell::force(&*&raw const copy_called_5);
    std::cell::LazyCell::force(&*&raw const move_called_6);
    std::cell::LazyCell::force(&*&raw const copy_called_9);
    std::cell::LazyCell::force(&*&raw const move_called_10);
}
