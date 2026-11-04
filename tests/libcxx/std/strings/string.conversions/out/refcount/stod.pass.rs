extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};

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
    {
        assert!(
            (({
                let ___str: Value<Vec<i8>> = Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"0").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                }));
                stod_0(___str.as_pointer(), None)
            }) == 0_f64)
        );
        assert!(
            (({
                let ___str: Value<Vec<i8>> = Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"-0").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                }));
                stod_0(___str.as_pointer(), None)
            }) == 0_f64)
        );
        assert!(
            (({
                let ___str: Value<Vec<i8>> = Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"-10").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                }));
                stod_0(___str.as_pointer(), None)
            }) == (-10_i32 as f64))
        );
        assert!(
            (({
                let ___str: Value<Vec<i8>> = Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b" 10").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                }));
                stod_0(___str.as_pointer(), None)
            }) == 10_f64)
        );
        {
            let idx: Value<usize> = Rc::new(RefCell::new(0_usize));
            assert!(
                (({
                    let ___str: Value<Vec<i8>> = Rc::new(RefCell::new({
                        let mut __bytes = Ptr::<i8>::from_string_literal(b"10g").to_c_bytes();
                        __bytes.push(0_i8);
                        __bytes
                    }));
                    stod_0(___str.as_pointer(), Some((idx.as_pointer())))
                }) == 10_f64)
            );
            assert!(({ ((*idx.borrow()) == 2_usize) }));
        }
        {
            let idx: Value<usize> = Rc::new(RefCell::new(0_usize));
            assert!(
                (({
                    let ___str: Value<Vec<i8>> = Rc::new(RefCell::new({
                        let mut __bytes = Ptr::<i8>::from_string_literal(b"1.e60").to_c_bytes();
                        __bytes.push(0_i8);
                        __bytes
                    }));
                    stod_0(___str.as_pointer(), Some((idx.as_pointer())))
                }) == 1e+60_f64)
            );
            assert!(({ ((*idx.borrow()) == 5_usize) }));
        }
        {
            let idx: Value<usize> = Rc::new(RefCell::new(0_usize));
            assert!(
                (({
                    let ___str: Value<Vec<i8>> = Rc::new(RefCell::new({
                        let mut __bytes = Ptr::<i8>::from_string_literal(b"INF").to_c_bytes();
                        __bytes.push(0_i8);
                        __bytes
                    }));
                    stod_0(___str.as_pointer(), Some((idx.as_pointer())))
                }) == ((f32::INFINITY) as f64))
            );
            assert!(({ ((*idx.borrow()) == 3_usize) }));
        }
        {
            let idx: Value<usize> = Rc::new(RefCell::new(0_usize));
            assert!(
                ({
                    isnan_1(
                        ({
                            let ___str: Value<Vec<i8>> = Rc::new(RefCell::new({
                                let mut __bytes =
                                    Ptr::<i8>::from_string_literal(b"NAN").to_c_bytes();
                                __bytes.push(0_i8);
                                __bytes
                            }));
                            stod_0(___str.as_pointer(), Some((idx.as_pointer())))
                        }),
                    )
                })
            );
            assert!(({ ((*idx.borrow()) == 3_usize) }));
        }
    }
    {
        assert!(
            (({
                let ___str: Value<Vec<i32>> = Rc::new(RefCell::new({
                    let mut __bytes =
                        Ptr::<i32>::from_string_literal(&[48 as i32, 0 as i32]).to_c_bytes();
                    __bytes.push(0_i32);
                    __bytes
                }));
                stod_2(___str.as_pointer(), None)
            }) == 0_f64)
        );
        assert!(
            (({
                let ___str: Value<Vec<i32>> = Rc::new(RefCell::new({
                    let mut __bytes =
                        Ptr::<i32>::from_string_literal(&[45 as i32, 48 as i32, 0 as i32])
                            .to_c_bytes();
                    __bytes.push(0_i32);
                    __bytes
                }));
                stod_2(___str.as_pointer(), None)
            }) == 0_f64)
        );
        assert!(
            (({
                let ___str: Value<Vec<i32>> = Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i32>::from_string_literal(&[
                        45 as i32, 49 as i32, 48 as i32, 46 as i32, 53 as i32, 0 as i32,
                    ])
                    .to_c_bytes();
                    __bytes.push(0_i32);
                    __bytes
                }));
                stod_2(___str.as_pointer(), None)
            }) == -10.5_f64)
        );
        assert!(
            (({
                let ___str: Value<Vec<i32>> = Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i32>::from_string_literal(&[
                        32 as i32, 49 as i32, 48 as i32, 0 as i32,
                    ])
                    .to_c_bytes();
                    __bytes.push(0_i32);
                    __bytes
                }));
                stod_2(___str.as_pointer(), None)
            }) == 10_f64)
        );
        {
            let idx: Value<usize> = Rc::new(RefCell::new(0_usize));
            assert!(
                (({
                    let ___str: Value<Vec<i32>> = Rc::new(RefCell::new({
                        let mut __bytes = Ptr::<i32>::from_string_literal(&[
                            49 as i32, 48 as i32, 103 as i32, 0 as i32,
                        ])
                        .to_c_bytes();
                        __bytes.push(0_i32);
                        __bytes
                    }));
                    stod_2(___str.as_pointer(), Some((idx.as_pointer())))
                }) == 10_f64)
            );
            assert!(({ ((*idx.borrow()) == 2_usize) }));
        }
        {
            let idx: Value<usize> = Rc::new(RefCell::new(0_usize));
            assert!(
                (({
                    let ___str: Value<Vec<i32>> = Rc::new(RefCell::new({
                        let mut __bytes = Ptr::<i32>::from_string_literal(&[
                            49 as i32, 46 as i32, 101 as i32, 54 as i32, 48 as i32, 0 as i32,
                        ])
                        .to_c_bytes();
                        __bytes.push(0_i32);
                        __bytes
                    }));
                    stod_2(___str.as_pointer(), Some((idx.as_pointer())))
                }) == 1e+60_f64)
            );
            assert!(({ ((*idx.borrow()) == 5_usize) }));
        }
        {
            let idx: Value<usize> = Rc::new(RefCell::new(0_usize));
            assert!(
                (({
                    let ___str: Value<Vec<i32>> = Rc::new(RefCell::new({
                        let mut __bytes = Ptr::<i32>::from_string_literal(&[
                            73 as i32, 78 as i32, 70 as i32, 0 as i32,
                        ])
                        .to_c_bytes();
                        __bytes.push(0_i32);
                        __bytes
                    }));
                    stod_2(___str.as_pointer(), Some((idx.as_pointer())))
                }) == ((f32::INFINITY) as f64))
            );
            assert!(({ ((*idx.borrow()) == 3_usize) }));
        }
        {
            let idx: Value<usize> = Rc::new(RefCell::new(0_usize));
            assert!(
                ({
                    isnan_1(
                        ({
                            let ___str: Value<Vec<i32>> = Rc::new(RefCell::new({
                                let mut __bytes = Ptr::<i32>::from_string_literal(&[
                                    78 as i32, 65 as i32, 78 as i32, 0 as i32,
                                ])
                                .to_c_bytes();
                                __bytes.push(0_i32);
                                __bytes
                            }));
                            stod_2(___str.as_pointer(), Some((idx.as_pointer())))
                        }),
                    )
                })
            );
            assert!(({ ((*idx.borrow()) == 3_usize) }));
        }
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
