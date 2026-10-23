extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut b: () = unimplemented!();
    let mut c: () = unimplemented!();
    let mut m: () = (unsafe {
        ( unsafe { let mut ___e : std__Expr_std___detail__BinClos_std___minus__std__ValArray__std__ValArray__double__double___double_  = unimplemented!()  ; abs_0 ( & mut ___e , ) } )  . max  ( )
    });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
