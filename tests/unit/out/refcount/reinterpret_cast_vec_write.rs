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
    let vec_: Value<std_vector_unsigned_int__std_allocator_unsigned_int__> = Rc::new(RefCell::new(
        <std_vector_unsigned_int__std_allocator_unsigned_int__>::default(),
    ));
    ({
        let ___x: Value<u32> = Rc::new(RefCell::new(67305985_u32));
        (*vec_.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<u32> = Rc::new(RefCell::new(134678021_u32));
        (*vec_.borrow()).push_back_60(___x.as_pointer())
    });
    let mut bytes: Ptr<u8> = ({ (*vec_.borrow()).data_57() }).reinterpret_cast::<u8>();
    ({ cpp2rust_assert_fail_0((((elem!(bytes, 0).read()) as i32) == 1)) });
    ({ cpp2rust_assert_fail_0((((elem!(bytes, 1).read()) as i32) == 2)) });
    ({ cpp2rust_assert_fail_0((((elem!(bytes, 2).read()) as i32) == 3)) });
    ({ cpp2rust_assert_fail_0((((elem!(bytes, 3).read()) as i32) == 4)) });
    ({ cpp2rust_assert_fail_0((((elem!(bytes, 4).read()) as i32) == 5)) });
    ({ cpp2rust_assert_fail_0((((elem!(bytes, 7).read()) as i32) == 8)) });
    elem!(bytes, 4).write(255_u8);
    ({ cpp2rust_assert_fail_0(({ (*vec_.borrow())[1_u64] } == 134678271_u32)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
