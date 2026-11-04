extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_char_ptr__char_ptr_ {}
impl CppMove for test_macros_detail_is_same_char_ptr__char_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_char_ptr__const_char_ptr_ {}
impl CppMove for test_macros_detail_is_same_const_char_ptr__const_char_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
impl CppMove for test_macros_detail_is_same_int_ptr__int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {}
impl CppMove for test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_4 = u32;
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_WithSize_1__ptr__WithSize_1__ptr_ {}
impl CppMove for test_macros_detail_is_same_WithSize_1__ptr__WithSize_1__ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_5 = u32;
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_WithSize_1__ptr__const_WithSize_1__ptr_ {}
impl CppMove for test_macros_detail_is_same_const_WithSize_1__ptr__const_WithSize_1__ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_6 = u32;
pub const anon_6_value: anon_6 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_WithSize_2__ptr__WithSize_2__ptr_ {}
impl CppMove for test_macros_detail_is_same_WithSize_2__ptr__WithSize_2__ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_7 = u32;
pub const anon_7_value: anon_7 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_WithSize_2__ptr__const_WithSize_2__ptr_ {}
impl CppMove for test_macros_detail_is_same_const_WithSize_2__ptr__const_WithSize_2__ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_8 = u32;
pub const anon_8_value: anon_8 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_WithSize_3__ptr__WithSize_3__ptr_ {}
impl CppMove for test_macros_detail_is_same_WithSize_3__ptr__WithSize_3__ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_9 = u32;
pub const anon_9_value: anon_9 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_WithSize_3__ptr__const_WithSize_3__ptr_ {}
impl CppMove for test_macros_detail_is_same_const_WithSize_3__ptr__const_WithSize_3__ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_10 = u32;
pub const anon_10_value: anon_10 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_WithSize_4__ptr__WithSize_4__ptr_ {}
impl CppMove for test_macros_detail_is_same_WithSize_4__ptr__WithSize_4__ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_11 = u32;
pub const anon_11_value: anon_11 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_WithSize_4__ptr__const_WithSize_4__ptr_ {}
impl CppMove for test_macros_detail_is_same_const_WithSize_4__ptr__const_WithSize_4__ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_12 = u32;
pub const anon_12_value: anon_12 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_WithSize_8__ptr__WithSize_8__ptr_ {}
impl CppMove for test_macros_detail_is_same_WithSize_8__ptr__WithSize_8__ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_13 = u32;
pub const anon_13_value: anon_13 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_WithSize_8__ptr__const_WithSize_8__ptr_ {}
impl CppMove for test_macros_detail_is_same_const_WithSize_8__ptr__const_WithSize_8__ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_14 = u32;
pub const anon_14_value: anon_14 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_WithSize_16__ptr__WithSize_16__ptr_ {}
impl CppMove for test_macros_detail_is_same_WithSize_16__ptr__WithSize_16__ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_15 = u32;
pub const anon_15_value: anon_15 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_WithSize_16__ptr__const_WithSize_16__ptr_ {}
impl CppMove for test_macros_detail_is_same_const_WithSize_16__ptr__const_WithSize_16__ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_16 = u32;
pub const anon_16_value: anon_16 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_WithSize_256__ptr__WithSize_256__ptr_ {}
impl CppMove for test_macros_detail_is_same_WithSize_256__ptr__WithSize_256__ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_17 = u32;
pub const anon_17_value: anon_17 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_WithSize_256__ptr__const_WithSize_256__ptr_ {}
impl CppMove for test_macros_detail_is_same_const_WithSize_256__ptr__const_WithSize_256__ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_basic_18() {
    {
        {
            let mut x: Ptr<i8> = Ptr::alloc(Default::default());
            let ptr: Value<std_unique_ptr_char__std_default_delete_char__> = Rc::new(RefCell::new(
                std_unique_ptr_char__std_default_delete_char__::new_1({ (x).clone() }),
            ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<i8> = Ptr::alloc(Default::default());
            let ptr: Value<std_unique_ptr_char__std_default_delete_char__> = Rc::new(RefCell::new(
                std_unique_ptr_char__std_default_delete_char__::new_1({ (x).clone() }),
            ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
    {
        {
            let mut x: Ptr<i8> = Ptr::alloc(Default::default());
            let ptr: Value<std_unique_ptr_const_char__std_default_delete_const_char__> =
                Rc::new(RefCell::new(
                    std_unique_ptr_const_char__std_default_delete_const_char__::new_1({
                        (x).clone()
                    }),
                ));
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<i8> = Ptr::alloc(Default::default());
            let ptr: Value<std_unique_ptr_const_char__std_default_delete_const_char__> =
                Rc::new(RefCell::new(
                    std_unique_ptr_const_char__std_default_delete_const_char__::new_1({
                        (x).clone()
                    }),
                ));
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
    {
        {
            let mut x: Ptr<i8> = Ptr::alloc_array((0..3_u64).map(|_| 0_i8).collect::<Box<[i8]>>());
            let ptr: Value<std_unique_ptr_chararrarr__std_default_delete_chararrarr__> =
                Rc::new(RefCell::new(
                    std_unique_ptr_chararrarr__std_default_delete_chararrarr__::new_1({
                        (x).clone()
                    }),
                ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<i8> = Ptr::alloc_array((0..3_u64).map(|_| 0_i8).collect::<Box<[i8]>>());
            let ptr: Value<std_unique_ptr_chararrarr__std_default_delete_chararrarr__> =
                Rc::new(RefCell::new(
                    std_unique_ptr_chararrarr__std_default_delete_chararrarr__::new_1({
                        (x).clone()
                    }),
                ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
    {
        {
            let mut x: Ptr<i8> = Ptr::alloc_array((0..3_u64).map(|_| 0_i8).collect::<Box<[i8]>>());
            let ptr: Value<std_unique_ptr_const_chararrarr__std_default_delete_const_chararrarr__> =
                Rc::new(RefCell::new(
                    std_unique_ptr_const_chararrarr__std_default_delete_const_chararrarr__::new_1(
                        { (x).clone() },
                    ),
                ));
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<i8> = Ptr::alloc_array((0..3_u64).map(|_| 0_i8).collect::<Box<[i8]>>());
            let ptr: Value<std_unique_ptr_const_chararrarr__std_default_delete_const_chararrarr__> =
                Rc::new(RefCell::new(
                    std_unique_ptr_const_chararrarr__std_default_delete_const_chararrarr__::new_1(
                        { (x).clone() },
                    ),
                ));
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
}
pub fn test_basic_20() {
    {
        {
            let mut x: Ptr<i32> = Ptr::alloc(Default::default());
            let ptr: Value<std_unique_ptr_int__std_default_delete_int__> = Rc::new(RefCell::new(
                std_unique_ptr_int__std_default_delete_int__::new_1({ (x).clone() }),
            ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<i32> = Ptr::alloc(Default::default());
            let ptr: Value<std_unique_ptr_int__std_default_delete_int__> = Rc::new(RefCell::new(
                std_unique_ptr_int__std_default_delete_int__::new_1({ (x).clone() }),
            ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
    {
        {
            let mut x: Ptr<i32> = Ptr::alloc(Default::default());
            let ptr: Value<std_unique_ptr_const_int__std_default_delete_const_int__> =
                Rc::new(RefCell::new(
                    std_unique_ptr_const_int__std_default_delete_const_int__::new_1({
                        (x).clone()
                    }),
                ));
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<i32> = Ptr::alloc(Default::default());
            let ptr: Value<std_unique_ptr_const_int__std_default_delete_const_int__> =
                Rc::new(RefCell::new(
                    std_unique_ptr_const_int__std_default_delete_const_int__::new_1({
                        (x).clone()
                    }),
                ));
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
    {
        {
            let mut x: Ptr<i32> =
                Ptr::alloc_array((0..3_u64).map(|_| 0_i32).collect::<Box<[i32]>>());
            let ptr: Value<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> =
                Rc::new(RefCell::new(
                    std_unique_ptr_intarrarr__std_default_delete_intarrarr__::new_1({
                        (x).clone()
                    }),
                ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<i32> =
                Ptr::alloc_array((0..3_u64).map(|_| 0_i32).collect::<Box<[i32]>>());
            let ptr: Value<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> =
                Rc::new(RefCell::new(
                    std_unique_ptr_intarrarr__std_default_delete_intarrarr__::new_1({
                        (x).clone()
                    }),
                ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
    {
        {
            let mut x: Ptr<i32> =
                Ptr::alloc_array((0..3_u64).map(|_| 0_i32).collect::<Box<[i32]>>());
            let ptr: Value<std_unique_ptr_const_intarrarr__std_default_delete_const_intarrarr__> =
                Rc::new(RefCell::new(
                    std_unique_ptr_const_intarrarr__std_default_delete_const_intarrarr__::new_1({
                        (x).clone()
                    }),
                ));
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<i32> =
                Ptr::alloc_array((0..3_u64).map(|_| 0_i32).collect::<Box<[i32]>>());
            let ptr: Value<std_unique_ptr_const_intarrarr__std_default_delete_const_intarrarr__> =
                Rc::new(RefCell::new(
                    std_unique_ptr_const_intarrarr__std_default_delete_const_intarrarr__::new_1({
                        (x).clone()
                    }),
                ));
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
}
pub fn test_basic_21() {
    {
        {
            let mut x: Ptr<WithSize_1_> = Ptr::alloc(<WithSize_1_>::default());
            let ptr: Value<std_unique_ptr_WithSize_1___std_default_delete_WithSize_1___> =
                Rc::new(RefCell::new(
                    std_unique_ptr_WithSize_1___std_default_delete_WithSize_1___::new_1({
                        (x).clone()
                    }),
                ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<WithSize_1_> = Ptr::alloc(<WithSize_1_>::default());
            let ptr: Value<std_unique_ptr_WithSize_1___std_default_delete_WithSize_1___> =
                Rc::new(RefCell::new(
                    std_unique_ptr_WithSize_1___std_default_delete_WithSize_1___::new_1({
                        (x).clone()
                    }),
                ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
    {
        {
            let mut x: Ptr<WithSize_1_> = Ptr::alloc(<WithSize_1_>::default());
            let ptr: Value<
                std_unique_ptr_const_WithSize_1___std_default_delete_const_WithSize_1___,
            > = Rc::new(RefCell::new(
                std_unique_ptr_const_WithSize_1___std_default_delete_const_WithSize_1___::new_1({
                    (x).clone()
                }),
            ));
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<WithSize_1_> = Ptr::alloc(<WithSize_1_>::default());
            let ptr: Value<
                std_unique_ptr_const_WithSize_1___std_default_delete_const_WithSize_1___,
            > = Rc::new(RefCell::new(
                std_unique_ptr_const_WithSize_1___std_default_delete_const_WithSize_1___::new_1({
                    (x).clone()
                }),
            ));
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
    {
        {
            let mut x: Ptr<WithSize_1_> = Ptr::alloc_array(
                (0..3_u64)
                    .map(|_| <WithSize_1_>::default())
                    .collect::<Box<[WithSize_1_]>>(),
            );
            let ptr: Value<
                std_unique_ptr_WithSize_1_arrarr__std_default_delete_WithSize_1_arrarr__,
            > = Rc::new(RefCell::new(
                std_unique_ptr_WithSize_1_arrarr__std_default_delete_WithSize_1_arrarr__::new_1({
                    (x).clone()
                }),
            ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<WithSize_1_> = Ptr::alloc_array(
                (0..3_u64)
                    .map(|_| <WithSize_1_>::default())
                    .collect::<Box<[WithSize_1_]>>(),
            );
            let ptr: Value<
                std_unique_ptr_WithSize_1_arrarr__std_default_delete_WithSize_1_arrarr__,
            > = Rc::new(RefCell::new(
                std_unique_ptr_WithSize_1_arrarr__std_default_delete_WithSize_1_arrarr__::new_1({
                    (x).clone()
                }),
            ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
    {
        {
            let mut x: Ptr<WithSize_1_> = Ptr::alloc_array(
                (0..3_u64)
                    .map(|_| <WithSize_1_>::default())
                    .collect::<Box<[WithSize_1_]>>(),
            );
            let ptr : Value<std_unique_ptr_const_WithSize_1_arrarr__std_default_delete_const_WithSize_1_arrarr__ > = Rc::new(RefCell::new(std_unique_ptr_const_WithSize_1_arrarr__std_default_delete_const_WithSize_1_arrarr__ :: new_1 ( { (x ).clone() } , )  )) ;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<WithSize_1_> = Ptr::alloc_array(
                (0..3_u64)
                    .map(|_| <WithSize_1_>::default())
                    .collect::<Box<[WithSize_1_]>>(),
            );
            let ptr : Value<std_unique_ptr_const_WithSize_1_arrarr__std_default_delete_const_WithSize_1_arrarr__ > = Rc::new(RefCell::new(std_unique_ptr_const_WithSize_1_arrarr__std_default_delete_const_WithSize_1_arrarr__ :: new_1 ( { (x ).clone() } , )  )) ;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
}
pub fn test_basic_22() {
    {
        {
            let mut x: Ptr<WithSize_2_> = Ptr::alloc(<WithSize_2_>::default());
            let ptr: Value<std_unique_ptr_WithSize_2___std_default_delete_WithSize_2___> =
                Rc::new(RefCell::new(
                    std_unique_ptr_WithSize_2___std_default_delete_WithSize_2___::new_1({
                        (x).clone()
                    }),
                ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<WithSize_2_> = Ptr::alloc(<WithSize_2_>::default());
            let ptr: Value<std_unique_ptr_WithSize_2___std_default_delete_WithSize_2___> =
                Rc::new(RefCell::new(
                    std_unique_ptr_WithSize_2___std_default_delete_WithSize_2___::new_1({
                        (x).clone()
                    }),
                ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
    {
        {
            let mut x: Ptr<WithSize_2_> = Ptr::alloc(<WithSize_2_>::default());
            let ptr: Value<
                std_unique_ptr_const_WithSize_2___std_default_delete_const_WithSize_2___,
            > = Rc::new(RefCell::new(
                std_unique_ptr_const_WithSize_2___std_default_delete_const_WithSize_2___::new_1({
                    (x).clone()
                }),
            ));
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<WithSize_2_> = Ptr::alloc(<WithSize_2_>::default());
            let ptr: Value<
                std_unique_ptr_const_WithSize_2___std_default_delete_const_WithSize_2___,
            > = Rc::new(RefCell::new(
                std_unique_ptr_const_WithSize_2___std_default_delete_const_WithSize_2___::new_1({
                    (x).clone()
                }),
            ));
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
    {
        {
            let mut x: Ptr<WithSize_2_> = Ptr::alloc_array(
                (0..3_u64)
                    .map(|_| <WithSize_2_>::default())
                    .collect::<Box<[WithSize_2_]>>(),
            );
            let ptr: Value<
                std_unique_ptr_WithSize_2_arrarr__std_default_delete_WithSize_2_arrarr__,
            > = Rc::new(RefCell::new(
                std_unique_ptr_WithSize_2_arrarr__std_default_delete_WithSize_2_arrarr__::new_1({
                    (x).clone()
                }),
            ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<WithSize_2_> = Ptr::alloc_array(
                (0..3_u64)
                    .map(|_| <WithSize_2_>::default())
                    .collect::<Box<[WithSize_2_]>>(),
            );
            let ptr: Value<
                std_unique_ptr_WithSize_2_arrarr__std_default_delete_WithSize_2_arrarr__,
            > = Rc::new(RefCell::new(
                std_unique_ptr_WithSize_2_arrarr__std_default_delete_WithSize_2_arrarr__::new_1({
                    (x).clone()
                }),
            ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
    {
        {
            let mut x: Ptr<WithSize_2_> = Ptr::alloc_array(
                (0..3_u64)
                    .map(|_| <WithSize_2_>::default())
                    .collect::<Box<[WithSize_2_]>>(),
            );
            let ptr : Value<std_unique_ptr_const_WithSize_2_arrarr__std_default_delete_const_WithSize_2_arrarr__ > = Rc::new(RefCell::new(std_unique_ptr_const_WithSize_2_arrarr__std_default_delete_const_WithSize_2_arrarr__ :: new_1 ( { (x ).clone() } , )  )) ;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<WithSize_2_> = Ptr::alloc_array(
                (0..3_u64)
                    .map(|_| <WithSize_2_>::default())
                    .collect::<Box<[WithSize_2_]>>(),
            );
            let ptr : Value<std_unique_ptr_const_WithSize_2_arrarr__std_default_delete_const_WithSize_2_arrarr__ > = Rc::new(RefCell::new(std_unique_ptr_const_WithSize_2_arrarr__std_default_delete_const_WithSize_2_arrarr__ :: new_1 ( { (x ).clone() } , )  )) ;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
}
pub fn test_basic_23() {
    {
        {
            let mut x: Ptr<WithSize_3_> = Ptr::alloc(<WithSize_3_>::default());
            let ptr: Value<std_unique_ptr_WithSize_3___std_default_delete_WithSize_3___> =
                Rc::new(RefCell::new(
                    std_unique_ptr_WithSize_3___std_default_delete_WithSize_3___::new_1({
                        (x).clone()
                    }),
                ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<WithSize_3_> = Ptr::alloc(<WithSize_3_>::default());
            let ptr: Value<std_unique_ptr_WithSize_3___std_default_delete_WithSize_3___> =
                Rc::new(RefCell::new(
                    std_unique_ptr_WithSize_3___std_default_delete_WithSize_3___::new_1({
                        (x).clone()
                    }),
                ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
    {
        {
            let mut x: Ptr<WithSize_3_> = Ptr::alloc(<WithSize_3_>::default());
            let ptr: Value<
                std_unique_ptr_const_WithSize_3___std_default_delete_const_WithSize_3___,
            > = Rc::new(RefCell::new(
                std_unique_ptr_const_WithSize_3___std_default_delete_const_WithSize_3___::new_1({
                    (x).clone()
                }),
            ));
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<WithSize_3_> = Ptr::alloc(<WithSize_3_>::default());
            let ptr: Value<
                std_unique_ptr_const_WithSize_3___std_default_delete_const_WithSize_3___,
            > = Rc::new(RefCell::new(
                std_unique_ptr_const_WithSize_3___std_default_delete_const_WithSize_3___::new_1({
                    (x).clone()
                }),
            ));
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
    {
        {
            let mut x: Ptr<WithSize_3_> = Ptr::alloc_array(
                (0..3_u64)
                    .map(|_| <WithSize_3_>::default())
                    .collect::<Box<[WithSize_3_]>>(),
            );
            let ptr: Value<
                std_unique_ptr_WithSize_3_arrarr__std_default_delete_WithSize_3_arrarr__,
            > = Rc::new(RefCell::new(
                std_unique_ptr_WithSize_3_arrarr__std_default_delete_WithSize_3_arrarr__::new_1({
                    (x).clone()
                }),
            ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<WithSize_3_> = Ptr::alloc_array(
                (0..3_u64)
                    .map(|_| <WithSize_3_>::default())
                    .collect::<Box<[WithSize_3_]>>(),
            );
            let ptr: Value<
                std_unique_ptr_WithSize_3_arrarr__std_default_delete_WithSize_3_arrarr__,
            > = Rc::new(RefCell::new(
                std_unique_ptr_WithSize_3_arrarr__std_default_delete_WithSize_3_arrarr__::new_1({
                    (x).clone()
                }),
            ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
    {
        {
            let mut x: Ptr<WithSize_3_> = Ptr::alloc_array(
                (0..3_u64)
                    .map(|_| <WithSize_3_>::default())
                    .collect::<Box<[WithSize_3_]>>(),
            );
            let ptr : Value<std_unique_ptr_const_WithSize_3_arrarr__std_default_delete_const_WithSize_3_arrarr__ > = Rc::new(RefCell::new(std_unique_ptr_const_WithSize_3_arrarr__std_default_delete_const_WithSize_3_arrarr__ :: new_1 ( { (x ).clone() } , )  )) ;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<WithSize_3_> = Ptr::alloc_array(
                (0..3_u64)
                    .map(|_| <WithSize_3_>::default())
                    .collect::<Box<[WithSize_3_]>>(),
            );
            let ptr : Value<std_unique_ptr_const_WithSize_3_arrarr__std_default_delete_const_WithSize_3_arrarr__ > = Rc::new(RefCell::new(std_unique_ptr_const_WithSize_3_arrarr__std_default_delete_const_WithSize_3_arrarr__ :: new_1 ( { (x ).clone() } , )  )) ;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
}
pub fn test_basic_24() {
    {
        {
            let mut x: Ptr<WithSize_4_> = Ptr::alloc(<WithSize_4_>::default());
            let ptr: Value<std_unique_ptr_WithSize_4___std_default_delete_WithSize_4___> =
                Rc::new(RefCell::new(
                    std_unique_ptr_WithSize_4___std_default_delete_WithSize_4___::new_1({
                        (x).clone()
                    }),
                ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<WithSize_4_> = Ptr::alloc(<WithSize_4_>::default());
            let ptr: Value<std_unique_ptr_WithSize_4___std_default_delete_WithSize_4___> =
                Rc::new(RefCell::new(
                    std_unique_ptr_WithSize_4___std_default_delete_WithSize_4___::new_1({
                        (x).clone()
                    }),
                ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
    {
        {
            let mut x: Ptr<WithSize_4_> = Ptr::alloc(<WithSize_4_>::default());
            let ptr: Value<
                std_unique_ptr_const_WithSize_4___std_default_delete_const_WithSize_4___,
            > = Rc::new(RefCell::new(
                std_unique_ptr_const_WithSize_4___std_default_delete_const_WithSize_4___::new_1({
                    (x).clone()
                }),
            ));
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<WithSize_4_> = Ptr::alloc(<WithSize_4_>::default());
            let ptr: Value<
                std_unique_ptr_const_WithSize_4___std_default_delete_const_WithSize_4___,
            > = Rc::new(RefCell::new(
                std_unique_ptr_const_WithSize_4___std_default_delete_const_WithSize_4___::new_1({
                    (x).clone()
                }),
            ));
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
    {
        {
            let mut x: Ptr<WithSize_4_> = Ptr::alloc_array(
                (0..3_u64)
                    .map(|_| <WithSize_4_>::default())
                    .collect::<Box<[WithSize_4_]>>(),
            );
            let ptr: Value<
                std_unique_ptr_WithSize_4_arrarr__std_default_delete_WithSize_4_arrarr__,
            > = Rc::new(RefCell::new(
                std_unique_ptr_WithSize_4_arrarr__std_default_delete_WithSize_4_arrarr__::new_1({
                    (x).clone()
                }),
            ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<WithSize_4_> = Ptr::alloc_array(
                (0..3_u64)
                    .map(|_| <WithSize_4_>::default())
                    .collect::<Box<[WithSize_4_]>>(),
            );
            let ptr: Value<
                std_unique_ptr_WithSize_4_arrarr__std_default_delete_WithSize_4_arrarr__,
            > = Rc::new(RefCell::new(
                std_unique_ptr_WithSize_4_arrarr__std_default_delete_WithSize_4_arrarr__::new_1({
                    (x).clone()
                }),
            ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
    {
        {
            let mut x: Ptr<WithSize_4_> = Ptr::alloc_array(
                (0..3_u64)
                    .map(|_| <WithSize_4_>::default())
                    .collect::<Box<[WithSize_4_]>>(),
            );
            let ptr : Value<std_unique_ptr_const_WithSize_4_arrarr__std_default_delete_const_WithSize_4_arrarr__ > = Rc::new(RefCell::new(std_unique_ptr_const_WithSize_4_arrarr__std_default_delete_const_WithSize_4_arrarr__ :: new_1 ( { (x ).clone() } , )  )) ;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<WithSize_4_> = Ptr::alloc_array(
                (0..3_u64)
                    .map(|_| <WithSize_4_>::default())
                    .collect::<Box<[WithSize_4_]>>(),
            );
            let ptr : Value<std_unique_ptr_const_WithSize_4_arrarr__std_default_delete_const_WithSize_4_arrarr__ > = Rc::new(RefCell::new(std_unique_ptr_const_WithSize_4_arrarr__std_default_delete_const_WithSize_4_arrarr__ :: new_1 ( { (x ).clone() } , )  )) ;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
}
pub fn test_basic_25() {
    {
        {
            let mut x: Ptr<WithSize_8_> = Ptr::alloc(<WithSize_8_>::default());
            let ptr: Value<std_unique_ptr_WithSize_8___std_default_delete_WithSize_8___> =
                Rc::new(RefCell::new(
                    std_unique_ptr_WithSize_8___std_default_delete_WithSize_8___::new_1({
                        (x).clone()
                    }),
                ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<WithSize_8_> = Ptr::alloc(<WithSize_8_>::default());
            let ptr: Value<std_unique_ptr_WithSize_8___std_default_delete_WithSize_8___> =
                Rc::new(RefCell::new(
                    std_unique_ptr_WithSize_8___std_default_delete_WithSize_8___::new_1({
                        (x).clone()
                    }),
                ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
    {
        {
            let mut x: Ptr<WithSize_8_> = Ptr::alloc(<WithSize_8_>::default());
            let ptr: Value<
                std_unique_ptr_const_WithSize_8___std_default_delete_const_WithSize_8___,
            > = Rc::new(RefCell::new(
                std_unique_ptr_const_WithSize_8___std_default_delete_const_WithSize_8___::new_1({
                    (x).clone()
                }),
            ));
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<WithSize_8_> = Ptr::alloc(<WithSize_8_>::default());
            let ptr: Value<
                std_unique_ptr_const_WithSize_8___std_default_delete_const_WithSize_8___,
            > = Rc::new(RefCell::new(
                std_unique_ptr_const_WithSize_8___std_default_delete_const_WithSize_8___::new_1({
                    (x).clone()
                }),
            ));
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
    {
        {
            let mut x: Ptr<WithSize_8_> = Ptr::alloc_array(
                (0..3_u64)
                    .map(|_| <WithSize_8_>::default())
                    .collect::<Box<[WithSize_8_]>>(),
            );
            let ptr: Value<
                std_unique_ptr_WithSize_8_arrarr__std_default_delete_WithSize_8_arrarr__,
            > = Rc::new(RefCell::new(
                std_unique_ptr_WithSize_8_arrarr__std_default_delete_WithSize_8_arrarr__::new_1({
                    (x).clone()
                }),
            ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<WithSize_8_> = Ptr::alloc_array(
                (0..3_u64)
                    .map(|_| <WithSize_8_>::default())
                    .collect::<Box<[WithSize_8_]>>(),
            );
            let ptr: Value<
                std_unique_ptr_WithSize_8_arrarr__std_default_delete_WithSize_8_arrarr__,
            > = Rc::new(RefCell::new(
                std_unique_ptr_WithSize_8_arrarr__std_default_delete_WithSize_8_arrarr__::new_1({
                    (x).clone()
                }),
            ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
    {
        {
            let mut x: Ptr<WithSize_8_> = Ptr::alloc_array(
                (0..3_u64)
                    .map(|_| <WithSize_8_>::default())
                    .collect::<Box<[WithSize_8_]>>(),
            );
            let ptr : Value<std_unique_ptr_const_WithSize_8_arrarr__std_default_delete_const_WithSize_8_arrarr__ > = Rc::new(RefCell::new(std_unique_ptr_const_WithSize_8_arrarr__std_default_delete_const_WithSize_8_arrarr__ :: new_1 ( { (x ).clone() } , )  )) ;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<WithSize_8_> = Ptr::alloc_array(
                (0..3_u64)
                    .map(|_| <WithSize_8_>::default())
                    .collect::<Box<[WithSize_8_]>>(),
            );
            let ptr : Value<std_unique_ptr_const_WithSize_8_arrarr__std_default_delete_const_WithSize_8_arrarr__ > = Rc::new(RefCell::new(std_unique_ptr_const_WithSize_8_arrarr__std_default_delete_const_WithSize_8_arrarr__ :: new_1 ( { (x ).clone() } , )  )) ;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
}
pub fn test_basic_26() {
    {
        {
            let mut x: Ptr<WithSize_16_> = Ptr::alloc(<WithSize_16_>::default());
            let ptr: Value<std_unique_ptr_WithSize_16___std_default_delete_WithSize_16___> =
                Rc::new(RefCell::new(
                    std_unique_ptr_WithSize_16___std_default_delete_WithSize_16___::new_1({
                        (x).clone()
                    }),
                ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<WithSize_16_> = Ptr::alloc(<WithSize_16_>::default());
            let ptr: Value<std_unique_ptr_WithSize_16___std_default_delete_WithSize_16___> =
                Rc::new(RefCell::new(
                    std_unique_ptr_WithSize_16___std_default_delete_WithSize_16___::new_1({
                        (x).clone()
                    }),
                ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
    {
        {
            let mut x: Ptr<WithSize_16_> = Ptr::alloc(<WithSize_16_>::default());
            let ptr: Value<
                std_unique_ptr_const_WithSize_16___std_default_delete_const_WithSize_16___,
            > = Rc::new(RefCell::new(
                std_unique_ptr_const_WithSize_16___std_default_delete_const_WithSize_16___::new_1(
                    { (x).clone() },
                ),
            ));
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<WithSize_16_> = Ptr::alloc(<WithSize_16_>::default());
            let ptr: Value<
                std_unique_ptr_const_WithSize_16___std_default_delete_const_WithSize_16___,
            > = Rc::new(RefCell::new(
                std_unique_ptr_const_WithSize_16___std_default_delete_const_WithSize_16___::new_1(
                    { (x).clone() },
                ),
            ));
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
    {
        {
            let mut x: Ptr<WithSize_16_> = Ptr::alloc_array(
                (0..3_u64)
                    .map(|_| <WithSize_16_>::default())
                    .collect::<Box<[WithSize_16_]>>(),
            );
            let ptr: Value<
                std_unique_ptr_WithSize_16_arrarr__std_default_delete_WithSize_16_arrarr__,
            > = Rc::new(RefCell::new(
                std_unique_ptr_WithSize_16_arrarr__std_default_delete_WithSize_16_arrarr__::new_1(
                    { (x).clone() },
                ),
            ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<WithSize_16_> = Ptr::alloc_array(
                (0..3_u64)
                    .map(|_| <WithSize_16_>::default())
                    .collect::<Box<[WithSize_16_]>>(),
            );
            let ptr: Value<
                std_unique_ptr_WithSize_16_arrarr__std_default_delete_WithSize_16_arrarr__,
            > = Rc::new(RefCell::new(
                std_unique_ptr_WithSize_16_arrarr__std_default_delete_WithSize_16_arrarr__::new_1(
                    { (x).clone() },
                ),
            ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
    {
        {
            let mut x: Ptr<WithSize_16_> = Ptr::alloc_array(
                (0..3_u64)
                    .map(|_| <WithSize_16_>::default())
                    .collect::<Box<[WithSize_16_]>>(),
            );
            let ptr : Value<std_unique_ptr_const_WithSize_16_arrarr__std_default_delete_const_WithSize_16_arrarr__ > = Rc::new(RefCell::new(std_unique_ptr_const_WithSize_16_arrarr__std_default_delete_const_WithSize_16_arrarr__ :: new_1 ( { (x ).clone() } , )  )) ;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<WithSize_16_> = Ptr::alloc_array(
                (0..3_u64)
                    .map(|_| <WithSize_16_>::default())
                    .collect::<Box<[WithSize_16_]>>(),
            );
            let ptr : Value<std_unique_ptr_const_WithSize_16_arrarr__std_default_delete_const_WithSize_16_arrarr__ > = Rc::new(RefCell::new(std_unique_ptr_const_WithSize_16_arrarr__std_default_delete_const_WithSize_16_arrarr__ :: new_1 ( { (x ).clone() } , )  )) ;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
}
pub fn test_basic_27() {
    {
        {
            let mut x: Ptr<WithSize_256_> = Ptr::alloc(<WithSize_256_>::default());
            let ptr: Value<std_unique_ptr_WithSize_256___std_default_delete_WithSize_256___> =
                Rc::new(RefCell::new(
                    std_unique_ptr_WithSize_256___std_default_delete_WithSize_256___::new_1({
                        (x).clone()
                    }),
                ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<WithSize_256_> = Ptr::alloc(<WithSize_256_>::default());
            let ptr: Value<std_unique_ptr_WithSize_256___std_default_delete_WithSize_256___> =
                Rc::new(RefCell::new(
                    std_unique_ptr_WithSize_256___std_default_delete_WithSize_256___::new_1({
                        (x).clone()
                    }),
                ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
    {
        {
            let mut x: Ptr<WithSize_256_> = Ptr::alloc(<WithSize_256_>::default());
            let ptr: Value<
                std_unique_ptr_const_WithSize_256___std_default_delete_const_WithSize_256___,
            > = Rc::new(RefCell::new(
                std_unique_ptr_const_WithSize_256___std_default_delete_const_WithSize_256___::new_1(
                    { (x).clone() },
                ),
            ));
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<WithSize_256_> = Ptr::alloc(<WithSize_256_>::default());
            let ptr: Value<
                std_unique_ptr_const_WithSize_256___std_default_delete_const_WithSize_256___,
            > = Rc::new(RefCell::new(
                std_unique_ptr_const_WithSize_256___std_default_delete_const_WithSize_256___::new_1(
                    { (x).clone() },
                ),
            ));
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
    {
        {
            let mut x: Ptr<WithSize_256_> = Ptr::alloc_array(
                (0..3_u64)
                    .map(|_| <WithSize_256_>::default())
                    .collect::<Box<[WithSize_256_]>>(),
            );
            let ptr: Value<
                std_unique_ptr_WithSize_256_arrarr__std_default_delete_WithSize_256_arrarr__,
            > = Rc::new(RefCell::new(
                std_unique_ptr_WithSize_256_arrarr__std_default_delete_WithSize_256_arrarr__::new_1(
                    { (x).clone() },
                ),
            ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<WithSize_256_> = Ptr::alloc_array(
                (0..3_u64)
                    .map(|_| <WithSize_256_>::default())
                    .collect::<Box<[WithSize_256_]>>(),
            );
            let ptr: Value<
                std_unique_ptr_WithSize_256_arrarr__std_default_delete_WithSize_256_arrarr__,
            > = Rc::new(RefCell::new(
                std_unique_ptr_WithSize_256_arrarr__std_default_delete_WithSize_256_arrarr__::new_1(
                    { (x).clone() },
                ),
            ));
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
    {
        {
            let mut x: Ptr<WithSize_256_> = Ptr::alloc_array(
                (0..3_u64)
                    .map(|_| <WithSize_256_>::default())
                    .collect::<Box<[WithSize_256_]>>(),
            );
            let ptr : Value<std_unique_ptr_const_WithSize_256_arrarr__std_default_delete_const_WithSize_256_arrarr__ > = Rc::new(RefCell::new(std_unique_ptr_const_WithSize_256_arrarr__std_default_delete_const_WithSize_256_arrarr__ :: new_1 ( { (x ).clone() } , )  )) ;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
        {
            let mut x: Ptr<WithSize_256_> = Ptr::alloc_array(
                (0..3_u64)
                    .map(|_| <WithSize_256_>::default())
                    .collect::<Box<[WithSize_256_]>>(),
            );
            let ptr : Value<std_unique_ptr_const_WithSize_256_arrarr__std_default_delete_const_WithSize_256_arrarr__ > = Rc::new(RefCell::new(std_unique_ptr_const_WithSize_256_arrarr__std_default_delete_const_WithSize_256_arrarr__ :: new_1 ( { (x ).clone() } , )  )) ;
            const _: () = assert!(true, "");;
            ({ cpp2rust_assert_fail_19(({ ({ (*ptr.borrow()).get() }) } == { (x).clone() })) });
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct WithSize_1_ {
    #[offset(0)]
    #[byte_size(1)]
    pub padding: Box<[i8]>,
}
impl CppMove for WithSize_1_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for WithSize_1_ {
    fn default() -> Self {
        {
            WithSize_1_ {
                padding: (0..1).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(2)]
pub struct WithSize_2_ {
    #[offset(0)]
    #[byte_size(2)]
    pub padding: Box<[i8]>,
}
impl CppMove for WithSize_2_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for WithSize_2_ {
    fn default() -> Self {
        {
            WithSize_2_ {
                padding: (0..2).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(3)]
pub struct WithSize_3_ {
    #[offset(0)]
    #[byte_size(3)]
    pub padding: Box<[i8]>,
}
impl CppMove for WithSize_3_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for WithSize_3_ {
    fn default() -> Self {
        {
            WithSize_3_ {
                padding: (0..3).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct WithSize_4_ {
    #[offset(0)]
    #[byte_size(4)]
    pub padding: Box<[i8]>,
}
impl CppMove for WithSize_4_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for WithSize_4_ {
    fn default() -> Self {
        {
            WithSize_4_ {
                padding: (0..4).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct WithSize_8_ {
    #[offset(0)]
    #[byte_size(8)]
    pub padding: Box<[i8]>,
}
impl CppMove for WithSize_8_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for WithSize_8_ {
    fn default() -> Self {
        {
            WithSize_8_ {
                padding: (0..8).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct WithSize_16_ {
    #[offset(0)]
    #[byte_size(16)]
    pub padding: Box<[i8]>,
}
impl CppMove for WithSize_16_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for WithSize_16_ {
    fn default() -> Self {
        {
            WithSize_16_ {
                padding: (0..16).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(256)]
pub struct WithSize_256_ {
    #[offset(0)]
    #[byte_size(256)]
    pub padding: Box<[i8]>,
}
impl CppMove for WithSize_256_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for WithSize_256_ {
    fn default() -> Self {
        {
            WithSize_256_ {
                padding: (0..256).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
pub fn test_28() -> bool {
    ({ test_basic_18() });
    ({ test_basic_20() });
    ({ test_basic_21() });
    ({ test_basic_22() });
    ({ test_basic_23() });
    ({ test_basic_24() });
    ({ test_basic_25() });
    ({ test_basic_26() });
    ({ test_basic_27() });
    return true;
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_28() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
