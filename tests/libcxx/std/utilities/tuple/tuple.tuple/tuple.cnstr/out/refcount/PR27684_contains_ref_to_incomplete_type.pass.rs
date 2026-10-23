extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static cinc1_2: Ptr<IncompleteType> = inc1_0.with(|v| v.as_pointer());
);
thread_local!(
    pub static cinc2_3: Ptr<IncompleteType> = inc2_1.with(|v| v.as_pointer());
);

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
        let t: Value<()> = Rc::new(RefCell::new(
            std_tuple_const_IncompleteType_ref__const_IncompleteType_ref_::new_1(
                { (cinc1_2.with(Ptr::clone)).clone() },
                { (cinc2_3.with(Ptr::clone)).clone() },
            ),
        ));
        unimplemented!();
        unimplemented!();
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(
            std_tuple_const_IncompleteType_ref__const_IncompleteType_ref_::new_2(
                { inc1_0.with(|v| v.as_pointer()) },
                { inc2_1.with(|v| v.as_pointer()) },
            ),
        ));
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct IncompleteType {}
thread_local!(
    pub static inc1_0: Value<IncompleteType> = Rc::new(RefCell::new(<IncompleteType>::default()));
);
thread_local!(
    pub static inc2_1: Value<IncompleteType> = Rc::new(RefCell::new(<IncompleteType>::default()));
);
pub fn __cpp2rust_init_globals() {
    let _ = cinc1_2.with(|_| ());
    let _ = cinc2_3.with(|_| ());
    let _ = inc1_0.with(|_| ());
    let _ = inc2_1.with(|_| ());
}
