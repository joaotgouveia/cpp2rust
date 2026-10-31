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
    pub static destructed_value_0: Value<i32> = Rc::new(RefCell::new(-1_i32));
);
thread_local!(
    pub static moved_value_1: Value<i32> = Rc::new(RefCell::new(2147483647));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_2(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_3(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
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
        operator_eq_2(__lhs.clone(), __rhs.clone())
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
pub fn operator_eq_5(x: Ptr<DefaultOnly>, y: Ptr<DefaultOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_lt_6(x: Ptr<DefaultOnly>, y: Ptr<DefaultOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(4)]
pub struct DefaultOnly {
    #[offset(0)]
    data_: i32,
}
impl DefaultOnly {
    pub fn new() -> Self {
        let __this: DefaultOnly = Self { data_: -1_i32 };
        (*count_4.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl CppLess for DefaultOnly {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_6(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for DefaultOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_5(__lhs.clone(), __rhs.clone())
    }
}
impl Default for DefaultOnly {
    fn default() -> Self {
        { DefaultOnly::new() }
    }
}
thread_local!(
    pub static count_4: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_8.with(|rc| *rc.borrow()) >= dealloc_count_9.with(|rc| *rc.borrow())));
        return ((alloc_count_8.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_9.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_10.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_7.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_8.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_9.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
thread_local!(
    pub static outstanding_bytes_7: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_8: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_9: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_10: Value<bool> = Rc::new(RefCell::new(false));
);
pub fn operator_sub_11(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
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
impl CppEq for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_12(__lhs.read(), __rhs.read())
    }
}
impl CppAdvance for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_8();
    }
}
pub fn operator_sub_14(
    mut x: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_eq_15(
    mut x: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_16(
    mut x: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !({
        let _x: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_15(_x, y)
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
}
impl CppEq for min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_15(__lhs.read(), __rhs.read())
    }
}
impl CppAdvance for min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {
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
pub struct min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<DefaultOnly>,
}
impl min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__ {}
pub fn operator_sub_17(
    mut x : min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_lt_18(
    mut x : min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } < { (y.ptr_).clone() });
}
pub fn operator_eq_19(
    mut x : min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_20(
    mut x : min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return !({
        let _x: min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__   = x   ;
        operator_eq_19(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__>,
}
impl min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn new_1 ( mut p : Ptr< min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__  >  , ) -> Self { Self { ptr_ : (p ).clone() , }  } }
impl CppLess for min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool { operator_lt_18(__lhs.read(), __rhs.read()) } }
impl CppEq for min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool { operator_eq_19(__lhs.read(), __rhs.read()) } }
impl CppAdvance for min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn cpp_inc(__it: &Ptr<Self>) { __it.operator_inc_8(); } }
pub fn operator_sub_21(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_lt_22(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } < { (y.ptr_).clone() });
}
pub fn operator_eq_23(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_24(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return !({
        let _x: min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__   = x   ;
        operator_eq_23(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>,
}
impl min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn new_1 ( mut p : Ptr< min_pointer_int__std_integral_constant_unsigned_long__0__  >  , ) -> Self { Self { ptr_ : (p ).clone() , }  } }
impl CppLess for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool { operator_lt_22(__lhs.read(), __rhs.read()) } }
impl CppEq for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool { operator_eq_23(__lhs.read(), __rhs.read()) } }
impl CppAdvance for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn cpp_inc(__it: &Ptr<Self>) { __it.operator_inc_8(); } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_DefaultOnly_ {}
pub fn operator_eq_25(mut _a0: min_allocator_int_, mut _a1: min_allocator_int_) -> bool {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
impl CppEq for min_allocator_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_25(__lhs.read(), __rhs.read())
    }
}
impl min_allocator_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0___ {
    pub fn new_1(mut _a0: min_allocator_DefaultOnly_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(min_allocator_DefaultOnly_, new_1, value)]
#[byte_size(1)]
pub struct min_allocator_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
    pub fn new_1(mut _a0: min_allocator_int_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(min_allocator_int_, new_1, value)]
#[byte_size(1)]
pub struct min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
pub fn test2_26(mut n: u32) {
    assert!((count_4.with(|rc| *rc.borrow()) == 0));
    {
        let d: Value<Vec<DefaultOnly>> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((count_4.with(|rc| *rc.borrow()) as u32) == n));
        assert!((unimplemented!() == (n as usize)));
        assert!(((unimplemented!() as usize) == unimplemented!()));
        const _: () = assert!(true, "");;
        let i: Value<()> = Rc::new(RefCell::new(<()>::from(unimplemented!())));
        let e: Value<()> = Rc::new(RefCell::new(<()>::from(unimplemented!())));
        'loop_: while (*i.borrow()) != (*e.borrow()) {
            assert!(
                ({
                    let _x: Ptr<DefaultOnly> = (*i.borrow()).iter_deref();
                    let _y: Value<DefaultOnly> = Rc::new(RefCell::new(DefaultOnly::new()));
                    operator_eq_5(_x, _y.as_pointer())
                })
            );
            (*i.borrow_mut()).iter_inc();
        }
    }
    assert!((count_4.with(|rc| *rc.borrow()) == 0));
}
pub fn test2_27(mut n: u32) {
    assert!((count_4.with(|rc| *rc.borrow()) == 0));
    {
        let d: Value<Vec<DefaultOnly>> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((count_4.with(|rc| *rc.borrow()) as u32) == n));
        assert!((unimplemented!() == (n as usize)));
        assert!(((unimplemented!() as usize) == unimplemented!()));
        const _: () = assert!(true, "");;
        let i: Value<()> = Rc::new(RefCell::new(<()>::from(unimplemented!())));
        let e: Value<()> = Rc::new(RefCell::new(<()>::from(unimplemented!())));
        'loop_: while (*i.borrow()) != (*e.borrow()) {
            assert!(
                ({
                    let _x: Ptr<DefaultOnly> = (*i.borrow()).iter_deref();
                    let _y: Value<DefaultOnly> = Rc::new(RefCell::new(DefaultOnly::new()));
                    operator_eq_5(_x, _y.as_pointer())
                })
            );
            (*i.borrow_mut()).iter_inc();
        }
    }
    assert!((count_4.with(|rc| *rc.borrow()) == 0));
}
pub fn test1_28(mut n: u32) {
    assert!((count_4.with(|rc| *rc.borrow()) == 0));
    {
        let d: Value<Vec<DefaultOnly>> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((count_4.with(|rc| *rc.borrow()) as u32) == n));
        assert!((unimplemented!() == (n as usize)));
        assert!(((unimplemented!() as usize) == unimplemented!()));
        const _: () = assert!(true, "");;
        let i: Value<()> = Rc::new(RefCell::new(<()>::from(unimplemented!())));
        let e: Value<()> = Rc::new(RefCell::new(<()>::from(unimplemented!())));
        'loop_: while (*i.borrow()) != (*e.borrow()) {
            assert!(
                ({
                    let _x: Ptr<DefaultOnly> = (*i.borrow()).iter_deref();
                    let _y: Value<DefaultOnly> = Rc::new(RefCell::new(DefaultOnly::new()));
                    operator_eq_5(_x, _y.as_pointer())
                })
            );
            (*i.borrow_mut()).iter_inc();
        }
    }
    assert!((count_4.with(|rc| *rc.borrow()) == 0));
}
pub fn test1_29(mut n: u32) {
    assert!((count_4.with(|rc| *rc.borrow()) == 0));
    {
        let d: Value<Vec<DefaultOnly>> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((count_4.with(|rc| *rc.borrow()) as u32) == n));
        assert!((unimplemented!() == (n as usize)));
        assert!(((unimplemented!() as usize) == unimplemented!()));
        const _: () = assert!(true, "");;
        let i: Value<()> = Rc::new(RefCell::new(<()>::from(unimplemented!())));
        let e: Value<()> = Rc::new(RefCell::new(<()>::from(unimplemented!())));
        'loop_: while (*i.borrow()) != (*e.borrow()) {
            assert!(
                ({
                    let _x: Ptr<DefaultOnly> = (*i.borrow()).iter_deref();
                    let _y: Value<DefaultOnly> = Rc::new(RefCell::new(DefaultOnly::new()));
                    operator_eq_5(_x, _y.as_pointer())
                })
            );
            (*i.borrow_mut()).iter_inc();
        }
    }
    assert!((count_4.with(|rc| *rc.borrow()) == 0));
}
pub fn test3_30(mut n: u32, alloc: Option<Ptr<()>>) {
    let mut __alloc_default: Option<Value<()>> = None;
    let alloc: Ptr<()> = alloc.unwrap_or_else(|| {
        __alloc_default
            .insert(Rc::new(RefCell::new(unimplemented!())))
            .as_pointer()
    });
    {
        let d: Value<Vec<DefaultOnly>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == (n as usize)));
        assert!(unimplemented!());
        const _: () = assert!(true, "");;
    }
}
pub fn test3_31(mut n: u32, alloc: Option<Ptr<()>>) {
    let mut __alloc_default: Option<Value<()>> = None;
    let alloc: Ptr<()> = alloc.unwrap_or_else(|| {
        __alloc_default
            .insert(Rc::new(RefCell::new(unimplemented!())))
            .as_pointer()
    });
    {
        let d: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == (n as usize)));
        assert!(unimplemented!());
        const _: () = assert!(true, "");;
    }
}
pub fn test3_32(mut n: u32, alloc: Option<Ptr<min_allocator_int_>>) {
    let mut __alloc_default: Option<Value<min_allocator_int_>> = None;
    let alloc: Ptr<min_allocator_int_> = alloc.unwrap_or_else(|| {
        __alloc_default
            .insert(Rc::new(RefCell::new(<min_allocator_int_>::default())))
            .as_pointer()
    });
    {
        let d: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == (n as usize)));
        assert!(
            ({
                let _arg0: min_allocator_int_ = unimplemented!();
                let _arg1: min_allocator_int_ = (*alloc.upgrade().deref()).clone();
                operator_eq_25(_arg0, _arg1)
            })
        );
        const _: () = assert!(true, "");;
    }
}
pub fn test_33(mut n: u32) {
    ({ test1_28(n) });
    ({ test2_26(n) });
}
pub fn test_34(mut n: u32) {
    ({ test1_29(n) });
    ({ test2_27(n) });
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
    ({ test_33(0_u32) });
    ({ test_33(1_u32) });
    ({ test_33(10_u32) });
    ({ test_33(1023_u32) });
    ({ test_33(1024_u32) });
    ({ test_33(1025_u32) });
    ({ test_33(2047_u32) });
    ({ test_33(2048_u32) });
    ({ test_33(2049_u32) });
    ({ test_33(4095_u32) });
    ({ test_33(4096_u32) });
    ({ test_33(4097_u32) });
    const _: () = assert!(true, "");;
    ({ test_34(4095_u32) });
    ({ test3_30(1023_u32, None) });
    ({ test3_31(1_u32, None) });
    ({ test3_32(3_u32, None) });
    return 0;
}
pub trait DefaultOnlyImpl {
    fn copy_assign(&self, _a0: Ptr<DefaultOnly>) -> Ptr<DefaultOnly> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl DefaultOnlyImpl for Ptr<DefaultOnly> {
    fn destructor(&self) {
        field!((*self), data_).write(0);
        (*count_4.with(Value::clone).borrow_mut()).prefix_dec();
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
    fn allocate(
        &self,
        n: usize,
    ) -> min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__;
    fn deallocate(
        &self,
        p: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__,
        n: usize,
    );
}
impl min_allocator_DefaultOnly_Impl for Ptr<min_allocator_DefaultOnly_> {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {
        return min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
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
pub trait min_allocator_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0___Impl {
    fn allocate ( &self , n : usize  , ) -> min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn deallocate(
        &self,
        p : min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        n: usize,
    );
}
impl min_allocator_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0___Impl
    for Ptr<min_allocator_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0___>
{
    fn allocate ( &self , mut n : usize  , ) -> min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()   } , )    ;
    }
    fn deallocate(
        &self,
        mut p : min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
pub trait min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___Impl {
    fn allocate ( &self , n : usize  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn deallocate(
        &self,
        p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        n: usize,
    );
}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___Impl
    for Ptr<min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___>
{
    fn allocate ( &self , mut n : usize  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()   } , )    ;
    }
    fn deallocate(
        &self,
        mut p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
pub trait min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<DefaultOnly>;
    fn operator_arrow(&self) -> Ptr<DefaultOnly> {
        unimplemented!()
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__>;
    fn operator_post_inc_9(
        &self,
        mut _a0: i32,
    ) -> min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: i32,
    ) -> min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: isize,
    ) -> Ptr<min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        n: isize,
    ) -> min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__;
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<DefaultOnly> {
        unimplemented!()
    }
}
impl min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__>
{
    fn operator_deref(&self) -> Ptr<DefaultOnly> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: isize,
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
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<DefaultOnly> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<DefaultOnly> {
        unimplemented!()
    }
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: i32,
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
        mut _a0: i32,
    ) -> min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<DefaultOnly> {
        unimplemented!()
    }
}
impl min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_DefaultOnly__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
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
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
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
pub trait min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl
{
    fn to_bool(&self) -> bool;
    fn operator_deref(
        &self,
    ) -> Ptr<min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__>;
    fn operator_arrow(
        &self,
    ) -> Ptr<min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_post_inc_9 ( &self , mut _a0 : i32  , ) -> min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_dec_11 ( &self , mut _a0 : i32  , ) -> min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_add_assign ( &self , n : isize  , ) -> Ptr< min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_sub_assign ( &self , n : isize  , ) -> Ptr< min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_add ( &self , n : isize  , ) -> min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn operator_sub ( &self , n : isize  , ) -> min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn operator_index(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
}
impl min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__> {
fn to_bool ( &self , ) -> bool  { return ! ( ( (* self ) .with(|__s| __s . ptr_ .clone()) ).is_null() )   ;
 } fn operator_deref ( &self , ) -> Ptr< min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__  > { return (* self ) .with(|__s| __s . ptr_ .clone())    ;
 } fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { field!((* self ) , ptr_) .with_mut(|__v| __v. prefix_inc ()) ;
 return ((* self )  ).clone()  ;
 } fn operator_add_assign ( &self , mut n : isize  , ) -> Ptr< min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { { let _ptr = field!((* self ) , ptr_); _ptr.write(_ptr.read() + n ) } ;
 return ((* self )  ).clone()  ;
 } fn operator_sub_assign ( &self , mut n : isize  , ) -> Ptr< min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { { let _ptr = field!((* self ) , ptr_); _ptr.write(_ptr.read() - n ) } ;
 return ((* self )  ).clone()  ;
 } fn operator_add ( &self , mut n : isize  , ) -> min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  { let tmp : Value<min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ > = Rc::new(RefCell::new((( * (* self ) .upgrade().deref() )  ).clone() )) ;
  ;
 ;
 ( { min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl :: operator_add_assign ( &tmp .as_pointer()  , n  , ) } )  ;
 return ((*tmp.borrow()) ).clone()   ;
 } fn operator_sub ( &self , mut n : isize  , ) -> min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  { let tmp : Value<min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ > = Rc::new(RefCell::new((( * (* self ) .upgrade().deref() )  ).clone() )) ;
  ;
 ;
 ( { min_pointer_min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl :: operator_sub_assign ( &tmp .as_pointer()  , n  , ) } )  ;
 return ((*tmp.borrow()) ).clone()   ;
 } }
pub trait min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl
{
    fn to_bool(&self) -> bool;
    fn operator_deref(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_arrow(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_post_inc_9 ( &self , mut _a0 : i32  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_dec_11 ( &self , mut _a0 : i32  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_add_assign ( &self , n : isize  , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_sub_assign ( &self , n : isize  , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_add ( &self , n : isize  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn operator_sub ( &self , n : isize  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn operator_index(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
}
impl min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__> {
fn to_bool ( &self , ) -> bool  { return ! ( ( (* self ) .with(|__s| __s . ptr_ .clone()) ).is_null() )   ;
 } fn operator_deref ( &self , ) -> Ptr< min_pointer_int__std_integral_constant_unsigned_long__0__  > { return (* self ) .with(|__s| __s . ptr_ .clone())    ;
 } fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { field!((* self ) , ptr_) .with_mut(|__v| __v. prefix_inc ()) ;
 return ((* self )  ).clone()  ;
 } fn operator_add_assign ( &self , mut n : isize  , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { { let _ptr = field!((* self ) , ptr_); _ptr.write(_ptr.read() + n ) } ;
 return ((* self )  ).clone()  ;
 } fn operator_sub_assign ( &self , mut n : isize  , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { { let _ptr = field!((* self ) , ptr_); _ptr.write(_ptr.read() - n ) } ;
 return ((* self )  ).clone()  ;
 } fn operator_add ( &self , mut n : isize  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  { let tmp : Value<min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ > = Rc::new(RefCell::new((( * (* self ) .upgrade().deref() )  ).clone() )) ;
  ;
 ;
 ( { min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl :: operator_add_assign ( &tmp .as_pointer()  , n  , ) } )  ;
 return ((*tmp.borrow()) ).clone()   ;
 } fn operator_sub ( &self , mut n : isize  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  { let tmp : Value<min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ > = Rc::new(RefCell::new((( * (* self ) .upgrade().deref() )  ).clone() )) ;
  ;
 ;
 ( { min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl :: operator_sub_assign ( &tmp .as_pointer()  , n  , ) } )  ;
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
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
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
    let _ = destructed_value_0.with(|_| ());
    let _ = moved_value_1.with(|_| ());
    let _ = count_4.with(|_| ());
    let _ = outstanding_bytes_7.with(|_| ());
    let _ = alloc_count_8.with(|_| ());
    let _ = dealloc_count_9.with(|_| ());
    let _ = disable_default_constructor_10.with(|_| ());
}
