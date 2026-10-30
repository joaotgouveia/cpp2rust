extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_weak_ordering__std_weak_ordering_ {}
pub unsafe fn has_weak_order_1(t: *mut (), u: *mut ()) -> () {
    return true;
}
pub unsafe fn has_weak_order_2(t: *mut *mut (), u: *mut *mut ()) -> () {
    return true;
}
pub unsafe fn has_weak_order_3(t: *mut *const (), u: *mut *const ()) -> () {
    return true;
}
pub unsafe fn has_weak_order_4(t: *mut N11_A, u: *mut N11_A) -> () {
    return true;
}
pub unsafe fn has_weak_order_5(__args: Box<[VaArg]>) -> () {
    return false;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N11_A {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N11_B {}
pub unsafe fn weak_order_6(_a0: *const N11_A, _a1: *const N11_A) -> () {
    return unimplemented!();
}
pub unsafe fn test_1_1_7() {
    const _: () = assert!(true, "has_weak_order(1, 2)");;
    const _: () = assert!(true, "!has_weak_order(1, (short)2)");;
    const _: () = assert!(true, "!has_weak_order(1, 2.0)");;
    const _: () = assert!(true, "!has_weak_order(1.0f, 2.0)");;
    const _: () = assert!(true, "has_weak_order((int*)nullptr, (int*)nullptr)");;
    const _: () = assert!(true, "!has_weak_order((int*)nullptr, (const int*)nullptr)");;
    const _: () = assert!(true, "!has_weak_order((const int*)nullptr, (int*)nullptr)");;
    const _: () = assert!(
        true,
        "has_weak_order((const int*)nullptr, (const int*)nullptr)"
    );;
    let mut a: N11_A = <N11_A>::default();
    let mut b: N11_B = <N11_B>::default();
    const _: () = assert!(true, "has_weak_order(a, a)");;
    const _: () = assert!(true, "!has_weak_order(a, b)");;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N12_A {}
pub unsafe fn weak_order_8(_a0: *mut N12_A, _a1: *mut N12_A) -> () {
    return unimplemented!();
}
pub unsafe fn weak_order_9(_a0: *mut N12_A, _a1: *mut N12_A) -> () {
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N12_B {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N12_WeakOrder {}
impl N12_WeakOrder {
    pub unsafe fn to_(&self) -> () {
        return unimplemented!();
    }
}
pub unsafe fn weak_order_10(lhs: *mut N12_C, _a1: *mut N12_C) -> N12_WeakOrder {
    (*lhs).touched = true;
    return <N12_WeakOrder>::default();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct N12_C {
    pub touched: (),
}
impl Default for N12_C {
    fn default() -> Self {
        unsafe { N12_C { touched: false } }
    }
}
pub unsafe fn test_1_2_11() {
    let mut a: N12_A = <N12_A>::default();
    unimplemented!();
    unimplemented!();
    let mut b: N12_B = <N12_B>::default();
    const _: () = assert!(true, "!has_weak_order(b, b)");;
    let mut c1: N12_C = <N12_C>::default();
    let mut c2: N12_C = <N12_C>::default();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_1_3_12() -> () {
    const _: () = assert!(true, "");;
    let mut v: [(); 14] = [
        -unimplemented!(),
        unimplemented!(),
        -1_f64,
        -0.1_f64,
        -unimplemented!(),
        -0_f64,
        0_f64,
        unimplemented!(),
        0.1_f64,
        1_f64,
        2_f64,
        3.14_f64,
        unimplemented!(),
        unimplemented!(),
    ];
    const _: () = assert!(true, "std::size(v) == 14");;
    let mut i: () = 0_();
    'loop_: while ((i) < (14_())) {
        let mut j: () = 0_();
        'loop_: while ((j) < (14_())) {
            let mut naturalOrder: () = (std::cmp::Ord::cmp(&(v[(i) as usize]), &(v[(j) as usize])));
            if ((v[(i) as usize]) == (0_())) && ((v[(j) as usize]) == (0_())) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut nq: () = unimplemented!();
        let mut ns: () = unimplemented!();
        let mut ps: () = unimplemented!();
        let mut pq: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (14_())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (14_())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (14_())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (14_())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
    }
    return true;
}
pub unsafe fn test_1_3_13() -> () {
    const _: () = assert!(true, "");;
    let mut v: [(); 14] = [
        -unimplemented!(),
        unimplemented!(),
        -1_f64,
        -0.1_f64,
        -unimplemented!(),
        -0_f64,
        0_f64,
        unimplemented!(),
        0.1_f64,
        1_f64,
        2_f64,
        3.14_f64,
        unimplemented!(),
        unimplemented!(),
    ];
    const _: () = assert!(true, "std::size(v) == 14");;
    let mut i: () = 0_();
    'loop_: while ((i) < (14_())) {
        let mut j: () = 0_();
        'loop_: while ((j) < (14_())) {
            let mut naturalOrder: () = (std::cmp::Ord::cmp(&(v[(i) as usize]), &(v[(j) as usize])));
            if ((v[(i) as usize]) == (0_())) && ((v[(j) as usize]) == (0_())) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut nq: () = unimplemented!();
        let mut ns: () = unimplemented!();
        let mut ps: () = unimplemented!();
        let mut pq: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (14_())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (14_())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (14_())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (14_())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
    }
    return true;
}
pub unsafe fn test_1_3_14() -> () {
    const _: () = assert!(true, "");;
    let mut v: [(); 14] = [
        -unimplemented!(),
        unimplemented!(),
        -1_f64,
        -0.1_f64,
        -unimplemented!(),
        -0_f64,
        0_f64,
        unimplemented!(),
        0.1_f64,
        1_f64,
        2_f64,
        3.14_f64,
        unimplemented!(),
        unimplemented!(),
    ];
    const _: () = assert!(true, "std::size(v) == 14");;
    let mut i: () = 0_();
    'loop_: while ((i) < (14_())) {
        let mut j: () = 0_();
        'loop_: while ((j) < (14_())) {
            let mut naturalOrder: () = (std::cmp::Ord::cmp(&(v[(i) as usize]), &(v[(j) as usize])));
            if ((v[(i) as usize]) == (0_())) && ((v[(j) as usize]) == (0_())) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut nq: () = unimplemented!();
        let mut ns: () = unimplemented!();
        let mut ps: () = unimplemented!();
        let mut pq: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (14_())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (14_())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (14_())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (14_())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N14_A {}
impl std::cmp::Ord for N14_A {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_15(self as *const N14_A, other as *const N14_A) }
    }
}
impl std::cmp::PartialOrd for N14_A {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for N14_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_cmp_15(self as *const N14_A, other as *const N14_A)
                == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for N14_A {}
pub unsafe fn operator_cmp_16(_a0: *mut N14_A, _a1: *mut N14_A) -> () {
    return unimplemented!();
}
pub unsafe fn operator_cmp_15(_a0: *mut N14_A, _a1: *mut N14_A) -> () {
    return unimplemented!();
}
const _: () = assert!(true, "std::three_way_comparable<A>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N14_B {}
const _: () = assert!(!(false), "!std::three_way_comparable<B>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N14_C {
    pub touched: *mut (),
}
impl N14_C {
    pub unsafe fn operator_cmp(&self, rhs: *const N14_C) -> () {
        (*(*rhs).touched) = true;
        return unimplemented!();
    }
}
impl std::cmp::Ord for N14_C {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { N14_C::operator_cmp(self, other as *const N14_C) }
    }
}
impl std::cmp::PartialOrd for N14_C {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for N14_C {
    fn eq(&self, other: &Self) -> bool {
        unsafe { N14_C::operator_cmp(self, other as *const N14_C) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for N14_C {}
const _: () = assert!(true, "std::three_way_comparable<C>");
pub unsafe fn test_1_4_17() -> () {
    let mut a: N14_A = <N14_A>::default();
    unimplemented!();
    unimplemented!();
    let mut b: N14_B = <N14_B>::default();
    const _: () = assert!(true, "!has_weak_order(b, b)");;
    let mut c1_touched: () = false;
    let mut c2_touched: () = false;
    let mut c1: N14_C = N14_C {
        touched: (&mut c1_touched as *mut ()),
    };
    let mut c2: N14_C = N14_C {
        touched: (&mut c2_touched as *mut ()),
    };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N15_A {}
pub unsafe fn strong_order_18(_a0: *mut N15_A, _a1: *mut N15_A) -> () {
    return unimplemented!();
}
pub unsafe fn strong_order_19(_a0: *mut N15_A, _a1: *mut N15_A) -> () {
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N15_B {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N15_WeakOrder {}
impl N15_WeakOrder {
    pub unsafe fn to_(&self) -> () {
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N15_C {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N15_StrongOrder {}
impl N15_StrongOrder {
    pub unsafe fn to__1(&self) -> () {
        return unimplemented!();
    }
}
pub unsafe fn strong_order_20(lhs: *mut N15_D, _a1: *mut N15_D) -> N15_StrongOrder {
    (*lhs).touched = true;
    return <N15_StrongOrder>::default();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct N15_D {
    pub touched: (),
}
impl Default for N15_D {
    fn default() -> Self {
        unsafe { N15_D { touched: false } }
    }
}
pub unsafe fn test_1_5_21() -> () {
    let mut a: N15_A = <N15_A>::default();
    unimplemented!();
    unimplemented!();
    let mut b: N15_B = <N15_B>::default();
    const _: () = assert!(true, "!has_weak_order(b, b)");;
    let mut c: N15_C = <N15_C>::default();
    const _: () = assert!(true, "!has_weak_order(c, c)");;
    let mut d1: N15_D = <N15_D>::default();
    let mut d2: N15_D = <N15_D>::default();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    (unsafe { test_1_1_7() });
    (unsafe { test_1_2_11() });
    (unsafe { test_1_3_12() });
    (unsafe { test_1_3_13() });
    (unsafe { test_1_3_14() });
    (unsafe { test_1_4_17() });
    (unsafe { test_1_5_21() });
    const _: () = assert!(true, "test_1_3<float>()");;
    const _: () = assert!(true, "test_1_3<double>()");;
    const _: () = assert!(true, "test_1_3<long double>()");;
    const _: () = assert!(true, "test_1_4()");;
    const _: () = assert!(true, "test_1_5()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
