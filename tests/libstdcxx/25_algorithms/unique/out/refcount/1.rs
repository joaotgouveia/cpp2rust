extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let menge: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let __first = unimplemented!();
        let __last = unimplemented!();
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                    __write.iter_inc();
                    __write.iter_deref().write(__it.iter_deref().read());
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    return 0;
}
pub fn __cpp2rust_init_globals() {}
