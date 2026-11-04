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
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3])));
    assert!(
        (((*match (Rc::new(RefCell::new(CppRange::end(&v.decay()))).as_pointer(),) {
            (__tmp_0,) => {
                {
                    __tmp_0.with_mut(|__it| {
                        __it.iter_dec();
                    });
                    (__tmp_0).clone()
                }
            }
        }
        .upgrade()
        .deref())
        .iter_at(0_isize)
        .read())
            == 3)
    );
    assert!(
        (((*match (Rc::new(RefCell::new(CppRange::end(&v.decay()))).as_pointer(),) {
            (__tmp_1,) => {
                {
                    __tmp_1.with_mut(|__it| {
                        __it.iter_dec();
                    });
                    (__tmp_1).clone()
                }
            }
        }
        .upgrade()
        .deref())
        .iter_deref()
        .read())
            == 3)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
