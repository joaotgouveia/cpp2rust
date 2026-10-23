extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int__int_ {}
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct nothrow_convertible_true_ {
    #[offset(0)]
    pub val: (),
}
impl Default for nothrow_convertible_true_ {
    fn default() -> Self {
        { nothrow_convertible_true_ { val: 0_() } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct nothrow_convertible_false_ {
    #[offset(0)]
    pub val: (),
}
impl Default for nothrow_convertible_false_ {
    fn default() -> Self {
        { nothrow_convertible_false_ { val: 0_() } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct convertible_from_int {}
impl convertible_from_int {
    pub fn new(mut _a0: ()) -> Self {
        Self {}
    }
}
pub fn meow_1(_a0: ()) {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
}
pub fn meow_2(mut _a0: convertible_from_int) {}
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
    {
        let t: Value<convertible_to_int_ref> =
            Rc::new(RefCell::new(<convertible_to_int_ref>::default()));
        let r: Value<()> = Rc::new(RefCell::new(
            std_reference_wrapper_convertible_to_int_ref_::new({ t.as_pointer() }),
        ));
        unimplemented!();
    }
    {
        let t: Value<convertible_to_int_ref> =
            Rc::new(RefCell::new(<convertible_to_int_ref>::default()));
        let r: Value<()> = Rc::new(RefCell::new(
            std_reference_wrapper_const_convertible_to_int_ref_::new({ t.as_pointer() }),
        ));
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    ({ meow_2(convertible_from_int::new({ 0_() })) });
    const _: () = assert!(true, "");;
    {
        let i: Value<()> = Rc::new(RefCell::new(0_()));
        let ri: Value<()> = Rc::new(RefCell::new(std_reference_wrapper_int_::new({
            i.as_pointer()
        })));
        const _: () = assert!(
            true,
            "(std::is_same<decltype(ri), std::reference_wrapper<int>>::value)"
        );;
    }
    {
        let j: Value<()> = Rc::new(RefCell::new(0_()));
        let rj: Value<()> = Rc::new(RefCell::new(std_reference_wrapper_const_int_::new({
            j.as_pointer()
        })));
        const _: () = assert!(
            true,
            "(std::is_same<decltype(rj), std::reference_wrapper<const int>>::value)"
        );;
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
pub trait nothrow_convertible_false_Impl {
    fn to_pmut(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl nothrow_convertible_false_Impl for Ptr<nothrow_convertible_false_> {}
pub trait nothrow_convertible_true_Impl {
    fn to_pmut(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl nothrow_convertible_true_Impl for Ptr<nothrow_convertible_true_> {}
pub fn __cpp2rust_init_globals() {}
