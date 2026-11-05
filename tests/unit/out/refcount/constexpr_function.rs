extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn runtime_only_0(mut x: i32) -> i32 {
    return (x * 2);
}
pub fn first_1(mut p: Ptr<i32>) -> i32 {
    return (p.read());
}
pub fn scaled_2(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    if ((*x.borrow()) < 0) {
        return ({ runtime_only_0(-(*x.borrow())) });
    }
    return (*x.borrow());
}
pub fn half_3(mut x: f64) -> f64 {
    return (x / 2_f64);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Flag {
    #[offset(0)]
    pub v: i32,
}
impl CppMove for Flag {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn use_4(f: Flag) -> i32 {
    let f: Value<Flag> = Rc::new(RefCell::new(f));
    ({ cpp2rust_assert_fail_5(({ FlagImpl::to_bool(&f.as_pointer()) })) });
    return { (*f.borrow()).v };
}
pub fn checked_6(mut x: i32) -> i32 {
    ({ cpp2rust_assert_fail_5((x > 0)) });
    return (x + 1);
}
pub fn in_constant_context_7() -> bool {
    {}
    return (({ runtime_only_0(1) }) == 0);
}
const _: () = assert!(true, "in_constant_context()");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct P {
    #[offset(0)]
    pub v: i32,
}
impl CppMove for P {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut runtime_context: bool = ({ in_constant_context_7() });
    ({ cpp2rust_assert_fail_5(!(runtime_context)) });
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([7, 8])));
    ({ cpp2rust_assert_fail_5((({ first_1((arr.as_pointer() as Ptr<i32>)) }) == 7)) });
    ({
        cpp2rust_assert_fail_5(
            (({ first_1((arr.as_pointer() as Ptr<i32>).offset((1) as isize)) }) == 8),
        )
    });
    ({ cpp2rust_assert_fail_5((({ scaled_2(3) }) == 3)) });
    ({ cpp2rust_assert_fail_5((({ scaled_2(-3_i32) }) == 6)) });
    ({ cpp2rust_assert_fail_5((({ half_3(5_f64) }) == 2.5_f64)) });
    ({ cpp2rust_assert_fail_5((({ checked_6(1) }) == 2)) });
    let mut c: i32 = 5;
    ({ cpp2rust_assert_fail_5((c == 5)) });
    ({ cpp2rust_assert_fail_5((({ use_4(Flag { v: 2 }) }) == 2)) });
    let mut u: i32 = 3;
    ({ cpp2rust_assert_fail_5((u == 3)) });
    let mut ptr: Ptr<i32> = (arr.as_pointer() as Ptr<i32>);
    ({ cpp2rust_assert_fail_5(!(ptr).is_null()) });
    let p: Value<P> = Rc::new(RefCell::new(P { v: 9 }));
    ({ cpp2rust_assert_fail_5((({ PImpl::get(&p.as_pointer()) }) == 9)) });
    let mut k: i32 = 4;
    ({ cpp2rust_assert_fail_5((k == 4)) });
    return 0;
}
pub trait FlagImpl {
    fn to_bool(&self) -> bool;
}
impl FlagImpl for Ptr<Flag> {
    fn to_bool(&self) -> bool {
        return ((*self).with(|__s| __s.v) != 0);
    }
}
pub trait PImpl {
    fn get(&self) -> i32;
}
impl PImpl for Ptr<P> {
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.v);
    }
}
pub fn __cpp2rust_init_globals() {}
