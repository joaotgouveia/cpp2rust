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
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(16)]
pub struct Emplaceable {
    #[offset(0)]
    int_: i32,
    #[offset(8)]
    double_: f64,
}
impl Emplaceable {
    pub fn new_1() -> Self {
        Self {
            int_: 0,
            double_: 0_f64,
        }
    }
    pub fn new_2(mut i: i32, mut d: f64) -> Self {
        Self {
            int_: i,
            double_: d,
        }
    }
    pub fn move_from(x: Ptr<Emplaceable>) -> Self {
        let __this: Emplaceable = Self {
            int_: x.with(|__s| __s.int_),
            double_: x.with(|__s| __s.double_),
        };
        field!(x, int_).write(0);
        field!(x, double_).write(0_f64);
        __this
    }
}
impl CppLess for Emplaceable {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        EmplaceableImpl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for Emplaceable {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        EmplaceableImpl::operator_eq(__lhs, __rhs.clone())
    }
}
impl CppMove for Emplaceable {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for Emplaceable {
    fn default() -> Self {
        { Emplaceable::new_1() }
    }
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
pub struct min_allocator_Emplaceable_ {}
impl CppMove for min_allocator_Emplaceable_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl min_allocator_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0___ {
    pub fn new_1(mut _a0: min_allocator_Emplaceable_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(min_allocator_Emplaceable_, new_1, value)]
#[byte_size(1)]
pub struct min_allocator_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0___ {}
impl CppMove for min_allocator_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0___ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl min_allocator_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0___ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct safe_allocator_Emplaceable_ {}
impl safe_allocator_Emplaceable_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl CppMove for safe_allocator_Emplaceable_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for safe_allocator_Emplaceable_ {
    fn default() -> Self {
        { safe_allocator_Emplaceable_::new() }
    }
}
impl safe_allocator_Emplaceable_ptr_ {
    pub fn new_1(mut _a0: safe_allocator_Emplaceable_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct safe_allocator_Emplaceable_ptr_ {}
impl safe_allocator_Emplaceable_ptr_ {}
impl CppMove for safe_allocator_Emplaceable_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
        let mut N: i32 = (((::std::mem::size_of::<[i32; 10]>() as u64)
            .wrapping_div((::std::mem::size_of::<i32>() as u64))) as i32);
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            let mut j: i32 = 0;
            'loop_: while (j < N) {
                ({
                    let _start: i32 = rng[(i) as usize];
                    let _N: i32 = rng[(j) as usize];
                    testN_7(_start, _N)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let mut rng: [i32; 10] = [0, 1, 2, 3, 1023, 1024, 1025, 2047, 2048, 2049];
        let mut N: i32 = (((::std::mem::size_of::<[i32; 10]>() as u64)
            .wrapping_div((::std::mem::size_of::<i32>() as u64))) as i32);
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            let mut j: i32 = 0;
            'loop_: while (j < N) {
                ({
                    let _start: i32 = rng[(i) as usize];
                    let _N: i32 = rng[(j) as usize];
                    testN_8(_start, _N)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let mut rng: [i32; 10] = [0, 1, 2, 3, 1023, 1024, 1025, 2047, 2048, 2049];
        let mut N: i32 = (((::std::mem::size_of::<[i32; 10]>() as u64)
            .wrapping_div((::std::mem::size_of::<i32>() as u64))) as i32);
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            let mut j: i32 = 0;
            'loop_: while (j < N) {
                ({
                    let _start: i32 = rng[(i) as usize];
                    let _N: i32 = rng[(j) as usize];
                    testN_9(_start, _N)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__;
pub trait EmplaceableImpl {
    fn copy_assign(&self, _a0: Ptr<Emplaceable>) -> Ptr<Emplaceable> {
        unimplemented!()
    }
    fn move_assign(&self, x: Ptr<Emplaceable>) -> Ptr<Emplaceable>;
    fn operator_eq(&self, x: Ptr<Emplaceable>) -> bool;
    fn operator_lt(&self, x: Ptr<Emplaceable>) -> bool;
    fn get(&self) -> i32;
}
impl EmplaceableImpl for Ptr<Emplaceable> {
    fn move_assign(&self, x: Ptr<Emplaceable>) -> Ptr<Emplaceable> {
        field!((*self), int_).write({ x.with(|__s| __s.int_) });
        field!(x, int_).write(0);
        field!((*self), double_).write({ x.with(|__s| __s.double_) });
        field!(x, double_).write(0_f64);
        return (*self).clone();
    }
    fn operator_eq(&self, x: Ptr<Emplaceable>) -> bool {
        return ({ (*self).with(|__s| __s.int_) } == { x.with(|__s| __s.int_) })
            && ({ (*self).with(|__s| __s.double_) } == { x.with(|__s| __s.double_) });
    }
    fn operator_lt(&self, x: Ptr<Emplaceable>) -> bool {
        return ({ (*self).with(|__s| __s.int_) } < { x.with(|__s| __s.int_) })
            || (({ (*self).with(|__s| __s.int_) } == { x.with(|__s| __s.int_) })
                && ({ (*self).with(|__s| __s.double_) } < { x.with(|__s| __s.double_) }));
    }
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.int_);
    }
}
pub trait min_allocator_Emplaceable_Impl {
    fn allocate(&self, n: u64)
    -> min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__;
    fn deallocate(
        &self,
        p: min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__,
        n: u64,
    );
}
impl min_allocator_Emplaceable_Impl for Ptr<min_allocator_Emplaceable_> {
    fn allocate(
        &self,
        mut n: u64,
    ) -> min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ {
        return min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__::new_1({
            ({ <std_allocator_Emplaceable_>::default().allocate(n) })
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__,
        mut n: u64,
    ) {
        ({ <std_allocator_Emplaceable_>::default().deallocate((p.ptr_).clone(), n) });
    }
}
pub trait min_allocator_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0___Impl {
    fn allocate ( &self , n : u64  , ) -> min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn deallocate(
        &self,
        p : min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        n: u64,
    );
}
impl min_allocator_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0___Impl
    for Ptr<min_allocator_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0___>
{
    fn allocate ( &self , mut n : u64  , ) -> min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { ( { <std_allocator_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0___  >::default() . allocate ( n  , ) } )   } , )    ;
    }
    fn deallocate(
        &self,
        mut p : min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: u64,
    ) {
        ({
            <std_allocator_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0___  >::default() . deallocate ( (p . ptr_  ).clone() , n  , )
        });
    }
}
pub trait safe_allocator_Emplaceable_Impl {
    fn allocate(&self, n: u64) -> Ptr<Emplaceable>;
    fn deallocate(&self, p: Ptr<Emplaceable>, n: u64);
}
impl safe_allocator_Emplaceable_Impl for Ptr<safe_allocator_Emplaceable_> {
    fn allocate(&self, mut n: u64) -> Ptr<Emplaceable> {
        let memory: Value<Ptr<Emplaceable>> = Rc::new(RefCell::new(
            ({ <std_allocator_Emplaceable_>::default().allocate(n) }),
        ));
        {
            ({
                memset_6(
                    (*memory.borrow()).to_any(),
                    0,
                    (16usize as u64).wrapping_mul(n),
                )
            });
        }
        return (*memory.borrow()).clone();
    }
    fn deallocate(&self, mut p: Ptr<Emplaceable>, mut n: u64) {
        {
            ({
                let _value: Value<AnyPtr> = Rc::new(RefCell::new(
                    ({ memset_6((p).to_any(), 0, (16usize as u64).wrapping_mul(n)) }),
                ));
                DoNotOptimize_0(_value.as_pointer())
            });
        }
        ({ <std_allocator_Emplaceable_>::default().deallocate((p).clone(), n) });
    }
}
pub trait safe_allocator_Emplaceable_ptr_Impl {
    fn allocate(&self, n: u64) -> Ptr<Ptr<Emplaceable>>;
    fn deallocate(&self, p: Ptr<Ptr<Emplaceable>>, n: u64);
}
impl safe_allocator_Emplaceable_ptr_Impl for Ptr<safe_allocator_Emplaceable_ptr_> {
    fn allocate(&self, mut n: u64) -> Ptr<Ptr<Emplaceable>> {
        let memory: Value<Ptr<Ptr<Emplaceable>>> = Rc::new(RefCell::new(
            ({ <std_allocator_Emplaceable_ptr_>::default().allocate(n) }),
        ));
        {
            ({
                memset_6(
                    (*memory.borrow()).to_any(),
                    0,
                    (8usize as u64).wrapping_mul(n),
                )
            });
        }
        return (*memory.borrow()).clone();
    }
    fn deallocate(&self, mut p: Ptr<Ptr<Emplaceable>>, mut n: u64) {
        {
            ({
                let _value: Value<AnyPtr> = Rc::new(RefCell::new(
                    ({ memset_6((p).to_any(), 0, (8usize as u64).wrapping_mul(n)) }),
                ));
                DoNotOptimize_0(_value.as_pointer())
            });
        }
        ({ <std_allocator_Emplaceable_ptr_>::default().deallocate((p).clone(), n) });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_1.with(|_| ());
    let _ = alloc_count_2.with(|_| ());
    let _ = dealloc_count_3.with(|_| ());
    let _ = disable_default_constructor_4.with(|_| ());
}
