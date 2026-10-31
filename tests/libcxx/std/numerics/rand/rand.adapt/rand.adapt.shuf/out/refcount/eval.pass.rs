extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct rand1_unsigned_long_long__0__18446744073709551615_ {
    #[offset(0)]
    x_: u64,
}
impl rand1_unsigned_long_long__0__18446744073709551615_ {
    pub fn min() -> u64 {
        return 0_u64;
    }
    pub fn max() -> u64 {
        return 18446744073709551615_u64;
    }
    pub fn new(sd: Option<u64>) -> Self {
        let mut sd: u64 = sd.unwrap_or_else(|| 0_u64);
        let __this: Value<rand1_unsigned_long_long__0__18446744073709551615_> =
            Rc::new(RefCell::new(Self { x_: sd }));
        let this: Ptr<rand1_unsigned_long_long__0__18446744073709551615_> = __this.as_pointer();
        if (this.with(|__s| __s.x_) > 18446744073709551615_u64) {
            field!(this, x_).write(18446744073709551615_u64);
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Callable0<u64> for rand1_unsigned_long_long__0__18446744073709551615_ {
    fn call(&self) -> u64 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            rand1_unsigned_long_long__0__18446744073709551615_Impl::operator_call(__it)
        })
    }
}
impl Default for rand1_unsigned_long_long__0__18446744073709551615_ {
    fn default() -> Self {
        { rand1_unsigned_long_long__0__18446744073709551615_::new(None) }
    }
}
pub fn test1_0() {
    let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!((unimplemented!() == 152607844_()));
}
pub fn test2_1() {
    let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    assert!((unimplemented!() == 501_()));
}
pub fn test3_2() {
    let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    assert!((unimplemented!() == 500_()));
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test1_0() });
    ({ test2_1() });
    ({ test3_2() });
    return 0;
}
pub trait rand1_unsigned_long_long__0__18446744073709551615_Impl {
    fn operator_call(&self) -> u64;
}
impl rand1_unsigned_long_long__0__18446744073709551615_Impl
    for Ptr<rand1_unsigned_long_long__0__18446744073709551615_>
{
    fn operator_call(&self) -> u64 {
        let r: Value<u64> = Rc::new(RefCell::new((*self).with(|__s| __s.x_)));
        if ((*self).with(|__s| __s.x_) < 18446744073709551615_u64) {
            field!((*self), x_).with_mut(|__v| __v.prefix_inc());
        } else {
            field!((*self), x_).write(0_u64);
        }
        return (*r.borrow());
    }
}
pub fn __cpp2rust_init_globals() {}
