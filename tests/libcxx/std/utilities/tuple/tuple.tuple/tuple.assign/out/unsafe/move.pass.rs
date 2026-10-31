extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_1(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_2(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_3(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_4(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_5(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_6(_a0: *const MoveOnly, _a1: *const MoveOnly) -> std::cmp::Ordering {
    {
        let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(cmp == std::cmp::Ordering::Equal) {
            return cmp;
        }
    }
    return std::cmp::Ordering::Equal;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, MoveAssignUnsafe)]
#[cpp_from(i32, new, some_value)]
pub struct MoveOnly {
    data_: i32,
}
impl MoveOnly {
    pub unsafe fn new(mut data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| unsafe { 1 });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(x: *mut MoveOnly) -> Self {
        let mut this = Self { data_: (*x).data_ };
        (*x).data_ = 0;
        this
    }
    pub unsafe fn move_assign(&mut self, x: *mut MoveOnly) -> *mut MoveOnly {
        self.data_ = (*x).data_;
        (*x).data_ = 0;
        return &mut (*(self as *mut MoveOnly));
    }
    pub unsafe fn get(&self) -> i32 {
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
pub unsafe fn operator_eq_7(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_8(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_9(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_10(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_11(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_12(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_13(
    _a0: *const TrivialMoveOnly,
    _a1: *const TrivialMoveOnly,
) -> std::cmp::Ordering {
    {
        let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(cmp == std::cmp::Ordering::Equal) {
            return cmp;
        }
    }
    return std::cmp::Ordering::Equal;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, MoveAssignUnsafe)]
#[cpp_from(i32, new, some_value)]
pub struct TrivialMoveOnly {
    data_: i32,
}
impl TrivialMoveOnly {
    pub unsafe fn new(mut data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| unsafe { 1 });
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
    pub unsafe fn get(&self) -> i32 {
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
            (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_15)) = 0;
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
pub static mut copied_14: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut moved_15: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub unsafe fn test_16() -> bool {
    {
        let mut t0: () = unimplemented!();
        let mut t: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut t0: () = unimplemented!();
        let mut t: () = unimplemented!();
        unimplemented!();
        assert!(
            (unsafe {
                let _x: *const MoveOnly = unimplemented!();
                let mut _y: MoveOnly = MoveOnly::new({ Some(0) });
                operator_eq_0(_x, &mut _y)
            })
        );
    }
    {
        let mut t0: () = unimplemented!();
        let mut t: () = unimplemented!();
        unimplemented!();
        assert!(
            (unsafe {
                let _x: *const MoveOnly = unimplemented!();
                let mut _y: MoveOnly = MoveOnly::new({ Some(0) });
                operator_eq_0(_x, &mut _y)
            })
        );
        assert!(
            (unsafe {
                let _x: *const MoveOnly = unimplemented!();
                let mut _y: MoveOnly = MoveOnly::new({ Some(1) });
                operator_eq_0(_x, &mut _y)
            })
        );
    }
    {
        let mut t0: () = unimplemented!();
        let mut t: () = unimplemented!();
        unimplemented!();
        assert!(
            (unsafe {
                let _x: *const MoveOnly = unimplemented!();
                let mut _y: MoveOnly = MoveOnly::new({ Some(0) });
                operator_eq_0(_x, &mut _y)
            })
        );
        assert!(
            (unsafe {
                let _x: *const MoveOnly = unimplemented!();
                let mut _y: MoveOnly = MoveOnly::new({ Some(1) });
                operator_eq_0(_x, &mut _y)
            })
        );
        assert!(
            (unsafe {
                let _x: *const MoveOnly = unimplemented!();
                let mut _y: MoveOnly = MoveOnly::new({ Some(2) });
                operator_eq_0(_x, &mut _y)
            })
        );
    }
    {
        let mut x: i32 = 42;
        let mut y: i32 = 100;
        let mut x2: i32 = -1_i32;
        let mut y2: i32 = 500;
        let mut t: () = unimplemented!();
        let mut t2: () = unimplemented!();
        unimplemented!();
        assert!(((*unimplemented!()) == (x2)));
        assert!(((unimplemented!()) == (&mut x as *mut i32)));
        assert!(((*unimplemented!()) == (y2)));
        assert!(((unimplemented!()) == (&mut y as *mut i32)));
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
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copied_14)) == (1)));
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut moved_15)) == (0)));
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
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copied_14);
    std::cell::LazyCell::force(&*&raw const moved_15);
}
