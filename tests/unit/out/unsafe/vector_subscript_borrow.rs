extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut next_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub unsafe fn bump_1(mut v: *mut (), mut x: ()) -> () {
    (&mut (*v))[(0_()) as usize] += x;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut next_0)).postfix_inc();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut v: () = unimplemented!();
    let mut items: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        items[(i) as usize] = (i).wrapping_mul(2_());
        i.prefix_inc();
    }
    items[(1_()) as usize] = (items[(1_()) as usize]).wrapping_add(5_());
    items[(2_()) as usize].postfix_inc();
    unimplemented!();
    (*{
        let __idx = v[(0_()) as usize] as usize;
        &mut v[__idx]
    }) = 3_();
    (*{
        let __idx = v[(1_()) as usize] as usize;
        &mut v[__idx]
    }) = ((v[(1_()) as usize]) + (4_()));
    unimplemented!();
    let mut p: *mut () = (&mut v[(0_()) as usize] as *mut ());
    v[(*p) as usize] = 9_();
    unimplemented!();
    (*{
        let __idx = (unsafe { bump_1((&mut v as *mut ()), 10_()) }) as usize;
        &mut v[__idx]
    }) = 2_();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const next_0);
}
