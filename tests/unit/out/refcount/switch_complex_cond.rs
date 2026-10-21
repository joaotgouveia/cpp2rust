extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn switch_complex_cond_0(mut p: Ptr<()>, mut bias: ()) -> () {
    'switch: {
        match { ({ (p.read()) } + { bias }) } {
            __v if __v == unimplemented!() => {
                return 1_();
            }
            __v if __v == 5_() => {
                return 2_();
            }
            __v if __v == 10_() => {
                return 3_();
            }
            _ => {
                return unimplemented!();
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let p_val: Value<()> = Rc::new(RefCell::new(5_()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
