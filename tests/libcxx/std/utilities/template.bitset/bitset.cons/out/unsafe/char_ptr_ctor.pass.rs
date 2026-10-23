extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_char_pointer_ctor_0() {
    const _: () = assert!(
        true,
        "!std::is_convertible<const char*, std::bitset<N> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible<std::bitset<N>, const char*>::value"
    );;
    {
        let s: [(); 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = std_bitset_0_::new_1({ s.as_ptr() }, None, None, None);
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let s: [(); 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = std_bitset_0_::new_1({ s.as_ptr() }, { Some(10_()) }, None, None);
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let s: [(); 11] = std::mem::transmute(*b"1a1a1a1a1a\0");
        let mut v: () =
            std_bitset_0_::new_1({ s.as_ptr() }, { Some(10_()) }, { Some(('a' as ())) }, None);
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let s: [(); 11] = std::mem::transmute(*b"bababababa\0");
        let mut v: () =
            std_bitset_0_::new_1({ s.as_ptr() }, { Some(10_()) }, { Some(('a' as ())) }, {
                Some(('b' as ()))
            });
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [(); 10] = [
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
        ];
        let mut v: () = std_bitset_0_::new_1(
            { not_null_terminated.as_ptr() },
            { Some(10_()) },
            None,
            None,
        );
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [(); 10] = [
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
        ];
        let mut v: () = std_bitset_0_::new_1(
            { not_null_terminated.as_ptr() },
            { Some(10_()) },
            { Some(('a' as ())) },
            None,
        );
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [(); 10] = [
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
        ];
        let mut v: () = std_bitset_0_::new_1(
            { not_null_terminated.as_ptr() },
            { Some(10_()) },
            { Some(('a' as ())) },
            { Some(('b' as ())) },
        );
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
}
pub unsafe fn test_char_pointer_ctor_1() {
    const _: () = assert!(
        true,
        "!std::is_convertible<const char*, std::bitset<N> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible<std::bitset<N>, const char*>::value"
    );;
    {
        let s: [(); 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = std_bitset_1_::new_1({ s.as_ptr() }, None, None, None);
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let s: [(); 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = std_bitset_1_::new_1({ s.as_ptr() }, { Some(10_()) }, None, None);
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let s: [(); 11] = std::mem::transmute(*b"1a1a1a1a1a\0");
        let mut v: () =
            std_bitset_1_::new_1({ s.as_ptr() }, { Some(10_()) }, { Some(('a' as ())) }, None);
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let s: [(); 11] = std::mem::transmute(*b"bababababa\0");
        let mut v: () =
            std_bitset_1_::new_1({ s.as_ptr() }, { Some(10_()) }, { Some(('a' as ())) }, {
                Some(('b' as ()))
            });
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [(); 10] = [
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
        ];
        let mut v: () = std_bitset_1_::new_1(
            { not_null_terminated.as_ptr() },
            { Some(10_()) },
            None,
            None,
        );
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [(); 10] = [
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
        ];
        let mut v: () = std_bitset_1_::new_1(
            { not_null_terminated.as_ptr() },
            { Some(10_()) },
            { Some(('a' as ())) },
            None,
        );
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [(); 10] = [
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
        ];
        let mut v: () = std_bitset_1_::new_1(
            { not_null_terminated.as_ptr() },
            { Some(10_()) },
            { Some(('a' as ())) },
            { Some(('b' as ())) },
        );
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
}
pub unsafe fn test_char_pointer_ctor_2() {
    const _: () = assert!(
        true,
        "!std::is_convertible<const char*, std::bitset<N> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible<std::bitset<N>, const char*>::value"
    );;
    {
        let s: [(); 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = std_bitset_31_::new_1({ s.as_ptr() }, None, None, None);
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let s: [(); 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = std_bitset_31_::new_1({ s.as_ptr() }, { Some(10_()) }, None, None);
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let s: [(); 11] = std::mem::transmute(*b"1a1a1a1a1a\0");
        let mut v: () =
            std_bitset_31_::new_1({ s.as_ptr() }, { Some(10_()) }, { Some(('a' as ())) }, None);
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let s: [(); 11] = std::mem::transmute(*b"bababababa\0");
        let mut v: () =
            std_bitset_31_::new_1({ s.as_ptr() }, { Some(10_()) }, { Some(('a' as ())) }, {
                Some(('b' as ()))
            });
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [(); 10] = [
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
        ];
        let mut v: () = std_bitset_31_::new_1(
            { not_null_terminated.as_ptr() },
            { Some(10_()) },
            None,
            None,
        );
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [(); 10] = [
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
        ];
        let mut v: () = std_bitset_31_::new_1(
            { not_null_terminated.as_ptr() },
            { Some(10_()) },
            { Some(('a' as ())) },
            None,
        );
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [(); 10] = [
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
        ];
        let mut v: () = std_bitset_31_::new_1(
            { not_null_terminated.as_ptr() },
            { Some(10_()) },
            { Some(('a' as ())) },
            { Some(('b' as ())) },
        );
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
}
pub unsafe fn test_char_pointer_ctor_3() {
    const _: () = assert!(
        true,
        "!std::is_convertible<const char*, std::bitset<N> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible<std::bitset<N>, const char*>::value"
    );;
    {
        let s: [(); 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = std_bitset_32_::new_1({ s.as_ptr() }, None, None, None);
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let s: [(); 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = std_bitset_32_::new_1({ s.as_ptr() }, { Some(10_()) }, None, None);
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let s: [(); 11] = std::mem::transmute(*b"1a1a1a1a1a\0");
        let mut v: () =
            std_bitset_32_::new_1({ s.as_ptr() }, { Some(10_()) }, { Some(('a' as ())) }, None);
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let s: [(); 11] = std::mem::transmute(*b"bababababa\0");
        let mut v: () =
            std_bitset_32_::new_1({ s.as_ptr() }, { Some(10_()) }, { Some(('a' as ())) }, {
                Some(('b' as ()))
            });
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [(); 10] = [
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
        ];
        let mut v: () = std_bitset_32_::new_1(
            { not_null_terminated.as_ptr() },
            { Some(10_()) },
            None,
            None,
        );
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [(); 10] = [
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
        ];
        let mut v: () = std_bitset_32_::new_1(
            { not_null_terminated.as_ptr() },
            { Some(10_()) },
            { Some(('a' as ())) },
            None,
        );
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [(); 10] = [
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
        ];
        let mut v: () = std_bitset_32_::new_1(
            { not_null_terminated.as_ptr() },
            { Some(10_()) },
            { Some(('a' as ())) },
            { Some(('b' as ())) },
        );
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
}
pub unsafe fn test_char_pointer_ctor_4() {
    const _: () = assert!(
        true,
        "!std::is_convertible<const char*, std::bitset<N> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible<std::bitset<N>, const char*>::value"
    );;
    {
        let s: [(); 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = std_bitset_33_::new_1({ s.as_ptr() }, None, None, None);
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let s: [(); 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = std_bitset_33_::new_1({ s.as_ptr() }, { Some(10_()) }, None, None);
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let s: [(); 11] = std::mem::transmute(*b"1a1a1a1a1a\0");
        let mut v: () =
            std_bitset_33_::new_1({ s.as_ptr() }, { Some(10_()) }, { Some(('a' as ())) }, None);
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let s: [(); 11] = std::mem::transmute(*b"bababababa\0");
        let mut v: () =
            std_bitset_33_::new_1({ s.as_ptr() }, { Some(10_()) }, { Some(('a' as ())) }, {
                Some(('b' as ()))
            });
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [(); 10] = [
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
        ];
        let mut v: () = std_bitset_33_::new_1(
            { not_null_terminated.as_ptr() },
            { Some(10_()) },
            None,
            None,
        );
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [(); 10] = [
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
        ];
        let mut v: () = std_bitset_33_::new_1(
            { not_null_terminated.as_ptr() },
            { Some(10_()) },
            { Some(('a' as ())) },
            None,
        );
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [(); 10] = [
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
        ];
        let mut v: () = std_bitset_33_::new_1(
            { not_null_terminated.as_ptr() },
            { Some(10_()) },
            { Some(('a' as ())) },
            { Some(('b' as ())) },
        );
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
}
pub unsafe fn test_char_pointer_ctor_5() {
    const _: () = assert!(
        true,
        "!std::is_convertible<const char*, std::bitset<N> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible<std::bitset<N>, const char*>::value"
    );;
    {
        let s: [(); 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = std_bitset_63_::new_1({ s.as_ptr() }, None, None, None);
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let s: [(); 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = std_bitset_63_::new_1({ s.as_ptr() }, { Some(10_()) }, None, None);
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let s: [(); 11] = std::mem::transmute(*b"1a1a1a1a1a\0");
        let mut v: () =
            std_bitset_63_::new_1({ s.as_ptr() }, { Some(10_()) }, { Some(('a' as ())) }, None);
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let s: [(); 11] = std::mem::transmute(*b"bababababa\0");
        let mut v: () =
            std_bitset_63_::new_1({ s.as_ptr() }, { Some(10_()) }, { Some(('a' as ())) }, {
                Some(('b' as ()))
            });
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [(); 10] = [
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
        ];
        let mut v: () = std_bitset_63_::new_1(
            { not_null_terminated.as_ptr() },
            { Some(10_()) },
            None,
            None,
        );
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [(); 10] = [
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
        ];
        let mut v: () = std_bitset_63_::new_1(
            { not_null_terminated.as_ptr() },
            { Some(10_()) },
            { Some(('a' as ())) },
            None,
        );
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [(); 10] = [
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
        ];
        let mut v: () = std_bitset_63_::new_1(
            { not_null_terminated.as_ptr() },
            { Some(10_()) },
            { Some(('a' as ())) },
            { Some(('b' as ())) },
        );
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
}
pub unsafe fn test_char_pointer_ctor_6() {
    const _: () = assert!(
        true,
        "!std::is_convertible<const char*, std::bitset<N> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible<std::bitset<N>, const char*>::value"
    );;
    {
        let s: [(); 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = std_bitset_64_::new_1({ s.as_ptr() }, None, None, None);
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let s: [(); 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = std_bitset_64_::new_1({ s.as_ptr() }, { Some(10_()) }, None, None);
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let s: [(); 11] = std::mem::transmute(*b"1a1a1a1a1a\0");
        let mut v: () =
            std_bitset_64_::new_1({ s.as_ptr() }, { Some(10_()) }, { Some(('a' as ())) }, None);
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let s: [(); 11] = std::mem::transmute(*b"bababababa\0");
        let mut v: () =
            std_bitset_64_::new_1({ s.as_ptr() }, { Some(10_()) }, { Some(('a' as ())) }, {
                Some(('b' as ()))
            });
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [(); 10] = [
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
        ];
        let mut v: () = std_bitset_64_::new_1(
            { not_null_terminated.as_ptr() },
            { Some(10_()) },
            None,
            None,
        );
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [(); 10] = [
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
        ];
        let mut v: () = std_bitset_64_::new_1(
            { not_null_terminated.as_ptr() },
            { Some(10_()) },
            { Some(('a' as ())) },
            None,
        );
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [(); 10] = [
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
        ];
        let mut v: () = std_bitset_64_::new_1(
            { not_null_terminated.as_ptr() },
            { Some(10_()) },
            { Some(('a' as ())) },
            { Some(('b' as ())) },
        );
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
}
pub unsafe fn test_char_pointer_ctor_7() {
    const _: () = assert!(
        true,
        "!std::is_convertible<const char*, std::bitset<N> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible<std::bitset<N>, const char*>::value"
    );;
    {
        let s: [(); 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = std_bitset_65_::new_1({ s.as_ptr() }, None, None, None);
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let s: [(); 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = std_bitset_65_::new_1({ s.as_ptr() }, { Some(10_()) }, None, None);
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let s: [(); 11] = std::mem::transmute(*b"1a1a1a1a1a\0");
        let mut v: () =
            std_bitset_65_::new_1({ s.as_ptr() }, { Some(10_()) }, { Some(('a' as ())) }, None);
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let s: [(); 11] = std::mem::transmute(*b"bababababa\0");
        let mut v: () =
            std_bitset_65_::new_1({ s.as_ptr() }, { Some(10_()) }, { Some(('a' as ())) }, {
                Some(('b' as ()))
            });
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [(); 10] = [
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
        ];
        let mut v: () = std_bitset_65_::new_1(
            { not_null_terminated.as_ptr() },
            { Some(10_()) },
            None,
            None,
        );
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [(); 10] = [
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
        ];
        let mut v: () = std_bitset_65_::new_1(
            { not_null_terminated.as_ptr() },
            { Some(10_()) },
            { Some(('a' as ())) },
            None,
        );
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [(); 10] = [
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
        ];
        let mut v: () = std_bitset_65_::new_1(
            { not_null_terminated.as_ptr() },
            { Some(10_()) },
            { Some(('a' as ())) },
            { Some(('b' as ())) },
        );
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
}
pub unsafe fn test_char_pointer_ctor_8() {
    const _: () = assert!(
        true,
        "!std::is_convertible<const char*, std::bitset<N> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible<std::bitset<N>, const char*>::value"
    );;
    {
        let s: [(); 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = std_bitset_1000_::new_1({ s.as_ptr() }, None, None, None);
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let s: [(); 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = std_bitset_1000_::new_1({ s.as_ptr() }, { Some(10_()) }, None, None);
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let s: [(); 11] = std::mem::transmute(*b"1a1a1a1a1a\0");
        let mut v: () =
            std_bitset_1000_::new_1({ s.as_ptr() }, { Some(10_()) }, { Some(('a' as ())) }, None);
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let s: [(); 11] = std::mem::transmute(*b"bababababa\0");
        let mut v: () =
            std_bitset_1000_::new_1({ s.as_ptr() }, { Some(10_()) }, { Some(('a' as ())) }, {
                Some(('b' as ()))
            });
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [(); 10] = [
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
            ('1' as ()),
            ('0' as ()),
        ];
        let mut v: () = std_bitset_1000_::new_1(
            { not_null_terminated.as_ptr() },
            { Some(10_()) },
            None,
            None,
        );
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [(); 10] = [
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
            ('1' as ()),
            ('a' as ()),
        ];
        let mut v: () = std_bitset_1000_::new_1(
            { not_null_terminated.as_ptr() },
            { Some(10_()) },
            { Some(('a' as ())) },
            None,
        );
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [(); 10] = [
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
            ('b' as ()),
            ('a' as ()),
        ];
        let mut v: () = std_bitset_1000_::new_1(
            { not_null_terminated.as_ptr() },
            { Some(10_()) },
            { Some(('a' as ())) },
            { Some(('b' as ())) },
        );
        let mut M: () = (*unimplemented!());
        let mut i: () = 0_();
        'loop_: while ((i) < (M)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
}
pub unsafe fn test_9() -> () {
    (unsafe { test_char_pointer_ctor_0() });
    (unsafe { test_char_pointer_ctor_1() });
    (unsafe { test_char_pointer_ctor_2() });
    (unsafe { test_char_pointer_ctor_3() });
    (unsafe { test_char_pointer_ctor_4() });
    (unsafe { test_char_pointer_ctor_5() });
    (unsafe { test_char_pointer_ctor_6() });
    (unsafe { test_char_pointer_ctor_7() });
    (unsafe { test_char_pointer_ctor_8() });
    return true;
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_9() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
