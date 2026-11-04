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
    pub first: i32,
    #[offset(4)]
    pub second: i32,
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
    let __decomp_0: Value<Pair> = Rc::new(RefCell::new(Pair {
        first: 1,
        second: 2,
    }));
    ({ cpp2rust_assert_fail_1(({ (*__decomp_0.borrow()).first } == 1)) });
    ({ cpp2rust_assert_fail_1(({ (*__decomp_0.borrow()).second } == 2)) });
    let mut p: Pair = Pair {
        first: 10,
        second: 20,
    };
    let __decomp_2: Value<Pair> = Rc::new(RefCell::new((p).clone()));
    (*__decomp_2.borrow_mut()).first = 11;
    (*__decomp_2.borrow_mut()).second += 1;
    ({ cpp2rust_assert_fail_1(({ (*__decomp_2.borrow()).first } == 11)) });
    ({ cpp2rust_assert_fail_1(({ (*__decomp_2.borrow()).second } == 21)) });
    ({ cpp2rust_assert_fail_1((p.first == 10)) });
    ({ cpp2rust_assert_fail_1((p.second == 20)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
