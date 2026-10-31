extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct indirect_less {}
impl indirect_less {
    pub unsafe fn operator_call_1_class_std_unique_ptr_int_(
        &self,
        x: *const Option<Box<i32>>,
        y: *const Option<Box<i32>>,
    ) -> bool {
        return ((*(*x)
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            < (*(*y)
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32)));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct first_only {}
impl first_only {
    pub unsafe fn operator_call(&self, x: *const (i32, i32), y: *const (i32, i32)) -> bool {
        return (((*x).0) < ((*y).0));
    }
}
pub unsafe fn generate_sawtooth_0(mut N: i32, mut M: i32) -> Vec<(i32, i32)> {
    let mut v: Vec<(i32, i32)> = (0..(N as usize) as usize)
        .map(|_| <(i32, i32)>::default())
        .collect::<Vec<_>>();
    let mut x: i32 = 0;
    let mut ver: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        (*(&mut v).as_mut_ptr().add((i as usize) as usize)) = (x, ver);
        if ((x.prefix_inc()) == (M)) {
            x = 0;
            ver.prefix_inc();
        }
        i.prefix_inc();
    }
    return std::mem::take(&mut v);
}
pub unsafe fn test_1() -> bool {
    let N: i32 = 1000;
    let M: i32 = 10;
    {
        let mut v: Vec<(i32, i32)> = (unsafe { generate_sawtooth_0(N, M) });
        {
            let __first = v.begin();
            let __n = v.end().iter_distance(&__first);
            let __cmp = <first_only>::default();
            let mut __order: Vec<isize> = (0..__n).collect();
            __order.sort_by(|&__x, &__y| {
                let __a = __first.iter_at(__x) as *const _;
                let __b = __first.iter_at(__y) as *const _;
                if __cmp.unsafe_less(__a, __b) {
                    std::cmp::Ordering::Less
                } else if __cmp.unsafe_less(__b, __a) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            let __values: Vec<_> = __order
                .iter()
                .map(|&__i| std::ptr::read(__first.iter_at(__i)))
                .collect();
            for (__i, __v) in (0..__n).zip(__values) {
                std::ptr::write(__first.iter_at(__i), __v);
            }
        };
        assert!(unimplemented!());
    }
    {
        {
            let mut v: Vec<(i32, i32)> = (unsafe { generate_sawtooth_0(N, M) });
            let mut randomness: () = unimplemented!();
            let mut i: i32 = 0;
            'loop_: while ((i) < ((N) - (M))) {
                unimplemented!();
                i += M;
            }
            {
                let __first = v.begin();
                let __n = v.end().iter_distance(&__first);
                let __cmp = <first_only>::default();
                let mut __order: Vec<isize> = (0..__n).collect();
                __order.sort_by(|&__x, &__y| {
                    let __a = __first.iter_at(__x) as *const _;
                    let __b = __first.iter_at(__y) as *const _;
                    if __cmp.unsafe_less(__a, __b) {
                        std::cmp::Ordering::Less
                    } else if __cmp.unsafe_less(__b, __a) {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Equal
                    }
                });
                let __values: Vec<_> = __order
                    .iter()
                    .map(|&__i| std::ptr::read(__first.iter_at(__i)))
                    .collect();
                for (__i, __v) in (0..__n).zip(__values) {
                    std::ptr::write(__first.iter_at(__i), __v);
                }
            };
            assert!(unimplemented!());
        }
    }
    {
        let mut v: Vec<Option<Box<i32>>> =
            (0..(1000_usize) as usize).map(|_| None).collect::<Vec<_>>();
        let mut i: i32 = 0;
        'loop_: while ((i as usize) < (v.len())) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                (*(&mut v).as_mut_ptr().add((i as usize) as usize)) = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        {
            let __first = v.begin();
            let __n = v.end().iter_distance(&__first);
            let __cmp = <indirect_less>::default();
            let mut __order: Vec<isize> = (0..__n).collect();
            __order.sort_by(|&__x, &__y| {
                let __a = __first.iter_at(__x) as *const _;
                let __b = __first.iter_at(__y) as *const _;
                if __cmp.unsafe_less(__a, __b) {
                    std::cmp::Ordering::Less
                } else if __cmp.unsafe_less(__b, __a) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            let __values: Vec<_> = __order
                .iter()
                .map(|&__i| std::ptr::read(__first.iter_at(__i)))
                .collect();
            for (__i, __v) in (0..__n).zip(__values) {
                std::ptr::write(__first.iter_at(__i), __v);
            }
        };
        assert!(unimplemented!());
        assert!(
            ((*(*(&mut v).as_mut_ptr().add(0_usize as usize))
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*(*(&mut v).as_mut_ptr().add(1_usize as usize))
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*(*(&mut v).as_mut_ptr().add(2_usize as usize))
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
    }
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_1() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
