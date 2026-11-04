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
pub struct min_allocator_std_pair_const_int__double__ {}
impl CppMove for min_allocator_std_pair_const_int__double__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_4() -> bool {
    {
        const _: () = assert!(true, "(std::is_same<C::key_type, int>::value)");;
        const _: () = assert!(true, "(std::is_same<C::mapped_type, double>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<C::value_type, std::pair<const int, double> >::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::key_compare, std::less<int> >::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::allocator_type, std::allocator<std::pair<const int, double> > >::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::reference, std::pair<const int, double>&>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::const_reference, const std::pair<const int, double>&>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::pointer, std::pair<const int, double>*>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::const_pointer, const std::pair<const int, double>*>::value)"
        );;
        const _: () = assert!(true, "(std::is_same<C::size_type, std::size_t>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<C::difference_type, std::ptrdiff_t>::value)"
        );;
    }
    {
        const _: () = assert!(true, "(std::is_same<C::key_type, int>::value)");;
        const _: () = assert!(true, "(std::is_same<C::mapped_type, double>::value)");;
        const _: () = assert!(
            true,
            "(std::is_same<C::value_type, std::pair<const int, double> >::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::key_compare, std::less<int> >::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::allocator_type, min_allocator<std::pair<const int, double> > >::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::reference, std::pair<const int, double>&>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::const_reference, const std::pair<const int, double>&>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::pointer, min_pointer<std::pair<const int, double>>>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::const_pointer, min_pointer<const std::pair<const int, double>>>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::size_type, std::make_unsigned<C::difference_type>::type>::value)"
        );;
        const _: () = assert!(
            true,
            "(std::is_same<C::difference_type, std::ptrdiff_t>::value)"
        );;
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
    ({ test_4() });
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
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
