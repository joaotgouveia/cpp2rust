extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(2)]
pub struct two {
    #[offset(0)]
    #[byte_size(2)]
    pub __: Box<[()]>,
}
impl Default for two {
    fn default() -> Self {
        {
            two {
                __: (0..2).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
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
    const _: () = assert!(true, "std::denorm_indeterminate == -1");;
    const _: () = assert!(true, "std::denorm_absent == 0");;
    const _: () = assert!(true, "std::denorm_present == 1");;
    const _: () = assert!(
        (::std::mem::size_of::<()>() == 1_()),
        "sizeof(test(std::denorm_present)) == 1"
    );;
    const _: () = assert!((2usize == 2_()), "sizeof(test(1)) == 2");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
