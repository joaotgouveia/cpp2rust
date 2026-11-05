extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Pair {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl CppMove for Pair {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut x: Ptr<i32> = Ptr::alloc(5);
    let mut out: i32 = (x.read());
    x.delete();
    ({ cpp2rust_assert_fail_0((out == 5)) });
    let mut y: Ptr<i32> = Ptr::alloc(Default::default());
    y.write(9);
    ({ cpp2rust_assert_fail_0(((y.read()) == 9)) });
    y.delete();
    let mut p: Ptr<Pair> = Ptr::alloc(<Pair>::default());
    field!(p, x).write(1);
    field!(p, y).write(2);
    ({ cpp2rust_assert_fail_0((({ p.with(|__s| __s.x) } + { p.with(|__s| __s.y) }) == 3)) });
    p.delete();
    let mut nullpointer: Ptr<i32> = Ptr::<i32>::null();
    nullpointer.delete();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
