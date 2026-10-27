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
pub type anon_1 = ();
pub const anon_1_zero: anon_1 = 0;
pub const anon_1_non_zero: anon_1 = 1;
pub const anon_1_inf: anon_1 = 2;
pub const anon_1_NaN: anon_1 = 3;
pub const anon_1_non_zero_nan: anon_1 = 4;
pub fn classify_2(mut x: ()) -> () {
    if (x == 0_()) {
        return (anon_1_zero as ());
    }
    if ({ isinf_3(x) }) {
        return (anon_1_inf as ());
    }
    if ({ isnan_4(x) }) {
        return (anon_1_NaN as ());
    }
    return (anon_1_non_zero as ());
}
pub fn is_about_5(mut x: (), mut y: ()) {
    unimplemented!();
}
pub fn is_about_6(mut x: (), mut y: ()) {
    unimplemented!();
}
pub fn is_about_7(mut x: (), mut y: ()) {
    unimplemented!();
}
pub fn test_8(a: Ptr<()>, b: Ptr<()>, x: ()) {
    let x: Value<()> = Rc::new(RefCell::new(x));
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({
        let _x: () = unimplemented!();
        let _y: () = unimplemented!();
        is_about_5(_x, _y)
    });
    unimplemented!();
}
pub fn test_9(a: Ptr<()>, b: Ptr<()>, x: ()) {
    let x: Value<()> = Rc::new(RefCell::new(x));
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({
        let _x: () = unimplemented!();
        let _y: () = unimplemented!();
        is_about_6(_x, _y)
    });
    unimplemented!();
}
pub fn test_10(a: Ptr<()>, b: Ptr<()>, x: ()) {
    let x: Value<()> = Rc::new(RefCell::new(x));
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({
        let _x: () = unimplemented!();
        let _y: () = unimplemented!();
        is_about_7(_x, _y)
    });
    unimplemented!();
}
pub fn test_11() {
    ({
        let _a: Value<()> = Rc::new(RefCell::new(2_()));
        let _b: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_8(_a.as_pointer(), _b.as_pointer(), unimplemented!())
    });
}
pub fn test_12() {
    ({
        let _a: Value<()> = Rc::new(RefCell::new(2_()));
        let _b: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(_a.as_pointer(), _b.as_pointer(), unimplemented!())
    });
}
pub fn test_13() {
    ({
        let _a: Value<()> = Rc::new(RefCell::new(2_()));
        let _b: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(_a.as_pointer(), _b.as_pointer(), unimplemented!())
    });
}
pub fn test_edges_14() {
    let mut N: () = (2448usize as ()).wrapping_div((16usize as ()));
    let mut i: () = 0_();
    'loop_: while (i < N) {
        let mut j: () = 0_();
        'loop_: while (j < N) {
            let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let z: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            if ({ isnan_4(unimplemented!()) }) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            if ({ isnan_4(unimplemented!()) }) {
                unimplemented!();
            } else {
                unimplemented!();
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_11() });
    ({ test_12() });
    ({ test_13() });
    ({ test_edges_14() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = testcases_0.with(|_| ());
}
