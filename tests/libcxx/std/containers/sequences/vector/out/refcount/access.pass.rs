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
impl CppMove for test_macros_detail_is_same_int_ref__int_ref_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_int_ref__const_int_ref_ {}
impl CppMove for test_macros_detail_is_same_const_int_ref__const_int_ref_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
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
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
impl CppMove for min_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct safe_allocator_int_ {}
impl safe_allocator_int_ {
    pub fn new() -> Self {
        Self {}
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
pub fn make_7(mut size: i32, mut start: i32) -> Vec<i32> {
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
pub fn make_8(mut size: i32, mut start: i32) -> Vec<i32> {
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
pub fn make_9(mut size: i32, mut start: i32) -> Vec<i32> {
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
pub fn test_get_basic_10(c: Ptr<Vec<i32>>, mut start_value: i32) {
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
pub fn test_get_basic_11(c: Ptr<Vec<i32>>, mut start_value: i32) {
    let mut n: i32 = ((*c.upgrade().deref()).len() as i32);
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        assert!(
            ({
                (match ({ (i as usize) },) {
                    (mut __tmp_2,) => {
                        if __tmp_2 as usize >= (*c.upgrade().deref()).len() {
                            panic!("out of bounds access")
                        } else {
                            c.decay().offset(__tmp_2 as isize)
                        }
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
                (match ({ (i as usize) },) {
                    (mut __tmp_3,) => {
                        if __tmp_3 as usize >= (*c.upgrade().deref()).len() {
                            panic!("out of bounds access")
                        } else {
                            c.decay().offset(__tmp_3 as isize)
                        }
                    }
                }
                .read())
            } == { (start_value + i) })
        );
        i.prefix_inc();
    }
    assert!(({ ({ (({ (*c.upgrade().deref()).front_54() }).read()) } == { start_value }) }));
    assert!(({ (c.decay().to_last().read()) } == { ((start_value + n) - 1) }));
}
pub fn test_get_basic_12(c: Ptr<Vec<i32>>, mut start_value: i32) {
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
pub fn test_get_basic_13(c: Ptr<Vec<i32>>, mut start_value: i32) {
    let mut n: i32 = ((*c.upgrade().deref()).len() as i32);
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        assert!(
            ({
                (match ({ (i as usize) },) {
                    (mut __tmp_4,) => {
                        if __tmp_4 as usize >= (*c.upgrade().deref()).len() {
                            panic!("out of bounds access")
                        } else {
                            c.decay().offset(__tmp_4 as isize)
                        }
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
                (match ({ (i as usize) },) {
                    (mut __tmp_5,) => {
                        if __tmp_5 as usize >= (*c.upgrade().deref()).len() {
                            panic!("out of bounds access")
                        } else {
                            c.decay().offset(__tmp_5 as isize)
                        }
                    }
                }
                .read())
            } == { (start_value + i) })
        );
        i.prefix_inc();
    }
    assert!(({ ({ (({ (*c.upgrade().deref()).front_54() }).read()) } == { start_value }) }));
    assert!(({ (c.decay().to_last().read()) } == { ((start_value + n) - 1) }));
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
                (match ({ (i as usize) },) {
                    (mut __tmp_6,) => {
                        if __tmp_6 as usize >= (*c.upgrade().deref()).len() {
                            panic!("out of bounds access")
                        } else {
                            c.decay().offset(__tmp_6 as isize)
                        }
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
                (match ({ (i as usize) },) {
                    (mut __tmp_7,) => {
                        if __tmp_7 as usize >= (*c.upgrade().deref()).len() {
                            panic!("out of bounds access")
                        } else {
                            c.decay().offset(__tmp_7 as isize)
                        }
                    }
                }
                .read())
            } == { (start_value + i) })
        );
        i.prefix_inc();
    }
    assert!(({ ({ (({ (*c.upgrade().deref()).front_54() }).read()) } == { start_value }) }));
    assert!(({ (c.decay().to_last().read()) } == { ((start_value + n) - 1) }));
}
pub fn test_get_16() {
    let mut start_value: i32 = 35;
    let c: Value<Vec<i32>> = Rc::new(RefCell::new(({ make_7(10, start_value) })));
    let cc: Ptr<Vec<i32>> = c.as_pointer();
    ({ test_get_basic_10(c.as_pointer(), start_value) });
    ({
        let _c: Ptr<Vec<i32>> = (cc).clone();
        let _start_value: i32 = start_value;
        test_get_basic_11(_c, _start_value)
    });
}
pub fn test_get_17() {
    let mut start_value: i32 = 35;
    let c: Value<Vec<i32>> = Rc::new(RefCell::new(({ make_8(10, start_value) })));
    let cc: Ptr<Vec<i32>> = c.as_pointer();
    ({ test_get_basic_12(c.as_pointer(), start_value) });
    ({
        let _c: Ptr<Vec<i32>> = (cc).clone();
        let _start_value: i32 = start_value;
        test_get_basic_13(_c, _start_value)
    });
}
pub fn test_get_18() {
    let mut start_value: i32 = 35;
    let c: Value<Vec<i32>> = Rc::new(RefCell::new(({ make_9(10, start_value) })));
    let cc: Ptr<Vec<i32>> = c.as_pointer();
    ({ test_get_basic_14(c.as_pointer(), start_value) });
    ({
        let _c: Ptr<Vec<i32>> = (cc).clone();
        let _start_value: i32 = start_value;
        test_get_basic_15(_c, _start_value)
    });
}
pub fn test_set_19() {
    let mut start_value: i32 = 35;
    let mut n: i32 = 10;
    let c: Value<Vec<i32>> = Rc::new(RefCell::new(({ make_7(n, start_value) })));
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
pub fn test_set_20() {
    let mut start_value: i32 = 35;
    let mut n: i32 = 10;
    let c: Value<Vec<i32>> = Rc::new(RefCell::new(({ make_8(n, start_value) })));
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
pub fn test_set_21() {
    let mut start_value: i32 = 35;
    let mut n: i32 = 10;
    let c: Value<Vec<i32>> = Rc::new(RefCell::new(({ make_9(n, start_value) })));
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
pub fn test_22() {
    ({ test_get_16() });
    ({ test_set_19() });
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
pub fn test_23() {
    ({ test_get_17() });
    ({ test_set_20() });
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
pub fn test_24() {
    ({ test_get_18() });
    ({ test_set_21() });
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
pub fn tests_25() -> bool {
    ({ test_22() });
    ({ test_23() });
    ({ test_24() });
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
    ({ tests_25() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
pub trait min_allocator_int_Impl {
    fn allocate(&self, n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_int__std_integral_constant_unsigned_long__0__, n: usize);
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {
    fn allocate(&self, mut n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_1({
            ({ <std_allocator_int_>::default().allocate(n) })
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        ({ <std_allocator_int_>::default().deallocate((p.ptr_).clone(), n) });
    }
}
pub trait safe_allocator_int_Impl {
    fn allocate(&self, n: usize) -> Ptr<i32>;
    fn deallocate(&self, p: Ptr<i32>, n: usize);
}
impl safe_allocator_int_Impl for Ptr<safe_allocator_int_> {
    fn allocate(&self, mut n: usize) -> Ptr<i32> {
        let memory: Value<Ptr<i32>> = Rc::new(RefCell::new(
            ({ <std_allocator_int_>::default().allocate(n) }),
        ));
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
                DoNotOptimize_2(_value.as_pointer())
            });
        }
        ({ <std_allocator_int_>::default().deallocate((p).clone(), n) });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_3.with(|_| ());
    let _ = alloc_count_4.with(|_| ());
    let _ = dealloc_count_5.with(|_| ());
    let _ = disable_default_constructor_6.with(|_| ());
}
