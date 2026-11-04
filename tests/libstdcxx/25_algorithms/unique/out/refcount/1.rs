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
    let menge: Value<std_list_int__std_allocator_int__> =
        Rc::new(RefCell::new(<std_list_int__std_allocator_int__>::default()));
    match ({ ({ (*menge.borrow()).begin_24() }) }, {
        ({ (*menge.borrow()).end_26() })
    }) {
        (mut __tmp_0, mut __tmp_1) => {
            if __tmp_0 == __tmp_1 {
                (__tmp_1).clone()
            } else {
                let mut __write = __tmp_0.clone();
                let mut __it = (__tmp_0).clone();
                __it.iter_inc();
                while __it != __tmp_1 {
                    if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                        __write.iter_inc();
                        __write.iter_deref().write(__it.iter_deref().read());
                    }
                    __it.iter_inc();
                }
                __write.iter_inc();
                __write
            }
        }
    };
    return 0;
}
pub fn __cpp2rust_init_globals() {}
