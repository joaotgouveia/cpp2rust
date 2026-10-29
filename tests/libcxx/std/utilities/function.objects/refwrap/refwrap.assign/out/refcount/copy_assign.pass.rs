extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct functor1 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct convertible_to_int_ref {
    #[offset(0)]
    pub val: (),
}
impl Default for convertible_to_int_ref {
    fn default() -> Self {
        { convertible_to_int_ref { val: 0_() } }
    }
}
pub fn test_0(t: Ptr<FnPtr<fn()>>) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut t2: FnPtr<fn()> = (t.read());
    let r2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_1(t: Ptr<functor1>) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut t2: functor1 = (*t.upgrade().deref()).clone();
    let r2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_2(t: Ptr<()>) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut t2: () = (t.read());
    let r2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_3(t: Ptr<()>) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut t2: () = (t.read());
    let r2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_4(t: Ptr<convertible_to_int_ref>) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut t2: convertible_to_int_ref = (*t.upgrade().deref()).clone();
    let r2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_5(t: Ptr<convertible_to_int_ref>) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut t2: convertible_to_int_ref = (*t.upgrade().deref()).clone();
    let r2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn f_6() {}
pub fn g_7() {}
pub fn test_function_8() {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let r2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    let fp: Value<FnPtr<fn()>> = Rc::new(RefCell::new(FnPtr::<fn()>::new(f_6)));
    ({ test_0(fp.as_pointer()) });
    ({ test_function_8() });
    let f1: Value<functor1> = Rc::new(RefCell::new(<functor1>::default()));
    ({ test_1(f1.as_pointer()) });
    let i: Value<()> = Rc::new(RefCell::new(0_()));
    ({ test_2(i.as_pointer()) });
    let j: Value<()> = Rc::new(RefCell::new(0_()));
    ({ test_3(j.as_pointer()) });
    let convi: Value<convertible_to_int_ref> =
        Rc::new(RefCell::new(<convertible_to_int_ref>::default()));
    ({ test_4(convi.as_pointer()) });
    let convic: Value<convertible_to_int_ref> =
        Rc::new(RefCell::new(<convertible_to_int_ref>::default()));
    ({ test_5(convic.as_pointer()) });
    {
        const _: () = assert!(true, "(std::is_assignable<Ref&, int&>::value)");;
        const _: () = assert!(true, "(!std::is_assignable<Ref&, int>::value)");;
        const _: () = assert!(true, "(!std::is_assignable<Ref&, int&&>::value)");;
    }
    return 0_();
}
pub trait convertible_to_int_refImpl {
    fn to_pmut(&self) -> Ptr<()>;
    fn to_pconst(&self) -> Ptr<()>;
}
impl convertible_to_int_refImpl for Ptr<convertible_to_int_ref> {
    fn to_pmut(&self) -> Ptr<()> {
        return field_ptr!((*self), val);
    }
    fn to_pconst(&self) -> Ptr<()> {
        return field_ptr!((*self), val);
    }
}
pub fn __cpp2rust_init_globals() {}
