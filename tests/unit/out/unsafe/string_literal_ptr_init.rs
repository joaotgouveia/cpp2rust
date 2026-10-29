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
pub struct label {
    pub name: *const (),
    pub probe: Option<unsafe fn() -> ()>,
    pub mask: (),
}
pub unsafe fn probe_two_0() -> () {
    return 1_();
}
pub static mut table_1: std::cell::LazyCell<[label; 2]> = std::cell::LazyCell::new(|| unsafe {
    [
        label {
            name: ((c"first").as_ptr().cast_mut()).cast_const(),
            probe: None,
            mask: ((1_()) << (4_())),
        },
        label {
            name: ((c"second").as_ptr().cast_mut()).cast_const(),
            probe: (Some(probe_two_0)),
            mask: ((1_()) << (5_())),
        },
    ]
});
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
    unimplemented!();
    let mut tail: *const () =
        (&mut (*c"ab.cd".as_ptr().cast_mut().offset((2_()) as isize)) as *mut ()).cast_const();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut have: () = 0_();
    let mut p: () = if (have != 0) {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut table_1))[(0_()) as usize].name as ())
    } else {
        (c"".as_ptr().cast_mut() as ())
    };
    unimplemented!();
    have = 1_();
    p = if (have != 0) {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut table_1))[(0_()) as usize].name as ())
    } else {
        (c"".as_ptr().cast_mut() as ())
    };
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const table_1);
}
