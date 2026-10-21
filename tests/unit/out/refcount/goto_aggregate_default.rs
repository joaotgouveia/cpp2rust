extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Point {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    pub y: (),
}
pub fn agg_0(mut n: ()) -> () {
    let mut buf40: [(); 40] = [unimplemented!(); 40];
    let mut buf256: [(); 256] = [unimplemented!(); 256];
    let mut arr64: [(); 64] = [unimplemented!(); 64];
    let mut longs: [(); 33] = [unimplemented!(); 33];
    let mut p: Point = <Point>::default();
    let mut ptr: Ptr<()> = Ptr::<()>::null();
    let mut fp: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::null();
    let mut file: () = unimplemented!();
    let mut total: () = unimplemented!();
    goto_block!({
        '__entry: {
            total = unimplemented!();
            if (((n < unimplemented!()) as ()) != 0) {
                goto!('out);
            }
            total = 1_();
        }
        'out: {
            return total;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
