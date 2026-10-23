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
pub struct test_macros_detail_is_same_char_ptr__char_ptr_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_char_ptr__const_char_ptr_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_WithSize_1__ptr__WithSize_1__ptr_ {}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_WithSize_1__ptr__const_WithSize_1__ptr_ {}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_WithSize_2__ptr__WithSize_2__ptr_ {}
pub type anon_7 = ();
pub const anon_7_value: anon_7 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_WithSize_2__ptr__const_WithSize_2__ptr_ {}
pub type anon_8 = ();
pub const anon_8_value: anon_8 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_WithSize_3__ptr__WithSize_3__ptr_ {}
pub type anon_9 = ();
pub const anon_9_value: anon_9 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_WithSize_3__ptr__const_WithSize_3__ptr_ {}
pub type anon_10 = ();
pub const anon_10_value: anon_10 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_WithSize_4__ptr__WithSize_4__ptr_ {}
pub type anon_11 = ();
pub const anon_11_value: anon_11 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_WithSize_4__ptr__const_WithSize_4__ptr_ {}
pub type anon_12 = ();
pub const anon_12_value: anon_12 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_WithSize_8__ptr__WithSize_8__ptr_ {}
pub type anon_13 = ();
pub const anon_13_value: anon_13 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_WithSize_8__ptr__const_WithSize_8__ptr_ {}
pub type anon_14 = ();
pub const anon_14_value: anon_14 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_WithSize_16__ptr__WithSize_16__ptr_ {}
pub type anon_15 = ();
pub const anon_15_value: anon_15 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_WithSize_16__ptr__const_WithSize_16__ptr_ {}
pub type anon_16 = ();
pub const anon_16_value: anon_16 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_WithSize_256__ptr__WithSize_256__ptr_ {}
pub type anon_17 = ();
pub const anon_17_value: anon_17 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_WithSize_256__ptr__const_WithSize_256__ptr_ {}
pub unsafe fn test_basic_18() {
    {
        {
            let mut x: *mut () = (Box::leak(Box::new(unimplemented!())) as *mut ());
            let mut ptr: () = std_unique_ptr_char__std_default_delete_char__::new({ x });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut () = (Box::leak(Box::new(unimplemented!())) as *mut ());
            let ptr: () = std_unique_ptr_char__std_default_delete_char__::new({ x });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
    {
        {
            let mut x: *mut () = (Box::leak(Box::new(unimplemented!())) as *mut ());
            let mut ptr: () = std_unique_ptr_const_char__std_default_delete_const_char__::new({
                (x).cast_const()
            });
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut () = (Box::leak(Box::new(unimplemented!())) as *mut ());
            let ptr: () = std_unique_ptr_const_char__std_default_delete_const_char__::new({
                (x).cast_const()
            });
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
    {
        {
            let mut x: *mut () =
                Box::leak((0..3_()).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
            let mut ptr: () =
                std_unique_ptr_chararrarr__std_default_delete_chararrarr__::new({ x });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut () =
                Box::leak((0..3_()).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
            let ptr: () = std_unique_ptr_chararrarr__std_default_delete_chararrarr__::new({ x });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
    {
        {
            let mut x: *mut () =
                Box::leak((0..3_()).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
            let mut ptr: () =
                std_unique_ptr_const_chararrarr__std_default_delete_const_chararrarr__::new({ x });
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut () =
                Box::leak((0..3_()).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
            let ptr: () =
                std_unique_ptr_const_chararrarr__std_default_delete_const_chararrarr__::new({ x });
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
}
pub unsafe fn test_basic_19() {
    {
        {
            let mut x: *mut () = (Box::leak(Box::new(unimplemented!())) as *mut ());
            let mut ptr: () = std_unique_ptr_int__std_default_delete_int__::new({ x });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut () = (Box::leak(Box::new(unimplemented!())) as *mut ());
            let ptr: () = std_unique_ptr_int__std_default_delete_int__::new({ x });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
    {
        {
            let mut x: *mut () = (Box::leak(Box::new(unimplemented!())) as *mut ());
            let mut ptr: () =
                std_unique_ptr_const_int__std_default_delete_const_int__::new({ (x).cast_const() });
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut () = (Box::leak(Box::new(unimplemented!())) as *mut ());
            let ptr: () =
                std_unique_ptr_const_int__std_default_delete_const_int__::new({ (x).cast_const() });
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
    {
        {
            let mut x: *mut () =
                Box::leak((0..3_()).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
            let mut ptr: () = std_unique_ptr_intarrarr__std_default_delete_intarrarr__::new({ x });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut () =
                Box::leak((0..3_()).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
            let ptr: () = std_unique_ptr_intarrarr__std_default_delete_intarrarr__::new({ x });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
    {
        {
            let mut x: *mut () =
                Box::leak((0..3_()).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
            let mut ptr: () =
                std_unique_ptr_const_intarrarr__std_default_delete_const_intarrarr__::new({ x });
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut () =
                Box::leak((0..3_()).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
            let ptr: () =
                std_unique_ptr_const_intarrarr__std_default_delete_const_intarrarr__::new({ x });
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
}
pub unsafe fn test_basic_20() {
    {
        {
            let mut x: *mut WithSize_1_ =
                (Box::leak(Box::new(<WithSize_1_>::default())) as *mut WithSize_1_);
            let mut ptr: () =
                std_unique_ptr_WithSize_1___std_default_delete_WithSize_1___::new({ x });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut WithSize_1_ =
                (Box::leak(Box::new(<WithSize_1_>::default())) as *mut WithSize_1_);
            let ptr: () = std_unique_ptr_WithSize_1___std_default_delete_WithSize_1___::new({ x });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
    {
        {
            let mut x: *mut WithSize_1_ =
                (Box::leak(Box::new(<WithSize_1_>::default())) as *mut WithSize_1_);
            let mut ptr: () =
                std_unique_ptr_const_WithSize_1___std_default_delete_const_WithSize_1___::new({
                    (x).cast_const()
                });
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut WithSize_1_ =
                (Box::leak(Box::new(<WithSize_1_>::default())) as *mut WithSize_1_);
            let ptr: () =
                std_unique_ptr_const_WithSize_1___std_default_delete_const_WithSize_1___::new({
                    (x).cast_const()
                });
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
    {
        {
            let mut x: *mut WithSize_1_ = Box::leak(
                (0..3_())
                    .map(|_| <WithSize_1_>::default())
                    .collect::<Box<[WithSize_1_]>>(),
            )
            .as_mut_ptr();
            let mut ptr: () =
                std_unique_ptr_WithSize_1_arrarr__std_default_delete_WithSize_1_arrarr__::new({
                    x
                });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut WithSize_1_ = Box::leak(
                (0..3_())
                    .map(|_| <WithSize_1_>::default())
                    .collect::<Box<[WithSize_1_]>>(),
            )
            .as_mut_ptr();
            let ptr: () =
                std_unique_ptr_WithSize_1_arrarr__std_default_delete_WithSize_1_arrarr__::new({
                    x
                });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
    {
        {
            let mut x: *mut WithSize_1_ = Box::leak(
                (0..3_())
                    .map(|_| <WithSize_1_>::default())
                    .collect::<Box<[WithSize_1_]>>(),
            )
            .as_mut_ptr();
            let mut ptr : () = std_unique_ptr_const_WithSize_1_arrarr__std_default_delete_const_WithSize_1_arrarr__ :: new ( { x } , )  ;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut WithSize_1_ = Box::leak(
                (0..3_())
                    .map(|_| <WithSize_1_>::default())
                    .collect::<Box<[WithSize_1_]>>(),
            )
            .as_mut_ptr();
            let ptr : () = std_unique_ptr_const_WithSize_1_arrarr__std_default_delete_const_WithSize_1_arrarr__ :: new ( { x } , )  ;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
}
pub unsafe fn test_basic_21() {
    {
        {
            let mut x: *mut WithSize_2_ =
                (Box::leak(Box::new(<WithSize_2_>::default())) as *mut WithSize_2_);
            let mut ptr: () =
                std_unique_ptr_WithSize_2___std_default_delete_WithSize_2___::new({ x });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut WithSize_2_ =
                (Box::leak(Box::new(<WithSize_2_>::default())) as *mut WithSize_2_);
            let ptr: () = std_unique_ptr_WithSize_2___std_default_delete_WithSize_2___::new({ x });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
    {
        {
            let mut x: *mut WithSize_2_ =
                (Box::leak(Box::new(<WithSize_2_>::default())) as *mut WithSize_2_);
            let mut ptr: () =
                std_unique_ptr_const_WithSize_2___std_default_delete_const_WithSize_2___::new({
                    (x).cast_const()
                });
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut WithSize_2_ =
                (Box::leak(Box::new(<WithSize_2_>::default())) as *mut WithSize_2_);
            let ptr: () =
                std_unique_ptr_const_WithSize_2___std_default_delete_const_WithSize_2___::new({
                    (x).cast_const()
                });
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
    {
        {
            let mut x: *mut WithSize_2_ = Box::leak(
                (0..3_())
                    .map(|_| <WithSize_2_>::default())
                    .collect::<Box<[WithSize_2_]>>(),
            )
            .as_mut_ptr();
            let mut ptr: () =
                std_unique_ptr_WithSize_2_arrarr__std_default_delete_WithSize_2_arrarr__::new({
                    x
                });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut WithSize_2_ = Box::leak(
                (0..3_())
                    .map(|_| <WithSize_2_>::default())
                    .collect::<Box<[WithSize_2_]>>(),
            )
            .as_mut_ptr();
            let ptr: () =
                std_unique_ptr_WithSize_2_arrarr__std_default_delete_WithSize_2_arrarr__::new({
                    x
                });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
    {
        {
            let mut x: *mut WithSize_2_ = Box::leak(
                (0..3_())
                    .map(|_| <WithSize_2_>::default())
                    .collect::<Box<[WithSize_2_]>>(),
            )
            .as_mut_ptr();
            let mut ptr : () = std_unique_ptr_const_WithSize_2_arrarr__std_default_delete_const_WithSize_2_arrarr__ :: new ( { x } , )  ;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut WithSize_2_ = Box::leak(
                (0..3_())
                    .map(|_| <WithSize_2_>::default())
                    .collect::<Box<[WithSize_2_]>>(),
            )
            .as_mut_ptr();
            let ptr : () = std_unique_ptr_const_WithSize_2_arrarr__std_default_delete_const_WithSize_2_arrarr__ :: new ( { x } , )  ;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
}
pub unsafe fn test_basic_22() {
    {
        {
            let mut x: *mut WithSize_3_ =
                (Box::leak(Box::new(<WithSize_3_>::default())) as *mut WithSize_3_);
            let mut ptr: () =
                std_unique_ptr_WithSize_3___std_default_delete_WithSize_3___::new({ x });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut WithSize_3_ =
                (Box::leak(Box::new(<WithSize_3_>::default())) as *mut WithSize_3_);
            let ptr: () = std_unique_ptr_WithSize_3___std_default_delete_WithSize_3___::new({ x });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
    {
        {
            let mut x: *mut WithSize_3_ =
                (Box::leak(Box::new(<WithSize_3_>::default())) as *mut WithSize_3_);
            let mut ptr: () =
                std_unique_ptr_const_WithSize_3___std_default_delete_const_WithSize_3___::new({
                    (x).cast_const()
                });
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut WithSize_3_ =
                (Box::leak(Box::new(<WithSize_3_>::default())) as *mut WithSize_3_);
            let ptr: () =
                std_unique_ptr_const_WithSize_3___std_default_delete_const_WithSize_3___::new({
                    (x).cast_const()
                });
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
    {
        {
            let mut x: *mut WithSize_3_ = Box::leak(
                (0..3_())
                    .map(|_| <WithSize_3_>::default())
                    .collect::<Box<[WithSize_3_]>>(),
            )
            .as_mut_ptr();
            let mut ptr: () =
                std_unique_ptr_WithSize_3_arrarr__std_default_delete_WithSize_3_arrarr__::new({
                    x
                });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut WithSize_3_ = Box::leak(
                (0..3_())
                    .map(|_| <WithSize_3_>::default())
                    .collect::<Box<[WithSize_3_]>>(),
            )
            .as_mut_ptr();
            let ptr: () =
                std_unique_ptr_WithSize_3_arrarr__std_default_delete_WithSize_3_arrarr__::new({
                    x
                });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
    {
        {
            let mut x: *mut WithSize_3_ = Box::leak(
                (0..3_())
                    .map(|_| <WithSize_3_>::default())
                    .collect::<Box<[WithSize_3_]>>(),
            )
            .as_mut_ptr();
            let mut ptr : () = std_unique_ptr_const_WithSize_3_arrarr__std_default_delete_const_WithSize_3_arrarr__ :: new ( { x } , )  ;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut WithSize_3_ = Box::leak(
                (0..3_())
                    .map(|_| <WithSize_3_>::default())
                    .collect::<Box<[WithSize_3_]>>(),
            )
            .as_mut_ptr();
            let ptr : () = std_unique_ptr_const_WithSize_3_arrarr__std_default_delete_const_WithSize_3_arrarr__ :: new ( { x } , )  ;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
}
pub unsafe fn test_basic_23() {
    {
        {
            let mut x: *mut WithSize_4_ =
                (Box::leak(Box::new(<WithSize_4_>::default())) as *mut WithSize_4_);
            let mut ptr: () =
                std_unique_ptr_WithSize_4___std_default_delete_WithSize_4___::new({ x });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut WithSize_4_ =
                (Box::leak(Box::new(<WithSize_4_>::default())) as *mut WithSize_4_);
            let ptr: () = std_unique_ptr_WithSize_4___std_default_delete_WithSize_4___::new({ x });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
    {
        {
            let mut x: *mut WithSize_4_ =
                (Box::leak(Box::new(<WithSize_4_>::default())) as *mut WithSize_4_);
            let mut ptr: () =
                std_unique_ptr_const_WithSize_4___std_default_delete_const_WithSize_4___::new({
                    (x).cast_const()
                });
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut WithSize_4_ =
                (Box::leak(Box::new(<WithSize_4_>::default())) as *mut WithSize_4_);
            let ptr: () =
                std_unique_ptr_const_WithSize_4___std_default_delete_const_WithSize_4___::new({
                    (x).cast_const()
                });
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
    {
        {
            let mut x: *mut WithSize_4_ = Box::leak(
                (0..3_())
                    .map(|_| <WithSize_4_>::default())
                    .collect::<Box<[WithSize_4_]>>(),
            )
            .as_mut_ptr();
            let mut ptr: () =
                std_unique_ptr_WithSize_4_arrarr__std_default_delete_WithSize_4_arrarr__::new({
                    x
                });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut WithSize_4_ = Box::leak(
                (0..3_())
                    .map(|_| <WithSize_4_>::default())
                    .collect::<Box<[WithSize_4_]>>(),
            )
            .as_mut_ptr();
            let ptr: () =
                std_unique_ptr_WithSize_4_arrarr__std_default_delete_WithSize_4_arrarr__::new({
                    x
                });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
    {
        {
            let mut x: *mut WithSize_4_ = Box::leak(
                (0..3_())
                    .map(|_| <WithSize_4_>::default())
                    .collect::<Box<[WithSize_4_]>>(),
            )
            .as_mut_ptr();
            let mut ptr : () = std_unique_ptr_const_WithSize_4_arrarr__std_default_delete_const_WithSize_4_arrarr__ :: new ( { x } , )  ;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut WithSize_4_ = Box::leak(
                (0..3_())
                    .map(|_| <WithSize_4_>::default())
                    .collect::<Box<[WithSize_4_]>>(),
            )
            .as_mut_ptr();
            let ptr : () = std_unique_ptr_const_WithSize_4_arrarr__std_default_delete_const_WithSize_4_arrarr__ :: new ( { x } , )  ;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
}
pub unsafe fn test_basic_24() {
    {
        {
            let mut x: *mut WithSize_8_ =
                (Box::leak(Box::new(<WithSize_8_>::default())) as *mut WithSize_8_);
            let mut ptr: () =
                std_unique_ptr_WithSize_8___std_default_delete_WithSize_8___::new({ x });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut WithSize_8_ =
                (Box::leak(Box::new(<WithSize_8_>::default())) as *mut WithSize_8_);
            let ptr: () = std_unique_ptr_WithSize_8___std_default_delete_WithSize_8___::new({ x });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
    {
        {
            let mut x: *mut WithSize_8_ =
                (Box::leak(Box::new(<WithSize_8_>::default())) as *mut WithSize_8_);
            let mut ptr: () =
                std_unique_ptr_const_WithSize_8___std_default_delete_const_WithSize_8___::new({
                    (x).cast_const()
                });
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut WithSize_8_ =
                (Box::leak(Box::new(<WithSize_8_>::default())) as *mut WithSize_8_);
            let ptr: () =
                std_unique_ptr_const_WithSize_8___std_default_delete_const_WithSize_8___::new({
                    (x).cast_const()
                });
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
    {
        {
            let mut x: *mut WithSize_8_ = Box::leak(
                (0..3_())
                    .map(|_| <WithSize_8_>::default())
                    .collect::<Box<[WithSize_8_]>>(),
            )
            .as_mut_ptr();
            let mut ptr: () =
                std_unique_ptr_WithSize_8_arrarr__std_default_delete_WithSize_8_arrarr__::new({
                    x
                });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut WithSize_8_ = Box::leak(
                (0..3_())
                    .map(|_| <WithSize_8_>::default())
                    .collect::<Box<[WithSize_8_]>>(),
            )
            .as_mut_ptr();
            let ptr: () =
                std_unique_ptr_WithSize_8_arrarr__std_default_delete_WithSize_8_arrarr__::new({
                    x
                });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
    {
        {
            let mut x: *mut WithSize_8_ = Box::leak(
                (0..3_())
                    .map(|_| <WithSize_8_>::default())
                    .collect::<Box<[WithSize_8_]>>(),
            )
            .as_mut_ptr();
            let mut ptr : () = std_unique_ptr_const_WithSize_8_arrarr__std_default_delete_const_WithSize_8_arrarr__ :: new ( { x } , )  ;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut WithSize_8_ = Box::leak(
                (0..3_())
                    .map(|_| <WithSize_8_>::default())
                    .collect::<Box<[WithSize_8_]>>(),
            )
            .as_mut_ptr();
            let ptr : () = std_unique_ptr_const_WithSize_8_arrarr__std_default_delete_const_WithSize_8_arrarr__ :: new ( { x } , )  ;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
}
pub unsafe fn test_basic_25() {
    {
        {
            let mut x: *mut WithSize_16_ =
                (Box::leak(Box::new(<WithSize_16_>::default())) as *mut WithSize_16_);
            let mut ptr: () =
                std_unique_ptr_WithSize_16___std_default_delete_WithSize_16___::new({ x });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut WithSize_16_ =
                (Box::leak(Box::new(<WithSize_16_>::default())) as *mut WithSize_16_);
            let ptr: () =
                std_unique_ptr_WithSize_16___std_default_delete_WithSize_16___::new({ x });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
    {
        {
            let mut x: *mut WithSize_16_ =
                (Box::leak(Box::new(<WithSize_16_>::default())) as *mut WithSize_16_);
            let mut ptr: () =
                std_unique_ptr_const_WithSize_16___std_default_delete_const_WithSize_16___::new({
                    (x).cast_const()
                });
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut WithSize_16_ =
                (Box::leak(Box::new(<WithSize_16_>::default())) as *mut WithSize_16_);
            let ptr: () =
                std_unique_ptr_const_WithSize_16___std_default_delete_const_WithSize_16___::new({
                    (x).cast_const()
                });
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
    {
        {
            let mut x: *mut WithSize_16_ = Box::leak(
                (0..3_())
                    .map(|_| <WithSize_16_>::default())
                    .collect::<Box<[WithSize_16_]>>(),
            )
            .as_mut_ptr();
            let mut ptr: () =
                std_unique_ptr_WithSize_16_arrarr__std_default_delete_WithSize_16_arrarr__::new({
                    x
                });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut WithSize_16_ = Box::leak(
                (0..3_())
                    .map(|_| <WithSize_16_>::default())
                    .collect::<Box<[WithSize_16_]>>(),
            )
            .as_mut_ptr();
            let ptr: () =
                std_unique_ptr_WithSize_16_arrarr__std_default_delete_WithSize_16_arrarr__::new({
                    x
                });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
    {
        {
            let mut x: *mut WithSize_16_ = Box::leak(
                (0..3_())
                    .map(|_| <WithSize_16_>::default())
                    .collect::<Box<[WithSize_16_]>>(),
            )
            .as_mut_ptr();
            let mut ptr : () = std_unique_ptr_const_WithSize_16_arrarr__std_default_delete_const_WithSize_16_arrarr__ :: new ( { x } , )  ;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut WithSize_16_ = Box::leak(
                (0..3_())
                    .map(|_| <WithSize_16_>::default())
                    .collect::<Box<[WithSize_16_]>>(),
            )
            .as_mut_ptr();
            let ptr : () = std_unique_ptr_const_WithSize_16_arrarr__std_default_delete_const_WithSize_16_arrarr__ :: new ( { x } , )  ;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
}
pub unsafe fn test_basic_26() {
    {
        {
            let mut x: *mut WithSize_256_ =
                (Box::leak(Box::new(<WithSize_256_>::default())) as *mut WithSize_256_);
            let mut ptr: () =
                std_unique_ptr_WithSize_256___std_default_delete_WithSize_256___::new({ x });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut WithSize_256_ =
                (Box::leak(Box::new(<WithSize_256_>::default())) as *mut WithSize_256_);
            let ptr: () =
                std_unique_ptr_WithSize_256___std_default_delete_WithSize_256___::new({ x });
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
    {
        {
            let mut x: *mut WithSize_256_ =
                (Box::leak(Box::new(<WithSize_256_>::default())) as *mut WithSize_256_);
            let mut ptr: () =
                std_unique_ptr_const_WithSize_256___std_default_delete_const_WithSize_256___::new(
                    { (x).cast_const() },
                );
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut WithSize_256_ =
                (Box::leak(Box::new(<WithSize_256_>::default())) as *mut WithSize_256_);
            let ptr: () =
                std_unique_ptr_const_WithSize_256___std_default_delete_const_WithSize_256___::new(
                    { (x).cast_const() },
                );
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
    {
        {
            let mut x: *mut WithSize_256_ = Box::leak(
                (0..3_())
                    .map(|_| <WithSize_256_>::default())
                    .collect::<Box<[WithSize_256_]>>(),
            )
            .as_mut_ptr();
            let mut ptr: () =
                std_unique_ptr_WithSize_256_arrarr__std_default_delete_WithSize_256_arrarr__::new(
                    { x },
                );
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut WithSize_256_ = Box::leak(
                (0..3_())
                    .map(|_| <WithSize_256_>::default())
                    .collect::<Box<[WithSize_256_]>>(),
            )
            .as_mut_ptr();
            let ptr: () =
                std_unique_ptr_WithSize_256_arrarr__std_default_delete_WithSize_256_arrarr__::new(
                    { x },
                );
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
    {
        {
            let mut x: *mut WithSize_256_ = Box::leak(
                (0..3_())
                    .map(|_| <WithSize_256_>::default())
                    .collect::<Box<[WithSize_256_]>>(),
            )
            .as_mut_ptr();
            let mut ptr : () = std_unique_ptr_const_WithSize_256_arrarr__std_default_delete_const_WithSize_256_arrarr__ :: new ( { x } , )  ;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let mut x: *mut WithSize_256_ = Box::leak(
                (0..3_())
                    .map(|_| <WithSize_256_>::default())
                    .collect::<Box<[WithSize_256_]>>(),
            )
            .as_mut_ptr();
            let ptr : () = std_unique_ptr_const_WithSize_256_arrarr__std_default_delete_const_WithSize_256_arrarr__ :: new ( { x } , )  ;
            const _: () = assert!(true, "");;
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct WithSize_1_ {
    pub padding: [(); 1],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct WithSize_2_ {
    pub padding: [(); 2],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct WithSize_3_ {
    pub padding: [(); 3],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct WithSize_4_ {
    pub padding: [(); 4],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct WithSize_8_ {
    pub padding: [(); 8],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct WithSize_16_ {
    pub padding: [(); 16],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct WithSize_256_ {
    pub padding: [(); 256],
}
impl Default for WithSize_256_ {
    fn default() -> Self {
        unsafe {
            WithSize_256_ {
                padding: [unimplemented!(); 256],
            }
        }
    }
}
pub unsafe fn test_27() -> () {
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_27() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
