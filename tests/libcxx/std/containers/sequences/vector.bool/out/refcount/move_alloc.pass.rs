extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_1: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_2: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_3: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_bool__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_bool__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
pub fn operator_eq_4(
    mut x: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_5(
    mut x: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
) -> () {
    return !({
        let _x: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_4(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn new_2(mut _a0: ()) -> Self {
        Self {
            ptr_: Ptr::<()>::null(),
        }
    }
}
impl std::cmp::PartialEq for min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_4(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {}
pub fn operator_sub_6(
    mut x: min_pointer_const_unsigned_long__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_unsigned_long__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as () as ());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_unsigned_long__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_const_unsigned_long__std_integral_constant_unsigned_long__0__ {
    pub fn new_1(
        mut p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
    ) -> Self {
        Self {
            ptr_: (p.ptr_).clone(),
        }
    }
}
impl min_allocator_bool_ {
    pub fn new_1(mut _a0: min_allocator_unsigned_long_) -> Self {
        Self {}
    }
}
pub fn operator_eq_7(mut _a0: min_allocator_bool_, mut _a1: min_allocator_bool_) -> () {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_bool_ {}
impl std::cmp::PartialEq for min_allocator_bool_ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_7(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_bool_ {}
impl min_allocator_unsigned_long_ {
    pub fn new_1(mut _a0: min_allocator_bool_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_unsigned_long_ {}
impl min_allocator_bool_ {}
impl min_allocator_unsigned_long_ {}
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
    pub static destructed_value_8: Value<()> = Rc::new(RefCell::new(-1_()));
);
thread_local!(
    pub static moved_value_9: Value<()> = Rc::new(RefCell::new(2147483647_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_10(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_11(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_10(_x, _y)
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
            operator_eq_10(
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
impl test_allocator_bool_ {
    pub fn new_1(a: Ptr<test_allocator_unsigned_int_>) -> Self {
        let __this: Value<test_allocator_bool_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_bool_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), converted).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn operator_eq_12(x: Ptr<test_allocator_bool_>, y: Ptr<test_allocator_bool_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_bool__rebind_unsigned_int_ {}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_bool_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_bool_ {
    pub fn new_2(mut data: ()) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
}
impl std::cmp::PartialEq for test_allocator_bool_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_12(
                Rc::new(RefCell::new(test_allocator_bool_ {
                    data_: self.data_.clone(),
                    id_: self.id_.clone(),
                    stats_: self.stats_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(test_allocator_bool_ {
                    data_: other.data_.clone(),
                    id_: other.id_.clone(),
                    stats_: other.stats_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_bool_ {}
impl Default for test_allocator_bool_ {
    fn default() -> Self {
        {
            test_allocator_bool_ {
                data_: unimplemented!(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
impl test_allocator_unsigned_int_ {
    pub fn new_1(a: Ptr<test_allocator_bool_>) -> Self {
        let __this: Value<test_allocator_unsigned_int_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_unsigned_int_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), converted).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
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
impl other_allocator_bool_ {
    pub fn new_1(a: Ptr<other_allocator_unsigned_long_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
        }
    }
}
pub fn operator_eq_13(x: Ptr<other_allocator_bool_>, y: Ptr<other_allocator_bool_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct other_allocator_bool_ {
    #[offset(0)]
    data_: (),
}
impl other_allocator_bool_ {
    pub fn new_2(mut i: ()) -> Self {
        Self { data_: i }
    }
}
impl std::cmp::PartialEq for other_allocator_bool_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_13(
                Rc::new(RefCell::new(other_allocator_bool_ {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(other_allocator_bool_ {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for other_allocator_bool_ {}
impl Default for other_allocator_bool_ {
    fn default() -> Self {
        {
            other_allocator_bool_ {
                data_: unimplemented!(),
            }
        }
    }
}
impl other_allocator_unsigned_long_ {
    pub fn new_1(a: Ptr<other_allocator_bool_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct other_allocator_unsigned_long_ {
    #[offset(0)]
    data_: (),
}
impl other_allocator_unsigned_long_ {}
impl Default for other_allocator_unsigned_long_ {
    fn default() -> Self {
        {
            other_allocator_unsigned_long_ {
                data_: unimplemented!(),
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
pub fn test_14(a: Ptr<()>, a0: Ptr<()>) {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while (i < 5_()) {
        unimplemented!() = true;
        unimplemented!() = true;
        i = { (i).wrapping_add(2_()) };
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_15(a: Ptr<()>, a0: Ptr<()>) {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while (i < 17_()) {
        unimplemented!() = true;
        unimplemented!() = true;
        i = { (i).wrapping_add(2_()) };
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_16(a: Ptr<()>, a0: Ptr<()>) {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while (i < 65_()) {
        unimplemented!() = true;
        unimplemented!() = true;
        i = { (i).wrapping_add(2_()) };
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_17(a: Ptr<()>, a0: Ptr<()>) {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while (i < 299_()) {
        unimplemented!() = true;
        unimplemented!() = true;
        i = { (i).wrapping_add(2_()) };
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_18(a: Ptr<test_allocator_bool_>, a0: Ptr<test_allocator_bool_>) {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while (i < 5_()) {
        unimplemented!() = true;
        unimplemented!() = true;
        i = { (i).wrapping_add(2_()) };
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    if ({
        let _x: Ptr<test_allocator_bool_> = (a).clone();
        let _y: Ptr<test_allocator_bool_> = (a0).clone();
        operator_eq_12(_x, _y)
    }) {
        unimplemented!();
    } else {
        unimplemented!();
    }
}
pub fn test_19(a: Ptr<test_allocator_bool_>, a0: Ptr<test_allocator_bool_>) {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while (i < 17_()) {
        unimplemented!() = true;
        unimplemented!() = true;
        i = { (i).wrapping_add(2_()) };
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    if ({
        let _x: Ptr<test_allocator_bool_> = (a).clone();
        let _y: Ptr<test_allocator_bool_> = (a0).clone();
        operator_eq_12(_x, _y)
    }) {
        unimplemented!();
    } else {
        unimplemented!();
    }
}
pub fn test_20(a: Ptr<test_allocator_bool_>, a0: Ptr<test_allocator_bool_>) {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while (i < 65_()) {
        unimplemented!() = true;
        unimplemented!() = true;
        i = { (i).wrapping_add(2_()) };
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    if ({
        let _x: Ptr<test_allocator_bool_> = (a).clone();
        let _y: Ptr<test_allocator_bool_> = (a0).clone();
        operator_eq_12(_x, _y)
    }) {
        unimplemented!();
    } else {
        unimplemented!();
    }
}
pub fn test_21(a: Ptr<test_allocator_bool_>, a0: Ptr<test_allocator_bool_>) {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while (i < 299_()) {
        unimplemented!() = true;
        unimplemented!() = true;
        i = { (i).wrapping_add(2_()) };
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    if ({
        let _x: Ptr<test_allocator_bool_> = (a).clone();
        let _y: Ptr<test_allocator_bool_> = (a0).clone();
        operator_eq_12(_x, _y)
    }) {
        unimplemented!();
    } else {
        unimplemented!();
    }
}
pub fn test_22(a: Ptr<other_allocator_bool_>, a0: Ptr<other_allocator_bool_>) {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while (i < 5_()) {
        unimplemented!() = true;
        unimplemented!() = true;
        i = { (i).wrapping_add(2_()) };
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    if ({
        let _x: Ptr<other_allocator_bool_> = (a).clone();
        let _y: Ptr<other_allocator_bool_> = (a0).clone();
        operator_eq_13(_x, _y)
    }) {
        unimplemented!();
    } else {
        unimplemented!();
    }
}
pub fn test_23(a: Ptr<other_allocator_bool_>, a0: Ptr<other_allocator_bool_>) {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while (i < 17_()) {
        unimplemented!() = true;
        unimplemented!() = true;
        i = { (i).wrapping_add(2_()) };
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    if ({
        let _x: Ptr<other_allocator_bool_> = (a).clone();
        let _y: Ptr<other_allocator_bool_> = (a0).clone();
        operator_eq_13(_x, _y)
    }) {
        unimplemented!();
    } else {
        unimplemented!();
    }
}
pub fn test_24(a: Ptr<other_allocator_bool_>, a0: Ptr<other_allocator_bool_>) {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while (i < 65_()) {
        unimplemented!() = true;
        unimplemented!() = true;
        i = { (i).wrapping_add(2_()) };
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    if ({
        let _x: Ptr<other_allocator_bool_> = (a).clone();
        let _y: Ptr<other_allocator_bool_> = (a0).clone();
        operator_eq_13(_x, _y)
    }) {
        unimplemented!();
    } else {
        unimplemented!();
    }
}
pub fn test_25(a: Ptr<other_allocator_bool_>, a0: Ptr<other_allocator_bool_>) {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while (i < 299_()) {
        unimplemented!() = true;
        unimplemented!() = true;
        i = { (i).wrapping_add(2_()) };
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    if ({
        let _x: Ptr<other_allocator_bool_> = (a).clone();
        let _y: Ptr<other_allocator_bool_> = (a0).clone();
        operator_eq_13(_x, _y)
    }) {
        unimplemented!();
    } else {
        unimplemented!();
    }
}
pub fn test_26(a: Ptr<min_allocator_bool_>, a0: Ptr<min_allocator_bool_>) {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while (i < 5_()) {
        unimplemented!() = true;
        unimplemented!() = true;
        i = { (i).wrapping_add(2_()) };
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_27(a: Ptr<min_allocator_bool_>, a0: Ptr<min_allocator_bool_>) {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while (i < 17_()) {
        unimplemented!() = true;
        unimplemented!() = true;
        i = { (i).wrapping_add(2_()) };
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_28(a: Ptr<min_allocator_bool_>, a0: Ptr<min_allocator_bool_>) {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while (i < 65_()) {
        unimplemented!() = true;
        unimplemented!() = true;
        i = { (i).wrapping_add(2_()) };
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_29(a: Ptr<min_allocator_bool_>, a0: Ptr<min_allocator_bool_>) {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut original: () = unimplemented!();
    let mut i: () = 1_();
    'loop_: while (i < 299_()) {
        unimplemented!() = true;
        unimplemented!() = true;
        i = { (i).wrapping_add(2_()) };
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn tests_30() -> () {
    {
        ({
            let _a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _a0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_14(_a.as_pointer(), _a0.as_pointer())
        });
        ({
            let _a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _a0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_15(_a.as_pointer(), _a0.as_pointer())
        });
        ({
            let _a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _a0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_16(_a.as_pointer(), _a0.as_pointer())
        });
        ({
            let _a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _a0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_17(_a.as_pointer(), _a0.as_pointer())
        });
    }
    {
        ({
            let _a: Value<test_allocator_bool_> =
                Rc::new(RefCell::new(test_allocator_bool_::new_2({ 5_() })));
            let _a0: Value<test_allocator_bool_> =
                Rc::new(RefCell::new(test_allocator_bool_::new_2({ 5_() })));
            test_18(_a.as_pointer(), _a0.as_pointer())
        });
        ({
            let _a: Value<test_allocator_bool_> =
                Rc::new(RefCell::new(test_allocator_bool_::new_2({ 5_() })));
            let _a0: Value<test_allocator_bool_> =
                Rc::new(RefCell::new(test_allocator_bool_::new_2({ 5_() })));
            test_19(_a.as_pointer(), _a0.as_pointer())
        });
        ({
            let _a: Value<test_allocator_bool_> =
                Rc::new(RefCell::new(test_allocator_bool_::new_2({ 5_() })));
            let _a0: Value<test_allocator_bool_> =
                Rc::new(RefCell::new(test_allocator_bool_::new_2({ 5_() })));
            test_20(_a.as_pointer(), _a0.as_pointer())
        });
        ({
            let _a: Value<test_allocator_bool_> =
                Rc::new(RefCell::new(test_allocator_bool_::new_2({ 5_() })));
            let _a0: Value<test_allocator_bool_> =
                Rc::new(RefCell::new(test_allocator_bool_::new_2({ 5_() })));
            test_21(_a.as_pointer(), _a0.as_pointer())
        });
        ({
            let _a: Value<test_allocator_bool_> =
                Rc::new(RefCell::new(test_allocator_bool_::new_2({ 5_() })));
            let _a0: Value<test_allocator_bool_> =
                Rc::new(RefCell::new(test_allocator_bool_::new_2({ 6_() })));
            test_18(_a.as_pointer(), _a0.as_pointer())
        });
        ({
            let _a: Value<test_allocator_bool_> =
                Rc::new(RefCell::new(test_allocator_bool_::new_2({ 5_() })));
            let _a0: Value<test_allocator_bool_> =
                Rc::new(RefCell::new(test_allocator_bool_::new_2({ 6_() })));
            test_19(_a.as_pointer(), _a0.as_pointer())
        });
        ({
            let _a: Value<test_allocator_bool_> =
                Rc::new(RefCell::new(test_allocator_bool_::new_2({ 5_() })));
            let _a0: Value<test_allocator_bool_> =
                Rc::new(RefCell::new(test_allocator_bool_::new_2({ 6_() })));
            test_20(_a.as_pointer(), _a0.as_pointer())
        });
        ({
            let _a: Value<test_allocator_bool_> =
                Rc::new(RefCell::new(test_allocator_bool_::new_2({ 5_() })));
            let _a0: Value<test_allocator_bool_> =
                Rc::new(RefCell::new(test_allocator_bool_::new_2({ 6_() })));
            test_21(_a.as_pointer(), _a0.as_pointer())
        });
    }
    {
        ({
            let _a: Value<other_allocator_bool_> =
                Rc::new(RefCell::new(other_allocator_bool_::new_2({ 5_() })));
            let _a0: Value<other_allocator_bool_> =
                Rc::new(RefCell::new(other_allocator_bool_::new_2({ 5_() })));
            test_22(_a.as_pointer(), _a0.as_pointer())
        });
        ({
            let _a: Value<other_allocator_bool_> =
                Rc::new(RefCell::new(other_allocator_bool_::new_2({ 5_() })));
            let _a0: Value<other_allocator_bool_> =
                Rc::new(RefCell::new(other_allocator_bool_::new_2({ 5_() })));
            test_23(_a.as_pointer(), _a0.as_pointer())
        });
        ({
            let _a: Value<other_allocator_bool_> =
                Rc::new(RefCell::new(other_allocator_bool_::new_2({ 5_() })));
            let _a0: Value<other_allocator_bool_> =
                Rc::new(RefCell::new(other_allocator_bool_::new_2({ 5_() })));
            test_24(_a.as_pointer(), _a0.as_pointer())
        });
        ({
            let _a: Value<other_allocator_bool_> =
                Rc::new(RefCell::new(other_allocator_bool_::new_2({ 5_() })));
            let _a0: Value<other_allocator_bool_> =
                Rc::new(RefCell::new(other_allocator_bool_::new_2({ 5_() })));
            test_25(_a.as_pointer(), _a0.as_pointer())
        });
        ({
            let _a: Value<other_allocator_bool_> =
                Rc::new(RefCell::new(other_allocator_bool_::new_2({ 5_() })));
            let _a0: Value<other_allocator_bool_> =
                Rc::new(RefCell::new(other_allocator_bool_::new_2({ 3_() })));
            test_22(_a.as_pointer(), _a0.as_pointer())
        });
        ({
            let _a: Value<other_allocator_bool_> =
                Rc::new(RefCell::new(other_allocator_bool_::new_2({ 5_() })));
            let _a0: Value<other_allocator_bool_> =
                Rc::new(RefCell::new(other_allocator_bool_::new_2({ 3_() })));
            test_23(_a.as_pointer(), _a0.as_pointer())
        });
        ({
            let _a: Value<other_allocator_bool_> =
                Rc::new(RefCell::new(other_allocator_bool_::new_2({ 5_() })));
            let _a0: Value<other_allocator_bool_> =
                Rc::new(RefCell::new(other_allocator_bool_::new_2({ 3_() })));
            test_24(_a.as_pointer(), _a0.as_pointer())
        });
        ({
            let _a: Value<other_allocator_bool_> =
                Rc::new(RefCell::new(other_allocator_bool_::new_2({ 5_() })));
            let _a0: Value<other_allocator_bool_> =
                Rc::new(RefCell::new(other_allocator_bool_::new_2({ 3_() })));
            test_25(_a.as_pointer(), _a0.as_pointer())
        });
    }
    {
        ({
            let _a: Value<min_allocator_bool_> =
                Rc::new(RefCell::new(<min_allocator_bool_>::default()));
            let _a0: Value<min_allocator_bool_> =
                Rc::new(RefCell::new(<min_allocator_bool_>::default()));
            test_26(_a.as_pointer(), _a0.as_pointer())
        });
        ({
            let _a: Value<min_allocator_bool_> =
                Rc::new(RefCell::new(<min_allocator_bool_>::default()));
            let _a0: Value<min_allocator_bool_> =
                Rc::new(RefCell::new(<min_allocator_bool_>::default()));
            test_27(_a.as_pointer(), _a0.as_pointer())
        });
        ({
            let _a: Value<min_allocator_bool_> =
                Rc::new(RefCell::new(<min_allocator_bool_>::default()));
            let _a0: Value<min_allocator_bool_> =
                Rc::new(RefCell::new(<min_allocator_bool_>::default()));
            test_28(_a.as_pointer(), _a0.as_pointer())
        });
        ({
            let _a: Value<min_allocator_bool_> =
                Rc::new(RefCell::new(<min_allocator_bool_>::default()));
            let _a0: Value<min_allocator_bool_> =
                Rc::new(RefCell::new(<min_allocator_bool_>::default()));
            test_29(_a.as_pointer(), _a0.as_pointer())
        });
    }
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
    ({ tests_30() });
    const _: () = assert!(true, "tests()");;
    return 0_();
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
pub trait min_allocator_bool_Impl {
    fn allocate(&self, mut n: ()) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_bool__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_bool_Impl for Ptr<min_allocator_bool_> {}
pub trait min_allocator_unsigned_long_Impl {
    fn allocate(
        &self,
        n: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__;
    fn deallocate(
        &self,
        p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        n: (),
    );
}
impl min_allocator_unsigned_long_Impl for Ptr<min_allocator_unsigned_long_> {
    fn allocate(
        &self,
        mut n: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        return min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub trait min_pointer_bool__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_bool__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_bool__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_bool__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_bool__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_bool__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_bool__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_const_unsigned_long__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_unsigned_long__std_integral_constant_unsigned_long__0__>;
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_unsigned_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: (),
    ) -> Ptr<min_pointer_const_unsigned_long__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_unsigned_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_const_unsigned_long__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_unsigned_long__std_integral_constant_unsigned_long__0__>
{
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_unsigned_long__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_unsigned_long__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
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
pub trait min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__>;
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
        n: (),
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        n: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__;
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
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__Impl :: operator_add_assign ( &tmp .as_pointer()  , n  , )
        });
        return (*tmp.borrow()).clone();
    }
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
pub trait other_allocator_bool_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!()
    }
    fn select_on_container_copy_construction(&self) -> other_allocator_bool_ {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
}
impl other_allocator_bool_Impl for Ptr<other_allocator_bool_> {}
pub trait other_allocator_unsigned_long_Impl {
    fn allocate(&self, n: ()) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, s: ());
    fn select_on_container_copy_construction(&self) -> other_allocator_unsigned_long_ {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
}
impl other_allocator_unsigned_long_Impl for Ptr<other_allocator_unsigned_long_> {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!();
    }
}
pub trait test_allocator_bool_Impl {
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
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_bool_Impl for Ptr<test_allocator_bool_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_8.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_8.with(|rc| *rc.borrow()));
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
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn allocate(&self, n: (), _a1: Option<()>) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, s: ());
    fn max_size(&self) -> ();
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
impl test_allocator_unsigned_int_Impl for Ptr<test_allocator_unsigned_int_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_8.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_8.with(|rc| *rc.borrow()));
    }
    fn allocate(&self, mut n: (), _a1: Option<()>) -> Ptr<()> {
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
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
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
            .wrapping_div((::std::mem::size_of::<()>() as ()));
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
        field!((*self), data_).write(destructed_value_8.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_8.with(|rc| *rc.borrow()));
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
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
    let _ = destructed_value_8.with(|_| ());
    let _ = moved_value_9.with(|_| ());
}
