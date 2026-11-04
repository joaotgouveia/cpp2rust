extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn DoNotOptimize_0(value: Ptr<AnyPtr>) -> Ptr<AnyPtr> {
    b"";
    return (value).clone();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(40)]
pub struct test_allocator_statistics {
    #[offset(0)]
    pub time_to_throw: i32,
    #[offset(4)]
    pub throw_after: i32,
    #[offset(8)]
    pub count: i32,
    #[offset(12)]
    pub alloc_count: i32,
    #[offset(16)]
    pub allocated_size: i32,
    #[offset(20)]
    pub construct_count: i32,
    #[offset(24)]
    pub destroy_count: i32,
    #[offset(28)]
    pub copied: i32,
    #[offset(32)]
    pub moved: i32,
    #[offset(36)]
    pub converted: i32,
}
impl CppMove for test_allocator_statistics {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        {
            test_allocator_statistics {
                time_to_throw: 0,
                throw_after: 2147483647,
                count: 0,
                alloc_count: 0,
                allocated_size: 0,
                construct_count: 0,
                destroy_count: 0,
                copied: 0,
                moved: 0,
                converted: 0,
            }
        }
    }
}
thread_local!(
    pub static destructed_value_1: Value<i32> = Rc::new(RefCell::new(-1_i32));
);
thread_local!(
    pub static moved_value_2: Value<i32> = Rc::new(RefCell::new(2147483647));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
impl CppMove for test_alloc_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_3(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_4(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_3(_x, _y)
    });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(16)]
pub struct test_allocator_void_ {
    #[offset(0)]
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_void_ {
    pub fn new_1(mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: 0,
            id_: 0,
            stats_: (stats).clone(),
        }
    }
    pub fn new_2(mut data: i32) -> Self {
        Self {
            data_: data,
            id_: 0,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_3(mut data: i32, mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: data,
            id_: 0,
            stats_: (stats).clone(),
        }
    }
    pub fn new_4(mut data: i32, mut id: i32) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_5(mut data: i32, mut id: i32, mut stats: Ptr<test_allocator_statistics>) -> Self {
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
impl CppEq for test_allocator_void_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_3(__lhs.clone(), __rhs.clone())
    }
}
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
impl CppMove for test_allocator_void_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for test_allocator_void_ {
    fn default() -> Self {
        {
            test_allocator_void_ {
                data_: 0,
                id_: 0,
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
pub fn operator_eq_5(x: Ptr<test_allocator_int_>, y: Ptr<test_allocator_int_>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_6(x: Ptr<test_allocator_int_>, y: Ptr<test_allocator_int_>) -> bool {
    return !({
        let _x: Ptr<test_allocator_int_> = (x).clone();
        let _y: Ptr<test_allocator_int_> = (y).clone();
        operator_eq_5(_x, _y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_int__rebind_int_ {}
impl CppMove for test_allocator_int__rebind_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_int__rebind_unsigned_long_ {}
impl CppMove for test_allocator_int__rebind_unsigned_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_int_ {
    #[offset(0)]
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_int_ {
    pub fn new_1(mut data: i32) -> Self {
        Self {
            data_: data,
            id_: 0,
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
        assert!(
            (({ a.with(|__s| __s.data_) } != { destructed_value_1.with(|rc| *rc.borrow()) })
                && ({ a.with(|__s| __s.id_) } != { destructed_value_1.with(|rc| *rc.borrow()) }))
                && (!(Ptr::<i8>::from_string_literal(b"copying from destroyed allocator"))
                    .is_null())
        );
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_inc();
            });
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| {
                __v.prefix_inc();
            });
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl CppEq for test_allocator_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_5(__lhs.clone(), __rhs.clone())
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
impl CppMove for test_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for test_allocator_int_ {
    fn default() -> Self {
        {
            test_allocator_int_ {
                data_: 0_i32,
                id_: 0,
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(16)]
pub struct test_allocator_unsigned_long_ {
    #[offset(0)]
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_unsigned_long_ {}
impl Default for test_allocator_unsigned_long_ {
    fn default() -> Self {
        {
            test_allocator_unsigned_long_ {
                data_: 0_i32,
                id_: 0_i32,
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
pub fn operator_eq_7(x: Ptr<other_allocator_int_>, y: Ptr<other_allocator_int_>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_8(x: Ptr<other_allocator_int_>, y: Ptr<other_allocator_int_>) -> bool {
    return !({
        let _x: Ptr<other_allocator_int_> = (x).clone();
        let _y: Ptr<other_allocator_int_> = (y).clone();
        operator_eq_7(_x, _y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct other_allocator_int_ {
    #[offset(0)]
    data_: i32,
}
impl other_allocator_int_ {
    pub fn new_1(mut i: i32) -> Self {
        Self { data_: i }
    }
}
impl CppEq for other_allocator_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_7(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for other_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for other_allocator_int_ {
    fn default() -> Self {
        { other_allocator_int_ { data_: 0_i32 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct other_allocator_unsigned_long_ {
    #[offset(0)]
    data_: i32,
}
impl other_allocator_unsigned_long_ {}
impl CppMove for other_allocator_unsigned_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for other_allocator_unsigned_long_ {
    fn default() -> Self {
        { other_allocator_unsigned_long_ { data_: 0_i32 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Ctor_Tag {}
impl CppMove for Ctor_Tag {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, FnPtrArg, CppFrom, CopyAssign, MoveAssign, Default)]
#[cpp_from(Ctor_Tag, new_1, value)]
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
    pub fn outstanding_alloc() -> usize {
        assert!(
            (alloc_count_10.with(|rc| *rc.borrow()) >= dealloc_count_11.with(|rc| *rc.borrow()))
        );
        return ((alloc_count_10.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_11.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_12.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_9.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_10.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_11.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_9: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_10: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_11: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_12: Value<bool> = Rc::new(RefCell::new(false));
);
pub fn operator_sub_13(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_eq_14(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_15(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !({
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_14(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(AnyPtr, new_1, value)]
#[byte_size(8)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<i32>,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn new_2(mut p: Ptr<i32>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn new_1(mut _a0: AnyPtr) -> Self {
        Self {
            ptr_: Ptr::<i32>::null(),
        }
    }
}
impl std::cmp::PartialEq for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_14(__lhs.read(), __rhs.read())
            })
        }
    }
}
impl std::cmp::Eq for min_pointer_int__std_integral_constant_unsigned_long__0__ {}
impl CppAdvance for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_8();
    }
}
impl CppIterator<i32> for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            min_pointer_int__std_integral_constant_unsigned_long__0__Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            min_pointer_int__std_integral_constant_unsigned_long__0__Impl::operator_inc_8(__it);
        });
        self
    }
}
impl CppInputIterator<i32> for min_pointer_int__std_integral_constant_unsigned_long__0__ {}
impl CppMove for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: AnyPtr,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
impl CppMove for min_pointer_void__std_integral_constant_unsigned_long__0__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_16(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_17(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } != { (y.ptr_).clone() });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(
    min_pointer_int__std_integral_constant_unsigned_long__0__,
    new_1,
    value
)]
#[byte_size(8)]
pub struct min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<i32>,
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
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_16(__lhs.read(), __rhs.read())
            })
        }
    }
}
impl std::cmp::Eq for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {}
impl CppAdvance for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_9();
    }
}
impl CppIterator<i32> for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            min_pointer_const_int__std_integral_constant_unsigned_long__0__Impl::operator_deref(
                __it,
            )
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            min_pointer_const_int__std_integral_constant_unsigned_long__0__Impl::operator_inc_9(
                __it,
            );
        });
        self
    }
}
impl CppInputIterator<i32> for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {}
impl CppMove for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: AnyPtr,
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
impl CppMove for min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<u64>,
}
impl min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {}
impl CppMove for min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_18(mut _a0: min_allocator_int_, mut _a1: min_allocator_int_) -> bool {
    return true;
}
pub fn operator_ne_19(mut x: min_allocator_int_, mut y: min_allocator_int_) -> bool {
    return !({
        let _arg0: min_allocator_int_ = x;
        operator_eq_18(_arg0, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
impl CppEq for min_allocator_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_18(__lhs.read(), __rhs.read())
    }
}
impl CppMove for min_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_unsigned_long_ {}
impl CppMove for min_allocator_unsigned_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_20(mut _a0: safe_allocator_int_, mut _a1: safe_allocator_int_) -> bool {
    return true;
}
pub fn operator_ne_21(mut x: safe_allocator_int_, mut y: safe_allocator_int_) -> bool {
    return !({
        let _arg0: safe_allocator_int_ = x;
        operator_eq_20(_arg0, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct safe_allocator_int_ {}
impl safe_allocator_int_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl CppEq for safe_allocator_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_20(__lhs.read(), __rhs.read())
    }
}
impl CppMove for safe_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for safe_allocator_int_ {
    fn default() -> Self {
        { safe_allocator_int_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct safe_allocator_unsigned_long_ {}
impl safe_allocator_unsigned_long_ {}
impl CppMove for safe_allocator_unsigned_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_22(
    lhs: Ptr<MaybePOCCAAllocator_int__false_>,
    rhs: Ptr<MaybePOCCAAllocator_int__false_>,
) -> bool {
    return ({ ({ MaybePOCCAAllocator_int__false_Impl::id(&lhs) }) } == {
        ({ MaybePOCCAAllocator_int__false_Impl::id(&rhs) })
    });
}
pub fn operator_ne_23(
    lhs: Ptr<MaybePOCCAAllocator_int__false_>,
    rhs: Ptr<MaybePOCCAAllocator_int__false_>,
) -> bool {
    return !({
        let _lhs: Ptr<MaybePOCCAAllocator_int__false_> = (lhs).clone();
        let _rhs: Ptr<MaybePOCCAAllocator_int__false_> = (rhs).clone();
        operator_eq_22(_lhs, _rhs)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct MaybePOCCAAllocator_int__false__rebind_int_ {}
impl CppMove for MaybePOCCAAllocator_int__false__rebind_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct MaybePOCCAAllocator_int__false__rebind_unsigned_long_ {}
impl CppMove for MaybePOCCAAllocator_int__false__rebind_unsigned_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct MaybePOCCAAllocator_int__false_ {
    #[offset(0)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    copy_assigned_into_: Ptr<bool>,
}
impl MaybePOCCAAllocator_int__false_ {
    pub fn new_1(mut id: i32, mut copy_assigned_into: Ptr<bool>) -> Self {
        Self {
            id_: id,
            copy_assigned_into_: (copy_assigned_into).clone(),
        }
    }
}
impl CppEq for MaybePOCCAAllocator_int__false_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_22(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for MaybePOCCAAllocator_int__false_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for MaybePOCCAAllocator_int__false_ {
    fn default() -> Self {
        {
            MaybePOCCAAllocator_int__false_ {
                id_: 0,
                copy_assigned_into_: Ptr::<bool>::null(),
            }
        }
    }
}
pub fn operator_eq_24(
    lhs: Ptr<MaybePOCCAAllocator_int__true_>,
    rhs: Ptr<MaybePOCCAAllocator_int__true_>,
) -> bool {
    return ({ ({ MaybePOCCAAllocator_int__true_Impl::id(&lhs) }) } == {
        ({ MaybePOCCAAllocator_int__true_Impl::id(&rhs) })
    });
}
pub fn operator_ne_25(
    lhs: Ptr<MaybePOCCAAllocator_int__true_>,
    rhs: Ptr<MaybePOCCAAllocator_int__true_>,
) -> bool {
    return !({
        let _lhs: Ptr<MaybePOCCAAllocator_int__true_> = (lhs).clone();
        let _rhs: Ptr<MaybePOCCAAllocator_int__true_> = (rhs).clone();
        operator_eq_24(_lhs, _rhs)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct MaybePOCCAAllocator_int__true__rebind_int_ {}
impl CppMove for MaybePOCCAAllocator_int__true__rebind_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct MaybePOCCAAllocator_int__true__rebind_unsigned_long_ {}
impl CppMove for MaybePOCCAAllocator_int__true__rebind_unsigned_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CopyAssign)]
#[byte_size(16)]
pub struct MaybePOCCAAllocator_int__true_ {
    #[offset(0)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    copy_assigned_into_: Ptr<bool>,
}
impl MaybePOCCAAllocator_int__true_ {
    pub fn new_1(mut id: i32, mut copy_assigned_into: Ptr<bool>) -> Self {
        Self {
            id_: id,
            copy_assigned_into_: (copy_assigned_into).clone(),
        }
    }
}
impl CppEq for MaybePOCCAAllocator_int__true_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_24(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for MaybePOCCAAllocator_int__true_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for MaybePOCCAAllocator_int__true_ {
    fn default() -> Self {
        {
            MaybePOCCAAllocator_int__true_ {
                id_: 0,
                copy_assigned_into_: Ptr::<bool>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct MaybePOCCAAllocator_unsigned_long__false_ {
    #[offset(0)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    copy_assigned_into_: Ptr<bool>,
}
impl MaybePOCCAAllocator_unsigned_long__false_ {}
impl CppMove for MaybePOCCAAllocator_unsigned_long__false_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for MaybePOCCAAllocator_unsigned_long__false_ {
    fn default() -> Self {
        {
            MaybePOCCAAllocator_unsigned_long__false_ {
                id_: 0,
                copy_assigned_into_: Ptr::<bool>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct MaybePOCCAAllocator_unsigned_long__true_ {
    #[offset(0)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    copy_assigned_into_: Ptr<bool>,
}
impl MaybePOCCAAllocator_unsigned_long__true_ {}
impl CppMove for MaybePOCCAAllocator_unsigned_long__true_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for MaybePOCCAAllocator_unsigned_long__true_ {
    fn default() -> Self {
        {
            MaybePOCCAAllocator_unsigned_long__true_ {
                id_: 0,
                copy_assigned_into_: Ptr::<bool>::null(),
            }
        }
    }
}
pub fn tests_26() -> bool {
    {
        let mut l: Vec<i32> = vec![2; 3_usize as usize];
        let l2: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        l2.as_pointer().write((l).clone());
        assert!(unimplemented!());
        assert!(
            ({
                let _x: Value<test_allocator_int_> = Rc::new(RefCell::new(unimplemented!()));
                let _y: Value<test_allocator_int_> =
                    Rc::new(RefCell::new(test_allocator_int_::new_1({ 3 })));
                operator_eq_5(_x.as_pointer(), _y.as_pointer())
            })
        );
    }
    {
        let mut l: Vec<i32> = vec![2; 3_usize as usize];
        let l2: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        l2.as_pointer().write((l).clone());
        assert!(unimplemented!());
        assert!(
            ({
                let _x: Value<other_allocator_int_> = Rc::new(RefCell::new(unimplemented!()));
                let _y: Value<other_allocator_int_> =
                    Rc::new(RefCell::new(other_allocator_int_::new_1({ 5 })));
                operator_eq_7(_x.as_pointer(), _y.as_pointer())
            })
        );
    }
    {
        let copy_assigned_into: Value<bool> = Rc::new(RefCell::new(false));
        let mut l: Vec<i32> = vec![2; 3_usize as usize];
        let l2: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ !(*copy_assigned_into.borrow()) }));
        l2.as_pointer().write((l).clone());
        assert!(({ !(*copy_assigned_into.borrow()) }));
        assert!(unimplemented!());
        assert!(
            ({
                let _lhs: Value<MaybePOCCAAllocator_int__false_> =
                    Rc::new(RefCell::new(unimplemented!()));
                let _rhs: Value<MaybePOCCAAllocator_int__false_> = Rc::new(RefCell::new(
                    MaybePOCCAAllocator_int__false_::new_1({ 3 }, { Ptr::<bool>::null() }),
                ));
                operator_eq_22(_lhs.as_pointer(), _rhs.as_pointer())
            })
        );
    }
    {
        let copy_assigned_into: Value<bool> = Rc::new(RefCell::new(false));
        let mut l: Vec<i32> = vec![2; 3_usize as usize];
        let l2: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ !(*copy_assigned_into.borrow()) }));
        l2.as_pointer().write((l).clone());
        assert!(({ (*copy_assigned_into.borrow()) }));
        assert!(unimplemented!());
        assert!(
            ({
                let _lhs: Value<MaybePOCCAAllocator_int__true_> =
                    Rc::new(RefCell::new(unimplemented!()));
                let _rhs: Value<MaybePOCCAAllocator_int__true_> = Rc::new(RefCell::new(
                    MaybePOCCAAllocator_int__true_::new_1({ 5 }, { Ptr::<bool>::null() }),
                ));
                operator_eq_24(_lhs.as_pointer(), _rhs.as_pointer())
            })
        );
    }
    {
        let copy_assigned_into: Value<bool> = Rc::new(RefCell::new(false));
        let mut l: Vec<i32> = vec![2; 3_usize as usize];
        let l2: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ !(*copy_assigned_into.borrow()) }));
        l2.as_pointer().write((l).clone());
        assert!(({ (*copy_assigned_into.borrow()) }));
        assert!(unimplemented!());
        assert!(
            ({
                let _lhs: Value<MaybePOCCAAllocator_int__true_> =
                    Rc::new(RefCell::new(unimplemented!()));
                let _rhs: Value<MaybePOCCAAllocator_int__true_> = Rc::new(RefCell::new(
                    MaybePOCCAAllocator_int__true_::new_1({ 5 }, { Ptr::<bool>::null() }),
                ));
                operator_eq_24(_lhs.as_pointer(), _rhs.as_pointer())
            })
        );
    }
    {
        let mut l: Vec<i32> = vec![2; 3_usize as usize];
        let l2: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        l2.as_pointer().write((l).clone());
        assert!(unimplemented!());
        assert!(
            ({
                let _arg0: min_allocator_int_ = unimplemented!();
                operator_eq_18(_arg0, <min_allocator_int_>::default())
            })
        );
    }
    {
        let mut l: Vec<i32> = vec![2; 3_usize as usize];
        let l2: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        l2.as_pointer().write((l).clone());
        assert!(unimplemented!());
        assert!(
            ({
                let _arg0: safe_allocator_int_ = unimplemented!();
                operator_eq_20(_arg0, safe_allocator_int_::new())
            })
        );
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ tests_26() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub trait MaybePOCCAAllocator_int__false_Impl {
    fn copy_assign(
        &self,
        a: Ptr<MaybePOCCAAllocator_int__false_>,
    ) -> Ptr<MaybePOCCAAllocator_int__false_> {
        unimplemented!()
    }
    fn allocate(&self, n: usize) -> Ptr<i32>;
    fn deallocate(&self, ptr: Ptr<i32>, n: usize);
    fn id(&self) -> i32;
}
impl MaybePOCCAAllocator_int__false_Impl for Ptr<MaybePOCCAAllocator_int__false_> {
    fn allocate(&self, mut n: usize) -> Ptr<i32> {
        return unimplemented!();
    }
    fn deallocate(&self, mut ptr: Ptr<i32>, mut n: usize) {
        unimplemented!();
    }
    fn id(&self) -> i32 {
        return (*self).with(|__s| __s.id_);
    }
}
pub trait MaybePOCCAAllocator_int__true_Impl {
    fn copy_assign(
        &self,
        a: Ptr<MaybePOCCAAllocator_int__true_>,
    ) -> Ptr<MaybePOCCAAllocator_int__true_>;
    fn allocate(&self, n: usize) -> Ptr<i32>;
    fn deallocate(&self, ptr: Ptr<i32>, n: usize);
    fn id(&self) -> i32;
}
impl MaybePOCCAAllocator_int__true_Impl for Ptr<MaybePOCCAAllocator_int__true_> {
    fn copy_assign(
        &self,
        a: Ptr<MaybePOCCAAllocator_int__true_>,
    ) -> Ptr<MaybePOCCAAllocator_int__true_> {
        field!((*self), id_).write(({ MaybePOCCAAllocator_int__true_Impl::id(&a) }));
        if !((*self).with(|__s| __s.copy_assigned_into_.clone())).is_null() {
            (*self)
                .with(|__s| __s.copy_assigned_into_.clone())
                .write(true);
        }
        return (*self).clone();
    }
    fn allocate(&self, mut n: usize) -> Ptr<i32> {
        return unimplemented!();
    }
    fn deallocate(&self, mut ptr: Ptr<i32>, mut n: usize) {
        unimplemented!();
    }
    fn id(&self) -> i32 {
        return (*self).with(|__s| __s.id_);
    }
}
pub trait MaybePOCCAAllocator_unsigned_long__false_Impl {
    fn copy_assign(
        &self,
        a: Ptr<MaybePOCCAAllocator_unsigned_long__false_>,
    ) -> Ptr<MaybePOCCAAllocator_unsigned_long__false_> {
        unimplemented!()
    }
    fn allocate(&self, mut n: usize) -> Ptr<u64> {
        unimplemented!()
    }
    fn deallocate(&self, mut ptr: Ptr<u64>, mut n: usize) {
        unimplemented!()
    }
    fn id(&self) -> i32 {
        unimplemented!()
    }
}
impl MaybePOCCAAllocator_unsigned_long__false_Impl
    for Ptr<MaybePOCCAAllocator_unsigned_long__false_>
{
}
pub trait MaybePOCCAAllocator_unsigned_long__true_Impl {
    fn copy_assign(
        &self,
        a: Ptr<MaybePOCCAAllocator_unsigned_long__true_>,
    ) -> Ptr<MaybePOCCAAllocator_unsigned_long__true_> {
        unimplemented!()
    }
    fn allocate(&self, mut n: usize) -> Ptr<u64> {
        unimplemented!()
    }
    fn deallocate(&self, mut ptr: Ptr<u64>, mut n: usize) {
        unimplemented!()
    }
    fn id(&self) -> i32 {
        unimplemented!()
    }
}
impl MaybePOCCAAllocator_unsigned_long__true_Impl
    for Ptr<MaybePOCCAAllocator_unsigned_long__true_>
{
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
pub trait min_allocator_int_Impl {
    fn allocate(&self, n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_int__std_integral_constant_unsigned_long__0__, n: usize);
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {
    fn allocate(&self, mut n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_2({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
pub trait min_allocator_unsigned_long_Impl {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_unsigned_long_Impl for Ptr<min_allocator_unsigned_long_> {}
pub trait min_pointer_const_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_arrow(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__>;
    fn operator_post_inc_10(
        &self,
        mut _a0: i32,
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
        mut _a0: i32,
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__>
{
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
}
pub trait min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool;
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_arrow(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_post_inc_9(
        &self,
        mut _a0: i32,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: i32,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(&self, n: isize) -> min_pointer_int__std_integral_constant_unsigned_long__0__;
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>
{
    fn to_bool(&self) -> bool {
        return !(((*self).with(|__s| __s.ptr_.clone())).is_null());
    }
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
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
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<u64> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<u64> {
        unimplemented!()
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: i32,
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
        mut _a0: i32,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<u64> {
        unimplemented!()
    }
}
impl min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait other_allocator_int_Impl {
    fn allocate(&self, n: usize) -> Ptr<i32>;
    fn deallocate(&self, p: Ptr<i32>, s: usize);
    fn select_on_container_copy_construction(&self) -> other_allocator_int_ {
        unimplemented!()
    }
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
}
impl other_allocator_int_Impl for Ptr<other_allocator_int_> {
    fn allocate(&self, mut n: usize) -> Ptr<i32> {
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<i32>, mut s: usize) {
        unimplemented!();
    }
}
pub trait other_allocator_unsigned_long_Impl {
    fn allocate(&self, mut n: usize) -> Ptr<u64> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<u64>, mut s: usize) {
        unimplemented!()
    }
    fn select_on_container_copy_construction(&self) -> other_allocator_unsigned_long_ {
        unimplemented!()
    }
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
}
impl other_allocator_unsigned_long_Impl for Ptr<other_allocator_unsigned_long_> {}
pub trait safe_allocator_int_Impl {
    fn allocate(&self, n: usize) -> Ptr<i32>;
    fn deallocate(&self, p: Ptr<i32>, n: usize);
}
impl safe_allocator_int_Impl for Ptr<safe_allocator_int_> {
    fn allocate(&self, mut n: usize) -> Ptr<i32> {
        let memory: Value<Ptr<i32>> = Rc::new(RefCell::new(unimplemented!()));
        {
            match ({ (*memory.borrow()).to_any() },) {
                (mut __tmp_0,) => {
                    __tmp_0.memset(
                        (0) as u8,
                        ((::std::mem::size_of::<i32>() as u64).wrapping_mul((n as u64)) as usize)
                            as usize,
                    );
                    (__tmp_0).clone()
                }
            };
        }
        return (*memory.borrow()).clone();
    }
    fn deallocate(&self, mut p: Ptr<i32>, mut n: usize) {
        {
            ({
                let _value: Value<AnyPtr> = Rc::new(RefCell::new(match ({ (p).to_any() },) {
                    (mut __tmp_1,) => {
                        __tmp_1.memset(
                            (0) as u8,
                            ((::std::mem::size_of::<i32>() as u64).wrapping_mul((n as u64))
                                as usize) as usize,
                        );
                        (__tmp_1).clone()
                    }
                }));
                DoNotOptimize_0(_value.as_pointer())
            });
        }
        unimplemented!();
    }
}
pub trait safe_allocator_unsigned_long_Impl {
    fn allocate(&self, mut n: usize) -> Ptr<u64> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<u64>, mut n: usize) {
        unimplemented!()
    }
}
impl safe_allocator_unsigned_long_Impl for Ptr<safe_allocator_unsigned_long_> {}
pub trait test_allocator_int_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<i32> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<i32> {
        unimplemented!()
    }
    fn allocate(&self, n: u32, _a1: Option<AnyPtr>) -> Ptr<i32>;
    fn deallocate(&self, p: Ptr<i32>, s: u32);
    fn max_size(&self) -> u32;
    fn destroy(&self, p: Ptr<i32>);
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
    fn get_id(&self) -> i32 {
        unimplemented!()
    }
    fn construct_16_const_int_ref(&self, p: Ptr<i32>, val: Ptr<i32>);
    fn construct_16_int_ref(&self, p: Ptr<i32>, val: Ptr<i32>);
}
impl test_allocator_int_Impl for Ptr<test_allocator_int_> {
    fn destructor(&self) {
        assert!(((*self).with(|__s| __s.data_) != destructed_value_1.with(|rc| *rc.borrow())));
        assert!(((*self).with(|__s| __s.id_) != destructed_value_1.with(|rc| *rc.borrow())));
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_dec();
            });
        }
        field!((*self), data_).write(destructed_value_1.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_1.with(|rc| *rc.borrow()));
    }
    fn allocate(&self, mut n: u32, _a1: Option<AnyPtr>) -> Ptr<i32> {
        let mut _a1: AnyPtr = _a1.unwrap_or_else(|| AnyPtr::default());
        assert!(((*self).with(|__s| __s.data_) != destructed_value_1.with(|rc| *rc.borrow())));
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
                std::process::abort();
            }
            field!((*self).with(|__s| __s.stats_.clone()), time_to_throw).with_mut(|__v| {
                __v.prefix_inc();
            });
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count).with_mut(|__v| {
                __v.prefix_inc();
            });
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                (((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size) as u32)
                    .wrapping_add(n)) as i32
            });
        }
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<i32>, mut s: u32) {
        assert!(((*self).with(|__s| __s.data_) != destructed_value_1.with(|rc| *rc.borrow())));
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count).with_mut(|__v| {
                __v.prefix_dec();
            });
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                (((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size) as u32)
                    .wrapping_sub(s)) as i32
            });
        }
        unimplemented!();
    }
    fn max_size(&self) -> u32 {
        return ((((((2147483647_u32).wrapping_mul((2_u32 as u32)) as u32)
            .wrapping_add((1_u32 as u32))) as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    }
    fn destroy(&self, mut p: Ptr<i32>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), destroy_count).with_mut(|__v| {
                __v.prefix_inc();
            });
        };
    }
    fn construct_16_const_int_ref(&self, mut p: Ptr<i32>, val: Ptr<i32>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), construct_count).with_mut(|__v| {
                __v.prefix_inc();
            });
        }
        unimplemented!();
    }
    fn construct_16_int_ref(&self, mut p: Ptr<i32>, val: Ptr<i32>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), construct_count).with_mut(|__v| {
                __v.prefix_inc();
            });
        }
        unimplemented!();
    }
}
pub trait test_allocator_statisticsImpl {
    fn clear(&self);
    fn clear_ctor_counters(&self);
}
impl test_allocator_statisticsImpl for Ptr<test_allocator_statistics> {
    fn clear(&self) {
        assert!(
            ((*self).with(|__s| __s.count) == 0)
                && (!(Ptr::<i8>::from_string_literal(b"clearing leaking allocator data?"))
                    .is_null())
        );
        field!((*self), count).write(0);
        field!((*self), time_to_throw).write(0);
        field!((*self), alloc_count).write(0);
        field!((*self), allocated_size).write(0);
        field!((*self), construct_count).write(0);
        field!((*self), destroy_count).write(0);
        field!((*self), throw_after).write(2147483647);
        ({ test_allocator_statisticsImpl::clear_ctor_counters(self) });
    }
    fn clear_ctor_counters(&self) {
        field!((*self), copied).write(0);
        field!((*self), moved).write(0);
        field!((*self), converted).write(0);
    }
}
pub trait test_allocator_unsigned_long_Impl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn address_11(&self, x: ()) -> Ptr<u64> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<u64> {
        unimplemented!()
    }
    fn allocate(&self, mut n: u32, mut _a1: AnyPtr) -> Ptr<u64> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<u64>, mut s: u32) {
        unimplemented!()
    }
    fn max_size(&self) -> u32 {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<u64>) {
        unimplemented!()
    }
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
    fn get_id(&self) -> i32 {
        unimplemented!()
    }
}
impl test_allocator_unsigned_long_Impl for Ptr<test_allocator_unsigned_long_> {}
pub trait test_allocator_void_Impl {
    fn destructor(&self);
    fn get_id(&self) -> i32;
    fn get_data(&self) -> i32;
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_>;
}
impl test_allocator_void_Impl for Ptr<test_allocator_void_> {
    fn destructor(&self) {
        field!((*self), data_).write(destructed_value_1.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_1.with(|rc| *rc.borrow()));
    }
    fn get_id(&self) -> i32 {
        return (*self).with(|__s| __s.id_);
    }
    fn get_data(&self) -> i32 {
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
    let _ = destructed_value_1.with(|_| ());
    let _ = moved_value_2.with(|_| ());
    let _ = outstanding_bytes_9.with(|_| ());
    let _ = alloc_count_10.with(|_| ());
    let _ = dealloc_count_11.with(|_| ());
    let _ = disable_default_constructor_12.with(|_| ());
}
