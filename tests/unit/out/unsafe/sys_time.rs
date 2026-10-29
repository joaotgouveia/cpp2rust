extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_time_0() {
    let mut t1: () = unimplemented!();
    let mut t2: () = 0_();
    let mut t3: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn print_tm_1(mut t: ()) {
    let mut tm: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_gmtime_r_2() {
    (unsafe { print_tm_1(0_()) });
    (unsafe { print_tm_1(1_()) });
    (unsafe { print_tm_1(86399_()) });
    (unsafe { print_tm_1(86400_()) });
    (unsafe { print_tm_1(951782400_()) });
    (unsafe { print_tm_1(951868799_()) });
    (unsafe { print_tm_1(1704067199_()) });
    (unsafe { print_tm_1(1704067200_()) });
    (unsafe { print_tm_1(1721126096_()) });
    (unsafe { print_tm_1(4102444800_()) });
}
pub unsafe fn print_local_tm_3(mut t: ()) {
    let mut tm: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_localtime_r_4() {
    (unsafe { print_local_tm_3(0_()) });
    (unsafe { print_local_tm_3(951782400_()) });
    (unsafe { print_local_tm_3(1704067199_()) });
    (unsafe { print_local_tm_3(1721126096_()) });
    (unsafe { print_local_tm_3(1735689600_()) });
}
pub unsafe fn test_strftime_5() {
    let mut t: () = 1721126096_();
    let mut tm: () = unimplemented!();
    unimplemented!();
    let mut buf: [(); 64] = [unimplemented!(); 64];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut small: [(); 4] = [unimplemented!(); 4];
    unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test_time_0() });
    (unsafe { test_gmtime_r_2() });
    (unsafe { test_localtime_r_4() });
    (unsafe { test_strftime_5() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
