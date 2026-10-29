extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Handler {
    #[offset(0)]
    pub tag: (),
    #[offset(8)]
    #[byte_size(8)]
    pub cb: FnPtr<fn(()) -> ()>,
}
pub fn double_it_0(mut x: ()) -> () {
    return (x * 2_());
}
pub fn negate_1(mut x: ()) -> () {
    return -x;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct S {}
impl S {
    pub fn pick_1(mut x: ()) -> () {
        return (x + 1_());
    }
    pub fn pick_2(mut x: ()) -> () {
        return (x + 2_());
    }
    pub fn solo(mut x: ()) -> () {
        return (x + 3_());
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    pub data_: (),
}
impl MoveOnly {
    pub fn new(mut data: ()) -> Self {
        Self { data_: data }
    }
    pub fn move_from(x: Ptr<MoveOnly>) -> Self {
        let __this: MoveOnly = Self {
            data_: x.with(|__s| __s.data_),
        };
        field!(x, data_).write(0_());
        __this
    }
}
pub fn make_move_only_2() -> MoveOnly {
    return MoveOnly::new({ 4_() });
}
pub fn scale_move_only_3(x: MoveOnly) -> MoveOnly {
    let x: Value<MoveOnly> = Rc::new(RefCell::new(x));
    return MoveOnly::new({ (({ MoveOnlyImpl::get(&x.as_pointer()) }) * 10_()) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut p1: FnPtr<fn(()) -> ()> = (FnPtr::<fn(()) -> ()>::new(S::pick_1));
    let mut p2: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(S::solo);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut h3: Handler = Handler {
        tag: 3_(),
        cb: (FnPtr::<fn(()) -> ()>::new(S::pick_1)),
    };
    unimplemented!();
    let mut h1: Handler = Handler {
        tag: 1_(),
        cb: FnPtr::<fn(()) -> ()>::new(double_it_0),
    };
    let mut h2: Handler = Handler {
        tag: 2_(),
        cb: FnPtr::<fn(()) -> ()>::new(negate_1),
    };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    h1.cb = FnPtr::<fn(()) -> ()>::new(negate_1);
    unimplemented!();
    unimplemented!();
    let mut mk: FnPtr<fn() -> MoveOnly> = FnPtr::<fn() -> MoveOnly>::new(make_move_only_2);
    let m: Value<MoveOnly> = Rc::new(RefCell::new(({ mk.call() })));
    unimplemented!();
    let mut sc: FnPtr<fn(MoveOnly) -> MoveOnly> =
        FnPtr::<fn(MoveOnly) -> MoveOnly>::new(scale_move_only_3);
    let n: Value<MoveOnly> = Rc::new(RefCell::new(
        ({ sc.call(MoveOnly::move_from({ m.as_pointer() })) }),
    ));
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait MoveOnlyImpl {
    fn get(&self) -> ();
}
impl MoveOnlyImpl for Ptr<MoveOnly> {
    fn get(&self) -> () {
        return (*self).with(|__s| __s.data_);
    }
}
pub fn __cpp2rust_init_globals() {}
