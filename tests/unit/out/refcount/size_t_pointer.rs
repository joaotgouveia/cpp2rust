extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn out_param_0(out: Option<Ptr<u64>>) {
    let mut out: Ptr<u64> = out.unwrap_or_else(|| Ptr::<u64>::null());
    if !(out).is_null() {
        out.write(4_u64);
    }
}
pub fn parse_1(v: i32, idx: Option<Ptr<u64>>) -> i32 {
    let v: Value<i32> = Rc::new(RefCell::new(v));
    let mut idx: Ptr<u64> = idx.unwrap_or_else(|| Ptr::<u64>::null());
    if !(idx).is_null() {
        idx.write(3_u64);
    }
    return (*v.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v2: Value<u64> = Rc::new(RefCell::new(0_u64));
    ({ out_param_0(Some((v2.as_pointer()))) });
    ({ cpp2rust_assert_fail_2(((*v2.borrow()) == 4_u64)) });
    let pidx: Value<u64> = Rc::new(RefCell::new(0_u64));
    ({ cpp2rust_assert_fail_2((({ parse_1(1, Some((pidx.as_pointer()))) }) == 1)) });
    ({ cpp2rust_assert_fail_2(((*pidx.borrow()) == 3_u64)) });
    let sv: Value<u64> = Rc::new(RefCell::new(7_u64));
    let sp: Value<Ptr<u64>> = Rc::new(RefCell::new((sv.as_pointer())));
    let mut spp: Ptr<Ptr<u64>> = (sp.as_pointer());
    ({ cpp2rust_assert_fail_2((((spp.read()).read()) == 7_u64)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
