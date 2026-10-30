extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn unused_param_0(mut x: ()) {
    &(x);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct NonTrivial {
    #[offset(0)]
    #[byte_size(24)]
    pub data: (),
}
impl NonTrivial {
    pub fn move_from(_a0: Ptr<NonTrivial>) -> Self {
        Self {
            data: unimplemented!(),
        }
    }
}
pub fn unused_ref_param_1(x: Ptr<NonTrivial>) {
    &(*x.upgrade().deref());
}
pub fn unused_ptr_param_2(mut p: Ptr<NonTrivial>) {
    &(*p.upgrade().deref());
}
thread_local!(
    pub static side_effect_counter_3: Value<()> = Rc::new(RefCell::new(0_()));
);
pub fn bump_and_return_4() -> () {
    (*side_effect_counter_3.with(Value::clone).borrow_mut()).prefix_inc();
    return side_effect_counter_3.with(|rc| *rc.borrow());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Holder {
    #[offset(0)]
    pub field: (),
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(8)]
pub struct NonCopyable {
    #[offset(0)]
    #[byte_size(8)]
    pub value: (),
}
impl NonCopyable {
    pub fn move_from(_a0: Ptr<NonCopyable>) -> Self {
        Self {
            value: unimplemented!(),
        }
    }
}
pub fn unused_noncopyable_param_5(x: Ptr<NonCopyable>) {
    &(*x.upgrade().deref());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ unused_param_0(42_()) });
    let mut y: () = 5_();
    &(y);
    let mut z: () = {
        &(y);
        7_()
    };
    unimplemented!();
    let counter: Value<()> = Rc::new(RefCell::new(0_()));
    let mut w: () = {
        {
            &(*counter.borrow_mut());
            (*counter.borrow_mut()) = 3_()
        };
        (*counter.borrow())
    };
    unimplemented!();
    unimplemented!();
    &({ bump_and_return_4() });
    unimplemented!();
    let mut v: () = {
        &({ bump_and_return_4() });
        99_()
    };
    unimplemented!();
    unimplemented!();
    &(0_());
    &(0_());
    &(y);
    (&(0_()));
    (&(y));
    let mut err: () = 0_();
    (&(err = 42_()));
    unimplemented!();
    let mut chosen: () = {
        &(err = 7_());
        123_()
    };
    unimplemented!();
    unimplemented!();
    &(bump_and_return_4);
    unimplemented!();
    &(FnPtr::<fn() -> ()>::new(bump_and_return_4));
    unimplemented!();
    &((FnPtr::<fn() -> ()>::new(bump_and_return_4)).cast::<fn() -> ()>());
    unimplemented!();
    let storage: Value<()> = Rc::new(RefCell::new(11_()));
    let mut p: Ptr<()> = (storage.as_pointer());
    &(p.read());
    &(p);
    let mut arr: [(); 3] = [1_(), 2_(), 3_()];
    &(arr[(1_()) as usize]);
    let h: Value<Holder> = Rc::new(RefCell::new(Holder { field: 17_() }));
    &((*h.borrow()).field);
    let mut hp: Ptr<Holder> = (h.as_pointer());
    &((*hp.upgrade().deref()).field);
    let nt: Value<NonTrivial> = Rc::new(RefCell::new(<NonTrivial>::default()));
    ({ unused_ref_param_1(nt.as_pointer()) });
    ({ unused_ptr_param_2((nt.as_pointer())) });
    let g: Value<NonCopyable> = Rc::new(RefCell::new(NonCopyable {
        value: unimplemented!(),
    }));
    (&(*g.borrow_mut()));
    &(*g.borrow_mut());
    ({ unused_noncopyable_param_5(g.as_pointer()) });
    unimplemented!();
    return 0_();
}
pub trait NonCopyableImpl {
    fn move_assign(&self, _a0: Ptr<NonCopyable>) -> Ptr<NonCopyable>;
}
impl NonCopyableImpl for Ptr<NonCopyable> {
    fn move_assign(&self, _a0: Ptr<NonCopyable>) -> Ptr<NonCopyable> {
        unimplemented!();
        return (*self).clone();
    }
}
pub trait NonTrivialImpl {
    fn move_assign(&self, _a0: Ptr<NonTrivial>) -> Ptr<NonTrivial>;
}
impl NonTrivialImpl for Ptr<NonTrivial> {
    fn move_assign(&self, _a0: Ptr<NonTrivial>) -> Ptr<NonTrivial> {
        unimplemented!();
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = side_effect_counter_3.with(|_| ());
}
