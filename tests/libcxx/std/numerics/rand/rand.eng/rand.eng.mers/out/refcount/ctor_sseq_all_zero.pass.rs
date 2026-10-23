extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct all_zero_seed_seq {}
impl all_zero_seed_seq {
    pub fn new_1() -> Self {
        Self {}
    }
    pub fn new_2(_a0: ()) -> Self {
        let _a0: Value<()> = Rc::new(RefCell::new(_a0));
        Self {}
    }
}
impl Default for all_zero_seed_seq {
    fn default() -> Self {
        { all_zero_seed_seq::new_1() }
    }
}
pub fn test_0() {
    let mut state_size: () = 1_();
    let mut shift_size: () = 1_();
    let mut tempering_l: () = 3_();
    let mut q: all_zero_seed_seq = all_zero_seed_seq::new_1();
    let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut Xneg1: () = (1_() << ((3_() as ()).wrapping_sub(1_())));
    let mut Y: () = Xneg1;
    let mut X0: () = (Xneg1 ^ (Y >> 1_()));
    unimplemented!();
}
pub fn test_1() {
    let mut state_size: () = 1_();
    let mut shift_size: () = 1_();
    let mut tempering_l: () = 33_();
    let mut q: all_zero_seed_seq = all_zero_seed_seq::new_1();
    let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut Xneg1: () = (1_() << ((33_() as ()).wrapping_sub(1_())));
    let mut Y: () = Xneg1;
    let mut X0: () = (Xneg1 ^ (Y >> 1_()));
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
    ({ test_0() });
    ({ test_1() });
    return 0_();
}
pub trait all_zero_seed_seqImpl {
    fn size(&self) -> ();
    fn generate_4_unsigned_int_ptr(&self, rb: Ptr<()>, re: Ptr<()>);
}
impl all_zero_seed_seqImpl for Ptr<all_zero_seed_seq> {
    fn size(&self) -> () {
        return 0_();
    }
    fn generate_4_unsigned_int_ptr(&self, mut rb: Ptr<()>, mut re: Ptr<()>) {
        unimplemented!();
    }
}
pub fn __cpp2rust_init_globals() {}
