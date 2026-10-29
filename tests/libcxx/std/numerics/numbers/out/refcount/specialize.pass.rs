extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn tests_0() -> () {
    let mut f0: () = unimplemented!();
    let mut f1: () = unimplemented!();
    let mut f2: () = unimplemented!();
    let mut f3: () = unimplemented!();
    let mut f4: () = unimplemented!();
    let mut f5: () = unimplemented!();
    let mut f6: () = unimplemented!();
    let mut f7: () = unimplemented!();
    let mut f8: () = unimplemented!();
    let mut f9: () = unimplemented!();
    let mut f10: () = unimplemented!();
    let mut f11: () = unimplemented!();
    let mut f12: () = unimplemented!();
    let mut d0: () = unimplemented!();
    let mut d1: () = unimplemented!();
    let mut d2: () = unimplemented!();
    let mut d3: () = unimplemented!();
    let mut d4: () = unimplemented!();
    let mut d5: () = unimplemented!();
    let mut d6: () = unimplemented!();
    let mut d7: () = unimplemented!();
    let mut d8: () = unimplemented!();
    let mut d9: () = unimplemented!();
    let mut d10: () = unimplemented!();
    let mut d11: () = unimplemented!();
    let mut d12: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut ld0: () = unimplemented!();
    let mut ld1: () = unimplemented!();
    let mut ld2: () = unimplemented!();
    let mut ld3: () = unimplemented!();
    let mut ld4: () = unimplemented!();
    let mut ld5: () = unimplemented!();
    let mut ld6: () = unimplemented!();
    let mut ld7: () = unimplemented!();
    let mut ld8: () = unimplemented!();
    let mut ld9: () = unimplemented!();
    let mut ld10: () = unimplemented!();
    let mut ld11: () = unimplemented!();
    let mut ld12: () = unimplemented!();
    return true;
}
const _: () = assert!(true, "tests()");
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
    ({ tests_0() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
