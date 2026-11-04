extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> u64 {
        ({
            cpp2rust_assert_fail_4(
                (alloc_count_1.with(|rc| *rc.borrow()) >= dealloc_count_2.with(|rc| *rc.borrow())),
            )
        });
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        ({ cpp2rust_assert_fail_4((({ malloc_allocator_base::outstanding_alloc() }) == 0_u64)) });
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_u64);
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_u64);
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_u64);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<u64> = Rc::new(RefCell::new(0_u64));
);
thread_local!(
    pub static alloc_count_1: Value<u64> = Rc::new(RefCell::new(0_u64));
);
thread_local!(
    pub static dealloc_count_2: Value<u64> = Rc::new(RefCell::new(0_u64));
);
thread_local!(
    pub static disable_default_constructor_3: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoDestroy_CountDestructor_ {}
impl CppMove for NoDestroy_CountDestructor_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoDestroy_Holder_NS_Incomplete__ptr_ {}
impl CppMove for NoDestroy_Holder_NS_Incomplete__ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CountDestroy_CountDestructor_ {
    #[offset(0)]
    #[byte_size(8)]
    pub counter_: Ptr<i32>,
}
impl CountDestroy_CountDestructor_ {
    pub fn new(mut counter: Ptr<i32>) -> Self {
        Self {
            counter_: (counter).clone(),
        }
    }
}
impl CppMove for CountDestroy_CountDestructor_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct CountDestructor {
    #[offset(0)]
    #[byte_size(8)]
    pub counter_: Ptr<i32>,
}
impl CountDestructor {
    pub fn new(mut counter: Ptr<i32>) -> Self {
        Self {
            counter_: (counter).clone(),
        }
    }
}
impl CppMove for CountDestructor {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_5() -> bool {
    {
        let destructors: Value<i32> = Rc::new(RefCell::new(0));
        let alloc: Value<NoDestroy_CountDestructor_> =
            Rc::new(RefCell::new(<NoDestroy_CountDestructor_>::default()));
        let mut pool: Ptr<CountDestructor> = ({
            std_allocator_traits_NoDestroy_CountDestructor__::allocate_1(alloc.as_pointer(), 1_u64)
        });
        ({
            let ___args: Value<Ptr<i32>> = Rc::new(RefCell::new((destructors.as_pointer())));
            std_allocator_traits_NoDestroy_CountDestructor__::construct(
                alloc.as_pointer(),
                (pool).clone(),
                ___args.as_pointer(),
            )
        });
        ({ cpp2rust_assert_fail_4(((*destructors.borrow()) == 0)) });
        ({
            std_allocator_traits_NoDestroy_CountDestructor__::destroy(
                alloc.as_pointer(),
                (pool).clone(),
            )
        });
        ({ cpp2rust_assert_fail_4(((*destructors.borrow()) == 1)) });
        ({
            std_allocator_traits_NoDestroy_CountDestructor__::deallocate(
                alloc.as_pointer(),
                (pool).clone(),
                1_u64,
            )
        });
    }
    {
        let alloc: Value<NoDestroy_Holder_NS_Incomplete__ptr_> = Rc::new(RefCell::new(
            <NoDestroy_Holder_NS_Incomplete__ptr_>::default(),
        ));
        let mut pool: Ptr<Ptr<Holder_NS_Incomplete_>> = ({
            std_allocator_traits_NoDestroy_Holder_NS_Incomplete__ptr__::allocate_1(
                alloc.as_pointer(),
                1_u64,
            )
        });
        ({
            let ___args: Value<AnyPtr> = Rc::new(RefCell::new(Default::default()));
            std_allocator_traits_NoDestroy_Holder_NS_Incomplete__ptr__::construct(
                alloc.as_pointer(),
                (pool).clone(),
                ___args.as_pointer(),
            )
        });
        ({
            std_allocator_traits_NoDestroy_Holder_NS_Incomplete__ptr__::destroy(
                alloc.as_pointer(),
                (pool).clone(),
            )
        });
        ({
            std_allocator_traits_NoDestroy_Holder_NS_Incomplete__ptr__::deallocate(
                alloc.as_pointer(),
                (pool).clone(),
                1_u64,
            )
        });
    }
    {
        let destroys_called: Value<i32> = Rc::new(RefCell::new(0));
        let destructors_called: Value<i32> = Rc::new(RefCell::new(0));
        let alloc: Value<CountDestroy_CountDestructor_> =
            Rc::new(RefCell::new(CountDestroy_CountDestructor_::new({
                (destroys_called.as_pointer())
            })));
        let mut pool: Ptr<CountDestructor> = ({
            std_allocator_traits_CountDestroy_CountDestructor__::allocate_1(
                alloc.as_pointer(),
                1_u64,
            )
        });
        ({
            let ___args: Value<Ptr<i32>> = Rc::new(RefCell::new((destructors_called.as_pointer())));
            std_allocator_traits_CountDestroy_CountDestructor__::construct(
                alloc.as_pointer(),
                (pool).clone(),
                ___args.as_pointer(),
            )
        });
        ({ cpp2rust_assert_fail_4(((*destroys_called.borrow()) == 0)) });
        ({ cpp2rust_assert_fail_4(((*destructors_called.borrow()) == 0)) });
        ({
            std_allocator_traits_CountDestroy_CountDestructor__::destroy(
                alloc.as_pointer(),
                (pool).clone(),
            )
        });
        ({ cpp2rust_assert_fail_4(((*destroys_called.borrow()) == 1)) });
        ({ cpp2rust_assert_fail_4(((*destructors_called.borrow()) == 1)) });
        ({
            std_allocator_traits_CountDestroy_CountDestructor__::deallocate(
                alloc.as_pointer(),
                (pool).clone(),
                1_u64,
            )
        });
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
    const _: () = assert!(true, "test()");;
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct Holder_NS_Incomplete_;
pub trait CountDestroy_CountDestructor_Impl {
    fn allocate(&self, n: u64) -> Ptr<CountDestructor>;
    fn deallocate(&self, p: Ptr<CountDestructor>, n: u64);
    fn destroy_4_struct_CountDestructor(&self, p: Ptr<CountDestructor>);
}
impl CountDestroy_CountDestructor_Impl for Ptr<CountDestroy_CountDestructor_> {
    fn allocate(&self, mut n: u64) -> Ptr<CountDestructor> {
        return ({ <std_allocator_CountDestructor_>::default().allocate(n) });
    }
    fn deallocate(&self, mut p: Ptr<CountDestructor>, mut n: u64) {
        ({ <std_allocator_CountDestructor_>::default().deallocate(p, n) });
        return;
    }
    fn destroy_4_struct_CountDestructor(&self, mut p: Ptr<CountDestructor>) {
        (*self).with(|__s| __s.counter_.clone()).with_mut(|__v| {
            __v.prefix_inc();
        });
        ({ CountDestructorImpl::destructor(&p) });
    }
}
pub trait CountDestructorImpl {
    fn destructor(&self);
}
impl CountDestructorImpl for Ptr<CountDestructor> {
    fn destructor(&self) {
        (*self).with(|__s| __s.counter_.clone()).with_mut(|__v| {
            __v.prefix_inc();
        });
    }
}
pub trait NoDestroy_CountDestructor_Impl {
    fn allocate(&self, n: u64) -> Ptr<CountDestructor>;
    fn deallocate(&self, p: Ptr<CountDestructor>, n: u64);
}
impl NoDestroy_CountDestructor_Impl for Ptr<NoDestroy_CountDestructor_> {
    fn allocate(&self, mut n: u64) -> Ptr<CountDestructor> {
        return ({ <std_allocator_CountDestructor_>::default().allocate(n) });
    }
    fn deallocate(&self, mut p: Ptr<CountDestructor>, mut n: u64) {
        ({ <std_allocator_CountDestructor_>::default().deallocate(p, n) });
        return;
    }
}
pub trait NoDestroy_Holder_NS_Incomplete__ptr_Impl {
    fn allocate(&self, n: u64) -> Ptr<Ptr<Holder_NS_Incomplete_>>;
    fn deallocate(&self, p: Ptr<Ptr<Holder_NS_Incomplete_>>, n: u64);
}
impl NoDestroy_Holder_NS_Incomplete__ptr_Impl for Ptr<NoDestroy_Holder_NS_Incomplete__ptr_> {
    fn allocate(&self, mut n: u64) -> Ptr<Ptr<Holder_NS_Incomplete_>> {
        return ({ <std_allocator_Holder_NS_Incomplete__ptr_>::default().allocate(n) });
    }
    fn deallocate(&self, mut p: Ptr<Ptr<Holder_NS_Incomplete_>>, mut n: u64) {
        ({ <std_allocator_Holder_NS_Incomplete__ptr_>::default().deallocate(p, n) });
        return;
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
