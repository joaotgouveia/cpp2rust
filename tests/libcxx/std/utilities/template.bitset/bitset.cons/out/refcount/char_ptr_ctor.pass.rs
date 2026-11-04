extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_char_pointer_ctor_0() {
    const _: () = assert!(
        true,
        "!std::is_convertible<const char*, std::bitset<N> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible<std::bitset<N>, const char*>::value"
    );;
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1010101010\0")));
        let v: Value<std_bitset_0_> = Rc::new(RefCell::new(std_bitset_0_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            None,
            None,
            None,
        )));
        let mut M: usize = ({
            let __tmp_1: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_0,) => {
                    if CppLess::less(&__tmp_1.as_pointer(), &(__tmp_0).clone()) {
                        __tmp_1.as_pointer()
                    } else {
                        (__tmp_0).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1010101010\0")));
        let v: Value<std_bitset_0_> = Rc::new(RefCell::new(std_bitset_0_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            None,
            None,
        )));
        let mut M: usize = ({
            let __tmp_3: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_2,) => {
                    if CppLess::less(&__tmp_3.as_pointer(), &(__tmp_2).clone()) {
                        __tmp_3.as_pointer()
                    } else {
                        (__tmp_2).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1a1a1a1a1a\0")));
        let v: Value<std_bitset_0_> = Rc::new(RefCell::new(std_bitset_0_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: usize = ({
            let __tmp_5: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_4,) => {
                    if CppLess::less(&__tmp_5.as_pointer(), &(__tmp_4).clone()) {
                        __tmp_5.as_pointer()
                    } else {
                        (__tmp_4).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"bababababa\0")));
        let v: Value<std_bitset_0_> = Rc::new(RefCell::new(std_bitset_0_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: usize = ({
            let __tmp_7: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_6,) => {
                    if CppLess::less(&__tmp_7.as_pointer(), &(__tmp_6).clone()) {
                        __tmp_7.as_pointer()
                    } else {
                        (__tmp_6).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('b' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
        ])));
        let v: Value<std_bitset_0_> = Rc::new(RefCell::new(std_bitset_0_::new_1(
            { (not_null_terminated.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            None,
            None,
        )));
        let mut M: usize = ({
            let __tmp_9: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_8,) => {
                    if CppLess::less(&__tmp_9.as_pointer(), &(__tmp_8).clone()) {
                        __tmp_9.as_pointer()
                    } else {
                        (__tmp_8).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*not_null_terminated.borrow())
                        [(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(!({ (*v.borrow())[i]() }));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
        ])));
        let v: Value<std_bitset_0_> = Rc::new(RefCell::new(std_bitset_0_::new_1(
            { (not_null_terminated.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: usize = ({
            let __tmp_11: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_10,) => {
                    if CppLess::less(&__tmp_11.as_pointer(), &(__tmp_10).clone()) {
                        __tmp_11.as_pointer()
                    } else {
                        (__tmp_10).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*not_null_terminated.borrow())
                        [(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(!({ (*v.borrow())[i]() }));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
        ])));
        let v: Value<std_bitset_0_> = Rc::new(RefCell::new(std_bitset_0_::new_1(
            { (not_null_terminated.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: usize = ({
            let __tmp_13: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_12,) => {
                    if CppLess::less(&__tmp_13.as_pointer(), &(__tmp_12).clone()) {
                        __tmp_13.as_pointer()
                    } else {
                        (__tmp_12).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*not_null_terminated.borrow())
                        [(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('b' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(!({ (*v.borrow())[i]() }));
            i.prefix_inc();
        }
    }
}
pub fn test_char_pointer_ctor_1() {
    const _: () = assert!(
        true,
        "!std::is_convertible<const char*, std::bitset<N> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible<std::bitset<N>, const char*>::value"
    );;
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1010101010\0")));
        let v: Value<std_bitset_1_> = Rc::new(RefCell::new(std_bitset_1_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            None,
            None,
            None,
        )));
        let mut M: usize = ({
            let __tmp_15: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_14,) => {
                    if CppLess::less(&__tmp_15.as_pointer(), &(__tmp_14).clone()) {
                        __tmp_15.as_pointer()
                    } else {
                        (__tmp_14).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1010101010\0")));
        let v: Value<std_bitset_1_> = Rc::new(RefCell::new(std_bitset_1_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            None,
            None,
        )));
        let mut M: usize = ({
            let __tmp_17: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_16,) => {
                    if CppLess::less(&__tmp_17.as_pointer(), &(__tmp_16).clone()) {
                        __tmp_17.as_pointer()
                    } else {
                        (__tmp_16).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1a1a1a1a1a\0")));
        let v: Value<std_bitset_1_> = Rc::new(RefCell::new(std_bitset_1_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: usize = ({
            let __tmp_19: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_18,) => {
                    if CppLess::less(&__tmp_19.as_pointer(), &(__tmp_18).clone()) {
                        __tmp_19.as_pointer()
                    } else {
                        (__tmp_18).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"bababababa\0")));
        let v: Value<std_bitset_1_> = Rc::new(RefCell::new(std_bitset_1_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: usize = ({
            let __tmp_21: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_20,) => {
                    if CppLess::less(&__tmp_21.as_pointer(), &(__tmp_20).clone()) {
                        __tmp_21.as_pointer()
                    } else {
                        (__tmp_20).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('b' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
        ])));
        let v: Value<std_bitset_1_> = Rc::new(RefCell::new(std_bitset_1_::new_1(
            { (not_null_terminated.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            None,
            None,
        )));
        let mut M: usize = ({
            let __tmp_23: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_22,) => {
                    if CppLess::less(&__tmp_23.as_pointer(), &(__tmp_22).clone()) {
                        __tmp_23.as_pointer()
                    } else {
                        (__tmp_22).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*not_null_terminated.borrow())
                        [(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(!({ (*v.borrow())[i]() }));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
        ])));
        let v: Value<std_bitset_1_> = Rc::new(RefCell::new(std_bitset_1_::new_1(
            { (not_null_terminated.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: usize = ({
            let __tmp_25: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_24,) => {
                    if CppLess::less(&__tmp_25.as_pointer(), &(__tmp_24).clone()) {
                        __tmp_25.as_pointer()
                    } else {
                        (__tmp_24).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*not_null_terminated.borrow())
                        [(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(!({ (*v.borrow())[i]() }));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
        ])));
        let v: Value<std_bitset_1_> = Rc::new(RefCell::new(std_bitset_1_::new_1(
            { (not_null_terminated.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: usize = ({
            let __tmp_27: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_26,) => {
                    if CppLess::less(&__tmp_27.as_pointer(), &(__tmp_26).clone()) {
                        __tmp_27.as_pointer()
                    } else {
                        (__tmp_26).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*not_null_terminated.borrow())
                        [(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('b' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(!({ (*v.borrow())[i]() }));
            i.prefix_inc();
        }
    }
}
pub fn test_char_pointer_ctor_2() {
    const _: () = assert!(
        true,
        "!std::is_convertible<const char*, std::bitset<N> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible<std::bitset<N>, const char*>::value"
    );;
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1010101010\0")));
        let v: Value<std_bitset_31_> = Rc::new(RefCell::new(std_bitset_31_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            None,
            None,
            None,
        )));
        let mut M: usize = ({
            let __tmp_29: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_28,) => {
                    if CppLess::less(&__tmp_29.as_pointer(), &(__tmp_28).clone()) {
                        __tmp_29.as_pointer()
                    } else {
                        (__tmp_28).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1010101010\0")));
        let v: Value<std_bitset_31_> = Rc::new(RefCell::new(std_bitset_31_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            None,
            None,
        )));
        let mut M: usize = ({
            let __tmp_31: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_30,) => {
                    if CppLess::less(&__tmp_31.as_pointer(), &(__tmp_30).clone()) {
                        __tmp_31.as_pointer()
                    } else {
                        (__tmp_30).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1a1a1a1a1a\0")));
        let v: Value<std_bitset_31_> = Rc::new(RefCell::new(std_bitset_31_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: usize = ({
            let __tmp_33: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_32,) => {
                    if CppLess::less(&__tmp_33.as_pointer(), &(__tmp_32).clone()) {
                        __tmp_33.as_pointer()
                    } else {
                        (__tmp_32).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"bababababa\0")));
        let v: Value<std_bitset_31_> = Rc::new(RefCell::new(std_bitset_31_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: usize = ({
            let __tmp_35: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_34,) => {
                    if CppLess::less(&__tmp_35.as_pointer(), &(__tmp_34).clone()) {
                        __tmp_35.as_pointer()
                    } else {
                        (__tmp_34).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('b' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
        ])));
        let v: Value<std_bitset_31_> = Rc::new(RefCell::new(std_bitset_31_::new_1(
            { (not_null_terminated.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            None,
            None,
        )));
        let mut M: usize = ({
            let __tmp_37: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_36,) => {
                    if CppLess::less(&__tmp_37.as_pointer(), &(__tmp_36).clone()) {
                        __tmp_37.as_pointer()
                    } else {
                        (__tmp_36).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*not_null_terminated.borrow())
                        [(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(!({ (*v.borrow())[i]() }));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
        ])));
        let v: Value<std_bitset_31_> = Rc::new(RefCell::new(std_bitset_31_::new_1(
            { (not_null_terminated.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: usize = ({
            let __tmp_39: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_38,) => {
                    if CppLess::less(&__tmp_39.as_pointer(), &(__tmp_38).clone()) {
                        __tmp_39.as_pointer()
                    } else {
                        (__tmp_38).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*not_null_terminated.borrow())
                        [(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(!({ (*v.borrow())[i]() }));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
        ])));
        let v: Value<std_bitset_31_> = Rc::new(RefCell::new(std_bitset_31_::new_1(
            { (not_null_terminated.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: usize = ({
            let __tmp_41: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_40,) => {
                    if CppLess::less(&__tmp_41.as_pointer(), &(__tmp_40).clone()) {
                        __tmp_41.as_pointer()
                    } else {
                        (__tmp_40).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*not_null_terminated.borrow())
                        [(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('b' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(!({ (*v.borrow())[i]() }));
            i.prefix_inc();
        }
    }
}
pub fn test_char_pointer_ctor_3() {
    const _: () = assert!(
        true,
        "!std::is_convertible<const char*, std::bitset<N> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible<std::bitset<N>, const char*>::value"
    );;
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1010101010\0")));
        let v: Value<std_bitset_32_> = Rc::new(RefCell::new(std_bitset_32_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            None,
            None,
            None,
        )));
        let mut M: usize = ({
            let __tmp_43: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_42,) => {
                    if CppLess::less(&__tmp_43.as_pointer(), &(__tmp_42).clone()) {
                        __tmp_43.as_pointer()
                    } else {
                        (__tmp_42).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1010101010\0")));
        let v: Value<std_bitset_32_> = Rc::new(RefCell::new(std_bitset_32_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            None,
            None,
        )));
        let mut M: usize = ({
            let __tmp_45: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_44,) => {
                    if CppLess::less(&__tmp_45.as_pointer(), &(__tmp_44).clone()) {
                        __tmp_45.as_pointer()
                    } else {
                        (__tmp_44).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1a1a1a1a1a\0")));
        let v: Value<std_bitset_32_> = Rc::new(RefCell::new(std_bitset_32_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: usize = ({
            let __tmp_47: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_46,) => {
                    if CppLess::less(&__tmp_47.as_pointer(), &(__tmp_46).clone()) {
                        __tmp_47.as_pointer()
                    } else {
                        (__tmp_46).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"bababababa\0")));
        let v: Value<std_bitset_32_> = Rc::new(RefCell::new(std_bitset_32_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: usize = ({
            let __tmp_49: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_48,) => {
                    if CppLess::less(&__tmp_49.as_pointer(), &(__tmp_48).clone()) {
                        __tmp_49.as_pointer()
                    } else {
                        (__tmp_48).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('b' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
        ])));
        let v: Value<std_bitset_32_> = Rc::new(RefCell::new(std_bitset_32_::new_1(
            { (not_null_terminated.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            None,
            None,
        )));
        let mut M: usize = ({
            let __tmp_51: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_50,) => {
                    if CppLess::less(&__tmp_51.as_pointer(), &(__tmp_50).clone()) {
                        __tmp_51.as_pointer()
                    } else {
                        (__tmp_50).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*not_null_terminated.borrow())
                        [(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(!({ (*v.borrow())[i]() }));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
        ])));
        let v: Value<std_bitset_32_> = Rc::new(RefCell::new(std_bitset_32_::new_1(
            { (not_null_terminated.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: usize = ({
            let __tmp_53: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_52,) => {
                    if CppLess::less(&__tmp_53.as_pointer(), &(__tmp_52).clone()) {
                        __tmp_53.as_pointer()
                    } else {
                        (__tmp_52).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*not_null_terminated.borrow())
                        [(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(!({ (*v.borrow())[i]() }));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
        ])));
        let v: Value<std_bitset_32_> = Rc::new(RefCell::new(std_bitset_32_::new_1(
            { (not_null_terminated.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: usize = ({
            let __tmp_55: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_54,) => {
                    if CppLess::less(&__tmp_55.as_pointer(), &(__tmp_54).clone()) {
                        __tmp_55.as_pointer()
                    } else {
                        (__tmp_54).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*not_null_terminated.borrow())
                        [(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('b' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(!({ (*v.borrow())[i]() }));
            i.prefix_inc();
        }
    }
}
pub fn test_char_pointer_ctor_4() {
    const _: () = assert!(
        true,
        "!std::is_convertible<const char*, std::bitset<N> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible<std::bitset<N>, const char*>::value"
    );;
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1010101010\0")));
        let v: Value<std_bitset_33_> = Rc::new(RefCell::new(std_bitset_33_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            None,
            None,
            None,
        )));
        let mut M: usize = ({
            let __tmp_57: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_56,) => {
                    if CppLess::less(&__tmp_57.as_pointer(), &(__tmp_56).clone()) {
                        __tmp_57.as_pointer()
                    } else {
                        (__tmp_56).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1010101010\0")));
        let v: Value<std_bitset_33_> = Rc::new(RefCell::new(std_bitset_33_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            None,
            None,
        )));
        let mut M: usize = ({
            let __tmp_59: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_58,) => {
                    if CppLess::less(&__tmp_59.as_pointer(), &(__tmp_58).clone()) {
                        __tmp_59.as_pointer()
                    } else {
                        (__tmp_58).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1a1a1a1a1a\0")));
        let v: Value<std_bitset_33_> = Rc::new(RefCell::new(std_bitset_33_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: usize = ({
            let __tmp_61: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_60,) => {
                    if CppLess::less(&__tmp_61.as_pointer(), &(__tmp_60).clone()) {
                        __tmp_61.as_pointer()
                    } else {
                        (__tmp_60).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"bababababa\0")));
        let v: Value<std_bitset_33_> = Rc::new(RefCell::new(std_bitset_33_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: usize = ({
            let __tmp_63: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_62,) => {
                    if CppLess::less(&__tmp_63.as_pointer(), &(__tmp_62).clone()) {
                        __tmp_63.as_pointer()
                    } else {
                        (__tmp_62).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('b' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
        ])));
        let v: Value<std_bitset_33_> = Rc::new(RefCell::new(std_bitset_33_::new_1(
            { (not_null_terminated.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            None,
            None,
        )));
        let mut M: usize = ({
            let __tmp_65: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_64,) => {
                    if CppLess::less(&__tmp_65.as_pointer(), &(__tmp_64).clone()) {
                        __tmp_65.as_pointer()
                    } else {
                        (__tmp_64).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*not_null_terminated.borrow())
                        [(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(!({ (*v.borrow())[i]() }));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
        ])));
        let v: Value<std_bitset_33_> = Rc::new(RefCell::new(std_bitset_33_::new_1(
            { (not_null_terminated.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: usize = ({
            let __tmp_67: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_66,) => {
                    if CppLess::less(&__tmp_67.as_pointer(), &(__tmp_66).clone()) {
                        __tmp_67.as_pointer()
                    } else {
                        (__tmp_66).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*not_null_terminated.borrow())
                        [(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(!({ (*v.borrow())[i]() }));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
        ])));
        let v: Value<std_bitset_33_> = Rc::new(RefCell::new(std_bitset_33_::new_1(
            { (not_null_terminated.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: usize = ({
            let __tmp_69: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_68,) => {
                    if CppLess::less(&__tmp_69.as_pointer(), &(__tmp_68).clone()) {
                        __tmp_69.as_pointer()
                    } else {
                        (__tmp_68).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*not_null_terminated.borrow())
                        [(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('b' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(!({ (*v.borrow())[i]() }));
            i.prefix_inc();
        }
    }
}
pub fn test_char_pointer_ctor_5() {
    const _: () = assert!(
        true,
        "!std::is_convertible<const char*, std::bitset<N> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible<std::bitset<N>, const char*>::value"
    );;
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1010101010\0")));
        let v: Value<std_bitset_63_> = Rc::new(RefCell::new(std_bitset_63_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            None,
            None,
            None,
        )));
        let mut M: usize = ({
            let __tmp_71: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_70,) => {
                    if CppLess::less(&__tmp_71.as_pointer(), &(__tmp_70).clone()) {
                        __tmp_71.as_pointer()
                    } else {
                        (__tmp_70).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1010101010\0")));
        let v: Value<std_bitset_63_> = Rc::new(RefCell::new(std_bitset_63_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            None,
            None,
        )));
        let mut M: usize = ({
            let __tmp_73: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_72,) => {
                    if CppLess::less(&__tmp_73.as_pointer(), &(__tmp_72).clone()) {
                        __tmp_73.as_pointer()
                    } else {
                        (__tmp_72).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1a1a1a1a1a\0")));
        let v: Value<std_bitset_63_> = Rc::new(RefCell::new(std_bitset_63_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: usize = ({
            let __tmp_75: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_74,) => {
                    if CppLess::less(&__tmp_75.as_pointer(), &(__tmp_74).clone()) {
                        __tmp_75.as_pointer()
                    } else {
                        (__tmp_74).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"bababababa\0")));
        let v: Value<std_bitset_63_> = Rc::new(RefCell::new(std_bitset_63_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: usize = ({
            let __tmp_77: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_76,) => {
                    if CppLess::less(&__tmp_77.as_pointer(), &(__tmp_76).clone()) {
                        __tmp_77.as_pointer()
                    } else {
                        (__tmp_76).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('b' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
        ])));
        let v: Value<std_bitset_63_> = Rc::new(RefCell::new(std_bitset_63_::new_1(
            { (not_null_terminated.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            None,
            None,
        )));
        let mut M: usize = ({
            let __tmp_79: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_78,) => {
                    if CppLess::less(&__tmp_79.as_pointer(), &(__tmp_78).clone()) {
                        __tmp_79.as_pointer()
                    } else {
                        (__tmp_78).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*not_null_terminated.borrow())
                        [(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(!({ (*v.borrow())[i]() }));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
        ])));
        let v: Value<std_bitset_63_> = Rc::new(RefCell::new(std_bitset_63_::new_1(
            { (not_null_terminated.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: usize = ({
            let __tmp_81: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_80,) => {
                    if CppLess::less(&__tmp_81.as_pointer(), &(__tmp_80).clone()) {
                        __tmp_81.as_pointer()
                    } else {
                        (__tmp_80).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*not_null_terminated.borrow())
                        [(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(!({ (*v.borrow())[i]() }));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
        ])));
        let v: Value<std_bitset_63_> = Rc::new(RefCell::new(std_bitset_63_::new_1(
            { (not_null_terminated.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: usize = ({
            let __tmp_83: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_82,) => {
                    if CppLess::less(&__tmp_83.as_pointer(), &(__tmp_82).clone()) {
                        __tmp_83.as_pointer()
                    } else {
                        (__tmp_82).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*not_null_terminated.borrow())
                        [(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('b' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(!({ (*v.borrow())[i]() }));
            i.prefix_inc();
        }
    }
}
pub fn test_char_pointer_ctor_6() {
    const _: () = assert!(
        true,
        "!std::is_convertible<const char*, std::bitset<N> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible<std::bitset<N>, const char*>::value"
    );;
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1010101010\0")));
        let v: Value<std_bitset_64_> = Rc::new(RefCell::new(std_bitset_64_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            None,
            None,
            None,
        )));
        let mut M: usize = ({
            let __tmp_85: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_84,) => {
                    if CppLess::less(&__tmp_85.as_pointer(), &(__tmp_84).clone()) {
                        __tmp_85.as_pointer()
                    } else {
                        (__tmp_84).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1010101010\0")));
        let v: Value<std_bitset_64_> = Rc::new(RefCell::new(std_bitset_64_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            None,
            None,
        )));
        let mut M: usize = ({
            let __tmp_87: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_86,) => {
                    if CppLess::less(&__tmp_87.as_pointer(), &(__tmp_86).clone()) {
                        __tmp_87.as_pointer()
                    } else {
                        (__tmp_86).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1a1a1a1a1a\0")));
        let v: Value<std_bitset_64_> = Rc::new(RefCell::new(std_bitset_64_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: usize = ({
            let __tmp_89: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_88,) => {
                    if CppLess::less(&__tmp_89.as_pointer(), &(__tmp_88).clone()) {
                        __tmp_89.as_pointer()
                    } else {
                        (__tmp_88).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"bababababa\0")));
        let v: Value<std_bitset_64_> = Rc::new(RefCell::new(std_bitset_64_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: usize = ({
            let __tmp_91: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_90,) => {
                    if CppLess::less(&__tmp_91.as_pointer(), &(__tmp_90).clone()) {
                        __tmp_91.as_pointer()
                    } else {
                        (__tmp_90).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('b' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
        ])));
        let v: Value<std_bitset_64_> = Rc::new(RefCell::new(std_bitset_64_::new_1(
            { (not_null_terminated.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            None,
            None,
        )));
        let mut M: usize = ({
            let __tmp_93: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_92,) => {
                    if CppLess::less(&__tmp_93.as_pointer(), &(__tmp_92).clone()) {
                        __tmp_93.as_pointer()
                    } else {
                        (__tmp_92).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*not_null_terminated.borrow())
                        [(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(!({ (*v.borrow())[i]() }));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
        ])));
        let v: Value<std_bitset_64_> = Rc::new(RefCell::new(std_bitset_64_::new_1(
            { (not_null_terminated.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: usize = ({
            let __tmp_95: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_94,) => {
                    if CppLess::less(&__tmp_95.as_pointer(), &(__tmp_94).clone()) {
                        __tmp_95.as_pointer()
                    } else {
                        (__tmp_94).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*not_null_terminated.borrow())
                        [(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(!({ (*v.borrow())[i]() }));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
        ])));
        let v: Value<std_bitset_64_> = Rc::new(RefCell::new(std_bitset_64_::new_1(
            { (not_null_terminated.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: usize = ({
            let __tmp_97: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_96,) => {
                    if CppLess::less(&__tmp_97.as_pointer(), &(__tmp_96).clone()) {
                        __tmp_97.as_pointer()
                    } else {
                        (__tmp_96).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*not_null_terminated.borrow())
                        [(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('b' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(!({ (*v.borrow())[i]() }));
            i.prefix_inc();
        }
    }
}
pub fn test_char_pointer_ctor_7() {
    const _: () = assert!(
        true,
        "!std::is_convertible<const char*, std::bitset<N> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible<std::bitset<N>, const char*>::value"
    );;
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1010101010\0")));
        let v: Value<std_bitset_65_> = Rc::new(RefCell::new(std_bitset_65_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            None,
            None,
            None,
        )));
        let mut M: usize = ({
            let __tmp_99: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_98,) => {
                    if CppLess::less(&__tmp_99.as_pointer(), &(__tmp_98).clone()) {
                        __tmp_99.as_pointer()
                    } else {
                        (__tmp_98).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1010101010\0")));
        let v: Value<std_bitset_65_> = Rc::new(RefCell::new(std_bitset_65_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            None,
            None,
        )));
        let mut M: usize = ({
            let __tmp_101: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_100,) => {
                    if CppLess::less(&__tmp_101.as_pointer(), &(__tmp_100).clone()) {
                        __tmp_101.as_pointer()
                    } else {
                        (__tmp_100).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1a1a1a1a1a\0")));
        let v: Value<std_bitset_65_> = Rc::new(RefCell::new(std_bitset_65_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: usize = ({
            let __tmp_103: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_102,) => {
                    if CppLess::less(&__tmp_103.as_pointer(), &(__tmp_102).clone()) {
                        __tmp_103.as_pointer()
                    } else {
                        (__tmp_102).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"bababababa\0")));
        let v: Value<std_bitset_65_> = Rc::new(RefCell::new(std_bitset_65_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: usize = ({
            let __tmp_105: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_104,) => {
                    if CppLess::less(&__tmp_105.as_pointer(), &(__tmp_104).clone()) {
                        __tmp_105.as_pointer()
                    } else {
                        (__tmp_104).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('b' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
        ])));
        let v: Value<std_bitset_65_> = Rc::new(RefCell::new(std_bitset_65_::new_1(
            { (not_null_terminated.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            None,
            None,
        )));
        let mut M: usize = ({
            let __tmp_107: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_106,) => {
                    if CppLess::less(&__tmp_107.as_pointer(), &(__tmp_106).clone()) {
                        __tmp_107.as_pointer()
                    } else {
                        (__tmp_106).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*not_null_terminated.borrow())
                        [(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(!({ (*v.borrow())[i]() }));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
        ])));
        let v: Value<std_bitset_65_> = Rc::new(RefCell::new(std_bitset_65_::new_1(
            { (not_null_terminated.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: usize = ({
            let __tmp_109: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_108,) => {
                    if CppLess::less(&__tmp_109.as_pointer(), &(__tmp_108).clone()) {
                        __tmp_109.as_pointer()
                    } else {
                        (__tmp_108).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*not_null_terminated.borrow())
                        [(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(!({ (*v.borrow())[i]() }));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
        ])));
        let v: Value<std_bitset_65_> = Rc::new(RefCell::new(std_bitset_65_::new_1(
            { (not_null_terminated.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: usize = ({
            let __tmp_111: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_110,) => {
                    if CppLess::less(&__tmp_111.as_pointer(), &(__tmp_110).clone()) {
                        __tmp_111.as_pointer()
                    } else {
                        (__tmp_110).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*not_null_terminated.borrow())
                        [(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('b' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(!({ (*v.borrow())[i]() }));
            i.prefix_inc();
        }
    }
}
pub fn test_char_pointer_ctor_8() {
    const _: () = assert!(
        true,
        "!std::is_convertible<const char*, std::bitset<N> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible<std::bitset<N>, const char*>::value"
    );;
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1010101010\0")));
        let v: Value<std_bitset_1000_> = Rc::new(RefCell::new(std_bitset_1000_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            None,
            None,
            None,
        )));
        let mut M: usize = ({
            let __tmp_113: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_112,) => {
                    if CppLess::less(&__tmp_113.as_pointer(), &(__tmp_112).clone()) {
                        __tmp_113.as_pointer()
                    } else {
                        (__tmp_112).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1010101010\0")));
        let v: Value<std_bitset_1000_> = Rc::new(RefCell::new(std_bitset_1000_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            None,
            None,
        )));
        let mut M: usize = ({
            let __tmp_115: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_114,) => {
                    if CppLess::less(&__tmp_115.as_pointer(), &(__tmp_114).clone()) {
                        __tmp_115.as_pointer()
                    } else {
                        (__tmp_114).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1a1a1a1a1a\0")));
        let v: Value<std_bitset_1000_> = Rc::new(RefCell::new(std_bitset_1000_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: usize = ({
            let __tmp_117: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_116,) => {
                    if CppLess::less(&__tmp_117.as_pointer(), &(__tmp_116).clone()) {
                        __tmp_117.as_pointer()
                    } else {
                        (__tmp_116).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"bababababa\0")));
        let v: Value<std_bitset_1000_> = Rc::new(RefCell::new(std_bitset_1000_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: usize = ({
            let __tmp_119: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_118,) => {
                    if CppLess::less(&__tmp_119.as_pointer(), &(__tmp_118).clone()) {
                        __tmp_119.as_pointer()
                    } else {
                        (__tmp_118).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*s.borrow())[(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('b' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(((({ (*v.borrow())[i]() }) as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
            ('1' as i8),
            ('0' as i8),
        ])));
        let v: Value<std_bitset_1000_> = Rc::new(RefCell::new(std_bitset_1000_::new_1(
            { (not_null_terminated.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            None,
            None,
        )));
        let mut M: usize = ({
            let __tmp_121: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_120,) => {
                    if CppLess::less(&__tmp_121.as_pointer(), &(__tmp_120).clone()) {
                        __tmp_121.as_pointer()
                    } else {
                        (__tmp_120).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*not_null_terminated.borrow())
                        [(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(!({ (*v.borrow())[i]() }));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
            ('1' as i8),
            ('a' as i8),
        ])));
        let v: Value<std_bitset_1000_> = Rc::new(RefCell::new(std_bitset_1000_::new_1(
            { (not_null_terminated.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: usize = ({
            let __tmp_123: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_122,) => {
                    if CppLess::less(&__tmp_123.as_pointer(), &(__tmp_122).clone()) {
                        __tmp_123.as_pointer()
                    } else {
                        (__tmp_122).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*not_null_terminated.borrow())
                        [(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('1' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(!({ (*v.borrow())[i]() }));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
            ('b' as i8),
            ('a' as i8),
        ])));
        let v: Value<std_bitset_1000_> = Rc::new(RefCell::new(std_bitset_1000_::new_1(
            { (not_null_terminated.as_pointer() as Ptr<i8>) },
            { Some(10_usize) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: usize = ({
            let __tmp_125: Value<u64> = Rc::new(RefCell::new(10_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_124,) => {
                    if CppLess::less(&__tmp_125.as_pointer(), &(__tmp_124).clone()) {
                        __tmp_125.as_pointer()
                    } else {
                        (__tmp_124).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(
                ((({ (*v.borrow())[i]() }) as i32)
                    == ((((*not_null_terminated.borrow())
                        [(((M).wrapping_sub(1_usize)).wrapping_sub(i)) as usize]
                        as i32)
                        == (('b' as i8) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(!({ (*v.borrow())[i]() }));
            i.prefix_inc();
        }
    }
}
pub fn test_9() -> bool {
    ({ test_char_pointer_ctor_0() });
    ({ test_char_pointer_ctor_1() });
    ({ test_char_pointer_ctor_2() });
    ({ test_char_pointer_ctor_3() });
    ({ test_char_pointer_ctor_4() });
    ({ test_char_pointer_ctor_5() });
    ({ test_char_pointer_ctor_6() });
    ({ test_char_pointer_ctor_7() });
    ({ test_char_pointer_ctor_8() });
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_9() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
