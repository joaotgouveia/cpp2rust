extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn my_foo_0(mut p: AnyPtr) -> i32 {
    return (p.reinterpret_cast::<i32>().read());
}
pub fn foo_1(mut fn_: FnPtr<fn(AnyPtr) -> i32>, mut pi: Ptr<i32>) -> i32 {
    return ({ fn_.call((pi as Ptr<i32>).to_any()) });
}
pub fn twice_2(mut x: u64) -> u64 {
    return (x).wrapping_mul(2_u64);
}
pub fn twice_in_place_3(x: Ptr<u64>) -> u64 {
    x.write({ (x.read()).wrapping_mul(2_u64) });
    return (x.read());
}
pub fn ret_size_4(mut v: i32) -> u64 {
    return ((v + 1) as u64);
}
pub fn call_fn_5(mut f: FnPtr<fn(i32) -> u64>, mut v: i32) -> u64 {
    return ({ f.call(v) }).wrapping_mul(2_u64);
}
pub fn identity_hash_6(v: bool) -> u64 {
    let v: Value<bool> = Rc::new(RefCell::new(v));
    return ((*v.borrow()) as u64);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(FnPtr<fn(bool  ,) -> u64  >  , new, ptr)]
#[byte_size(8)]
pub struct HashHolder_unsigned_long__ptr__bool__ {
    #[offset(0)]
    #[byte_size(8)]
    pub h: FnPtr<fn(bool) -> u64>,
}
impl HashHolder_unsigned_long__ptr__bool__ {
    pub fn new(h: Ptr<FnPtr<fn(bool) -> u64>>) -> Self {
        Self { h: (h.read()) }
    }
}
impl CppMove for HashHolder_unsigned_long__ptr__bool__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static total_7: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn add_to_total_8(mut x: i32) {
    total_7.with(|rc| *rc.borrow_mut() += x);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut fn_: FnPtr<fn(AnyPtr) -> i32> = FnPtr::<fn(AnyPtr) -> i32>::null();
    ({ cpp2rust_assert_fail_9((fn_).is_null()) });
    ({
        cpp2rust_assert_fail_9(({ (fn_).clone() } != { FnPtr::<fn(AnyPtr) -> i32>::new(my_foo_0) }))
    });
    fn_ = FnPtr::<fn(AnyPtr) -> i32>::new(my_foo_0);
    ({ cpp2rust_assert_fail_9(!((fn_).is_null())) });
    ({
        cpp2rust_assert_fail_9(({ (fn_).clone() } == { FnPtr::<fn(AnyPtr) -> i32>::new(my_foo_0) }))
    });
    let a: Value<i32> = Rc::new(RefCell::new(10));
    ({
        cpp2rust_assert_fail_9(
            ({ ({ foo_1((fn_).clone(), (a.as_pointer())) }) } == { (*a.borrow()) }),
        )
    });
    let mut ul_fn: FnPtr<fn(u64) -> u64> = (FnPtr::<fn(u64) -> u64>::new(twice_2));
    let mut n: u64 = 21_u64;
    let mut r: u64 = ({ ul_fn.call(n) });
    ({ cpp2rust_assert_fail_9((r == 42_u64)) });
    let mut ul_ref_fn: FnPtr<fn(Ptr<u64>) -> u64> =
        (FnPtr::<fn(Ptr<u64>) -> u64>::new(twice_in_place_3));
    let m: Value<u64> = Rc::new(RefCell::new(21_u64));
    let mut q: u64 = ({ ul_ref_fn.call(m.as_pointer()) });
    ({ cpp2rust_assert_fail_9((q == 42_u64)) });
    ({ cpp2rust_assert_fail_9(((*m.borrow()) == 42_u64)) });
    ({
        cpp2rust_assert_fail_9(
            (({ call_fn_5(FnPtr::<fn(i32) -> u64>::new(ret_size_4), 3) }) == 8_u64),
        )
    });
    let mut hh: HashHolder_unsigned_long__ptr__bool__ = {
        let __tmp_0: Value<FnPtr<fn(bool) -> u64>> =
            Rc::new(RefCell::new(FnPtr::<fn(bool) -> u64>::new(identity_hash_6)));
        HashHolder_unsigned_long__ptr__bool__::new({ __tmp_0.as_pointer() })
    };
    ({ cpp2rust_assert_fail_9((({ hh.h.call(true) }) == 1_u64)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = total_7.with(|_| ());
}
