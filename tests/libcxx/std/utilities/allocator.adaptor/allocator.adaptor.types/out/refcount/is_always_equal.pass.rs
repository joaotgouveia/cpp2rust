extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(4)]
pub struct A1_int_ {
    #[offset(0)]
    id_: i32,
}
impl A1_int_ {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(4)]
pub struct A2_int_ {
    #[offset(0)]
    id_: i32,
}
impl A2_int_ {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(4)]
pub struct A3_int_ {
    #[offset(0)]
    id_: i32,
}
impl A3_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!(
            (alloc_count_12.with(|rc| *rc.borrow()) >= dealloc_count_13.with(|rc| *rc.borrow()))
        );
        return ((alloc_count_12.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_13.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_14.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_11.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_12.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_13.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_11: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_12: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_13: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_14: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
impl CppMove for min_allocator_int_ {
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
    const _: () = assert!(
        true,
        "(std::is_same< std::allocator_traits<A1<int>>::is_always_equal, std::false_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< std::allocator_traits<min_allocator<int>>::is_always_equal, std::true_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< std::scoped_allocator_adaptor<A1<int>>::is_always_equal,\n                               std::allocator_traits<A1<int>>::is_always_equal >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same< std::scoped_allocator_adaptor<min_allocator<int>>::is_always_equal,\n                               std::allocator_traits<min_allocator<int>>::is_always_equal >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::scoped_allocator_adaptor<A1<int>, A2<int>>::is_always_equal::value ==\n                 (std::allocator_traits<A1<int>>::is_always_equal::value &&\n                  std::allocator_traits<A2<int>>::is_always_equal::value))"
    );;
    const _: () = assert!(
        true,
        "(std::scoped_allocator_adaptor<A1<int>, min_allocator<int>>::is_always_equal::value ==\n                 (std::allocator_traits<A1<int>>::is_always_equal::value &&\n                  std::allocator_traits<min_allocator<int>>::is_always_equal::value))"
    );;
    const _: () = assert!(
        true,
        "(std::scoped_allocator_adaptor<A1<int>, A2<int>, A3<int>>::is_always_equal::value ==\n                 (std::allocator_traits<A1<int>>::is_always_equal::value &&\n                  std::allocator_traits<A2<int>>::is_always_equal::value &&\n                  std::allocator_traits<A3<int>>::is_always_equal::value))"
    );;
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
pub trait A1_int_Impl {
    fn id(&self) -> i32 {
        unimplemented!()
    }
    fn copy_assign(&self, a: Ptr<A1_int_>) -> Ptr<A1_int_> {
        unimplemented!()
    }
    fn move_assign(&self, a: Ptr<A1_int_>) -> Ptr<A1_int_> {
        unimplemented!()
    }
    fn allocate(&self, mut n: usize) -> Ptr<i32> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<i32>, mut n: usize) {
        unimplemented!()
    }
    fn max_size(&self) -> usize {
        unimplemented!()
    }
}
impl A1_int_Impl for Ptr<A1_int_> {}
pub trait A2_int_Impl {
    fn id(&self) -> i32 {
        unimplemented!()
    }
    fn copy_assign(&self, a: Ptr<A2_int_>) -> Ptr<A2_int_> {
        unimplemented!()
    }
    fn move_assign(&self, a: Ptr<A2_int_>) -> Ptr<A2_int_> {
        unimplemented!()
    }
    fn allocate(&self, mut _a0: usize, mut hint: AnyPtr) -> Ptr<i32> {
        unimplemented!()
    }
}
impl A2_int_Impl for Ptr<A2_int_> {}
pub trait A3_int_Impl {
    fn id(&self) -> i32 {
        unimplemented!()
    }
    fn copy_assign(&self, a: Ptr<A3_int_>) -> Ptr<A3_int_> {
        unimplemented!()
    }
    fn move_assign(&self, a: Ptr<A3_int_>) -> Ptr<A3_int_> {
        unimplemented!()
    }
    fn select_on_container_copy_construction(&self) -> A3_int_ {
        unimplemented!()
    }
}
impl A3_int_Impl for Ptr<A3_int_> {}
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
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_11.with(|_| ());
    let _ = alloc_count_12.with(|_| ());
    let _ = dealloc_count_13.with(|_| ());
    let _ = disable_default_constructor_14.with(|_| ());
}
