extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
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
    pub static destructed_value_1: Value<()> = Rc::new(RefCell::new(-1_()));
);
thread_local!(
    pub static moved_value_2: Value<()> = Rc::new(RefCell::new(2147483647_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_3(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_4(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
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
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_std_pair_const_int__int___rebind_std_pair_const_int__int__ {}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(16)]
pub struct test_allocator_std_pair_const_int__int__ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_std_pair_const_int__int__ {}
impl Default for test_allocator_std_pair_const_int__int__ {
    fn default() -> Self {
        {
            test_allocator_std_pair_const_int__int__ {
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_6.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_7.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_8.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_5.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_6.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_7.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_5: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_6: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_7: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_8: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_pair_const_int__int__ {}
pub fn make_9(vals: ()) -> () {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut v: () = (v.read());
        let __rhs = ((v + 10_()) as ());
        unimplemented!().write(__rhs);
    }
    return unimplemented!();
}
pub fn make_10(vals: ()) -> () {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut v: () = (v.read());
        let __rhs = ((v + 10_()) as ());
        unimplemented!().write(__rhs);
    }
    return unimplemented!();
}
pub fn make_11(vals: ()) -> () {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut v: () = (v.read());
        let __rhs = ((v + 10_()) as ());
        unimplemented!().write(__rhs);
    }
    return unimplemented!();
}
pub fn make_12(vals: ()) -> () {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut v: () = (v.read());
        let __rhs = (v + 10_());
        unimplemented!().write(__rhs);
    }
    return unimplemented!();
}
pub fn make_13(vals: ()) -> () {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut v: () = (v.read());
        let __rhs = (v + 10_());
        unimplemented!().write(__rhs);
    }
    return unimplemented!();
}
pub fn test0_14(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_9((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_15(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_9((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_16(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_9((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_17(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_9((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_18(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_9((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_19(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_9((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_20(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_10((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_21(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_10((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_22(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_10((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_23(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_10((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_24(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_10((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_25(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_10((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_26(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_11((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_27(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_11((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_28(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_11((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_29(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_11((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_30(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_11((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_31(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_11((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_32(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_12((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_33(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_12((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_34(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_12((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_35(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_12((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_36(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_12((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_37(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_12((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_38(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_13((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_39(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_13((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_40(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_13((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_41(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_13((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_42(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_13((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test0_43(vals: (), p: FnPtr<Generic>, expected: (), mut expected_erased_count: ()) {
    let vals: Value<()> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_13((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_44() -> () {
    let is1: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(Generic, {}, |v: ()| -> () {
        let v: Value<()> = Rc::new(RefCell::new(v));
        return ({ (*v.borrow()).first } == 1_());
    })));
    let is2: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(Generic, {}, |v: ()| -> () {
        let v: Value<()> = Rc::new(RefCell::new(v));
        return ({ (*v.borrow()).first } == 2_());
    })));
    let is3: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(Generic, {}, |v: ()| -> () {
        let v: Value<()> = Rc::new(RefCell::new(v));
        return ({ (*v.borrow()).first } == 3_());
    })));
    let is4: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(Generic, {}, |v: ()| -> () {
        let v: Value<()> = Rc::new(RefCell::new(v));
        return ({ (*v.borrow()).first } == 4_());
    })));
    let True: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |_a0: ()| -> () {
            let _a0: Value<()> = Rc::new(RefCell::new(_a0));
            return true;
        })));
    let False: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |_a0: ()| -> () {
            let _a0: Value<()> = Rc::new(RefCell::new(_a0));
            return false;
        })));
    ({
        test0_14(
            unimplemented!(),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test0_14(
            unimplemented!(),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_15(
            unimplemented!(),
            (*is2.borrow()).copy_from(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test0_14(
            unimplemented!(),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_15(
            unimplemented!(),
            (*is2.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_16(
            unimplemented!(),
            (*is3.borrow()).copy_from(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test0_14(
            unimplemented!(),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_15(
            unimplemented!(),
            (*is2.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_16(
            unimplemented!(),
            (*is3.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_17(
            unimplemented!(),
            (*is4.borrow()).copy_from(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test0_18(
            unimplemented!(),
            (*True.borrow()).copy_from(),
            unimplemented!(),
            3_(),
        )
    });
    ({
        test0_19(
            unimplemented!(),
            (*False.borrow()).copy_from(),
            unimplemented!(),
            0_(),
        )
    });
    return true;
}
pub fn test_45() -> () {
    let is1: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(Generic, {}, |v: ()| -> () {
        let v: Value<()> = Rc::new(RefCell::new(v));
        return ({ (*v.borrow()).first } == 1_());
    })));
    let is2: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(Generic, {}, |v: ()| -> () {
        let v: Value<()> = Rc::new(RefCell::new(v));
        return ({ (*v.borrow()).first } == 2_());
    })));
    let is3: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(Generic, {}, |v: ()| -> () {
        let v: Value<()> = Rc::new(RefCell::new(v));
        return ({ (*v.borrow()).first } == 3_());
    })));
    let is4: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(Generic, {}, |v: ()| -> () {
        let v: Value<()> = Rc::new(RefCell::new(v));
        return ({ (*v.borrow()).first } == 4_());
    })));
    let True: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |_a0: ()| -> () {
            let _a0: Value<()> = Rc::new(RefCell::new(_a0));
            return true;
        })));
    let False: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |_a0: ()| -> () {
            let _a0: Value<()> = Rc::new(RefCell::new(_a0));
            return false;
        })));
    ({
        test0_20(
            unimplemented!(),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test0_20(
            unimplemented!(),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_21(
            unimplemented!(),
            (*is2.borrow()).copy_from(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test0_20(
            unimplemented!(),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_21(
            unimplemented!(),
            (*is2.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_22(
            unimplemented!(),
            (*is3.borrow()).copy_from(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test0_20(
            unimplemented!(),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_21(
            unimplemented!(),
            (*is2.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_22(
            unimplemented!(),
            (*is3.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_23(
            unimplemented!(),
            (*is4.borrow()).copy_from(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test0_24(
            unimplemented!(),
            (*True.borrow()).copy_from(),
            unimplemented!(),
            3_(),
        )
    });
    ({
        test0_25(
            unimplemented!(),
            (*False.borrow()).copy_from(),
            unimplemented!(),
            0_(),
        )
    });
    return true;
}
pub fn test_46() -> () {
    let is1: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(Generic, {}, |v: ()| -> () {
        let v: Value<()> = Rc::new(RefCell::new(v));
        return ({ (*v.borrow()).first } == 1_());
    })));
    let is2: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(Generic, {}, |v: ()| -> () {
        let v: Value<()> = Rc::new(RefCell::new(v));
        return ({ (*v.borrow()).first } == 2_());
    })));
    let is3: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(Generic, {}, |v: ()| -> () {
        let v: Value<()> = Rc::new(RefCell::new(v));
        return ({ (*v.borrow()).first } == 3_());
    })));
    let is4: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(Generic, {}, |v: ()| -> () {
        let v: Value<()> = Rc::new(RefCell::new(v));
        return ({ (*v.borrow()).first } == 4_());
    })));
    let True: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |_a0: ()| -> () {
            let _a0: Value<()> = Rc::new(RefCell::new(_a0));
            return true;
        })));
    let False: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |_a0: ()| -> () {
            let _a0: Value<()> = Rc::new(RefCell::new(_a0));
            return false;
        })));
    ({
        test0_26(
            unimplemented!(),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test0_26(
            unimplemented!(),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_27(
            unimplemented!(),
            (*is2.borrow()).copy_from(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test0_26(
            unimplemented!(),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_27(
            unimplemented!(),
            (*is2.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_28(
            unimplemented!(),
            (*is3.borrow()).copy_from(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test0_26(
            unimplemented!(),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_27(
            unimplemented!(),
            (*is2.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_28(
            unimplemented!(),
            (*is3.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_29(
            unimplemented!(),
            (*is4.borrow()).copy_from(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test0_30(
            unimplemented!(),
            (*True.borrow()).copy_from(),
            unimplemented!(),
            3_(),
        )
    });
    ({
        test0_31(
            unimplemented!(),
            (*False.borrow()).copy_from(),
            unimplemented!(),
            0_(),
        )
    });
    return true;
}
pub fn test_47() -> () {
    let is1: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(Generic, {}, |v: ()| -> () {
        let v: Value<()> = Rc::new(RefCell::new(v));
        return ({ (*v.borrow()).first } == 1_());
    })));
    let is2: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(Generic, {}, |v: ()| -> () {
        let v: Value<()> = Rc::new(RefCell::new(v));
        return ({ (*v.borrow()).first } == 2_());
    })));
    let is3: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(Generic, {}, |v: ()| -> () {
        let v: Value<()> = Rc::new(RefCell::new(v));
        return ({ (*v.borrow()).first } == 3_());
    })));
    let is4: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(Generic, {}, |v: ()| -> () {
        let v: Value<()> = Rc::new(RefCell::new(v));
        return ({ (*v.borrow()).first } == 4_());
    })));
    let True: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |_a0: ()| -> () {
            let _a0: Value<()> = Rc::new(RefCell::new(_a0));
            return true;
        })));
    let False: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |_a0: ()| -> () {
            let _a0: Value<()> = Rc::new(RefCell::new(_a0));
            return false;
        })));
    ({
        test0_32(
            unimplemented!(),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test0_32(
            unimplemented!(),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_33(
            unimplemented!(),
            (*is2.borrow()).copy_from(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test0_32(
            unimplemented!(),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_33(
            unimplemented!(),
            (*is2.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_34(
            unimplemented!(),
            (*is3.borrow()).copy_from(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test0_32(
            unimplemented!(),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_33(
            unimplemented!(),
            (*is2.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_34(
            unimplemented!(),
            (*is3.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_35(
            unimplemented!(),
            (*is4.borrow()).copy_from(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test0_36(
            unimplemented!(),
            (*True.borrow()).copy_from(),
            unimplemented!(),
            3_(),
        )
    });
    ({
        test0_37(
            unimplemented!(),
            (*False.borrow()).copy_from(),
            unimplemented!(),
            0_(),
        )
    });
    return true;
}
pub fn test_48() -> () {
    let is1: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(Generic, {}, |v: ()| -> () {
        let v: Value<()> = Rc::new(RefCell::new(v));
        return ({ (*v.borrow()).first } == 1_());
    })));
    let is2: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(Generic, {}, |v: ()| -> () {
        let v: Value<()> = Rc::new(RefCell::new(v));
        return ({ (*v.borrow()).first } == 2_());
    })));
    let is3: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(Generic, {}, |v: ()| -> () {
        let v: Value<()> = Rc::new(RefCell::new(v));
        return ({ (*v.borrow()).first } == 3_());
    })));
    let is4: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(Generic, {}, |v: ()| -> () {
        let v: Value<()> = Rc::new(RefCell::new(v));
        return ({ (*v.borrow()).first } == 4_());
    })));
    let True: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |_a0: ()| -> () {
            let _a0: Value<()> = Rc::new(RefCell::new(_a0));
            return true;
        })));
    let False: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |_a0: ()| -> () {
            let _a0: Value<()> = Rc::new(RefCell::new(_a0));
            return false;
        })));
    ({
        test0_38(
            unimplemented!(),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test0_38(
            unimplemented!(),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_39(
            unimplemented!(),
            (*is2.borrow()).copy_from(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test0_38(
            unimplemented!(),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_39(
            unimplemented!(),
            (*is2.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_40(
            unimplemented!(),
            (*is3.borrow()).copy_from(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test0_38(
            unimplemented!(),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_39(
            unimplemented!(),
            (*is2.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_40(
            unimplemented!(),
            (*is3.borrow()).copy_from(),
            unimplemented!(),
            1_(),
        )
    });
    ({
        test0_41(
            unimplemented!(),
            (*is4.borrow()).copy_from(),
            unimplemented!(),
            0_(),
        )
    });
    ({
        test0_42(
            unimplemented!(),
            (*True.borrow()).copy_from(),
            unimplemented!(),
            3_(),
        )
    });
    ({
        test0_43(
            unimplemented!(),
            (*False.borrow()).copy_from(),
            unimplemented!(),
            0_(),
        )
    });
    return true;
}
pub fn test_upper_49() -> () {
    ({ test_44() });
    ({ test_45() });
    ({ test_46() });
    ({ test_47() });
    ({ test_48() });
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
    ({ test_upper_49() });
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
pub trait min_allocator_std_pair_const_int__int__Impl {
    fn allocate(
        &self,
        mut n: (),
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_std_pair_const_int__int__Impl for Ptr<min_allocator_std_pair_const_int__int__> {}
pub trait min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__Impl {
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
    ) -> Ptr<min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
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
        mut _a0: (),
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__>
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
pub trait test_allocator_std_pair_const_int__int__Impl {
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
impl test_allocator_std_pair_const_int__int__Impl
    for Ptr<test_allocator_std_pair_const_int__int__>
{
}
pub trait test_allocator_void_Impl {
    fn destructor(&self);
    fn get_id(&self) -> ();
    fn get_data(&self) -> ();
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_>;
}
impl test_allocator_void_Impl for Ptr<test_allocator_void_> {
    fn destructor(&self) {
        field!((*self), data_).write(destructed_value_1.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_1.with(|rc| *rc.borrow()));
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
    let _ = destructed_value_1.with(|_| ());
    let _ = moved_value_2.with(|_| ());
    let _ = outstanding_bytes_5.with(|_| ());
    let _ = alloc_count_6.with(|_| ());
    let _ = dealloc_count_7.with(|_| ());
    let _ = disable_default_constructor_8.with(|_| ());
}
