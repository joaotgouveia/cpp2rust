extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn square_0(mut x: i32) -> i32 {
    return (x * x);
}
pub fn twice_1(mut x: i32) -> i32 {
    return (2 * x);
}
pub fn call_ref_2(f: FnPtr<fn(i32) -> i32>, mut x: i32) -> i32 {
    return ({ f.call(x) });
}
pub fn call_deduced_3(f: FnPtr<fn(i32) -> i32>, mut x: i32) -> i32 {
    return ({ f.call(x) });
}
pub fn call_forwarded_4(f: FnPtr<fn(i32) -> i32>, mut x: i32) -> i32 {
    return ({ f.call(x) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(8)]
    pub f: FnPtr<fn(i32) -> i32>,
}
impl CppMove for Holder {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({
        cpp2rust_assert_fail_5((({ call_ref_2(FnPtr::<fn(i32) -> i32>::new(square_0), 3) }) == 9))
    });
    ({ cpp2rust_assert_fail_5((({ call_ref_2(FnPtr::<fn(i32) -> i32>::new(twice_1), 3) }) == 6)) });
    let r: FnPtr<fn(i32) -> i32> = FnPtr::<fn(i32) -> i32>::new(square_0);
    ({ cpp2rust_assert_fail_5((({ r.call(4) }) == 16)) });
    ({ cpp2rust_assert_fail_5((({ call_ref_2((r).clone(), 5) }) == 25)) });
    ({
        cpp2rust_assert_fail_5(
            (({ call_deduced_3(FnPtr::<fn(i32) -> i32>::new(twice_1), 7) }) == 14),
        )
    });
    ({
        cpp2rust_assert_fail_5(
            (({ call_forwarded_4(FnPtr::<fn(i32) -> i32>::new(square_0), 6) }) == 36),
        )
    });
    let h: Value<Holder> = Rc::new(RefCell::new(Holder {
        f: FnPtr::<fn(i32) -> i32>::new(twice_1),
    }));
    ({ cpp2rust_assert_fail_5((({ HolderImpl::run(&h.as_pointer(), 8) }) == 16)) });
    return 0;
}
pub trait HolderImpl {
    fn run(&self, x: i32) -> i32;
}
impl HolderImpl for Ptr<Holder> {
    fn run(&self, mut x: i32) -> i32 {
        return ({ (*self).with(|__s| __s.f.clone()).call(x) });
    }
}
pub fn __cpp2rust_init_globals() {}
