extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(32)]
pub struct S {
    #[offset(0)]
    pub tag: i32,
    #[offset(8)]
    #[byte_size(24)]
    pub v: std_vector_int__std_allocator_int__,
}
impl S {
    pub fn move_from(_a0: Ptr<S>) -> Self {
        Self {
            tag: { (*_a0.upgrade().deref()).tag },
            v: field!(_a0, v).with_mut(std::mem::take),
        }
    }
}
impl CppMove for S {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn add_0(mut v: Ptr<std_vector_int__std_allocator_int__>, k: i32) {
    let k: Value<i32> = Rc::new(RefCell::new(k));
    ({ (*v.upgrade().deref()).push_back_59(k.as_pointer()) });
}
pub fn run_1(mut h: Ptr<S>) {
    ({
        let _v: Ptr<std_vector_int__std_allocator_int__> = (field_ptr!(h, v));
        let _k: i32 = h.with(|__s| __s.tag);
        add_0(_v, _k)
    });
    let mut pv: Ptr<std_vector_int__std_allocator_int__> = (field_ptr!(h, v));
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new((({ (*pv.upgrade().deref()).size() }) as i32)));
        (*pv.upgrade().deref()).push_back_60(___x.as_pointer())
    });
    ({
        cpp2rust_assert_fail_2(
            ((({ (*h.upgrade().deref()).v.size() }) == 2_u64)
                && ((elem!(field_ptr!(h, v), 0_u64).read()) == 7))
                && ((elem!(field_ptr!(h, v), 1_u64).read()) == 1),
        )
    });
    ({ cpp2rust_assert_fail_2((h.with(|__s| __s.tag) == 7)) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let local: Value<S> = Rc::new(RefCell::new(<S>::default()));
    (*local.borrow_mut()).tag = 7;
    ({ run_1((local.as_pointer())) });
    let mut heap: Ptr<S> = Ptr::alloc(<S>::default());
    field!(heap, tag).write(7);
    ({ run_1((heap).clone()) });
    heap.delete();
    return 0;
}
pub trait SImpl {
    fn move_assign(&self, _a0: Ptr<S>) -> Ptr<S>;
}
impl SImpl for Ptr<S> {
    fn move_assign(&self, _a0: Ptr<S>) -> Ptr<S> {
        field!((*self), tag).write({ { (*_a0.upgrade().deref()).tag } });
        field!((*self), v).write({ { (*_a0.upgrade().deref()).v.clone() } });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
