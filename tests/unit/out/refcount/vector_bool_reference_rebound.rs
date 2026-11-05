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
    let vb: Value<Vec<bool>> = Rc::new(RefCell::new(
        (0..(2_usize) as usize).map(|_| false).collect::<Vec<_>>(),
    ));
    match ({ vb.decay().offset(1_usize as isize) },) {
        (mut __tmp_0,) => {
            __tmp_0.write(false);
            (__tmp_0).clone()
        }
    };
    assert!(!(vb.decay().offset(1_usize as isize).read()));
    match ({ vb.decay().offset(0_usize as isize) },) {
        (mut __tmp_1,) => {
            __tmp_1.write(true);
            (__tmp_1).clone()
        }
    };
    assert!(vb.decay().offset(0_usize as isize).read());
    match ({ vb.decay().offset(1_usize as isize) }, {
        vb.decay().offset(0_usize as isize)
    }) {
        (mut __tmp_2, mut __tmp_3) => {
            __tmp_2.write(__tmp_3.read());
            (__tmp_2).clone()
        }
    };
    assert!(vb.decay().offset(1_usize as isize).read());
    let mut b: bool = (match ({ vb.decay().offset(0_usize as isize) }, {
        vb.decay().offset(1_usize as isize)
    }) {
        (mut __tmp_4, mut __tmp_5) => {
            __tmp_4.write(__tmp_5.read());
            (__tmp_4).clone()
        }
    })
    .read();
    assert!(b);
    let mut i: i32 = 0;
    match ({ vb.decay().offset((i.postfix_inc() as usize) as isize) },) {
        (mut __tmp_6,) => {
            __tmp_6.write(false);
            (__tmp_6).clone()
        }
    };
    assert!((!(vb.decay().offset(0_usize as isize).read())) && (i == 1));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
