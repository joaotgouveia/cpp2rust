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
pub struct node {
    #[offset(0)]
    pub data: (),
    #[offset(8)]
    #[byte_size(8)]
    pub next: Ptr<node>,
}
pub type opt = ();
pub const opt_OPT_STRING_OUT: opt = 0;
pub const opt_OPT_FILE: opt = 1;
pub const opt_OPT_NODE: opt = 2;
pub const opt_OPT_NODE_OUT: opt = 3;
pub fn dispatch_0(option: (), __args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let option: Value<()> = Rc::new(RefCell::new(option));
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*ap.borrow_mut()) = __args.as_pointer();
    let mut result: () = 0_();
    'switch: {
        match { (*option.borrow()) } {
            __v if __v == (opt_OPT_STRING_OUT as i32) => {
                let mut out: Ptr<Ptr<()>> = (*ap.borrow_mut()).arg::<Ptr<Ptr<()>>>();
                out.write(Ptr::<()>::from_string_literal(b"hello"));
                result = 1_();
                break 'switch;
            }
            __v if __v == (opt_OPT_FILE as i32) => {
                let mut f: *mut ::libc::FILE = (*ap.borrow_mut()).arg::<*mut ::libc::FILE>();
                result = ((!((f).is_null())) as ());
                break 'switch;
            }
            __v if __v == (opt_OPT_NODE as i32) => {
                let mut n: Ptr<node> = (*ap.borrow_mut()).arg::<Ptr<node>>();
                result = n.with(|__s| __s.data);
                break 'switch;
            }
            __v if __v == (opt_OPT_NODE_OUT as i32) => {
                let mut out: Ptr<Ptr<node>> = (*ap.borrow_mut()).arg::<Ptr<Ptr<node>>>();
                out.write(Ptr::<node>::null());
                result = 2_();
                break 'switch;
            }
            _ => {}
        }
    };
    return result;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let s: Value<Ptr<()>> = Rc::new(RefCell::new(Ptr::<()>::null()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let head: Value<node> = Rc::new(RefCell::new(node {
        data: 42_(),
        next: Ptr::<node>::null(),
    }));
    unimplemented!();
    let outp: Value<Ptr<node>> = Rc::new(RefCell::new((head.as_pointer())));
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
