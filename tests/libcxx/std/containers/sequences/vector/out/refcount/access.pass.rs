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
pub struct test_macros_detail_is_same_int_ref__int_ref_ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_int_ref__const_int_ref_ {}
pub fn DoNotOptimize_2(value: Ptr<AnyPtr>) -> Ptr<AnyPtr> {
    b"";
    return (value).clone();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_4.with(|rc| *rc.borrow()) >= dealloc_count_5.with(|rc| *rc.borrow())));
        return ((alloc_count_4.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_5.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_6.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_3.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_4.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_5.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
thread_local!(
    pub static outstanding_bytes_3: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_4: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_5: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_6: Value<bool> = Rc::new(RefCell::new(false));
);
pub fn operator_sub_7(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_eq_8(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_9(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !({
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_8(_x, y)
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
        operator_eq_8(__lhs.read(), __rhs.read())
    }
}
impl CppAdvance for min_pointer_int__std_integral_constant_unsigned_long__0__ {
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
pub fn operator_eq_10(
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
impl CppEq for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_10(__lhs.read(), __rhs.read())
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
pub fn make_11(mut size: i32, mut start: i32) -> Vec<i32> {
    let mut c: Vec<i32> = Vec::new();
    let mut i: i32 = 0;
    'loop_: while (i < size) {
        {
            let __a1 = (start + i);
            c.push(__a1)
        };
        i.prefix_inc();
    }
    return std::mem::take(&mut c);
}
pub fn make_12(mut size: i32, mut start: i32) -> Vec<i32> {
    let mut c: Vec<i32> = Vec::new();
    let mut i: i32 = 0;
    'loop_: while (i < size) {
        {
            let __a1 = (start + i);
            c.push(__a1)
        };
        i.prefix_inc();
    }
    return std::mem::take(&mut c);
}
pub fn make_13(mut size: i32, mut start: i32) -> Vec<i32> {
    let mut c: Vec<i32> = Vec::new();
    let mut i: i32 = 0;
    'loop_: while (i < size) {
        {
            let __a1 = (start + i);
            c.push(__a1)
        };
        i.prefix_inc();
    }
    return std::mem::take(&mut c);
}
pub fn test_get_basic_14(c: Ptr<Vec<i32>>, mut start_value: i32) {
    let mut n: i32 = ((*c.upgrade().deref()).len() as i32);
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        assert!(({ (c.decay().offset((i as usize) as isize).read()) } == { (start_value + i) }));
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        assert!(({ (c.decay().offset((i as usize) as isize).read()) } == { (start_value + i) }));
        i.prefix_inc();
    }
    assert!(({ (c.decay().read()) } == { start_value }));
    assert!(({ (c.decay().to_last().read()) } == { ((start_value + n) - 1) }));
}
pub fn test_get_basic_15(c: Ptr<Vec<i32>>, mut start_value: i32) {
    let mut n: i32 = ((*c.upgrade().deref()).len() as i32);
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        assert!(
            ({
                ({
                    let __idx = (i as usize);
                    if __idx as usize >= (*c.upgrade().deref()).len() {
                        panic!("out of bounds access")
                    } else {
                        c.decay().offset(__idx as isize)
                    }
                }
                .read())
            } == { (start_value + i) })
        );
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        assert!(
            ({
                ({
                    let __idx = (i as usize);
                    if __idx as usize >= (*c.upgrade().deref()).len() {
                        panic!("out of bounds access")
                    } else {
                        c.decay().offset(__idx as isize)
                    }
                }
                .read())
            } == { (start_value + i) })
        );
        i.prefix_inc();
    }
    assert!(({ (unimplemented!().read()) } == { start_value }));
    assert!(({ (c.decay().to_last().read()) } == { ((start_value + n) - 1) }));
}
pub fn test_get_basic_16(c: Ptr<Vec<i32>>, mut start_value: i32) {
    let mut n: i32 = ((*c.upgrade().deref()).len() as i32);
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        assert!(({ (c.decay().offset((i as usize) as isize).read()) } == { (start_value + i) }));
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        assert!(({ (c.decay().offset((i as usize) as isize).read()) } == { (start_value + i) }));
        i.prefix_inc();
    }
    assert!(({ (c.decay().read()) } == { start_value }));
    assert!(({ (c.decay().to_last().read()) } == { ((start_value + n) - 1) }));
}
pub fn test_get_basic_17(c: Ptr<Vec<i32>>, mut start_value: i32) {
    let mut n: i32 = ((*c.upgrade().deref()).len() as i32);
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        assert!(
            ({
                ({
                    let __idx = (i as usize);
                    if __idx as usize >= (*c.upgrade().deref()).len() {
                        panic!("out of bounds access")
                    } else {
                        c.decay().offset(__idx as isize)
                    }
                }
                .read())
            } == { (start_value + i) })
        );
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        assert!(
            ({
                ({
                    let __idx = (i as usize);
                    if __idx as usize >= (*c.upgrade().deref()).len() {
                        panic!("out of bounds access")
                    } else {
                        c.decay().offset(__idx as isize)
                    }
                }
                .read())
            } == { (start_value + i) })
        );
        i.prefix_inc();
    }
    assert!(({ (unimplemented!().read()) } == { start_value }));
    assert!(({ (c.decay().to_last().read()) } == { ((start_value + n) - 1) }));
}
pub fn test_get_basic_18(c: Ptr<Vec<i32>>, mut start_value: i32) {
    let mut n: i32 = ((*c.upgrade().deref()).len() as i32);
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        assert!(({ (c.decay().offset((i as usize) as isize).read()) } == { (start_value + i) }));
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        assert!(({ (c.decay().offset((i as usize) as isize).read()) } == { (start_value + i) }));
        i.prefix_inc();
    }
    assert!(({ (c.decay().read()) } == { start_value }));
    assert!(({ (c.decay().to_last().read()) } == { ((start_value + n) - 1) }));
}
pub fn test_get_basic_19(c: Ptr<Vec<i32>>, mut start_value: i32) {
    let mut n: i32 = ((*c.upgrade().deref()).len() as i32);
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        assert!(
            ({
                ({
                    let __idx = (i as usize);
                    if __idx as usize >= (*c.upgrade().deref()).len() {
                        panic!("out of bounds access")
                    } else {
                        c.decay().offset(__idx as isize)
                    }
                }
                .read())
            } == { (start_value + i) })
        );
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        assert!(
            ({
                ({
                    let __idx = (i as usize);
                    if __idx as usize >= (*c.upgrade().deref()).len() {
                        panic!("out of bounds access")
                    } else {
                        c.decay().offset(__idx as isize)
                    }
                }
                .read())
            } == { (start_value + i) })
        );
        i.prefix_inc();
    }
    assert!(({ (unimplemented!().read()) } == { start_value }));
    assert!(({ (c.decay().to_last().read()) } == { ((start_value + n) - 1) }));
}
pub fn test_get_20() {
    let mut start_value: i32 = 35;
    let c: Value<Vec<i32>> = Rc::new(RefCell::new(({ make_11(10, start_value) })));
    let cc: Ptr<Vec<i32>> = c.as_pointer();
    ({ test_get_basic_14(c.as_pointer(), start_value) });
    ({
        let _c: Ptr<Vec<i32>> = (cc).clone();
        let _start_value: i32 = start_value;
        test_get_basic_15(_c, _start_value)
    });
}
pub fn test_get_21() {
    let mut start_value: i32 = 35;
    let c: Value<Vec<i32>> = Rc::new(RefCell::new(({ make_12(10, start_value) })));
    let cc: Ptr<Vec<i32>> = c.as_pointer();
    ({ test_get_basic_16(c.as_pointer(), start_value) });
    ({
        let _c: Ptr<Vec<i32>> = (cc).clone();
        let _start_value: i32 = start_value;
        test_get_basic_17(_c, _start_value)
    });
}
pub fn test_get_22() {
    let mut start_value: i32 = 35;
    let c: Value<Vec<i32>> = Rc::new(RefCell::new(({ make_13(10, start_value) })));
    let cc: Ptr<Vec<i32>> = c.as_pointer();
    ({ test_get_basic_18(c.as_pointer(), start_value) });
    ({
        let _c: Ptr<Vec<i32>> = (cc).clone();
        let _start_value: i32 = start_value;
        test_get_basic_19(_c, _start_value)
    });
}
pub fn test_set_23() {
    let mut start_value: i32 = 35;
    let mut n: i32 = 10;
    let c: Value<Vec<i32>> = Rc::new(RefCell::new(({ make_11(n, start_value) })));
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        assert!(((c.decay().offset((i as usize) as isize).read()) == (start_value + i)));
        let __rhs = ((start_value + i) + 1);
        c.decay().offset((i as usize) as isize).write(__rhs);
        assert!(((c.decay().offset((i as usize) as isize).read()) == ((start_value + i) + 1)));
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        assert!(((c.decay().offset((i as usize) as isize).read()) == ((start_value + i) + 1)));
        let __rhs = ((start_value + i) + 2);
        c.decay().offset((i as usize) as isize).write(__rhs);
        assert!(((c.decay().offset((i as usize) as isize).read()) == ((start_value + i) + 2)));
        i.prefix_inc();
    }
    assert!(((c.decay().read()) == (start_value + 2)));
    c.decay().write((start_value + 3));
    assert!(((c.decay().read()) == (start_value + 3)));
    assert!(((c.decay().to_last().read()) == ((start_value + n) + 1)));
    c.decay().to_last().write(((start_value + n) + 2));
    assert!(((c.decay().to_last().read()) == ((start_value + n) + 2)));
}
pub fn test_set_24() {
    let mut start_value: i32 = 35;
    let mut n: i32 = 10;
    let c: Value<Vec<i32>> = Rc::new(RefCell::new(({ make_12(n, start_value) })));
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        assert!(((c.decay().offset((i as usize) as isize).read()) == (start_value + i)));
        let __rhs = ((start_value + i) + 1);
        c.decay().offset((i as usize) as isize).write(__rhs);
        assert!(((c.decay().offset((i as usize) as isize).read()) == ((start_value + i) + 1)));
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        assert!(((c.decay().offset((i as usize) as isize).read()) == ((start_value + i) + 1)));
        let __rhs = ((start_value + i) + 2);
        c.decay().offset((i as usize) as isize).write(__rhs);
        assert!(((c.decay().offset((i as usize) as isize).read()) == ((start_value + i) + 2)));
        i.prefix_inc();
    }
    assert!(((c.decay().read()) == (start_value + 2)));
    c.decay().write((start_value + 3));
    assert!(((c.decay().read()) == (start_value + 3)));
    assert!(((c.decay().to_last().read()) == ((start_value + n) + 1)));
    c.decay().to_last().write(((start_value + n) + 2));
    assert!(((c.decay().to_last().read()) == ((start_value + n) + 2)));
}
pub fn test_set_25() {
    let mut start_value: i32 = 35;
    let mut n: i32 = 10;
    let c: Value<Vec<i32>> = Rc::new(RefCell::new(({ make_13(n, start_value) })));
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        assert!(((c.decay().offset((i as usize) as isize).read()) == (start_value + i)));
        let __rhs = ((start_value + i) + 1);
        c.decay().offset((i as usize) as isize).write(__rhs);
        assert!(((c.decay().offset((i as usize) as isize).read()) == ((start_value + i) + 1)));
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        assert!(((c.decay().offset((i as usize) as isize).read()) == ((start_value + i) + 1)));
        let __rhs = ((start_value + i) + 2);
        c.decay().offset((i as usize) as isize).write(__rhs);
        assert!(((c.decay().offset((i as usize) as isize).read()) == ((start_value + i) + 2)));
        i.prefix_inc();
    }
    assert!(((c.decay().read()) == (start_value + 2)));
    c.decay().write((start_value + 3));
    assert!(((c.decay().read()) == (start_value + 3)));
    assert!(((c.decay().to_last().read()) == ((start_value + n) + 1)));
    c.decay().to_last().write(((start_value + n) + 2));
    assert!(((c.decay().to_last().read()) == ((start_value + n) + 2)));
}
pub fn test_26() {
    ({ test_get_20() });
    ({ test_set_23() });
    let c: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    let cc: Ptr<Vec<i32>> = c.as_pointer();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_27() {
    ({ test_get_21() });
    ({ test_set_24() });
    let c: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    let cc: Ptr<Vec<i32>> = c.as_pointer();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_28() {
    ({ test_get_22() });
    ({ test_set_25() });
    let c: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    let cc: Ptr<Vec<i32>> = c.as_pointer();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn tests_29() -> bool {
    ({ test_26() });
    ({ test_27() });
    ({ test_28() });
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
    ({ tests_29() });
    const _: () = assert!(true, "tests()");;
    return 0;
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
    fn operator_deref(&self) -> Ptr<i32>;
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
        mut n: isize,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        n: isize,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__>;
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        n: isize,
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__;
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
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_const_int__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_const_int__std_integral_constant_unsigned_long__0__Impl::operator_sub_assign(
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
                DoNotOptimize_2(_value.as_pointer())
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
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_3.with(|_| ());
    let _ = alloc_count_4.with(|_| ());
    let _ = dealloc_count_5.with(|_| ());
    let _ = disable_default_constructor_6.with(|_| ());
}
