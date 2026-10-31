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
pub struct test_macros_detail_is_same_char_ptr__char_ptr_ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_char_ptr__const_char_ptr_ {}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {}
pub type anon_4 = u32;
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_WithSize_1__ptr__WithSize_1__ptr_ {}
pub type anon_5 = u32;
pub const anon_5_value: anon_5 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_WithSize_1__ptr__const_WithSize_1__ptr_ {}
pub type anon_6 = u32;
pub const anon_6_value: anon_6 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_WithSize_2__ptr__WithSize_2__ptr_ {}
pub type anon_7 = u32;
pub const anon_7_value: anon_7 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_WithSize_2__ptr__const_WithSize_2__ptr_ {}
pub type anon_8 = u32;
pub const anon_8_value: anon_8 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_WithSize_3__ptr__WithSize_3__ptr_ {}
pub type anon_9 = u32;
pub const anon_9_value: anon_9 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_WithSize_3__ptr__const_WithSize_3__ptr_ {}
pub type anon_10 = u32;
pub const anon_10_value: anon_10 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_WithSize_4__ptr__WithSize_4__ptr_ {}
pub type anon_11 = u32;
pub const anon_11_value: anon_11 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_WithSize_4__ptr__const_WithSize_4__ptr_ {}
pub type anon_12 = u32;
pub const anon_12_value: anon_12 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_WithSize_8__ptr__WithSize_8__ptr_ {}
pub type anon_13 = u32;
pub const anon_13_value: anon_13 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_WithSize_8__ptr__const_WithSize_8__ptr_ {}
pub type anon_14 = u32;
pub const anon_14_value: anon_14 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_WithSize_16__ptr__WithSize_16__ptr_ {}
pub type anon_15 = u32;
pub const anon_15_value: anon_15 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_WithSize_16__ptr__const_WithSize_16__ptr_ {}
pub type anon_16 = u32;
pub const anon_16_value: anon_16 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_WithSize_256__ptr__WithSize_256__ptr_ {}
pub type anon_17 = u32;
pub const anon_17_value: anon_17 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_WithSize_256__ptr__const_WithSize_256__ptr_ {}
pub unsafe fn test_basic_18() {
    {
        {
            let mut x: *mut libc::c_char =
                (Box::leak(Box::new((0 as libc::c_char))) as *mut libc::c_char);
            let mut ptr: Option<Box<libc::c_char>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut libc::c_char))
                    == (x))
            );
        }
        {
            let mut x: *mut libc::c_char =
                (Box::leak(Box::new((0 as libc::c_char))) as *mut libc::c_char);
            let ptr: Option<Box<libc::c_char>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut libc::c_char))
                    == (x))
            );
        }
    }
    {
        {
            let mut x: *mut libc::c_char =
                (Box::leak(Box::new((0 as libc::c_char))) as *mut libc::c_char);
            let mut ptr: Option<Box<libc::c_char>> = Some(Box::from_raw((x).cast_const()));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut libc::c_char))
                    == ((x).cast_const()))
            );
        }
        {
            let mut x: *mut libc::c_char =
                (Box::leak(Box::new((0 as libc::c_char))) as *mut libc::c_char);
            let ptr: Option<Box<libc::c_char>> = Some(Box::from_raw((x).cast_const()));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut libc::c_char))
                    == ((x).cast_const()))
            );
        }
    }
    {
        {
            let mut x: *mut libc::c_char = Box::leak(
                (0..3_usize)
                    .map(|_| (0 as libc::c_char))
                    .collect::<Box<[libc::c_char]>>(),
            )
            .as_mut_ptr();
            let mut ptr: Option<Box<[libc::c_char]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == (x))
            );
        }
        {
            let mut x: *mut libc::c_char = Box::leak(
                (0..3_usize)
                    .map(|_| (0 as libc::c_char))
                    .collect::<Box<[libc::c_char]>>(),
            )
            .as_mut_ptr();
            let ptr: Option<Box<[libc::c_char]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == (x))
            );
        }
    }
    {
        {
            let mut x: *mut libc::c_char = Box::leak(
                (0..3_usize)
                    .map(|_| (0 as libc::c_char))
                    .collect::<Box<[libc::c_char]>>(),
            )
            .as_mut_ptr();
            let mut ptr: Option<Box<[libc::c_char]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == ((x).cast_const()))
            );
        }
        {
            let mut x: *mut libc::c_char = Box::leak(
                (0..3_usize)
                    .map(|_| (0 as libc::c_char))
                    .collect::<Box<[libc::c_char]>>(),
            )
            .as_mut_ptr();
            let ptr: Option<Box<[libc::c_char]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == ((x).cast_const()))
            );
        }
    }
}
pub unsafe fn test_basic_19() {
    {
        {
            let mut x: *mut i32 = (Box::leak(Box::new(0_i32)) as *mut i32);
            let mut ptr: Option<Box<i32>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut i32))
                    == (x))
            );
        }
        {
            let mut x: *mut i32 = (Box::leak(Box::new(0_i32)) as *mut i32);
            let ptr: Option<Box<i32>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut i32))
                    == (x))
            );
        }
    }
    {
        {
            let mut x: *mut i32 = (Box::leak(Box::new(0_i32)) as *mut i32);
            let mut ptr: Option<Box<i32>> = Some(Box::from_raw((x).cast_const()));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut i32))
                    == ((x).cast_const()))
            );
        }
        {
            let mut x: *mut i32 = (Box::leak(Box::new(0_i32)) as *mut i32);
            let ptr: Option<Box<i32>> = Some(Box::from_raw((x).cast_const()));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut i32))
                    == ((x).cast_const()))
            );
        }
    }
    {
        {
            let mut x: *mut i32 =
                Box::leak((0..3_usize).map(|_| 0_i32).collect::<Box<[i32]>>()).as_mut_ptr();
            let mut ptr: Option<Box<[i32]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == (x))
            );
        }
        {
            let mut x: *mut i32 =
                Box::leak((0..3_usize).map(|_| 0_i32).collect::<Box<[i32]>>()).as_mut_ptr();
            let ptr: Option<Box<[i32]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == (x))
            );
        }
    }
    {
        {
            let mut x: *mut i32 =
                Box::leak((0..3_usize).map(|_| 0_i32).collect::<Box<[i32]>>()).as_mut_ptr();
            let mut ptr: Option<Box<[i32]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == ((x).cast_const()))
            );
        }
        {
            let mut x: *mut i32 =
                Box::leak((0..3_usize).map(|_| 0_i32).collect::<Box<[i32]>>()).as_mut_ptr();
            let ptr: Option<Box<[i32]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == ((x).cast_const()))
            );
        }
    }
}
pub unsafe fn test_basic_20() {
    {
        {
            let mut x: *mut WithSize_1_ =
                (Box::leak(Box::new(<WithSize_1_>::default())) as *mut WithSize_1_);
            let mut ptr: Option<Box<WithSize_1_>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut WithSize_1_))
                    == (x))
            );
        }
        {
            let mut x: *mut WithSize_1_ =
                (Box::leak(Box::new(<WithSize_1_>::default())) as *mut WithSize_1_);
            let ptr: Option<Box<WithSize_1_>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut WithSize_1_))
                    == (x))
            );
        }
    }
    {
        {
            let mut x: *mut WithSize_1_ =
                (Box::leak(Box::new(<WithSize_1_>::default())) as *mut WithSize_1_);
            let mut ptr: Option<Box<WithSize_1_>> = Some(Box::from_raw((x).cast_const()));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut WithSize_1_))
                    == ((x).cast_const()))
            );
        }
        {
            let mut x: *mut WithSize_1_ =
                (Box::leak(Box::new(<WithSize_1_>::default())) as *mut WithSize_1_);
            let ptr: Option<Box<WithSize_1_>> = Some(Box::from_raw((x).cast_const()));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut WithSize_1_))
                    == ((x).cast_const()))
            );
        }
    }
    {
        {
            let mut x: *mut WithSize_1_ = Box::leak(
                (0..3_usize)
                    .map(|_| <WithSize_1_>::default())
                    .collect::<Box<[WithSize_1_]>>(),
            )
            .as_mut_ptr();
            let mut ptr: Option<Box<[WithSize_1_]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == (x))
            );
        }
        {
            let mut x: *mut WithSize_1_ = Box::leak(
                (0..3_usize)
                    .map(|_| <WithSize_1_>::default())
                    .collect::<Box<[WithSize_1_]>>(),
            )
            .as_mut_ptr();
            let ptr: Option<Box<[WithSize_1_]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == (x))
            );
        }
    }
    {
        {
            let mut x: *mut WithSize_1_ = Box::leak(
                (0..3_usize)
                    .map(|_| <WithSize_1_>::default())
                    .collect::<Box<[WithSize_1_]>>(),
            )
            .as_mut_ptr();
            let mut ptr: Option<Box<[WithSize_1_]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == ((x).cast_const()))
            );
        }
        {
            let mut x: *mut WithSize_1_ = Box::leak(
                (0..3_usize)
                    .map(|_| <WithSize_1_>::default())
                    .collect::<Box<[WithSize_1_]>>(),
            )
            .as_mut_ptr();
            let ptr: Option<Box<[WithSize_1_]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == ((x).cast_const()))
            );
        }
    }
}
pub unsafe fn test_basic_21() {
    {
        {
            let mut x: *mut WithSize_2_ =
                (Box::leak(Box::new(<WithSize_2_>::default())) as *mut WithSize_2_);
            let mut ptr: Option<Box<WithSize_2_>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut WithSize_2_))
                    == (x))
            );
        }
        {
            let mut x: *mut WithSize_2_ =
                (Box::leak(Box::new(<WithSize_2_>::default())) as *mut WithSize_2_);
            let ptr: Option<Box<WithSize_2_>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut WithSize_2_))
                    == (x))
            );
        }
    }
    {
        {
            let mut x: *mut WithSize_2_ =
                (Box::leak(Box::new(<WithSize_2_>::default())) as *mut WithSize_2_);
            let mut ptr: Option<Box<WithSize_2_>> = Some(Box::from_raw((x).cast_const()));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut WithSize_2_))
                    == ((x).cast_const()))
            );
        }
        {
            let mut x: *mut WithSize_2_ =
                (Box::leak(Box::new(<WithSize_2_>::default())) as *mut WithSize_2_);
            let ptr: Option<Box<WithSize_2_>> = Some(Box::from_raw((x).cast_const()));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut WithSize_2_))
                    == ((x).cast_const()))
            );
        }
    }
    {
        {
            let mut x: *mut WithSize_2_ = Box::leak(
                (0..3_usize)
                    .map(|_| <WithSize_2_>::default())
                    .collect::<Box<[WithSize_2_]>>(),
            )
            .as_mut_ptr();
            let mut ptr: Option<Box<[WithSize_2_]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == (x))
            );
        }
        {
            let mut x: *mut WithSize_2_ = Box::leak(
                (0..3_usize)
                    .map(|_| <WithSize_2_>::default())
                    .collect::<Box<[WithSize_2_]>>(),
            )
            .as_mut_ptr();
            let ptr: Option<Box<[WithSize_2_]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == (x))
            );
        }
    }
    {
        {
            let mut x: *mut WithSize_2_ = Box::leak(
                (0..3_usize)
                    .map(|_| <WithSize_2_>::default())
                    .collect::<Box<[WithSize_2_]>>(),
            )
            .as_mut_ptr();
            let mut ptr: Option<Box<[WithSize_2_]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == ((x).cast_const()))
            );
        }
        {
            let mut x: *mut WithSize_2_ = Box::leak(
                (0..3_usize)
                    .map(|_| <WithSize_2_>::default())
                    .collect::<Box<[WithSize_2_]>>(),
            )
            .as_mut_ptr();
            let ptr: Option<Box<[WithSize_2_]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == ((x).cast_const()))
            );
        }
    }
}
pub unsafe fn test_basic_22() {
    {
        {
            let mut x: *mut WithSize_3_ =
                (Box::leak(Box::new(<WithSize_3_>::default())) as *mut WithSize_3_);
            let mut ptr: Option<Box<WithSize_3_>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut WithSize_3_))
                    == (x))
            );
        }
        {
            let mut x: *mut WithSize_3_ =
                (Box::leak(Box::new(<WithSize_3_>::default())) as *mut WithSize_3_);
            let ptr: Option<Box<WithSize_3_>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut WithSize_3_))
                    == (x))
            );
        }
    }
    {
        {
            let mut x: *mut WithSize_3_ =
                (Box::leak(Box::new(<WithSize_3_>::default())) as *mut WithSize_3_);
            let mut ptr: Option<Box<WithSize_3_>> = Some(Box::from_raw((x).cast_const()));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut WithSize_3_))
                    == ((x).cast_const()))
            );
        }
        {
            let mut x: *mut WithSize_3_ =
                (Box::leak(Box::new(<WithSize_3_>::default())) as *mut WithSize_3_);
            let ptr: Option<Box<WithSize_3_>> = Some(Box::from_raw((x).cast_const()));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut WithSize_3_))
                    == ((x).cast_const()))
            );
        }
    }
    {
        {
            let mut x: *mut WithSize_3_ = Box::leak(
                (0..3_usize)
                    .map(|_| <WithSize_3_>::default())
                    .collect::<Box<[WithSize_3_]>>(),
            )
            .as_mut_ptr();
            let mut ptr: Option<Box<[WithSize_3_]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == (x))
            );
        }
        {
            let mut x: *mut WithSize_3_ = Box::leak(
                (0..3_usize)
                    .map(|_| <WithSize_3_>::default())
                    .collect::<Box<[WithSize_3_]>>(),
            )
            .as_mut_ptr();
            let ptr: Option<Box<[WithSize_3_]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == (x))
            );
        }
    }
    {
        {
            let mut x: *mut WithSize_3_ = Box::leak(
                (0..3_usize)
                    .map(|_| <WithSize_3_>::default())
                    .collect::<Box<[WithSize_3_]>>(),
            )
            .as_mut_ptr();
            let mut ptr: Option<Box<[WithSize_3_]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == ((x).cast_const()))
            );
        }
        {
            let mut x: *mut WithSize_3_ = Box::leak(
                (0..3_usize)
                    .map(|_| <WithSize_3_>::default())
                    .collect::<Box<[WithSize_3_]>>(),
            )
            .as_mut_ptr();
            let ptr: Option<Box<[WithSize_3_]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == ((x).cast_const()))
            );
        }
    }
}
pub unsafe fn test_basic_23() {
    {
        {
            let mut x: *mut WithSize_4_ =
                (Box::leak(Box::new(<WithSize_4_>::default())) as *mut WithSize_4_);
            let mut ptr: Option<Box<WithSize_4_>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut WithSize_4_))
                    == (x))
            );
        }
        {
            let mut x: *mut WithSize_4_ =
                (Box::leak(Box::new(<WithSize_4_>::default())) as *mut WithSize_4_);
            let ptr: Option<Box<WithSize_4_>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut WithSize_4_))
                    == (x))
            );
        }
    }
    {
        {
            let mut x: *mut WithSize_4_ =
                (Box::leak(Box::new(<WithSize_4_>::default())) as *mut WithSize_4_);
            let mut ptr: Option<Box<WithSize_4_>> = Some(Box::from_raw((x).cast_const()));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut WithSize_4_))
                    == ((x).cast_const()))
            );
        }
        {
            let mut x: *mut WithSize_4_ =
                (Box::leak(Box::new(<WithSize_4_>::default())) as *mut WithSize_4_);
            let ptr: Option<Box<WithSize_4_>> = Some(Box::from_raw((x).cast_const()));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut WithSize_4_))
                    == ((x).cast_const()))
            );
        }
    }
    {
        {
            let mut x: *mut WithSize_4_ = Box::leak(
                (0..3_usize)
                    .map(|_| <WithSize_4_>::default())
                    .collect::<Box<[WithSize_4_]>>(),
            )
            .as_mut_ptr();
            let mut ptr: Option<Box<[WithSize_4_]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == (x))
            );
        }
        {
            let mut x: *mut WithSize_4_ = Box::leak(
                (0..3_usize)
                    .map(|_| <WithSize_4_>::default())
                    .collect::<Box<[WithSize_4_]>>(),
            )
            .as_mut_ptr();
            let ptr: Option<Box<[WithSize_4_]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == (x))
            );
        }
    }
    {
        {
            let mut x: *mut WithSize_4_ = Box::leak(
                (0..3_usize)
                    .map(|_| <WithSize_4_>::default())
                    .collect::<Box<[WithSize_4_]>>(),
            )
            .as_mut_ptr();
            let mut ptr: Option<Box<[WithSize_4_]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == ((x).cast_const()))
            );
        }
        {
            let mut x: *mut WithSize_4_ = Box::leak(
                (0..3_usize)
                    .map(|_| <WithSize_4_>::default())
                    .collect::<Box<[WithSize_4_]>>(),
            )
            .as_mut_ptr();
            let ptr: Option<Box<[WithSize_4_]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == ((x).cast_const()))
            );
        }
    }
}
pub unsafe fn test_basic_24() {
    {
        {
            let mut x: *mut WithSize_8_ =
                (Box::leak(Box::new(<WithSize_8_>::default())) as *mut WithSize_8_);
            let mut ptr: Option<Box<WithSize_8_>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut WithSize_8_))
                    == (x))
            );
        }
        {
            let mut x: *mut WithSize_8_ =
                (Box::leak(Box::new(<WithSize_8_>::default())) as *mut WithSize_8_);
            let ptr: Option<Box<WithSize_8_>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut WithSize_8_))
                    == (x))
            );
        }
    }
    {
        {
            let mut x: *mut WithSize_8_ =
                (Box::leak(Box::new(<WithSize_8_>::default())) as *mut WithSize_8_);
            let mut ptr: Option<Box<WithSize_8_>> = Some(Box::from_raw((x).cast_const()));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut WithSize_8_))
                    == ((x).cast_const()))
            );
        }
        {
            let mut x: *mut WithSize_8_ =
                (Box::leak(Box::new(<WithSize_8_>::default())) as *mut WithSize_8_);
            let ptr: Option<Box<WithSize_8_>> = Some(Box::from_raw((x).cast_const()));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut WithSize_8_))
                    == ((x).cast_const()))
            );
        }
    }
    {
        {
            let mut x: *mut WithSize_8_ = Box::leak(
                (0..3_usize)
                    .map(|_| <WithSize_8_>::default())
                    .collect::<Box<[WithSize_8_]>>(),
            )
            .as_mut_ptr();
            let mut ptr: Option<Box<[WithSize_8_]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == (x))
            );
        }
        {
            let mut x: *mut WithSize_8_ = Box::leak(
                (0..3_usize)
                    .map(|_| <WithSize_8_>::default())
                    .collect::<Box<[WithSize_8_]>>(),
            )
            .as_mut_ptr();
            let ptr: Option<Box<[WithSize_8_]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == (x))
            );
        }
    }
    {
        {
            let mut x: *mut WithSize_8_ = Box::leak(
                (0..3_usize)
                    .map(|_| <WithSize_8_>::default())
                    .collect::<Box<[WithSize_8_]>>(),
            )
            .as_mut_ptr();
            let mut ptr: Option<Box<[WithSize_8_]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == ((x).cast_const()))
            );
        }
        {
            let mut x: *mut WithSize_8_ = Box::leak(
                (0..3_usize)
                    .map(|_| <WithSize_8_>::default())
                    .collect::<Box<[WithSize_8_]>>(),
            )
            .as_mut_ptr();
            let ptr: Option<Box<[WithSize_8_]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == ((x).cast_const()))
            );
        }
    }
}
pub unsafe fn test_basic_25() {
    {
        {
            let mut x: *mut WithSize_16_ =
                (Box::leak(Box::new(<WithSize_16_>::default())) as *mut WithSize_16_);
            let mut ptr: Option<Box<WithSize_16_>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut WithSize_16_))
                    == (x))
            );
        }
        {
            let mut x: *mut WithSize_16_ =
                (Box::leak(Box::new(<WithSize_16_>::default())) as *mut WithSize_16_);
            let ptr: Option<Box<WithSize_16_>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut WithSize_16_))
                    == (x))
            );
        }
    }
    {
        {
            let mut x: *mut WithSize_16_ =
                (Box::leak(Box::new(<WithSize_16_>::default())) as *mut WithSize_16_);
            let mut ptr: Option<Box<WithSize_16_>> = Some(Box::from_raw((x).cast_const()));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut WithSize_16_))
                    == ((x).cast_const()))
            );
        }
        {
            let mut x: *mut WithSize_16_ =
                (Box::leak(Box::new(<WithSize_16_>::default())) as *mut WithSize_16_);
            let ptr: Option<Box<WithSize_16_>> = Some(Box::from_raw((x).cast_const()));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut WithSize_16_))
                    == ((x).cast_const()))
            );
        }
    }
    {
        {
            let mut x: *mut WithSize_16_ = Box::leak(
                (0..3_usize)
                    .map(|_| <WithSize_16_>::default())
                    .collect::<Box<[WithSize_16_]>>(),
            )
            .as_mut_ptr();
            let mut ptr: Option<Box<[WithSize_16_]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == (x))
            );
        }
        {
            let mut x: *mut WithSize_16_ = Box::leak(
                (0..3_usize)
                    .map(|_| <WithSize_16_>::default())
                    .collect::<Box<[WithSize_16_]>>(),
            )
            .as_mut_ptr();
            let ptr: Option<Box<[WithSize_16_]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == (x))
            );
        }
    }
    {
        {
            let mut x: *mut WithSize_16_ = Box::leak(
                (0..3_usize)
                    .map(|_| <WithSize_16_>::default())
                    .collect::<Box<[WithSize_16_]>>(),
            )
            .as_mut_ptr();
            let mut ptr: Option<Box<[WithSize_16_]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == ((x).cast_const()))
            );
        }
        {
            let mut x: *mut WithSize_16_ = Box::leak(
                (0..3_usize)
                    .map(|_| <WithSize_16_>::default())
                    .collect::<Box<[WithSize_16_]>>(),
            )
            .as_mut_ptr();
            let ptr: Option<Box<[WithSize_16_]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == ((x).cast_const()))
            );
        }
    }
}
pub unsafe fn test_basic_26() {
    {
        {
            let mut x: *mut WithSize_256_ =
                (Box::leak(Box::new(<WithSize_256_>::default())) as *mut WithSize_256_);
            let mut ptr: Option<Box<WithSize_256_>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut WithSize_256_))
                    == (x))
            );
        }
        {
            let mut x: *mut WithSize_256_ =
                (Box::leak(Box::new(<WithSize_256_>::default())) as *mut WithSize_256_);
            let ptr: Option<Box<WithSize_256_>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut WithSize_256_))
                    == (x))
            );
        }
    }
    {
        {
            let mut x: *mut WithSize_256_ =
                (Box::leak(Box::new(<WithSize_256_>::default())) as *mut WithSize_256_);
            let mut ptr: Option<Box<WithSize_256_>> = Some(Box::from_raw((x).cast_const()));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut WithSize_256_))
                    == ((x).cast_const()))
            );
        }
        {
            let mut x: *mut WithSize_256_ =
                (Box::leak(Box::new(<WithSize_256_>::default())) as *mut WithSize_256_);
            let ptr: Option<Box<WithSize_256_>> = Some(Box::from_raw((x).cast_const()));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |v| v as *mut WithSize_256_))
                    == ((x).cast_const()))
            );
        }
    }
    {
        {
            let mut x: *mut WithSize_256_ = Box::leak(
                (0..3_usize)
                    .map(|_| <WithSize_256_>::default())
                    .collect::<Box<[WithSize_256_]>>(),
            )
            .as_mut_ptr();
            let mut ptr: Option<Box<[WithSize_256_]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == (x))
            );
        }
        {
            let mut x: *mut WithSize_256_ = Box::leak(
                (0..3_usize)
                    .map(|_| <WithSize_256_>::default())
                    .collect::<Box<[WithSize_256_]>>(),
            )
            .as_mut_ptr();
            let ptr: Option<Box<[WithSize_256_]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == (x))
            );
        }
    }
    {
        {
            let mut x: *mut WithSize_256_ = Box::leak(
                (0..3_usize)
                    .map(|_| <WithSize_256_>::default())
                    .collect::<Box<[WithSize_256_]>>(),
            )
            .as_mut_ptr();
            let mut ptr: Option<Box<[WithSize_256_]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == ((x).cast_const()))
            );
        }
        {
            let mut x: *mut WithSize_256_ = Box::leak(
                (0..3_usize)
                    .map(|_| <WithSize_256_>::default())
                    .collect::<Box<[WithSize_256_]>>(),
            )
            .as_mut_ptr();
            let ptr: Option<Box<[WithSize_256_]>> = Some(Box::from_raw(x));
            const _: () = assert!(true, "");;
            assert!(
                ((ptr
                    .as_deref_mut()
                    .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
                    == ((x).cast_const()))
            );
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct WithSize_1_ {
    pub padding: [libc::c_char; 1],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct WithSize_2_ {
    pub padding: [libc::c_char; 2],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct WithSize_3_ {
    pub padding: [libc::c_char; 3],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct WithSize_4_ {
    pub padding: [libc::c_char; 4],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct WithSize_8_ {
    pub padding: [libc::c_char; 8],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct WithSize_16_ {
    pub padding: [libc::c_char; 16],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct WithSize_256_ {
    pub padding: [libc::c_char; 256],
}
impl Default for WithSize_256_ {
    fn default() -> Self {
        unsafe {
            WithSize_256_ {
                padding: [(0 as libc::c_char); 256],
            }
        }
    }
}
pub unsafe fn test_27() -> bool {
    (unsafe { test_basic_18() });
    (unsafe { test_basic_19() });
    (unsafe { test_basic_20() });
    (unsafe { test_basic_21() });
    (unsafe { test_basic_22() });
    (unsafe { test_basic_23() });
    (unsafe { test_basic_24() });
    (unsafe { test_basic_25() });
    (unsafe { test_basic_26() });
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
    (unsafe { test_27() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
