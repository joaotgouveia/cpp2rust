extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct TriviallyCopyable {
    #[offset(0)]
    pub i_: (),
}
impl TriviallyCopyable {
    pub fn new(mut i: ()) -> Self {
        Self { i_: i }
    }
}
pub fn test_0(mut t: TriviallyCopyable) {
    let t0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn test_1(t: ()) {
    let t: Value<()> = Rc::new(RefCell::new(t));
    let t0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
    ({ test_0(TriviallyCopyable::new({ 42_() })) });
    ({
        test_1({
            let __tmp_0: Value<()> = Rc::new(RefCell::new(2_()));
            std_chrono_duration_long_long__std_ratio_1__1000000000__::new_1({
                __tmp_0.as_pointer()
            })
        })
    });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
