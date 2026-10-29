extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sum_0(mut args_0: (), mut args_1: (), mut args_2: ()) -> () {
    return ((args_0) + ((args_1) + ((args_2) + (0_()))));
}
pub unsafe fn sum_1(mut args_0: (), mut args_1: ()) -> () {
    return ((args_0) + ((args_1) + (0_())));
}
pub unsafe fn sum_2(mut args: ()) -> () {
    return ((args) + (0_()));
}
pub unsafe fn first_3(mut x: (), mut args_1: (), mut args_2: ()) -> () {
    return ((x) + (unsafe { sum_1(args_1, args_2) }));
}
pub unsafe fn first_4(mut x: (), mut args: ()) -> () {
    return ((x) + (unsafe { sum_2(args) }));
}
pub unsafe fn sizeof_pack_5() -> () {
    return 0;
}
pub unsafe fn sizeof_pack_6(mut args: ()) -> () {
    return 1;
}
pub unsafe fn sizeof_pack_7(mut args_0: (), mut args_1: ()) -> () {
    return 2;
}
pub unsafe fn sizeof_pack_8(mut args_0: (), mut args_1: (), mut args_2: ()) -> () {
    return 3;
}
pub unsafe fn sizeof_pack_9(mut args_0: (), mut args_1: (), mut args_2: (), mut args_3: ()) -> () {
    return 4;
}
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
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
