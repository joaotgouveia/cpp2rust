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
    if ({ isinf_6(x) }) {
        return (anon_1_inf as ());
    }
    if ({ isnan_7(x) }) {
        return (anon_1_NaN as ());
    }
    return (anon_1_non_zero as ());
}
pub fn is_about_8(mut x: (), mut y: ()) {
    unimplemented!();
}
pub fn is_about_9(mut x: (), mut y: ()) {
    unimplemented!();
}
pub fn is_about_10(mut x: (), mut y: ()) {
    unimplemented!();
}
pub fn test_11(rho: Ptr<()>, x: ()) {
    let x: Value<()> = Rc::new(RefCell::new(x));
    unimplemented!();
}
pub fn test_12(rho: Ptr<()>, x: ()) {
    let x: Value<()> = Rc::new(RefCell::new(x));
    unimplemented!();
}
pub fn test_13(rho: Ptr<()>, x: ()) {
    let x: Value<()> = Rc::new(RefCell::new(x));
    unimplemented!();
}
pub fn test_14(rho: Ptr<()>, theta: Ptr<()>, x: ()) {
    let x: Value<()> = Rc::new(RefCell::new(x));
    unimplemented!();
}
pub fn test_15(rho: Ptr<()>, theta: Ptr<()>, x: ()) {
    let x: Value<()> = Rc::new(RefCell::new(x));
    unimplemented!();
}
pub fn test_16(rho: Ptr<()>, theta: Ptr<()>, x: ()) {
    let x: Value<()> = Rc::new(RefCell::new(x));
    unimplemented!();
}
pub fn test_17() {
    ({
        let _rho: Value<()> = Rc::new(RefCell::new(0_()));
        test_11(_rho.as_pointer(), unimplemented!())
    });
    ({
        let _rho: Value<()> = Rc::new(RefCell::new(1_()));
        test_11(_rho.as_pointer(), unimplemented!())
    });
    ({
        let _rho: Value<()> = Rc::new(RefCell::new(100_()));
        test_11(_rho.as_pointer(), unimplemented!())
    });
    ({
        let _rho: Value<()> = Rc::new(RefCell::new(0_()));
        let _theta: Value<()> = Rc::new(RefCell::new(0_()));
        test_14(_rho.as_pointer(), _theta.as_pointer(), unimplemented!())
    });
    ({
        let _rho: Value<()> = Rc::new(RefCell::new(1_()));
        let _theta: Value<()> = Rc::new(RefCell::new(0_()));
        test_14(_rho.as_pointer(), _theta.as_pointer(), unimplemented!())
    });
    ({
        let _rho: Value<()> = Rc::new(RefCell::new(100_()));
        let _theta: Value<()> = Rc::new(RefCell::new(0_()));
        test_14(_rho.as_pointer(), _theta.as_pointer(), unimplemented!())
    });
}
pub fn test_18() {
    ({
        let _rho: Value<()> = Rc::new(RefCell::new(0_()));
        test_12(_rho.as_pointer(), unimplemented!())
    });
    ({
        let _rho: Value<()> = Rc::new(RefCell::new(1_()));
        test_12(_rho.as_pointer(), unimplemented!())
    });
    ({
        let _rho: Value<()> = Rc::new(RefCell::new(100_()));
        test_12(_rho.as_pointer(), unimplemented!())
    });
    ({
        let _rho: Value<()> = Rc::new(RefCell::new(0_()));
        let _theta: Value<()> = Rc::new(RefCell::new(0_()));
        test_15(_rho.as_pointer(), _theta.as_pointer(), unimplemented!())
    });
    ({
        let _rho: Value<()> = Rc::new(RefCell::new(1_()));
        let _theta: Value<()> = Rc::new(RefCell::new(0_()));
        test_15(_rho.as_pointer(), _theta.as_pointer(), unimplemented!())
    });
    ({
        let _rho: Value<()> = Rc::new(RefCell::new(100_()));
        let _theta: Value<()> = Rc::new(RefCell::new(0_()));
        test_15(_rho.as_pointer(), _theta.as_pointer(), unimplemented!())
    });
}
pub fn test_19() {
    ({
        let _rho: Value<()> = Rc::new(RefCell::new(0_()));
        test_13(_rho.as_pointer(), unimplemented!())
    });
    ({
        let _rho: Value<()> = Rc::new(RefCell::new(1_()));
        test_13(_rho.as_pointer(), unimplemented!())
    });
    ({
        let _rho: Value<()> = Rc::new(RefCell::new(100_()));
        test_13(_rho.as_pointer(), unimplemented!())
    });
    ({
        let _rho: Value<()> = Rc::new(RefCell::new(0_()));
        let _theta: Value<()> = Rc::new(RefCell::new(0_()));
        test_16(_rho.as_pointer(), _theta.as_pointer(), unimplemented!())
    });
    ({
        let _rho: Value<()> = Rc::new(RefCell::new(1_()));
        let _theta: Value<()> = Rc::new(RefCell::new(0_()));
        test_16(_rho.as_pointer(), _theta.as_pointer(), unimplemented!())
    });
    ({
        let _rho: Value<()> = Rc::new(RefCell::new(100_()));
        let _theta: Value<()> = Rc::new(RefCell::new(0_()));
        test_16(_rho.as_pointer(), _theta.as_pointer(), unimplemented!())
    });
}
pub fn test_edges_20() {
    let mut N: () = (2448usize as ()).wrapping_div((16usize as ()));
    let mut i: () = 0_();
    'loop_: while (i < N) {
        let mut r: () = unimplemented!();
        let mut theta: () = unimplemented!();
        let z: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        'switch: {
            match { ({ classify_5(r) }) } {
                __v if __v == (anon_1_zero as ()) => {
                    if (({ signbit_21(r) }) || (({ classify_5(theta) }) == (anon_1_inf as ())))
                        || (({ classify_5(theta) }) == (anon_1_NaN as ()))
                    {
                        let mut c: () = ({ classify_4(z.as_pointer()) });
                        unimplemented!();
                    } else {
                        unimplemented!();
                    }
                    break 'switch;
                }
                __v if __v == (anon_1_non_zero as ()) => {
                    if (({ signbit_21(r) }) || (({ classify_5(theta) }) == (anon_1_inf as ())))
                        || (({ classify_5(theta) }) == (anon_1_NaN as ()))
                    {
                        let mut c: () = ({ classify_4(z.as_pointer()) });
                        unimplemented!();
                    } else {
                        ({ is_about_9(unimplemented!(), r) });
                    }
                    break 'switch;
                }
                __v if __v == (anon_1_inf as ()) => {
                    if (r < 0_()) {
                        let mut c: () = ({ classify_4(z.as_pointer()) });
                        unimplemented!();
                    } else {
                        unimplemented!();
                        if (({ classify_5(theta) }) != (anon_1_NaN as ()))
                            && (({ classify_5(theta) }) != (anon_1_inf as ()))
                        {
                            unimplemented!();
                            unimplemented!();
                        }
                    }
                    break 'switch;
                }
                __v if __v == (anon_1_NaN as ()) || __v == (anon_1_non_zero_nan as ()) => {
                    {
                        let mut c: () = ({ classify_4(z.as_pointer()) });
                        unimplemented!();
                    }
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
    ({ test_17() });
    ({ test_18() });
    ({ test_19() });
    ({ test_edges_20() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = testcases_0.with(|_| ());
}
