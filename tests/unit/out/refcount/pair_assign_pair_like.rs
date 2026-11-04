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
    let a: Value<std_pair_int__long_> = Rc::new(RefCell::new({
        let __tmp_0: Value<i32> = Rc::new(RefCell::new(1));
        let __tmp_1: Value<i64> = Rc::new(RefCell::new(2_i64));
        std_pair_int__long_::new_1({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
    }));
    let b: Value<std_pair_short__int_> = Rc::new(RefCell::new({
        let __tmp_2: Value<i32> = Rc::new(RefCell::new(7));
        let __tmp_3: Value<i32> = Rc::new(RefCell::new(8));
        std_pair_short__int_::new_1({ __tmp_2.as_pointer() }, { __tmp_3.as_pointer() })
    }));
    (*a.borrow_mut()) = (*b.borrow()).clone();
    ({
        cpp2rust_assert_fail_0(
            ({ (*a.borrow()).first } == 7) && ({ (*a.borrow()).second } == 8_i64),
        )
    });
    (*b.borrow_mut()).first = 9_i16;
    ({ cpp2rust_assert_fail_0(({ (*a.borrow()).first } == 7)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
