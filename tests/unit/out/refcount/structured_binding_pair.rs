extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn make_std_pair_0(x: i32) -> std_pair_int__bool_ {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return {
        let __tmp_0: Value<bool> = Rc::new(RefCell::new(((*x.borrow()) > 0)));
        std_pair_int__bool_::new_1({ x.as_pointer() }, { __tmp_0.as_pointer() })
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let __decomp_1: Value<std_pair_int__bool_> = Rc::new(RefCell::new(({ make_std_pair_0(5) })));
    let value: Ptr<i32> = ({ get_2(__decomp_1.as_pointer()) });
    let positive: Ptr<bool> = ({ get_3(__decomp_1.as_pointer()) });
    ({ cpp2rust_assert_fail_4(((value.read()) == 5)) });
    ({ cpp2rust_assert_fail_4(((positive.read()) as bool)) });
    let p: Value<std_pair_int__int_> = Rc::new(RefCell::new({
        let __tmp_1: Value<i32> = Rc::new(RefCell::new(1));
        let __tmp_2: Value<i32> = Rc::new(RefCell::new(2));
        std_pair_int__int_::new_1({ __tmp_1.as_pointer() }, { __tmp_2.as_pointer() })
    }));
    let __decomp_5: Value<std_pair_int__int_> =
        Rc::new(RefCell::new(std_pair_int__int_::new_2({ p.as_pointer() })));
    let a: Ptr<i32> = ({ get_6(__decomp_5.as_pointer()) });
    let b: Ptr<i32> = ({ get_7(__decomp_5.as_pointer()) });
    a.write(10);
    ({ cpp2rust_assert_fail_4(((a.read()) == 10)) });
    ({ cpp2rust_assert_fail_4(((b.read()) == 2)) });
    ({ cpp2rust_assert_fail_4(({ (*p.borrow()).first } == 1)) });
    let __decomp_8: Ptr<std_pair_int__int_> = p.as_pointer();
    let x: Ptr<i32> = ({ get_9((__decomp_8).clone()) });
    let y: Ptr<i32> = ({ get_10((__decomp_8).clone()) });
    x.write(3);
    {
        y.with_mut(|__v| *__v = *__v + 4)
    };
    ({ cpp2rust_assert_fail_4(({ (*p.borrow()).first } == 3)) });
    ({ cpp2rust_assert_fail_4(({ (*p.borrow()).second } == 6)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
