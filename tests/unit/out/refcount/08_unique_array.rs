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
    let g: Value<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> =
        Rc::new(RefCell::new(({ make_unique_0(2_u64) })));
    (*g.borrow_mut())[0_u64] = 11;
    (*g.borrow_mut())[1_u64] = 12;
    let mut g_ptr: Ptr<i32> = ({ (*g.borrow()).get() });
    elem!(g_ptr, 0).write(13);
    elem!(g_ptr, 1).write(14);
    ({ cpp2rust_assert_fail_1((({ (*g.borrow())[0_u64] } + { (*g.borrow())[1_u64] }) == 27)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
