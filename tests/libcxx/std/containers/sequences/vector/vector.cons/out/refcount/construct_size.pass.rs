extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn DoNotOptimize_0(value: Ptr<()>) -> Ptr<()> {
    b"";
    return (value).clone();
}
pub fn operator_eq_2(x: Ptr<DefaultOnly>, y: Ptr<DefaultOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_lt_3(x: Ptr<DefaultOnly>, y: Ptr<DefaultOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(4)]
pub struct DefaultOnly {
    #[offset(0)]
    data_: (),
}
impl DefaultOnly {
    pub fn new() -> Self {
        let __this: DefaultOnly = Self { data_: -1_() };
        (*count_1.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl std::cmp::Ord for DefaultOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_3(
                Rc::new(RefCell::new(DefaultOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(DefaultOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_3(
                Rc::new(RefCell::new(DefaultOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(DefaultOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for DefaultOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for DefaultOnly {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_2(
                Rc::new(RefCell::new(DefaultOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(DefaultOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for DefaultOnly {}
impl Default for DefaultOnly {
    fn default() -> Self {
        { DefaultOnly::new() }
    }
}
thread_local!(
    pub static count_1: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_5.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_6.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_7.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_4.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_5.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_6.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_4: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_5: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_6: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_7: Value<()> = Rc::new(RefCell::new(false));
);
pub fn operator_sub_8(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as () as ());
}
pub fn operator_lt_9(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } < { (y.ptr_).clone() });
}
pub fn operator_gt_10(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = y;
        operator_lt_9(_x, x)
    });
}
pub fn operator_le_11(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return !({
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = y;
        operator_lt_9(_x, x)
    });
}
pub fn operator_eq_12(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_13(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return !({
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_12(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn new_2(mut _a0: ()) -> Self {
        Self {
            ptr_: Ptr::<()>::null(),
        }
    }
}
impl std::cmp::Ord for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_9(self.clone(), other.clone()) {
                std::cmp::Ordering::Less
            } else if operator_lt_9(other.clone(), self.clone()) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_12(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub fn operator_sub_14(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as () as ());
}
pub fn operator_eq_15(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_16(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } != { (y.ptr_).clone() });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    pub fn new_1(mut p: min_pointer_int__std_integral_constant_unsigned_long__0__) -> Self {
        Self {
            ptr_: (p.ptr_).clone(),
        }
    }
}
impl std::cmp::PartialEq for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_15(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
pub fn operator_sub_17(
    mut x: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as () as ());
}
pub fn operator_lt_18(
    mut x: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } < { (y.ptr_).clone() });
}
pub fn operator_gt_19(
    mut x: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({
        let _x: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ = y;
        operator_lt_18(_x, x)
    });
}
pub fn operator_le_20(
    mut x: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return !({
        let _x: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ = y;
        operator_lt_18(_x, x)
    });
}
pub fn operator_eq_21(
    mut x: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_22(
    mut x: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return !({
        let _x: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_21(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<DefaultOnly>,
}
impl min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<DefaultOnly>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn new_2(mut _a0: ()) -> Self {
        Self {
            ptr_: Ptr::<DefaultOnly>::null(),
        }
    }
}
impl std::cmp::Ord for min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_18(self.clone(), other.clone()) {
                std::cmp::Ordering::Less
            } else if operator_lt_18(other.clone(), self.clone()) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_21(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {}
pub fn operator_sub_23(
    mut x: min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as () as ());
}
pub fn operator_eq_24(
    mut x: min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_25(
    mut x: min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } != { (y.ptr_).clone() });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<DefaultOnly>,
}
impl min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__ {
    pub fn new_1(mut p: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__) -> Self {
        Self {
            ptr_: (p.ptr_).clone(),
        }
    }
}
impl std::cmp::PartialEq
    for min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__
{
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_24(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {}
pub fn operator_eq_26(mut _a0: min_allocator_int_, mut _a1: min_allocator_int_) -> () {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
impl std::cmp::PartialEq for min_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_26(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_int_ {}
pub fn operator_eq_27(
    mut _a0: min_allocator_DefaultOnly_,
    mut _a1: min_allocator_DefaultOnly_,
) -> () {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_DefaultOnly_ {}
impl std::cmp::PartialEq for min_allocator_DefaultOnly_ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_27(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_DefaultOnly_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_unsigned_long_ {}
pub fn operator_eq_28(mut _a0: safe_allocator_int_, mut _a1: safe_allocator_int_) -> () {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct safe_allocator_int_ {}
impl safe_allocator_int_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl std::cmp::PartialEq for safe_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_28(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for safe_allocator_int_ {}
impl Default for safe_allocator_int_ {
    fn default() -> Self {
        { safe_allocator_int_::new() }
    }
}
pub fn operator_eq_30(
    mut _a0: safe_allocator_DefaultOnly_,
    mut _a1: safe_allocator_DefaultOnly_,
) -> () {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct safe_allocator_DefaultOnly_ {}
impl safe_allocator_DefaultOnly_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl std::cmp::PartialEq for safe_allocator_DefaultOnly_ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_30(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for safe_allocator_DefaultOnly_ {}
impl Default for safe_allocator_DefaultOnly_ {
    fn default() -> Self {
        { safe_allocator_DefaultOnly_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct safe_allocator_unsigned_long_ {}
impl safe_allocator_unsigned_long_ {}
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
    pub static destructed_value_31: Value<()> = Rc::new(RefCell::new(-1_()));
);
thread_local!(
    pub static moved_value_32: Value<()> = Rc::new(RefCell::new(2147483647_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_33(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_34(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_33(_x, _y)
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
            operator_eq_33(
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
pub fn operator_eq_35(
    x: Ptr<test_allocator_DefaultOnly_>,
    y: Ptr<test_allocator_DefaultOnly_>,
) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_DefaultOnly__rebind_DefaultOnly_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_DefaultOnly__rebind_unsigned_int_ {}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_DefaultOnly_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_DefaultOnly_ {
    pub fn new_1(mut data: ()) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn copy_from(a: Ptr<test_allocator_DefaultOnly_>) -> Self {
        let __this: Value<test_allocator_DefaultOnly_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_DefaultOnly_> = __this.as_pointer();
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::PartialEq for test_allocator_DefaultOnly_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_35(
                Rc::new(RefCell::new(test_allocator_DefaultOnly_ {
                    data_: self.data_.clone(),
                    id_: self.id_.clone(),
                    stats_: self.stats_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(test_allocator_DefaultOnly_ {
                    data_: other.data_.clone(),
                    id_: other.id_.clone(),
                    stats_: other.stats_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_DefaultOnly_ {}
impl Clone for test_allocator_DefaultOnly_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_DefaultOnly_> =
            Rc::new(RefCell::new(test_allocator_DefaultOnly_ {
                data_: self.data_.clone(),
                id_: self.id_.clone(),
                stats_: self.stats_.clone(),
            }));
        test_allocator_DefaultOnly_::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_DefaultOnly_ {
    fn default() -> Self {
        {
            test_allocator_DefaultOnly_ {
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(16)]
pub struct test_allocator_unsigned_int_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_unsigned_int_ {}
impl Default for test_allocator_unsigned_int_ {
    fn default() -> Self {
        {
            test_allocator_unsigned_int_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
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
pub fn is_contiguous_container_asan_correct_36(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_contiguous_container_asan_correct_37(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_contiguous_container_asan_correct_38(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_contiguous_container_asan_correct_39(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_contiguous_container_asan_correct_40(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_contiguous_container_asan_correct_41(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_contiguous_container_asan_correct_42(_a0: Ptr<()>) -> () {
    return true;
}
pub fn test_43(mut n: (), a: Option<Ptr<()>>) {
    let mut __a_default: Option<Value<()>> = None;
    let a: Ptr<()> = a.unwrap_or_else(|| {
        __a_default
            .insert(Rc::new(RefCell::new(unimplemented!())))
            .as_pointer()
    });
    &(*a.upgrade().deref());
    {
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        'loop_: while !(unimplemented!()) {
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        'loop_: while !(unimplemented!()) {
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_44(mut n: (), a: Option<Ptr<min_allocator_int_>>) {
    let mut __a_default: Option<Value<min_allocator_int_>> = None;
    let a: Ptr<min_allocator_int_> = a.unwrap_or_else(|| {
        __a_default
            .insert(Rc::new(RefCell::new(<min_allocator_int_>::default())))
            .as_pointer()
    });
    &(*a.upgrade().deref());
    {
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        'loop_: while !(unimplemented!()) {
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        'loop_: while !(unimplemented!()) {
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_45(mut n: (), a: Option<Ptr<safe_allocator_int_>>) {
    let mut __a_default: Option<Value<safe_allocator_int_>> = None;
    let a: Ptr<safe_allocator_int_> = a.unwrap_or_else(|| {
        __a_default
            .insert(Rc::new(RefCell::new(safe_allocator_int_::new())))
            .as_pointer()
    });
    &(*a.upgrade().deref());
    {
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        'loop_: while !(unimplemented!()) {
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        'loop_: while !(unimplemented!()) {
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_46(mut n: (), a: Option<Ptr<()>>) {
    let mut __a_default: Option<Value<()>> = None;
    let a: Ptr<()> = a.unwrap_or_else(|| {
        __a_default
            .insert(Rc::new(RefCell::new(unimplemented!())))
            .as_pointer()
    });
    &(*a.upgrade().deref());
    {
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        'loop_: while !(unimplemented!()) {
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        'loop_: while !(unimplemented!()) {
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_47(mut n: (), a: Option<Ptr<min_allocator_DefaultOnly_>>) {
    let mut __a_default: Option<Value<min_allocator_DefaultOnly_>> = None;
    let a: Ptr<min_allocator_DefaultOnly_> = a.unwrap_or_else(|| {
        __a_default
            .insert(Rc::new(RefCell::new(
                <min_allocator_DefaultOnly_>::default(),
            )))
            .as_pointer()
    });
    &(*a.upgrade().deref());
    {
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        'loop_: while !(unimplemented!()) {
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        'loop_: while !(unimplemented!()) {
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_48(mut n: (), a: Option<Ptr<safe_allocator_DefaultOnly_>>) {
    let mut __a_default: Option<Value<safe_allocator_DefaultOnly_>> = None;
    let a: Ptr<safe_allocator_DefaultOnly_> = a.unwrap_or_else(|| {
        __a_default
            .insert(Rc::new(RefCell::new(safe_allocator_DefaultOnly_::new())))
            .as_pointer()
    });
    &(*a.upgrade().deref());
    {
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        'loop_: while !(unimplemented!()) {
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        'loop_: while !(unimplemented!()) {
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_49(mut n: (), a: Ptr<test_allocator_DefaultOnly_>) {
    &(*a.upgrade().deref());
    {
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        'loop_: while !(unimplemented!()) {
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        'loop_: while !(unimplemented!()) {
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn tests_50() -> () {
    ({ test_43(0_(), None) });
    ({ test_43(50_(), None) });
    ({ test_44(0_(), None) });
    ({ test_44(50_(), None) });
    ({ test_45(0_(), None) });
    ({ test_45(50_(), None) });
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
    ({ tests_50() });
    const _: () = assert!(true, "tests()");;
    ({ test_46(0_(), None) });
    ({ test_46(500_(), None) });
    unimplemented!();
    ({ test_47(0_(), None) });
    ({ test_47(500_(), None) });
    ({ test_48(0_(), None) });
    ({ test_48(500_(), None) });
    ({
        let _a: Value<test_allocator_DefaultOnly_> =
            Rc::new(RefCell::new(test_allocator_DefaultOnly_::new_1({ 23_() })));
        test_49(0_(), _a.as_pointer())
    });
    ({
        let _a: Value<test_allocator_DefaultOnly_> =
            Rc::new(RefCell::new(test_allocator_DefaultOnly_::new_1({ 23_() })));
        test_49(100_(), _a.as_pointer())
    });
    unimplemented!();
    return 0_();
}
pub trait DefaultOnlyImpl {
    fn copy_assign(&self, _a0: Ptr<DefaultOnly>) -> Ptr<DefaultOnly> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl DefaultOnlyImpl for Ptr<DefaultOnly> {
    fn destructor(&self) {
        field!((*self), data_).write(0_());
        (*count_1.with(Value::clone).borrow_mut()).prefix_dec();
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
pub trait min_allocator_DefaultOnly_Impl {
    fn allocate(&self, n: ()) -> min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__;
    fn deallocate(
        &self,
        p: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__,
        n: (),
    );
}
impl min_allocator_DefaultOnly_Impl for Ptr<min_allocator_DefaultOnly_> {
    fn allocate(
        &self,
        mut n: (),
    ) -> min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {
        return min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub trait min_allocator_int_Impl {
    fn allocate(&self, n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_int__std_integral_constant_unsigned_long__0__, n: ());
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {
    fn allocate(&self, mut n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub trait min_allocator_unsigned_long_Impl {
    fn allocate(
        &self,
        mut n: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_unsigned_long_Impl for Ptr<min_allocator_unsigned_long_> {}
pub trait min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<DefaultOnly> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<DefaultOnly>;
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__>;
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__>;
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: (),
    ) -> Ptr<min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        n: (),
    ) -> min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__;
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<DefaultOnly> {
        unimplemented!()
    }
}
impl min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__>
{
    fn operator_arrow(&self) -> Ptr<DefaultOnly> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__Impl :: operator_add_assign ( &tmp .as_pointer()  , n  , )
        });
        return (*tmp.borrow()).clone();
    }
}
pub trait min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<DefaultOnly>;
    fn operator_arrow(&self) -> Ptr<DefaultOnly> {
        unimplemented!()
    }
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__>;
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<DefaultOnly> {
        unimplemented!()
    }
}
impl min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__>
{
    fn operator_deref(&self) -> Ptr<DefaultOnly> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
}
pub trait min_pointer_const_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__>;
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__>
{
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
}
pub trait min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: (),
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(&self, n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__;
    fn operator_sub(&self, mut n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>
{
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_add(&self, mut n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_int__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_int__std_integral_constant_unsigned_long__0__Impl::operator_add_assign(
                &tmp.as_pointer(),
                n,
            )
        });
        return (*tmp.borrow()).clone();
    }
}
pub trait min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait safe_allocator_DefaultOnly_Impl {
    fn allocate(&self, n: ()) -> Ptr<DefaultOnly>;
    fn deallocate(&self, p: Ptr<DefaultOnly>, n: ());
}
impl safe_allocator_DefaultOnly_Impl for Ptr<safe_allocator_DefaultOnly_> {
    fn allocate(&self, mut n: ()) -> Ptr<DefaultOnly> {
        let memory: Value<Ptr<DefaultOnly>> = Rc::new(RefCell::new(unimplemented!()));
        {
            ({
                memset_29(
                    (*memory.borrow()).to_any(),
                    0_(),
                    (4usize as ()).wrapping_mul(n),
                )
            });
        }
        return (*memory.borrow()).clone();
    }
    fn deallocate(&self, mut p: Ptr<DefaultOnly>, mut n: ()) {
        {
            ({
                let _value: Value<()> = Rc::new(RefCell::new(
                    ({ memset_29((p).to_any(), 0_(), (4usize as ()).wrapping_mul(n)) }),
                ));
                DoNotOptimize_0(_value.as_pointer())
            });
        }
        unimplemented!();
    }
}
pub trait safe_allocator_int_Impl {
    fn allocate(&self, n: ()) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, n: ());
}
impl safe_allocator_int_Impl for Ptr<safe_allocator_int_> {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        let memory: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
        {
            ({
                let ___s: () = (*memory.borrow()).to_any();
                memset_29(
                    ___s,
                    0_(),
                    (::std::mem::size_of::<()>() as ()).wrapping_mul(n),
                )
            });
        }
        return (*memory.borrow()).clone();
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        {
            ({
                let _value: Value<()> = Rc::new(RefCell::new(
                    ({
                        let ___s: () = (p).to_any();
                        memset_29(
                            ___s,
                            0_(),
                            (::std::mem::size_of::<()>() as ()).wrapping_mul(n),
                        )
                    }),
                ));
                DoNotOptimize_0(_value.as_pointer())
            });
        }
        unimplemented!();
    }
}
pub trait safe_allocator_unsigned_long_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!()
    }
}
impl safe_allocator_unsigned_long_Impl for Ptr<safe_allocator_unsigned_long_> {}
pub trait test_allocator_DefaultOnly_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<DefaultOnly> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<DefaultOnly> {
        unimplemented!()
    }
    fn allocate(&self, n: (), _a1: Option<()>) -> Ptr<DefaultOnly>;
    fn deallocate(&self, p: Ptr<DefaultOnly>, s: ());
    fn max_size(&self) -> ();
    fn destroy(&self, p: Ptr<DefaultOnly>);
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_DefaultOnly_Impl for Ptr<test_allocator_DefaultOnly_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_31.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_31.with(|rc| *rc.borrow()));
    }
    fn allocate(&self, mut n: (), _a1: Option<()>) -> Ptr<DefaultOnly> {
        let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            if ({
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.time_to_throw)
            } >= {
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.throw_after)
            }) {
                unimplemented!();
            }
            field!((*self).with(|__s| __s.stats_.clone()), time_to_throw)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_add(n)
            });
        }
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<DefaultOnly>, mut s: ()) {
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_dec());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_sub(s)
            });
        }
        unimplemented!();
    }
    fn max_size(&self) -> () {
        return (((2147483647_()).wrapping_mul((2_() as ())) as ()).wrapping_add((1_() as ())))
            .wrapping_div((4usize as ()));
    }
    fn destroy(&self, mut p: Ptr<DefaultOnly>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), destroy_count)
                .with_mut(|__v| __v.prefix_inc());
        }
        ({ DefaultOnlyImpl::destructor(&p) });
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
pub trait test_allocator_unsigned_int_Impl {
    fn destructor(&self) {
        unimplemented!()
    }
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
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_unsigned_int_Impl for Ptr<test_allocator_unsigned_int_> {}
pub trait test_allocator_void_Impl {
    fn destructor(&self);
    fn get_id(&self) -> ();
    fn get_data(&self) -> ();
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_>;
}
impl test_allocator_void_Impl for Ptr<test_allocator_void_> {
    fn destructor(&self) {
        field!((*self), data_).write(destructed_value_31.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_31.with(|rc| *rc.borrow()));
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
    let _ = count_1.with(|_| ());
    let _ = outstanding_bytes_4.with(|_| ());
    let _ = alloc_count_5.with(|_| ());
    let _ = dealloc_count_6.with(|_| ());
    let _ = disable_default_constructor_7.with(|_| ());
    let _ = destructed_value_31.with(|_| ());
    let _ = moved_value_32.with(|_| ());
}
