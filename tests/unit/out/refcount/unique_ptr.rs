extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(8)]
pub struct SafePointer {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: (),
}
impl SafePointer {
    pub fn move_from(_a0: Ptr<SafePointer>) -> Self {
        Self {
            ptr: unimplemented!(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Pair {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    pub y: (),
}
pub fn DoStuffWithSafePointer_0(safe_ptr: Ptr<()>) {
    let mut x1: () = unimplemented!();
    let mut x2: () = unimplemented!();
    unimplemented!().write(1_());
    unimplemented!();
    let mut raw_ptr1: Ptr<()> = (unimplemented!());
    raw_ptr1.with_mut(|__v| __v.prefix_inc());
    unimplemented!();
    ({ SafePointerImpl::inc(&(unimplemented!())) });
    ({ SafePointerImpl::inc(&(unimplemented!())) });
    let mut x3: () = unimplemented!();
    let mut x4: () = unimplemented!();
    let __rhs = ((unimplemented!().read()) + (unimplemented!().read()));
    unimplemented!().write(__rhs);
    unimplemented!();
    let mut raw_ptr2: Ptr<()> = (unimplemented!());
    {
        raw_ptr2.with_mut(|__v| *__v = *__v + 1_())
    };
    let mut pair: () = unimplemented!();
    ({ PairImpl::inc(&(unimplemented!()), 10_()) });
    let __rhs = ({ ({ (unimplemented!().read()) } + { unimplemented!().with(|__s| (__s).x) }) }
        + { unimplemented!().with(|__s| (__s).y) });
    unimplemented!().write(__rhs);
}
pub fn Consume_1(mut safe_ptr: ()) -> () {
    let mut x: () = unimplemented!();
    let mut p: () =
        std_unique_ptr_Pair__std_default_delete_Pair__::new({ Ptr::alloc(<Pair>::default()) });
    return ((unimplemented!().read()) + { unimplemented!().x });
}
pub fn RndStuff_2() {
    let mut x1: () = unimplemented!();
    let mut x2: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < 100_()) {
        unimplemented!().write(1_());
        i.prefix_inc();
    }
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < 200_()) {
        unimplemented!().write(2_());
        i.prefix_inc();
    }
    let mut p2: Ptr<()> = unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < 200_()) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut x3: () = ({ make_unique_3(10_()) });
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        unimplemented!().write(Pair { x: 1_(), y: 2_() });
        i.prefix_inc();
    }
    let mut p3_0: Ptr<Pair> = unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        unimplemented!();
        unimplemented!();
        ({ PairImpl::inc(&unimplemented!(), 10_()) });
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < 50_()) {
        unimplemented!().write(Pair { x: -1_(), y: -2_() });
        i.prefix_inc();
    }
    let mut p3_1: Ptr<Pair> = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < 50_()) {
        unimplemented!();
        unimplemented!();
        ({ PairImpl::inc(&unimplemented!(), -10_()) });
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut x: () = unimplemented!();
    let safe_ptr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ DoStuffWithSafePointer_0(safe_ptr.as_pointer()) });
    unimplemented!();
    let mut p: () = std_unique_ptr_int__std_default_delete_int__::new({ Ptr::alloc(1_()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait PairImpl {
    fn inc(&self, k: ());
}
impl PairImpl for Ptr<Pair> {
    fn inc(&self, mut k: ()) {
        {
            let __rhs = k;
            field!((*self), x).with_mut(|__v| *__v = *__v + __rhs)
        };
        {
            let __rhs = k;
            field!((*self), y).with_mut(|__v| *__v = *__v + __rhs)
        };
    }
}
pub trait SafePointerImpl {
    fn inc(&self);
    fn move_assign(&self, _a0: Ptr<SafePointer>) -> Ptr<SafePointer>;
}
impl SafePointerImpl for Ptr<SafePointer> {
    fn inc(&self) {
        unimplemented!().with_mut(|__v| __v.prefix_inc());
    }
    fn move_assign(&self, _a0: Ptr<SafePointer>) -> Ptr<SafePointer> {
        unimplemented!();
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
