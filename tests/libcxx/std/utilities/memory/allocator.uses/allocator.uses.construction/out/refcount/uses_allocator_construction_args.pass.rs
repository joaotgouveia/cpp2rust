extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(40)]
pub struct test_allocator_statistics {
    #[offset(0)]
    pub time_to_throw: (),
    #[offset(4)]
    pub throw_after: (),
    #[offset(8)]
    pub count: (),
    #[offset(12)]
    pub alloc_count: (),
    #[offset(16)]
    pub allocated_size: (),
    #[offset(20)]
    pub construct_count: (),
    #[offset(24)]
    pub destroy_count: (),
    #[offset(28)]
    pub copied: (),
    #[offset(32)]
    pub moved: (),
    #[offset(36)]
    pub converted: (),
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        {
            test_allocator_statistics {
                time_to_throw: 0_(),
                throw_after: 2147483647_(),
                count: 0_(),
                alloc_count: 0_(),
                allocated_size: 0_(),
                construct_count: 0_(),
                destroy_count: 0_(),
                copied: 0_(),
                moved: 0_(),
                converted: 0_(),
            }
        }
    }
}
thread_local!(
    pub static destructed_value_0: Value<()> = Rc::new(RefCell::new(-1_()));
);
thread_local!(
    pub static moved_value_1: Value<()> = Rc::new(RefCell::new(2147483647_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_2(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_3(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_2(_x, _y)
    });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(16)]
pub struct test_allocator_void_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_void_ {
    pub fn new_1(mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: 0_(),
            id_: 0_(),
            stats_: (stats).clone(),
        }
    }
    pub fn new_2(mut data: ()) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_3(mut data: (), mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: (stats).clone(),
        }
    }
    pub fn new_4(mut data: (), mut id: ()) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_5(mut data: (), mut id: (), mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: (stats).clone(),
        }
    }
    pub fn copy_from(a: Ptr<test_allocator_void_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }
    }
}
impl std::cmp::PartialEq for test_allocator_void_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_2(
                Rc::new(RefCell::new(test_allocator_void_ {
                    data_: self.data_.clone(),
                    id_: self.id_.clone(),
                    stats_: self.stats_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(test_allocator_void_ {
                    data_: other.data_.clone(),
                    id_: other.id_.clone(),
                    stats_: other.stats_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_void_ {}
impl Clone for test_allocator_void_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_void_> = Rc::new(RefCell::new(test_allocator_void_ {
            data_: self.data_.clone(),
            id_: self.id_.clone(),
            stats_: self.stats_.clone(),
        }));
        test_allocator_void_::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_void_ {
    fn default() -> Self {
        {
            test_allocator_void_ {
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_int_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_int_ {
    pub fn new_1(mut data: ()) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn copy_from(a: Ptr<test_allocator_int_>) -> Self {
        let __this: Value<test_allocator_int_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_int_> = __this.as_pointer();
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for test_allocator_int_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_int_> = Rc::new(RefCell::new(test_allocator_int_ {
            data_: self.data_.clone(),
            id_: self.id_.clone(),
            stats_: self.stats_.clone(),
        }));
        test_allocator_int_::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_int_ {
    fn default() -> Self {
        {
            test_allocator_int_ {
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Ctor_Tag {}
#[derive(Record, ByteRepr, FnPtrArg, CopyAssign, MoveAssign, Default)]
#[byte_size(1)]
pub struct Tag_X {}
impl Tag_X {
    pub fn new_1(mut _a0: Ctor_Tag) -> Self {
        Self {}
    }
    pub fn new_2(mut _a0: Ctor_Tag, _a1: Ptr<Tag_X>) -> Self {
        Self {}
    }
    pub fn new_3(mut _a0: Ctor_Tag, _a1: Ptr<Tag_X>) -> Self {
        Self {}
    }
}
pub type RefType = ();
pub const RefType_LValue: RefType = 0;
pub const RefType_ConstLValue: RefType = 1;
pub const RefType_RValue: RefType = 2;
pub const RefType_ConstRValue: RefType = 3;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(48)]
pub struct UsesAllocArgT {
    #[offset(0)]
    pub allocator_constructed_: (),
    #[offset(8)]
    #[byte_size(16)]
    pub a_: test_allocator_int_,
    #[offset(24)]
    #[byte_size(8)]
    pub alloc_: Ptr<test_allocator_int_>,
    #[offset(32)]
    #[byte_size(8)]
    pub val_ptr_: Ptr<()>,
    #[offset(40)]
    #[byte_size(4)]
    pub ref_type_: RefType,
}
impl UsesAllocArgT {
    pub fn new_1(_a0: (), alloc: Ptr<test_allocator_int_>) -> Self {
        let _a0: Value<()> = Rc::new(RefCell::new(_a0));
        Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: (alloc).clone(),
            val_ptr_: Ptr::<()>::null(),
            ref_type_: <RefType>::default(),
        }
    }
    pub fn new_2(_a0: (), alloc: Ptr<test_allocator_int_>, val: Ptr<()>) -> Self {
        let _a0: Value<()> = Rc::new(RefCell::new(_a0));
        Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: (alloc).clone(),
            val_ptr_: (val).clone(),
            ref_type_: RefType_LValue,
        }
    }
    pub fn new_3(_a0: (), alloc: Ptr<test_allocator_int_>, val: Ptr<()>) -> Self {
        let _a0: Value<()> = Rc::new(RefCell::new(_a0));
        Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: (alloc).clone(),
            val_ptr_: (val).clone(),
            ref_type_: RefType_ConstLValue,
        }
    }
    pub fn new_4(_a0: (), alloc: Ptr<test_allocator_int_>, val: Ptr<()>) -> Self {
        let _a0: Value<()> = Rc::new(RefCell::new(_a0));
        Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: (alloc).clone(),
            val_ptr_: (val).clone(),
            ref_type_: RefType_RValue,
        }
    }
    pub fn new_5(_a0: (), alloc: Ptr<test_allocator_int_>, val: Ptr<()>) -> Self {
        let _a0: Value<()> = Rc::new(RefCell::new(_a0));
        Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: (alloc).clone(),
            val_ptr_: (val).clone(),
            ref_type_: RefType_ConstRValue,
        }
    }
}
impl Default for UsesAllocArgT {
    fn default() -> Self {
        {
            UsesAllocArgT {
                allocator_constructed_: false,
                a_: <test_allocator_int_>::default(),
                alloc_: field_ptr!((*self), a_),
                val_ptr_: Ptr::<()>::null(),
                ref_type_: <RefType>::default(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(48)]
pub struct UsesAllocLast {
    #[offset(0)]
    pub allocator_constructed_: (),
    #[offset(8)]
    #[byte_size(16)]
    pub a_: test_allocator_int_,
    #[offset(24)]
    #[byte_size(8)]
    pub alloc_: Ptr<test_allocator_int_>,
    #[offset(32)]
    #[byte_size(8)]
    pub val_ptr_: Ptr<()>,
    #[offset(40)]
    #[byte_size(4)]
    pub ref_type_: RefType,
}
impl UsesAllocLast {
    pub fn new_1(alloc: Ptr<test_allocator_int_>) -> Self {
        Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: (alloc).clone(),
            val_ptr_: Ptr::<()>::null(),
            ref_type_: <RefType>::default(),
        }
    }
    pub fn new_2(val: Ptr<()>, alloc: Ptr<test_allocator_int_>) -> Self {
        Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: (alloc).clone(),
            val_ptr_: (val).clone(),
            ref_type_: RefType_LValue,
        }
    }
    pub fn new_3(val: Ptr<()>, alloc: Ptr<test_allocator_int_>) -> Self {
        Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: (alloc).clone(),
            val_ptr_: (val).clone(),
            ref_type_: RefType_ConstLValue,
        }
    }
    pub fn new_4(val: Ptr<()>, alloc: Ptr<test_allocator_int_>) -> Self {
        Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: (alloc).clone(),
            val_ptr_: (val).clone(),
            ref_type_: RefType_RValue,
        }
    }
    pub fn new_5(val: Ptr<()>, alloc: Ptr<test_allocator_int_>) -> Self {
        Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: (alloc).clone(),
            val_ptr_: (val).clone(),
            ref_type_: RefType_ConstRValue,
        }
    }
}
impl Default for UsesAllocLast {
    fn default() -> Self {
        {
            UsesAllocLast {
                allocator_constructed_: false,
                a_: <test_allocator_int_>::default(),
                alloc_: field_ptr!((*self), a_),
                val_ptr_: Ptr::<()>::null(),
                ref_type_: <RefType>::default(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct NotAllocatorAware {
    #[offset(0)]
    pub allocator_constructed_: (),
}
impl NotAllocatorAware {
    pub fn new_1(_a0: Ptr<test_allocator_int_>) -> Self {
        Self {
            allocator_constructed_: true,
        }
    }
    pub fn new_2(_a0: Ptr<test_allocator_int_>, mut _a1: ()) -> Self {
        Self {
            allocator_constructed_: true,
        }
    }
}
impl Default for NotAllocatorAware {
    fn default() -> Self {
        {
            NotAllocatorAware {
                allocator_constructed_: false,
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToPair {}
pub fn test_uses_allocator_construction_args_4(args: Ptr<test_allocator_int_>) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({ uses_allocator_construction_args_5((args).clone()) });
}
pub fn test_uses_allocator_construction_args_6(args: Ptr<test_allocator_int_>) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({ uses_allocator_construction_args_7((args).clone()) });
}
pub fn test_uses_allocator_construction_args_8(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
    args_2: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args_1: Ptr<()> = (args_1).clone();
        let ___args_2: Ptr<()> = (args_2).clone();
        uses_allocator_construction_args_9(___alloc, ___args_1, ___args_2)
    });
}
pub fn test_uses_allocator_construction_args_10(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<()> = (args_1).clone();
        uses_allocator_construction_args_11(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_12(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<()> = (args_1).clone();
        uses_allocator_construction_args_13(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_14(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<()> = (args_1).clone();
        uses_allocator_construction_args_15(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_16(args: Ptr<test_allocator_int_>) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({ uses_allocator_construction_args_17((args).clone()) });
}
pub fn test_uses_allocator_construction_args_18(args: Ptr<test_allocator_int_>) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({ uses_allocator_construction_args_19((args).clone()) });
}
pub fn test_uses_allocator_construction_args_20(args: Ptr<test_allocator_int_>) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({ uses_allocator_construction_args_21((args).clone()) });
}
pub fn test_uses_allocator_construction_args_22(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
    args_2: Ptr<()>,
    args_3: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args_1: Ptr<()> = (args_1).clone();
        let ___args_2: Ptr<()> = (args_2).clone();
        let ___args_3: Ptr<()> = (args_3).clone();
        uses_allocator_construction_args_23(___alloc, ___args_1, ___args_2, ___args_3)
    });
}
pub fn test_uses_allocator_construction_args_24(args: Ptr<test_allocator_int_>) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({ uses_allocator_construction_args_25((args).clone()) });
}
pub fn test_uses_allocator_construction_args_26(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
    args_2: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args_1: Ptr<()> = (args_1).clone();
        let ___args_2: Ptr<()> = (args_2).clone();
        uses_allocator_construction_args_27(___alloc, ___args_1, ___args_2)
    });
}
pub fn test_uses_allocator_construction_args_28(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
    args_2: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args_1: Ptr<()> = (args_1).clone();
        let ___args_2: Ptr<()> = (args_2).clone();
        uses_allocator_construction_args_29(___alloc, ___args_1, ___args_2)
    });
}
pub fn test_uses_allocator_construction_args_30(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<()> = (args_1).clone();
        uses_allocator_construction_args_31(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_32(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<()> = (args_1).clone();
        uses_allocator_construction_args_33(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_34(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<()> = (args_1).clone();
        uses_allocator_construction_args_35(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_36(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<()> = (args_1).clone();
        uses_allocator_construction_args_37(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_38(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<ConvertibleToPair>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<ConvertibleToPair> = (args_1).clone();
        uses_allocator_construction_args_39(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_40(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<ConvertibleToPair>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<ConvertibleToPair> = (args_1).clone();
        uses_allocator_construction_args_41(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_42(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<()> = (args_1).clone();
        uses_allocator_construction_args_43(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_44(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_> =
            (args_1).clone();
        uses_allocator_construction_args_45(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_46(args: Ptr<test_allocator_int_>) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({ uses_allocator_construction_args_47((args).clone()) });
}
pub fn test_uses_allocator_construction_args_48(args: Ptr<test_allocator_int_>) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({ uses_allocator_construction_args_49((args).clone()) });
}
pub fn test_uses_allocator_construction_args_50(args: Ptr<test_allocator_int_>) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({ uses_allocator_construction_args_51((args).clone()) });
}
pub fn test_uses_allocator_construction_args_52(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
    args_2: Ptr<()>,
    args_3: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args_1: Ptr<()> = (args_1).clone();
        let ___args_2: Ptr<()> = (args_2).clone();
        let ___args_3: Ptr<()> = (args_3).clone();
        uses_allocator_construction_args_53(___alloc, ___args_1, ___args_2, ___args_3)
    });
}
pub fn test_uses_allocator_construction_args_54(args: Ptr<test_allocator_int_>) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({ uses_allocator_construction_args_55((args).clone()) });
}
pub fn test_uses_allocator_construction_args_56(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
    args_2: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args_1: Ptr<()> = (args_1).clone();
        let ___args_2: Ptr<()> = (args_2).clone();
        uses_allocator_construction_args_57(___alloc, ___args_1, ___args_2)
    });
}
pub fn test_uses_allocator_construction_args_58(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
    args_2: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args_1: Ptr<()> = (args_1).clone();
        let ___args_2: Ptr<()> = (args_2).clone();
        uses_allocator_construction_args_59(___alloc, ___args_1, ___args_2)
    });
}
pub fn test_uses_allocator_construction_args_60(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<()> = (args_1).clone();
        uses_allocator_construction_args_61(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_62(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<()> = (args_1).clone();
        uses_allocator_construction_args_63(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_64(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<()> = (args_1).clone();
        uses_allocator_construction_args_65(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_66(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<()> = (args_1).clone();
        uses_allocator_construction_args_67(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_68(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<ConvertibleToPair>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<ConvertibleToPair> = (args_1).clone();
        uses_allocator_construction_args_69(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_70(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<ConvertibleToPair>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<ConvertibleToPair> = (args_1).clone();
        uses_allocator_construction_args_71(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_72(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<()> = (args_1).clone();
        uses_allocator_construction_args_73(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_74(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_> =
            (args_1).clone();
        uses_allocator_construction_args_75(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_76(args: Ptr<test_allocator_int_>) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({ uses_allocator_construction_args_77((args).clone()) });
}
pub fn test_uses_allocator_construction_args_78(args: Ptr<test_allocator_int_>) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({ uses_allocator_construction_args_79((args).clone()) });
}
pub fn test_uses_allocator_construction_args_80(args: Ptr<test_allocator_int_>) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({ uses_allocator_construction_args_81((args).clone()) });
}
pub fn test_uses_allocator_construction_args_82(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
    args_2: Ptr<()>,
    args_3: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args_1: Ptr<()> = (args_1).clone();
        let ___args_2: Ptr<()> = (args_2).clone();
        let ___args_3: Ptr<()> = (args_3).clone();
        uses_allocator_construction_args_83(___alloc, ___args_1, ___args_2, ___args_3)
    });
}
pub fn test_uses_allocator_construction_args_84(args: Ptr<test_allocator_int_>) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({ uses_allocator_construction_args_85((args).clone()) });
}
pub fn test_uses_allocator_construction_args_86(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
    args_2: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args_1: Ptr<()> = (args_1).clone();
        let ___args_2: Ptr<()> = (args_2).clone();
        uses_allocator_construction_args_87(___alloc, ___args_1, ___args_2)
    });
}
pub fn test_uses_allocator_construction_args_88(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
    args_2: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args_1: Ptr<()> = (args_1).clone();
        let ___args_2: Ptr<()> = (args_2).clone();
        uses_allocator_construction_args_89(___alloc, ___args_1, ___args_2)
    });
}
pub fn test_uses_allocator_construction_args_90(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<()> = (args_1).clone();
        uses_allocator_construction_args_91(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_92(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<()> = (args_1).clone();
        uses_allocator_construction_args_93(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_94(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<()> = (args_1).clone();
        uses_allocator_construction_args_95(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_96(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<()> = (args_1).clone();
        uses_allocator_construction_args_97(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_98(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<ConvertibleToPair>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<ConvertibleToPair> = (args_1).clone();
        uses_allocator_construction_args_99(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_100(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<ConvertibleToPair>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<ConvertibleToPair> = (args_1).clone();
        uses_allocator_construction_args_101(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_102(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<()> = (args_1).clone();
        uses_allocator_construction_args_103(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_104(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_> =
            (args_1).clone();
        uses_allocator_construction_args_105(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_106(args: Ptr<test_allocator_int_>) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({ uses_allocator_construction_args_107((args).clone()) });
}
pub fn test_uses_allocator_construction_args_108(args: Ptr<test_allocator_int_>) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({ uses_allocator_construction_args_109((args).clone()) });
}
pub fn test_uses_allocator_construction_args_110(args: Ptr<test_allocator_int_>) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({ uses_allocator_construction_args_111((args).clone()) });
}
pub fn test_uses_allocator_construction_args_112(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
    args_2: Ptr<()>,
    args_3: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args_1: Ptr<()> = (args_1).clone();
        let ___args_2: Ptr<()> = (args_2).clone();
        let ___args_3: Ptr<()> = (args_3).clone();
        uses_allocator_construction_args_113(___alloc, ___args_1, ___args_2, ___args_3)
    });
}
pub fn test_uses_allocator_construction_args_114(args: Ptr<test_allocator_int_>) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({ uses_allocator_construction_args_115((args).clone()) });
}
pub fn test_uses_allocator_construction_args_116(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
    args_2: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args_1: Ptr<()> = (args_1).clone();
        let ___args_2: Ptr<()> = (args_2).clone();
        uses_allocator_construction_args_117(___alloc, ___args_1, ___args_2)
    });
}
pub fn test_uses_allocator_construction_args_118(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
    args_2: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args_1: Ptr<()> = (args_1).clone();
        let ___args_2: Ptr<()> = (args_2).clone();
        uses_allocator_construction_args_119(___alloc, ___args_1, ___args_2)
    });
}
pub fn test_uses_allocator_construction_args_120(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<()> = (args_1).clone();
        uses_allocator_construction_args_121(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_122(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<()> = (args_1).clone();
        uses_allocator_construction_args_123(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_124(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<()> = (args_1).clone();
        uses_allocator_construction_args_125(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_126(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<()> = (args_1).clone();
        uses_allocator_construction_args_127(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_128(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<ConvertibleToPair>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<ConvertibleToPair> = (args_1).clone();
        uses_allocator_construction_args_129(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_130(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<ConvertibleToPair>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<ConvertibleToPair> = (args_1).clone();
        uses_allocator_construction_args_131(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_132(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<()>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<()> = (args_1).clone();
        uses_allocator_construction_args_133(___alloc, ___args)
    });
}
pub fn test_uses_allocator_construction_args_134(
    args_0: Ptr<test_allocator_int_>,
    args_1: Ptr<std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_>,
) -> () {
    const _: () = assert!(
        true,
        "noexcept(std::uses_allocator_construction_args<Type>(std::forward<Args>(args)...))"
    );;
    return ({
        let ___alloc: Ptr<test_allocator_int_> = (args_0).clone();
        let ___args: Ptr<std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_> =
            (args_1).clone();
        uses_allocator_construction_args_135(___alloc, ___args)
    });
}
pub fn testOne_136() {
    let a: Value<test_allocator_int_> =
        Rc::new(RefCell::new(test_allocator_int_::new_1({ 12_() })));
    let _dtor_a = ScopedDestructor::new(&a, |__p| __p.destructor());
    {
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_16(a.as_pointer()) }),
        ));
        unimplemented!();
    }
    {
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_18(a.as_pointer()) }),
        ));
        unimplemented!();
    }
    {
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_20(a.as_pointer()) }),
        ));
    }
    {
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let _args_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let _args_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                test_uses_allocator_construction_args_22(
                    a.as_pointer(),
                    piecewise_construct_137.with(|v| v.as_pointer()),
                    _args_2.as_pointer(),
                    _args_3.as_pointer(),
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
    }
    {
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_24(a.as_pointer()) }),
        ));
        unimplemented!();
        unimplemented!();
    }
    {
        let val: Value<()> = Rc::new(RefCell::new(0_()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let _args_1: Ptr<()> = val.as_pointer();
                let _args_2: Ptr<()> = val.as_pointer();
                test_uses_allocator_construction_args_26(a.as_pointer(), _args_1, _args_2)
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let val: Value<()> = Rc::new(RefCell::new(0_()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let _args_1: Ptr<()> = (val.as_pointer()).reinterpret_cast::<()>();
                let _args_2: Ptr<()> = (val.as_pointer()).reinterpret_cast::<()>();
                test_uses_allocator_construction_args_28(a.as_pointer(), _args_1, _args_2)
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new({
            let __tmp_2: Value<()> = Rc::new(RefCell::new(3_()));
            let __tmp_3: Value<()> = Rc::new(RefCell::new(4_()));
            std_pair_int__int_::new_1({ __tmp_2.as_pointer() }, { __tmp_3.as_pointer() })
        }));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_30(a.as_pointer(), p.as_pointer()) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new({
            let __tmp_4: Value<()> = Rc::new(RefCell::new(3_()));
            let __tmp_5: Value<()> = Rc::new(RefCell::new(4_()));
            std_pair_int__int_::new_1({ __tmp_4.as_pointer() }, { __tmp_5.as_pointer() })
        }));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_32(a.as_pointer(), unimplemented!()) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new({
            let __tmp_6: Value<()> = Rc::new(RefCell::new(3_()));
            let __tmp_7: Value<()> = Rc::new(RefCell::new(4_()));
            std_pair_int__int_::new_1({ __tmp_6.as_pointer() }, { __tmp_7.as_pointer() })
        }));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                test_uses_allocator_construction_args_34(
                    a.as_pointer(),
                    (p.as_pointer()).reinterpret_cast::<()>(),
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        {
            let ret: Value<()> = Rc::new(RefCell::new(
                ({ test_uses_allocator_construction_args_4(a.as_pointer()) }),
            ));
            &(*ret.borrow_mut());
        }
        {
            let ret: Value<()> = Rc::new(RefCell::new(
                ({ test_uses_allocator_construction_args_6(a.as_pointer()) }),
            ));
            &(*ret.borrow_mut());
        }
        {
            let up: Value<()> = Rc::new(RefCell::new(1_()));
            let vp: Value<()> = Rc::new(RefCell::new(2_()));
            let ret: Value<()> = Rc::new(RefCell::new(
                ({
                    let _args_1: Ptr<()> = (up.as_pointer()).reinterpret_cast::<()>();
                    let _args_2: Ptr<()> = (vp.as_pointer()).reinterpret_cast::<()>();
                    test_uses_allocator_construction_args_8(a.as_pointer(), _args_1, _args_2)
                }),
            ));
            &(*ret.borrow_mut());
        }
        {
            let p: Value<()> = Rc::new(RefCell::new({
                let __tmp_8: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_9: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_int__int_::new_1({ __tmp_8.as_pointer() }, { __tmp_9.as_pointer() })
            }));
            let ret: Value<()> = Rc::new(RefCell::new(
                ({ test_uses_allocator_construction_args_10(a.as_pointer(), p.as_pointer()) }),
            ));
            &(*ret.borrow_mut());
        }
        {
            let p: Value<()> = Rc::new(RefCell::new({
                let __tmp_10: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_11: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_int__int_::new_1({ __tmp_10.as_pointer() }, { __tmp_11.as_pointer() })
            }));
            let ret: Value<()> = Rc::new(RefCell::new(
                ({
                    test_uses_allocator_construction_args_12(
                        a.as_pointer(),
                        (p.as_pointer()).reinterpret_cast::<()>(),
                    )
                }),
            ));
            &(*ret.borrow_mut());
        }
        {
            let p: Value<()> = Rc::new(RefCell::new({
                let __tmp_12: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_13: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_int__int_::new_1({ __tmp_12.as_pointer() }, { __tmp_13.as_pointer() })
            }));
            let ret: Value<()> = Rc::new(RefCell::new(
                ({ test_uses_allocator_construction_args_14(a.as_pointer(), p.as_pointer()) }),
            ));
            &(*ret.borrow_mut());
        }
    }
    {
        let p: Value<()> = Rc::new(RefCell::new({
            let __tmp_14: Value<()> = Rc::new(RefCell::new(3_()));
            let __tmp_15: Value<()> = Rc::new(RefCell::new(4_()));
            std_pair_int__int_::new_1({ __tmp_14.as_pointer() }, { __tmp_15.as_pointer() })
        }));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_36(a.as_pointer(), unimplemented!()) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let ctp: Value<ConvertibleToPair> = Rc::new(RefCell::new(<ConvertibleToPair>::default()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_38(a.as_pointer(), ctp.as_pointer()) }),
        ));
        let v: Value<()> = Rc::new(RefCell::new(
            ({ (*({ get_138(ret.as_pointer()) }).upgrade().deref())() }),
        ));
        unimplemented!();
        unimplemented!();
    }
    {
        let ctp: Value<ConvertibleToPair> = Rc::new(RefCell::new(<ConvertibleToPair>::default()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_40(a.as_pointer(), ctp.as_pointer()) }),
        ));
        let v: Value<()> = Rc::new(RefCell::new(
            ({ (*({ get_139(ret.as_pointer()) }).upgrade().deref())() }),
        ));
        unimplemented!();
        unimplemented!();
    }
    {
        let pair_like: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_42(a.as_pointer(), pair_like.as_pointer()) }),
        ));
        let pair: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let i: Value<()> = Rc::new(RefCell::new(5_()));
        let r: Value<std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_> =
            Rc::new(RefCell::new(
                std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_::new_1(
                    { (i.as_pointer()) },
                    { (i.as_pointer()) },
                ),
            ));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_44(a.as_pointer(), r.as_pointer()) }),
        ));
        let pair: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
}
pub fn testOne_142() {
    let a: Value<test_allocator_int_> =
        Rc::new(RefCell::new(test_allocator_int_::new_1({ 12_() })));
    let _dtor_a = ScopedDestructor::new(&a, |__p| __p.destructor());
    {
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_46(a.as_pointer()) }),
        ));
        unimplemented!();
    }
    {
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_48(a.as_pointer()) }),
        ));
        unimplemented!();
    }
    {
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_50(a.as_pointer()) }),
        ));
    }
    {
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let _args_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let _args_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                test_uses_allocator_construction_args_52(
                    a.as_pointer(),
                    piecewise_construct_137.with(|v| v.as_pointer()),
                    _args_2.as_pointer(),
                    _args_3.as_pointer(),
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
    }
    {
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_54(a.as_pointer()) }),
        ));
        unimplemented!();
        unimplemented!();
    }
    {
        let val: Value<()> = Rc::new(RefCell::new(0_()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let _args_1: Ptr<()> = val.as_pointer();
                let _args_2: Ptr<()> = val.as_pointer();
                test_uses_allocator_construction_args_56(a.as_pointer(), _args_1, _args_2)
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let val: Value<()> = Rc::new(RefCell::new(0_()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let _args_1: Ptr<()> = (val.as_pointer()).reinterpret_cast::<()>();
                let _args_2: Ptr<()> = (val.as_pointer()).reinterpret_cast::<()>();
                test_uses_allocator_construction_args_58(a.as_pointer(), _args_1, _args_2)
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new({
            let __tmp_16: Value<()> = Rc::new(RefCell::new(3_()));
            let __tmp_17: Value<()> = Rc::new(RefCell::new(4_()));
            std_pair_int__int_::new_1({ __tmp_16.as_pointer() }, { __tmp_17.as_pointer() })
        }));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_60(a.as_pointer(), p.as_pointer()) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new({
            let __tmp_18: Value<()> = Rc::new(RefCell::new(3_()));
            let __tmp_19: Value<()> = Rc::new(RefCell::new(4_()));
            std_pair_int__int_::new_1({ __tmp_18.as_pointer() }, { __tmp_19.as_pointer() })
        }));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_62(a.as_pointer(), unimplemented!()) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new({
            let __tmp_20: Value<()> = Rc::new(RefCell::new(3_()));
            let __tmp_21: Value<()> = Rc::new(RefCell::new(4_()));
            std_pair_int__int_::new_1({ __tmp_20.as_pointer() }, { __tmp_21.as_pointer() })
        }));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                test_uses_allocator_construction_args_64(
                    a.as_pointer(),
                    (p.as_pointer()).reinterpret_cast::<()>(),
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        {
            let ret: Value<()> = Rc::new(RefCell::new(
                ({ test_uses_allocator_construction_args_4(a.as_pointer()) }),
            ));
            &(*ret.borrow_mut());
        }
        {
            let ret: Value<()> = Rc::new(RefCell::new(
                ({ test_uses_allocator_construction_args_6(a.as_pointer()) }),
            ));
            &(*ret.borrow_mut());
        }
        {
            let up: Value<()> = Rc::new(RefCell::new(1_()));
            let vp: Value<()> = Rc::new(RefCell::new(2_()));
            let ret: Value<()> = Rc::new(RefCell::new(
                ({
                    let _args_1: Ptr<()> = (up.as_pointer()).reinterpret_cast::<()>();
                    let _args_2: Ptr<()> = (vp.as_pointer()).reinterpret_cast::<()>();
                    test_uses_allocator_construction_args_8(a.as_pointer(), _args_1, _args_2)
                }),
            ));
            &(*ret.borrow_mut());
        }
        {
            let p: Value<()> = Rc::new(RefCell::new({
                let __tmp_22: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_23: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_int__int_::new_1({ __tmp_22.as_pointer() }, { __tmp_23.as_pointer() })
            }));
            let ret: Value<()> = Rc::new(RefCell::new(
                ({ test_uses_allocator_construction_args_10(a.as_pointer(), p.as_pointer()) }),
            ));
            &(*ret.borrow_mut());
        }
        {
            let p: Value<()> = Rc::new(RefCell::new({
                let __tmp_24: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_25: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_int__int_::new_1({ __tmp_24.as_pointer() }, { __tmp_25.as_pointer() })
            }));
            let ret: Value<()> = Rc::new(RefCell::new(
                ({
                    test_uses_allocator_construction_args_12(
                        a.as_pointer(),
                        (p.as_pointer()).reinterpret_cast::<()>(),
                    )
                }),
            ));
            &(*ret.borrow_mut());
        }
        {
            let p: Value<()> = Rc::new(RefCell::new({
                let __tmp_26: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_27: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_int__int_::new_1({ __tmp_26.as_pointer() }, { __tmp_27.as_pointer() })
            }));
            let ret: Value<()> = Rc::new(RefCell::new(
                ({ test_uses_allocator_construction_args_14(a.as_pointer(), p.as_pointer()) }),
            ));
            &(*ret.borrow_mut());
        }
    }
    {
        let p: Value<()> = Rc::new(RefCell::new({
            let __tmp_28: Value<()> = Rc::new(RefCell::new(3_()));
            let __tmp_29: Value<()> = Rc::new(RefCell::new(4_()));
            std_pair_int__int_::new_1({ __tmp_28.as_pointer() }, { __tmp_29.as_pointer() })
        }));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_66(a.as_pointer(), unimplemented!()) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let ctp: Value<ConvertibleToPair> = Rc::new(RefCell::new(<ConvertibleToPair>::default()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_68(a.as_pointer(), ctp.as_pointer()) }),
        ));
        let v: Value<()> = Rc::new(RefCell::new(
            ({ (*({ get_143(ret.as_pointer()) }).upgrade().deref())() }),
        ));
        unimplemented!();
        unimplemented!();
    }
    {
        let ctp: Value<ConvertibleToPair> = Rc::new(RefCell::new(<ConvertibleToPair>::default()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_70(a.as_pointer(), ctp.as_pointer()) }),
        ));
        let v: Value<()> = Rc::new(RefCell::new(
            ({ (*({ get_144(ret.as_pointer()) }).upgrade().deref())() }),
        ));
        unimplemented!();
        unimplemented!();
    }
    {
        let pair_like: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_72(a.as_pointer(), pair_like.as_pointer()) }),
        ));
        let pair: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let i: Value<()> = Rc::new(RefCell::new(5_()));
        let r: Value<std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_> =
            Rc::new(RefCell::new(
                std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_::new_1(
                    { (i.as_pointer()) },
                    { (i.as_pointer()) },
                ),
            ));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_74(a.as_pointer(), r.as_pointer()) }),
        ));
        let pair: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
}
pub fn testOne_147() {
    let a: Value<test_allocator_int_> =
        Rc::new(RefCell::new(test_allocator_int_::new_1({ 12_() })));
    let _dtor_a = ScopedDestructor::new(&a, |__p| __p.destructor());
    {
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_76(a.as_pointer()) }),
        ));
        unimplemented!();
    }
    {
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_78(a.as_pointer()) }),
        ));
        unimplemented!();
    }
    {
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_80(a.as_pointer()) }),
        ));
    }
    {
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let _args_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let _args_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                test_uses_allocator_construction_args_82(
                    a.as_pointer(),
                    piecewise_construct_137.with(|v| v.as_pointer()),
                    _args_2.as_pointer(),
                    _args_3.as_pointer(),
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
    }
    {
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_84(a.as_pointer()) }),
        ));
        unimplemented!();
        unimplemented!();
    }
    {
        let val: Value<()> = Rc::new(RefCell::new(0_()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let _args_1: Ptr<()> = val.as_pointer();
                let _args_2: Ptr<()> = val.as_pointer();
                test_uses_allocator_construction_args_86(a.as_pointer(), _args_1, _args_2)
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let val: Value<()> = Rc::new(RefCell::new(0_()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let _args_1: Ptr<()> = (val.as_pointer()).reinterpret_cast::<()>();
                let _args_2: Ptr<()> = (val.as_pointer()).reinterpret_cast::<()>();
                test_uses_allocator_construction_args_88(a.as_pointer(), _args_1, _args_2)
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new({
            let __tmp_30: Value<()> = Rc::new(RefCell::new(3_()));
            let __tmp_31: Value<()> = Rc::new(RefCell::new(4_()));
            std_pair_int__int_::new_1({ __tmp_30.as_pointer() }, { __tmp_31.as_pointer() })
        }));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_90(a.as_pointer(), p.as_pointer()) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new({
            let __tmp_32: Value<()> = Rc::new(RefCell::new(3_()));
            let __tmp_33: Value<()> = Rc::new(RefCell::new(4_()));
            std_pair_int__int_::new_1({ __tmp_32.as_pointer() }, { __tmp_33.as_pointer() })
        }));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_92(a.as_pointer(), unimplemented!()) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new({
            let __tmp_34: Value<()> = Rc::new(RefCell::new(3_()));
            let __tmp_35: Value<()> = Rc::new(RefCell::new(4_()));
            std_pair_int__int_::new_1({ __tmp_34.as_pointer() }, { __tmp_35.as_pointer() })
        }));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                test_uses_allocator_construction_args_94(
                    a.as_pointer(),
                    (p.as_pointer()).reinterpret_cast::<()>(),
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        {
            let ret: Value<()> = Rc::new(RefCell::new(
                ({ test_uses_allocator_construction_args_4(a.as_pointer()) }),
            ));
            &(*ret.borrow_mut());
        }
        {
            let ret: Value<()> = Rc::new(RefCell::new(
                ({ test_uses_allocator_construction_args_6(a.as_pointer()) }),
            ));
            &(*ret.borrow_mut());
        }
        {
            let up: Value<()> = Rc::new(RefCell::new(1_()));
            let vp: Value<()> = Rc::new(RefCell::new(2_()));
            let ret: Value<()> = Rc::new(RefCell::new(
                ({
                    let _args_1: Ptr<()> = (up.as_pointer()).reinterpret_cast::<()>();
                    let _args_2: Ptr<()> = (vp.as_pointer()).reinterpret_cast::<()>();
                    test_uses_allocator_construction_args_8(a.as_pointer(), _args_1, _args_2)
                }),
            ));
            &(*ret.borrow_mut());
        }
        {
            let p: Value<()> = Rc::new(RefCell::new({
                let __tmp_36: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_37: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_int__int_::new_1({ __tmp_36.as_pointer() }, { __tmp_37.as_pointer() })
            }));
            let ret: Value<()> = Rc::new(RefCell::new(
                ({ test_uses_allocator_construction_args_10(a.as_pointer(), p.as_pointer()) }),
            ));
            &(*ret.borrow_mut());
        }
        {
            let p: Value<()> = Rc::new(RefCell::new({
                let __tmp_38: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_39: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_int__int_::new_1({ __tmp_38.as_pointer() }, { __tmp_39.as_pointer() })
            }));
            let ret: Value<()> = Rc::new(RefCell::new(
                ({
                    test_uses_allocator_construction_args_12(
                        a.as_pointer(),
                        (p.as_pointer()).reinterpret_cast::<()>(),
                    )
                }),
            ));
            &(*ret.borrow_mut());
        }
        {
            let p: Value<()> = Rc::new(RefCell::new({
                let __tmp_40: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_41: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_int__int_::new_1({ __tmp_40.as_pointer() }, { __tmp_41.as_pointer() })
            }));
            let ret: Value<()> = Rc::new(RefCell::new(
                ({ test_uses_allocator_construction_args_14(a.as_pointer(), p.as_pointer()) }),
            ));
            &(*ret.borrow_mut());
        }
    }
    {
        let p: Value<()> = Rc::new(RefCell::new({
            let __tmp_42: Value<()> = Rc::new(RefCell::new(3_()));
            let __tmp_43: Value<()> = Rc::new(RefCell::new(4_()));
            std_pair_int__int_::new_1({ __tmp_42.as_pointer() }, { __tmp_43.as_pointer() })
        }));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_96(a.as_pointer(), unimplemented!()) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let ctp: Value<ConvertibleToPair> = Rc::new(RefCell::new(<ConvertibleToPair>::default()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_98(a.as_pointer(), ctp.as_pointer()) }),
        ));
        let v: Value<()> = Rc::new(RefCell::new(
            ({ (*({ get_148(ret.as_pointer()) }).upgrade().deref())() }),
        ));
        unimplemented!();
        unimplemented!();
    }
    {
        let ctp: Value<ConvertibleToPair> = Rc::new(RefCell::new(<ConvertibleToPair>::default()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_100(a.as_pointer(), ctp.as_pointer()) }),
        ));
        let v: Value<()> = Rc::new(RefCell::new(
            ({ (*({ get_149(ret.as_pointer()) }).upgrade().deref())() }),
        ));
        unimplemented!();
        unimplemented!();
    }
    {
        let pair_like: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_102(a.as_pointer(), pair_like.as_pointer()) }),
        ));
        let pair: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let i: Value<()> = Rc::new(RefCell::new(5_()));
        let r: Value<std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_> =
            Rc::new(RefCell::new(
                std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_::new_1(
                    { (i.as_pointer()) },
                    { (i.as_pointer()) },
                ),
            ));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_104(a.as_pointer(), r.as_pointer()) }),
        ));
        let pair: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
}
pub fn testOne_152() {
    let a: Value<test_allocator_int_> =
        Rc::new(RefCell::new(test_allocator_int_::new_1({ 12_() })));
    let _dtor_a = ScopedDestructor::new(&a, |__p| __p.destructor());
    {
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_106(a.as_pointer()) }),
        ));
        unimplemented!();
    }
    {
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_108(a.as_pointer()) }),
        ));
        unimplemented!();
    }
    {
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_110(a.as_pointer()) }),
        ));
    }
    {
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let _args_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let _args_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                test_uses_allocator_construction_args_112(
                    a.as_pointer(),
                    piecewise_construct_137.with(|v| v.as_pointer()),
                    _args_2.as_pointer(),
                    _args_3.as_pointer(),
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
    }
    {
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_114(a.as_pointer()) }),
        ));
        unimplemented!();
        unimplemented!();
    }
    {
        let val: Value<()> = Rc::new(RefCell::new(0_()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let _args_1: Ptr<()> = val.as_pointer();
                let _args_2: Ptr<()> = val.as_pointer();
                test_uses_allocator_construction_args_116(a.as_pointer(), _args_1, _args_2)
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let val: Value<()> = Rc::new(RefCell::new(0_()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                let _args_1: Ptr<()> = (val.as_pointer()).reinterpret_cast::<()>();
                let _args_2: Ptr<()> = (val.as_pointer()).reinterpret_cast::<()>();
                test_uses_allocator_construction_args_118(a.as_pointer(), _args_1, _args_2)
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new({
            let __tmp_44: Value<()> = Rc::new(RefCell::new(3_()));
            let __tmp_45: Value<()> = Rc::new(RefCell::new(4_()));
            std_pair_int__int_::new_1({ __tmp_44.as_pointer() }, { __tmp_45.as_pointer() })
        }));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_120(a.as_pointer(), p.as_pointer()) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new({
            let __tmp_46: Value<()> = Rc::new(RefCell::new(3_()));
            let __tmp_47: Value<()> = Rc::new(RefCell::new(4_()));
            std_pair_int__int_::new_1({ __tmp_46.as_pointer() }, { __tmp_47.as_pointer() })
        }));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_122(a.as_pointer(), unimplemented!()) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new({
            let __tmp_48: Value<()> = Rc::new(RefCell::new(3_()));
            let __tmp_49: Value<()> = Rc::new(RefCell::new(4_()));
            std_pair_int__int_::new_1({ __tmp_48.as_pointer() }, { __tmp_49.as_pointer() })
        }));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({
                test_uses_allocator_construction_args_124(
                    a.as_pointer(),
                    (p.as_pointer()).reinterpret_cast::<()>(),
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        {
            let ret: Value<()> = Rc::new(RefCell::new(
                ({ test_uses_allocator_construction_args_4(a.as_pointer()) }),
            ));
            &(*ret.borrow_mut());
        }
        {
            let ret: Value<()> = Rc::new(RefCell::new(
                ({ test_uses_allocator_construction_args_6(a.as_pointer()) }),
            ));
            &(*ret.borrow_mut());
        }
        {
            let up: Value<()> = Rc::new(RefCell::new(1_()));
            let vp: Value<()> = Rc::new(RefCell::new(2_()));
            let ret: Value<()> = Rc::new(RefCell::new(
                ({
                    let _args_1: Ptr<()> = (up.as_pointer()).reinterpret_cast::<()>();
                    let _args_2: Ptr<()> = (vp.as_pointer()).reinterpret_cast::<()>();
                    test_uses_allocator_construction_args_8(a.as_pointer(), _args_1, _args_2)
                }),
            ));
            &(*ret.borrow_mut());
        }
        {
            let p: Value<()> = Rc::new(RefCell::new({
                let __tmp_50: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_51: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_int__int_::new_1({ __tmp_50.as_pointer() }, { __tmp_51.as_pointer() })
            }));
            let ret: Value<()> = Rc::new(RefCell::new(
                ({ test_uses_allocator_construction_args_10(a.as_pointer(), p.as_pointer()) }),
            ));
            &(*ret.borrow_mut());
        }
        {
            let p: Value<()> = Rc::new(RefCell::new({
                let __tmp_52: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_53: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_int__int_::new_1({ __tmp_52.as_pointer() }, { __tmp_53.as_pointer() })
            }));
            let ret: Value<()> = Rc::new(RefCell::new(
                ({
                    test_uses_allocator_construction_args_12(
                        a.as_pointer(),
                        (p.as_pointer()).reinterpret_cast::<()>(),
                    )
                }),
            ));
            &(*ret.borrow_mut());
        }
        {
            let p: Value<()> = Rc::new(RefCell::new({
                let __tmp_54: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_55: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_int__int_::new_1({ __tmp_54.as_pointer() }, { __tmp_55.as_pointer() })
            }));
            let ret: Value<()> = Rc::new(RefCell::new(
                ({ test_uses_allocator_construction_args_14(a.as_pointer(), p.as_pointer()) }),
            ));
            &(*ret.borrow_mut());
        }
    }
    {
        let p: Value<()> = Rc::new(RefCell::new({
            let __tmp_56: Value<()> = Rc::new(RefCell::new(3_()));
            let __tmp_57: Value<()> = Rc::new(RefCell::new(4_()));
            std_pair_int__int_::new_1({ __tmp_56.as_pointer() }, { __tmp_57.as_pointer() })
        }));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_126(a.as_pointer(), unimplemented!()) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let ctp: Value<ConvertibleToPair> = Rc::new(RefCell::new(<ConvertibleToPair>::default()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_128(a.as_pointer(), ctp.as_pointer()) }),
        ));
        let v: Value<()> = Rc::new(RefCell::new(
            ({ (*({ get_153(ret.as_pointer()) }).upgrade().deref())() }),
        ));
        unimplemented!();
        unimplemented!();
    }
    {
        let ctp: Value<ConvertibleToPair> = Rc::new(RefCell::new(<ConvertibleToPair>::default()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_130(a.as_pointer(), ctp.as_pointer()) }),
        ));
        let v: Value<()> = Rc::new(RefCell::new(
            ({ (*({ get_154(ret.as_pointer()) }).upgrade().deref())() }),
        ));
        unimplemented!();
        unimplemented!();
    }
    {
        let pair_like: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_132(a.as_pointer(), pair_like.as_pointer()) }),
        ));
        let pair: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let i: Value<()> = Rc::new(RefCell::new(5_()));
        let r: Value<std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_> =
            Rc::new(RefCell::new(
                std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_::new_1(
                    { (i.as_pointer()) },
                    { (i.as_pointer()) },
                ),
            ));
        let ret: Value<()> = Rc::new(RefCell::new(
            ({ test_uses_allocator_construction_args_134(a.as_pointer(), r.as_pointer()) }),
        ));
        let pair: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Foo_140 {
    #[offset(0)]
    pub i: (),
}
impl Default for Foo_140 {
    fn default() -> Self {
        { Foo_140 { i: 5_() } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Bar_141 {
    #[offset(0)]
    pub i: (),
}
impl Bar_141 {
    pub fn new(mut foo: Foo_140) -> Self {
        Self { i: foo.i }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Foo_145 {
    #[offset(0)]
    pub i: (),
}
impl Default for Foo_145 {
    fn default() -> Self {
        { Foo_145 { i: 5_() } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Bar_146 {
    #[offset(0)]
    pub i: (),
}
impl Bar_146 {
    pub fn new(mut foo: Foo_145) -> Self {
        Self { i: foo.i }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Foo_150 {
    #[offset(0)]
    pub i: (),
}
impl Default for Foo_150 {
    fn default() -> Self {
        { Foo_150 { i: 5_() } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Bar_151 {
    #[offset(0)]
    pub i: (),
}
impl Bar_151 {
    pub fn new(mut foo: Foo_150) -> Self {
        Self { i: foo.i }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Foo_155 {
    #[offset(0)]
    pub i: (),
}
impl Default for Foo_155 {
    fn default() -> Self {
        { Foo_155 { i: 5_() } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Bar_156 {
    #[offset(0)]
    pub i: (),
}
impl Bar_156 {
    pub fn new(mut foo: Foo_155) -> Self {
        Self { i: foo.i }
    }
}
pub fn test_157() -> () {
    ({ testOne_136() });
    ({ testOne_142() });
    ({ testOne_147() });
    ({ testOne_152() });
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_157() });
    const _: () = assert!(true, "test()");;
    0
}
pub trait ConvertibleToPairImpl {
    fn to_(&self) -> ();
}
impl ConvertibleToPairImpl for Ptr<ConvertibleToPair> {
    fn to_(&self) -> () {
        return {
            let __tmp_0: Value<()> = Rc::new(RefCell::new(1_()));
            let __tmp_1: Value<()> = Rc::new(RefCell::new(2_()));
            std_pair_int__int_::new_1({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
        };
    }
}
pub trait Tag_XImpl {
    fn copy_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X>;
    fn move_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X>;
}
impl Tag_XImpl for Ptr<Tag_X> {
    fn copy_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X> {
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X> {
        return (*self).clone();
    }
}
pub trait UsesAllocArgTImpl {
    fn destructor(&self);
}
impl UsesAllocArgTImpl for Ptr<UsesAllocArgT> {
    fn destructor(&self) {
        field_ptr!(self, a_).destructor();
    }
}
pub trait UsesAllocLastImpl {
    fn destructor(&self);
}
impl UsesAllocLastImpl for Ptr<UsesAllocLast> {
    fn destructor(&self) {
        field_ptr!(self, a_).destructor();
    }
}
pub trait test_allocator_int_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), mut _a1: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<()>) {
        unimplemented!()
    }
    fn get_data(&self) -> ();
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_int_Impl for Ptr<test_allocator_int_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
    }
    fn get_data(&self) -> () {
        return (*self).with(|__s| __s.data_);
    }
}
pub trait test_allocator_statisticsImpl {
    fn clear(&self);
    fn clear_ctor_counters(&self);
}
impl test_allocator_statisticsImpl for Ptr<test_allocator_statistics> {
    fn clear(&self) {
        unimplemented!();
        field!((*self), count).write(0_());
        field!((*self), time_to_throw).write(0_());
        field!((*self), alloc_count).write(0_());
        field!((*self), allocated_size).write(0_());
        field!((*self), construct_count).write(0_());
        field!((*self), destroy_count).write(0_());
        field!((*self), throw_after).write(2147483647_());
        ({ test_allocator_statisticsImpl::clear_ctor_counters(self) });
    }
    fn clear_ctor_counters(&self) {
        field!((*self), copied).write(0_());
        field!((*self), moved).write(0_());
        field!((*self), converted).write(0_());
    }
}
pub trait test_allocator_void_Impl {
    fn destructor(&self);
    fn get_id(&self) -> ();
    fn get_data(&self) -> ();
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_>;
}
impl test_allocator_void_Impl for Ptr<test_allocator_void_> {
    fn destructor(&self) {
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
    }
    fn get_id(&self) -> () {
        return (*self).with(|__s| __s.id_);
    }
    fn get_data(&self) -> () {
        return (*self).with(|__s| __s.data_);
    }
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_> {
        field!((*self), data_).write({ _a0.with(|__s| __s.data_) });
        field!((*self), id_).write({ _a0.with(|__s| __s.id_) });
        field!((*self), stats_).write({ _a0.with(|__s| __s.stats_.clone()) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = destructed_value_0.with(|_| ());
    let _ = moved_value_1.with(|_| ());
}
