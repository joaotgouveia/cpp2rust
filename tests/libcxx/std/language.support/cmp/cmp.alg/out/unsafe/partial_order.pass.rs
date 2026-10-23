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
pub struct test_macros_detail_is_same_std_partial_ordering__std_partial_ordering_ {}
pub unsafe fn has_partial_order_1(t: *mut (), u: *mut ()) -> () {
    return true;
}
pub unsafe fn has_partial_order_2(t: *mut *mut (), u: *mut *mut ()) -> () {
    return true;
}
pub unsafe fn has_partial_order_3(t: *mut *const (), u: *mut *const ()) -> () {
    return true;
}
pub unsafe fn has_partial_order_4(t: *mut N11_A, u: *mut N11_A) -> () {
    return true;
}
pub unsafe fn has_partial_order_5(__args: Box<[VaArg]>) -> () {
    return false;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N11_A {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N11_B {}
pub unsafe fn partial_order_6(_a0: *const N11_A, _a1: *const N11_A) -> () {
    return unimplemented!();
}
pub unsafe fn test_1_1_7() {
    const _: () = assert!(true, "has_partial_order(1, 2)");;
    const _: () = assert!(true, "!has_partial_order(1, (short)2)");;
    const _: () = assert!(true, "!has_partial_order(1, 2.0)");;
    const _: () = assert!(true, "!has_partial_order(1.0f, 2.0)");;
    const _: () = assert!(true, "has_partial_order((int*)nullptr, (int*)nullptr)");;
    const _: () = assert!(
        true,
        "!has_partial_order((int*)nullptr, (const int*)nullptr)"
    );;
    const _: () = assert!(
        true,
        "!has_partial_order((const int*)nullptr, (int*)nullptr)"
    );;
    const _: () = assert!(
        true,
        "has_partial_order((const int*)nullptr, (const int*)nullptr)"
    );;
    let mut a: N11_A = <N11_A>::default();
    let mut b: N11_B = <N11_B>::default();
    const _: () = assert!(true, "has_partial_order(a, a)");;
    const _: () = assert!(true, "!has_partial_order(a, b)");;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N12_A {}
pub unsafe fn partial_order_8(_a0: *mut N12_A, _a1: *mut N12_A) -> () {
    return unimplemented!();
}
pub unsafe fn partial_order_9(_a0: *mut N12_A, _a1: *mut N12_A) -> () {
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N12_B {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N12_PartialOrder {}
impl N12_PartialOrder {
    pub unsafe fn to_(&self) -> () {
        return unimplemented!();
    }
}
pub unsafe fn partial_order_10(lhs: *mut N12_C, _a1: *mut N12_C) -> N12_PartialOrder {
    (*lhs).touched = true;
    return <N12_PartialOrder>::default();
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
    const _: () = assert!(true, "!has_partial_order(b, b)");;
    let mut c1: N12_C = <N12_C>::default();
    let mut c2: N12_C = <N12_C>::default();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N13_A {}
impl std::cmp::Ord for N13_A {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_12(self as *const N13_A, other as *const N13_A) }
    }
}
impl std::cmp::PartialOrd for N13_A {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for N13_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_cmp_12(self as *const N13_A, other as *const N13_A)
                == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for N13_A {}
pub unsafe fn operator_cmp_13(_a0: *mut N13_A, _a1: *mut N13_A) -> () {
    return unimplemented!();
}
pub unsafe fn operator_cmp_12(_a0: *mut N13_A, _a1: *mut N13_A) -> () {
    return unimplemented!();
}
const _: () = assert!(true, "std::three_way_comparable<A>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N13_B {}
const _: () = assert!(!(false), "!std::three_way_comparable<B>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N13_C {
    pub touched: *mut (),
}
impl N13_C {
    pub unsafe fn operator_cmp(&self, rhs: *const N13_C) -> () {
        (*(*rhs).touched) = true;
        return unimplemented!();
    }
}
impl std::cmp::Ord for N13_C {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { N13_C::operator_cmp(self, other as *const N13_C) }
    }
}
impl std::cmp::PartialOrd for N13_C {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for N13_C {
    fn eq(&self, other: &Self) -> bool {
        unsafe { N13_C::operator_cmp(self, other as *const N13_C) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for N13_C {}
const _: () = assert!(true, "std::three_way_comparable<C>");
pub unsafe fn test_1_3_14() -> () {
    let mut a: N13_A = <N13_A>::default();
    unimplemented!();
    unimplemented!();
    let mut b: N13_B = <N13_B>::default();
    const _: () = assert!(true, "!has_partial_order(b, b)");;
    let mut c1_touched: () = false;
    let mut c2_touched: () = false;
    let mut c1: N13_C = N13_C {
        touched: (&mut c1_touched as *mut ()),
    };
    let mut c2: N13_C = N13_C {
        touched: (&mut c2_touched as *mut ()),
    };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut nan: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut nan: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut nan: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N14_A {}
pub unsafe fn weak_order_15(_a0: *mut N14_A, _a1: *mut N14_A) -> () {
    return unimplemented!();
}
pub unsafe fn weak_order_16(_a0: *mut N14_A, _a1: *mut N14_A) -> () {
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N14_B {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N14_StrongOrder {}
impl N14_StrongOrder {
    pub unsafe fn to_(&self) -> () {
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N14_C {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N14_WeakOrder {}
impl N14_WeakOrder {
    pub unsafe fn to__1(&self) -> () {
        return unimplemented!();
    }
}
pub unsafe fn weak_order_17(lhs: *mut N14_D, _a1: *mut N14_D) -> N14_WeakOrder {
    (*lhs).touched = true;
    return <N14_WeakOrder>::default();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct N14_D {
    pub touched: (),
}
impl Default for N14_D {
    fn default() -> Self {
        unsafe { N14_D { touched: false } }
    }
}
pub unsafe fn test_1_4_18() -> () {
    let mut a: N14_A = <N14_A>::default();
    unimplemented!();
    unimplemented!();
    let mut b: N14_B = <N14_B>::default();
    const _: () = assert!(true, "!has_partial_order(b, b)");;
    let mut c: N14_C = <N14_C>::default();
    const _: () = assert!(true, "!has_partial_order(c, c)");;
    let mut d1: N14_D = <N14_D>::default();
    let mut d2: N14_D = <N14_D>::default();
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
    (unsafe { test_1_3_14() });
    (unsafe { test_1_4_18() });
    const _: () = assert!(true, "test_1_3()");;
    const _: () = assert!(true, "test_1_4()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
