extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};

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
    let h: Value<()> = Rc::new(RefCell::new(({ operator_literal_h_0(4_()) })));
    unimplemented!();
    let h2: Value<()> = Rc::new(RefCell::new(({ operator_literal_h_1(4_f64) })));
    unimplemented!();
    let min: Value<()> = Rc::new(RefCell::new(({ operator_literal_min_2(36_()) })));
    unimplemented!();
    let min2: Value<()> = Rc::new(RefCell::new(({ operator_literal_min_3(36_f64) })));
    unimplemented!();
    let s: Value<()> = Rc::new(RefCell::new(({ operator_literal_s_4(24_()) })));
    unimplemented!();
    let s2: Value<()> = Rc::new(RefCell::new(({ operator_literal_s_5(24_f64) })));
    unimplemented!();
    let ms: Value<()> = Rc::new(RefCell::new(({ operator_literal_ms_6(247_()) })));
    unimplemented!();
    let ms2: Value<()> = Rc::new(RefCell::new(({ operator_literal_ms_7(247_f64) })));
    unimplemented!();
    let us: Value<()> = Rc::new(RefCell::new(({ operator_literal_us_8(867_()) })));
    unimplemented!();
    let us2: Value<()> = Rc::new(RefCell::new(({ operator_literal_us_9(867_f64) })));
    unimplemented!();
    let ns: Value<()> = Rc::new(RefCell::new(({ operator_literal_ns_10(645_()) })));
    unimplemented!();
    let ns2: Value<()> = Rc::new(RefCell::new(({ operator_literal_ns_11(645_f64) })));
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
