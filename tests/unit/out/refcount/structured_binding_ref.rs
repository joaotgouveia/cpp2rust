extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct Triple {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: bool,
    #[offset(8)]
    pub c: i32,
}
impl CppMove for Triple {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn sum_0(t: Ptr<Triple>) -> i32 {
    let __decomp_1: Ptr<Triple> = (t).clone();
    return ((__decomp_1.with(|__s| __s.a) + (if __decomp_1.with(|__s| __s.b) { 1 } else { 0 }))
        + __decomp_1.with(|__s| __s.c));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let t: Value<Triple> = Rc::new(RefCell::new(Triple {
        a: 10,
        b: false,
        c: 20,
    }));
    let __decomp_2: Ptr<Triple> = t.as_pointer();
    field!(__decomp_2, a).write(11);
    field!(__decomp_2, b).write(true);
    {
        field!(__decomp_2, c).with_mut(|__v| *__v = *__v + 1)
    };
    ({ cpp2rust_assert_fail_3(({ (*t.borrow()).a } == 11)) });
    ({ cpp2rust_assert_fail_3({ (*t.borrow()).b }) });
    ({ cpp2rust_assert_fail_3(({ (*t.borrow()).c } == 21)) });
    (*t.borrow_mut()).a = 12;
    ({ cpp2rust_assert_fail_3((__decomp_2.with(|__s| __s.a) == 12)) });
    let __decomp_4: Ptr<Triple> = t.as_pointer();
    (*t.borrow_mut()).c = 30;
    ({ cpp2rust_assert_fail_3((__decomp_4.with(|__s| __s.a) == 12)) });
    ({ cpp2rust_assert_fail_3(__decomp_4.with(|__s| __s.b)) });
    ({ cpp2rust_assert_fail_3((__decomp_4.with(|__s| __s.c) == 30)) });
    ({ cpp2rust_assert_fail_3((({ sum_0(t.as_pointer()) }) == 43)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
