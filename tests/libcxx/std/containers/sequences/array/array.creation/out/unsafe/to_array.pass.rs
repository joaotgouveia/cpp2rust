extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_array_int__3___std_array_int__3__ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_long__long_ {}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_array_char__5___std_array_char__5__ {}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_array_double__3___std_array_double__3__ {}
pub type anon_4 = u32;
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_array_MoveOnly__3___std_array_MoveOnly__3__ {}
pub type anon_5 = u32;
pub const anon_5_value: anon_5 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_array_int__2___std_array_int__2__ {}
pub type anon_6 = u32;
pub const anon_6_value: anon_6 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_array_long__3___std_array_long__3__ {}
pub type anon_7 = u32;
pub const anon_7_value: anon_7 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_array_A__1___std_array_A__1__ {}
pub unsafe fn operator_eq_8(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_9(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_10(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_11(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_12(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_13(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_14(_a0: *const MoveOnly, _a1: *const MoveOnly) -> std::cmp::Ordering {
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
        unsafe { operator_cmp_14(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::PartialOrd for MoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_8(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::Eq for MoveOnly {}
impl Default for MoveOnly {
    fn default() -> Self {
        unsafe { MoveOnly::new(None) }
    }
}
pub unsafe fn operator_eq_15(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_16(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_17(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_18(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_19(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_20(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_21(
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
            operator_cmp_21(
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
            operator_eq_15(
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
pub unsafe fn tests_22() -> bool {
    {
        let mut arr: Vec<i32> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            ((*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<i32> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) == (1))
        );
        assert!(
            ((*{
                let __idx = 1_usize as usize;
                let __v: &mut Vec<i32> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) == (2))
        );
        assert!(
            ((*{
                let __idx = 2_usize as usize;
                let __v: &mut Vec<i32> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) == (3))
        );
    }
    {
        let l1: i64 = 42_i64;
        let mut arr: Vec<i64> = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "arr.size() == 4");;
        assert!(
            ((*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<i64> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) == (1_i64))
        );
        assert!(
            ((*{
                let __idx = 1_usize as usize;
                let __v: &mut Vec<i64> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) == (4_i64))
        );
        assert!(
            ((*{
                let __idx = 2_usize as usize;
                let __v: &mut Vec<i64> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) == (9_i64))
        );
        assert!(
            ((*{
                let __idx = 3_usize as usize;
                let __v: &mut Vec<i64> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) == (l1))
        );
    }
    {
        let mut arr: Vec<libc::c_char> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            (((*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<libc::c_char> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) as i32)
                == (('m' as libc::c_char) as i32))
        );
        assert!(
            (((*{
                let __idx = 1_usize as usize;
                let __v: &mut Vec<libc::c_char> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) as i32)
                == (('e' as libc::c_char) as i32))
        );
        assert!(
            (((*{
                let __idx = 2_usize as usize;
                let __v: &mut Vec<libc::c_char> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) as i32)
                == (('o' as libc::c_char) as i32))
        );
        assert!(
            (((*{
                let __idx = 3_usize as usize;
                let __v: &mut Vec<libc::c_char> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) as i32)
                == (('w' as libc::c_char) as i32))
        );
        assert!(
            (((*{
                let __idx = 4_usize as usize;
                let __v: &mut Vec<libc::c_char> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) as i32)
                == (('\0' as libc::c_char) as i32))
        );
    }
    {
        let mut source: [f64; 3] = [4_f64, 5_f64, 6_f64];
        let mut arr: Vec<f64> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            ((*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<f64> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) == (4_f64))
        );
        assert!(
            ((*{
                let __idx = 1_usize as usize;
                let __v: &mut Vec<f64> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) == (5_f64))
        );
        assert!(
            ((*{
                let __idx = 2_usize as usize;
                let __v: &mut Vec<f64> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) == (6_f64))
        );
    }
    {
        let mut source: [f64; 3] = [4_f64, 5_f64, 6_f64];
        let mut arr: Vec<f64> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            ((*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<f64> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) == (4_f64))
        );
        assert!(
            ((*{
                let __idx = 1_usize as usize;
                let __v: &mut Vec<f64> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) == (5_f64))
        );
        assert!(
            ((*{
                let __idx = 2_usize as usize;
                let __v: &mut Vec<f64> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) == (6_f64))
        );
    }
    {
        let mut source: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let mut arr: Vec<MoveOnly> = unimplemented!();
        const _: () = assert!(true, "");;
        let mut i: i32 = 0;
        'loop_: while ((i) < (3)) {
            assert!(
                ((unsafe {
                    MoveOnly::get(
                        &(*{
                            let __idx = (i as usize) as usize;
                            let __v: &mut Vec<MoveOnly> = &mut arr;
                            __v.as_mut_ptr().add(__idx)
                        }),
                    )
                }) == (i))
                    && ((unsafe { MoveOnly::get(&source[(i) as usize],) }) == (0))
            );
            i.prefix_inc();
        }
    }
    {
        let mut arr: Vec<i32> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            ((*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<i32> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) == (3))
        );
        assert!(
            ((*{
                let __idx = 1_usize as usize;
                let __v: &mut Vec<i32> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) == (4))
        );
    }
    {
        let mut arr: Vec<i64> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            ((*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<i64> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) == (1_i64))
        );
        assert!(
            ((*{
                let __idx = 1_usize as usize;
                let __v: &mut Vec<i64> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) == (2_i64))
        );
        assert!(
            ((*{
                let __idx = 2_usize as usize;
                let __v: &mut Vec<i64> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) == (3_i64))
        );
    }
    {
        let mut arr: Vec<A_23> = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(
            (((*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<A_23> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            })
            .a) == (3))
        );
        assert!(
            (((*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<A_23> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            })
            .b) == (0.1_f64))
        );
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A_23 {
    pub a: i32,
    pub b: f64,
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
    (unsafe { tests_22() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
