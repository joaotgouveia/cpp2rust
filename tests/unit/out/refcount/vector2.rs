extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fn_0(v: Ptr<std_vector_int__std_allocator_int__>, v3: std_vector_int__std_allocator_int__) {
    let v3: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(v3));
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(20));
        (*v.upgrade().deref()).push_back_60(___x.as_pointer())
    });
    let mut x: i32 = 0_i32;
    let v2: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
        <std_vector_int__std_allocator_int__>::default(),
    ));
    let mut v4: Ptr<std_vector_int__std_allocator_int__> = (v3.as_pointer());
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(0));
        (*v2.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(1));
        (*v2.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(3));
        (*v2.borrow()).push_back_60(___x.as_pointer())
    });
    x = (elem!(v, 2_u64).read());
    (*v2.borrow_mut())[0_u64] = 1;
    elem!((if true { (v3.as_pointer()) } else { (v).clone() }), 0_u64).write(7);
    (*v2.borrow_mut()) = { (*v.upgrade().deref()).clone() };
    elem!((v4), 1_u64).write(13);
    ({ cpp2rust_assert_fail_1((x == 6)) });
    ({ cpp2rust_assert_fail_1(((({ (*v.upgrade().deref()).front_53() }).read()) == 4)) });
    ({ cpp2rust_assert_fail_1(((elem!(v, 1_u64).read()) == 5)) });
    ({ cpp2rust_assert_fail_1(((elem!(v, 2_u64).read()) == 6)) });
    ({ cpp2rust_assert_fail_1(((({ (*v.upgrade().deref()).back_55() }).read()) == 20)) });
    ({ cpp2rust_assert_fail_1(({ (*v2.borrow())[0_u64] } == 4)) });
    ({ cpp2rust_assert_fail_1(({ (*v2.borrow())[1_u64] } == 5)) });
    ({ cpp2rust_assert_fail_1(({ (*v2.borrow())[2_u64] } == 6)) });
    ({ cpp2rust_assert_fail_1(({ (*v3.borrow())[0_u64] } == 7)) });
    ({ cpp2rust_assert_fail_1(({ (*v3.borrow())[1_u64] } == 13)) });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(20));
        (*v.upgrade().deref()).push_back_60(___x.as_pointer())
    });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
        <std_vector_int__std_allocator_int__>::default(),
    ));
    let v2: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
        <std_vector_int__std_allocator_int__>::default(),
    ));
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(4));
        (*v.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(5));
        (*v.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(6));
        (*v.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(8));
        (*v2.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(9));
        (*v2.borrow()).push_back_60(___x.as_pointer())
    });
    ({ fn_0(v.as_pointer(), (*v2.borrow()).clone()) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
