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
pub struct NonCopyable {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NonDefaultCtor {}
impl NonDefaultCtor {
    pub unsafe fn new(mut _a0: ()) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Empty {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LessThan3 {}
impl LessThan3 {
    pub unsafe fn operator_call(&self, mut i: ()) -> () {
        return ((i) < (3_()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EqualTo33 {}
impl EqualTo33 {
    pub unsafe fn operator_call(&self, mut i: ()) -> () {
        return ((i) == (33_()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Add3 {}
impl Add3 {
    pub unsafe fn operator_call(&self, mut i: ()) -> () {
        return ((i) + (3_()));
    }
}
const _: () = assert!(
    true,
    "std::is_invocable_v<decltype(std::views::repeat), int>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<decltype(std::views::repeat), void>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<decltype(std::views::repeat), int, Empty>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<decltype(std::views::repeat), int, NonCopyable>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<decltype(std::views::repeat), int, NonDefaultCtor>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<decltype(std::views::repeat), int, std::unreachable_sentinel_t>"
);
const _: () = assert!(
    true,
    "!std::is_invocable_v<decltype(std::views::repeat), NonCopyable>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<decltype(std::views::repeat), MoveOnly>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::views::repeat(std::views::repeat(42))),\n                             std::ranges::repeat_view<std::ranges::repeat_view<int>>>"
);
const _: () = assert!(
    true,
    "std::same_as<decltype(std::views::repeat(\"foo\", std::unreachable_sentinel)), RPV>"
);
const _: () = assert!(
    true,
    "std::same_as<decltype(std::views::repeat(+\"foo\", std::unreachable_sentinel)), RPV>"
);
const _: () = assert!(
    true,
    "std::same_as<decltype(std::views::repeat(\"foo\")), RPV>"
);
const _: () = assert!(
    true,
    "std::same_as<decltype(std::views::repeat(+\"foo\")), RPV>"
);
pub unsafe fn test_14() -> () {
    unimplemented!();
    unimplemented!();
    const _: () = assert!(
        true,
        "std::same_as<decltype(std::views::repeat(42)), std::ranges::repeat_view<int>>"
    );;
    const _: () = assert!(
        true,
        "std::same_as<decltype(std::views::repeat(42, 3)), std::ranges::repeat_view<int, int>>"
    );;
    const _: () = assert!(
        true,
        "std::same_as<decltype(std::views::repeat), decltype(std::ranges::views::repeat)>"
    );;
    {
        let mut r: std_ranges_repeat_view_int__std_unreachable_sentinel_t_ = unimplemented!();
        const _: () = assert!(!(false), "!std::ranges::sized_range<decltype(r)>");;
        unimplemented!();
    }
    {
        let mut r: std_ranges_repeat_view_int__int_ = unimplemented!();
        const _: () = assert!(true, "std::ranges::sized_range<decltype(r)>");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut r: std_ranges_repeat_view_int__long_ = unimplemented!();
        const _: () = assert!(true, "std::ranges::sized_range<decltype(r)>");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut r: std_ranges_repeat_view_int__int_ = unimplemented!();
        const _: () = assert!(true, "std::ranges::sized_range<decltype(r)>");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut r: std_ranges_transform_view_std_ranges_repeat_view_int__int___Add3_ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut r : std_ranges_transform_view_std_ranges_repeat_view_int__std_unreachable_sentinel_t___Add3_ = unimplemented!()  ;
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
    (unsafe { test_14() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
