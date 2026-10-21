extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Code = ();
pub const Code_CODE_OK: Code = 0;
pub const Code_CODE_ERR: Code = 1;
pub const Code_CODE_FATAL: Code = 2;
pub static mut side_effect_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub unsafe fn observe_1(mut v: ()) -> () {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut side_effect_0)).prefix_inc();
    return v;
}
pub unsafe fn returns_one_2() -> () {
    return 1_();
}
pub unsafe fn returns_zero_3() -> () {
    return unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut n: () = 3_();
    let mut zero: () = unimplemented!();
    let mut storage: () = 7_();
    let mut p: *mut () = (&mut storage as *mut ());
    let mut np: *mut () = std::ptr::null_mut();
    let mut u: () = 4_();
    let mut code: Code = Code_CODE_OK;
    if ((((n != 0) && (!(p).is_null())) as ()) != 0) {
        unimplemented!();
    }
    if ((((n != 0) && (!(np).is_null())) as ()) != 0) {
        unimplemented!();
    }
    if ((((zero != 0) || (!(p).is_null())) as ()) != 0) {
        unimplemented!();
    }
    if ((((zero != 0) || (!(np).is_null())) as ()) != 0) {
        unimplemented!();
    }
    if ((((((((((n != 0) && (u != 0)) as ()) != 0) && (!(p).is_null())) as ()) != 0)
        && ((((code as ()) == (Code_CODE_OK as i32)) as ()) != 0)) as ())
        != 0)
    {
        unimplemented!();
    }
    (*std::cell::LazyCell::force_mut(&mut *&raw mut side_effect_0)) = unimplemented!();
    if ((((zero != 0) && ((unsafe { observe_1(1_()) }) != 0)) as ()) != 0) {
        unimplemented!();
    }
    unimplemented!();
    if ((((n != 0) || ((unsafe { observe_1(1_()) }) != 0)) as ()) != 0) {
        unimplemented!();
    }
    unimplemented!();
    let mut x: () = 5_();
    let mut y: () = 3_();
    let mut flags: () = 2_();
    if (((((((x) > (y)) as ()) != 0) || (((flags) & (1_())) != 0)) as ()) != 0) {
        unimplemented!();
    }
    if (((((((x) < (y)) as ()) != 0) || (((flags) & (1_())) != 0)) as ()) != 0) {
        unimplemented!();
    }
    let mut a: () = 1_();
    let mut b: () = 2_();
    let mut c: () = 3_();
    if (((((((a) != (c)) as ()) != 0) && ((((b) != (c)) as ()) != 0)) as ()) != 0) {
        unimplemented!();
    }
    let mut s: () = -1_();
    if ((((((!((p).is_null())) as ()) != 0) && ((((s) < (unimplemented!())) as ()) != 0)) as ())
        != 0)
    {
        unimplemented!();
    }
    let mut k: () = 2_();
    let mut done: () = (unimplemented!() != 0);
    if (((((((k) > (1_())) as ()) != 0) || (!(done))) as ()) != 0) {
        unimplemented!();
    }
    if (((((((x) > (y)) as ()) != 0) || (((flags) & (4_())) != 0)) as ()) != 0) {
        unimplemented!();
    }
    let mut ull: () = 7_();
    if ((((((!((p).is_null())) as ()) != 0) && (ull != 0)) as ()) != 0) {
        unimplemented!();
    }
    if (((((((x) > (y)) as ()) != 0) && (ull != 0)) as ()) != 0) {
        unimplemented!();
    }
    let mut mask: () = (((1_()) << (4_())) | ((1_()) << (5_())));
    let mut bits: () = ((1_()) << (4_()));
    if (((((((n) != (unimplemented!())) as ()) != 0) && (((bits) & (mask)) != 0)) as ()) != 0) {
        unimplemented!();
    }
    if (((((((n) != (unimplemented!())) as ()) != 0) || (((bits) & (256_())) != 0)) as ()) != 0) {
        unimplemented!();
    }
    let mut cp: *const () = (c"hi".as_ptr().cast_mut()).cast_const();
    let mut cnp: *const () = std::ptr::null();
    if (((((((x) > (y)) as ()) != 0) && (!(cp).is_null())) as ()) != 0) {
        unimplemented!();
    }
    if (((((((x) < (y)) as ()) != 0) || (!(cnp).is_null())) as ()) != 0) {
        unimplemented!();
    }
    if (((((((x) > (y)) as ()) != 0) && ((((n != 0) && (!(cp).is_null())) as ()) != 0)) as ()) != 0)
    {
        unimplemented!();
    }
    if (((((((x) > (y)) as ()) != 0) && ((unsafe { returns_one_2() }) != 0)) as ()) != 0) {
        unimplemented!();
    }
    if (((((((x) > (y)) as ()) != 0) && (!((unsafe { returns_zero_3() }) != 0))) as ()) != 0) {
        unimplemented!();
    }
    if (((((((x) < (y)) as ()) != 0) || ((unsafe { returns_one_2() }) != 0)) as ()) != 0) {
        unimplemented!();
    }
    if (((((((x) < (y)) as ()) != 0) || (!((unsafe { returns_one_2() }) != 0))) as ()) != 0) {
        unimplemented!();
    }
    if (((((((((!((p).is_null())) as ()) != 0) && ((unsafe { returns_one_2() }) != 0)) as ()) != 0)
        && ((((n) != (unimplemented!())) as ()) != 0)) as ())
        != 0)
    {
        unimplemented!();
    }
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const side_effect_0);
}
