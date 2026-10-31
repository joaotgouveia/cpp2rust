extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn tests_0() -> bool {
    let mut f0: f32 = unimplemented!();
    let mut f1: f32 = unimplemented!();
    let mut f2: f32 = unimplemented!();
    let mut f3: f32 = unimplemented!();
    let mut f4: f32 = unimplemented!();
    let mut f5: f32 = unimplemented!();
    let mut f6: f32 = unimplemented!();
    let mut f7: f32 = unimplemented!();
    let mut f8: f32 = unimplemented!();
    let mut f9: f32 = unimplemented!();
    let mut f10: f32 = unimplemented!();
    let mut f11: f32 = unimplemented!();
    let mut f12: f32 = unimplemented!();
    let mut d0: f64 = unimplemented!();
    let mut d1: f64 = unimplemented!();
    let mut d2: f64 = unimplemented!();
    let mut d3: f64 = unimplemented!();
    let mut d4: f64 = unimplemented!();
    let mut d5: f64 = unimplemented!();
    let mut d6: f64 = unimplemented!();
    let mut d7: f64 = unimplemented!();
    let mut d8: f64 = unimplemented!();
    let mut d9: f64 = unimplemented!();
    let mut d10: f64 = unimplemented!();
    let mut d11: f64 = unimplemented!();
    let mut d12: f64 = unimplemented!();
    assert!((d0 == unimplemented!()));
    assert!((d1 == unimplemented!()));
    assert!((d2 == unimplemented!()));
    assert!((d3 == unimplemented!()));
    assert!((d4 == unimplemented!()));
    assert!((d5 == unimplemented!()));
    assert!((d6 == unimplemented!()));
    assert!((d7 == unimplemented!()));
    assert!((d8 == unimplemented!()));
    assert!((d9 == unimplemented!()));
    assert!((d10 == unimplemented!()));
    assert!((d11 == unimplemented!()));
    assert!((d12 == unimplemented!()));
    let mut ld0: f64 = unimplemented!();
    let mut ld1: f64 = unimplemented!();
    let mut ld2: f64 = unimplemented!();
    let mut ld3: f64 = unimplemented!();
    let mut ld4: f64 = unimplemented!();
    let mut ld5: f64 = unimplemented!();
    let mut ld6: f64 = unimplemented!();
    let mut ld7: f64 = unimplemented!();
    let mut ld8: f64 = unimplemented!();
    let mut ld9: f64 = unimplemented!();
    let mut ld10: f64 = unimplemented!();
    let mut ld11: f64 = unimplemented!();
    let mut ld12: f64 = unimplemented!();
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ tests_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
