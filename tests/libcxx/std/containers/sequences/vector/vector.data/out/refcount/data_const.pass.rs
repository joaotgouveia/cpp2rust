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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> u64 {
        ({
            cpp2rust_assert_fail_5(
                (alloc_count_2.with(|rc| *rc.borrow()) >= dealloc_count_3.with(|rc| *rc.borrow())),
            )
        });
        return ((alloc_count_2.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_3.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        ({ cpp2rust_assert_fail_5((({ malloc_allocator_base::outstanding_alloc() }) == 0_u64)) });
        disable_default_constructor_4.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_1.with(|rc| *rc.borrow_mut() = 0_u64);
        alloc_count_2.with(|rc| *rc.borrow_mut() = 0_u64);
        dealloc_count_3.with(|rc| *rc.borrow_mut() = 0_u64);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_1: Value<u64> = Rc::new(RefCell::new(0_u64));
);
thread_local!(
    pub static alloc_count_2: Value<u64> = Rc::new(RefCell::new(0_u64));
);
thread_local!(
    pub static dealloc_count_3: Value<u64> = Rc::new(RefCell::new(0_u64));
);
thread_local!(
    pub static disable_default_constructor_4: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
impl CppMove for min_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_Nasty_ {}
impl CppMove for min_allocator_Nasty_ {
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct safe_allocator_Nasty_ {}
impl safe_allocator_Nasty_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl CppMove for safe_allocator_Nasty_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for safe_allocator_Nasty_ {
    fn default() -> Self {
        { safe_allocator_Nasty_::new() }
    }
}
pub fn is_contiguous_container_asan_correct_7(
    _a0: Ptr<std_vector_int__min_allocator_int__>,
) -> bool {
    return true;
}
pub fn is_contiguous_container_asan_correct_8(
    _a0: Ptr<std_vector_Nasty__min_allocator_Nasty__>,
) -> bool {
    return true;
}
pub fn is_contiguous_container_asan_correct_9(
    _a0: Ptr<std_vector_int__safe_allocator_int__>,
) -> bool {
    return true;
}
pub fn is_contiguous_container_asan_correct_10(
    _a0: Ptr<std_vector_Nasty__safe_allocator_Nasty__>,
) -> bool {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Destructor)]
#[cpp_from(i32, new_1, value)]
#[byte_size(4)]
pub struct Nasty {
    #[offset(0)]
    pub i_: i32,
}
impl Nasty {
    pub fn new_2() -> Self {
        Self { i_: 0 }
    }
    pub fn new_1(mut i: i32) -> Self {
        Self { i_: i }
    }
}
impl CppMove for Nasty {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for Nasty {
    fn default() -> Self {
        { Nasty::new_2() }
    }
}
pub fn tests_11() -> bool {
    {
        let v: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
            <std_vector_int__std_allocator_int__>::default(),
        ));
        ({ cpp2rust_assert_fail_5(({ (*v.borrow()).data_58() }).is_null()) });
        ({ cpp2rust_assert_fail_5(({ is_contiguous_container_asan_correct_12(v.as_pointer()) })) });
    }
    {
        let v: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
            std_vector_int__std_allocator_int__::new_1({ 100_u64 }, None),
        ));
        ({
            cpp2rust_assert_fail_5(
                (({ (*v.borrow()).data_58() })
                    == ({ addressof_13(({ (*v.borrow()).front_54() })) })),
            )
        });
        ({ cpp2rust_assert_fail_5(({ is_contiguous_container_asan_correct_12(v.as_pointer()) })) });
    }
    {
        let v: Value<std_vector_Nasty__std_allocator_Nasty__> = Rc::new(RefCell::new(
            std_vector_Nasty__std_allocator_Nasty__::new_1({ 100_u64 }, None),
        ));
        ({
            cpp2rust_assert_fail_5(
                (({ (*v.borrow()).data_58() })
                    == ({ addressof_14(({ (*v.borrow()).front_54() })) })),
            )
        });
        ({ cpp2rust_assert_fail_5(({ is_contiguous_container_asan_correct_15(v.as_pointer()) })) });
    }
    {
        let v: Value<std_vector_int__min_allocator_int__> = Rc::new(RefCell::new(
            <std_vector_int__min_allocator_int__>::default(),
        ));
        ({ cpp2rust_assert_fail_5(({ (*v.borrow()).data_58() }).is_null()) });
        ({ cpp2rust_assert_fail_5(({ is_contiguous_container_asan_correct_7(v.as_pointer()) })) });
    }
    {
        let v: Value<std_vector_int__min_allocator_int__> = Rc::new(RefCell::new(
            std_vector_int__min_allocator_int__::new_1({ 100_u64 }, None),
        ));
        ({
            cpp2rust_assert_fail_5(
                (({ (*v.borrow()).data_58() }) == ({ (*v.borrow()).front_54() })),
            )
        });
        ({ cpp2rust_assert_fail_5(({ is_contiguous_container_asan_correct_7(v.as_pointer()) })) });
    }
    {
        let v: Value<std_vector_Nasty__min_allocator_Nasty__> = Rc::new(RefCell::new(
            std_vector_Nasty__min_allocator_Nasty__::new_1({ 100_u64 }, None),
        ));
        ({
            cpp2rust_assert_fail_5(
                (({ (*v.borrow()).data_58() })
                    == ({ addressof_14(({ (*v.borrow()).front_54() })) })),
            )
        });
        ({ cpp2rust_assert_fail_5(({ is_contiguous_container_asan_correct_8(v.as_pointer()) })) });
    }
    {
        let v: Value<std_vector_int__safe_allocator_int__> = Rc::new(RefCell::new(
            <std_vector_int__safe_allocator_int__>::default(),
        ));
        ({ cpp2rust_assert_fail_5(({ (*v.borrow()).data_58() }).is_null()) });
        ({ cpp2rust_assert_fail_5(({ is_contiguous_container_asan_correct_9(v.as_pointer()) })) });
    }
    {
        let v: Value<std_vector_int__safe_allocator_int__> = Rc::new(RefCell::new(
            std_vector_int__safe_allocator_int__::new_1({ 100_u64 }, None),
        ));
        ({
            cpp2rust_assert_fail_5(
                (({ (*v.borrow()).data_58() }) == ({ (*v.borrow()).front_54() })),
            )
        });
        ({ cpp2rust_assert_fail_5(({ is_contiguous_container_asan_correct_9(v.as_pointer()) })) });
    }
    {
        let v: Value<std_vector_Nasty__safe_allocator_Nasty__> = Rc::new(RefCell::new(
            std_vector_Nasty__safe_allocator_Nasty__::new_1({ 100_u64 }, None),
        ));
        ({
            cpp2rust_assert_fail_5(
                (({ (*v.borrow()).data_58() })
                    == ({ addressof_14(({ (*v.borrow()).front_54() })) })),
            )
        });
        ({ cpp2rust_assert_fail_5(({ is_contiguous_container_asan_correct_10(v.as_pointer()) })) });
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
    ({ tests_11() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_Nasty__std_integral_constant_unsigned_long__0__;
pub trait NastyImpl {
    fn destructor(&self);
    fn operator_addr(&self) -> Ptr<Nasty>;
}
impl NastyImpl for Ptr<Nasty> {
    fn destructor(&self) {}
    fn operator_addr(&self) -> Ptr<Nasty> {
        ({ cpp2rust_assert_fail_5(false) });
        return Ptr::<Nasty>::null();
    }
}
pub trait min_allocator_Nasty_Impl {
    fn allocate(&self, n: u64) -> min_pointer_Nasty__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_Nasty__std_integral_constant_unsigned_long__0__, n: u64);
}
impl min_allocator_Nasty_Impl for Ptr<min_allocator_Nasty_> {
    fn allocate(&self, mut n: u64) -> min_pointer_Nasty__std_integral_constant_unsigned_long__0__ {
        return min_pointer_Nasty__std_integral_constant_unsigned_long__0__::new_1({
            ({ <std_allocator_Nasty_>::default().allocate(n) })
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_Nasty__std_integral_constant_unsigned_long__0__,
        mut n: u64,
    ) {
        ({ <std_allocator_Nasty_>::default().deallocate((p.ptr_).clone(), n) });
    }
}
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
pub trait safe_allocator_Nasty_Impl {
    fn allocate(&self, n: u64) -> Ptr<Nasty>;
    fn deallocate(&self, p: Ptr<Nasty>, n: u64);
}
impl safe_allocator_Nasty_Impl for Ptr<safe_allocator_Nasty_> {
    fn allocate(&self, mut n: u64) -> Ptr<Nasty> {
        let memory: Value<Ptr<Nasty>> = Rc::new(RefCell::new(
            ({ <std_allocator_Nasty_>::default().allocate(n) }),
        ));
        {
            ({
                memset_6(
                    (*memory.borrow()).to_any(),
                    0,
                    (4usize as u64).wrapping_mul(n),
                )
            });
        }
        return (*memory.borrow()).clone();
    }
    fn deallocate(&self, mut p: Ptr<Nasty>, mut n: u64) {
        {
            ({
                let _value: Value<AnyPtr> = Rc::new(RefCell::new(
                    ({ memset_6((p).to_any(), 0, (4usize as u64).wrapping_mul(n)) }),
                ));
                DoNotOptimize_0(_value.as_pointer())
            });
        }
        ({ <std_allocator_Nasty_>::default().deallocate((p).clone(), n) });
    }
}
pub trait safe_allocator_int_Impl {
    fn allocate(&self, n: u64) -> Ptr<i32>;
    fn deallocate(&self, p: Ptr<i32>, n: u64);
}
impl safe_allocator_int_Impl for Ptr<safe_allocator_int_> {
    fn allocate(&self, mut n: u64) -> Ptr<i32> {
        let memory: Value<Ptr<i32>> = Rc::new(RefCell::new(
            ({ <std_allocator_int_>::default().allocate(n) }),
        ));
        {
            ({
                let ___s: AnyPtr = (*memory.borrow()).to_any();
                memset_6(
                    ___s,
                    0,
                    (::std::mem::size_of::<i32>() as u64).wrapping_mul(n),
                )
            });
        }
        return (*memory.borrow()).clone();
    }
    fn deallocate(&self, mut p: Ptr<i32>, mut n: u64) {
        {
            ({
                let _value: Value<AnyPtr> = Rc::new(RefCell::new(
                    ({
                        let ___s: AnyPtr = (p).to_any();
                        memset_6(
                            ___s,
                            0,
                            (::std::mem::size_of::<i32>() as u64).wrapping_mul(n),
                        )
                    }),
                ));
                DoNotOptimize_0(_value.as_pointer())
            });
        }
        ({ <std_allocator_int_>::default().deallocate((p).clone(), n) });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_1.with(|_| ());
    let _ = alloc_count_2.with(|_| ());
    let _ = dealloc_count_3.with(|_| ());
    let _ = disable_default_constructor_4.with(|_| ());
}
