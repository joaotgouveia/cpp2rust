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
pub struct Point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl Point {
    pub fn new(mut x: i32, mut y: i32) -> Self {
        Self { x: x, y: y }
    }
}
impl CppMove for Point {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<std_any> = Rc::new(RefCell::new(
        ({
            let ___args_0: Value<i32> = Rc::new(RefCell::new(3));
            let ___args_1: Value<i32> = Rc::new(RefCell::new(4));
            make_any_0(___args_0.as_pointer(), ___args_1.as_pointer())
        }),
    ));
    ({ cpp2rust_assert_fail_1(({ (*a.borrow()).has_value() })) });
    let mut p: Ptr<Point> = ({ any_cast_2((a.as_pointer())) });
    ({ cpp2rust_assert_fail_1(!((p).is_null())) });
    ({ cpp2rust_assert_fail_1((p.with(|__s| __s.x) == 3)) });
    ({ cpp2rust_assert_fail_1((p.with(|__s| __s.y) == 4)) });
    ({ cpp2rust_assert_fail_1(({ any_cast_3((a.as_pointer())) }).is_null()) });
    let q: Ptr<Point> = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(5));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(6));
        (*a.borrow()).emplace(___args_0.as_pointer(), ___args_1.as_pointer())
    });
    ({ cpp2rust_assert_fail_1((q.with(|__s| __s.x) == 5)) });
    ({ cpp2rust_assert_fail_1((({ any_cast_2((a.as_pointer())) }).with(|__s| __s.y) == 6)) });
    let b: Value<std_any> = Rc::new(RefCell::new(
        ({
            let ___args: Value<i32> = Rc::new(RefCell::new(7));
            make_any_4(___args.as_pointer())
        }),
    ));
    ({ cpp2rust_assert_fail_1(((({ any_cast_3((b.as_pointer())) }).read()) == 7)) });
    ({ (*b.borrow()).reset() });
    ({ cpp2rust_assert_fail_1(!({ (*b.borrow()).has_value() })) });
    ({ cpp2rust_assert_fail_1(({ any_cast_3((b.as_pointer())) }).is_null()) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
