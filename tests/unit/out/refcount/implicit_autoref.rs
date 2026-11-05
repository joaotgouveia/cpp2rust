extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(24)]
    pub v: std_vector_int__std_allocator_int__,
}
impl Holder {
    pub fn move_from(_a0: Ptr<Holder>) -> Self {
        Self {
            v: field!(_a0, v).with_mut(std::mem::take),
        }
    }
}
impl CppMove for Holder {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn write_through_0(mut p: Ptr<i32>) {
    p.write(42);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
        <std_vector_int__std_allocator_int__>::default(),
    ));
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(10));
        (*v.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(20));
        (*v.borrow()).push_back_60(___x.as_pointer())
    });
    let mut p: Ptr<std_vector_int__std_allocator_int__> = (v.as_pointer());
    let mut a: i32 = (elem!((p), 0_u64).read());
    elem!((p), 1_u64).write(30);
    let h: Value<Holder> = Rc::new(RefCell::new(<Holder>::default()));
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(40));
        (*h.borrow()).v.push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(50));
        (*h.borrow()).v.push_back_60(___x.as_pointer())
    });
    let mut hp: Ptr<Holder> = (h.as_pointer());
    let mut b: i32 = (elem!(field_ptr!((hp), v), 0_u64).read());
    elem!(field_ptr!((hp), v), 1_u64).write(60);
    ({ cpp2rust_assert_fail_1((a == 10)) });
    ({ cpp2rust_assert_fail_1(((elem!((p), 1_u64).read()) == 30)) });
    ({ cpp2rust_assert_fail_1((b == 40)) });
    ({ cpp2rust_assert_fail_1(((elem!(field_ptr!((hp), v), 1_u64).read()) == 60)) });
    ({ write_through_0(({ (*p.upgrade().deref()).at_51(0_u64) })) });
    ({ cpp2rust_assert_fail_1(((elem!((p), 0_u64).read()) == 42)) });
    return 0;
}
pub trait HolderImpl {
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder>;
}
impl HolderImpl for Ptr<Holder> {
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder> {
        field!((*self), v).write({ { (*_a0.upgrade().deref()).v.clone() } });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
