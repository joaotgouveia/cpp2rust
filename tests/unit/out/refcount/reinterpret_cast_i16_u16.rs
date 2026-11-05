extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let vals: Value<Box<[i16]>> = Rc::new(RefCell::new(Box::new([
        (-1_i32 as i16),
        256_i16,
        (-32768_i32 as i16),
    ])));
    let mut uvals: Ptr<u16> = (vals.as_pointer() as Ptr<i16>).reinterpret_cast::<u16>();
    ({ cpp2rust_assert_fail_0((((elem!(uvals, 0).read()) as i32) == 65535)) });
    ({ cpp2rust_assert_fail_0((((elem!(uvals, 1).read()) as i32) == 256)) });
    ({ cpp2rust_assert_fail_0((((elem!(uvals, 2).read()) as i32) == 32768)) });
    elem!(uvals, 0).write(42_u16);
    ({ cpp2rust_assert_fail_0((((*vals.borrow())[(0) as usize] as i32) == 42)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
