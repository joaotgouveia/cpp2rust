extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_expect_0() {
    let mut x: () = 42_();
    unimplemented!();
}
pub unsafe fn test_ctz_1() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_clz_2() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_bswap16_3() {
    unimplemented!();
}
pub unsafe fn test_bswap32_4() {
    unimplemented!();
}
pub unsafe fn test_bswap64_5() {
    unimplemented!();
}
pub unsafe fn test_ctzl_6() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_popcountl_7() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_clzl_8() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_inff_9() {
    let mut inf: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_nanf_10() {
    let mut nan: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_mul_overflow_long_11() {
    let mut r: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_mul_overflow_long_long_12() {
    let mut r: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test_expect_0() });
    (unsafe { test_ctz_1() });
    (unsafe { test_clz_2() });
    (unsafe { test_bswap16_3() });
    (unsafe { test_bswap32_4() });
    (unsafe { test_bswap64_5() });
    (unsafe { test_ctzl_6() });
    (unsafe { test_popcountl_7() });
    (unsafe { test_clzl_8() });
    (unsafe { test_inff_9() });
    (unsafe { test_nanf_10() });
    (unsafe { test_mul_overflow_long_11() });
    (unsafe { test_mul_overflow_long_long_12() });
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
