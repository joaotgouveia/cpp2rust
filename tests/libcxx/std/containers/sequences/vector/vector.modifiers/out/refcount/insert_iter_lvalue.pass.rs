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
pub struct limited_alloc_handle_300_ {
    #[offset(0)]
    pub outstanding_: usize,
    #[offset(8)]
    #[byte_size(8)]
    pub last_alloc_: AnyPtr,
}
impl Default for limited_alloc_handle_300_ {
    fn default() -> Self {
        {
            limited_alloc_handle_300_ {
                outstanding_: 0_usize,
                last_alloc_: AnyPtr::default(),
            }
        }
    }
}
pub fn make_thread_unsafe_shared_5() -> detail_thread_unsafe_shared_ptr_limited_alloc_handle_300__ {
    let ptr: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_300__> = Rc::new(
        RefCell::new(<detail_thread_unsafe_shared_ptr_limited_alloc_handle_300__>::default()),
    );
    let _dtor_ptr = ScopedDestructor::new(&ptr, |__p| __p.destructor());
    let alloc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*ptr.borrow_mut()).block = unimplemented!();
    unimplemented!();
    return detail_thread_unsafe_shared_ptr_limited_alloc_handle_300__::copy_from({
        ptr.as_pointer()
    });
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_300___control_block {
    pub fn new() -> Self {
        Self {
            ref_count: 1_usize,
            content: <limited_alloc_handle_300_>::default(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_300___control_block {
    #[offset(0)]
    pub ref_count: usize,
    #[offset(8)]
    #[byte_size(16)]
    pub content: limited_alloc_handle_300_,
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_300___control_block {
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_300___control_block {
                ref_count: 1_usize,
                content: <limited_alloc_handle_300_>::default(),
            }
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(8)]
pub struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_300__ {
    #[offset(0)]
    #[byte_size(8)]
    block: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_300___control_block>,
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_300__ {
    pub fn copy_from(
        other: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_300__>,
    ) -> Self {
        let __this: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_300__> =
            Rc::new(RefCell::new(Self {
                block: other.with(|__s| __s.block.clone()),
            }));
        let this: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_300__> =
            __this.as_pointer();
        field!(this.with(|__s| __s.block.clone()), ref_count).with_mut(|__v| __v.prefix_inc());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for detail_thread_unsafe_shared_ptr_limited_alloc_handle_300__ {
    fn clone(&self) -> Self {
        let __src: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_300__> = Rc::new(
            RefCell::new(detail_thread_unsafe_shared_ptr_limited_alloc_handle_300__ {
                block: self.block.clone(),
            }),
        );
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_300__::copy_from(__src.as_pointer())
    }
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_300__ {
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_300__ {
                block: Ptr::<
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_300___control_block,
                >::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct limited_allocator_int__300__rebind_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct limited_allocator_int__300__rebind_unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct limited_allocator_int__300_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_300__,
}
impl limited_allocator_int__300_ {
    pub fn new() -> Self {
        Self {
            handle_: ({ make_thread_unsafe_shared_5() }),
        }
    }
}
impl Default for limited_allocator_int__300_ {
    fn default() -> Self {
        { limited_allocator_int__300_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct limited_allocator_unsigned_long__300_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_300__,
}
impl limited_allocator_unsigned_long__300_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_7.with(|rc| *rc.borrow()) >= dealloc_count_8.with(|rc| *rc.borrow())));
        return ((alloc_count_7.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_8.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_9.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_6.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_7.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_8.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
thread_local!(
    pub static outstanding_bytes_6: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_7: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_8: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_9: Value<bool> = Rc::new(RefCell::new(false));
);
pub fn operator_sub_10(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_lt_11(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } < { (y.ptr_).clone() });
}
pub fn operator_eq_12(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_13(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
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
    ptr_: Ptr<i32>,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<i32>) -> Self {
        Self { ptr_: (p).clone() }
    }
}
impl std::cmp::Ord for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                if operator_lt_11(__lhs.read(), __rhs.read()) {
                    std::cmp::Ordering::Less
                } else if operator_lt_11(__rhs.read(), __lhs.read()) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            })
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
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_12(__lhs.read(), __rhs.read())
            })
        }
    }
}
impl std::cmp::Eq for min_pointer_int__std_integral_constant_unsigned_long__0__ {}
impl CppAdvance for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_8();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_10();
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
impl CppBidirectionalIterator<i32> for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn iter_dec(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            min_pointer_int__std_integral_constant_unsigned_long__0__Impl::operator_dec_10(__it);
        });
        self
    }
}
impl CppRandomAccessIterator<i32> for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn iter_add_assign(&mut self, n: isize) -> Self {
        libcc2rs::with_record_ptr(self, |__it| {
            min_pointer_int__std_integral_constant_unsigned_long__0__Impl::operator_add_assign(
                __it, n as _,
            );
        });
        self.clone()
    }
    fn iter_distance(&self, from: &Self) -> isize {
        libcc2rs::with_record_ptrs(self, from, |__lhs, __rhs| {
            (operator_sub_10(__lhs.read(), __rhs.read())) as isize
        })
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
pub fn operator_sub_14(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_lt_15(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } < { (y.ptr_).clone() });
}
pub fn operator_eq_16(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
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
impl CppLess for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_15(__lhs.read(), __rhs.read())
    }
}
impl CppEq for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_16(__lhs.read(), __rhs.read())
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
pub struct min_allocator_int_ {}
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
pub fn is_contiguous_container_asan_correct_17(_a0: Ptr<Vec<i32>>) -> bool {
    return true;
}
pub fn is_contiguous_container_asan_correct_18(_a0: Ptr<Vec<i32>>) -> bool {
    return true;
}
pub fn is_contiguous_container_asan_correct_19(_a0: Ptr<Vec<i32>>) -> bool {
    return true;
}
pub fn is_contiguous_container_asan_correct_20(_a0: Ptr<Vec<i32>>) -> bool {
    return true;
}
pub fn test_21() -> bool {
    {
        let v: Value<Vec<i32>> = Rc::new(RefCell::new(
            (0..(100_usize) as usize).map(|_| 0_i32).collect::<Vec<_>>(),
        ));
        let mut lvalue: i32 = 1;
        let i: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new({
            let __it = unimplemented!().iter_add(10_isize);
            let __off = __it.index();
            (*v.borrow_mut()).insert(__off, lvalue);
            __it
        }));
        assert!(((*v.borrow()).len() == 101_usize));
        assert!(({ is_contiguous_container_asan_correct_17(v.as_pointer(),) }));
        assert!((*i.borrow()) == v.decay().begin().iter_add(10_isize));
        let mut j: i32 = 0_i32;
        j = 0;
        'loop_: while (j < 10) {
            assert!(((v.decay().offset((j as usize) as isize).read()) == 0));
            j.prefix_inc();
        }
        assert!(((v.decay().offset((j as usize) as isize).read()) == 1));
        j.prefix_inc();
        'loop_: while (j < 101) {
            assert!(((v.decay().offset((j as usize) as isize).read()) == 0));
            j.prefix_inc();
        }
    }
    {
        let mut n: usize = 100_usize;
        let v: Value<Vec<i32>> = Rc::new(RefCell::new(
            (0..(n) as usize).map(|_| 0_i32).collect::<Vec<_>>(),
        ));
        if (n).wrapping_add(1_usize) as usize > (*v.borrow()).capacity() as usize {
            let len_0 = (*v.borrow()).len();
            (*v.borrow_mut()).reserve_exact((n).wrapping_add(1_usize) as usize - len_0 as usize);
        };
        let mut lvalue: i32 = 1;
        let it: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new({
            let __it = unimplemented!().iter_add((n as isize));
            let __off = __it.index();
            (*v.borrow_mut()).insert(__off, lvalue);
            __it
        }));
        assert!(((*v.borrow()).len() == (n).wrapping_add(1_usize)));
        assert!(({ is_contiguous_container_asan_correct_17(v.as_pointer(),) }));
        assert!((*it.borrow()) == v.decay().begin().iter_add((n as isize)));
        let mut i: usize = 0_usize;
        'loop_: while (i < n) {
            assert!(((v.decay().offset(i as isize).read()) == 0));
            i.prefix_inc();
        }
        assert!(((v.decay().offset(n as isize).read()) == lvalue));
    }
    {
        let v: Value<Vec<i32>> = Rc::new(RefCell::new(
            (0..(100_usize) as usize).map(|_| 0_i32).collect::<Vec<_>>(),
        ));
        'loop_: while ((*v.borrow()).len() < (*v.borrow()).capacity()) {
            {
                let __a1 = 0;
                (*v.borrow_mut()).push(__a1)
            };
        }
        let mut sz: usize = (*v.borrow()).len();
        let mut lvalue: i32 = 1;
        let i: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new({
            let __it = unimplemented!().iter_add(10_isize);
            let __off = __it.index();
            (*v.borrow_mut()).insert(__off, lvalue);
            __it
        }));
        assert!(((*v.borrow()).len() == (sz).wrapping_add(1_usize)));
        assert!(({ is_contiguous_container_asan_correct_17(v.as_pointer(),) }));
        assert!((*i.borrow()) == v.decay().begin().iter_add(10_isize));
        let mut j: usize = 0_usize;
        j = 0_usize;
        'loop_: while (j < 10_usize) {
            assert!(((v.decay().offset(j as isize).read()) == 0));
            j.prefix_inc();
        }
        assert!(((v.decay().offset(j as isize).read()) == 1));
        j.prefix_inc();
        'loop_: while (j < (*v.borrow()).len()) {
            assert!(((v.decay().offset(j as isize).read()) == 0));
            j.prefix_inc();
        }
    }
    {
        let v: Value<Vec<i32>> = Rc::new(RefCell::new(
            (0..(100_usize) as usize).map(|_| 0_i32).collect::<Vec<_>>(),
        ));
        'loop_: while ((*v.borrow()).len() < (*v.borrow()).capacity()) {
            {
                let __a1 = 0;
                (*v.borrow_mut()).push(__a1)
            };
        }
        (*v.borrow_mut()).pop();
        (*v.borrow_mut()).pop();
        let mut sz: usize = (*v.borrow()).len();
        let mut lvalue: i32 = 1;
        let i: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new({
            let __it = unimplemented!().iter_add(10_isize);
            let __off = __it.index();
            (*v.borrow_mut()).insert(__off, lvalue);
            __it
        }));
        assert!(((*v.borrow()).len() == (sz).wrapping_add(1_usize)));
        assert!(({ is_contiguous_container_asan_correct_17(v.as_pointer(),) }));
        assert!((*i.borrow()) == v.decay().begin().iter_add(10_isize));
        let mut j: usize = 0_usize;
        j = 0_usize;
        'loop_: while (j < 10_usize) {
            assert!(((v.decay().offset(j as isize).read()) == 0));
            j.prefix_inc();
        }
        assert!(((v.decay().offset(j as isize).read()) == 1));
        j.prefix_inc();
        'loop_: while (j < (*v.borrow()).len()) {
            assert!(((v.decay().offset(j as isize).read()) == 0));
            j.prefix_inc();
        }
    }
    {
        let v: Value<Vec<i32>> = Rc::new(RefCell::new(
            (0..(100_usize) as usize).map(|_| 0_i32).collect::<Vec<_>>(),
        ));
        let mut lvalue: i32 = 1;
        let i: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new({
            let __it = unimplemented!().iter_add(10_isize);
            let __off = __it.index();
            (*v.borrow_mut()).insert(__off, lvalue);
            __it
        }));
        assert!(((*v.borrow()).len() == 101_usize));
        assert!(({ is_contiguous_container_asan_correct_18(v.as_pointer(),) }));
        assert!((*i.borrow()) == v.decay().begin().iter_add(10_isize));
        let mut j: i32 = 0_i32;
        j = 0;
        'loop_: while (j < 10) {
            assert!(((v.decay().offset((j as usize) as isize).read()) == 0));
            j.prefix_inc();
        }
        assert!(((v.decay().offset((j as usize) as isize).read()) == 1));
        j.prefix_inc();
        'loop_: while (j < 101) {
            assert!(((v.decay().offset((j as usize) as isize).read()) == 0));
            j.prefix_inc();
        }
    }
    {
        let v: Value<Vec<i32>> = Rc::new(RefCell::new(
            (0..(100_usize) as usize).map(|_| 0_i32).collect::<Vec<_>>(),
        ));
        let mut lvalue: i32 = 1;
        let i: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new({
            let __it = unimplemented!().iter_add(10_isize);
            let __off = __it.index();
            (*v.borrow_mut()).insert(__off, lvalue);
            __it
        }));
        assert!(((*v.borrow()).len() == 101_usize));
        assert!(({ is_contiguous_container_asan_correct_19(v.as_pointer(),) }));
        assert!((*i.borrow()) == v.decay().begin().iter_add(10_isize));
        let mut j: i32 = 0_i32;
        j = 0;
        'loop_: while (j < 10) {
            assert!(((v.decay().offset((j as usize) as isize).read()) == 0));
            j.prefix_inc();
        }
        assert!(((v.decay().offset((j as usize) as isize).read()) == 1));
        j.prefix_inc();
        'loop_: while (j < 101) {
            assert!(((v.decay().offset((j as usize) as isize).read()) == 0));
            j.prefix_inc();
        }
    }
    {
        let v: Value<Vec<i32>> = Rc::new(RefCell::new(
            (0..(100_usize) as usize).map(|_| 0_i32).collect::<Vec<_>>(),
        ));
        let mut lvalue: i32 = 1;
        let i: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new({
            let __it = unimplemented!().iter_add(10_isize);
            let __off = __it.index();
            (*v.borrow_mut()).insert(__off, lvalue);
            __it
        }));
        assert!(((*v.borrow()).len() == 101_usize));
        assert!(({ is_contiguous_container_asan_correct_20(v.as_pointer(),) }));
        assert!((*i.borrow()) == v.decay().begin().iter_add(10_isize));
        let mut j: i32 = 0_i32;
        j = 0;
        'loop_: while (j < 10) {
            assert!(((v.decay().offset((j as usize) as isize).read()) == 0));
            j.prefix_inc();
        }
        assert!(((v.decay().offset((j as usize) as isize).read()) == 1));
        j.prefix_inc();
        'loop_: while (j < 101) {
            assert!(((v.decay().offset((j as usize) as isize).read()) == 0));
            j.prefix_inc();
        }
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
    ({ test_21() });
    const _: () = assert!(true, "test()");;
    return 0;
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
pub trait detail_thread_unsafe_shared_ptr_limited_alloc_handle_300__Impl {
    fn destructor(&self);
    fn operator_deref_4(&self) -> Ptr<limited_alloc_handle_300_> {
        unimplemented!()
    }
    fn operator_arrow_5(&self) -> Ptr<limited_alloc_handle_300_> {
        unimplemented!()
    }
    fn operator_deref_6(&self) -> Ptr<limited_alloc_handle_300_> {
        unimplemented!()
    }
    fn operator_arrow_7(&self) -> Ptr<limited_alloc_handle_300_>;
    fn get_8(&self) -> Ptr<limited_alloc_handle_300_> {
        unimplemented!()
    }
    fn get_9(&self) -> Ptr<limited_alloc_handle_300_> {
        unimplemented!()
    }
    fn copy_assign(
        &self,
        _a0: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_300__>,
    ) -> Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_300__>;
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_300__Impl
    for Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_300__>
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
    fn operator_arrow_7(&self) -> Ptr<limited_alloc_handle_300_> {
        return (field_ptr!((*self).with(|__s| __s.block.clone()), content));
    }
    fn copy_assign(
        &self,
        _a0: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_300__>,
    ) -> Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_300__> {
        field!((*self), block).write({ _a0.with(|__s| __s.block.clone()) });
        return (*self).clone();
    }
}
pub trait limited_alloc_handle_300_Impl {
    fn allocate_1_int(&self, N: usize) -> Ptr<i32>;
    fn deallocate_2_int(&self, ptr: Ptr<i32>, N: usize);
}
impl limited_alloc_handle_300_Impl for Ptr<limited_alloc_handle_300_> {
    fn allocate_1_int(&self, mut N: usize) -> Ptr<i32> {
        if ((N).wrapping_add((*self).with(|__s| __s.outstanding_)) > (300_u64 as usize)) {
            std::process::abort();
        }
        let alloc: Value<Ptr<i32>> = Rc::new(RefCell::new(unimplemented!()));
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
}
pub trait limited_allocator_int__300_Impl {
    fn allocate(&self, n: usize) -> Ptr<i32>;
    fn deallocate(&self, p: Ptr<i32>, n: usize);
    fn max_size(&self) -> usize;
    fn getHandle(&self) -> Ptr<limited_alloc_handle_300_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl limited_allocator_int__300_Impl for Ptr<limited_allocator_int__300_> {
    fn allocate(&self, mut n: usize) -> Ptr<i32> {
        return ({
            limited_alloc_handle_300_Impl::allocate_1_int(
                &({
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_300__Impl::operator_arrow_7(
                        &field_ptr!((*self), handle_),
                    )
                }),
                n,
            )
        });
    }
    fn deallocate(&self, mut p: Ptr<i32>, mut n: usize) {
        ({
            limited_alloc_handle_300_Impl::deallocate_2_int(
                &({
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_300__Impl::operator_arrow_7(
                        &field_ptr!((*self), handle_),
                    )
                }),
                (p).clone(),
                n,
            )
        });
    }
    fn max_size(&self) -> usize {
        return (300_u64 as usize);
    }
    fn destructor(&self) {
        field_ptr!(self, handle_).destructor();
    }
}
pub trait limited_allocator_unsigned_long__300_Impl {
    fn allocate(&self, mut n: usize) -> Ptr<u64> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<u64>, mut n: usize) {
        unimplemented!()
    }
    fn max_size(&self) -> usize {
        unimplemented!()
    }
    fn getHandle(&self) -> Ptr<limited_alloc_handle_300_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl limited_allocator_unsigned_long__300_Impl for Ptr<limited_allocator_unsigned_long__300_> {
    fn destructor(&self) {
        field_ptr!(self, handle_).destructor();
    }
}
pub trait min_allocator_int_Impl {
    fn allocate(&self, n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_int__std_integral_constant_unsigned_long__0__, n: usize);
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {
    fn allocate(&self, mut n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_1({
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
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
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
        n: isize,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        n: isize,
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__;
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
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_const_int__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_const_int__std_integral_constant_unsigned_long__0__Impl::operator_add_assign(
                &tmp.as_pointer(),
                n,
            )
        });
        return (*tmp.borrow()).clone();
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
    fn operator_arrow(&self) -> Ptr<i32>;
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_post_inc_9(
        &self,
        mut _a0: i32,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
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
        n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_add(&self, n: isize) -> min_pointer_int__std_integral_constant_unsigned_long__0__;
    fn operator_sub(&self, n: isize) -> min_pointer_int__std_integral_constant_unsigned_long__0__;
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
    fn operator_arrow(&self) -> Ptr<i32> {
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
        mut n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() - n)
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
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_int__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_int__std_integral_constant_unsigned_long__0__Impl::operator_sub_assign(
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
    let _ = outstanding_bytes_6.with(|_| ());
    let _ = alloc_count_7.with(|_| ());
    let _ = dealloc_count_8.with(|_| ());
    let _ = disable_default_constructor_9.with(|_| ());
}
