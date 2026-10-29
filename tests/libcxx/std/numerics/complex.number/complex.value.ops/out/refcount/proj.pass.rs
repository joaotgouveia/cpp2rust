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
pub fn test_isinf_2(mut v: ()) -> () {
    return (v == unimplemented!()) || (v == -unimplemented!());
}
pub fn test_isnan_3(mut v: ()) -> () {
    return (v != v);
}
pub fn classify_4(x: Ptr<()>) -> () {
    if unimplemented!() {
        return (anon_1_zero as ());
    }
    if ({ test_isinf_2(unimplemented!()) }) || ({ test_isinf_2(unimplemented!()) }) {
        return (anon_1_inf as ());
    }
    if ({ test_isnan_3(unimplemented!()) }) && ({ test_isnan_3(unimplemented!()) }) {
        return (anon_1_NaN as ());
    }
    if ({ test_isnan_3(unimplemented!()) }) {
        if (unimplemented!() == 0_()) {
            return (anon_1_NaN as ());
        }
        return (anon_1_non_zero_nan as ());
    }
    if ({ test_isnan_3(unimplemented!()) }) {
        if (unimplemented!() == 0_()) {
            return (anon_1_NaN as ());
        }
        return (anon_1_non_zero_nan as ());
    }
    return (anon_1_non_zero as ());
}
pub fn classify_5(mut x: ()) -> () {
    if (x == 0_()) {
        return (anon_1_zero as ());
    }
    if unimplemented!() {
        return (anon_1_inf as ());
    }
    if unimplemented!() {
        return (anon_1_NaN as ());
    }
    return (anon_1_non_zero as ());
}
pub fn is_about_6(mut x: (), mut y: ()) {
    unimplemented!();
}
pub fn is_about_7(mut x: (), mut y: ()) {
    unimplemented!();
}
pub fn is_about_8(mut x: (), mut y: ()) {
    unimplemented!();
}
pub fn test_9(z: Ptr<()>, x: ()) {
    let x: Value<()> = Rc::new(RefCell::new(x));
    unimplemented!();
}
pub fn test_10(z: Ptr<()>, x: ()) {
    let x: Value<()> = Rc::new(RefCell::new(x));
    unimplemented!();
}
pub fn test_11(z: Ptr<()>, x: ()) {
    let x: Value<()> = Rc::new(RefCell::new(x));
    unimplemented!();
}
pub fn test_12() {
    ({
        let _z: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(_z.as_pointer(), unimplemented!())
    });
    ({
        let _z: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(_z.as_pointer(), unimplemented!())
    });
    ({
        let _z: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(_z.as_pointer(), unimplemented!())
    });
    ({
        let _z: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(_z.as_pointer(), unimplemented!())
    });
}
pub fn test_13() {
    ({
        let _z: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(_z.as_pointer(), unimplemented!())
    });
    ({
        let _z: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(_z.as_pointer(), unimplemented!())
    });
    ({
        let _z: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(_z.as_pointer(), unimplemented!())
    });
    ({
        let _z: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(_z.as_pointer(), unimplemented!())
    });
}
pub fn test_14() {
    ({
        let _z: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_11(_z.as_pointer(), unimplemented!())
    });
    ({
        let _z: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_11(_z.as_pointer(), unimplemented!())
    });
    ({
        let _z: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_11(_z.as_pointer(), unimplemented!())
    });
    ({
        let _z: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_11(_z.as_pointer(), unimplemented!())
    });
}
pub fn test_edges_15() {
    let mut N: () = (2448usize as ()).wrapping_div((16usize as ()));
    let mut i: () = 0_();
    'loop_: while (i < N) {
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        'switch: {
            match { ({ classify_4((testcases_0.with(|v| v.as_pointer()) as Ptr<()>).offset(i)) }) }
            {
                __v if __v == (anon_1_zero as ()) || __v == (anon_1_non_zero as ()) => {
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    break 'switch;
                }
                __v if __v == (anon_1_inf as ()) => {
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    break 'switch;
                }
                __v if __v == (anon_1_NaN as ()) || __v == (anon_1_non_zero_nan as ()) => {
                    unimplemented!();
                    break 'switch;
                }
                _ => {}
            }
        };
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
    ({ test_12() });
    ({ test_13() });
    ({ test_14() });
    ({ test_edges_15() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = testcases_0.with(|_| ());
}
