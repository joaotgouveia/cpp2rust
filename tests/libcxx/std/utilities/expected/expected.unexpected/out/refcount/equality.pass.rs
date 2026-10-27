extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(_a0: Ptr<Error>, _a1: Ptr<Error>) -> () {
    return ({ _a0.with(|__s| __s.i) } == { _a1.with(|__s| __s.i) });
}
pub fn operator_eq_1(lhs: Ptr<Error>, mut rhs: ()) -> () {
    return ({ lhs.with(|__s| __s.i) } == { rhs });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Error {
    #[offset(0)]
    pub i: (),
}
impl std::cmp::PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_0(
                Rc::new(RefCell::new(Error { i: self.i.clone() })).as_pointer(),
                Rc::new(RefCell::new(Error { i: other.i.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Error {}
pub fn test_2() -> () {
    let unex1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let unex2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let unex3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let unex_i1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let unex_i2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
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
    ({ test_2() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
