extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(
    mut c1: Vec<Option<Box<i32>>>,
    mut j: (),
    mut x1: Option<Box<i32>>,
    mut x2: Option<Box<i32>>,
    mut x3: Option<Box<i32>>,
    c2: *const Vec<Option<Box<i32>>>,
) {
    let mut q: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn insert3at_1(
    c: *mut Vec<Option<Box<i32>>>,
    mut i: UnsafeRandomAccessIterator<Option<Box<i32>>>,
    mut x1: Option<Box<i32>>,
    mut x2: Option<Box<i32>>,
    mut x3: Option<Box<i32>>,
) {
    i = {
        let __it = <UnsafeRandomAccessIterator<Option<Box<i32>>>>::from((i).clone());
        let pos = __it.iter_distance(&UnsafeRandomAccessIterator::Vec((*c).as_mut_ptr())) as usize;
        (*c).insert(pos, (x1).clone());
        __it
    };
    i = {
        let __it = <UnsafeRandomAccessIterator<Option<Box<i32>>>>::from((*i.iter_inc()).clone());
        let pos = __it.iter_distance(&UnsafeRandomAccessIterator::Vec((*c).as_mut_ptr())) as usize;
        (*c).insert(pos, (x2).clone());
        __it
    };
    {
        let __it = <UnsafeRandomAccessIterator<Option<Box<i32>>>>::from((*i.iter_inc()).clone());
        let pos = __it.iter_distance(&UnsafeRandomAccessIterator::Vec((*c).as_mut_ptr())) as usize;
        (*c).insert(pos, (x3).clone());
        __it
    };
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct do_nothing {}
impl do_nothing {
    pub unsafe fn operator_call(&self, mut _a0: *mut ::libc::c_void) {}
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
    {
        let mut c1: Vec<Option<Box<i32>>> = Vec::new();
        let mut x: [i32; 6] = [0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32];
        let mut i: i32 = 0;
        'loop_: while ((i) < (3)) {
            {
                let __a1 = Some(Box::from_raw(x.as_mut_ptr().offset((i) as isize)));
                c1.push(__a1)
            };
            i.prefix_inc();
        }
        let mut c2: Vec<Option<Box<i32>>> = Vec::new();
        let mut i: i32 = 0;
        'loop_: while ((i) < (3)) {
            {
                let __a1 = Some(Box::from_raw(x.as_mut_ptr().offset((i) as isize)));
                c2.push(__a1)
            };
            i.prefix_inc();
        }
        (unsafe {
            let _c: *mut Vec<Option<Box<i32>>> = &mut c2;
            let _i: UnsafeRandomAccessIterator<Option<Box<i32>>> = c2.begin();
            let _x1: Option<Box<i32>> = Some(Box::from_raw(x.as_mut_ptr().offset((3) as isize)));
            let _x2: Option<Box<i32>> = Some(Box::from_raw(x.as_mut_ptr().offset((4) as isize)));
            let _x3: Option<Box<i32>> = Some(Box::from_raw(x.as_mut_ptr().offset((5) as isize)));
            insert3at_1(_c, _i, _x1, _x2, _x3)
        });
        (unsafe {
            let _x1: Option<Box<i32>> = Some(Box::from_raw(x.as_mut_ptr().offset((3) as isize)));
            let _x2: Option<Box<i32>> = Some(Box::from_raw(x.as_mut_ptr().offset((4) as isize)));
            let _x3: Option<Box<i32>> = Some(Box::from_raw(x.as_mut_ptr().offset((5) as isize)));
            test_0(std::mem::take(&mut c1), 0_(), _x1, _x2, _x3, &c2)
        });
        c1.clear();
        let mut i: i32 = 0;
        'loop_: while ((i) < (3)) {
            {
                let __a1 = Some(Box::from_raw(x.as_mut_ptr().offset((i) as isize)));
                c1.push(__a1)
            };
            i.prefix_inc();
        }
        c2.clear();
        let mut i: i32 = 0;
        'loop_: while ((i) < (3)) {
            {
                let __a1 = Some(Box::from_raw(x.as_mut_ptr().offset((i) as isize)));
                c2.push(__a1)
            };
            i.prefix_inc();
        }
        (unsafe {
            let _c: *mut Vec<Option<Box<i32>>> = &mut c2;
            let _i: UnsafeRandomAccessIterator<Option<Box<i32>>> = c2.begin().iter_add(1_isize);
            let _x1: Option<Box<i32>> = Some(Box::from_raw(x.as_mut_ptr().offset((3) as isize)));
            let _x2: Option<Box<i32>> = Some(Box::from_raw(x.as_mut_ptr().offset((4) as isize)));
            let _x3: Option<Box<i32>> = Some(Box::from_raw(x.as_mut_ptr().offset((5) as isize)));
            insert3at_1(_c, _i, _x1, _x2, _x3)
        });
        (unsafe {
            let _x1: Option<Box<i32>> = Some(Box::from_raw(x.as_mut_ptr().offset((3) as isize)));
            let _x2: Option<Box<i32>> = Some(Box::from_raw(x.as_mut_ptr().offset((4) as isize)));
            let _x3: Option<Box<i32>> = Some(Box::from_raw(x.as_mut_ptr().offset((5) as isize)));
            test_0(std::mem::take(&mut c1), 1_(), _x1, _x2, _x3, &c2)
        });
        c1.clear();
        let mut i: i32 = 0;
        'loop_: while ((i) < (3)) {
            {
                let __a1 = Some(Box::from_raw(x.as_mut_ptr().offset((i) as isize)));
                c1.push(__a1)
            };
            i.prefix_inc();
        }
        c2.clear();
        let mut i: i32 = 0;
        'loop_: while ((i) < (3)) {
            {
                let __a1 = Some(Box::from_raw(x.as_mut_ptr().offset((i) as isize)));
                c2.push(__a1)
            };
            i.prefix_inc();
        }
        (unsafe {
            let _c: *mut Vec<Option<Box<i32>>> = &mut c2;
            let _i: UnsafeRandomAccessIterator<Option<Box<i32>>> = c2.begin().iter_add(2_isize);
            let _x1: Option<Box<i32>> = Some(Box::from_raw(x.as_mut_ptr().offset((3) as isize)));
            let _x2: Option<Box<i32>> = Some(Box::from_raw(x.as_mut_ptr().offset((4) as isize)));
            let _x3: Option<Box<i32>> = Some(Box::from_raw(x.as_mut_ptr().offset((5) as isize)));
            insert3at_1(_c, _i, _x1, _x2, _x3)
        });
        (unsafe {
            let _x1: Option<Box<i32>> = Some(Box::from_raw(x.as_mut_ptr().offset((3) as isize)));
            let _x2: Option<Box<i32>> = Some(Box::from_raw(x.as_mut_ptr().offset((4) as isize)));
            let _x3: Option<Box<i32>> = Some(Box::from_raw(x.as_mut_ptr().offset((5) as isize)));
            test_0(std::mem::take(&mut c1), 2_(), _x1, _x2, _x3, &c2)
        });
        c1.clear();
        let mut i: i32 = 0;
        'loop_: while ((i) < (3)) {
            {
                let __a1 = Some(Box::from_raw(x.as_mut_ptr().offset((i) as isize)));
                c1.push(__a1)
            };
            i.prefix_inc();
        }
        c2.clear();
        let mut i: i32 = 0;
        'loop_: while ((i) < (3)) {
            {
                let __a1 = Some(Box::from_raw(x.as_mut_ptr().offset((i) as isize)));
                c2.push(__a1)
            };
            i.prefix_inc();
        }
        (unsafe {
            let _c: *mut Vec<Option<Box<i32>>> = &mut c2;
            let _i: UnsafeRandomAccessIterator<Option<Box<i32>>> = c2.begin().iter_add(3_isize);
            let _x1: Option<Box<i32>> = Some(Box::from_raw(x.as_mut_ptr().offset((3) as isize)));
            let _x2: Option<Box<i32>> = Some(Box::from_raw(x.as_mut_ptr().offset((4) as isize)));
            let _x3: Option<Box<i32>> = Some(Box::from_raw(x.as_mut_ptr().offset((5) as isize)));
            insert3at_1(_c, _i, _x1, _x2, _x3)
        });
        (unsafe {
            let _x1: Option<Box<i32>> = Some(Box::from_raw(x.as_mut_ptr().offset((3) as isize)));
            let _x2: Option<Box<i32>> = Some(Box::from_raw(x.as_mut_ptr().offset((4) as isize)));
            let _x3: Option<Box<i32>> = Some(Box::from_raw(x.as_mut_ptr().offset((5) as isize)));
            test_0(std::mem::take(&mut c1), 3_(), _x1, _x2, _x3, &c2)
        });
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
