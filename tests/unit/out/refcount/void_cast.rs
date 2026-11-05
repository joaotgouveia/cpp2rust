extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn unused_param_0(mut x: i32) {
    &(x);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct NonTrivial {
    #[offset(0)]
    #[byte_size(24)]
    pub data: std_vector_int__std_allocator_int__,
}
impl NonTrivial {
    pub fn move_from(_a0: Ptr<NonTrivial>) -> Self {
        Self {
            data: field!(_a0, data).with_mut(std::mem::take),
        }
    }
}
impl CppMove for NonTrivial {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn unused_ref_param_1(x: Ptr<NonTrivial>) {
    &(*x.upgrade().deref());
}
pub fn unused_ptr_param_2(mut p: Ptr<NonTrivial>) {
    &(*p.upgrade().deref());
}
thread_local!(
    pub static side_effect_counter_3: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn bump_and_return_4() -> i32 {
    (*side_effect_counter_3.with(Value::clone).borrow_mut()).prefix_inc();
    return side_effect_counter_3.with(|rc| *rc.borrow());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Holder {
    #[offset(0)]
    pub field: i32,
}
impl CppMove for Holder {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(8)]
pub struct NonCopyable {
    #[offset(0)]
    #[byte_size(8)]
    pub value: std_unique_ptr_int__std_default_delete_int__,
}
impl NonCopyable {
    pub fn move_from(_a0: Ptr<NonCopyable>) -> Self {
        Self {
            value: field!(_a0, value).with_mut(std::mem::take),
        }
    }
}
impl CppMove for NonCopyable {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn unused_noncopyable_param_5(x: Ptr<NonCopyable>) {
    &(*x.upgrade().deref());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ unused_param_0(42) });
    let mut y: i32 = 5;
    &(y);
    let mut z: i32 = {
        &(y);
        7
    };
    ({ cpp2rust_assert_fail_6((z == 7)) });
    let counter: Value<i32> = Rc::new(RefCell::new(0));
    let mut w: i32 = {
        {
            &(*counter.borrow_mut());
            (*counter.borrow_mut()) = 3
        };
        (*counter.borrow())
    };
    ({ cpp2rust_assert_fail_6((w == 3)) });
    ({ cpp2rust_assert_fail_6(((*counter.borrow()) == 3)) });
    &({ bump_and_return_4() });
    ({ cpp2rust_assert_fail_6((side_effect_counter_3.with(|rc| *rc.borrow()) == 1)) });
    let mut v: i32 = {
        &({ bump_and_return_4() });
        99
    };
    ({ cpp2rust_assert_fail_6((side_effect_counter_3.with(|rc| *rc.borrow()) == 2)) });
    ({ cpp2rust_assert_fail_6((v == 99)) });
    &(0);
    &(0);
    &(y);
    (&(0));
    (&(y));
    let mut err: i32 = 0;
    (&(err = 42));
    ({ cpp2rust_assert_fail_6((err == 42)) });
    let mut chosen: i32 = {
        &(err = 7);
        123
    };
    ({ cpp2rust_assert_fail_6((err == 7)) });
    ({ cpp2rust_assert_fail_6((chosen == 123)) });
    &(bump_and_return_4);
    ({ cpp2rust_assert_fail_6((side_effect_counter_3.with(|rc| *rc.borrow()) == 2)) });
    &(FnPtr::<fn() -> i32>::new(bump_and_return_4));
    ({ cpp2rust_assert_fail_6((side_effect_counter_3.with(|rc| *rc.borrow()) == 2)) });
    &((FnPtr::<fn() -> i32>::new(bump_and_return_4)).cast::<fn() -> i32>());
    ({ cpp2rust_assert_fail_6((side_effect_counter_3.with(|rc| *rc.borrow()) == 2)) });
    let storage: Value<i32> = Rc::new(RefCell::new(11));
    let mut p: Ptr<i32> = (storage.as_pointer());
    &(p.read());
    &(p);
    let mut arr: [i32; 3] = [1, 2, 3];
    &(arr[(1) as usize]);
    let h: Value<Holder> = Rc::new(RefCell::new(Holder { field: 17 }));
    &((*h.borrow()).field);
    let mut hp: Ptr<Holder> = (h.as_pointer());
    &((*hp.upgrade().deref()).field);
    let nt: Value<NonTrivial> = Rc::new(RefCell::new(<NonTrivial>::default()));
    ({ unused_ref_param_1(nt.as_pointer()) });
    ({ unused_ptr_param_2((nt.as_pointer())) });
    let g: Value<NonCopyable> = Rc::new(RefCell::new(NonCopyable {
        value: ({
            let ___args: Value<i32> = Rc::new(RefCell::new(9));
            make_unique_7(___args.as_pointer())
        }),
    }));
    (&(*g.borrow_mut()));
    &(*g.borrow_mut());
    ({ unused_noncopyable_param_5(g.as_pointer()) });
    ({ cpp2rust_assert_fail_6((({ (*g.borrow()).value.clone() }.read()) == 9)) });
    return 0;
}
pub trait NonCopyableImpl {
    fn move_assign(&self, _a0: Ptr<NonCopyable>) -> Ptr<NonCopyable>;
}
impl NonCopyableImpl for Ptr<NonCopyable> {
    fn move_assign(&self, _a0: Ptr<NonCopyable>) -> Ptr<NonCopyable> {
        field!((*self), value).write({ { (*_a0.upgrade().deref()).value.clone() } });
        return (*self).clone();
    }
}
pub trait NonTrivialImpl {
    fn move_assign(&self, _a0: Ptr<NonTrivial>) -> Ptr<NonTrivial>;
}
impl NonTrivialImpl for Ptr<NonTrivial> {
    fn move_assign(&self, _a0: Ptr<NonTrivial>) -> Ptr<NonTrivial> {
        field!((*self), data).write({ { (*_a0.upgrade().deref()).data.clone() } });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = side_effect_counter_3.with(|_| ());
}
