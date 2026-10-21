extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_expect_0() {
    let mut x: () = 42_();
    unimplemented!();
}
pub fn test_ctz_1() {
    unimplemented!();
    unimplemented!();
}
pub fn test_clz_2() {
    unimplemented!();
    unimplemented!();
}
pub fn test_bswap16_3() {
    unimplemented!();
}
pub fn test_bswap32_4() {
    unimplemented!();
}
pub fn test_bswap64_5() {
    unimplemented!();
}
pub fn test_ctzl_6() {
    unimplemented!();
    unimplemented!();
}
pub fn test_popcountl_7() {
    unimplemented!();
    unimplemented!();
}
pub fn test_clzl_8() {
    unimplemented!();
    unimplemented!();
}
pub fn test_inff_9() {
    let mut inf: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_nanf_10() {
    let mut nan: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_mul_overflow_long_11() {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_mul_overflow_long_long_12() {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test_expect_0() });
    ({ test_ctz_1() });
    ({ test_clz_2() });
    ({ test_bswap16_3() });
    ({ test_bswap32_4() });
    ({ test_bswap64_5() });
    ({ test_ctzl_6() });
    ({ test_popcountl_7() });
    ({ test_clzl_8() });
    ({ test_inff_9() });
    ({ test_nanf_10() });
    ({ test_mul_overflow_long_11() });
    ({ test_mul_overflow_long_long_12() });
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
