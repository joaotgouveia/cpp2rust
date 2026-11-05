extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn change_0(n: Ptr<std_unique_ptr_int__std_default_delete_int__>) {
    let m: Value<std_unique_ptr_int__std_default_delete_int__> = Rc::new(RefCell::new(
        ({
            let ___args: Value<i32> = Rc::new(RefCell::new(20));
            make_unique_1(___args.as_pointer())
        }),
    ));
    n.write({ (*m.borrow()).clone() });
    ({ cpp2rust_assert_fail_2(({ (*m.borrow()).get() }).is_null()) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let n: Value<std_unique_ptr_int__std_default_delete_int__> = Rc::new(RefCell::new(
        ({
            let ___args: Value<i32> = Rc::new(RefCell::new(10));
            make_unique_1(___args.as_pointer())
        }),
    ));
    ({ change_0(n.as_pointer()) });
    ({ cpp2rust_assert_fail_2((((*n.borrow()).read()) == 20)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
