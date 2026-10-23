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
pub struct test_macros_detail_is_same_std_tuple_Unconstrained___std_tuple_Unconstrained__ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_tuple_int__const_int__int_ref__const_int_ref__int_refref__int__const_int__int_ref__const_int_ref__int_refref__int__const_int__int_ref__const_int_ref__int__const_int__int_ref__const_int_ref___std_tuple_int__const_int__int_ref__const_int_ref__int_refref__int__const_int__int_ref__const_int_ref__int_refref__int__const_int__int_ref__const_int_ref__int__const_int__int_ref__const_int_ref__
{}
pub unsafe fn operator_eq_2(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_3(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_4(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_5(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_6(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_7(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_8(_a0: *const MoveOnly, _a1: *const MoveOnly) -> () {
    {
        let mut cmp: () = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct MoveOnly {
    data_: (),
}
impl MoveOnly {
    pub unsafe fn new(mut data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| unsafe { 1_() });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(x: *mut MoveOnly) -> Self {
        let mut this = Self { data_: (*x).data_ };
        (*x).data_ = 0_();
        this
    }
    pub unsafe fn move_assign(&mut self, x: *mut MoveOnly) -> *mut MoveOnly {
        self.data_ = (*x).data_;
        (*x).data_ = 0_();
        return &mut (*(self as *mut MoveOnly));
    }
    pub unsafe fn get(&self) -> () {
        return self.data_;
    }
    pub unsafe fn operator_add(&self, x: *const MoveOnly) -> MoveOnly {
        return MoveOnly::new({ Some(((self.data_) + ((*x).data_))) });
    }
    pub unsafe fn operator_mul(&self, x: *const MoveOnly) -> MoveOnly {
        return MoveOnly::new({ Some(((self.data_) * ((*x).data_))) });
    }
}
impl std::cmp::Ord for MoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_8(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::PartialOrd for MoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_2(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::Eq for MoveOnly {}
impl Default for MoveOnly {
    fn default() -> Self {
        unsafe { MoveOnly::new(None) }
    }
}
pub unsafe fn operator_eq_9(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_10(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_11(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_12(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_13(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_14(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_15(_a0: *const TrivialMoveOnly, _a1: *const TrivialMoveOnly) -> () {
    {
        let mut cmp: () = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct TrivialMoveOnly {
    data_: (),
}
impl TrivialMoveOnly {
    pub unsafe fn new(mut data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| unsafe { 1_() });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(_a0: *mut TrivialMoveOnly) -> Self {
        let mut this = Self {
            data_: (*_a0).data_,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut TrivialMoveOnly) -> *mut TrivialMoveOnly {
        self.data_ = (*_a0).data_;
        return &mut (*(self as *mut TrivialMoveOnly));
    }
    pub unsafe fn get(&self) -> () {
        return self.data_;
    }
    pub unsafe fn operator_add(&self, x: *const TrivialMoveOnly) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({ Some(((self.data_) + ((*x).data_))) });
    }
    pub unsafe fn operator_mul(&self, x: *const TrivialMoveOnly) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({ Some(((self.data_) * ((*x).data_))) });
    }
}
impl std::cmp::Ord for TrivialMoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_15(
                self as *const TrivialMoveOnly,
                other as *const TrivialMoveOnly,
            )
        }
    }
}
impl std::cmp::PartialOrd for TrivialMoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for TrivialMoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_9(
                self as *const TrivialMoveOnly,
                other as *const TrivialMoveOnly,
            )
        }
    }
}
impl std::cmp::Eq for TrivialMoveOnly {}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        unsafe { TrivialMoveOnly::new(None) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NS_Namespaced {
    pub i: (),
}
impl Unconstrained {
    pub unsafe fn new(mut arg: ()) -> Self {
        let mut this = Self { data: arg };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Unconstrained {
    pub data: (),
}
pub unsafe fn test_tuple_cat_with_unconstrained_constructor_16() -> () {
    {
        let mut tup_src: () = unimplemented!();
        let mut tup: std_tuple_Unconstrained_ = unimplemented!();
        unimplemented!();
    }
    {
        let mut tup: std_tuple_Unconstrained_ = unimplemented!();
        unimplemented!();
    }
    {
        let mut tup: std_tuple_Unconstrained_ = unimplemented!();
        unimplemented!();
    }
    {
        let mut tup_src: () = unimplemented!();
        let mut tup: std_tuple_Unconstrained_ = unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let mut tup: std_tuple_Unconstrained_ = unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let mut tup: std_tuple_Unconstrained_ = unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    {
        let mut t: () = unimplemented!();
        (&(t));
    }
    {
        let mut t1: () = unimplemented!();
        let mut t2: () = unimplemented!();
        (&(t2));
    }
    {
        let mut t: () = unimplemented!();
        (&(t));
    }
    {
        let mut t: () = unimplemented!();
        (&(t));
    }
    {
        let mut t1: () = unimplemented!();
        let mut t: () = unimplemented!();
        unimplemented!();
    }
    {
        let t: () = unimplemented!();
        (&(t));
    }
    {
        let t1: () = unimplemented!();
        let t2: () = unimplemented!();
        (&(t2));
    }
    {
        let t: () = unimplemented!();
        (&(t));
    }
    {
        let t: () = unimplemented!();
        (&(t));
    }
    {
        let t1: () = unimplemented!();
        let t: () = unimplemented!();
        const _: () = assert!(true, "std::get<0>(t) == 1");;
    }
    {
        let t1: () = unimplemented!();
        let t: () = unimplemented!();
        const _: () = assert!(true, "std::get<0>(t) == 1");;
        const _: () = assert!(true, "std::get<1>(t) == 1");;
    }
    {
        let mut t: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut t: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut t: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut t1: () = unimplemented!();
        let mut t2: () = unimplemented!();
        let mut t3: () = unimplemented!();
        (&(t3));
    }
    {
        let mut t1: () = unimplemented!();
        let mut t2: () = unimplemented!();
        let mut t3: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut t1: () = unimplemented!();
        let mut t2: () = unimplemented!();
        let mut t3: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut t1: () = unimplemented!();
        let mut t2: () = unimplemented!();
        let mut t3: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut t1: () = unimplemented!();
        let mut t2: () = unimplemented!();
        let mut t3: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut t1: () = unimplemented!();
        let mut t2: () = unimplemented!();
        let mut t3: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut t1: () = unimplemented!();
        let mut t2: () = unimplemented!();
        let mut t3: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut t1: () = unimplemented!();
        let mut t2: () = unimplemented!();
        let mut t3: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut t1: () = unimplemented!();
        let mut t2: () = unimplemented!();
        let mut t3: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut t1: () = unimplemented!();
        let mut t2: () = unimplemented!();
        let mut t3: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut t1: () = unimplemented!();
        let mut t2: () = unimplemented!();
        let mut t3: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut t1: () = unimplemented!();
        let mut t2: () = unimplemented!();
        let mut t3: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut t1: () = unimplemented!();
        let mut t2: () = unimplemented!();
        let mut t3: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut t1: () = unimplemented!();
        let mut t2: () = unimplemented!();
        let mut t3: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut t1: std_tuple_std_tuple_int__ = unimplemented!();
        unimplemented!();
        let mut t2: std_tuple_std_tuple_int___std_tuple_int__ = unimplemented!();
        unimplemented!();
    }
    {
        let mut x: () = 101_();
        let mut t: () = unimplemented!();
        let ct: *const () = &t;
        let mut t2: () = unimplemented!();
        let ct2: *const () = &t2;
        let mut r : std_tuple_int__const_int__int_ref__const_int_ref__int_refref__int__const_int__int_ref__const_int_ref__int_refref__int__const_int__int_ref__const_int_ref__int__const_int__int_ref__const_int_ref_ = unimplemented!()  ;
        const _: () = assert!(true, "");;
        (&(r));
    }
    {
        let mut t1: () = unimplemented!();
        let mut t: () = unimplemented!();
        let mut t2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        (unsafe { test_tuple_cat_with_unconstrained_constructor_16() });
        const _: () = assert!(true, "test_tuple_cat_with_unconstrained_constructor()");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
