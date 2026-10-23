extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_1(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_2(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_3(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_4(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_5(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_6(_a0: Ptr<MoveOnly>, _a1: Ptr<MoveOnly>) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    data_: (),
}
impl MoveOnly {
    pub fn new(data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| 1_());
        Self { data_: data }
    }
    pub fn move_from(x: Ptr<MoveOnly>) -> Self {
        let __this: MoveOnly = Self {
            data_: x.with(|__s| __s.data_),
        };
        field!(x, data_).write(0_());
        __this
    }
}
impl std::cmp::Ord for MoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_6(
                Rc::new(RefCell::new(MoveOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(MoveOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for MoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MoveOnly {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_0(
                Rc::new(RefCell::new(MoveOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(MoveOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for MoveOnly {}
impl Default for MoveOnly {
    fn default() -> Self {
        { MoveOnly::new(None) }
    }
}
pub fn operator_eq_7(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_8(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_9(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_10(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_11(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_12(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_13(_a0: Ptr<TrivialMoveOnly>, _a1: Ptr<TrivialMoveOnly>) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(4)]
pub struct TrivialMoveOnly {
    #[offset(0)]
    data_: (),
}
impl TrivialMoveOnly {
    pub fn new(data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| 1_());
        Self { data_: data }
    }
    pub fn move_from(_a0: Ptr<TrivialMoveOnly>) -> Self {
        Self {
            data_: { (*_a0.upgrade().deref()).data_ },
        }
    }
}
impl std::cmp::Ord for TrivialMoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_13(
                Rc::new(RefCell::new(TrivialMoveOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(TrivialMoveOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
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
        {
            operator_eq_7(
                Rc::new(RefCell::new(TrivialMoveOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(TrivialMoveOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for TrivialMoveOnly {}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        { TrivialMoveOnly::new(None) }
    }
}
pub fn operator_eq_14(a: Ptr<test_less_MoveOnly_>, b: Ptr<test_less_MoveOnly_>) -> () {
    return ({ a.with(|__s| __s.data_) } == { b.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct test_less_MoveOnly_ {
    #[offset(0)]
    pub data_: (),
}
impl test_less_MoveOnly_ {
    pub fn new_1(mut data: ()) -> Self {
        Self { data_: data }
    }
}
impl std::cmp::PartialEq for test_less_MoveOnly_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_14(
                Rc::new(RefCell::new(test_less_MoveOnly_ {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(test_less_MoveOnly_ {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for test_less_MoveOnly_ {}
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
    pub static destructed_value_15: Value<()> = Rc::new(RefCell::new(-1_()));
);
thread_local!(
    pub static moved_value_16: Value<()> = Rc::new(RefCell::new(2147483647_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_17(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_18(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_17(_x, _y)
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
            operator_eq_17(
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
pub fn operator_eq_19(
    x: Ptr<test_allocator_std_pair_const_MoveOnly__MoveOnly__>,
    y: Ptr<test_allocator_std_pair_const_MoveOnly__MoveOnly__>,
) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_std_pair_const_MoveOnly__MoveOnly___rebind_std_pair_const_MoveOnly__MoveOnly__
{}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    pub fn new_1(mut data: ()) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn copy_from(a: Ptr<test_allocator_std_pair_const_MoveOnly__MoveOnly__>) -> Self {
        let __this: Value<test_allocator_std_pair_const_MoveOnly__MoveOnly__> =
            Rc::new(RefCell::new(Self {
                data_: a.with(|__s| __s.data_),
                id_: a.with(|__s| __s.id_),
                stats_: a.with(|__s| __s.stats_.clone()),
            }));
        let this: Ptr<test_allocator_std_pair_const_MoveOnly__MoveOnly__> = __this.as_pointer();
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::PartialEq for test_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_19(
                Rc::new(RefCell::new(
                    test_allocator_std_pair_const_MoveOnly__MoveOnly__ {
                        data_: self.data_.clone(),
                        id_: self.id_.clone(),
                        stats_: self.stats_.clone(),
                    },
                ))
                .as_pointer(),
                Rc::new(RefCell::new(
                    test_allocator_std_pair_const_MoveOnly__MoveOnly__ {
                        data_: other.data_.clone(),
                        id_: other.id_.clone(),
                        stats_: other.stats_.clone(),
                    },
                ))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_std_pair_const_MoveOnly__MoveOnly__ {}
impl Clone for test_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_std_pair_const_MoveOnly__MoveOnly__> = Rc::new(
            RefCell::new(test_allocator_std_pair_const_MoveOnly__MoveOnly__ {
                data_: self.data_.clone(),
                id_: self.id_.clone(),
                stats_: self.stats_.clone(),
            }),
        );
        test_allocator_std_pair_const_MoveOnly__MoveOnly__::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    fn default() -> Self {
        {
            test_allocator_std_pair_const_MoveOnly__MoveOnly__ {
                data_: unimplemented!(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
pub fn operator_eq_20(
    x: Ptr<test_allocator_std_pair_const_int__Counter_int___>,
    y: Ptr<test_allocator_std_pair_const_int__Counter_int___>,
) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_std_pair_const_int__Counter_int____rebind_std_pair_const_int__Counter_int___
{}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_std_pair_const_int__Counter_int___ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_std_pair_const_int__Counter_int___ {
    pub fn new_1(mut data: ()) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn copy_from(a: Ptr<test_allocator_std_pair_const_int__Counter_int___>) -> Self {
        let __this: Value<test_allocator_std_pair_const_int__Counter_int___> =
            Rc::new(RefCell::new(Self {
                data_: a.with(|__s| __s.data_),
                id_: a.with(|__s| __s.id_),
                stats_: a.with(|__s| __s.stats_.clone()),
            }));
        let this: Ptr<test_allocator_std_pair_const_int__Counter_int___> = __this.as_pointer();
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::PartialEq for test_allocator_std_pair_const_int__Counter_int___ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_20(
                Rc::new(RefCell::new(
                    test_allocator_std_pair_const_int__Counter_int___ {
                        data_: self.data_.clone(),
                        id_: self.id_.clone(),
                        stats_: self.stats_.clone(),
                    },
                ))
                .as_pointer(),
                Rc::new(RefCell::new(
                    test_allocator_std_pair_const_int__Counter_int___ {
                        data_: other.data_.clone(),
                        id_: other.id_.clone(),
                        stats_: other.stats_.clone(),
                    },
                ))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_std_pair_const_int__Counter_int___ {}
impl Clone for test_allocator_std_pair_const_int__Counter_int___ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_std_pair_const_int__Counter_int___> = Rc::new(
            RefCell::new(test_allocator_std_pair_const_int__Counter_int___ {
                data_: self.data_.clone(),
                id_: self.id_.clone(),
                stats_: self.stats_.clone(),
            }),
        );
        test_allocator_std_pair_const_int__Counter_int___::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_std_pair_const_int__Counter_int___ {
    fn default() -> Self {
        {
            test_allocator_std_pair_const_int__Counter_int___ {
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
pub fn operator_eq_21(
    x: Ptr<other_allocator_std_pair_const_MoveOnly__MoveOnly__>,
    y: Ptr<other_allocator_std_pair_const_MoveOnly__MoveOnly__>,
) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct other_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    #[offset(0)]
    data_: (),
}
impl other_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    pub fn new_1(mut i: ()) -> Self {
        Self { data_: i }
    }
}
impl std::cmp::PartialEq for other_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_21(
                Rc::new(RefCell::new(
                    other_allocator_std_pair_const_MoveOnly__MoveOnly__ {
                        data_: self.data_.clone(),
                    },
                ))
                .as_pointer(),
                Rc::new(RefCell::new(
                    other_allocator_std_pair_const_MoveOnly__MoveOnly__ {
                        data_: other.data_.clone(),
                    },
                ))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for other_allocator_std_pair_const_MoveOnly__MoveOnly__ {}
impl Default for other_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    fn default() -> Self {
        {
            other_allocator_std_pair_const_MoveOnly__MoveOnly__ {
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_23.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_24.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_25.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_22.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_23.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_24.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_22: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_23: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_24: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_25: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_std_pair_const_MoveOnly__MoveOnly___std_integral_constant_unsigned_long__0__
{
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_std_pair_const_MoveOnly__MoveOnly___std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub fn operator_eq_26(
    mut _a0: min_allocator_std_pair_const_MoveOnly__MoveOnly__,
    mut _a1: min_allocator_std_pair_const_MoveOnly__MoveOnly__,
) -> () {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_pair_const_MoveOnly__MoveOnly__ {}
impl std::cmp::PartialEq for min_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_26(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_std_pair_const_MoveOnly__MoveOnly__ {}
pub fn operator_eq_27(
    mut _a0: explicit_allocator_std_pair_const_MoveOnly__MoveOnly__,
    mut _a1: explicit_allocator_std_pair_const_MoveOnly__MoveOnly__,
) -> () {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct explicit_allocator_std_pair_const_MoveOnly__MoveOnly__ {}
impl explicit_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    pub fn new() -> Self {
        Self {}
    }
}
impl std::cmp::PartialEq for explicit_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_27(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for explicit_allocator_std_pair_const_MoveOnly__MoveOnly__ {}
impl Default for explicit_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    fn default() -> Self {
        { explicit_allocator_std_pair_const_MoveOnly__MoveOnly__::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Counter_base {}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct Counter_int_ {
    #[offset(0)]
    data_: (),
}
impl Counter_int_ {
    pub fn new_1(data: Ptr<()>) -> Self {
        let __this: Counter_int_ = Self {
            data_: (data.read()),
        };
        (*gConstructed_28.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(rhs: Ptr<Counter_int_>) -> Self {
        let __this: Counter_int_ = Self {
            data_: rhs.with(|__s| __s.data_),
        };
        (*gConstructed_28.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn move_from(rhs: Ptr<Counter_int_>) -> Self {
        let __this: Counter_int_ = Self {
            data_: (rhs.with(|__s| __s.data_) as ()),
        };
        (*gConstructed_28.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl std::cmp::PartialEq for Counter_int_ {
    fn eq(&self, other: &Self) -> bool {
        {
            Counter_int_Impl::operator_eq(
                &Rc::new(RefCell::new(Counter_int_ {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Counter_int_ {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Counter_int_ {}
impl Clone for Counter_int_ {
    fn clone(&self) -> Self {
        let __src: Value<Counter_int_> = Rc::new(RefCell::new(Counter_int_ {
            data_: self.data_.clone(),
        }));
        Counter_int_::copy_from(__src.as_pointer())
    }
}
thread_local!(
    pub static gConstructed_28: Value<()> = Rc::new(RefCell::new(0_()));
);
pub fn test_29() -> () {
    {
        let a1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            {
                let __tmp_0: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_1: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_0.as_pointer() }, {
                    __tmp_1.as_pointer()
                })
            },
            {
                let __tmp_2: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_3: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_2.as_pointer() }, {
                    __tmp_3.as_pointer()
                })
            },
            {
                let __tmp_4: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_5: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_4.as_pointer() }, {
                    __tmp_5.as_pointer()
                })
            },
            {
                let __tmp_6: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_7: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_6.as_pointer() }, {
                    __tmp_7.as_pointer()
                })
            },
            {
                let __tmp_8: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_9: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_8.as_pointer() }, {
                    __tmp_9.as_pointer()
                })
            },
            {
                let __tmp_10: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_11: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_10.as_pointer() }, {
                    __tmp_11.as_pointer()
                })
            },
            {
                let __tmp_12: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_13: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_12.as_pointer() }, {
                    __tmp_13.as_pointer()
                })
            },
            {
                let __tmp_14: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_15: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_14.as_pointer() }, {
                    __tmp_15.as_pointer()
                })
            },
            {
                let __tmp_16: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_17: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_16.as_pointer() }, {
                    __tmp_17.as_pointer()
                })
            },
        ])));
        let m1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let a2: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            {
                let __tmp_18: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_19: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_18.as_pointer() }, {
                    __tmp_19.as_pointer()
                })
            },
            {
                let __tmp_20: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_21: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_20.as_pointer() }, {
                    __tmp_21.as_pointer()
                })
            },
            {
                let __tmp_22: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_23: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_22.as_pointer() }, {
                    __tmp_23.as_pointer()
                })
            },
            {
                let __tmp_24: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_25: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_24.as_pointer() }, {
                    __tmp_25.as_pointer()
                })
            },
            {
                let __tmp_26: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_27: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_26.as_pointer() }, {
                    __tmp_27.as_pointer()
                })
            },
            {
                let __tmp_28: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_29: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_28.as_pointer() }, {
                    __tmp_29.as_pointer()
                })
            },
            {
                let __tmp_30: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_31: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_30.as_pointer() }, {
                    __tmp_31.as_pointer()
                })
            },
            {
                let __tmp_32: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_33: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_32.as_pointer() }, {
                    __tmp_33.as_pointer()
                })
            },
            {
                let __tmp_34: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_35: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_34.as_pointer() }, {
                    __tmp_35.as_pointer()
                })
            },
        ])));
        let m2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let m3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let a1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            {
                let __tmp_36: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_37: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_36.as_pointer() }, {
                    __tmp_37.as_pointer()
                })
            },
            {
                let __tmp_38: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_39: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_38.as_pointer() }, {
                    __tmp_39.as_pointer()
                })
            },
            {
                let __tmp_40: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_41: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_40.as_pointer() }, {
                    __tmp_41.as_pointer()
                })
            },
            {
                let __tmp_42: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_43: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_42.as_pointer() }, {
                    __tmp_43.as_pointer()
                })
            },
            {
                let __tmp_44: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_45: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_44.as_pointer() }, {
                    __tmp_45.as_pointer()
                })
            },
            {
                let __tmp_46: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_47: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_46.as_pointer() }, {
                    __tmp_47.as_pointer()
                })
            },
            {
                let __tmp_48: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_49: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_48.as_pointer() }, {
                    __tmp_49.as_pointer()
                })
            },
            {
                let __tmp_50: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_51: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_50.as_pointer() }, {
                    __tmp_51.as_pointer()
                })
            },
            {
                let __tmp_52: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_53: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_52.as_pointer() }, {
                    __tmp_53.as_pointer()
                })
            },
        ])));
        let m1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let a2: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            {
                let __tmp_54: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_55: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_54.as_pointer() }, {
                    __tmp_55.as_pointer()
                })
            },
            {
                let __tmp_56: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_57: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_56.as_pointer() }, {
                    __tmp_57.as_pointer()
                })
            },
            {
                let __tmp_58: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_59: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_58.as_pointer() }, {
                    __tmp_59.as_pointer()
                })
            },
            {
                let __tmp_60: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_61: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_60.as_pointer() }, {
                    __tmp_61.as_pointer()
                })
            },
            {
                let __tmp_62: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_63: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_62.as_pointer() }, {
                    __tmp_63.as_pointer()
                })
            },
            {
                let __tmp_64: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_65: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_64.as_pointer() }, {
                    __tmp_65.as_pointer()
                })
            },
            {
                let __tmp_66: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_67: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_66.as_pointer() }, {
                    __tmp_67.as_pointer()
                })
            },
            {
                let __tmp_68: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_69: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_68.as_pointer() }, {
                    __tmp_69.as_pointer()
                })
            },
            {
                let __tmp_70: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_71: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_70.as_pointer() }, {
                    __tmp_71.as_pointer()
                })
            },
        ])));
        let m2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let m3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let a1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            {
                let __tmp_72: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_73: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_72.as_pointer() }, {
                    __tmp_73.as_pointer()
                })
            },
            {
                let __tmp_74: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_75: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_74.as_pointer() }, {
                    __tmp_75.as_pointer()
                })
            },
            {
                let __tmp_76: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_77: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_76.as_pointer() }, {
                    __tmp_77.as_pointer()
                })
            },
            {
                let __tmp_78: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_79: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_78.as_pointer() }, {
                    __tmp_79.as_pointer()
                })
            },
            {
                let __tmp_80: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_81: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_80.as_pointer() }, {
                    __tmp_81.as_pointer()
                })
            },
            {
                let __tmp_82: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_83: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_82.as_pointer() }, {
                    __tmp_83.as_pointer()
                })
            },
            {
                let __tmp_84: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_85: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_84.as_pointer() }, {
                    __tmp_85.as_pointer()
                })
            },
            {
                let __tmp_86: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_87: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_86.as_pointer() }, {
                    __tmp_87.as_pointer()
                })
            },
            {
                let __tmp_88: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_89: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_88.as_pointer() }, {
                    __tmp_89.as_pointer()
                })
            },
        ])));
        let m1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let a2: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            {
                let __tmp_90: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_91: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_90.as_pointer() }, {
                    __tmp_91.as_pointer()
                })
            },
            {
                let __tmp_92: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_93: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_92.as_pointer() }, {
                    __tmp_93.as_pointer()
                })
            },
            {
                let __tmp_94: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_95: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_94.as_pointer() }, {
                    __tmp_95.as_pointer()
                })
            },
            {
                let __tmp_96: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_97: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_96.as_pointer() }, {
                    __tmp_97.as_pointer()
                })
            },
            {
                let __tmp_98: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_99: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_98.as_pointer() }, {
                    __tmp_99.as_pointer()
                })
            },
            {
                let __tmp_100: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_101: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_100.as_pointer() }, {
                    __tmp_101.as_pointer()
                })
            },
            {
                let __tmp_102: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_103: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_102.as_pointer() }, {
                    __tmp_103.as_pointer()
                })
            },
            {
                let __tmp_104: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_105: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_104.as_pointer() }, {
                    __tmp_105.as_pointer()
                })
            },
            {
                let __tmp_106: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_107: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_106.as_pointer() }, {
                    __tmp_107.as_pointer()
                })
            },
        ])));
        let m2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let m3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        gConstructed_28.with(|rc| *rc.borrow_mut() = 0_());
        {
            let a1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
                {
                    let __tmp_108: Value<()> = Rc::new(RefCell::new(1_()));
                    let __tmp_109: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_int__Counter_int__::new({ __tmp_108.as_pointer() }, {
                        __tmp_109.as_pointer()
                    })
                },
                {
                    let __tmp_110: Value<()> = Rc::new(RefCell::new(1_()));
                    let __tmp_111: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_int__Counter_int__::new({ __tmp_110.as_pointer() }, {
                        __tmp_111.as_pointer()
                    })
                },
                {
                    let __tmp_112: Value<()> = Rc::new(RefCell::new(1_()));
                    let __tmp_113: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_int__Counter_int__::new({ __tmp_112.as_pointer() }, {
                        __tmp_113.as_pointer()
                    })
                },
                {
                    let __tmp_114: Value<()> = Rc::new(RefCell::new(2_()));
                    let __tmp_115: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_int__Counter_int__::new({ __tmp_114.as_pointer() }, {
                        __tmp_115.as_pointer()
                    })
                },
                {
                    let __tmp_116: Value<()> = Rc::new(RefCell::new(2_()));
                    let __tmp_117: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_int__Counter_int__::new({ __tmp_116.as_pointer() }, {
                        __tmp_117.as_pointer()
                    })
                },
                {
                    let __tmp_118: Value<()> = Rc::new(RefCell::new(2_()));
                    let __tmp_119: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_int__Counter_int__::new({ __tmp_118.as_pointer() }, {
                        __tmp_119.as_pointer()
                    })
                },
                {
                    let __tmp_120: Value<()> = Rc::new(RefCell::new(3_()));
                    let __tmp_121: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_int__Counter_int__::new({ __tmp_120.as_pointer() }, {
                        __tmp_121.as_pointer()
                    })
                },
                {
                    let __tmp_122: Value<()> = Rc::new(RefCell::new(3_()));
                    let __tmp_123: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_int__Counter_int__::new({ __tmp_122.as_pointer() }, {
                        __tmp_123.as_pointer()
                    })
                },
                {
                    let __tmp_124: Value<()> = Rc::new(RefCell::new(3_()));
                    let __tmp_125: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_int__Counter_int__::new({ __tmp_124.as_pointer() }, {
                        __tmp_125.as_pointer()
                    })
                },
            ])));
            let mut num: () = (72usize as ()).wrapping_div((8usize as ()));
            unimplemented!();
            let m1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            let m2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            let m3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            {
                let m4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
    }
    {
        let a1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            {
                let __tmp_126: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_127: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_126.as_pointer() }, {
                    __tmp_127.as_pointer()
                })
            },
            {
                let __tmp_128: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_129: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_128.as_pointer() }, {
                    __tmp_129.as_pointer()
                })
            },
            {
                let __tmp_130: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_131: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_130.as_pointer() }, {
                    __tmp_131.as_pointer()
                })
            },
            {
                let __tmp_132: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_133: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_132.as_pointer() }, {
                    __tmp_133.as_pointer()
                })
            },
            {
                let __tmp_134: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_135: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_134.as_pointer() }, {
                    __tmp_135.as_pointer()
                })
            },
            {
                let __tmp_136: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_137: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_136.as_pointer() }, {
                    __tmp_137.as_pointer()
                })
            },
            {
                let __tmp_138: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_139: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_138.as_pointer() }, {
                    __tmp_139.as_pointer()
                })
            },
            {
                let __tmp_140: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_141: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_140.as_pointer() }, {
                    __tmp_141.as_pointer()
                })
            },
            {
                let __tmp_142: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_143: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_142.as_pointer() }, {
                    __tmp_143.as_pointer()
                })
            },
        ])));
        let m1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let a2: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            {
                let __tmp_144: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_145: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_144.as_pointer() }, {
                    __tmp_145.as_pointer()
                })
            },
            {
                let __tmp_146: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_147: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_146.as_pointer() }, {
                    __tmp_147.as_pointer()
                })
            },
            {
                let __tmp_148: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_149: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_148.as_pointer() }, {
                    __tmp_149.as_pointer()
                })
            },
            {
                let __tmp_150: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_151: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_150.as_pointer() }, {
                    __tmp_151.as_pointer()
                })
            },
            {
                let __tmp_152: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_153: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_152.as_pointer() }, {
                    __tmp_153.as_pointer()
                })
            },
            {
                let __tmp_154: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_155: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_154.as_pointer() }, {
                    __tmp_155.as_pointer()
                })
            },
            {
                let __tmp_156: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_157: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_156.as_pointer() }, {
                    __tmp_157.as_pointer()
                })
            },
            {
                let __tmp_158: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_159: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_158.as_pointer() }, {
                    __tmp_159.as_pointer()
                })
            },
            {
                let __tmp_160: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_161: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_160.as_pointer() }, {
                    __tmp_161.as_pointer()
                })
            },
        ])));
        let m2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let m3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let a1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            {
                let __tmp_162: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_163: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_162.as_pointer() }, {
                    __tmp_163.as_pointer()
                })
            },
            {
                let __tmp_164: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_165: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_164.as_pointer() }, {
                    __tmp_165.as_pointer()
                })
            },
            {
                let __tmp_166: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_167: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_166.as_pointer() }, {
                    __tmp_167.as_pointer()
                })
            },
            {
                let __tmp_168: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_169: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_168.as_pointer() }, {
                    __tmp_169.as_pointer()
                })
            },
            {
                let __tmp_170: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_171: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_170.as_pointer() }, {
                    __tmp_171.as_pointer()
                })
            },
            {
                let __tmp_172: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_173: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_172.as_pointer() }, {
                    __tmp_173.as_pointer()
                })
            },
            {
                let __tmp_174: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_175: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_174.as_pointer() }, {
                    __tmp_175.as_pointer()
                })
            },
            {
                let __tmp_176: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_177: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_176.as_pointer() }, {
                    __tmp_177.as_pointer()
                })
            },
            {
                let __tmp_178: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_179: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_178.as_pointer() }, {
                    __tmp_179.as_pointer()
                })
            },
        ])));
        let m1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let a2: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            {
                let __tmp_180: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_181: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_180.as_pointer() }, {
                    __tmp_181.as_pointer()
                })
            },
            {
                let __tmp_182: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_183: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_182.as_pointer() }, {
                    __tmp_183.as_pointer()
                })
            },
            {
                let __tmp_184: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_185: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_184.as_pointer() }, {
                    __tmp_185.as_pointer()
                })
            },
            {
                let __tmp_186: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_187: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_186.as_pointer() }, {
                    __tmp_187.as_pointer()
                })
            },
            {
                let __tmp_188: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_189: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_188.as_pointer() }, {
                    __tmp_189.as_pointer()
                })
            },
            {
                let __tmp_190: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_191: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_190.as_pointer() }, {
                    __tmp_191.as_pointer()
                })
            },
            {
                let __tmp_192: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_193: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_192.as_pointer() }, {
                    __tmp_193.as_pointer()
                })
            },
            {
                let __tmp_194: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_195: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_194.as_pointer() }, {
                    __tmp_195.as_pointer()
                })
            },
            {
                let __tmp_196: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_197: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_MoveOnly__MoveOnly_::new({ __tmp_196.as_pointer() }, {
                    __tmp_197.as_pointer()
                })
            },
        ])));
        let m2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let m3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
    ({ test_29() });
    return 0_();
}
pub trait Counter_int_Impl {
    fn copy_assign(&self, rhs: Ptr<Counter_int_>) -> Ptr<Counter_int_> {
        unimplemented!()
    }
    fn move_assign(&self, rhs: Ptr<Counter_int_>) -> Ptr<Counter_int_> {
        unimplemented!()
    }
    fn destructor(&self);
    fn get(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_eq(&self, x: Ptr<Counter_int_>) -> ();
    fn operator_lt(&self, x: Ptr<Counter_int_>) -> () {
        unimplemented!()
    }
}
impl Counter_int_Impl for Ptr<Counter_int_> {
    fn destructor(&self) {
        (*gConstructed_28.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn operator_eq(&self, x: Ptr<Counter_int_>) -> () {
        return ({ (*self).with(|__s| __s.data_) } == { x.with(|__s| __s.data_) });
    }
}
pub trait MoveOnlyImpl {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly>;
    fn get(&self) -> ();
    fn operator_add(&self, x: Ptr<MoveOnly>) -> MoveOnly;
    fn operator_mul(&self, x: Ptr<MoveOnly>) -> MoveOnly;
}
impl MoveOnlyImpl for Ptr<MoveOnly> {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        field!((*self), data_).write({ x.with(|__s| __s.data_) });
        field!(x, data_).write(0_());
        return (*self).clone();
    }
    fn get(&self) -> () {
        return (*self).with(|__s| __s.data_);
    }
    fn operator_add(&self, x: Ptr<MoveOnly>) -> MoveOnly {
        return MoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } + { x.with(|__s| __s.data_) }))
        });
    }
    fn operator_mul(&self, x: Ptr<MoveOnly>) -> MoveOnly {
        return MoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } * { x.with(|__s| __s.data_) }))
        });
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
pub trait TrivialMoveOnlyImpl {
    fn move_assign(&self, _a0: Ptr<TrivialMoveOnly>) -> Ptr<TrivialMoveOnly>;
    fn get(&self) -> ();
    fn operator_add(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
    fn operator_mul(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
}
impl TrivialMoveOnlyImpl for Ptr<TrivialMoveOnly> {
    fn move_assign(&self, _a0: Ptr<TrivialMoveOnly>) -> Ptr<TrivialMoveOnly> {
        field!((*self), data_).write({ { (*_a0.upgrade().deref()).data_ } });
        return (*self).clone();
    }
    fn get(&self) -> () {
        return (*self).with(|__s| __s.data_);
    }
    fn operator_add(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } + { x.with(|__s| __s.data_) }))
        });
    }
    fn operator_mul(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } * { x.with(|__s| __s.data_) }))
        });
    }
}
pub trait explicit_allocator_std_pair_const_MoveOnly__MoveOnly__Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!()
    }
}
impl explicit_allocator_std_pair_const_MoveOnly__MoveOnly__Impl
    for Ptr<explicit_allocator_std_pair_const_MoveOnly__MoveOnly__>
{
}
pub trait min_allocator_std_pair_const_MoveOnly__MoveOnly__Impl {
    fn allocate(
        &self,
        mut n: (),
    ) -> min_pointer_std_pair_const_MoveOnly__MoveOnly___std_integral_constant_unsigned_long__0__
    {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p : min_pointer_std_pair_const_MoveOnly__MoveOnly___std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_std_pair_const_MoveOnly__MoveOnly__Impl
    for Ptr<min_allocator_std_pair_const_MoveOnly__MoveOnly__>
{
}
pub trait min_pointer_std_pair_const_MoveOnly__MoveOnly___std_integral_constant_unsigned_long__0__Impl
{
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
    ) -> Ptr<min_pointer_std_pair_const_MoveOnly__MoveOnly___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_std_pair_const_MoveOnly__MoveOnly___std_integral_constant_unsigned_long__0__
    {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_std_pair_const_MoveOnly__MoveOnly___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_std_pair_const_MoveOnly__MoveOnly___std_integral_constant_unsigned_long__0__
    {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_std_pair_const_MoveOnly__MoveOnly___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_std_pair_const_MoveOnly__MoveOnly___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_std_pair_const_MoveOnly__MoveOnly___std_integral_constant_unsigned_long__0__
    {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_std_pair_const_MoveOnly__MoveOnly___std_integral_constant_unsigned_long__0__
    {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_std_pair_const_MoveOnly__MoveOnly___std_integral_constant_unsigned_long__0__Impl
    for Ptr<
        min_pointer_std_pair_const_MoveOnly__MoveOnly___std_integral_constant_unsigned_long__0__,
    >
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
pub trait other_allocator_std_pair_const_MoveOnly__MoveOnly__Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!()
    }
    fn select_on_container_copy_construction(
        &self,
    ) -> other_allocator_std_pair_const_MoveOnly__MoveOnly__ {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
}
impl other_allocator_std_pair_const_MoveOnly__MoveOnly__Impl
    for Ptr<other_allocator_std_pair_const_MoveOnly__MoveOnly__>
{
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
pub trait test_allocator_std_pair_const_MoveOnly__MoveOnly__Impl {
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
impl test_allocator_std_pair_const_MoveOnly__MoveOnly__Impl
    for Ptr<test_allocator_std_pair_const_MoveOnly__MoveOnly__>
{
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_15.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_15.with(|rc| *rc.borrow()));
    }
}
pub trait test_allocator_std_pair_const_int__Counter_int___Impl {
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
impl test_allocator_std_pair_const_int__Counter_int___Impl
    for Ptr<test_allocator_std_pair_const_int__Counter_int___>
{
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_15.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_15.with(|rc| *rc.borrow()));
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
        field!((*self), data_).write(destructed_value_15.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_15.with(|rc| *rc.borrow()));
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
pub trait test_less_MoveOnly_Impl {
    fn operator_call(&self, a: Ptr<MoveOnly>, b: Ptr<MoveOnly>) -> ();
}
impl test_less_MoveOnly_Impl for Ptr<test_less_MoveOnly_> {
    fn operator_call(&self, a: Ptr<MoveOnly>, b: Ptr<MoveOnly>) -> () {
        return ({
            let _x: Ptr<MoveOnly> = (a).clone();
            let _y: Ptr<MoveOnly> = (b).clone();
            operator_lt_2(_x, _y)
        });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = destructed_value_15.with(|_| ());
    let _ = moved_value_16.with(|_| ());
    let _ = outstanding_bytes_22.with(|_| ());
    let _ = alloc_count_23.with(|_| ());
    let _ = dealloc_count_24.with(|_| ());
    let _ = disable_default_constructor_25.with(|_| ());
    let _ = gConstructed_28.with(|_| ());
}
