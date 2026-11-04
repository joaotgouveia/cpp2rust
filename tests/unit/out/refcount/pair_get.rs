extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn second_of_0(p: Ptr<std_pair_double__int_>) -> i32 {
    return (({ get_1((p).clone()) }).read());
}
pub fn first_of_2(p: Ptr<std_pair_double__int_>) -> f64 {
    return (({ get_3((p).clone()) }).read());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p: Value<std_pair_double__int_> = Rc::new(RefCell::new({
        let __tmp_0: Value<f64> = Rc::new(RefCell::new(1.5_f64));
        let __tmp_1: Value<i32> = Rc::new(RefCell::new(2));
        std_pair_double__int_::new_1({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
    }));
    ({ cpp2rust_assert_fail_4(((({ get_5(p.as_pointer()) }).read()) == 1.5_f64)) });
    ({ cpp2rust_assert_fail_4(((({ get_6(p.as_pointer()) }).read()) == 2)) });
    ({ cpp2rust_assert_fail_4((({ first_of_2(p.as_pointer()) }) == 1.5_f64)) });
    ({ cpp2rust_assert_fail_4((({ second_of_0(p.as_pointer()) }) == 2)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
