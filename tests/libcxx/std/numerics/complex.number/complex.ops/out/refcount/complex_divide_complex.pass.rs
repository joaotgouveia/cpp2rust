extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn is_close_0(mut v: (), mut comp: ()) -> () {
    return (v <= (comp + unimplemented!())) && (v >= (comp - unimplemented!()));
}
pub fn is_close_1(mut v: (), mut comp: ()) -> () {
    return (v <= (comp + unimplemented!())) && (v >= (comp - unimplemented!()));
}
pub fn is_close_2(mut v: (), mut comp: ()) -> () {
    return (v <= (comp + unimplemented!())) && (v >= (comp - unimplemented!()));
}
thread_local!(
    pub static testcases_3: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
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
pub type anon_4 = ();
pub const anon_4_zero: anon_4 = 0;
pub const anon_4_non_zero: anon_4 = 1;
pub const anon_4_inf: anon_4 = 2;
pub const anon_4_NaN: anon_4 = 3;
pub const anon_4_non_zero_nan: anon_4 = 4;
pub fn test_isinf_5(mut v: ()) -> () {
    return (v == unimplemented!()) || (v == -unimplemented!());
}
pub fn test_isnan_6(mut v: ()) -> () {
    return (v != v);
}
pub fn classify_7(x: Ptr<()>) -> () {
    if unimplemented!() {
        return (anon_4_zero as ());
    }
    if ({ test_isinf_5(unimplemented!()) }) || ({ test_isinf_5(unimplemented!()) }) {
        return (anon_4_inf as ());
    }
    if ({ test_isnan_6(unimplemented!()) }) && ({ test_isnan_6(unimplemented!()) }) {
        return (anon_4_NaN as ());
    }
    if ({ test_isnan_6(unimplemented!()) }) {
        if (unimplemented!() == 0_()) {
            return (anon_4_NaN as ());
        }
        return (anon_4_non_zero_nan as ());
    }
    if ({ test_isnan_6(unimplemented!()) }) {
        if (unimplemented!() == 0_()) {
            return (anon_4_NaN as ());
        }
        return (anon_4_non_zero_nan as ());
    }
    return (anon_4_non_zero as ());
}
pub fn classify_8(mut x: ()) -> () {
    if (x == 0_()) {
        return (anon_4_zero as ());
    }
    if ({ isinf_9(x) }) {
        return (anon_4_inf as ());
    }
    if ({ isnan_10(x) }) {
        return (anon_4_NaN as ());
    }
    return (anon_4_non_zero as ());
}
pub fn is_about_11(mut x: (), mut y: ()) {
    unimplemented!();
}
pub fn is_about_12(mut x: (), mut y: ()) {
    unimplemented!();
}
pub fn is_about_13(mut x: (), mut y: ()) {
    unimplemented!();
}
pub fn test_14() -> () {
    let lhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_15() -> () {
    let lhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_16() -> () {
    let lhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_edges_17() -> () {
    let N: Value<()> = Rc::new(RefCell::new(
        (2448usize as ()).wrapping_div((16usize as ())),
    ));
    let mut classification: [(); 153] = [unimplemented!(); 153];
    let mut i: () = 0_();
    'loop_: while (i < (*N.borrow())) {
        let __rhs = ({ classify_7((testcases_3.with(|v| v.as_pointer()) as Ptr<()>).offset(i)) });
        classification[(i) as usize] = __rhs;
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i < (*N.borrow())) {
        let mut j: () = 0_();
        'loop_: while (j < (*N.borrow())) {
            let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            'switch: {
                match { classification[(i) as usize] } {
                    __v if __v == (anon_4_zero as ()) => {
                        'switch: {
                            match { classification[(j) as usize] } {
                                __v if __v == (anon_4_zero as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_non_zero as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_inf as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_NaN as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_non_zero_nan as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                _ => {}
                            }
                        };
                        break 'switch;
                    }
                    __v if __v == (anon_4_non_zero as ()) => {
                        'switch: {
                            match { classification[(j) as usize] } {
                                __v if __v == (anon_4_zero as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_non_zero as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_inf as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_NaN as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_non_zero_nan as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                _ => {}
                            }
                        };
                        break 'switch;
                    }
                    __v if __v == (anon_4_inf as ()) => {
                        'switch: {
                            match { classification[(j) as usize] } {
                                __v if __v == (anon_4_zero as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_non_zero as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_inf as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_NaN as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_non_zero_nan as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                _ => {}
                            }
                        };
                        break 'switch;
                    }
                    __v if __v == (anon_4_NaN as ()) => {
                        'switch: {
                            match { classification[(j) as usize] } {
                                __v if __v == (anon_4_zero as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_non_zero as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_inf as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_NaN as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_non_zero_nan as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                _ => {}
                            }
                        };
                        break 'switch;
                    }
                    __v if __v == (anon_4_non_zero_nan as ()) => {
                        'switch: {
                            match { classification[(j) as usize] } {
                                __v if __v == (anon_4_zero as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_non_zero as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_inf as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_NaN as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_non_zero_nan as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                _ => {}
                            }
                        };
                        break 'switch;
                    }
                    _ => {}
                }
            };
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    return true;
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
    ({ test_14() });
    ({ test_15() });
    ({ test_16() });
    ({ test_edges_17() });
    const _: () = assert!(true, "test<float>()");;
    const _: () = assert!(true, "test<double>()");;
    const _: () = assert!(true, "test<long double>()");;
    const _: () = assert!(true, "test_edges()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = testcases_3.with(|_| ());
}
