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
        let sb: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == 0_()));
        assert!((unimplemented!() == 0_()));
        assert!((unimplemented!() == 0_()));
    }
    {
        let sb: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == 0_()));
        assert!((unimplemented!() == 0_()));
        assert!((unimplemented!() == 0_()));
    }
    {
        let sb: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == 3_()));
        assert!((unimplemented!() == (('3' as i8) as i32)));
        assert!((unimplemented!() == 6_()));
        assert!((unimplemented!() == (('6' as i8) as i32)));
        assert!((unimplemented!() == 7_()));
        assert!((unimplemented!() == (('7' as i8) as i32)));
    }
    {
        let sb: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == 3_()));
        assert!((unimplemented!() == (('a' as i8) as i32)));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_0,) => {
                    Ptr::<i8>::from_string_literal(b"012a456789").with_c_str(|__s| {
                        (*__tmp_0.upgrade().deref())
                            [..(*__tmp_0.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!((unimplemented!() == 7_()));
        assert!((unimplemented!() == (('b' as i8) as i32)));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_1,) => {
                    Ptr::<i8>::from_string_literal(b"012a456b89").with_c_str(|__s| {
                        (*__tmp_1.upgrade().deref())
                            [..(*__tmp_1.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!((unimplemented!() == 7_()));
        assert!((unimplemented!() == (('c' as i8) as i32)));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_2,) => {
                    Ptr::<i8>::from_string_literal(b"012a456c89").with_c_str(|__s| {
                        (*__tmp_2.upgrade().deref())
                            [..(*__tmp_2.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let sb: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == 3_()));
        assert!((unimplemented!() == (('3' as i8) as i32)));
        assert!((unimplemented!() == 6_()));
        assert!((unimplemented!() == (('6' as i8) as i32)));
        assert!((unimplemented!() == 7_()));
        assert!((unimplemented!() == (('7' as i8) as i32)));
        assert!((unimplemented!() == 3_()));
        assert!((unimplemented!() == (('3' as i8) as i32)));
        assert!((unimplemented!() == (('a' as i8) as i32)));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_3,) => {
                    Ptr::<i8>::from_string_literal(b"012a456789").with_c_str(|__s| {
                        (*__tmp_3.upgrade().deref())
                            [..(*__tmp_3.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == 7_()));
        assert!((unimplemented!() == (('7' as i8) as i32)));
        assert!((unimplemented!() == (('c' as i8) as i32)));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_4,) => {
                    Ptr::<i8>::from_string_literal(b"012a456c89").with_c_str(|__s| {
                        (*__tmp_4.upgrade().deref())
                            [..(*__tmp_4.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!((unimplemented!() == 3_()));
        assert!((unimplemented!() == (('3' as i8) as i32)));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_5,) => {
                    Ptr::<i8>::from_string_literal(b"0123456c89").with_c_str(|__s| {
                        (*__tmp_5.upgrade().deref())
                            [..(*__tmp_5.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!((unimplemented!() == 7_()));
        assert!((unimplemented!() == (('7' as i8) as i32)));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_6,) => {
                    Ptr::<i8>::from_string_literal(b"0123456789").with_c_str(|__s| {
                        (*__tmp_6.upgrade().deref())
                            [..(*__tmp_6.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!((unimplemented!() == 7_()));
        assert!((unimplemented!() == (('c' as i8) as i32)));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_7,) => {
                    Ptr::<i8>::from_string_literal(b"0123456c89").with_c_str(|__s| {
                        (*__tmp_7.upgrade().deref())
                            [..(*__tmp_7.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let sb: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == 3_()));
        assert!((unimplemented!() == ((51 as i32) as ())));
        assert!((unimplemented!() == 6_()));
        assert!((unimplemented!() == ((54 as i32) as ())));
        assert!((unimplemented!() == 7_()));
        assert!((unimplemented!() == ((55 as i32) as ())));
    }
    {
        let sb: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == 3_()));
        assert!((unimplemented!() == ((97 as i32) as ())));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_8,) => {
                    Ptr::<i32>::from_string_literal(&[
                        48 as i32, 49 as i32, 50 as i32, 97 as i32, 52 as i32, 53 as i32,
                        54 as i32, 55 as i32, 56 as i32, 57 as i32, 0 as i32,
                    ])
                    .with_c_str(|__s| {
                        (*__tmp_8.upgrade().deref())
                            [..(*__tmp_8.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!((unimplemented!() == 7_()));
        assert!((unimplemented!() == ((98 as i32) as ())));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_9,) => {
                    Ptr::<i32>::from_string_literal(&[
                        48 as i32, 49 as i32, 50 as i32, 97 as i32, 52 as i32, 53 as i32,
                        54 as i32, 98 as i32, 56 as i32, 57 as i32, 0 as i32,
                    ])
                    .with_c_str(|__s| {
                        (*__tmp_9.upgrade().deref())
                            [..(*__tmp_9.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!((unimplemented!() == 7_()));
        assert!((unimplemented!() == ((99 as i32) as ())));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_10,) => {
                    Ptr::<i32>::from_string_literal(&[
                        48 as i32, 49 as i32, 50 as i32, 97 as i32, 52 as i32, 53 as i32,
                        54 as i32, 99 as i32, 56 as i32, 57 as i32, 0 as i32,
                    ])
                    .with_c_str(|__s| {
                        (*__tmp_10.upgrade().deref())
                            [..(*__tmp_10.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let sb: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == 3_()));
        assert!((unimplemented!() == ((51 as i32) as ())));
        assert!((unimplemented!() == 6_()));
        assert!((unimplemented!() == ((54 as i32) as ())));
        assert!((unimplemented!() == 7_()));
        assert!((unimplemented!() == ((55 as i32) as ())));
        assert!((unimplemented!() == 3_()));
        assert!((unimplemented!() == ((51 as i32) as ())));
        assert!((unimplemented!() == ((97 as i32) as ())));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_11,) => {
                    Ptr::<i32>::from_string_literal(&[
                        48 as i32, 49 as i32, 50 as i32, 97 as i32, 52 as i32, 53 as i32,
                        54 as i32, 55 as i32, 56 as i32, 57 as i32, 0 as i32,
                    ])
                    .with_c_str(|__s| {
                        (*__tmp_11.upgrade().deref())
                            [..(*__tmp_11.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!((unimplemented!() == (-1_i32 as ())));
        assert!((unimplemented!() == 7_()));
        assert!((unimplemented!() == ((55 as i32) as ())));
        assert!((unimplemented!() == ((99 as i32) as ())));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_12,) => {
                    Ptr::<i32>::from_string_literal(&[
                        48 as i32, 49 as i32, 50 as i32, 97 as i32, 52 as i32, 53 as i32,
                        54 as i32, 99 as i32, 56 as i32, 57 as i32, 0 as i32,
                    ])
                    .with_c_str(|__s| {
                        (*__tmp_12.upgrade().deref())
                            [..(*__tmp_12.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!((unimplemented!() == 3_()));
        assert!((unimplemented!() == ((51 as i32) as ())));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_13,) => {
                    Ptr::<i32>::from_string_literal(&[
                        48 as i32, 49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32,
                        54 as i32, 99 as i32, 56 as i32, 57 as i32, 0 as i32,
                    ])
                    .with_c_str(|__s| {
                        (*__tmp_13.upgrade().deref())
                            [..(*__tmp_13.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!((unimplemented!() == 7_()));
        assert!((unimplemented!() == ((55 as i32) as ())));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_14,) => {
                    Ptr::<i32>::from_string_literal(&[
                        48 as i32, 49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32,
                        54 as i32, 55 as i32, 56 as i32, 57 as i32, 0 as i32,
                    ])
                    .with_c_str(|__s| {
                        (*__tmp_14.upgrade().deref())
                            [..(*__tmp_14.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!((unimplemented!() == 7_()));
        assert!((unimplemented!() == ((99 as i32) as ())));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_15,) => {
                    Ptr::<i32>::from_string_literal(&[
                        48 as i32, 49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32,
                        54 as i32, 99 as i32, 56 as i32, 57 as i32, 0 as i32,
                    ])
                    .with_c_str(|__s| {
                        (*__tmp_15.upgrade().deref())
                            [..(*__tmp_15.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
