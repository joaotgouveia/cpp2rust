extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn tests_0() -> () {
    let mut dd0: Ptr<()> = (unimplemented!());
    let mut dd1: Ptr<()> = (unimplemented!());
    let mut dd2: Ptr<()> = (unimplemented!());
    let mut dd3: Ptr<()> = (unimplemented!());
    let mut dd4: Ptr<()> = (unimplemented!());
    let mut dd5: Ptr<()> = (unimplemented!());
    let mut dd6: Ptr<()> = (unimplemented!());
    let mut dd7: Ptr<()> = (unimplemented!());
    let mut dd8: Ptr<()> = (unimplemented!());
    let mut dd9: Ptr<()> = (unimplemented!());
    let mut dd10: Ptr<()> = (unimplemented!());
    let mut dd11: Ptr<()> = (unimplemented!());
    let mut dd12: Ptr<()> = (unimplemented!());
    let mut f0: Ptr<()> = (unimplemented!());
    let mut f1: Ptr<()> = (unimplemented!());
    let mut f2: Ptr<()> = (unimplemented!());
    let mut f3: Ptr<()> = (unimplemented!());
    let mut f4: Ptr<()> = (unimplemented!());
    let mut f5: Ptr<()> = (unimplemented!());
    let mut f6: Ptr<()> = (unimplemented!());
    let mut f7: Ptr<()> = (unimplemented!());
    let mut f8: Ptr<()> = (unimplemented!());
    let mut f9: Ptr<()> = (unimplemented!());
    let mut f10: Ptr<()> = (unimplemented!());
    let mut f11: Ptr<()> = (unimplemented!());
    let mut f12: Ptr<()> = (unimplemented!());
    let mut d0: Ptr<()> = (unimplemented!());
    let mut d1: Ptr<()> = (unimplemented!());
    let mut d2: Ptr<()> = (unimplemented!());
    let mut d3: Ptr<()> = (unimplemented!());
    let mut d4: Ptr<()> = (unimplemented!());
    let mut d5: Ptr<()> = (unimplemented!());
    let mut d6: Ptr<()> = (unimplemented!());
    let mut d7: Ptr<()> = (unimplemented!());
    let mut d8: Ptr<()> = (unimplemented!());
    let mut d9: Ptr<()> = (unimplemented!());
    let mut d10: Ptr<()> = (unimplemented!());
    let mut d11: Ptr<()> = (unimplemented!());
    let mut d12: Ptr<()> = (unimplemented!());
    let mut ld0: Ptr<()> = (unimplemented!());
    let mut ld1: Ptr<()> = (unimplemented!());
    let mut ld2: Ptr<()> = (unimplemented!());
    let mut ld3: Ptr<()> = (unimplemented!());
    let mut ld4: Ptr<()> = (unimplemented!());
    let mut ld5: Ptr<()> = (unimplemented!());
    let mut ld6: Ptr<()> = (unimplemented!());
    let mut ld7: Ptr<()> = (unimplemented!());
    let mut ld8: Ptr<()> = (unimplemented!());
    let mut ld9: Ptr<()> = (unimplemented!());
    let mut ld10: Ptr<()> = (unimplemented!());
    let mut ld11: Ptr<()> = (unimplemented!());
    let mut ld12: Ptr<()> = (unimplemented!());
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
