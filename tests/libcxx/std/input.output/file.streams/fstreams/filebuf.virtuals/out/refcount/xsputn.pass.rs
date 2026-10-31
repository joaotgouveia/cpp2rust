extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sputn_seekoff_0(
    mut buf: Ptr<i8>,
    mut buf_size: usize,
    mut chunk_size1: (),
    mut offset1: (),
    mut chunk_size2: (),
) {
    let data: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes =
            Ptr::<i8>::from_string_literal(b"abcdefghijklmnopqrstuvwxyz").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let mut data_size: () = (((*data.borrow()).len() - 1) as ());
    assert!((chunk_size1 <= data_size));
    assert!((chunk_size2 <= data_size));
    let mut result_size: usize = ((((5_() + chunk_size1) + chunk_size2) + 1_()) as usize);
    if (offset1 > 0_()) {
        result_size = { (result_size).wrapping_add((offset1 as usize)) };
    }
    let result: Value<Vec<i8>> = Rc::new(RefCell::new(vec![0_i8; result_size as usize]));
    {
        let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(!((unimplemented!()).is_null()));
        assert!(unimplemented!());
        assert!((unimplemented!() != 0));
        let chunk: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v = Vec::new();
            let mut __it = RandomAccessIterator::Vec(data.decay()).iter_add(5_isize);
            while __it
                != RandomAccessIterator::Vec(data.decay())
                    .iter_add(5_isize)
                    .iter_add((chunk_size1 as isize))
            {
                __v.push((__it.iter_deref().read()));
                __it.iter_inc();
            }
            __v
        }));
        {
            let mut __first = chunk.decay().begin();
            let __last = chunk.decay().end();
            let mut __out = result.decay().begin().iter_add(5_isize);
            while __first != __last {
                __out.iter_deref().write((__first.iter_deref().read()));
                __first.iter_inc();
                __out.iter_inc();
            }
            __out
        };
        let mut len1: () = unimplemented!();
        assert!((len1 == chunk_size1));
        assert!(
            ({
                let __n = (len1 as usize);
                let mut __it1 = chunk.decay().to_c_string_iterator().take(__n);
                let mut __it2 = Rc::new(RefCell::new({
                    let mut __tmp1 = (*data.borrow())[(5_usize) as usize
                        ..::std::cmp::min(
                            (5_usize + (len1 as usize)) as usize,
                            (*data.borrow()).len().saturating_sub(1),
                        )]
                        .to_vec();
                    __tmp1.push(0_i8);
                    __tmp1
                }))
                .decay()
                .to_c_string_iterator()
                .take(__n);
                loop {
                    let __c1 = __it1.next();
                    let __c2 = __it2.next();
                    if __c1 != __c2 {
                        break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                    }
                    if __c1.is_none() {
                        break 0;
                    }
                }
            } == 0)
        );
        let p1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut c: i8 = 0_i8;
        if (unimplemented!() < 0_()) {
            unimplemented!();
            assert!((unimplemented!() == 0_()));
            c = ('^' as i8);
        } else {
            assert!((unimplemented!() == (((5_() + len1) as i64) + (offset1 as i64))));
            if (unimplemented!() > data_size) {
                c = ('_' as i8);
            } else {
                c = (data
                    .decay()
                    .offset((unimplemented!() as usize) as isize)
                    .read());
            }
        }
        result
            .decay()
            .offset((unimplemented!() as usize) as isize)
            .write(c);
        assert!((unimplemented!() == (c as i32)));
        unimplemented!();
        result.decay().offset(0_usize as isize).write(('A' as i8));
        assert!((unimplemented!() == (('A' as i8) as i32)));
        let end_pos: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == (('Z' as i8) as i32)));
        result
            .decay()
            .offset((unimplemented!() as usize) as isize)
            .write(('Z' as i8));
        assert!(unimplemented!());
        result
            .decay()
            .offset((unimplemented!() as usize) as isize)
            .write(
                (if (0..=255).contains(&(c as i32)) {
                    ((c as i32) as u8).to_ascii_uppercase() as i32
                } else {
                    (c as i32)
                } as i8),
            );
        assert!(
            (unimplemented!()
                == if (0..=255).contains(&(c as i32)) {
                    ((c as i32) as u8).to_ascii_uppercase() as i32
                } else {
                    (c as i32)
                })
        );
        let new_pos: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
        chunk.as_pointer().write({
            let mut __v = Vec::new();
            let mut __it =
                RandomAccessIterator::Vec(data.decay().to_last()).iter_add(-(chunk_size2 as isize));
            while __it != RandomAccessIterator::Vec(data.decay().to_last()) {
                __v.push((__it.iter_deref().read()));
                __it.iter_inc();
            }
            __v
        });
        {
            let mut __first = chunk.decay().begin();
            let __last = chunk.decay().end();
            let mut __out = result.decay().begin().iter_add((unimplemented!() as isize));
            while __first != __last {
                __out.iter_deref().write((__first.iter_deref().read()));
                __first.iter_inc();
                __out.iter_inc();
            }
            __out
        };
        let mut len2: () = unimplemented!();
        assert!((len2 == chunk_size2));
        assert!(
            ({
                let __n = (len2 as usize);
                let mut __it1 = chunk.decay().to_c_string_iterator().take(__n);
                let mut __it2 = Rc::new(RefCell::new({
                    let mut __tmp1 = (*data.borrow())[((data_size - chunk_size2) as usize) as usize
                        ..::std::cmp::min(
                            (((data_size - chunk_size2) as usize) + (chunk_size2 as usize))
                                as usize,
                            (*data.borrow()).len().saturating_sub(1),
                        )]
                        .to_vec();
                    __tmp1.push(0_i8);
                    __tmp1
                }))
                .decay()
                .to_c_string_iterator()
                .take(__n);
                loop {
                    let __c1 = __it1.next();
                    let __c2 = __it2.next();
                    if __c1 != __c2 {
                        break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                    }
                    if __c1.is_none() {
                        break 0;
                    }
                }
            } == 0)
        );
        unimplemented!();
    }
    let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(!((unimplemented!()).is_null()));
    assert!(unimplemented!());
    let check: Value<Vec<i8>> = Rc::new(RefCell::new(vec![
        (-1_i32 as i8);
        (*result.borrow()).len() as usize
    ]));
    let mut len: usize = (unimplemented!() as usize);
    assert!((len == (*result.borrow()).len()));
    let mut i: usize = 0_usize;
    'loop_: while (i < len) {
        assert!(
            (((check.decay().offset(i as isize).read()) as i32)
                == ((result.decay().offset(i as isize).read()) as i32))
        );
        i.prefix_inc();
    }
}
pub fn sputn_not_open_1() {
    let data: Value<Vec<i8>> = Rc::new(RefCell::new(vec![('a' as i8); 10_usize as usize]));
    let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut len: () = unimplemented!();
    assert!((len == 0_()));
    assert!(
        ({
            let __n = 10_usize;
            let mut __it1 = data.decay().to_c_string_iterator().take(__n);
            let mut __it2 = Ptr::<i8>::from_string_literal(b"aaaaaaaaaa")
                .to_c_string_iterator()
                .take(__n);
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0)
    );
}
pub fn sputn_not_open_wchar_2() {
    let data: Value<Vec<i32>> = Rc::new(RefCell::new(vec![(97 as i32); 10_usize as usize]));
    let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut len: () = unimplemented!();
    assert!((len == 0_()));
    assert!((unimplemented!() == 0));
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
    ({ sputn_not_open_1() });
    ({ sputn_not_open_wchar_2() });
    ({ sputn_seekoff_0(Ptr::<i8>::null(), 10_usize, 21_(), (-27_i32 as ()), 1_()) });
    ({ sputn_seekoff_0(Ptr::<i8>::null(), 10_usize, 1_(), (-27_i32 as ()), 1_()) });
    ({ sputn_seekoff_0(Ptr::<i8>::null(), 10_usize, 10_(), 14_(), 12_()) });
    ({ sputn_seekoff_0(Ptr::<i8>::null(), 10_usize, 1_(), (-2_i32 as ()), 1_()) });
    ({ sputn_seekoff_0(Ptr::<i8>::null(), 10_usize, 10_(), (-4_i32 as ()), 12_()) });
    ({ sputn_seekoff_0(Ptr::<i8>::null(), 10_usize, 11_(), (-12_i32 as ()), 3_()) });
    ({ sputn_seekoff_0(Ptr::<i8>::null(), 10_usize, 7_(), 3_(), 8_()) });
    ({ sputn_seekoff_0(Ptr::<i8>::null(), 10_usize, 5_(), (-5_i32 as ()), 12_()) });
    ({ sputn_seekoff_0(Ptr::<i8>::null(), 10_usize, 1_(), 1_(), 1_()) });
    ({ sputn_seekoff_0(Ptr::<i8>::null(), 10_usize, 9_(), 0_(), 1_()) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
