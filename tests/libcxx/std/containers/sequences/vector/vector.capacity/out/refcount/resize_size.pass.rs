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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Ctor_Tag {}
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct limited_alloc_handle_301_ {
    #[offset(0)]
    pub outstanding_: usize,
    #[offset(8)]
    #[byte_size(8)]
    pub last_alloc_: AnyPtr,
}
impl Default for limited_alloc_handle_301_ {
    fn default() -> Self {
        {
            limited_alloc_handle_301_ {
                outstanding_: 0_usize,
                last_alloc_: AnyPtr::default(),
            }
        }
    }
}
pub fn make_thread_unsafe_shared_5() -> detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__ {
    let ptr: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__> = Rc::new(
        RefCell::new(<detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__>::default()),
    );
    let _dtor_ptr = ScopedDestructor::new(&ptr, |__p| __p.destructor());
    let alloc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*ptr.borrow_mut()).block = unimplemented!();
    unimplemented!();
    return detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__::copy_from({
        ptr.as_pointer()
    });
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_301___control_block {
    pub fn new() -> Self {
        Self {
            ref_count: 1_usize,
            content: <limited_alloc_handle_301_>::default(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_301___control_block {
    #[offset(0)]
    pub ref_count: usize,
    #[offset(8)]
    #[byte_size(16)]
    pub content: limited_alloc_handle_301_,
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_301___control_block {
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_301___control_block {
                ref_count: 1_usize,
                content: <limited_alloc_handle_301_>::default(),
            }
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(8)]
pub struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__ {
    #[offset(0)]
    #[byte_size(8)]
    block: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_301___control_block>,
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__ {
    pub fn copy_from(
        other: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__>,
    ) -> Self {
        let __this: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__> =
            Rc::new(RefCell::new(Self {
                block: other.with(|__s| __s.block.clone()),
            }));
        let this: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__> =
            __this.as_pointer();
        field!(this.with(|__s| __s.block.clone()), ref_count).with_mut(|__v| __v.prefix_inc());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__ {
    fn clone(&self) -> Self {
        let __src: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__> = Rc::new(
            RefCell::new(detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__ {
                block: self.block.clone(),
            }),
        );
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__::copy_from(__src.as_pointer())
    }
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__ {
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__ {
                block: Ptr::<
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_301___control_block,
                >::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct limited_allocator_int__301__rebind_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct limited_allocator_int__301__rebind_unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct limited_allocator_int__301_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__,
}
impl limited_allocator_int__301_ {
    pub fn new() -> Self {
        Self {
            handle_: ({ make_thread_unsafe_shared_5() }),
        }
    }
}
impl Default for limited_allocator_int__301_ {
    fn default() -> Self {
        { limited_allocator_int__301_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct limited_allocator_MoveOnly__301__rebind_MoveOnly_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct limited_allocator_MoveOnly__301__rebind_unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct limited_allocator_MoveOnly__301_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__,
}
impl limited_allocator_MoveOnly__301_ {
    pub fn new() -> Self {
        Self {
            handle_: ({ make_thread_unsafe_shared_5() }),
        }
    }
}
impl Default for limited_allocator_MoveOnly__301_ {
    fn default() -> Self {
        { limited_allocator_MoveOnly__301_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct limited_allocator_unsigned_long__301_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__,
}
impl limited_allocator_unsigned_long__301_ {}
pub fn operator_eq_6(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_7(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_8(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_9(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_10(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_11(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_12(_a0: Ptr<MoveOnly>, _a1: Ptr<MoveOnly>) -> std::cmp::Ordering {
    {
        let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
            return (*cmp.borrow());
        }
    }
    return std::cmp::Ordering::Equal;
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, CppFrom, MoveAssign)]
#[cpp_from(i32, new, some_value)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    data_: i32,
}
impl MoveOnly {
    pub fn new(data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| 1);
        Self { data_: data }
    }
    pub fn move_from(x: Ptr<MoveOnly>) -> Self {
        let __this: MoveOnly = Self {
            data_: x.with(|__s| __s.data_),
        };
        field!(x, data_).write(0);
        __this
    }
}
impl CppLess for MoveOnly {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_8(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for MoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_6(__lhs.clone(), __rhs.clone())
    }
}
impl Default for MoveOnly {
    fn default() -> Self {
        { MoveOnly::new(None) }
    }
}
pub fn operator_eq_13(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_14(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_15(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_16(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_17(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_18(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_19(_a0: Ptr<TrivialMoveOnly>, _a1: Ptr<TrivialMoveOnly>) -> std::cmp::Ordering {
    {
        let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
            return (*cmp.borrow());
        }
    }
    return std::cmp::Ordering::Equal;
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, CppFrom, MoveAssign)]
#[cpp_from(i32, new, some_value)]
#[byte_size(4)]
pub struct TrivialMoveOnly {
    #[offset(0)]
    data_: i32,
}
impl TrivialMoveOnly {
    pub fn new(data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| 1);
        Self { data_: data }
    }
    pub fn move_from(_a0: Ptr<TrivialMoveOnly>) -> Self {
        Self {
            data_: { (*_a0.upgrade().deref()).data_ },
        }
    }
}
impl CppLess for TrivialMoveOnly {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_15(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for TrivialMoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_13(__lhs.clone(), __rhs.clone())
    }
}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        { TrivialMoveOnly::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!(
            (alloc_count_21.with(|rc| *rc.borrow()) >= dealloc_count_22.with(|rc| *rc.borrow()))
        );
        return ((alloc_count_21.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_22.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_23.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_20.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_21.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_22.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
thread_local!(
    pub static outstanding_bytes_20: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_21: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_22: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_23: Value<bool> = Rc::new(RefCell::new(false));
);
pub fn operator_sub_24(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_eq_25(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_26(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !({
        let _x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_25(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<MoveOnly>,
}
impl min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<MoveOnly>) -> Self {
        Self { ptr_: (p).clone() }
    }
}
impl CppEq for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_25(__lhs.read(), __rhs.read())
    }
}
impl CppAdvance for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_8();
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<u64>,
}
impl min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_MoveOnly_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct safe_allocator_int_ {}
impl safe_allocator_int_ {
    pub fn new() -> Self {
        Self {}
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
pub fn is_contiguous_container_asan_correct_27(_a0: Ptr<Vec<i32>>) -> bool {
    return true;
}
pub fn is_contiguous_container_asan_correct_28(_a0: Ptr<Vec<i32>>) -> bool {
    return true;
}
pub fn is_contiguous_container_asan_correct_29(_a0: Ptr<Vec<MoveOnly>>) -> bool {
    return true;
}
pub fn is_contiguous_container_asan_correct_30(_a0: Ptr<Vec<MoveOnly>>) -> bool {
    return true;
}
pub fn is_contiguous_container_asan_correct_31(_a0: Ptr<Vec<MoveOnly>>) -> bool {
    return true;
}
pub fn is_contiguous_container_asan_correct_32(_a0: Ptr<Vec<i32>>) -> bool {
    return true;
}
pub fn tests_33() -> bool {
    {
        let v: Value<Vec<i32>> = Rc::new(RefCell::new(
            (0..(100_usize) as usize).map(|_| 0_i32).collect::<Vec<_>>(),
        ));
        {
            let __a0 = 50_usize as usize;
            (*v.borrow_mut()).resize_with(__a0, || 0_i32)
        };
        assert!(((*v.borrow()).len() == 50_usize));
        assert!(((*v.borrow()).capacity() == 100_usize));
        assert!(({ is_contiguous_container_asan_correct_27(v.as_pointer(),) }));
        {
            let __a0 = 200_usize as usize;
            (*v.borrow_mut()).resize_with(__a0, || 0_i32)
        };
        assert!(((*v.borrow()).len() == 200_usize));
        assert!(((*v.borrow()).capacity() >= 200_usize));
        assert!(({ is_contiguous_container_asan_correct_27(v.as_pointer(),) }));
    }
    {
        let v: Value<Vec<i32>> = Rc::new(RefCell::new(
            (0..(100_usize) as usize).map(|_| 0_i32).collect::<Vec<_>>(),
        ));
        {
            let __a0 = 50_usize as usize;
            (*v.borrow_mut()).resize_with(__a0, || 0_i32)
        };
        assert!(((*v.borrow()).len() == 50_usize));
        assert!(((*v.borrow()).capacity() == 100_usize));
        assert!(({ is_contiguous_container_asan_correct_28(v.as_pointer(),) }));
        {
            let __a0 = 200_usize as usize;
            (*v.borrow_mut()).resize_with(__a0, || 0_i32)
        };
        assert!(((*v.borrow()).len() == 200_usize));
        assert!(((*v.borrow()).capacity() >= 200_usize));
        assert!(({ is_contiguous_container_asan_correct_28(v.as_pointer(),) }));
    }
    {
        let v: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(
            (0..(100_usize) as usize)
                .map(|_| <MoveOnly>::default())
                .collect::<Vec<_>>(),
        ));
        {
            let __a0 = 50_usize as usize;
            (*v.borrow_mut()).resize_with(__a0, || <MoveOnly>::default())
        };
        assert!(((*v.borrow()).len() == 50_usize));
        assert!(((*v.borrow()).capacity() == 100_usize));
        assert!(({ is_contiguous_container_asan_correct_29(v.as_pointer(),) }));
        {
            let __a0 = 200_usize as usize;
            (*v.borrow_mut()).resize_with(__a0, || <MoveOnly>::default())
        };
        assert!(((*v.borrow()).len() == 200_usize));
        assert!(((*v.borrow()).capacity() >= 200_usize));
        assert!(({ is_contiguous_container_asan_correct_29(v.as_pointer(),) }));
    }
    {
        let v: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(
            (0..(100_usize) as usize)
                .map(|_| <MoveOnly>::default())
                .collect::<Vec<_>>(),
        ));
        {
            let __a0 = 50_usize as usize;
            (*v.borrow_mut()).resize_with(__a0, || <MoveOnly>::default())
        };
        assert!(((*v.borrow()).len() == 50_usize));
        assert!(((*v.borrow()).capacity() == 100_usize));
        assert!(({ is_contiguous_container_asan_correct_30(v.as_pointer(),) }));
        {
            let __a0 = 200_usize as usize;
            (*v.borrow_mut()).resize_with(__a0, || <MoveOnly>::default())
        };
        assert!(((*v.borrow()).len() == 200_usize));
        assert!(((*v.borrow()).capacity() >= 200_usize));
        assert!(({ is_contiguous_container_asan_correct_30(v.as_pointer(),) }));
    }
    {
        let v: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(
            (0..(100_usize) as usize)
                .map(|_| <MoveOnly>::default())
                .collect::<Vec<_>>(),
        ));
        {
            let __a0 = 50_usize as usize;
            (*v.borrow_mut()).resize_with(__a0, || <MoveOnly>::default())
        };
        assert!(((*v.borrow()).len() == 50_usize));
        assert!(((*v.borrow()).capacity() == 100_usize));
        assert!(({ is_contiguous_container_asan_correct_31(v.as_pointer(),) }));
        {
            let __a0 = 200_usize as usize;
            (*v.borrow_mut()).resize_with(__a0, || <MoveOnly>::default())
        };
        assert!(((*v.borrow()).len() == 200_usize));
        assert!(((*v.borrow()).capacity() >= 200_usize));
        assert!(({ is_contiguous_container_asan_correct_31(v.as_pointer(),) }));
    }
    {
        let v: Value<Vec<i32>> = Rc::new(RefCell::new(
            (0..(100_usize) as usize).map(|_| 0_i32).collect::<Vec<_>>(),
        ));
        {
            let __a0 = 50_usize as usize;
            (*v.borrow_mut()).resize_with(__a0, || 0_i32)
        };
        assert!(((*v.borrow()).len() == 50_usize));
        assert!(((*v.borrow()).capacity() == 100_usize));
        assert!(({ is_contiguous_container_asan_correct_32(v.as_pointer(),) }));
        {
            let __a0 = 200_usize as usize;
            (*v.borrow_mut()).resize_with(__a0, || 0_i32)
        };
        assert!(((*v.borrow()).len() == 200_usize));
        assert!(((*v.borrow()).capacity() >= 200_usize));
        assert!(({ is_contiguous_container_asan_correct_32(v.as_pointer(),) }));
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
    ({ tests_33() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub trait MoveOnlyImpl {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly>;
    fn get(&self) -> i32;
    fn operator_add(&self, x: Ptr<MoveOnly>) -> MoveOnly;
    fn operator_mul(&self, x: Ptr<MoveOnly>) -> MoveOnly;
}
impl MoveOnlyImpl for Ptr<MoveOnly> {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        field!((*self), data_).write({ x.with(|__s| __s.data_) });
        field!(x, data_).write(0);
        return (*self).clone();
    }
    fn get(&self) -> i32 {
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
    fn get(&self) -> i32;
    fn operator_add(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
    fn operator_mul(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
}
impl TrivialMoveOnlyImpl for Ptr<TrivialMoveOnly> {
    fn move_assign(&self, _a0: Ptr<TrivialMoveOnly>) -> Ptr<TrivialMoveOnly> {
        field!((*self), data_).write({ { (*_a0.upgrade().deref()).data_ } });
        return (*self).clone();
    }
    fn get(&self) -> i32 {
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
pub trait detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__Impl {
    fn destructor(&self);
    fn operator_deref_4(&self) -> Ptr<limited_alloc_handle_301_> {
        unimplemented!()
    }
    fn operator_arrow_5(&self) -> Ptr<limited_alloc_handle_301_> {
        unimplemented!()
    }
    fn operator_deref_6(&self) -> Ptr<limited_alloc_handle_301_> {
        unimplemented!()
    }
    fn operator_arrow_7(&self) -> Ptr<limited_alloc_handle_301_>;
    fn get_8(&self) -> Ptr<limited_alloc_handle_301_> {
        unimplemented!()
    }
    fn get_9(&self) -> Ptr<limited_alloc_handle_301_> {
        unimplemented!()
    }
    fn copy_assign(
        &self,
        _a0: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__>,
    ) -> Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__>;
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__Impl
    for Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__>
{
    fn destructor(&self) {
        field!((*self).with(|__s| __s.block.clone()), ref_count).with_mut(|__v| __v.prefix_dec());
        if ((*self)
            .with(|__s| __s.block.clone())
            .with(|__s| __s.ref_count)
            != 0_usize)
        {
            return;
        };
        let alloc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    fn operator_arrow_7(&self) -> Ptr<limited_alloc_handle_301_> {
        return (field_ptr!((*self).with(|__s| __s.block.clone()), content));
    }
    fn copy_assign(
        &self,
        _a0: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__>,
    ) -> Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__> {
        field!((*self), block).write({ _a0.with(|__s| __s.block.clone()) });
        return (*self).clone();
    }
}
pub trait limited_alloc_handle_301_Impl {
    fn allocate_1_int(&self, N: usize) -> Ptr<i32>;
    fn allocate_1_class_MoveOnly(&self, N: usize) -> Ptr<MoveOnly>;
    fn deallocate_2_int(&self, ptr: Ptr<i32>, N: usize);
    fn deallocate_2_class_MoveOnly(&self, ptr: Ptr<MoveOnly>, N: usize);
}
impl limited_alloc_handle_301_Impl for Ptr<limited_alloc_handle_301_> {
    fn allocate_1_int(&self, mut N: usize) -> Ptr<i32> {
        if ((N).wrapping_add((*self).with(|__s| __s.outstanding_)) > (301_u64 as usize)) {
            std::process::abort();
        }
        let alloc: Value<Ptr<i32>> = Rc::new(RefCell::new(unimplemented!()));
        field!((*self), last_alloc_).write((*alloc.borrow()).to_any());
        field!((*self), outstanding_)
            .write({ ((*self).with(|__s| __s.outstanding_)).wrapping_add(N) });
        return (*alloc.borrow()).clone();
    }
    fn allocate_1_class_MoveOnly(&self, mut N: usize) -> Ptr<MoveOnly> {
        if ((N).wrapping_add((*self).with(|__s| __s.outstanding_)) > (301_u64 as usize)) {
            std::process::abort();
        }
        let alloc: Value<Ptr<MoveOnly>> = Rc::new(RefCell::new(unimplemented!()));
        field!((*self), last_alloc_).write((*alloc.borrow()).to_any());
        field!((*self), outstanding_)
            .write({ ((*self).with(|__s| __s.outstanding_)).wrapping_add(N) });
        return (*alloc.borrow()).clone();
    }
    fn deallocate_2_int(&self, mut ptr: Ptr<i32>, mut N: usize) {
        if ({ (ptr).to_any() } == { (*self).with(|__s| __s.last_alloc_.clone()) }) {
            field!((*self), last_alloc_).write(AnyPtr::default());
            assert!(((*self).with(|__s| __s.outstanding_) >= N));
            field!((*self), outstanding_)
                .write({ ((*self).with(|__s| __s.outstanding_)).wrapping_sub(N) });
        }
        unimplemented!();
    }
    fn deallocate_2_class_MoveOnly(&self, mut ptr: Ptr<MoveOnly>, mut N: usize) {
        if ({ (ptr).to_any() } == { (*self).with(|__s| __s.last_alloc_.clone()) }) {
            field!((*self), last_alloc_).write(AnyPtr::default());
            assert!(((*self).with(|__s| __s.outstanding_) >= N));
            field!((*self), outstanding_)
                .write({ ((*self).with(|__s| __s.outstanding_)).wrapping_sub(N) });
        }
        unimplemented!();
    }
}
pub trait limited_allocator_MoveOnly__301_Impl {
    fn allocate(&self, n: usize) -> Ptr<MoveOnly>;
    fn deallocate(&self, p: Ptr<MoveOnly>, n: usize);
    fn max_size(&self) -> usize;
    fn getHandle(&self) -> Ptr<limited_alloc_handle_301_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl limited_allocator_MoveOnly__301_Impl for Ptr<limited_allocator_MoveOnly__301_> {
    fn allocate(&self, mut n: usize) -> Ptr<MoveOnly> {
        return ({
            limited_alloc_handle_301_Impl::allocate_1_class_MoveOnly(
                &({
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__Impl::operator_arrow_7(
                        &field_ptr!((*self), handle_),
                    )
                }),
                n,
            )
        });
    }
    fn deallocate(&self, mut p: Ptr<MoveOnly>, mut n: usize) {
        ({
            limited_alloc_handle_301_Impl::deallocate_2_class_MoveOnly(
                &({
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__Impl::operator_arrow_7(
                        &field_ptr!((*self), handle_),
                    )
                }),
                (p).clone(),
                n,
            )
        });
    }
    fn max_size(&self) -> usize {
        return (301_u64 as usize);
    }
    fn destructor(&self) {
        field_ptr!(self, handle_).destructor();
    }
}
pub trait limited_allocator_int__301_Impl {
    fn allocate(&self, n: usize) -> Ptr<i32>;
    fn deallocate(&self, p: Ptr<i32>, n: usize);
    fn max_size(&self) -> usize;
    fn getHandle(&self) -> Ptr<limited_alloc_handle_301_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl limited_allocator_int__301_Impl for Ptr<limited_allocator_int__301_> {
    fn allocate(&self, mut n: usize) -> Ptr<i32> {
        return ({
            limited_alloc_handle_301_Impl::allocate_1_int(
                &({
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__Impl::operator_arrow_7(
                        &field_ptr!((*self), handle_),
                    )
                }),
                n,
            )
        });
    }
    fn deallocate(&self, mut p: Ptr<i32>, mut n: usize) {
        ({
            limited_alloc_handle_301_Impl::deallocate_2_int(
                &({
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_301__Impl::operator_arrow_7(
                        &field_ptr!((*self), handle_),
                    )
                }),
                (p).clone(),
                n,
            )
        });
    }
    fn max_size(&self) -> usize {
        return (301_u64 as usize);
    }
    fn destructor(&self) {
        field_ptr!(self, handle_).destructor();
    }
}
pub trait limited_allocator_unsigned_long__301_Impl {
    fn allocate(&self, mut n: usize) -> Ptr<u64> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<u64>, mut n: usize) {
        unimplemented!()
    }
    fn max_size(&self) -> usize {
        unimplemented!()
    }
    fn getHandle(&self) -> Ptr<limited_alloc_handle_301_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl limited_allocator_unsigned_long__301_Impl for Ptr<limited_allocator_unsigned_long__301_> {
    fn destructor(&self) {
        field_ptr!(self, handle_).destructor();
    }
}
pub trait min_allocator_MoveOnly_Impl {
    fn allocate(&self, n: usize) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__;
    fn deallocate(
        &self,
        p: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
        n: usize,
    );
}
impl min_allocator_MoveOnly_Impl for Ptr<min_allocator_MoveOnly_> {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        return min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
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
pub trait min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool;
    fn operator_deref(&self) -> Ptr<MoveOnly>;
    fn operator_arrow(&self) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn operator_inc_8(&self)
    -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__>;
    fn operator_post_inc_9(
        &self,
        mut _a0: i32,
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: i32,
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: isize,
    ) -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        n: isize,
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__;
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<MoveOnly> {
        unimplemented!()
    }
}
impl min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__>
{
    fn to_bool(&self) -> bool {
        return !(((*self).with(|__s| __s.ptr_.clone())).is_null());
    }
    fn operator_deref(&self) -> Ptr<MoveOnly> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__Impl::operator_add_assign(
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
pub trait safe_allocator_int_Impl {
    fn allocate(&self, n: usize) -> Ptr<i32>;
    fn deallocate(&self, p: Ptr<i32>, n: usize);
}
impl safe_allocator_int_Impl for Ptr<safe_allocator_int_> {
    fn allocate(&self, mut n: usize) -> Ptr<i32> {
        let memory: Value<Ptr<i32>> = Rc::new(RefCell::new(unimplemented!()));
        {
            {
                (*memory.borrow()).to_any().memset(
                    (0) as u8,
                    ((::std::mem::size_of::<i32>() as u64).wrapping_mul((n as u64)) as usize)
                        as usize,
                );
                (*memory.borrow()).to_any()
            };
        }
        return (*memory.borrow()).clone();
    }
    fn deallocate(&self, mut p: Ptr<i32>, mut n: usize) {
        {
            ({
                let _value: Value<AnyPtr> = Rc::new(RefCell::new({
                    (p).to_any().memset(
                        (0) as u8,
                        ((::std::mem::size_of::<i32>() as u64).wrapping_mul((n as u64)) as usize)
                            as usize,
                    );
                    (p).to_any()
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
    let _ = outstanding_bytes_20.with(|_| ());
    let _ = alloc_count_21.with(|_| ());
    let _ = dealloc_count_22.with(|_| ());
    let _ = disable_default_constructor_23.with(|_| ());
}
