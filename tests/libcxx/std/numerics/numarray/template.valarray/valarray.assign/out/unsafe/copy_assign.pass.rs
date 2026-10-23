extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct S {
    pub x_: (),
}
impl S {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { x_: 0_() };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut default_ctor_called_0)) = true;
        this
    }
    pub unsafe fn new_2(mut x: ()) -> Self {
        let mut this = Self { x_: x };
        this
    }
}
impl std::cmp::PartialEq for S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_1(self as *const S, other as *const S) }
    }
}
impl std::cmp::Eq for S {}
impl Default for S {
    fn default() -> Self {
        unsafe { S::new_1() }
    }
}
pub static mut default_ctor_called_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub unsafe fn operator_eq_1(lhs: *const S, rhs: *const S) -> () {
    return (((*lhs).x_) == ((*rhs).x_));
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
        let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
        let N: () = (::std::mem::size_of::<[(); 5]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 2.5_f64, 3_(), 4.25_f64, 5_()];
        let N: () = (::std::mem::size_of::<[(); 5]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let mut a: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let N: () = (::std::mem::size_of::<[(); 5]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (N)) {
            unimplemented!();
            let mut j: () = 0_();
            'loop_: while ((j) < (unimplemented!())) {
                unimplemented!();
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let mut a: [S; 5] = [
            S::new_2({ 1_() }),
            S::new_2({ 2_() }),
            S::new_2({ 3_() }),
            S::new_2({ 4_() }),
            S::new_2({ 5_() }),
        ];
        let N: () = (::std::mem::size_of::<[S; 5]>() as ())
            .wrapping_div((::std::mem::size_of::<S>() as ()));
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const default_ctor_called_0);
}
