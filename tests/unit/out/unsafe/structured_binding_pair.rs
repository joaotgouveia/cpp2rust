extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn make_std_pair_0(mut x: ()) -> std_pair_int__bool_ {
    return {
        let mut __tmp_0: () = ((x) > (0_()));
        std_pair_int__bool_::new({ &mut x }, { &mut __tmp_0 })
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut __decomp_1: std_pair_int__bool_ = (unsafe { make_std_pair_0(5_()) });
    let value: *mut () = (unsafe { get_2(&mut __decomp_1) });
    let positive: *mut () = (unsafe { get_3(&mut __decomp_1) });
    unimplemented!();
    unimplemented!();
    let mut p: std_pair_int__int_ = {
        let mut __tmp_1: () = 1_();
        let mut __tmp_2: () = 2_();
        std_pair_int__int_::new({ &mut __tmp_1 }, { &mut __tmp_2 })
    };
    let mut __decomp_4: std_pair_int__int_ = p;
    let a: *mut () = (unsafe { get_5(&mut __decomp_4) });
    let b: *mut () = (unsafe { get_6(&mut __decomp_4) });
    (*a) = 10_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let __decomp_7: *mut std_pair_int__int_ = &mut p;
    let x: *mut () = (unsafe { get_8(__decomp_7) });
    let y: *mut () = (unsafe { get_9(__decomp_7) });
    (*x) = 3_();
    (*y) += 4_();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
