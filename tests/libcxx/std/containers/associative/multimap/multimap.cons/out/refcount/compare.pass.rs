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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct test_less_int_ {
    #[offset(0)]
    pub data_: i32,
}
impl test_less_int_ {
    pub fn new_1(mut data: i32) -> Self {
        Self { data_: data }
    }
}
impl CppEq for test_less_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_0(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for test_less_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_2.with(|rc| *rc.borrow()) >= dealloc_count_3.with(|rc| *rc.borrow())));
        return ((alloc_count_2.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_3.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_4.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_1.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_2.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_3.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_1: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_2: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_3: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_4: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_pair_const_int__double__ {}
impl CppMove for min_allocator_std_pair_const_int__double__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_5() -> bool {
    {
        let m: Value<
            std_multimap_int__double__test_less_int___std_allocator_std_pair_const_int__double___,
        > = Rc::new(RefCell::new({
            let __tmp_0: Value<test_less_int_> =
                Rc::new(RefCell::new(test_less_int_::new_1({ 3 })));
            std_multimap_int__double__test_less_int___std_allocator_std_pair_const_int__double___ :: new_1 ( { __tmp_0.as_pointer() } , None , )
        }));
        assert!(({ ({ (*m.borrow()).empty() }) }));
        assert!(({ (*m.borrow()).begin_21() }) == ({ (*m.borrow()).end_23() }));
        assert!(
            ({
                let _a: Value<test_less_int_> =
                    Rc::new(RefCell::new(({ (*m.borrow()).key_comp() })));
                let _b: Value<test_less_int_> = Rc::new(RefCell::new(test_less_int_::new_1({ 3 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
    }
    {
        let m: Value<
            std_multimap_int__double__test_less_int___min_allocator_std_pair_const_int__double___,
        > = Rc::new(RefCell::new({
            let __tmp_1: Value<test_less_int_> =
                Rc::new(RefCell::new(test_less_int_::new_1({ 3 })));
            std_multimap_int__double__test_less_int___min_allocator_std_pair_const_int__double___ :: new_1 ( { __tmp_1.as_pointer() } , None , )
        }));
        assert!(({ ({ (*m.borrow()).empty() }) }));
        assert!(({ (*m.borrow()).begin_21() }) == ({ (*m.borrow()).end_23() }));
        assert!(
            ({
                let _a: Value<test_less_int_> =
                    Rc::new(RefCell::new(({ (*m.borrow()).key_comp() })));
                let _b: Value<test_less_int_> = Rc::new(RefCell::new(test_less_int_::new_1({ 3 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
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
    ({ test_5() });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__;
pub trait min_allocator_std_pair_const_int__double__Impl {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_std_pair_const_int__double__Impl
    for Ptr<min_allocator_std_pair_const_int__double__>
{
}
pub trait test_less_int_Impl {
    fn operator_call(&self, a: Ptr<i32>, b: Ptr<i32>) -> bool {
        unimplemented!()
    }
}
impl test_less_int_Impl for Ptr<test_less_int_> {}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_1.with(|_| ());
    let _ = alloc_count_2.with(|_| ());
    let _ = dealloc_count_3.with(|_| ());
    let _ = disable_default_constructor_4.with(|_| ());
}
