extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
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
pub struct test_allocator_int__rebind_int_ {}
#[derive(Record, ByteRepr, FnPtrArg)]
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
impl test_allocator_int_ {}
impl Default for test_allocator_int_ {
    fn default() -> Self {
        {
            test_allocator_int_ {
                data_: 0_i32,
                id_: 0_i32,
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
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<i32>,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: AnyPtr,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
pub fn make_9(vals: Vec<i32>) -> () {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut v: i32 = (v.read());
        unimplemented!();
    }
    return unimplemented!();
}
pub fn make_10(vals: Vec<i32>) -> () {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut v: i32 = (v.read());
        unimplemented!();
    }
    return unimplemented!();
}
pub fn make_11(vals: Vec<i32>) -> () {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut v: i32 = (v.read());
        unimplemented!();
    }
    return unimplemented!();
}
pub fn make_12(vals: Vec<i32>) -> () {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut v: i32 = (v.read());
        unimplemented!();
    }
    return unimplemented!();
}
pub fn make_13(vals: Vec<i32>) -> () {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut v: i32 = (v.read());
        unimplemented!();
    }
    return unimplemented!();
}
pub fn test0_14(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_9((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_9((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_15(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_9((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_9((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_16(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_9((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_9((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_17(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_9((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_9((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_18(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_9((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_9((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_19(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_9((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_9((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_20(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_10((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_10((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_21(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_10((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_10((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_22(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_10((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_10((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_23(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_10((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_10((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_24(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_10((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_10((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_25(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_10((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_10((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_26(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_11((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_11((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_27(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_11((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_11((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_28(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_11((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_11((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_29(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_11((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_11((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_30(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_11((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_11((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_31(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_11((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_11((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_32(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_12((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_12((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_33(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_12((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_12((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_34(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_12((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_12((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_35(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_12((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_12((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_36(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_12((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_12((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_37(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_12((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_12((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_38(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_13((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_13((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_39(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_13((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_13((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_40(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_13((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_13((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_41(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_13((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_13((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_42(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_13((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_13((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test0_43(
    vals: Vec<i32>,
    p: FnPtr<Generic>,
    expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    let vals: Value<Vec<i32>> = Rc::new(RefCell::new(vals));
    let p: Value<FnPtr<Generic>> = Rc::new(RefCell::new(p));
    let expected: Value<Vec<i32>> = Rc::new(RefCell::new(expected));
    let s: Value<()> = Rc::new(RefCell::new(({ make_13((*vals.borrow()).clone()) })));
    const _: () = assert!(true, "");;
    assert!((expected_erased_count == (unimplemented!() as usize)));
    let e: Value<()> = Rc::new(RefCell::new(({ make_13((*expected.borrow()).clone()) })));
    assert!((unimplemented!()));
}
pub fn test_44() {
    let is1: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut v: i32| -> bool {
            return (v == 1);
        })));
    let is2: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut v: i32| -> bool {
            return (v == 2);
        })));
    let is3: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut v: i32| -> bool {
            return (v == 3);
        })));
    let is4: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut v: i32| -> bool {
            return (v == 4);
        })));
    let True: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut _a0: i32| -> bool {
            return true;
        })));
    let False: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut _a0: i32| -> bool {
            return false;
        })));
    ({
        test0_14(
            unimplemented!(),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            0_usize,
        )
    });
    ({
        test0_14(
            Vec::from([1]),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            1_usize,
        )
    });
    ({
        test0_15(
            Vec::from([1]),
            (*is2.borrow()).copy_from(),
            Vec::from([1]),
            0_usize,
        )
    });
    ({
        test0_14(
            Vec::from([1, 2]),
            (*is1.borrow()).copy_from(),
            Vec::from([2]),
            1_usize,
        )
    });
    ({
        test0_15(
            Vec::from([1, 2]),
            (*is2.borrow()).copy_from(),
            Vec::from([1]),
            1_usize,
        )
    });
    ({
        test0_16(
            Vec::from([1, 2]),
            (*is3.borrow()).copy_from(),
            Vec::from([1, 2]),
            0_usize,
        )
    });
    ({
        test0_14(
            Vec::from([1, 1]),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            2_usize,
        )
    });
    ({
        test0_16(
            Vec::from([1, 1]),
            (*is3.borrow()).copy_from(),
            Vec::from([1, 1]),
            0_usize,
        )
    });
    ({
        test0_14(
            Vec::from([1, 2, 3]),
            (*is1.borrow()).copy_from(),
            Vec::from([2, 3]),
            1_usize,
        )
    });
    ({
        test0_15(
            Vec::from([1, 2, 3]),
            (*is2.borrow()).copy_from(),
            Vec::from([1, 3]),
            1_usize,
        )
    });
    ({
        test0_16(
            Vec::from([1, 2, 3]),
            (*is3.borrow()).copy_from(),
            Vec::from([1, 2]),
            1_usize,
        )
    });
    ({
        test0_17(
            Vec::from([1, 2, 3]),
            (*is4.borrow()).copy_from(),
            Vec::from([1, 2, 3]),
            0_usize,
        )
    });
    ({
        test0_14(
            Vec::from([1, 1, 1]),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            3_usize,
        )
    });
    ({
        test0_15(
            Vec::from([1, 1, 1]),
            (*is2.borrow()).copy_from(),
            Vec::from([1, 1, 1]),
            0_usize,
        )
    });
    ({
        test0_14(
            Vec::from([1, 1, 2]),
            (*is1.borrow()).copy_from(),
            Vec::from([2]),
            2_usize,
        )
    });
    ({
        test0_15(
            Vec::from([1, 1, 2]),
            (*is2.borrow()).copy_from(),
            Vec::from([1, 1]),
            1_usize,
        )
    });
    ({
        test0_16(
            Vec::from([1, 1, 2]),
            (*is3.borrow()).copy_from(),
            Vec::from([1, 1, 2]),
            0_usize,
        )
    });
    ({
        test0_14(
            Vec::from([1, 2, 2]),
            (*is1.borrow()).copy_from(),
            Vec::from([2, 2]),
            1_usize,
        )
    });
    ({
        test0_15(
            Vec::from([1, 2, 2]),
            (*is2.borrow()).copy_from(),
            Vec::from([1]),
            2_usize,
        )
    });
    ({
        test0_16(
            Vec::from([1, 2, 2]),
            (*is3.borrow()).copy_from(),
            Vec::from([1, 2, 2]),
            0_usize,
        )
    });
    ({
        test0_18(
            Vec::from([1, 2, 3]),
            (*True.borrow()).copy_from(),
            unimplemented!(),
            3_usize,
        )
    });
    ({
        test0_19(
            Vec::from([1, 2, 3]),
            (*False.borrow()).copy_from(),
            Vec::from([1, 2, 3]),
            0_usize,
        )
    });
}
pub fn test_45() {
    let is1: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut v: i32| -> bool {
            return (v == 1);
        })));
    let is2: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut v: i32| -> bool {
            return (v == 2);
        })));
    let is3: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut v: i32| -> bool {
            return (v == 3);
        })));
    let is4: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut v: i32| -> bool {
            return (v == 4);
        })));
    let True: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut _a0: i32| -> bool {
            return true;
        })));
    let False: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut _a0: i32| -> bool {
            return false;
        })));
    ({
        test0_20(
            unimplemented!(),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            0_usize,
        )
    });
    ({
        test0_20(
            Vec::from([1]),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            1_usize,
        )
    });
    ({
        test0_21(
            Vec::from([1]),
            (*is2.borrow()).copy_from(),
            Vec::from([1]),
            0_usize,
        )
    });
    ({
        test0_20(
            Vec::from([1, 2]),
            (*is1.borrow()).copy_from(),
            Vec::from([2]),
            1_usize,
        )
    });
    ({
        test0_21(
            Vec::from([1, 2]),
            (*is2.borrow()).copy_from(),
            Vec::from([1]),
            1_usize,
        )
    });
    ({
        test0_22(
            Vec::from([1, 2]),
            (*is3.borrow()).copy_from(),
            Vec::from([1, 2]),
            0_usize,
        )
    });
    ({
        test0_20(
            Vec::from([1, 1]),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            2_usize,
        )
    });
    ({
        test0_22(
            Vec::from([1, 1]),
            (*is3.borrow()).copy_from(),
            Vec::from([1, 1]),
            0_usize,
        )
    });
    ({
        test0_20(
            Vec::from([1, 2, 3]),
            (*is1.borrow()).copy_from(),
            Vec::from([2, 3]),
            1_usize,
        )
    });
    ({
        test0_21(
            Vec::from([1, 2, 3]),
            (*is2.borrow()).copy_from(),
            Vec::from([1, 3]),
            1_usize,
        )
    });
    ({
        test0_22(
            Vec::from([1, 2, 3]),
            (*is3.borrow()).copy_from(),
            Vec::from([1, 2]),
            1_usize,
        )
    });
    ({
        test0_23(
            Vec::from([1, 2, 3]),
            (*is4.borrow()).copy_from(),
            Vec::from([1, 2, 3]),
            0_usize,
        )
    });
    ({
        test0_20(
            Vec::from([1, 1, 1]),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            3_usize,
        )
    });
    ({
        test0_21(
            Vec::from([1, 1, 1]),
            (*is2.borrow()).copy_from(),
            Vec::from([1, 1, 1]),
            0_usize,
        )
    });
    ({
        test0_20(
            Vec::from([1, 1, 2]),
            (*is1.borrow()).copy_from(),
            Vec::from([2]),
            2_usize,
        )
    });
    ({
        test0_21(
            Vec::from([1, 1, 2]),
            (*is2.borrow()).copy_from(),
            Vec::from([1, 1]),
            1_usize,
        )
    });
    ({
        test0_22(
            Vec::from([1, 1, 2]),
            (*is3.borrow()).copy_from(),
            Vec::from([1, 1, 2]),
            0_usize,
        )
    });
    ({
        test0_20(
            Vec::from([1, 2, 2]),
            (*is1.borrow()).copy_from(),
            Vec::from([2, 2]),
            1_usize,
        )
    });
    ({
        test0_21(
            Vec::from([1, 2, 2]),
            (*is2.borrow()).copy_from(),
            Vec::from([1]),
            2_usize,
        )
    });
    ({
        test0_22(
            Vec::from([1, 2, 2]),
            (*is3.borrow()).copy_from(),
            Vec::from([1, 2, 2]),
            0_usize,
        )
    });
    ({
        test0_24(
            Vec::from([1, 2, 3]),
            (*True.borrow()).copy_from(),
            unimplemented!(),
            3_usize,
        )
    });
    ({
        test0_25(
            Vec::from([1, 2, 3]),
            (*False.borrow()).copy_from(),
            Vec::from([1, 2, 3]),
            0_usize,
        )
    });
}
pub fn test_46() {
    let is1: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut v: i32| -> bool {
            return (v == 1);
        })));
    let is2: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut v: i32| -> bool {
            return (v == 2);
        })));
    let is3: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut v: i32| -> bool {
            return (v == 3);
        })));
    let is4: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut v: i32| -> bool {
            return (v == 4);
        })));
    let True: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut _a0: i32| -> bool {
            return true;
        })));
    let False: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut _a0: i32| -> bool {
            return false;
        })));
    ({
        test0_26(
            unimplemented!(),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            0_usize,
        )
    });
    ({
        test0_26(
            Vec::from([1]),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            1_usize,
        )
    });
    ({
        test0_27(
            Vec::from([1]),
            (*is2.borrow()).copy_from(),
            Vec::from([1]),
            0_usize,
        )
    });
    ({
        test0_26(
            Vec::from([1, 2]),
            (*is1.borrow()).copy_from(),
            Vec::from([2]),
            1_usize,
        )
    });
    ({
        test0_27(
            Vec::from([1, 2]),
            (*is2.borrow()).copy_from(),
            Vec::from([1]),
            1_usize,
        )
    });
    ({
        test0_28(
            Vec::from([1, 2]),
            (*is3.borrow()).copy_from(),
            Vec::from([1, 2]),
            0_usize,
        )
    });
    ({
        test0_26(
            Vec::from([1, 1]),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            2_usize,
        )
    });
    ({
        test0_28(
            Vec::from([1, 1]),
            (*is3.borrow()).copy_from(),
            Vec::from([1, 1]),
            0_usize,
        )
    });
    ({
        test0_26(
            Vec::from([1, 2, 3]),
            (*is1.borrow()).copy_from(),
            Vec::from([2, 3]),
            1_usize,
        )
    });
    ({
        test0_27(
            Vec::from([1, 2, 3]),
            (*is2.borrow()).copy_from(),
            Vec::from([1, 3]),
            1_usize,
        )
    });
    ({
        test0_28(
            Vec::from([1, 2, 3]),
            (*is3.borrow()).copy_from(),
            Vec::from([1, 2]),
            1_usize,
        )
    });
    ({
        test0_29(
            Vec::from([1, 2, 3]),
            (*is4.borrow()).copy_from(),
            Vec::from([1, 2, 3]),
            0_usize,
        )
    });
    ({
        test0_26(
            Vec::from([1, 1, 1]),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            3_usize,
        )
    });
    ({
        test0_27(
            Vec::from([1, 1, 1]),
            (*is2.borrow()).copy_from(),
            Vec::from([1, 1, 1]),
            0_usize,
        )
    });
    ({
        test0_26(
            Vec::from([1, 1, 2]),
            (*is1.borrow()).copy_from(),
            Vec::from([2]),
            2_usize,
        )
    });
    ({
        test0_27(
            Vec::from([1, 1, 2]),
            (*is2.borrow()).copy_from(),
            Vec::from([1, 1]),
            1_usize,
        )
    });
    ({
        test0_28(
            Vec::from([1, 1, 2]),
            (*is3.borrow()).copy_from(),
            Vec::from([1, 1, 2]),
            0_usize,
        )
    });
    ({
        test0_26(
            Vec::from([1, 2, 2]),
            (*is1.borrow()).copy_from(),
            Vec::from([2, 2]),
            1_usize,
        )
    });
    ({
        test0_27(
            Vec::from([1, 2, 2]),
            (*is2.borrow()).copy_from(),
            Vec::from([1]),
            2_usize,
        )
    });
    ({
        test0_28(
            Vec::from([1, 2, 2]),
            (*is3.borrow()).copy_from(),
            Vec::from([1, 2, 2]),
            0_usize,
        )
    });
    ({
        test0_30(
            Vec::from([1, 2, 3]),
            (*True.borrow()).copy_from(),
            unimplemented!(),
            3_usize,
        )
    });
    ({
        test0_31(
            Vec::from([1, 2, 3]),
            (*False.borrow()).copy_from(),
            Vec::from([1, 2, 3]),
            0_usize,
        )
    });
}
pub fn test_47() {
    let is1: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut v: i64| -> bool {
            return (v == 1_i64);
        })));
    let is2: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut v: i64| -> bool {
            return (v == 2_i64);
        })));
    let is3: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut v: i64| -> bool {
            return (v == 3_i64);
        })));
    let is4: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut v: i64| -> bool {
            return (v == 4_i64);
        })));
    let True: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut _a0: i64| -> bool {
            return true;
        })));
    let False: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut _a0: i64| -> bool {
            return false;
        })));
    ({
        test0_32(
            unimplemented!(),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            0_usize,
        )
    });
    ({
        test0_32(
            Vec::from([1]),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            1_usize,
        )
    });
    ({
        test0_33(
            Vec::from([1]),
            (*is2.borrow()).copy_from(),
            Vec::from([1]),
            0_usize,
        )
    });
    ({
        test0_32(
            Vec::from([1, 2]),
            (*is1.borrow()).copy_from(),
            Vec::from([2]),
            1_usize,
        )
    });
    ({
        test0_33(
            Vec::from([1, 2]),
            (*is2.borrow()).copy_from(),
            Vec::from([1]),
            1_usize,
        )
    });
    ({
        test0_34(
            Vec::from([1, 2]),
            (*is3.borrow()).copy_from(),
            Vec::from([1, 2]),
            0_usize,
        )
    });
    ({
        test0_32(
            Vec::from([1, 1]),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            2_usize,
        )
    });
    ({
        test0_34(
            Vec::from([1, 1]),
            (*is3.borrow()).copy_from(),
            Vec::from([1, 1]),
            0_usize,
        )
    });
    ({
        test0_32(
            Vec::from([1, 2, 3]),
            (*is1.borrow()).copy_from(),
            Vec::from([2, 3]),
            1_usize,
        )
    });
    ({
        test0_33(
            Vec::from([1, 2, 3]),
            (*is2.borrow()).copy_from(),
            Vec::from([1, 3]),
            1_usize,
        )
    });
    ({
        test0_34(
            Vec::from([1, 2, 3]),
            (*is3.borrow()).copy_from(),
            Vec::from([1, 2]),
            1_usize,
        )
    });
    ({
        test0_35(
            Vec::from([1, 2, 3]),
            (*is4.borrow()).copy_from(),
            Vec::from([1, 2, 3]),
            0_usize,
        )
    });
    ({
        test0_32(
            Vec::from([1, 1, 1]),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            3_usize,
        )
    });
    ({
        test0_33(
            Vec::from([1, 1, 1]),
            (*is2.borrow()).copy_from(),
            Vec::from([1, 1, 1]),
            0_usize,
        )
    });
    ({
        test0_32(
            Vec::from([1, 1, 2]),
            (*is1.borrow()).copy_from(),
            Vec::from([2]),
            2_usize,
        )
    });
    ({
        test0_33(
            Vec::from([1, 1, 2]),
            (*is2.borrow()).copy_from(),
            Vec::from([1, 1]),
            1_usize,
        )
    });
    ({
        test0_34(
            Vec::from([1, 1, 2]),
            (*is3.borrow()).copy_from(),
            Vec::from([1, 1, 2]),
            0_usize,
        )
    });
    ({
        test0_32(
            Vec::from([1, 2, 2]),
            (*is1.borrow()).copy_from(),
            Vec::from([2, 2]),
            1_usize,
        )
    });
    ({
        test0_33(
            Vec::from([1, 2, 2]),
            (*is2.borrow()).copy_from(),
            Vec::from([1]),
            2_usize,
        )
    });
    ({
        test0_34(
            Vec::from([1, 2, 2]),
            (*is3.borrow()).copy_from(),
            Vec::from([1, 2, 2]),
            0_usize,
        )
    });
    ({
        test0_36(
            Vec::from([1, 2, 3]),
            (*True.borrow()).copy_from(),
            unimplemented!(),
            3_usize,
        )
    });
    ({
        test0_37(
            Vec::from([1, 2, 3]),
            (*False.borrow()).copy_from(),
            Vec::from([1, 2, 3]),
            0_usize,
        )
    });
}
pub fn test_48() {
    let is1: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut v: f64| -> bool {
            return (v == 1_f64);
        })));
    let is2: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut v: f64| -> bool {
            return (v == 2_f64);
        })));
    let is3: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut v: f64| -> bool {
            return (v == 3_f64);
        })));
    let is4: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut v: f64| -> bool {
            return (v == 4_f64);
        })));
    let True: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut _a0: f64| -> bool {
            return true;
        })));
    let False: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut _a0: f64| -> bool {
            return false;
        })));
    ({
        test0_38(
            unimplemented!(),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            0_usize,
        )
    });
    ({
        test0_38(
            Vec::from([1]),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            1_usize,
        )
    });
    ({
        test0_39(
            Vec::from([1]),
            (*is2.borrow()).copy_from(),
            Vec::from([1]),
            0_usize,
        )
    });
    ({
        test0_38(
            Vec::from([1, 2]),
            (*is1.borrow()).copy_from(),
            Vec::from([2]),
            1_usize,
        )
    });
    ({
        test0_39(
            Vec::from([1, 2]),
            (*is2.borrow()).copy_from(),
            Vec::from([1]),
            1_usize,
        )
    });
    ({
        test0_40(
            Vec::from([1, 2]),
            (*is3.borrow()).copy_from(),
            Vec::from([1, 2]),
            0_usize,
        )
    });
    ({
        test0_38(
            Vec::from([1, 1]),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            2_usize,
        )
    });
    ({
        test0_40(
            Vec::from([1, 1]),
            (*is3.borrow()).copy_from(),
            Vec::from([1, 1]),
            0_usize,
        )
    });
    ({
        test0_38(
            Vec::from([1, 2, 3]),
            (*is1.borrow()).copy_from(),
            Vec::from([2, 3]),
            1_usize,
        )
    });
    ({
        test0_39(
            Vec::from([1, 2, 3]),
            (*is2.borrow()).copy_from(),
            Vec::from([1, 3]),
            1_usize,
        )
    });
    ({
        test0_40(
            Vec::from([1, 2, 3]),
            (*is3.borrow()).copy_from(),
            Vec::from([1, 2]),
            1_usize,
        )
    });
    ({
        test0_41(
            Vec::from([1, 2, 3]),
            (*is4.borrow()).copy_from(),
            Vec::from([1, 2, 3]),
            0_usize,
        )
    });
    ({
        test0_38(
            Vec::from([1, 1, 1]),
            (*is1.borrow()).copy_from(),
            unimplemented!(),
            3_usize,
        )
    });
    ({
        test0_39(
            Vec::from([1, 1, 1]),
            (*is2.borrow()).copy_from(),
            Vec::from([1, 1, 1]),
            0_usize,
        )
    });
    ({
        test0_38(
            Vec::from([1, 1, 2]),
            (*is1.borrow()).copy_from(),
            Vec::from([2]),
            2_usize,
        )
    });
    ({
        test0_39(
            Vec::from([1, 1, 2]),
            (*is2.borrow()).copy_from(),
            Vec::from([1, 1]),
            1_usize,
        )
    });
    ({
        test0_40(
            Vec::from([1, 1, 2]),
            (*is3.borrow()).copy_from(),
            Vec::from([1, 1, 2]),
            0_usize,
        )
    });
    ({
        test0_38(
            Vec::from([1, 2, 2]),
            (*is1.borrow()).copy_from(),
            Vec::from([2, 2]),
            1_usize,
        )
    });
    ({
        test0_39(
            Vec::from([1, 2, 2]),
            (*is2.borrow()).copy_from(),
            Vec::from([1]),
            2_usize,
        )
    });
    ({
        test0_40(
            Vec::from([1, 2, 2]),
            (*is3.borrow()).copy_from(),
            Vec::from([1, 2, 2]),
            0_usize,
        )
    });
    ({
        test0_42(
            Vec::from([1, 2, 3]),
            (*True.borrow()).copy_from(),
            unimplemented!(),
            3_usize,
        )
    });
    ({
        test0_43(
            Vec::from([1, 2, 3]),
            (*False.borrow()).copy_from(),
            Vec::from([1, 2, 3]),
            0_usize,
        )
    });
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
    ({ test_44() });
    ({ test_45() });
    ({ test_46() });
    ({ test_47() });
    ({ test_48() });
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
pub trait min_allocator_int_Impl {
    fn allocate(&self, mut n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {}
pub trait min_pointer_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
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
        mut n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
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
pub trait test_allocator_int_Impl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn address_11(&self, x: ()) -> Ptr<i32> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<i32> {
        unimplemented!()
    }
    fn allocate(&self, mut n: u32, mut _a1: AnyPtr) -> Ptr<i32> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<i32>, mut s: u32) {
        unimplemented!()
    }
    fn max_size(&self) -> u32 {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<i32>) {
        unimplemented!()
    }
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
    fn get_id(&self) -> i32 {
        unimplemented!()
    }
}
impl test_allocator_int_Impl for Ptr<test_allocator_int_> {}
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
    let _ = outstanding_bytes_5.with(|_| ());
    let _ = alloc_count_6.with(|_| ());
    let _ = dealloc_count_7.with(|_| ());
    let _ = disable_default_constructor_8.with(|_| ());
}
