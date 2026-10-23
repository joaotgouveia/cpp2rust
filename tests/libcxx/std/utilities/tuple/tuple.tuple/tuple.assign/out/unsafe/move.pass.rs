extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_1(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_2(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_3(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_4(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_5(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_6(_a0: *const MoveOnly, _a1: *const MoveOnly) -> () {
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
        unsafe { operator_cmp_6(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::PartialOrd for MoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_0(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::Eq for MoveOnly {}
impl Default for MoveOnly {
    fn default() -> Self {
        unsafe { MoveOnly::new(None) }
    }
}
pub unsafe fn operator_eq_7(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_8(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_9(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_10(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_11(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_12(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_13(_a0: *const TrivialMoveOnly, _a1: *const TrivialMoveOnly) -> () {
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
            operator_cmp_13(
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
            operator_eq_7(
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
#[derive(FnPtrArg, Default)]
pub struct NonAssignable {}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct CopyAssignable {}
const _: () = assert!(true, "std::is_copy_assignable<CopyAssignable>::value");
#[repr(C)]
#[derive(FnPtrArg, MoveAssignUnsafe, Default)]
pub struct MoveAssignable {}
impl MoveAssignable {
    pub unsafe fn move_assign(&mut self, _a0: *mut MoveAssignable) -> *mut MoveAssignable {
        return &mut (*(self as *mut MoveAssignable));
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveAssignUnsafe, Default)]
pub struct NothrowMoveAssignable {}
impl NothrowMoveAssignable {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut NothrowMoveAssignable,
    ) -> *mut NothrowMoveAssignable {
        return &mut (*(self as *mut NothrowMoveAssignable));
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveAssignUnsafe, Default)]
pub struct PotentiallyThrowingMoveAssignable {}
impl PotentiallyThrowingMoveAssignable {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut PotentiallyThrowingMoveAssignable,
    ) -> *mut PotentiallyThrowingMoveAssignable {
        return &mut (*(self as *mut PotentiallyThrowingMoveAssignable));
    }
}
#[repr(C)]
#[derive(FnPtrArg, CopyAssignUnsafe, MoveAssignUnsafe, Default)]
pub struct CountAssign {}
impl CountAssign {
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_14)) = {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_15)) = 0_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_15))
        };
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const CountAssign) -> *mut CountAssign {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copied_14)).prefix_inc();
        return &mut (*(self as *mut CountAssign));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut CountAssign) -> *mut CountAssign {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_15)).prefix_inc();
        return &mut (*(self as *mut CountAssign));
    }
}
pub static mut copied_14: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut moved_15: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub unsafe fn test_16() -> () {
    {
        let mut t0: () = unimplemented!();
        let mut t: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut t0: () = MoveOnly::new({ Some(0_()) });
        let mut t: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut t0: () = {
            let mut __tmp_0: MoveOnly = MoveOnly::new({ Some(0_()) });
            let mut __tmp_1: MoveOnly = MoveOnly::new({ Some(1_()) });
            std_tuple_MoveOnly__MoveOnly_::new_1({ &mut __tmp_0 }, { &mut __tmp_1 })
        };
        let mut t: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut t0: () = {
            let mut __tmp_2: MoveOnly = MoveOnly::new({ Some(0_()) });
            let mut __tmp_3: MoveOnly = MoveOnly::new({ Some(1_()) });
            let mut __tmp_4: MoveOnly = MoveOnly::new({ Some(2_()) });
            std_tuple_MoveOnly__MoveOnly__MoveOnly_::new_1({ &mut __tmp_2 }, { &mut __tmp_3 }, {
                &mut __tmp_4
            })
        };
        let mut t: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut x: () = 42_();
        let mut y: () = 100_();
        let mut x2: () = -1_();
        let mut y2: () = 500_();
        let mut t: () =
            std_tuple_int_ref__int_refref_::new({ &mut x }, { &mut *(&raw mut y).cast::<()>() });
        let mut t2: () =
            std_tuple_int_ref__int_refref_::new({ &mut x2 }, { &mut *(&raw mut y2).cast::<()>() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
    (unsafe { test_16() });
    const _: () = assert!(true, "test()");;
    {
        const _: () = assert!(true, "std::is_move_assignable<T>::value");;
        const _: () = assert!(true, "!std::is_copy_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_move_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "std::is_move_assignable<T>::value");;
    }
    {
        (unsafe { CountAssign::reset() });
        const _: () = assert!(true, "std::is_move_assignable<T>::value");;
        let mut t1: () = unimplemented!();
        let mut t2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "!std::is_move_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "std::is_move_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "std::is_nothrow_move_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_nothrow_move_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_move_assignable<T1>::value");;
        const _: () = assert!(true, "std::is_move_assignable<T2>::value");;
        const _: () = assert!(true, "std::is_move_assignable<T3>::value");;
        const _: () = assert!(true, "std::is_move_assignable<T4>::value");;
        const _: () = assert!(true, "!std::is_move_assignable<T5>::value");;
        const _: () = assert!(true, "!std::is_move_assignable<T6>::value");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copied_14);
    std::cell::LazyCell::force(&*&raw const moved_15);
}
