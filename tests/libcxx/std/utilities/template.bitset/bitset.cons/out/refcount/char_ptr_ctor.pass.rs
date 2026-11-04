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
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1010101010\0")));
        let v: Value<std_bitset_0_> = Rc::new(RefCell::new(std_bitset_0_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_u64) },
            None,
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1a1a1a1a1a\0")));
        let v: Value<std_bitset_0_> = Rc::new(RefCell::new(std_bitset_0_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_u64) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"bababababa\0")));
        let v: Value<std_bitset_0_> = Rc::new(RefCell::new(std_bitset_0_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_u64) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('b' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
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
            { Some(10_u64) },
            None,
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*not_null_terminated.borrow())
                            [(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(!({ (*v.borrow())[i]() })) });
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
            { Some(10_u64) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*not_null_terminated.borrow())
                            [(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(!({ (*v.borrow())[i]() })) });
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
            { Some(10_u64) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*not_null_terminated.borrow())
                            [(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('b' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(!({ (*v.borrow())[i]() })) });
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
        let v: Value<std_bitset_1_> = Rc::new(RefCell::new(std_bitset_1_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            None,
            None,
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1010101010\0")));
        let v: Value<std_bitset_1_> = Rc::new(RefCell::new(std_bitset_1_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_u64) },
            None,
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1a1a1a1a1a\0")));
        let v: Value<std_bitset_1_> = Rc::new(RefCell::new(std_bitset_1_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_u64) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"bababababa\0")));
        let v: Value<std_bitset_1_> = Rc::new(RefCell::new(std_bitset_1_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_u64) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('b' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
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
            { Some(10_u64) },
            None,
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*not_null_terminated.borrow())
                            [(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(!({ (*v.borrow())[i]() })) });
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
            { Some(10_u64) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*not_null_terminated.borrow())
                            [(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(!({ (*v.borrow())[i]() })) });
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
            { Some(10_u64) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*not_null_terminated.borrow())
                            [(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('b' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(!({ (*v.borrow())[i]() })) });
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
        let v: Value<std_bitset_31_> = Rc::new(RefCell::new(std_bitset_31_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            None,
            None,
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1010101010\0")));
        let v: Value<std_bitset_31_> = Rc::new(RefCell::new(std_bitset_31_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_u64) },
            None,
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1a1a1a1a1a\0")));
        let v: Value<std_bitset_31_> = Rc::new(RefCell::new(std_bitset_31_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_u64) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"bababababa\0")));
        let v: Value<std_bitset_31_> = Rc::new(RefCell::new(std_bitset_31_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_u64) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('b' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
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
            { Some(10_u64) },
            None,
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*not_null_terminated.borrow())
                            [(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(!({ (*v.borrow())[i]() })) });
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
            { Some(10_u64) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*not_null_terminated.borrow())
                            [(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(!({ (*v.borrow())[i]() })) });
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
            { Some(10_u64) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*not_null_terminated.borrow())
                            [(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('b' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(!({ (*v.borrow())[i]() })) });
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
        let v: Value<std_bitset_32_> = Rc::new(RefCell::new(std_bitset_32_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            None,
            None,
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1010101010\0")));
        let v: Value<std_bitset_32_> = Rc::new(RefCell::new(std_bitset_32_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_u64) },
            None,
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1a1a1a1a1a\0")));
        let v: Value<std_bitset_32_> = Rc::new(RefCell::new(std_bitset_32_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_u64) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"bababababa\0")));
        let v: Value<std_bitset_32_> = Rc::new(RefCell::new(std_bitset_32_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_u64) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('b' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
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
            { Some(10_u64) },
            None,
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*not_null_terminated.borrow())
                            [(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(!({ (*v.borrow())[i]() })) });
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
            { Some(10_u64) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*not_null_terminated.borrow())
                            [(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(!({ (*v.borrow())[i]() })) });
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
            { Some(10_u64) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*not_null_terminated.borrow())
                            [(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('b' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(!({ (*v.borrow())[i]() })) });
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
        let v: Value<std_bitset_33_> = Rc::new(RefCell::new(std_bitset_33_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            None,
            None,
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1010101010\0")));
        let v: Value<std_bitset_33_> = Rc::new(RefCell::new(std_bitset_33_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_u64) },
            None,
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1a1a1a1a1a\0")));
        let v: Value<std_bitset_33_> = Rc::new(RefCell::new(std_bitset_33_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_u64) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"bababababa\0")));
        let v: Value<std_bitset_33_> = Rc::new(RefCell::new(std_bitset_33_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_u64) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('b' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
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
            { Some(10_u64) },
            None,
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*not_null_terminated.borrow())
                            [(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(!({ (*v.borrow())[i]() })) });
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
            { Some(10_u64) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*not_null_terminated.borrow())
                            [(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(!({ (*v.borrow())[i]() })) });
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
            { Some(10_u64) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*not_null_terminated.borrow())
                            [(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('b' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(!({ (*v.borrow())[i]() })) });
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
        let v: Value<std_bitset_63_> = Rc::new(RefCell::new(std_bitset_63_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            None,
            None,
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1010101010\0")));
        let v: Value<std_bitset_63_> = Rc::new(RefCell::new(std_bitset_63_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_u64) },
            None,
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1a1a1a1a1a\0")));
        let v: Value<std_bitset_63_> = Rc::new(RefCell::new(std_bitset_63_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_u64) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"bababababa\0")));
        let v: Value<std_bitset_63_> = Rc::new(RefCell::new(std_bitset_63_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_u64) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('b' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
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
            { Some(10_u64) },
            None,
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*not_null_terminated.borrow())
                            [(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(!({ (*v.borrow())[i]() })) });
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
            { Some(10_u64) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*not_null_terminated.borrow())
                            [(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(!({ (*v.borrow())[i]() })) });
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
            { Some(10_u64) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*not_null_terminated.borrow())
                            [(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('b' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(!({ (*v.borrow())[i]() })) });
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
        let v: Value<std_bitset_64_> = Rc::new(RefCell::new(std_bitset_64_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            None,
            None,
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1010101010\0")));
        let v: Value<std_bitset_64_> = Rc::new(RefCell::new(std_bitset_64_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_u64) },
            None,
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1a1a1a1a1a\0")));
        let v: Value<std_bitset_64_> = Rc::new(RefCell::new(std_bitset_64_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_u64) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"bababababa\0")));
        let v: Value<std_bitset_64_> = Rc::new(RefCell::new(std_bitset_64_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_u64) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('b' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
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
            { Some(10_u64) },
            None,
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*not_null_terminated.borrow())
                            [(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(!({ (*v.borrow())[i]() })) });
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
            { Some(10_u64) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*not_null_terminated.borrow())
                            [(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(!({ (*v.borrow())[i]() })) });
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
            { Some(10_u64) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*not_null_terminated.borrow())
                            [(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('b' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(!({ (*v.borrow())[i]() })) });
            i.prefix_inc();
        }
    }
}
pub fn test_char_pointer_ctor_9() {
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
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1010101010\0")));
        let v: Value<std_bitset_65_> = Rc::new(RefCell::new(std_bitset_65_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_u64) },
            None,
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1a1a1a1a1a\0")));
        let v: Value<std_bitset_65_> = Rc::new(RefCell::new(std_bitset_65_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_u64) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"bababababa\0")));
        let v: Value<std_bitset_65_> = Rc::new(RefCell::new(std_bitset_65_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_u64) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('b' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
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
            { Some(10_u64) },
            None,
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*not_null_terminated.borrow())
                            [(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(!({ (*v.borrow())[i]() })) });
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
            { Some(10_u64) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*not_null_terminated.borrow())
                            [(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(!({ (*v.borrow())[i]() })) });
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
            { Some(10_u64) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*not_null_terminated.borrow())
                            [(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('b' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(!({ (*v.borrow())[i]() })) });
            i.prefix_inc();
        }
    }
}
pub fn test_char_pointer_ctor_10() {
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
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1010101010\0")));
        let v: Value<std_bitset_1000_> = Rc::new(RefCell::new(std_bitset_1000_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_u64) },
            None,
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1a1a1a1a1a\0")));
        let v: Value<std_bitset_1000_> = Rc::new(RefCell::new(std_bitset_1000_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_u64) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
            i.prefix_inc();
        }
    }
    {
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"bababababa\0")));
        let v: Value<std_bitset_1000_> = Rc::new(RefCell::new(std_bitset_1000_::new_1(
            { (s.as_pointer() as Ptr<i8>) },
            { Some(10_u64) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*s.borrow())[(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('b' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(((({ (*v.borrow())[i]() }) as i32) == (false as i32))) });
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
            { Some(10_u64) },
            None,
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*not_null_terminated.borrow())
                            [(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(!({ (*v.borrow())[i]() })) });
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
            { Some(10_u64) },
            { Some(('a' as i8)) },
            None,
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*not_null_terminated.borrow())
                            [(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('1' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(!({ (*v.borrow())[i]() })) });
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
            { Some(10_u64) },
            { Some(('a' as i8)) },
            { Some(('b' as i8)) },
        )));
        let mut M: u64 = (({
            let ___a: Value<u64> = Rc::new(RefCell::new(({ (*v.borrow()).size() })));
            let ___b: Value<u64> = Rc::new(RefCell::new(10_u64));
            min_1(___a.as_pointer(), ___b.as_pointer())
        })
        .read());
        let mut i: u64 = 0_u64;
        'loop_: while (i < M) {
            ({
                cpp2rust_assert_fail_2(
                    ((({ (*v.borrow())[i]() }) as i32)
                        == ((((*not_null_terminated.borrow())
                            [(((M).wrapping_sub(1_u64)).wrapping_sub(i)) as usize]
                            as i32)
                            == (('b' as i8) as i32)) as i32)),
                )
            });
            i.prefix_inc();
        }
        let mut i: u64 = 10_u64;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            ({ cpp2rust_assert_fail_2(!({ (*v.borrow())[i]() })) });
            i.prefix_inc();
        }
    }
}
pub fn test_11() -> bool {
    ({ test_char_pointer_ctor_0() });
    ({ test_char_pointer_ctor_3() });
    ({ test_char_pointer_ctor_4() });
    ({ test_char_pointer_ctor_5() });
    ({ test_char_pointer_ctor_6() });
    ({ test_char_pointer_ctor_7() });
    ({ test_char_pointer_ctor_8() });
    ({ test_char_pointer_ctor_9() });
    ({ test_char_pointer_ctor_10() });
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
    ({ test_11() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
