extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn double_it_0(mut x: ()) -> () {
    return (x * 2_());
}
pub fn test_roundtrip_1() {
    let mut fn_: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(double_it_0);
    unimplemented!();
    let mut gfn: FnPtr<fn()> = fn_.cast::<fn()>();
    unimplemented!();
    let mut fn2: FnPtr<fn(()) -> ()> = gfn.cast::<fn(()) -> ()>();
    unimplemented!();
    unimplemented!();
}
pub fn test_double_cast_2() {
    let mut fn_: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(double_it_0);
    let mut fn2: FnPtr<fn(()) -> ()> = fn_.cast::<fn()>().cast::<fn(()) -> ()>();
    unimplemented!();
    unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Command {
    #[offset(0)]
    #[byte_size(8)]
    pub data: (),
}
pub fn test_void_ptr_to_fn_3() {
    let mut cmd: Command = <Command>::default();
    cmd.data = FnPtr::<fn(()) -> ()>::new(double_it_0).to_any();
    let mut fn_: FnPtr<fn(()) -> ()> = cmd
        .data
        .cast_fn::<fn(()) -> ()>()
        .expect("ub:wrong fn type");
    unimplemented!();
}
pub fn add_offset_4(mut base: Ptr<()>, mut offset: ()) -> () {
    return ({ (base.read()) } + { offset });
}
pub fn test_call_through_cast_5() {
    let mut gfn: FnPtr<fn((), ()) -> ()> =
        FnPtr::<fn(Ptr<()>, ()) -> ()>::new(add_offset_4).cast::<fn((), ()) -> ()>();
    let val: Value<()> = Rc::new(RefCell::new(100_()));
    let mut result: () = ({ gfn.call(((val.as_pointer()) as Ptr<()>).to_any(), 42_()) });
    unimplemented!();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test_roundtrip_1() });
    ({ test_double_cast_2() });
    ({ test_void_ptr_to_fn_3() });
    ({ test_call_through_cast_5() });
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
