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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_unsigned_long_ {}
impl CppMove for min_allocator_unsigned_long_ {
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct safe_allocator_unsigned_long_ {}
impl safe_allocator_unsigned_long_ {}
impl CppMove for safe_allocator_unsigned_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn is_contiguous_container_asan_correct_5(_a0: Ptr<Vec<i32>>) -> bool {
    return true;
}
pub fn is_contiguous_container_asan_correct_6(_a0: Ptr<Vec<Nasty>>) -> bool {
    return true;
}
pub fn is_contiguous_container_asan_correct_7(_a0: Ptr<Vec<i32>>) -> bool {
    return true;
}
pub fn is_contiguous_container_asan_correct_8(_a0: Ptr<Vec<Nasty>>) -> bool {
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
pub fn tests_9() -> bool {
    {
        let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        assert!((v.decay()).is_null());
        assert!(({ is_contiguous_container_asan_correct_10(v.as_pointer(),) }));
    }
    {
        let v: Value<Vec<i32>> = Rc::new(RefCell::new(
            (0..(100_usize) as usize).map(|_| 0_i32).collect::<Vec<_>>(),
        ));
        assert!((v.decay() == ({ addressof_11(v.decay(),) })));
        assert!(({ is_contiguous_container_asan_correct_10(v.as_pointer(),) }));
    }
    {
        let v: Value<Vec<Nasty>> = Rc::new(RefCell::new(
            (0..(100_usize) as usize)
                .map(|_| <Nasty>::default())
                .collect::<Vec<_>>(),
        ));
        assert!((v.decay() == ({ addressof_12(v.decay(),) })));
        assert!(({ is_contiguous_container_asan_correct_13(v.as_pointer(),) }));
    }
    {
        let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        assert!((v.decay()).is_null());
        assert!(({ is_contiguous_container_asan_correct_5(v.as_pointer(),) }));
    }
    {
        let v: Value<Vec<i32>> = Rc::new(RefCell::new(
            (0..(100_usize) as usize).map(|_| 0_i32).collect::<Vec<_>>(),
        ));
        assert!((v.decay() == ({ addressof_11(v.decay(),) })));
        assert!(({ is_contiguous_container_asan_correct_5(v.as_pointer(),) }));
    }
    {
        let v: Value<Vec<Nasty>> = Rc::new(RefCell::new(
            (0..(100_usize) as usize)
                .map(|_| <Nasty>::default())
                .collect::<Vec<_>>(),
        ));
        assert!((v.decay() == ({ addressof_12(v.decay(),) })));
        assert!(({ is_contiguous_container_asan_correct_6(v.as_pointer(),) }));
    }
    {
        let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        assert!((v.decay()).is_null());
        assert!(({ is_contiguous_container_asan_correct_7(v.as_pointer(),) }));
    }
    {
        let v: Value<Vec<i32>> = Rc::new(RefCell::new(
            (0..(100_usize) as usize).map(|_| 0_i32).collect::<Vec<_>>(),
        ));
        assert!((v.decay() == ({ addressof_11(v.decay(),) })));
        assert!(({ is_contiguous_container_asan_correct_7(v.as_pointer(),) }));
    }
    {
        let v: Value<Vec<Nasty>> = Rc::new(RefCell::new(
            (0..(100_usize) as usize)
                .map(|_| <Nasty>::default())
                .collect::<Vec<_>>(),
        ));
        assert!((v.decay() == ({ addressof_12(v.decay(),) })));
        assert!(({ is_contiguous_container_asan_correct_8(v.as_pointer(),) }));
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
    ({ tests_9() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__;
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
        assert!(false);
        return Ptr::<Nasty>::null();
    }
}
pub trait min_allocator_Nasty_Impl {
    fn allocate(&self, n: usize) -> min_pointer_Nasty__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_Nasty__std_integral_constant_unsigned_long__0__, n: usize);
}
impl min_allocator_Nasty_Impl for Ptr<min_allocator_Nasty_> {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_Nasty__std_integral_constant_unsigned_long__0__ {
        return min_pointer_Nasty__std_integral_constant_unsigned_long__0__::new_1({
            ({ <std_allocator_Nasty_>::default().allocate(n) })
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_Nasty__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        ({ <std_allocator_Nasty_>::default().deallocate((p.ptr_).clone(), n) });
    }
}
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
pub trait safe_allocator_Nasty_Impl {
    fn allocate(&self, n: usize) -> Ptr<Nasty>;
    fn deallocate(&self, p: Ptr<Nasty>, n: usize);
}
impl safe_allocator_Nasty_Impl for Ptr<safe_allocator_Nasty_> {
    fn allocate(&self, mut n: usize) -> Ptr<Nasty> {
        let memory: Value<Ptr<Nasty>> = Rc::new(RefCell::new(
            ({ <std_allocator_Nasty_>::default().allocate(n) }),
        ));
        {
            match ({ (*memory.borrow()).to_any() },) {
                (mut __tmp_2,) => {
                    __tmp_2.memset(
                        (0) as u8,
                        ((4usize as u64).wrapping_mul((n as u64)) as usize) as usize,
                    );
                    (__tmp_2).clone()
                }
            };
        }
        return (*memory.borrow()).clone();
    }
    fn deallocate(&self, mut p: Ptr<Nasty>, mut n: usize) {
        {
            ({
                let _value: Value<AnyPtr> = Rc::new(RefCell::new(match ({ (p).to_any() },) {
                    (mut __tmp_3,) => {
                        __tmp_3.memset(
                            (0) as u8,
                            ((4usize as u64).wrapping_mul((n as u64)) as usize) as usize,
                        );
                        (__tmp_3).clone()
                    }
                }));
                DoNotOptimize_0(_value.as_pointer())
            });
        }
        ({ <std_allocator_Nasty_>::default().deallocate((p).clone(), n) });
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
                DoNotOptimize_0(_value.as_pointer())
            });
        }
        ({ <std_allocator_int_>::default().deallocate((p).clone(), n) });
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
    let _ = outstanding_bytes_1.with(|_| ());
    let _ = alloc_count_2.with(|_| ());
    let _ = dealloc_count_3.with(|_| ());
    let _ = disable_default_constructor_4.with(|_| ());
}
