extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(a: Ptr<test_less_int_>, b: Ptr<test_less_int_>) -> bool {
    return ({ a.with(|__s| __s.data_) } == { b.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct test_less_int_ {
    #[offset(0)]
    pub data_: i32,
}
impl test_less_int_ {
    pub fn new_1() -> Self {
        Self { data_: 0 }
    }
    pub fn new_2(mut data: i32) -> Self {
        Self { data_: data }
    }
}
impl CppEq for test_less_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_0(__lhs.clone(), __rhs.clone())
    }
}
impl Callable2<Ptr<i32>, Ptr<i32>, bool> for test_less_int_ {
    fn call(&self, a1: Ptr<i32>, a2: Ptr<i32>) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            test_less_int_Impl::operator_call(__it, a1, a2)
        })
    }
}
impl CppMove for test_less_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for test_less_int_ {
    fn default() -> Self {
        { test_less_int_::new_1() }
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
        assert!((alloc_count_6.with(|rc| *rc.borrow()) >= dealloc_count_7.with(|rc| *rc.borrow())));
        return ((alloc_count_6.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_7.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_8.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_5.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_6.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_7.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_5: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_6: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_7: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_8: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<(Value<i32>, Value<i32>)>,
}
impl min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {}
impl CppMove for min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_pair_const_int__int__ {}
impl CppMove for min_allocator_std_pair_const_int__int__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_lt_9(left: Ptr<NonTrivial>, right: Ptr<NonTrivial>) -> bool {
    return ({ left.with(|__s| __s.i) } < { right.with(|__s| __s.i) });
}
pub fn operator_eq_10(left: Ptr<NonTrivial>, right: Ptr<NonTrivial>) -> bool {
    return ({ left.with(|__s| __s.i) } == { right.with(|__s| __s.i) });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CppFrom, CopyAssign, MoveAssign, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct NonTrivial {
    #[offset(0)]
    pub i: i32,
}
impl NonTrivial {
    pub fn new(mut arg_i: i32) -> Self {
        Self { i: arg_i }
    }
    pub fn copy_from(other: Ptr<NonTrivial>) -> Self {
        Self {
            i: other.with(|__s| __s.i),
        }
    }
    pub fn move_from(other: Ptr<NonTrivial>) -> Self {
        let __this: NonTrivial = Self {
            i: other.with(|__s| __s.i),
        };
        field!(other, i).write(0);
        __this
    }
}
impl CppLess for NonTrivial {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_9(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for NonTrivial {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_10(__lhs.clone(), __rhs.clone())
    }
}
impl Clone for NonTrivial {
    fn clone(&self) -> Self {
        let __src: Value<NonTrivial> = Rc::new(RefCell::new(NonTrivial { i: self.i.clone() }));
        NonTrivial::copy_from(__src.as_pointer())
    }
}
impl CppMove for NonTrivial {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
const _: () = assert!(
    true,
    "!std::is_trivially_copy_assignable<NonTrivial>::value"
);
const _: () = assert!(
    true,
    "!std::is_trivially_default_constructible<NonTrivial>::value"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Vec<AnyPtr  >  , new, mut_ptr)]
#[byte_size(8)]
pub struct tracking_allocator_std_pair_const_int__int__ {
    #[offset(0)]
    #[byte_size(8)]
    allocs_: Ptr<Vec<AnyPtr>>,
}
impl tracking_allocator_std_pair_const_int__int__ {
    pub fn new(allocs: Ptr<Vec<AnyPtr>>) -> Self {
        Self {
            allocs_: (allocs).clone(),
        }
    }
}
impl CppMove for tracking_allocator_std_pair_const_int__int__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoOp {}
impl Callable0<()> for NoOp {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| NoOpImpl::operator_call(__it))
    }
}
impl CppMove for NoOp {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_alloc_11(
    lhs_alloc: Option<Ptr<()>>,
    rhs_alloc: Option<Ptr<()>>,
    check_alloc_invariant: Option<NoOp>,
) -> bool {
    let mut __lhs_alloc_default: Option<Value<()>> = None;
    let lhs_alloc: Ptr<()> = lhs_alloc.unwrap_or_else(|| {
        __lhs_alloc_default
            .insert(Rc::new(RefCell::new(unimplemented!())))
            .as_pointer()
    });
    let mut __rhs_alloc_default: Option<Value<()>> = None;
    let rhs_alloc: Ptr<()> = rhs_alloc.unwrap_or_else(|| {
        __rhs_alloc_default
            .insert(Rc::new(RefCell::new(unimplemented!())))
            .as_pointer()
    });
    let check_alloc_invariant: Value<NoOp> = Rc::new(RefCell::new(
        check_alloc_invariant.unwrap_or_else(|| <NoOp>::default()),
    ));
    {
        {
            let arr: Value<Box<[(Value<i32>, Value<i32>)]>> = Rc::new(RefCell::new(Box::new([
                (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(1))),
                (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(3))),
                (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(6))),
            ])));
            let orig: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            copy.as_pointer().write((*orig.borrow()).clone());
            assert!(({ ((*copy.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    copy.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                    3_isize
                ) == copy
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
            assert!(({ ((*orig.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    orig.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                    3_isize
                ) == orig
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let orig: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            copy.as_pointer().write((*orig.borrow()).clone());
            assert!(({ ((*copy.borrow()).len() == 0_usize) }));
            assert!(
                copy.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin())
                    == copy
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
            assert!(({ ((*orig.borrow()).len() == 0_usize) }));
            assert!(
                orig.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin())
                    == orig
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let arr: Value<Box<[(Value<i32>, Value<i32>)]>> = Rc::new(RefCell::new(Box::new([
                (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(1))),
                (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(3))),
                (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(6))),
            ])));
            let orig: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            copy.as_pointer().write((*orig.borrow()).clone());
            assert!(({ ((*copy.borrow()).len() == 0_usize) }));
            assert!(
                copy.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin())
                    == copy
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
            assert!(({ ((*orig.borrow()).len() == 0_usize) }));
            assert!(
                orig.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin())
                    == orig
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
        }
    }
    {
        {
            let arr: Value<Box<[(Value<i32>, Value<i32>)]>> = Rc::new(RefCell::new(Box::new([
                (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(1))),
                (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(3))),
                (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(6))),
            ])));
            let orig: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            orig.as_pointer().write((*orig.borrow()).clone());
            assert!(({ ((*orig.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    orig.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                    3_isize
                ) == orig
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
        }
        {
            let orig: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            orig.as_pointer().write((*orig.borrow()).clone());
            assert!(({ ((*orig.borrow()).len() == 0_usize) }));
            assert!(
                orig.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin())
                    == orig
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
        }
    }
    {
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let lhs_arr: Value<Box<[(Value<i32>, Value<i32>)]>> =
                Rc::new(RefCell::new(Box::new([
                    (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(1))),
                    (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(3))),
                    (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(6))),
                ])));
            let orig: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            let rhs_arr: Value<Box<[(Value<i32>, Value<i32>)]>> =
                Rc::new(RefCell::new(Box::new([
                    (Rc::new(RefCell::new(4)), Rc::new(RefCell::new(2))),
                    (Rc::new(RefCell::new(5)), Rc::new(RefCell::new(1))),
                ])));
            let copy: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            copy.as_pointer().write((*orig.borrow()).clone());
            assert!(({ ((*copy.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    copy.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                    3_isize
                ) == copy
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
            assert!(({ ((*orig.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    orig.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                    3_isize
                ) == orig
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let lhs_arr: Value<Box<[(Value<i32>, Value<i32>)]>> =
                Rc::new(RefCell::new(Box::new([
                    (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(1))),
                    (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(3))),
                    (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(6))),
                ])));
            let orig: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            let rhs_arr: Value<Box<[(Value<i32>, Value<i32>)]>> =
                Rc::new(RefCell::new(Box::new([
                    (Rc::new(RefCell::new(4)), Rc::new(RefCell::new(2))),
                    (Rc::new(RefCell::new(5)), Rc::new(RefCell::new(1))),
                    (Rc::new(RefCell::new(6)), Rc::new(RefCell::new(0))),
                ])));
            let copy: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            copy.as_pointer().write((*orig.borrow()).clone());
            assert!(({ ((*copy.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    copy.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                    3_isize
                ) == copy
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
            assert!(({ ((*orig.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    orig.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                    3_isize
                ) == orig
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let lhs_arr: Value<Box<[(Value<i32>, Value<i32>)]>> =
                Rc::new(RefCell::new(Box::new([
                    (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(1))),
                    (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(3))),
                    (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(6))),
                ])));
            let orig: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            let rhs_arr: Value<Box<[(Value<i32>, Value<i32>)]>> =
                Rc::new(RefCell::new(Box::new([
                    (Rc::new(RefCell::new(4)), Rc::new(RefCell::new(2))),
                    (Rc::new(RefCell::new(5)), Rc::new(RefCell::new(1))),
                    (Rc::new(RefCell::new(6)), Rc::new(RefCell::new(0))),
                    (Rc::new(RefCell::new(7)), Rc::new(RefCell::new(9))),
                ])));
            let copy: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            copy.as_pointer().write((*orig.borrow()).clone());
            assert!(({ ((*copy.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    copy.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                    3_isize
                ) == copy
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
            assert!(({ ((*orig.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    orig.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                    3_isize
                ) == orig
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
    }
    {
        let orig: Value<Map<i32, (Value<i32>, Value<i32>)>> =
            Rc::new(RefCell::new(unimplemented!()));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) != 50) {
            let __rhs = ((*i.borrow()) + 3);
            Map::subscript(orig.as_pointer(), i.as_pointer()).write(__rhs);
            (*i.borrow_mut()).prefix_inc();
        }
        let copy: Value<Map<i32, (Value<i32>, Value<i32>)>> =
            Rc::new(RefCell::new(unimplemented!()));
        copy.as_pointer().write((*orig.borrow()).clone());
        let mut i: i32 = 0;
        'loop_: for v in IterRange::new(
            copy.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
            copy.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end()),
        ) {
            let v: Value<(Value<i32>, Value<i32>)> =
                Rc::new(RefCell::new(match (v.iter_deref(),) {
                    (__tmp_0,) => (
                        Rc::new(RefCell::new(
                            ((*__tmp_0.upgrade().deref()).0.as_pointer().read()),
                        )),
                        Rc::new(RefCell::new(
                            ((*__tmp_0.upgrade().deref()).1.as_pointer().read()),
                        )),
                    ),
                }));
            assert!(unimplemented!());
            i.prefix_inc();
        }
    }
    ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
    return true;
}
pub fn test_alloc_12(
    lhs_alloc: Option<Ptr<min_allocator_std_pair_const_int__int__>>,
    rhs_alloc: Option<Ptr<min_allocator_std_pair_const_int__int__>>,
    check_alloc_invariant: Option<NoOp>,
) -> bool {
    let mut __lhs_alloc_default: Option<Value<min_allocator_std_pair_const_int__int__>> = None;
    let lhs_alloc: Ptr<min_allocator_std_pair_const_int__int__> = lhs_alloc.unwrap_or_else(|| {
        __lhs_alloc_default
            .insert(Rc::new(RefCell::new(
                <min_allocator_std_pair_const_int__int__>::default(),
            )))
            .as_pointer()
    });
    let mut __rhs_alloc_default: Option<Value<min_allocator_std_pair_const_int__int__>> = None;
    let rhs_alloc: Ptr<min_allocator_std_pair_const_int__int__> = rhs_alloc.unwrap_or_else(|| {
        __rhs_alloc_default
            .insert(Rc::new(RefCell::new(
                <min_allocator_std_pair_const_int__int__>::default(),
            )))
            .as_pointer()
    });
    let check_alloc_invariant: Value<NoOp> = Rc::new(RefCell::new(
        check_alloc_invariant.unwrap_or_else(|| <NoOp>::default()),
    ));
    {
        {
            let arr: Value<Box<[(Value<i32>, Value<i32>)]>> = Rc::new(RefCell::new(Box::new([
                (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(1))),
                (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(3))),
                (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(6))),
            ])));
            let orig: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            copy.as_pointer().write((*orig.borrow()).clone());
            assert!(({ ((*copy.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    copy.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                    3_isize
                ) == copy
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
            assert!(({ ((*orig.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    orig.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                    3_isize
                ) == orig
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let orig: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            copy.as_pointer().write((*orig.borrow()).clone());
            assert!(({ ((*copy.borrow()).len() == 0_usize) }));
            assert!(
                copy.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin())
                    == copy
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
            assert!(({ ((*orig.borrow()).len() == 0_usize) }));
            assert!(
                orig.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin())
                    == orig
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let arr: Value<Box<[(Value<i32>, Value<i32>)]>> = Rc::new(RefCell::new(Box::new([
                (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(1))),
                (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(3))),
                (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(6))),
            ])));
            let orig: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            copy.as_pointer().write((*orig.borrow()).clone());
            assert!(({ ((*copy.borrow()).len() == 0_usize) }));
            assert!(
                copy.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin())
                    == copy
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
            assert!(({ ((*orig.borrow()).len() == 0_usize) }));
            assert!(
                orig.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin())
                    == orig
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
        }
    }
    {
        {
            let arr: Value<Box<[(Value<i32>, Value<i32>)]>> = Rc::new(RefCell::new(Box::new([
                (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(1))),
                (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(3))),
                (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(6))),
            ])));
            let orig: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            orig.as_pointer().write((*orig.borrow()).clone());
            assert!(({ ((*orig.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    orig.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                    3_isize
                ) == orig
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
        }
        {
            let orig: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            orig.as_pointer().write((*orig.borrow()).clone());
            assert!(({ ((*orig.borrow()).len() == 0_usize) }));
            assert!(
                orig.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin())
                    == orig
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
        }
    }
    {
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let lhs_arr: Value<Box<[(Value<i32>, Value<i32>)]>> =
                Rc::new(RefCell::new(Box::new([
                    (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(1))),
                    (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(3))),
                    (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(6))),
                ])));
            let orig: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            let rhs_arr: Value<Box<[(Value<i32>, Value<i32>)]>> =
                Rc::new(RefCell::new(Box::new([
                    (Rc::new(RefCell::new(4)), Rc::new(RefCell::new(2))),
                    (Rc::new(RefCell::new(5)), Rc::new(RefCell::new(1))),
                ])));
            let copy: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            copy.as_pointer().write((*orig.borrow()).clone());
            assert!(({ ((*copy.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    copy.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                    3_isize
                ) == copy
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
            assert!(({ ((*orig.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    orig.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                    3_isize
                ) == orig
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let lhs_arr: Value<Box<[(Value<i32>, Value<i32>)]>> =
                Rc::new(RefCell::new(Box::new([
                    (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(1))),
                    (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(3))),
                    (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(6))),
                ])));
            let orig: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            let rhs_arr: Value<Box<[(Value<i32>, Value<i32>)]>> =
                Rc::new(RefCell::new(Box::new([
                    (Rc::new(RefCell::new(4)), Rc::new(RefCell::new(2))),
                    (Rc::new(RefCell::new(5)), Rc::new(RefCell::new(1))),
                    (Rc::new(RefCell::new(6)), Rc::new(RefCell::new(0))),
                ])));
            let copy: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            copy.as_pointer().write((*orig.borrow()).clone());
            assert!(({ ((*copy.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    copy.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                    3_isize
                ) == copy
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
            assert!(({ ((*orig.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    orig.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                    3_isize
                ) == orig
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let lhs_arr: Value<Box<[(Value<i32>, Value<i32>)]>> =
                Rc::new(RefCell::new(Box::new([
                    (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(1))),
                    (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(3))),
                    (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(6))),
                ])));
            let orig: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            let rhs_arr: Value<Box<[(Value<i32>, Value<i32>)]>> =
                Rc::new(RefCell::new(Box::new([
                    (Rc::new(RefCell::new(4)), Rc::new(RefCell::new(2))),
                    (Rc::new(RefCell::new(5)), Rc::new(RefCell::new(1))),
                    (Rc::new(RefCell::new(6)), Rc::new(RefCell::new(0))),
                    (Rc::new(RefCell::new(7)), Rc::new(RefCell::new(9))),
                ])));
            let copy: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            copy.as_pointer().write((*orig.borrow()).clone());
            assert!(({ ((*copy.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    copy.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                    3_isize
                ) == copy
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
            assert!(({ ((*orig.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    orig.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                    3_isize
                ) == orig
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
    }
    {
        let orig: Value<Map<i32, (Value<i32>, Value<i32>)>> =
            Rc::new(RefCell::new(unimplemented!()));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) != 50) {
            let __rhs = ((*i.borrow()) + 3);
            Map::subscript(orig.as_pointer(), i.as_pointer()).write(__rhs);
            (*i.borrow_mut()).prefix_inc();
        }
        let copy: Value<Map<i32, (Value<i32>, Value<i32>)>> =
            Rc::new(RefCell::new(unimplemented!()));
        copy.as_pointer().write((*orig.borrow()).clone());
        let mut i: i32 = 0;
        'loop_: for v in IterRange::new(
            copy.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
            copy.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end()),
        ) {
            let v: Value<(Value<i32>, Value<i32>)> =
                Rc::new(RefCell::new(match (v.iter_deref(),) {
                    (__tmp_1,) => (
                        Rc::new(RefCell::new(
                            ((*__tmp_1.upgrade().deref()).0.as_pointer().read()),
                        )),
                        Rc::new(RefCell::new(
                            ((*__tmp_1.upgrade().deref()).1.as_pointer().read()),
                        )),
                    ),
                }));
            assert!(unimplemented!());
            i.prefix_inc();
        }
    }
    ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
    return true;
}
pub fn test_alloc_13(
    lhs_alloc: Ptr<tracking_allocator_std_pair_const_int__int__>,
    rhs_alloc: Ptr<tracking_allocator_std_pair_const_int__int__>,
    check_alloc_invariant: AssertEmpty_14,
) -> bool {
    let check_alloc_invariant: Value<AssertEmpty_14> = Rc::new(RefCell::new(check_alloc_invariant));
    {
        {
            let arr: Value<Box<[(Value<i32>, Value<i32>)]>> = Rc::new(RefCell::new(Box::new([
                (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(1))),
                (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(3))),
                (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(6))),
            ])));
            let orig: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            copy.as_pointer().write((*orig.borrow()).clone());
            assert!(({ ((*copy.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    copy.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                    3_isize
                ) == copy
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
            assert!(({ ((*orig.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    orig.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                    3_isize
                ) == orig
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
        }
        ({ AssertEmpty_14Impl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let orig: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            copy.as_pointer().write((*orig.borrow()).clone());
            assert!(({ ((*copy.borrow()).len() == 0_usize) }));
            assert!(
                copy.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin())
                    == copy
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
            assert!(({ ((*orig.borrow()).len() == 0_usize) }));
            assert!(
                orig.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin())
                    == orig
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
        }
        ({ AssertEmpty_14Impl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let arr: Value<Box<[(Value<i32>, Value<i32>)]>> = Rc::new(RefCell::new(Box::new([
                (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(1))),
                (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(3))),
                (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(6))),
            ])));
            let orig: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            copy.as_pointer().write((*orig.borrow()).clone());
            assert!(({ ((*copy.borrow()).len() == 0_usize) }));
            assert!(
                copy.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin())
                    == copy
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
            assert!(({ ((*orig.borrow()).len() == 0_usize) }));
            assert!(
                orig.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin())
                    == orig
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
        }
    }
    {
        {
            let arr: Value<Box<[(Value<i32>, Value<i32>)]>> = Rc::new(RefCell::new(Box::new([
                (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(1))),
                (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(3))),
                (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(6))),
            ])));
            let orig: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            orig.as_pointer().write((*orig.borrow()).clone());
            assert!(({ ((*orig.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    orig.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                    3_isize
                ) == orig
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
        }
        {
            let orig: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            orig.as_pointer().write((*orig.borrow()).clone());
            assert!(({ ((*orig.borrow()).len() == 0_usize) }));
            assert!(
                orig.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin())
                    == orig
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
        }
    }
    {
        ({ AssertEmpty_14Impl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let lhs_arr: Value<Box<[(Value<i32>, Value<i32>)]>> =
                Rc::new(RefCell::new(Box::new([
                    (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(1))),
                    (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(3))),
                    (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(6))),
                ])));
            let orig: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            let rhs_arr: Value<Box<[(Value<i32>, Value<i32>)]>> =
                Rc::new(RefCell::new(Box::new([
                    (Rc::new(RefCell::new(4)), Rc::new(RefCell::new(2))),
                    (Rc::new(RefCell::new(5)), Rc::new(RefCell::new(1))),
                ])));
            let copy: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            copy.as_pointer().write((*orig.borrow()).clone());
            assert!(({ ((*copy.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    copy.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                    3_isize
                ) == copy
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
            assert!(({ ((*orig.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    orig.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                    3_isize
                ) == orig
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
        }
        ({ AssertEmpty_14Impl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let lhs_arr: Value<Box<[(Value<i32>, Value<i32>)]>> =
                Rc::new(RefCell::new(Box::new([
                    (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(1))),
                    (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(3))),
                    (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(6))),
                ])));
            let orig: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            let rhs_arr: Value<Box<[(Value<i32>, Value<i32>)]>> =
                Rc::new(RefCell::new(Box::new([
                    (Rc::new(RefCell::new(4)), Rc::new(RefCell::new(2))),
                    (Rc::new(RefCell::new(5)), Rc::new(RefCell::new(1))),
                    (Rc::new(RefCell::new(6)), Rc::new(RefCell::new(0))),
                ])));
            let copy: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            copy.as_pointer().write((*orig.borrow()).clone());
            assert!(({ ((*copy.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    copy.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                    3_isize
                ) == copy
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
            assert!(({ ((*orig.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    orig.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                    3_isize
                ) == orig
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
        }
        ({ AssertEmpty_14Impl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let lhs_arr: Value<Box<[(Value<i32>, Value<i32>)]>> =
                Rc::new(RefCell::new(Box::new([
                    (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(1))),
                    (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(3))),
                    (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(6))),
                ])));
            let orig: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            let rhs_arr: Value<Box<[(Value<i32>, Value<i32>)]>> =
                Rc::new(RefCell::new(Box::new([
                    (Rc::new(RefCell::new(4)), Rc::new(RefCell::new(2))),
                    (Rc::new(RefCell::new(5)), Rc::new(RefCell::new(1))),
                    (Rc::new(RefCell::new(6)), Rc::new(RefCell::new(0))),
                    (Rc::new(RefCell::new(7)), Rc::new(RefCell::new(9))),
                ])));
            let copy: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            copy.as_pointer().write((*orig.borrow()).clone());
            assert!(({ ((*copy.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    copy.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                    3_isize
                ) == copy
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
            assert!(({ ((*orig.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    orig.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                    3_isize
                ) == orig
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            );
        }
        ({ AssertEmpty_14Impl::operator_call(&check_alloc_invariant.as_pointer()) });
    }
    {
        let orig: Value<Map<i32, (Value<i32>, Value<i32>)>> =
            Rc::new(RefCell::new(unimplemented!()));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) != 50) {
            let __rhs = ((*i.borrow()) + 3);
            Map::subscript(orig.as_pointer(), i.as_pointer()).write(__rhs);
            (*i.borrow_mut()).prefix_inc();
        }
        let copy: Value<Map<i32, (Value<i32>, Value<i32>)>> =
            Rc::new(RefCell::new(unimplemented!()));
        copy.as_pointer().write((*orig.borrow()).clone());
        let mut i: i32 = 0;
        'loop_: for v in IterRange::new(
            copy.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
            copy.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end()),
        ) {
            let v: Value<(Value<i32>, Value<i32>)> =
                Rc::new(RefCell::new(match (v.iter_deref(),) {
                    (__tmp_2,) => (
                        Rc::new(RefCell::new(
                            ((*__tmp_2.upgrade().deref()).0.as_pointer().read()),
                        )),
                        Rc::new(RefCell::new(
                            ((*__tmp_2.upgrade().deref()).1.as_pointer().read()),
                        )),
                    ),
                }));
            assert!(unimplemented!());
            i.prefix_inc();
        }
    }
    ({ AssertEmpty_14Impl::operator_call(&check_alloc_invariant.as_pointer()) });
    return true;
}
pub fn test_15() -> bool {
    ({ test_alloc_11(None, None, None) });
    ({ test_alloc_12(None, None, None) });
    {
        let lhs_allocs: Value<Vec<AnyPtr>> = Rc::new(RefCell::new(Vec::new()));
        let rhs_allocs: Value<Vec<AnyPtr>> = Rc::new(RefCell::new(Vec::new()));
        ({
            let _lhs_alloc: Value<tracking_allocator_std_pair_const_int__int__> =
                Rc::new(RefCell::new(
                    tracking_allocator_std_pair_const_int__int__::new({ lhs_allocs.as_pointer() }),
                ));
            let _rhs_alloc: Value<tracking_allocator_std_pair_const_int__int__> =
                Rc::new(RefCell::new(
                    tracking_allocator_std_pair_const_int__int__::new({ rhs_allocs.as_pointer() }),
                ));
            let _check_alloc_invariant: AssertEmpty_14 =
                AssertEmpty_14::new({ lhs_allocs.as_pointer() }, { rhs_allocs.as_pointer() });
            test_alloc_13(
                _lhs_alloc.as_pointer(),
                _rhs_alloc.as_pointer(),
                _check_alloc_invariant,
            )
        });
    }
    {
        {
            let arr: Value<Box<[(Value<NonTrivial>, Value<NonTrivial>)]>> =
                Rc::new(RefCell::new(Box::new([
                    (
                        Rc::new(RefCell::new(NonTrivial::new({ 1 }))),
                        Rc::new(RefCell::new(NonTrivial::new({ 1 }))),
                    ),
                    (
                        Rc::new(RefCell::new(NonTrivial::new({ 2 }))),
                        Rc::new(RefCell::new(NonTrivial::new({ 3 }))),
                    ),
                    (
                        Rc::new(RefCell::new(NonTrivial::new({ 3 }))),
                        Rc::new(RefCell::new(NonTrivial::new({ 6 }))),
                    ),
                ])));
            let orig: Value<Map<NonTrivial, (Value<NonTrivial>, Value<NonTrivial>)>> =
                Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<Map<NonTrivial, (Value<NonTrivial>, Value<NonTrivial>)>> =
                Rc::new(RefCell::new(Map::new()));
            copy.as_pointer().write((*orig.borrow()).clone());
            assert!(({ ((*copy.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    copy.as_pointer().with(
                        |__m: &Map<NonTrivial, (Value<NonTrivial>, Value<NonTrivial>)>| __m.begin()
                    ),
                    3_isize
                ) == copy.as_pointer().with(
                    |__m: &Map<NonTrivial, (Value<NonTrivial>, Value<NonTrivial>)>| __m.end()
                )
            );
            assert!(({ ((*orig.borrow()).len() == 3_usize) }));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(
                libcc2rs::next_refcount(
                    orig.as_pointer().with(
                        |__m: &Map<NonTrivial, (Value<NonTrivial>, Value<NonTrivial>)>| __m.begin()
                    ),
                    3_isize
                ) == orig.as_pointer().with(
                    |__m: &Map<NonTrivial, (Value<NonTrivial>, Value<NonTrivial>)>| __m.end()
                )
            );
        }
    }
    {
        let arr: Value<Box<[(Value<i32>, Value<i32>)]>> = Rc::new(RefCell::new(Box::new([
            (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(1))),
            (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(3))),
            (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(6))),
        ])));
        let orig: Value<Map<i32, (Value<i32>, Value<i32>)>> =
            Rc::new(RefCell::new(unimplemented!()));
        let copy: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
        copy.as_pointer().write((*orig.borrow()).clone());
        assert!(({ ((*copy.borrow()).len() == 3_usize) }));
        assert!(
            ({
                let _a: Value<test_less_int_> = Rc::new(RefCell::new(unimplemented!()));
                let _b: Value<test_less_int_> = Rc::new(RefCell::new(test_less_int_::new_2({ 3 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(({ ((*orig.borrow()).len() == 3_usize) }));
        assert!(
            ({
                let _a: Value<test_less_int_> = Rc::new(RefCell::new(unimplemented!()));
                let _b: Value<test_less_int_> = Rc::new(RefCell::new(test_less_int_::new_2({ 3 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
    }
    {
        let in_: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) != 32) {
            let __rhs = (*i.borrow());
            Map::subscript(in_.as_pointer(), i.as_pointer()).write(__rhs);
            (*i.borrow_mut()).prefix_inc();
        }
        let out: Value<Map<i32, (Value<i32>, Value<i32>)>> =
            Rc::new(RefCell::new((*in_.borrow()).clone()));
        unimplemented!();
        out.as_pointer().write((*in_.borrow()).clone());
        unimplemented!();
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct AssertEmpty_14 {
    #[offset(0)]
    #[byte_size(8)]
    lhs_allocs_: Ptr<Vec<AnyPtr>>,
    #[offset(8)]
    #[byte_size(8)]
    rhs_allocs_: Ptr<Vec<AnyPtr>>,
}
impl AssertEmpty_14 {
    pub fn new(lhs_allocs: Ptr<Vec<AnyPtr>>, rhs_allocs: Ptr<Vec<AnyPtr>>) -> Self {
        Self {
            lhs_allocs_: (lhs_allocs).clone(),
            rhs_allocs_: (rhs_allocs).clone(),
        }
    }
}
impl Callable0<()> for AssertEmpty_14 {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            AssertEmpty_14Impl::operator_call(__it)
        })
    }
}
impl CppMove for AssertEmpty_14 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
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
    ({ test_15() });
    return 0;
}
pub trait AssertEmpty_14Impl {
    fn operator_call(&self);
}
impl AssertEmpty_14Impl for Ptr<AssertEmpty_14> {
    fn operator_call(&self) {
        assert!(
            ({
                (*(*self)
                    .with(|__s| __s.lhs_allocs_.clone())
                    .upgrade()
                    .deref())
                .is_empty()
            })
        );
        assert!(
            ({
                (*(*self)
                    .with(|__s| __s.rhs_allocs_.clone())
                    .upgrade()
                    .deref())
                .is_empty()
            })
        );
    }
}
pub trait NoOpImpl {
    fn operator_call(&self);
}
impl NoOpImpl for Ptr<NoOp> {
    fn operator_call(&self) {}
}
pub trait NonTrivialImpl {
    fn copy_assign(&self, other: Ptr<NonTrivial>) -> Ptr<NonTrivial>;
    fn move_assign(&self, other: Ptr<NonTrivial>) -> Ptr<NonTrivial>;
}
impl NonTrivialImpl for Ptr<NonTrivial> {
    fn copy_assign(&self, other: Ptr<NonTrivial>) -> Ptr<NonTrivial> {
        if ((*self) != (other)) {
            field!((*self), i).write({ other.with(|__s| __s.i) });
        }
        return (*self).clone();
    }
    fn move_assign(&self, other: Ptr<NonTrivial>) -> Ptr<NonTrivial> {
        field!((*self), i).write({ other.with(|__s| __s.i) });
        field!(other, i).write(0);
        return (*self).clone();
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
pub trait min_allocator_std_pair_const_int__int__Impl {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_std_pair_const_int__int__Impl for Ptr<min_allocator_std_pair_const_int__int__> {}
pub trait min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<(Value<i32>, Value<i32>)> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<(Value<i32>, Value<i32>)> {
        unimplemented!()
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: i32,
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: i32,
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<(Value<i32>, Value<i32>)> {
        unimplemented!()
    }
}
impl min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__>
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
pub trait test_less_int_Impl {
    fn operator_call(&self, a: Ptr<i32>, b: Ptr<i32>) -> bool;
}
impl test_less_int_Impl for Ptr<test_less_int_> {
    fn operator_call(&self, a: Ptr<i32>, b: Ptr<i32>) -> bool {
        return ({ (a.read()) } < { (b.read()) });
    }
}
pub trait tracking_allocator_std_pair_const_int__int__Impl {
    fn allocate(&self, mut n: usize) -> Ptr<(Value<i32>, Value<i32>)> {
        unimplemented!()
    }
    fn deallocate(&self, mut ptr: Ptr<(Value<i32>, Value<i32>)>, mut n: usize) {
        unimplemented!()
    }
}
impl tracking_allocator_std_pair_const_int__int__Impl
    for Ptr<tracking_allocator_std_pair_const_int__int__>
{
}
pub fn __cpp2rust_init_globals() {
    let _ = destructed_value_1.with(|_| ());
    let _ = moved_value_2.with(|_| ());
    let _ = outstanding_bytes_5.with(|_| ());
    let _ = alloc_count_6.with(|_| ());
    let _ = dealloc_count_7.with(|_| ());
    let _ = disable_default_constructor_8.with(|_| ());
}
