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
    let u: Value<std_unique_ptr_int__std_default_delete_int__> = Rc::new(RefCell::new(
        std_unique_ptr_int__std_default_delete_int__::new_1({ Ptr::alloc(3) }),
    ));
    ({ cpp2rust_assert_fail_0(({ (*u.borrow())() })) });
    let e: Value<std_unique_ptr_int__std_default_delete_int__> = Rc::new(RefCell::new(
        <std_unique_ptr_int__std_default_delete_int__>::default(),
    ));
    ({ cpp2rust_assert_fail_0(!({ (*e.borrow())() })) });
    if ({ (*u.borrow())() }) {
        (*u.borrow_mut()).write(4);
    }
    ({ cpp2rust_assert_fail_0((((*u.borrow()).read()) == 4)) });
    let a: Value<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> =
        Rc::new(RefCell::new(({ make_unique_1(2_u64) })));
    ({ cpp2rust_assert_fail_0(({ (*a.borrow())() })) });
    let ea: Value<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> = Rc::new(
        RefCell::new(<std_unique_ptr_intarrarr__std_default_delete_intarrarr__>::default()),
    );
    ({ cpp2rust_assert_fail_0(!({ (*ea.borrow())() })) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
