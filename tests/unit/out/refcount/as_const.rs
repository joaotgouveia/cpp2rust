extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Overload = ();
pub const Overload_kMutableOverload: Overload = 1;
pub const Overload_kConstOverload: Overload = 2;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub v: (),
}
pub fn g_0(_a0: Ptr<S>) -> Overload {
    return Overload_kMutableOverload;
}
pub fn g_1(_a0: Ptr<S>) -> Overload {
    return Overload_kConstOverload;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let s: Value<S> = Rc::new(RefCell::new(S { v: 7_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    ({ SImpl::value_ref_3(&s.as_pointer()) }).write(9_());
    unimplemented!();
    unimplemented!();
    let cs: Ptr<S> = s.as_pointer();
    unimplemented!();
    unimplemented!();
    let mut p: Ptr<S> = unimplemented!();
    field!(p, v).write(11_());
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait SImpl {
    fn f_1(&self) -> Overload;
    fn f_2(&self) -> Overload;
    fn value_ref_3(&self) -> Ptr<()>;
    fn value_ref_4(&self) -> Ptr<()>;
}
impl SImpl for Ptr<S> {
    fn f_1(&self) -> Overload {
        return Overload_kMutableOverload;
    }
    fn f_2(&self) -> Overload {
        return Overload_kConstOverload;
    }
    fn value_ref_3(&self) -> Ptr<()> {
        return field_ptr!((*self), v);
    }
    fn value_ref_4(&self) -> Ptr<()> {
        return field_ptr!((*self), v);
    }
}
pub fn __cpp2rust_init_globals() {}
