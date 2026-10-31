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
pub fn operator_eq_1(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_2(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_3(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_4(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_5(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_6(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_7(_a0: Ptr<MoveOnly>, _a1: Ptr<MoveOnly>) -> std::cmp::Ordering {
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
        operator_lt_3(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for MoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_1(__lhs.clone(), __rhs.clone())
    }
}
impl Default for MoveOnly {
    fn default() -> Self {
        { MoveOnly::new(None) }
    }
}
pub fn operator_eq_8(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_9(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_10(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_11(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_12(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_13(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_14(_a0: Ptr<TrivialMoveOnly>, _a1: Ptr<TrivialMoveOnly>) -> std::cmp::Ordering {
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
        operator_lt_10(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for TrivialMoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_8(__lhs.clone(), __rhs.clone())
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
            (alloc_count_16.with(|rc| *rc.borrow()) >= dealloc_count_17.with(|rc| *rc.borrow()))
        );
        return ((alloc_count_16.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_17.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_18.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_15.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_16.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_17.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
thread_local!(
    pub static outstanding_bytes_15: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_16: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_17: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_18: Value<bool> = Rc::new(RefCell::new(false));
);
pub fn operator_sub_19(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_eq_20(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_21(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !({
        let _x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_20(_x, y)
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
        operator_eq_20(__lhs.read(), __rhs.read())
    }
}
impl CppAdvance for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_8();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_10();
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
pub struct min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<MoveOnly>,
}
impl min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {}
pub fn operator_sub_22(
    mut x : min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_lt_23(
    mut x : min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } < { (y.ptr_).clone() });
}
pub fn operator_eq_24(
    mut x : min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_25(
    mut x : min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return !({
        let _x: min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__   = x   ;
        operator_eq_24(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__>,
}
impl min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn new_1 ( mut p : Ptr< min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__  >  , ) -> Self { Self { ptr_ : (p ).clone() , }  } }
impl CppLess for min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool { operator_lt_23(__lhs.read(), __rhs.read()) } }
impl CppEq for min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool { operator_eq_24(__lhs.read(), __rhs.read()) } }
impl CppAdvance for min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn cpp_inc(__it: &Ptr<Self>) { __it.operator_inc_8(); } fn cpp_dec(__it: &Ptr<Self>) { __it.operator_dec_10(); } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_MoveOnly_ {}
impl min_allocator_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0___ {
    pub fn new_1(mut _a0: min_allocator_MoveOnly_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(min_allocator_MoveOnly_, new_1, value)]
#[byte_size(1)]
pub struct min_allocator_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0___ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct safe_allocator_MoveOnly_ {}
impl safe_allocator_MoveOnly_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for safe_allocator_MoveOnly_ {
    fn default() -> Self {
        { safe_allocator_MoveOnly_::new() }
    }
}
impl safe_allocator_MoveOnly_ptr_ {
    pub fn new_1(mut _a0: safe_allocator_MoveOnly_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct safe_allocator_MoveOnly_ptr_ {}
impl safe_allocator_MoveOnly_ptr_ {}
pub fn make_26(mut size: i32, mut start: i32) -> Vec<MoveOnly> {
    let mut b: i32 = (((4096_usize).wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
    let mut init: i32 = 0;
    if (start > 0) {
        init = (((start + 1) / b) + ((((start + 1) % b) != 0) as i32));
        init *= b;
        init.prefix_dec();
    }
    let c: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: i32 = 0;
    'loop_: while (i < (init - start)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < size) {
        (*c.borrow_mut()).push(MoveOnly::new({ Some(i) }));
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < start) {
        (*c.borrow_mut()).remove(0);
        i.prefix_inc();
    }
    return std::mem::take(&mut (*c.borrow_mut()));
}
pub fn make_27(mut size: i32, mut start: i32) -> Vec<MoveOnly> {
    let mut b: i32 = (((4096_usize).wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
    let mut init: i32 = 0;
    if (start > 0) {
        init = (((start + 1) / b) + ((((start + 1) % b) != 0) as i32));
        init *= b;
        init.prefix_dec();
    }
    let c: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: i32 = 0;
    'loop_: while (i < (init - start)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < size) {
        (*c.borrow_mut()).push(MoveOnly::new({ Some(i) }));
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < start) {
        (*c.borrow_mut()).remove(0);
        i.prefix_inc();
    }
    return std::mem::take(&mut (*c.borrow_mut()));
}
pub fn make_28(mut size: i32, mut start: i32) -> Vec<MoveOnly> {
    let mut b: i32 = (((4096_usize).wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
    let mut init: i32 = 0;
    if (start > 0) {
        init = (((start + 1) / b) + ((((start + 1) % b) != 0) as i32));
        init *= b;
        init.prefix_dec();
    }
    let c: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: i32 = 0;
    'loop_: while (i < (init - start)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < size) {
        (*c.borrow_mut()).push(MoveOnly::new({ Some(i) }));
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < start) {
        (*c.borrow_mut()).remove(0);
        i.prefix_inc();
    }
    return std::mem::take(&mut (*c.borrow_mut()));
}
pub fn test_29(mut P: i32, c1: Ptr<Vec<MoveOnly>>, mut x: i32) {
    let mut c1_osize: usize = unimplemented!();
    let i: Value<()> = Rc::new(RefCell::new(<()>::from(unimplemented!())));
    assert!((*i.borrow()) == unimplemented!().iter_add((P as isize)));
    assert!(({ unimplemented!() } == { (c1_osize).wrapping_add(1_usize) }));
    assert!(({ (unimplemented!() as usize) } == { unimplemented!() }));
    const _: () = assert!(true, "");;
    (*i.borrow_mut()) = <()>::from(unimplemented!()).clone();
    let mut j: i32 = 0;
    'loop_: while (j < P) {
        assert!(
            ({
                let _x: Ptr<MoveOnly> = (*i.borrow()).iter_deref();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(j) })));
                operator_eq_1(_x, _y.as_pointer())
            })
        );
        {
            j.prefix_inc();
            &((*i.borrow_mut()).iter_inc())
        };
    }
    assert!(
        ({
            let _x: Ptr<MoveOnly> = (*i.borrow()).iter_deref();
            let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(x) })));
            operator_eq_1(_x, _y.as_pointer())
        })
    );
    (*i.borrow_mut()).iter_inc();
    let mut j: i32 = P;
    'loop_: while ((j as usize) < c1_osize) {
        assert!(
            ({
                let _x: Ptr<MoveOnly> = (*i.borrow()).iter_deref();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(j) })));
                operator_eq_1(_x, _y.as_pointer())
            })
        );
        {
            j.prefix_inc();
            &((*i.borrow_mut()).iter_inc())
        };
    }
}
pub fn test_30(mut P: i32, c1: Ptr<Vec<MoveOnly>>, mut x: i32) {
    let mut c1_osize: usize = unimplemented!();
    let i: Value<()> = Rc::new(RefCell::new(<()>::from(unimplemented!())));
    assert!((*i.borrow()) == unimplemented!().iter_add((P as isize)));
    assert!(({ unimplemented!() } == { (c1_osize).wrapping_add(1_usize) }));
    assert!(({ (unimplemented!() as usize) } == { unimplemented!() }));
    const _: () = assert!(true, "");;
    (*i.borrow_mut()) = <()>::from(unimplemented!()).clone();
    let mut j: i32 = 0;
    'loop_: while (j < P) {
        assert!(
            ({
                let _x: Ptr<MoveOnly> = (*i.borrow()).iter_deref();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(j) })));
                operator_eq_1(_x, _y.as_pointer())
            })
        );
        {
            j.prefix_inc();
            &((*i.borrow_mut()).iter_inc())
        };
    }
    assert!(
        ({
            let _x: Ptr<MoveOnly> = (*i.borrow()).iter_deref();
            let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(x) })));
            operator_eq_1(_x, _y.as_pointer())
        })
    );
    (*i.borrow_mut()).iter_inc();
    let mut j: i32 = P;
    'loop_: while ((j as usize) < c1_osize) {
        assert!(
            ({
                let _x: Ptr<MoveOnly> = (*i.borrow()).iter_deref();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(j) })));
                operator_eq_1(_x, _y.as_pointer())
            })
        );
        {
            j.prefix_inc();
            &((*i.borrow_mut()).iter_inc())
        };
    }
}
pub fn test_31(mut P: i32, c1: Ptr<Vec<MoveOnly>>, mut x: i32) {
    let mut c1_osize: usize = unimplemented!();
    let i: Value<()> = Rc::new(RefCell::new(<()>::from(unimplemented!())));
    assert!((*i.borrow()) == unimplemented!().iter_add((P as isize)));
    assert!(({ unimplemented!() } == { (c1_osize).wrapping_add(1_usize) }));
    assert!(({ (unimplemented!() as usize) } == { unimplemented!() }));
    const _: () = assert!(true, "");;
    (*i.borrow_mut()) = <()>::from(unimplemented!()).clone();
    let mut j: i32 = 0;
    'loop_: while (j < P) {
        assert!(
            ({
                let _x: Ptr<MoveOnly> = (*i.borrow()).iter_deref();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(j) })));
                operator_eq_1(_x, _y.as_pointer())
            })
        );
        {
            j.prefix_inc();
            &((*i.borrow_mut()).iter_inc())
        };
    }
    assert!(
        ({
            let _x: Ptr<MoveOnly> = (*i.borrow()).iter_deref();
            let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(x) })));
            operator_eq_1(_x, _y.as_pointer())
        })
    );
    (*i.borrow_mut()).iter_inc();
    let mut j: i32 = P;
    'loop_: while ((j as usize) < c1_osize) {
        assert!(
            ({
                let _x: Ptr<MoveOnly> = (*i.borrow()).iter_deref();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(j) })));
                operator_eq_1(_x, _y.as_pointer())
            })
        );
        {
            j.prefix_inc();
            &((*i.borrow_mut()).iter_inc())
        };
    }
}
pub fn testN_32(mut start: i32, mut N: i32) {
    let mut i: i32 = 0;
    'loop_: while (i <= 3) {
        if (0 <= i) && (i <= N) {
            let c1: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(({ make_26(N, start) })));
            ({ test_29(i, c1.as_pointer(), -10_i32) });
        }
        i.prefix_inc();
    }
    let mut i: i32 = ((N / 2) - 1);
    'loop_: while (i <= ((N / 2) + 1)) {
        if (0 <= i) && (i <= N) {
            let c1: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(({ make_26(N, start) })));
            ({ test_29(i, c1.as_pointer(), -10_i32) });
        }
        i.prefix_inc();
    }
    let mut i: i32 = (N - 3);
    'loop_: while (i <= N) {
        if (0 <= i) && (i <= N) {
            let c1: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(({ make_26(N, start) })));
            ({ test_29(i, c1.as_pointer(), -10_i32) });
        }
        i.prefix_inc();
    }
}
pub fn testN_33(mut start: i32, mut N: i32) {
    let mut i: i32 = 0;
    'loop_: while (i <= 3) {
        if (0 <= i) && (i <= N) {
            let c1: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(({ make_27(N, start) })));
            ({ test_30(i, c1.as_pointer(), -10_i32) });
        }
        i.prefix_inc();
    }
    let mut i: i32 = ((N / 2) - 1);
    'loop_: while (i <= ((N / 2) + 1)) {
        if (0 <= i) && (i <= N) {
            let c1: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(({ make_27(N, start) })));
            ({ test_30(i, c1.as_pointer(), -10_i32) });
        }
        i.prefix_inc();
    }
    let mut i: i32 = (N - 3);
    'loop_: while (i <= N) {
        if (0 <= i) && (i <= N) {
            let c1: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(({ make_27(N, start) })));
            ({ test_30(i, c1.as_pointer(), -10_i32) });
        }
        i.prefix_inc();
    }
}
pub fn testN_34(mut start: i32, mut N: i32) {
    let mut i: i32 = 0;
    'loop_: while (i <= 3) {
        if (0 <= i) && (i <= N) {
            let c1: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(({ make_28(N, start) })));
            ({ test_31(i, c1.as_pointer(), -10_i32) });
        }
        i.prefix_inc();
    }
    let mut i: i32 = ((N / 2) - 1);
    'loop_: while (i <= ((N / 2) + 1)) {
        if (0 <= i) && (i <= N) {
            let c1: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(({ make_28(N, start) })));
            ({ test_31(i, c1.as_pointer(), -10_i32) });
        }
        i.prefix_inc();
    }
    let mut i: i32 = (N - 3);
    'loop_: while (i <= N) {
        if (0 <= i) && (i <= N) {
            let c1: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(({ make_28(N, start) })));
            ({ test_31(i, c1.as_pointer(), -10_i32) });
        }
        i.prefix_inc();
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
    {
        let mut rng: [i32; 10] = [0, 1, 2, 3, 1023, 1024, 1025, 2047, 2048, 2049];
        let mut N: i32 = (((::std::mem::size_of::<[i32; 10]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            let mut j: i32 = 0;
            'loop_: while (j < N) {
                ({
                    let _start: i32 = rng[(i) as usize];
                    let _N: i32 = rng[(j) as usize];
                    testN_32(_start, _N)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let mut rng: [i32; 10] = [0, 1, 2, 3, 1023, 1024, 1025, 2047, 2048, 2049];
        let mut N: i32 = (((::std::mem::size_of::<[i32; 10]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            let mut j: i32 = 0;
            'loop_: while (j < N) {
                ({
                    let _start: i32 = rng[(i) as usize];
                    let _N: i32 = rng[(j) as usize];
                    testN_33(_start, _N)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let mut rng: [i32; 10] = [0, 1, 2, 3, 1023, 1024, 1025, 2047, 2048, 2049];
        let mut N: i32 = (((::std::mem::size_of::<[i32; 10]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            let mut j: i32 = 0;
            'loop_: while (j < N) {
                ({
                    let _start: i32 = rng[(i) as usize];
                    let _N: i32 = rng[(j) as usize];
                    testN_34(_start, _N)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
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
pub trait min_allocator_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0___Impl {
    fn allocate ( &self , n : usize  , ) -> min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn deallocate(
        &self,
        p : min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        n: usize,
    );
}
impl min_allocator_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0___Impl
    for Ptr<min_allocator_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0___>
{
    fn allocate ( &self , mut n : usize  , ) -> min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()   } , )    ;
    }
    fn deallocate(
        &self,
        mut p : min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
pub trait min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
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
    ) -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__>;
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
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
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
pub trait min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl
{
    fn to_bool(&self) -> bool;
    fn operator_deref(&self)
    -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__>;
    fn operator_arrow(
        &self,
    ) -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_post_inc_9 ( &self , mut _a0 : i32  , ) -> min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_post_dec_11 ( &self , mut _a0 : i32  , ) -> min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_add_assign ( &self , n : isize  , ) -> Ptr< min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_sub_assign ( &self , n : isize  , ) -> Ptr< min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_add ( &self , n : isize  , ) -> min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn operator_sub ( &self , n : isize  , ) -> min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn operator_index(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
}
impl min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__> {
fn to_bool ( &self , ) -> bool  { return ! ( ( (* self ) .with(|__s| __s . ptr_ .clone()) ).is_null() )   ;
 } fn operator_deref ( &self , ) -> Ptr< min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__  > { return (* self ) .with(|__s| __s . ptr_ .clone())    ;
 } fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { field!((* self ) , ptr_) .with_mut(|__v| __v. prefix_inc ()) ;
 return ((* self )  ).clone()  ;
 } fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { field!((* self ) , ptr_) .with_mut(|__v| __v. prefix_dec ()) ;
 return ((* self )  ).clone()  ;
 } fn operator_add_assign ( &self , mut n : isize  , ) -> Ptr< min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { { let _ptr = field!((* self ) , ptr_); _ptr.write(_ptr.read() + n ) } ;
 return ((* self )  ).clone()  ;
 } fn operator_sub_assign ( &self , mut n : isize  , ) -> Ptr< min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { { let _ptr = field!((* self ) , ptr_); _ptr.write(_ptr.read() - n ) } ;
 return ((* self )  ).clone()  ;
 } fn operator_add ( &self , mut n : isize  , ) -> min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  { let tmp : Value<min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ > = Rc::new(RefCell::new((( * (* self ) .upgrade().deref() )  ).clone() )) ;
  ;
 ;
 ( { min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl :: operator_add_assign ( &tmp .as_pointer()  , n  , ) } )  ;
 return ((*tmp.borrow()) ).clone()   ;
 } fn operator_sub ( &self , mut n : isize  , ) -> min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  { let tmp : Value<min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ > = Rc::new(RefCell::new((( * (* self ) .upgrade().deref() )  ).clone() )) ;
  ;
 ;
 ( { min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl :: operator_sub_assign ( &tmp .as_pointer()  , n  , ) } )  ;
 return ((*tmp.borrow()) ).clone()   ;
 } }
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait safe_allocator_MoveOnly_Impl {
    fn allocate(&self, n: usize) -> Ptr<MoveOnly>;
    fn deallocate(&self, p: Ptr<MoveOnly>, n: usize);
}
impl safe_allocator_MoveOnly_Impl for Ptr<safe_allocator_MoveOnly_> {
    fn allocate(&self, mut n: usize) -> Ptr<MoveOnly> {
        let memory: Value<Ptr<MoveOnly>> = Rc::new(RefCell::new(unimplemented!()));
        {
            {
                (*memory.borrow()).to_any().memset(
                    (0) as u8,
                    ((4usize as u64).wrapping_mul((n as u64)) as usize) as usize,
                );
                (*memory.borrow()).to_any()
            };
        }
        return (*memory.borrow()).clone();
    }
    fn deallocate(&self, mut p: Ptr<MoveOnly>, mut n: usize) {
        {
            ({
                let _value: Value<AnyPtr> = Rc::new(RefCell::new({
                    (p).to_any().memset(
                        (0) as u8,
                        ((4usize as u64).wrapping_mul((n as u64)) as usize) as usize,
                    );
                    (p).to_any()
                }));
                DoNotOptimize_0(_value.as_pointer())
            });
        }
        unimplemented!();
    }
}
pub trait safe_allocator_MoveOnly_ptr_Impl {
    fn allocate(&self, n: usize) -> Ptr<Ptr<MoveOnly>>;
    fn deallocate(&self, p: Ptr<Ptr<MoveOnly>>, n: usize);
}
impl safe_allocator_MoveOnly_ptr_Impl for Ptr<safe_allocator_MoveOnly_ptr_> {
    fn allocate(&self, mut n: usize) -> Ptr<Ptr<MoveOnly>> {
        let memory: Value<Ptr<Ptr<MoveOnly>>> = Rc::new(RefCell::new(unimplemented!()));
        {
            {
                (*memory.borrow()).to_any().memset(
                    (0) as u8,
                    ((8usize as u64).wrapping_mul((n as u64)) as usize) as usize,
                );
                (*memory.borrow()).to_any()
            };
        }
        return (*memory.borrow()).clone();
    }
    fn deallocate(&self, mut p: Ptr<Ptr<MoveOnly>>, mut n: usize) {
        {
            ({
                let _value: Value<AnyPtr> = Rc::new(RefCell::new({
                    (p).to_any().memset(
                        (0) as u8,
                        ((8usize as u64).wrapping_mul((n as u64)) as usize) as usize,
                    );
                    (p).to_any()
                }));
                DoNotOptimize_0(_value.as_pointer())
            });
        }
        unimplemented!();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_15.with(|_| ());
    let _ = alloc_count_16.with(|_| ());
    let _ = dealloc_count_17.with(|_| ());
    let _ = disable_default_constructor_18.with(|_| ());
}
