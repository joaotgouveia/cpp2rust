extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn yield_vector_0() -> () {
    {
        let foo: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3])));
        let mut x: i32 = 123;
        unimplemented!();
        unimplemented!();
        x = 456;
        unimplemented!();
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let __tmp_0: Value<()> = Rc::new(RefCell::new(({ yield_vector_0() })));
    let __range1: Ptr<()> = __tmp_0.as_pointer();
    'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut x: i32 = (unimplemented!().read());
        {
            let __s = {
                let __s = libcc2rs::cout();
                write_int_refcount(&__s, x);
                __s
            };
            let _ = __s.write_all(&[('\n' as i8) as u8]);
            __s
        };
    }
    0
}
pub fn __cpp2rust_init_globals() {}
