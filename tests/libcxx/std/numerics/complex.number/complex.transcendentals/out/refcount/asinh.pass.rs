extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static testcases_0: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
);
pub type anon_1 = u32;
pub const anon_1_zero: anon_1 = 0;
pub const anon_1_non_zero: anon_1 = 1;
pub const anon_1_inf: anon_1 = 2;
pub const anon_1_NaN: anon_1 = 3;
pub const anon_1_non_zero_nan: anon_1 = 4;
pub fn classify_2(mut x: f64) -> i32 {
    if (x == 0_f64) {
        return (anon_1_zero as i32);
    }
    if unimplemented!() {
        return (anon_1_inf as i32);
    }
    if unimplemented!() {
        return (anon_1_NaN as i32);
    }
    return (anon_1_non_zero as i32);
}
pub fn is_about_3(mut x: f32, mut y: f32) {
    assert!(((unimplemented!() as f64) < 1e-06_f64));
}
pub fn is_about_4(mut x: f64, mut y: f64) {
    assert!((unimplemented!() < 1e-14_f64));
}
pub fn is_about_5(mut x: f64, mut y: f64) {
    assert!((unimplemented!() < 1e-14_f64));
}
pub fn test_6(c: Ptr<()>, x: ()) {
    let x: Value<()> = Rc::new(RefCell::new(x));
    assert!(unimplemented!());
}
pub fn test_7(c: Ptr<()>, x: ()) {
    let x: Value<()> = Rc::new(RefCell::new(x));
    assert!(unimplemented!());
}
pub fn test_8(c: Ptr<()>, x: ()) {
    let x: Value<()> = Rc::new(RefCell::new(x));
    assert!(unimplemented!());
}
pub fn test_9() {
    ({
        let _c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_6(_c.as_pointer(), unimplemented!())
    });
}
pub fn test_10() {
    ({
        let _c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_7(_c.as_pointer(), unimplemented!())
    });
}
pub fn test_11() {
    ({
        let _c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_8(_c.as_pointer(), unimplemented!())
    });
}
pub fn test_edges_12() {
    let mut pi: f64 = unimplemented!();
    let mut N: u32 = (((2448usize as usize).wrapping_div((16usize as usize))) as u32);
    let mut i: u32 = 0_u32;
    'loop_: while (i < N) {
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        if (unimplemented!() == 0_f64) && (unimplemented!() == 0_f64) {
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
        } else if (unimplemented!() == 0_f64) && (unimplemented!() == 1_f64) {
            assert!((unimplemented!() == 0_f64));
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            if unimplemented!() {
                ({ is_about_4(unimplemented!(), (-pi / 2_f64)) });
            } else {
                ({ is_about_4(unimplemented!(), (pi / 2_f64)) });
            }
        } else if (unimplemented!()) && (unimplemented!()) {
            assert!(unimplemented!());
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            if unimplemented!() {
                ({ is_about_4(unimplemented!(), (-pi / 2_f64)) });
            } else {
                ({ is_about_4(unimplemented!(), (pi / 2_f64)) });
            }
        } else if (unimplemented!()) && (unimplemented!()) {
            assert!(unimplemented!());
            assert!(unimplemented!());
        } else if (unimplemented!()) && (unimplemented!()) {
            assert!(unimplemented!());
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            assert!((unimplemented!() == 0_f64));
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
        } else if (unimplemented!()) && (unimplemented!()) {
            assert!(unimplemented!());
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            if unimplemented!() {
                ({ is_about_4(unimplemented!(), (-pi / 4_f64)) });
            } else {
                ({ is_about_4(unimplemented!(), (pi / 4_f64)) });
            }
        } else if (unimplemented!()) && (unimplemented!()) {
            assert!(unimplemented!());
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            assert!(unimplemented!());
        } else if (unimplemented!()) && (unimplemented!() == 0_f64) {
            assert!(unimplemented!());
            assert!((unimplemented!() == 0_f64));
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
        } else if (unimplemented!()) && (unimplemented!()) {
            assert!(unimplemented!());
            assert!(unimplemented!());
        } else if (unimplemented!()) && (unimplemented!()) {
            assert!(unimplemented!());
            assert!(unimplemented!());
        } else if (unimplemented!()) && (unimplemented!()) {
            assert!(unimplemented!());
            assert!(unimplemented!());
        } else {
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
        }
        i.prefix_inc();
    }
}
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
    ({ test_9() });
    ({ test_10() });
    ({ test_11() });
    ({ test_edges_12() });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = testcases_0.with(|_| ());
}
