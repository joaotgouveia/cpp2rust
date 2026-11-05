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
    let d: Value<std_deque_int__std_allocator_int__> =
        Rc::new(RefCell::new(<std_deque_int__std_allocator_int__>::default()));
    let x: Ptr<i32> = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(1));
        (*d.borrow()).emplace_back(___args.as_pointer())
    });
    ({ cpp2rust_assert_fail_0(((x.read()) == 1)) });
    ({
        let ___args: Value<i32> = Rc::new(RefCell::new(2));
        (*d.borrow()).emplace_back(___args.as_pointer())
    });
    let y: Ptr<i32> = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(3));
        (*d.borrow()).emplace_back(___args.as_pointer())
    });
    y.write(30);
    ({ cpp2rust_assert_fail_0(((({ (*d.borrow()).back_49() }).read()) == 30)) });
    ({ cpp2rust_assert_fail_0(((({ (*d.borrow()).front_47() }).read()) == 1)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
