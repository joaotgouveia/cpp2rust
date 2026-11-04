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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> u64 {
        ({
            cpp2rust_assert_fail_6(
                (alloc_count_3.with(|rc| *rc.borrow()) >= dealloc_count_4.with(|rc| *rc.borrow())),
            )
        });
        return ((alloc_count_3.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_4.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        ({ cpp2rust_assert_fail_6((({ malloc_allocator_base::outstanding_alloc() }) == 0_u64)) });
        disable_default_constructor_5.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_2.with(|rc| *rc.borrow_mut() = 0_u64);
        alloc_count_3.with(|rc| *rc.borrow_mut() = 0_u64);
        dealloc_count_4.with(|rc| *rc.borrow_mut() = 0_u64);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_2: Value<u64> = Rc::new(RefCell::new(0_u64));
);
thread_local!(
    pub static alloc_count_3: Value<u64> = Rc::new(RefCell::new(0_u64));
);
thread_local!(
    pub static dealloc_count_4: Value<u64> = Rc::new(RefCell::new(0_u64));
);
thread_local!(
    pub static disable_default_constructor_5: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
impl CppMove for min_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
    pub fn new_1(mut _a0: min_allocator_int_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(min_allocator_int_, new_1, value)]
#[byte_size(1)]
pub struct min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
impl CppMove for min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
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
        let c: Value<std_deque_int__std_allocator_int__> =
            Rc::new(RefCell::new(({ make_7(10, None) })));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        let mut i: i32 = 0;
        'loop_: while (i < 10) {
            ({ cpp2rust_assert_fail_6(({ (*c.borrow())[(i as u64)] } == i)) });
            i.prefix_inc();
        }
        let mut i: i32 = 0;
        'loop_: while (i < 10) {
            ({ cpp2rust_assert_fail_6(((({ (*c.borrow()).at_45((i as u64)) }).read()) == i)) });
            i.prefix_inc();
        }
        ({ cpp2rust_assert_fail_6(((({ (*c.borrow()).front_47() }).read()) == 0)) });
        ({ cpp2rust_assert_fail_6(((({ (*c.borrow()).back_49() }).read()) == 9)) });
        const _: () = assert!(true, "");;
    }
    {
        let c: Value<std_deque_int__std_allocator_int__> =
            Rc::new(RefCell::new(({ make_7(10, None) })));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        let mut i: i32 = 0;
        'loop_: while (i < 10) {
            ({ cpp2rust_assert_fail_6(({ (*c.borrow())[(i as u64)] } == i)) });
            i.prefix_inc();
        }
        let mut i: i32 = 0;
        'loop_: while (i < 10) {
            ({ cpp2rust_assert_fail_6(((({ (*c.borrow()).at_46((i as u64)) }).read()) == i)) });
            i.prefix_inc();
        }
        ({ cpp2rust_assert_fail_6(((({ (*c.borrow()).front_48() }).read()) == 0)) });
        ({ cpp2rust_assert_fail_6(((({ (*c.borrow()).back_50() }).read()) == 9)) });
        const _: () = assert!(true, "");;
    }
    {
        let c: Value<std_deque_int__min_allocator_int__> =
            Rc::new(RefCell::new(({ make_8(10, None) })));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        let mut i: i32 = 0;
        'loop_: while (i < 10) {
            ({ cpp2rust_assert_fail_6(({ (*c.borrow())[(i as u64)] } == i)) });
            i.prefix_inc();
        }
        let mut i: i32 = 0;
        'loop_: while (i < 10) {
            ({ cpp2rust_assert_fail_6(((({ (*c.borrow()).at_45((i as u64)) }).read()) == i)) });
            i.prefix_inc();
        }
        ({ cpp2rust_assert_fail_6(((({ (*c.borrow()).front_47() }).read()) == 0)) });
        ({ cpp2rust_assert_fail_6(((({ (*c.borrow()).back_49() }).read()) == 9)) });
        const _: () = assert!(true, "");;
    }
    {
        let c: Value<std_deque_int__min_allocator_int__> =
            Rc::new(RefCell::new(({ make_8(10, None) })));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        let mut i: i32 = 0;
        'loop_: while (i < 10) {
            ({ cpp2rust_assert_fail_6(({ (*c.borrow())[(i as u64)] } == i)) });
            i.prefix_inc();
        }
        let mut i: i32 = 0;
        'loop_: while (i < 10) {
            ({ cpp2rust_assert_fail_6(((({ (*c.borrow()).at_46((i as u64)) }).read()) == i)) });
            i.prefix_inc();
        }
        ({ cpp2rust_assert_fail_6(((({ (*c.borrow()).front_48() }).read()) == 0)) });
        ({ cpp2rust_assert_fail_6(((({ (*c.borrow()).back_50() }).read()) == 9)) });
        const _: () = assert!(true, "");;
    }
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
pub trait min_allocator_int_Impl {
    fn allocate(&self, n: u64) -> min_pointer_int__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_int__std_integral_constant_unsigned_long__0__, n: u64);
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {
    fn allocate(&self, mut n: u64) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_1({
            ({ <std_allocator_int_>::default().allocate(n) })
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: u64,
    ) {
        ({ <std_allocator_int_>::default().deallocate((p.ptr_).clone(), n) });
    }
}
pub trait min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___Impl {
    fn allocate ( &self , n : u64  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn deallocate(
        &self,
        p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        n: u64,
    );
}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___Impl
    for Ptr<min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___>
{
    fn allocate ( &self , mut n : u64  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { ( { <std_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___  >::default() . allocate ( n  , ) } )   } , )    ;
    }
    fn deallocate(
        &self,
        mut p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: u64,
    ) {
        ({
            <std_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___>::default()
                .deallocate((p.ptr_).clone(), n)
        });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_2.with(|_| ());
    let _ = alloc_count_3.with(|_| ());
    let _ = dealloc_count_4.with(|_| ());
    let _ = disable_default_constructor_5.with(|_| ());
}
