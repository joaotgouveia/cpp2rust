extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(i32, new_1, value)]
pub struct S {
    pub x_: i32,
}
impl S {
    pub unsafe fn new_2() -> Self {
        let mut this = Self { x_: 0 };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut default_ctor_called_0)) = true;
        this
    }
    pub unsafe fn new_1(mut x: i32) -> Self {
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
        unsafe { S::new_2() }
    }
}
pub static mut default_ctor_called_0: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub unsafe fn operator_eq_1(lhs: *const S, rhs: *const S) -> bool {
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    {
        let mut a: [i32; 5] = [1, 2, 3, 4, 5];
        let N: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        assert!(((unimplemented!()) == (unimplemented!())));
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((*unimplemented!()) == (*unimplemented!())));
            i.prefix_inc();
        }
    }
    {
        let mut a: [f64; 5] = [1_f64, 2.5_f64, 3_f64, 4.25_f64, 5_f64];
        let N: u32 = (((::std::mem::size_of::<[f64; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<f64>() as usize))) as u32);
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        assert!(((unimplemented!()) == (unimplemented!())));
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((*unimplemented!()) == (*unimplemented!())));
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
        let N: u32 = (((::std::mem::size_of::<[(); 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<()>() as usize))) as u32);
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        assert!(((unimplemented!()) == (unimplemented!())));
        let mut i: u32 = 0_u32;
        'loop_: while ((i) < (N)) {
            assert!(((unimplemented!()) == (unimplemented!())));
            let mut j: usize = 0_usize;
            'loop_: while ((j) < (unimplemented!())) {
                assert!(((*unimplemented!()) == (*unimplemented!())));
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let mut a: [S; 5] = [
            S::new_1({ 1 }),
            S::new_1({ 2 }),
            S::new_1({ 3 }),
            S::new_1({ 4 }),
            S::new_1({ 5 }),
        ];
        let N: u32 = (((::std::mem::size_of::<[S; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<S>() as usize))) as u32);
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        assert!(((unimplemented!()) == (unimplemented!())));
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(
                (unsafe {
                    let _lhs: *const S = &(*unimplemented!());
                    let _rhs: *const S = &(*unimplemented!());
                    operator_eq_1(_lhs, _rhs)
                })
            );
            i.prefix_inc();
        }
        assert!(!(*std::cell::LazyCell::force_mut(&mut *&raw mut default_ctor_called_0)));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const default_ctor_called_0);
}
