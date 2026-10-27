extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(0)]
pub struct A {
    #[offset(0)]
    #[byte_size(0)]
    pub m: Box<[()]>,
}
impl Default for A {
    fn default() -> Self {
        {
            A {
                m: (0..0).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
const _: () = assert!((0usize == 0_()), "sizeof(A) == 0");
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
    let a: Value<Box<[A]>> = Rc::new(RefCell::new(
        (0..10).map(|_| <A>::default()).collect::<Box<[A]>>(),
    ));
    let mut p: Ptr<A> = ({ (*begin_0.with(Value::clone).borrow_mut())(a.as_pointer()) });
    unimplemented!();
    let mut cp: Ptr<A> = ({ (*cbegin_1.with(Value::clone).borrow_mut())(a.as_pointer()) });
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
