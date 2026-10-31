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
pub fn test_6(a: Ptr<()>, b: Ptr<f32>, x: ()) {
    let x: Value<()> = Rc::new(RefCell::new(x));
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({
        let _x: f32 = unimplemented!();
        let _y: f32 = unimplemented!();
        is_about_3(_x, _y)
    });
    ({
        let _x: f32 = unimplemented!();
        let _y: f32 = unimplemented!();
        is_about_3(_x, _y)
    });
}
pub fn test_7(a: Ptr<()>, b: Ptr<f64>, x: ()) {
    let x: Value<()> = Rc::new(RefCell::new(x));
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({
        let _x: f64 = unimplemented!();
        let _y: f64 = unimplemented!();
        is_about_4(_x, _y)
    });
    ({
        let _x: f64 = unimplemented!();
        let _y: f64 = unimplemented!();
        is_about_4(_x, _y)
    });
}
pub fn test_8(a: Ptr<()>, b: Ptr<f64>, x: ()) {
    let x: Value<()> = Rc::new(RefCell::new(x));
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({
        let _x: f64 = unimplemented!();
        let _y: f64 = unimplemented!();
        is_about_5(_x, _y)
    });
    ({
        let _x: f64 = unimplemented!();
        let _y: f64 = unimplemented!();
        is_about_5(_x, _y)
    });
}
pub fn test_9() {
    ({
        let _a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _b: Value<f32> = Rc::new(RefCell::new(2_f32));
        test_6(_a.as_pointer(), _b.as_pointer(), unimplemented!())
    });
}
pub fn test_10() {
    ({
        let _a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _b: Value<f64> = Rc::new(RefCell::new(2_f64));
        test_7(_a.as_pointer(), _b.as_pointer(), unimplemented!())
    });
}
pub fn test_11() {
    ({
        let _a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _b: Value<f64> = Rc::new(RefCell::new(2_f64));
        test_8(_a.as_pointer(), _b.as_pointer(), unimplemented!())
    });
}
pub fn test_edges_12() {
    let mut N: u32 = (((2448usize as usize).wrapping_div((16usize as usize))) as u32);
    let mut i: u32 = 0_u32;
    'loop_: while (i < N) {
        let mut j: u32 = 0_u32;
        'loop_: while (j < N) {
            let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let z: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            if unimplemented!() {
                assert!(unimplemented!());
            } else {
                assert!((unimplemented!() == unimplemented!()));
            }
            if unimplemented!() {
                assert!(unimplemented!());
            } else {
                assert!((unimplemented!() == unimplemented!()));
            }
            j.prefix_inc();
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
