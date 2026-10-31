extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_1(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_2(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_3(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_4(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_5(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_6(_a0: Ptr<MoveOnly>, _a1: Ptr<MoveOnly>) -> std::cmp::Ordering {
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
        operator_lt_2(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for MoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_0(__lhs.clone(), __rhs.clone())
    }
}
impl Default for MoveOnly {
    fn default() -> Self {
        { MoveOnly::new(None) }
    }
}
pub fn operator_eq_7(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_8(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_9(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_10(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_11(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_12(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_13(_a0: Ptr<TrivialMoveOnly>, _a1: Ptr<TrivialMoveOnly>) -> std::cmp::Ordering {
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
        operator_lt_9(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for TrivialMoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_7(__lhs.clone(), __rhs.clone())
    }
}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        { TrivialMoveOnly::new(None) }
    }
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
    pub static destructed_value_14: Value<i32> = Rc::new(RefCell::new(-1_i32));
);
thread_local!(
    pub static moved_value_15: Value<i32> = Rc::new(RefCell::new(2147483647));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_16(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_17(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_16(_x, _y)
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
        operator_eq_16(__lhs.clone(), __rhs.clone())
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
pub struct limited_alloc_handle_17_ {
    #[offset(0)]
    pub outstanding_: usize,
    #[offset(8)]
    #[byte_size(8)]
    pub last_alloc_: AnyPtr,
}
impl Default for limited_alloc_handle_17_ {
    fn default() -> Self {
        {
            limited_alloc_handle_17_ {
                outstanding_: 0_usize,
                last_alloc_: AnyPtr::default(),
            }
        }
    }
}
pub fn make_thread_unsafe_shared_18() -> detail_thread_unsafe_shared_ptr_limited_alloc_handle_17__ {
    let ptr: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_17__> = Rc::new(
        RefCell::new(<detail_thread_unsafe_shared_ptr_limited_alloc_handle_17__>::default()),
    );
    let _dtor_ptr = ScopedDestructor::new(&ptr, |__p| __p.destructor());
    let alloc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*ptr.borrow_mut()).block = unimplemented!();
    unimplemented!();
    return detail_thread_unsafe_shared_ptr_limited_alloc_handle_17__::copy_from({
        ptr.as_pointer()
    });
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_17___control_block {
    pub fn new() -> Self {
        Self {
            ref_count: 1_usize,
            content: <limited_alloc_handle_17_>::default(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_17___control_block {
    #[offset(0)]
    pub ref_count: usize,
    #[offset(8)]
    #[byte_size(16)]
    pub content: limited_alloc_handle_17_,
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_17___control_block {
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_17___control_block {
                ref_count: 1_usize,
                content: <limited_alloc_handle_17_>::default(),
            }
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(8)]
pub struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_17__ {
    #[offset(0)]
    #[byte_size(8)]
    block: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_17___control_block>,
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_17__ {
    pub fn copy_from(
        other: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_17__>,
    ) -> Self {
        let __this: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_17__> =
            Rc::new(RefCell::new(Self {
                block: other.with(|__s| __s.block.clone()),
            }));
        let this: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_17__> =
            __this.as_pointer();
        field!(this.with(|__s| __s.block.clone()), ref_count).with_mut(|__v| __v.prefix_inc());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for detail_thread_unsafe_shared_ptr_limited_alloc_handle_17__ {
    fn clone(&self) -> Self {
        let __src: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_17__> = Rc::new(
            RefCell::new(detail_thread_unsafe_shared_ptr_limited_alloc_handle_17__ {
                block: self.block.clone(),
            }),
        );
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_17__::copy_from(__src.as_pointer())
    }
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_17__ {
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_17__ { block : Ptr::<detail_thread_unsafe_shared_ptr_limited_alloc_handle_17___control_block>::null()  , }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct limited_allocator_MoveOnly__17__rebind_MoveOnly_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct limited_allocator_MoveOnly__17_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_17__,
}
impl limited_allocator_MoveOnly__17_ {
    pub fn new() -> Self {
        Self {
            handle_: ({ make_thread_unsafe_shared_18() }),
        }
    }
}
impl Default for limited_allocator_MoveOnly__17_ {
    fn default() -> Self {
        { limited_allocator_MoveOnly__17_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!(
            (alloc_count_20.with(|rc| *rc.borrow()) >= dealloc_count_21.with(|rc| *rc.borrow()))
        );
        return ((alloc_count_20.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_21.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_22.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_19.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_20.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_21.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
thread_local!(
    pub static outstanding_bytes_19: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_20: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_21: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_22: Value<bool> = Rc::new(RefCell::new(false));
);
pub fn operator_sub_23(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_eq_24(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_25(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !({
        let _x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_24(_x, y)
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
impl std::cmp::PartialEq for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_24(__lhs.read(), __rhs.read())
            })
        }
    }
}
impl std::cmp::Eq for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {}
impl CppAdvance for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_8();
    }
}
impl CppIterator<MoveOnly> for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    fn iter_deref(&self) -> Ptr<MoveOnly> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__Impl::operator_inc_8(
                __it,
            );
        });
        self
    }
}
impl CppInputIterator<MoveOnly> for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: AnyPtr,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub fn operator_eq_26(
    mut x: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(
    min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    new_1,
    value
)]
#[byte_size(8)]
pub struct min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<MoveOnly>,
}
impl min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {
    pub fn new_1(mut p: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__) -> Self {
        Self {
            ptr_: (p.ptr_).clone(),
        }
    }
}
impl CppEq for min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_26(__lhs.read(), __rhs.read())
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
#[byte_size(1)]
pub struct min_allocator_MoveOnly_ {}
pub fn is_contiguous_container_asan_correct_27(_a0: Ptr<Vec<MoveOnly>>) -> bool {
    return true;
}
pub fn is_contiguous_container_asan_correct_28(_a0: Ptr<Vec<MoveOnly>>) -> bool {
    return true;
}
pub fn is_contiguous_container_asan_correct_29(_a0: Ptr<Vec<MoveOnly>>) -> bool {
    return true;
}
pub fn tests_30() -> bool {
    {
        let c: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(Vec::new()));
        {
            let __a1 = MoveOnly::new({ Some(0) });
            (*c.borrow_mut()).push(__a1)
        };
        assert!(((*c.borrow()).len() == 1_usize));
        assert!(({ is_contiguous_container_asan_correct_27(c.as_pointer(),) }));
        let mut j: i32 = 0;
        'loop_: while ((j as usize) < (*c.borrow()).len()) {
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = c.decay().offset((j as usize) as isize);
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(j) })));
                    operator_eq_0(_x, _y.as_pointer())
                })
            );
            j.prefix_inc();
        }
        {
            let __a1 = MoveOnly::new({ Some(1) });
            (*c.borrow_mut()).push(__a1)
        };
        assert!(((*c.borrow()).len() == 2_usize));
        assert!(({ is_contiguous_container_asan_correct_27(c.as_pointer(),) }));
        let mut j: i32 = 0;
        'loop_: while ((j as usize) < (*c.borrow()).len()) {
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = c.decay().offset((j as usize) as isize);
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(j) })));
                    operator_eq_0(_x, _y.as_pointer())
                })
            );
            j.prefix_inc();
        }
        {
            let __a1 = MoveOnly::new({ Some(2) });
            (*c.borrow_mut()).push(__a1)
        };
        assert!(((*c.borrow()).len() == 3_usize));
        assert!(({ is_contiguous_container_asan_correct_27(c.as_pointer(),) }));
        let mut j: i32 = 0;
        'loop_: while ((j as usize) < (*c.borrow()).len()) {
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = c.decay().offset((j as usize) as isize);
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(j) })));
                    operator_eq_0(_x, _y.as_pointer())
                })
            );
            j.prefix_inc();
        }
        {
            let __a1 = MoveOnly::new({ Some(3) });
            (*c.borrow_mut()).push(__a1)
        };
        assert!(((*c.borrow()).len() == 4_usize));
        assert!(({ is_contiguous_container_asan_correct_27(c.as_pointer(),) }));
        let mut j: i32 = 0;
        'loop_: while ((j as usize) < (*c.borrow()).len()) {
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = c.decay().offset((j as usize) as isize);
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(j) })));
                    operator_eq_0(_x, _y.as_pointer())
                })
            );
            j.prefix_inc();
        }
        {
            let __a1 = MoveOnly::new({ Some(4) });
            (*c.borrow_mut()).push(__a1)
        };
        assert!(((*c.borrow()).len() == 5_usize));
        assert!(({ is_contiguous_container_asan_correct_27(c.as_pointer(),) }));
        let mut j: i32 = 0;
        'loop_: while ((j as usize) < (*c.borrow()).len()) {
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = c.decay().offset((j as usize) as isize);
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(j) })));
                    operator_eq_0(_x, _y.as_pointer())
                })
            );
            j.prefix_inc();
        }
    }
    {
        let c: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(Vec::new()));
        {
            let __a1 = MoveOnly::new({ Some(0) });
            (*c.borrow_mut()).push(__a1)
        };
        assert!(((*c.borrow()).len() == 1_usize));
        assert!(({ is_contiguous_container_asan_correct_28(c.as_pointer(),) }));
        let mut j: i32 = 0;
        'loop_: while ((j as usize) < (*c.borrow()).len()) {
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = c.decay().offset((j as usize) as isize);
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(j) })));
                    operator_eq_0(_x, _y.as_pointer())
                })
            );
            j.prefix_inc();
        }
        {
            let __a1 = MoveOnly::new({ Some(1) });
            (*c.borrow_mut()).push(__a1)
        };
        assert!(((*c.borrow()).len() == 2_usize));
        assert!(({ is_contiguous_container_asan_correct_28(c.as_pointer(),) }));
        let mut j: i32 = 0;
        'loop_: while ((j as usize) < (*c.borrow()).len()) {
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = c.decay().offset((j as usize) as isize);
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(j) })));
                    operator_eq_0(_x, _y.as_pointer())
                })
            );
            j.prefix_inc();
        }
        {
            let __a1 = MoveOnly::new({ Some(2) });
            (*c.borrow_mut()).push(__a1)
        };
        assert!(((*c.borrow()).len() == 3_usize));
        assert!(({ is_contiguous_container_asan_correct_28(c.as_pointer(),) }));
        let mut j: i32 = 0;
        'loop_: while ((j as usize) < (*c.borrow()).len()) {
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = c.decay().offset((j as usize) as isize);
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(j) })));
                    operator_eq_0(_x, _y.as_pointer())
                })
            );
            j.prefix_inc();
        }
        {
            let __a1 = MoveOnly::new({ Some(3) });
            (*c.borrow_mut()).push(__a1)
        };
        assert!(((*c.borrow()).len() == 4_usize));
        assert!(({ is_contiguous_container_asan_correct_28(c.as_pointer(),) }));
        let mut j: i32 = 0;
        'loop_: while ((j as usize) < (*c.borrow()).len()) {
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = c.decay().offset((j as usize) as isize);
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(j) })));
                    operator_eq_0(_x, _y.as_pointer())
                })
            );
            j.prefix_inc();
        }
        {
            let __a1 = MoveOnly::new({ Some(4) });
            (*c.borrow_mut()).push(__a1)
        };
        assert!(((*c.borrow()).len() == 5_usize));
        assert!(({ is_contiguous_container_asan_correct_28(c.as_pointer(),) }));
        let mut j: i32 = 0;
        'loop_: while ((j as usize) < (*c.borrow()).len()) {
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = c.decay().offset((j as usize) as isize);
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(j) })));
                    operator_eq_0(_x, _y.as_pointer())
                })
            );
            j.prefix_inc();
        }
    }
    {
        let c: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(Vec::new()));
        {
            let __a1 = MoveOnly::new({ Some(0) });
            (*c.borrow_mut()).push(__a1)
        };
        assert!(((*c.borrow()).len() == 1_usize));
        assert!(({ is_contiguous_container_asan_correct_29(c.as_pointer(),) }));
        let mut j: i32 = 0;
        'loop_: while ((j as usize) < (*c.borrow()).len()) {
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = c.decay().offset((j as usize) as isize);
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(j) })));
                    operator_eq_0(_x, _y.as_pointer())
                })
            );
            j.prefix_inc();
        }
        {
            let __a1 = MoveOnly::new({ Some(1) });
            (*c.borrow_mut()).push(__a1)
        };
        assert!(((*c.borrow()).len() == 2_usize));
        assert!(({ is_contiguous_container_asan_correct_29(c.as_pointer(),) }));
        let mut j: i32 = 0;
        'loop_: while ((j as usize) < (*c.borrow()).len()) {
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = c.decay().offset((j as usize) as isize);
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(j) })));
                    operator_eq_0(_x, _y.as_pointer())
                })
            );
            j.prefix_inc();
        }
        {
            let __a1 = MoveOnly::new({ Some(2) });
            (*c.borrow_mut()).push(__a1)
        };
        assert!(((*c.borrow()).len() == 3_usize));
        assert!(({ is_contiguous_container_asan_correct_29(c.as_pointer(),) }));
        let mut j: i32 = 0;
        'loop_: while ((j as usize) < (*c.borrow()).len()) {
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = c.decay().offset((j as usize) as isize);
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(j) })));
                    operator_eq_0(_x, _y.as_pointer())
                })
            );
            j.prefix_inc();
        }
        {
            let __a1 = MoveOnly::new({ Some(3) });
            (*c.borrow_mut()).push(__a1)
        };
        assert!(((*c.borrow()).len() == 4_usize));
        assert!(({ is_contiguous_container_asan_correct_29(c.as_pointer(),) }));
        let mut j: i32 = 0;
        'loop_: while ((j as usize) < (*c.borrow()).len()) {
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = c.decay().offset((j as usize) as isize);
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(j) })));
                    operator_eq_0(_x, _y.as_pointer())
                })
            );
            j.prefix_inc();
        }
        {
            let __a1 = MoveOnly::new({ Some(4) });
            (*c.borrow_mut()).push(__a1)
        };
        assert!(((*c.borrow()).len() == 5_usize));
        assert!(({ is_contiguous_container_asan_correct_29(c.as_pointer(),) }));
        let mut j: i32 = 0;
        'loop_: while ((j as usize) < (*c.borrow()).len()) {
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = c.decay().offset((j as usize) as isize);
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(j) })));
                    operator_eq_0(_x, _y.as_pointer())
                })
            );
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
    ({ tests_30() });
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
pub trait detail_thread_unsafe_shared_ptr_limited_alloc_handle_17__Impl {
    fn destructor(&self);
    fn operator_deref_4(&self) -> Ptr<limited_alloc_handle_17_> {
        unimplemented!()
    }
    fn operator_arrow_5(&self) -> Ptr<limited_alloc_handle_17_> {
        unimplemented!()
    }
    fn operator_deref_6(&self) -> Ptr<limited_alloc_handle_17_> {
        unimplemented!()
    }
    fn operator_arrow_7(&self) -> Ptr<limited_alloc_handle_17_>;
    fn get_8(&self) -> Ptr<limited_alloc_handle_17_> {
        unimplemented!()
    }
    fn get_9(&self) -> Ptr<limited_alloc_handle_17_> {
        unimplemented!()
    }
    fn copy_assign(
        &self,
        _a0: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_17__>,
    ) -> Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_17__>;
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_17__Impl
    for Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_17__>
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
    fn operator_arrow_7(&self) -> Ptr<limited_alloc_handle_17_> {
        return (field_ptr!((*self).with(|__s| __s.block.clone()), content));
    }
    fn copy_assign(
        &self,
        _a0: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_17__>,
    ) -> Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_17__> {
        field!((*self), block).write({ _a0.with(|__s| __s.block.clone()) });
        return (*self).clone();
    }
}
pub trait limited_alloc_handle_17_Impl {
    fn allocate_1_class_MoveOnly(&self, N: usize) -> Ptr<MoveOnly>;
    fn deallocate_2_class_MoveOnly(&self, ptr: Ptr<MoveOnly>, N: usize);
}
impl limited_alloc_handle_17_Impl for Ptr<limited_alloc_handle_17_> {
    fn allocate_1_class_MoveOnly(&self, mut N: usize) -> Ptr<MoveOnly> {
        if ((N).wrapping_add((*self).with(|__s| __s.outstanding_)) > (17_u64 as usize)) {
            std::process::abort();
        }
        let alloc: Value<Ptr<MoveOnly>> = Rc::new(RefCell::new(unimplemented!()));
        field!((*self), last_alloc_).write((*alloc.borrow()).to_any());
        field!((*self), outstanding_)
            .write({ ((*self).with(|__s| __s.outstanding_)).wrapping_add(N) });
        return (*alloc.borrow()).clone();
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
pub trait limited_allocator_MoveOnly__17_Impl {
    fn allocate(&self, n: usize) -> Ptr<MoveOnly>;
    fn deallocate(&self, p: Ptr<MoveOnly>, n: usize);
    fn max_size(&self) -> usize;
    fn getHandle(&self) -> Ptr<limited_alloc_handle_17_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl limited_allocator_MoveOnly__17_Impl for Ptr<limited_allocator_MoveOnly__17_> {
    fn allocate(&self, mut n: usize) -> Ptr<MoveOnly> {
        return ({
            limited_alloc_handle_17_Impl::allocate_1_class_MoveOnly(
                &({
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_17__Impl::operator_arrow_7(
                        &field_ptr!((*self), handle_),
                    )
                }),
                n,
            )
        });
    }
    fn deallocate(&self, mut p: Ptr<MoveOnly>, mut n: usize) {
        ({
            limited_alloc_handle_17_Impl::deallocate_2_class_MoveOnly(
                &({
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_17__Impl::operator_arrow_7(
                        &field_ptr!((*self), handle_),
                    )
                }),
                (p).clone(),
                n,
            )
        });
    }
    fn max_size(&self) -> usize {
        return (17_u64 as usize);
    }
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
pub trait min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool;
    fn operator_deref(&self) -> Ptr<MoveOnly>;
    fn operator_arrow(&self) -> Ptr<MoveOnly>;
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
        n: isize,
    ) -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__>;
    fn operator_add(
        &self,
        n: isize,
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__;
    fn operator_sub(
        &self,
        n: isize,
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__;
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
    fn operator_arrow(&self) -> Ptr<MoveOnly> {
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
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() - n)
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
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__Impl::operator_sub_assign(
                &tmp.as_pointer(),
                n,
            )
        });
        return (*tmp.borrow()).clone();
    }
}
pub trait min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: i32,
    ) -> min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: i32,
    ) -> min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<MoveOnly> {
        unimplemented!()
    }
}
impl min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__>
{
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
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
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
pub trait test_allocator_void_Impl {
    fn destructor(&self);
    fn get_id(&self) -> i32;
    fn get_data(&self) -> i32;
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_>;
}
impl test_allocator_void_Impl for Ptr<test_allocator_void_> {
    fn destructor(&self) {
        field!((*self), data_).write(destructed_value_14.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_14.with(|rc| *rc.borrow()));
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
    let _ = destructed_value_14.with(|_| ());
    let _ = moved_value_15.with(|_| ());
    let _ = outstanding_bytes_19.with(|_| ());
    let _ = alloc_count_20.with(|_| ());
    let _ = dealloc_count_21.with(|_| ());
    let _ = disable_default_constructor_22.with(|_| ());
}
