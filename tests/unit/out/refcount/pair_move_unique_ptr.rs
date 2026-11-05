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
    let p: Value<std_pair_std_unique_ptr_int__std_default_delete_int____int_> =
        Rc::new(RefCell::new({
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(1));
            std_pair_std_unique_ptr_int__std_default_delete_int____int_::new_1(
                { u.as_pointer() },
                { __tmp_0.as_pointer() },
            )
        }));
    ({ cpp2rust_assert_fail_0(({ (*u.borrow()).get() }).is_null()) });
    ({ cpp2rust_assert_fail_0((({ (*p.borrow()).first.clone() }.read()) == 3)) });
    let source: Value<std_pair_int__std_unique_ptr_int__std_default_delete_int___> =
        Rc::new(RefCell::new({
            let __tmp_1: Value<i32> = Rc::new(RefCell::new(0));
            let __tmp_2: Value<std_unique_ptr_int__std_default_delete_int__> =
                Rc::new(RefCell::new(
                    std_unique_ptr_int__std_default_delete_int__::new_1({ Ptr::alloc(81) }),
                ));
            std_pair_int__std_unique_ptr_int__std_default_delete_int___::new_1(
                { __tmp_1.as_pointer() },
                { __tmp_2.as_pointer() },
            )
        }));
    let mut original: Ptr<i32> = ({ { (*source.borrow()).second.clone() }.get() });
    let converted: Value<std_pair_long__std_unique_ptr_int__std_default_delete_int___> =
        Rc::new(RefCell::new((*source.borrow_mut()).clone()));
    ({ cpp2rust_assert_fail_0(({ (*converted.borrow()).first } == 0_i64)) });
    ({
        cpp2rust_assert_fail_0(
            ({ ({ { (*converted.borrow()).second.clone() }.get() }) } == { (original).clone() }),
        )
    });
    ({ cpp2rust_assert_fail_0((({ (*converted.borrow()).second.clone() }.read()) == 81)) });
    ({ cpp2rust_assert_fail_0(({ { (*source.borrow()).second.clone() }.get() }).is_null()) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
