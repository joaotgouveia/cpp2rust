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
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_1.with(|rc| *rc.borrow()) >= dealloc_count_2.with(|rc| *rc.borrow())));
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_1: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_2: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_3: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A_A0_ {}
impl CppMove for A_A0_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A_A1_ {}
impl CppMove for A_A1_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A_A2_ {}
impl CppMove for A_A2_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A_Holder_NS_Incomplete__ptr_ {}
impl CppMove for A_Holder_NS_Incomplete__ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, mut_ptr)]
#[byte_size(8)]
pub struct B_A0_ {
    #[offset(0)]
    #[byte_size(8)]
    pub count_: Ptr<i32>,
}
impl B_A0_ {
    pub fn new(count: Ptr<i32>) -> Self {
        Self {
            count_: (count).clone(),
        }
    }
}
impl CppMove for B_A0_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, mut_ptr)]
#[byte_size(8)]
pub struct B_A1_ {
    #[offset(0)]
    #[byte_size(8)]
    pub count_: Ptr<i32>,
}
impl B_A1_ {
    pub fn new(count: Ptr<i32>) -> Self {
        Self {
            count_: (count).clone(),
        }
    }
}
impl CppMove for B_A1_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, mut_ptr)]
#[byte_size(8)]
pub struct B_A2_ {
    #[offset(0)]
    #[byte_size(8)]
    pub count_: Ptr<i32>,
}
impl B_A2_ {
    pub fn new(count: Ptr<i32>) -> Self {
        Self {
            count_: (count).clone(),
        }
    }
}
impl CppMove for B_A2_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Ptr< i32  >  , new, value)]
#[byte_size(1)]
pub struct A0 {}
impl A0 {
    pub fn new(mut count: Ptr<i32>) -> Self {
        let __this: A0 = Self {};
        count.with_mut(|__v| {
            __v.prefix_inc();
        });
        __this
    }
}
impl CppMove for A0 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A1 {}
impl A1 {
    pub fn new(mut count: Ptr<i32>, mut c: i8) -> Self {
        let __this: A1 = Self {};
        assert!(((c as i32) == (('c' as i8) as i32)));
        count.with_mut(|__v| {
            __v.prefix_inc();
        });
        __this
    }
}
impl CppMove for A1 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A2 {}
impl A2 {
    pub fn new(mut count: Ptr<i32>, mut c: i8, mut i: i32) -> Self {
        let __this: A2 = Self {};
        assert!(((c as i32) == (('d' as i8) as i32)));
        assert!((i == 5));
        count.with_mut(|__v| {
            __v.prefix_inc();
        });
        __this
    }
}
impl CppMove for A2 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_7() -> bool {
    {
        let A0_count: Value<i32> = Rc::new(RefCell::new(0));
        let a: Value<A_A0_> = Rc::new(RefCell::new(<A_A0_>::default()));
        let alloc: Value<std_allocator_A0_> = Rc::new(RefCell::new(<std_allocator_A0_>::default()));
        let mut a0: Ptr<A0> = ({ (*alloc.borrow()).allocate(1_usize) });
        assert!(({ ((*A0_count.borrow()) == 0) }));
        ({
            let ___args: Value<Ptr<i32>> = Rc::new(RefCell::new((A0_count.as_pointer())));
            std_allocator_traits_A_A0__::construct(
                a.as_pointer(),
                (a0).clone(),
                ___args.as_pointer(),
            )
        });
        assert!(({ ((*A0_count.borrow()) == 1) }));
        ({ (*alloc.borrow()).deallocate((a0).clone(), 1_usize) });
    }
    {
        let A1_count: Value<i32> = Rc::new(RefCell::new(0));
        let a: Value<A_A1_> = Rc::new(RefCell::new(<A_A1_>::default()));
        let alloc: Value<std_allocator_A1_> = Rc::new(RefCell::new(<std_allocator_A1_>::default()));
        let mut a1: Ptr<A1> = ({ (*alloc.borrow()).allocate(1_usize) });
        assert!(({ ((*A1_count.borrow()) == 0) }));
        ({
            let ___args_2: Value<Ptr<i32>> = Rc::new(RefCell::new((A1_count.as_pointer())));
            let ___args_3: Value<i8> = Rc::new(RefCell::new(('c' as i8)));
            std_allocator_traits_A_A1__::construct(
                a.as_pointer(),
                (a1).clone(),
                ___args_2.as_pointer(),
                ___args_3.as_pointer(),
            )
        });
        assert!(({ ((*A1_count.borrow()) == 1) }));
        ({ (*alloc.borrow()).deallocate((a1).clone(), 1_usize) });
    }
    {
        let A2_count: Value<i32> = Rc::new(RefCell::new(0));
        let a: Value<A_A2_> = Rc::new(RefCell::new(<A_A2_>::default()));
        let alloc: Value<std_allocator_A2_> = Rc::new(RefCell::new(<std_allocator_A2_>::default()));
        let mut a2: Ptr<A2> = ({ (*alloc.borrow()).allocate(1_usize) });
        assert!(({ ((*A2_count.borrow()) == 0) }));
        ({
            let ___args_2: Value<Ptr<i32>> = Rc::new(RefCell::new((A2_count.as_pointer())));
            let ___args_3: Value<i8> = Rc::new(RefCell::new(('d' as i8)));
            let ___args_4: Value<i32> = Rc::new(RefCell::new(5));
            std_allocator_traits_A_A2__::construct(
                a.as_pointer(),
                (a2).clone(),
                ___args_2.as_pointer(),
                ___args_3.as_pointer(),
                ___args_4.as_pointer(),
            )
        });
        assert!(({ ((*A2_count.borrow()) == 1) }));
        ({ (*alloc.borrow()).deallocate((a2).clone(), 1_usize) });
    }
    {
        let a: Value<A_Holder_NS_Incomplete__ptr_> =
            Rc::new(RefCell::new(<A_Holder_NS_Incomplete__ptr_>::default()));
        let alloc: Value<std_allocator_Holder_NS_Incomplete__ptr_> = Rc::new(RefCell::new(
            <std_allocator_Holder_NS_Incomplete__ptr_>::default(),
        ));
        let mut vt: Ptr<Ptr<Holder_NS_Incomplete_>> = ({ (*alloc.borrow()).allocate(1_usize) });
        ({
            let ___args: Value<AnyPtr> = Rc::new(RefCell::new(Default::default()));
            std_allocator_traits_A_Holder_NS_Incomplete__ptr__::construct(
                a.as_pointer(),
                (vt).clone(),
                ___args.as_pointer(),
            )
        });
        ({ (*alloc.borrow()).deallocate((vt).clone(), 1_usize) });
    }
    {
        let A0_count: Value<i32> = Rc::new(RefCell::new(0));
        let b_construct: Value<i32> = Rc::new(RefCell::new(0));
        let b: Value<B_A0_> = Rc::new(RefCell::new(B_A0_::new({ b_construct.as_pointer() })));
        let alloc: Value<std_allocator_A0_> = Rc::new(RefCell::new(<std_allocator_A0_>::default()));
        let mut a0: Ptr<A0> = ({ (*alloc.borrow()).allocate(1_usize) });
        assert!(({ ((*A0_count.borrow()) == 0) }));
        assert!(({ ((*b_construct.borrow()) == 0) }));
        ({
            let ___args: Value<Ptr<i32>> = Rc::new(RefCell::new((A0_count.as_pointer())));
            std_allocator_traits_B_A0__::construct(
                b.as_pointer(),
                (a0).clone(),
                ___args.as_pointer(),
            )
        });
        assert!(({ ((*A0_count.borrow()) == 1) }));
        assert!(({ ((*b_construct.borrow()) == 1) }));
        ({ (*alloc.borrow()).deallocate((a0).clone(), 1_usize) });
    }
    {
        let A1_count: Value<i32> = Rc::new(RefCell::new(0));
        let b_construct: Value<i32> = Rc::new(RefCell::new(0));
        let b: Value<B_A1_> = Rc::new(RefCell::new(B_A1_::new({ b_construct.as_pointer() })));
        let alloc: Value<std_allocator_A1_> = Rc::new(RefCell::new(<std_allocator_A1_>::default()));
        let mut a1: Ptr<A1> = ({ (*alloc.borrow()).allocate(1_usize) });
        assert!(({ ((*A1_count.borrow()) == 0) }));
        assert!(({ ((*b_construct.borrow()) == 0) }));
        ({
            let ___args_2: Value<Ptr<i32>> = Rc::new(RefCell::new((A1_count.as_pointer())));
            let ___args_3: Value<i8> = Rc::new(RefCell::new(('c' as i8)));
            std_allocator_traits_B_A1__::construct(
                b.as_pointer(),
                (a1).clone(),
                ___args_2.as_pointer(),
                ___args_3.as_pointer(),
            )
        });
        assert!(({ ((*A1_count.borrow()) == 1) }));
        assert!(({ ((*b_construct.borrow()) == 1) }));
        ({ (*alloc.borrow()).deallocate((a1).clone(), 1_usize) });
    }
    {
        let A2_count: Value<i32> = Rc::new(RefCell::new(0));
        let b_construct: Value<i32> = Rc::new(RefCell::new(0));
        let b: Value<B_A2_> = Rc::new(RefCell::new(B_A2_::new({ b_construct.as_pointer() })));
        let alloc: Value<std_allocator_A2_> = Rc::new(RefCell::new(<std_allocator_A2_>::default()));
        let mut a2: Ptr<A2> = ({ (*alloc.borrow()).allocate(1_usize) });
        assert!(({ ((*A2_count.borrow()) == 0) }));
        assert!(({ ((*b_construct.borrow()) == 0) }));
        ({
            let ___args_2: Value<Ptr<i32>> = Rc::new(RefCell::new((A2_count.as_pointer())));
            let ___args_3: Value<i8> = Rc::new(RefCell::new(('d' as i8)));
            let ___args_4: Value<i32> = Rc::new(RefCell::new(5));
            std_allocator_traits_B_A2__::construct(
                b.as_pointer(),
                (a2).clone(),
                ___args_2.as_pointer(),
                ___args_3.as_pointer(),
                ___args_4.as_pointer(),
            )
        });
        assert!(({ ((*A2_count.borrow()) == 1) }));
        assert!(({ ((*b_construct.borrow()) == 1) }));
        ({ (*alloc.borrow()).deallocate((a2).clone(), 1_usize) });
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
    ({ test_7() });
    const _: () = assert!(true, "test()");;
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct Holder_NS_Incomplete_;
pub trait B_A0_Impl {
    fn construct_2_struct_A0_int_ptr(&self, p: Ptr<A0>, args: Ptr<Ptr<i32>>);
}
impl B_A0_Impl for Ptr<B_A0_> {
    fn construct_2_struct_A0_int_ptr(&self, mut p: Ptr<A0>, args: Ptr<Ptr<i32>>) {
        (*self).with(|__s| __s.count_.clone()).with_mut(|__v| {
            __v.prefix_inc();
        });
        ({
            let ___location: Ptr<A0> = (p).clone();
            let ___args: Ptr<Ptr<i32>> = (args).clone();
            construct_at_4(___location, ___args)
        });
    }
}
pub trait B_A1_Impl {
    fn construct_2_struct_A1_int_ptr_char(
        &self,
        p: Ptr<A1>,
        args_1: Ptr<Ptr<i32>>,
        args_2: Ptr<i8>,
    );
}
impl B_A1_Impl for Ptr<B_A1_> {
    fn construct_2_struct_A1_int_ptr_char(
        &self,
        mut p: Ptr<A1>,
        args_1: Ptr<Ptr<i32>>,
        args_2: Ptr<i8>,
    ) {
        (*self).with(|__s| __s.count_.clone()).with_mut(|__v| {
            __v.prefix_inc();
        });
        ({
            let ___location: Ptr<A1> = (p).clone();
            let ___args_1: Ptr<Ptr<i32>> = (args_1).clone();
            let ___args_2: Ptr<i8> = (args_2).clone();
            construct_at_5(___location, ___args_1, ___args_2)
        });
    }
}
pub trait B_A2_Impl {
    fn construct_2_struct_A2_int_ptr_char_int(
        &self,
        p: Ptr<A2>,
        args_1: Ptr<Ptr<i32>>,
        args_2: Ptr<i8>,
        args_3: Ptr<i32>,
    );
}
impl B_A2_Impl for Ptr<B_A2_> {
    fn construct_2_struct_A2_int_ptr_char_int(
        &self,
        mut p: Ptr<A2>,
        args_1: Ptr<Ptr<i32>>,
        args_2: Ptr<i8>,
        args_3: Ptr<i32>,
    ) {
        (*self).with(|__s| __s.count_.clone()).with_mut(|__v| {
            __v.prefix_inc();
        });
        ({
            let ___location: Ptr<A2> = (p).clone();
            let ___args_1: Ptr<Ptr<i32>> = (args_1).clone();
            let ___args_2: Ptr<i8> = (args_2).clone();
            let ___args_3: Ptr<i32> = (args_3).clone();
            construct_at_6(___location, ___args_1, ___args_2, ___args_3)
        });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
