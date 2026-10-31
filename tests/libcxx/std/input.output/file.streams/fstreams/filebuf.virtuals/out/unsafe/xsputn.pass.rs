extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sputn_seekoff_0(
    mut buf: *mut libc::c_char,
    buf_size: usize,
    chunk_size1: (),
    offset1: (),
    chunk_size2: (),
) {
    let mut data: Vec<libc::c_char> = std::slice::from_raw_parts(
        c"abcdefghijklmnopqrstuvwxyz".as_ptr(),
        libcc2rs::c_str_len(c"abcdefghijklmnopqrstuvwxyz".as_ptr()) + 1,
    )
    .to_vec();
    let data_size: () = ((data.len() - 1) as ());
    assert!(((chunk_size1) <= (data_size)));
    assert!(((chunk_size2) <= (data_size)));
    let mut result_size: usize = (((((5_()) + (chunk_size1)) + (chunk_size2)) + (1_())) as usize);
    if ((offset1) > (0_())) {
        result_size = (result_size).wrapping_add((offset1 as usize));
    }
    let mut result: Vec<libc::c_char> = vec![(0 as libc::c_char); result_size as usize];
    {
        let mut f: () = unimplemented!();
        unimplemented!();
        assert!(!((unimplemented!()).is_null()));
        assert!(unimplemented!());
        assert!((unimplemented!() != 0));
        let mut chunk: Vec<libc::c_char> = {
            let mut __v = Vec::new();
            let mut __it = UnsafeRandomAccessIterator::Vec(data.as_mut_ptr()).iter_add(5_isize);
            while __it
                != UnsafeRandomAccessIterator::Vec(data.as_mut_ptr())
                    .iter_add(5_isize)
                    .iter_add((chunk_size1 as isize))
            {
                __v.push((*__it.iter_deref()));
                __it.iter_inc();
            }
            __v
        };
        {
            let mut __first = chunk.begin();
            let __last = chunk.end();
            let mut __out = result.begin().iter_add(5_isize);
            while __first != __last {
                *__out.iter_deref() = (*__first.iter_deref());
                __first.iter_inc();
                __out.iter_inc();
            }
            __out
        };
        let len1: () = unimplemented!();
        assert!(((len1) == (chunk_size1)));
        assert!(
            ((libc::strncmp(
                (chunk.as_mut_ptr()).cast_const(),
                {
                    let mut __tmp1 = data[(5_usize) as usize
                        ..::std::cmp::min((5_usize + (len1 as usize)) as usize, data.len() - 1)]
                        .to_vec();
                    __tmp1.push((0 as libc::c_char));
                    __tmp1
                }
                .as_ptr(),
                (len1 as usize) as usize
            )) == (0))
        );
        let mut p1: () = unimplemented!();
        let mut c: libc::c_char = (0 as libc::c_char);
        if ((unimplemented!()) < (0_())) {
            unimplemented!();
            assert!(((unimplemented!()) == (0_())));
            c = ('^' as libc::c_char);
        } else {
            assert!(((unimplemented!()) == ((((5_()) + (len1)) as i64) + (offset1 as i64))));
            if ((unimplemented!()) > (data_size)) {
                c = ('_' as libc::c_char);
            } else {
                c = (*(&mut data)
                    .as_mut_ptr()
                    .add((unimplemented!() as usize) as usize));
            }
        }
        (*(&mut result)
            .as_mut_ptr()
            .add((unimplemented!() as usize) as usize)) = c;
        assert!(((unimplemented!()) == (c as i32)));
        unimplemented!();
        (*(&mut result).as_mut_ptr().add(0_usize as usize)) = ('A' as libc::c_char);
        assert!(((unimplemented!()) == (('A' as libc::c_char) as i32)));
        let mut end_pos: () = unimplemented!();
        assert!(((unimplemented!()) == (('Z' as libc::c_char) as i32)));
        (*(&mut result)
            .as_mut_ptr()
            .add((unimplemented!() as usize) as usize)) = ('Z' as libc::c_char);
        assert!(unimplemented!());
        (*(&mut result)
            .as_mut_ptr()
            .add((unimplemented!() as usize) as usize)) = (if (0..=255).contains(&(c as i32)) {
            ((c as i32) as u8).to_ascii_uppercase() as i32
        } else {
            (c as i32)
        } as libc::c_char);
        assert!(
            ((unimplemented!())
                == (if (0..=255).contains(&(c as i32)) {
                    ((c as i32) as u8).to_ascii_uppercase() as i32
                } else {
                    (c as i32)
                }))
        );
        let mut new_pos: () = unimplemented!();
        let mut p2: () = unimplemented!();
        assert!(unimplemented!());
        chunk = {
            let mut __v = Vec::new();
            let mut __it = UnsafeRandomAccessIterator::Vec(data.as_mut_ptr().add(data.len() - 1))
                .iter_add(-(chunk_size2 as isize));
            while __it != UnsafeRandomAccessIterator::Vec(data.as_mut_ptr().add(data.len() - 1)) {
                __v.push((*__it.iter_deref()));
                __it.iter_inc();
            }
            __v
        };
        {
            let mut __first = chunk.begin();
            let __last = chunk.end();
            let mut __out = result.begin().iter_add((unimplemented!() as isize));
            while __first != __last {
                *__out.iter_deref() = (*__first.iter_deref());
                __first.iter_inc();
                __out.iter_inc();
            }
            __out
        };
        let len2: () = unimplemented!();
        assert!(((len2) == (chunk_size2)));
        assert!(
            ((libc::strncmp(
                (chunk.as_mut_ptr()).cast_const(),
                {
                    let mut __tmp1 = data[(((data_size) - (chunk_size2)) as usize) as usize
                        ..::std::cmp::min(
                            ((((data_size) - (chunk_size2)) as usize) + (chunk_size2 as usize))
                                as usize,
                            data.len() - 1,
                        )]
                        .to_vec();
                    __tmp1.push((0 as libc::c_char));
                    __tmp1
                }
                .as_ptr(),
                (len2 as usize) as usize
            )) == (0))
        );
        unimplemented!();
    }
    let mut f: () = unimplemented!();
    assert!(!((unimplemented!()).is_null()));
    assert!(unimplemented!());
    let mut check: Vec<libc::c_char> = vec![(-1_i32 as libc::c_char); result.len() as usize];
    let len: usize = (unimplemented!() as usize);
    assert!(((len) == (result.len())));
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (len)) {
        assert!(
            (((*(&mut check).as_mut_ptr().add(i as usize)) as i32)
                == ((*(&mut result).as_mut_ptr().add(i as usize)) as i32))
        );
        i.prefix_inc();
    }
}
pub unsafe fn sputn_not_open_1() {
    let mut data: Vec<libc::c_char> = vec![('a' as libc::c_char); 10_usize as usize];
    let mut f: () = unimplemented!();
    let mut len: () = unimplemented!();
    assert!(((len) == (0_())));
    assert!(
        ((libc::strncmp(
            (data.as_mut_ptr()).cast_const(),
            c"aaaaaaaaaa".as_ptr(),
            10_usize as usize
        )) == (0))
    );
}
pub unsafe fn sputn_not_open_wchar_2() {
    let mut data: Vec<i32> = vec![(97 as i32); 10_usize as usize];
    let mut f: () = unimplemented!();
    let mut len: () = unimplemented!();
    assert!(((len) == (0_())));
    assert!(((unimplemented!()) == (0)));
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { sputn_not_open_1() });
    (unsafe { sputn_not_open_wchar_2() });
    (unsafe { sputn_seekoff_0(std::ptr::null_mut(), 10_usize, 21_(), (-27_i32 as ()), 1_()) });
    (unsafe { sputn_seekoff_0(std::ptr::null_mut(), 10_usize, 1_(), (-27_i32 as ()), 1_()) });
    (unsafe { sputn_seekoff_0(std::ptr::null_mut(), 10_usize, 10_(), 14_(), 12_()) });
    (unsafe { sputn_seekoff_0(std::ptr::null_mut(), 10_usize, 1_(), (-2_i32 as ()), 1_()) });
    (unsafe { sputn_seekoff_0(std::ptr::null_mut(), 10_usize, 10_(), (-4_i32 as ()), 12_()) });
    (unsafe { sputn_seekoff_0(std::ptr::null_mut(), 10_usize, 11_(), (-12_i32 as ()), 3_()) });
    (unsafe { sputn_seekoff_0(std::ptr::null_mut(), 10_usize, 7_(), 3_(), 8_()) });
    (unsafe { sputn_seekoff_0(std::ptr::null_mut(), 10_usize, 5_(), (-5_i32 as ()), 12_()) });
    (unsafe { sputn_seekoff_0(std::ptr::null_mut(), 10_usize, 1_(), 1_(), 1_()) });
    (unsafe { sputn_seekoff_0(std::ptr::null_mut(), 10_usize, 9_(), 0_(), 1_()) });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
