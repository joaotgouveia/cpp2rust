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
pub struct min_allocator_int_ {}
impl CppMove for min_allocator_int_ {
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
pub fn is_contiguous_container_asan_correct_4(_a0: Ptr<Vec<i32>>) -> bool {
    return true;
}
pub fn tests_5() -> bool {
    {
        let d: Value<Vec<i32>> = Rc::new(RefCell::new(vec![1; 10_usize as usize]));
        let i: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
            match ({ ({ (*d.borrow()).cbegin() }).iter_add(2_isize) },) {
                (mut __tmp_0,) => {
                    let __off = __tmp_0.index();
                    (*d.borrow_mut()).splice(__off..__off, Vec::from([3, 4, 5, 6]));
                    (__tmp_0).clone()
                }
            },
        ));
        assert!(({ ((*d.borrow()).len() == 14_usize) }));
        assert!(({ is_contiguous_container_asan_correct_6(d.as_pointer(),) }));
        assert!((*i.borrow()) == CppRange::begin(&d.decay()).iter_add(2_isize));
        assert!(((d.decay().offset(0_usize as isize).read()) == 1));
        assert!(((d.decay().offset(1_usize as isize).read()) == 1));
        assert!(((d.decay().offset(2_usize as isize).read()) == 3));
        assert!(((d.decay().offset(3_usize as isize).read()) == 4));
        assert!(((d.decay().offset(4_usize as isize).read()) == 5));
        assert!(((d.decay().offset(5_usize as isize).read()) == 6));
        assert!(((d.decay().offset(6_usize as isize).read()) == 1));
        assert!(((d.decay().offset(7_usize as isize).read()) == 1));
        assert!(((d.decay().offset(8_usize as isize).read()) == 1));
        assert!(((d.decay().offset(9_usize as isize).read()) == 1));
        assert!(((d.decay().offset(10_usize as isize).read()) == 1));
        assert!(((d.decay().offset(11_usize as isize).read()) == 1));
        assert!(((d.decay().offset(12_usize as isize).read()) == 1));
        assert!(((d.decay().offset(13_usize as isize).read()) == 1));
    }
    {
        let d: Value<Vec<i32>> = Rc::new(RefCell::new(vec![1; 10_usize as usize]));
        let i: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
            match ({ ({ (*d.borrow()).cbegin() }).iter_add(2_isize) },) {
                (mut __tmp_1,) => {
                    let __off = __tmp_1.index();
                    (*d.borrow_mut()).splice(__off..__off, Vec::from([3, 4, 5, 6]));
                    (__tmp_1).clone()
                }
            },
        ));
        assert!(({ ((*d.borrow()).len() == 14_usize) }));
        assert!(({ is_contiguous_container_asan_correct_4(d.as_pointer(),) }));
        assert!((*i.borrow()) == CppRange::begin(&d.decay()).iter_add(2_isize));
        assert!(((d.decay().offset(0_usize as isize).read()) == 1));
        assert!(((d.decay().offset(1_usize as isize).read()) == 1));
        assert!(((d.decay().offset(2_usize as isize).read()) == 3));
        assert!(((d.decay().offset(3_usize as isize).read()) == 4));
        assert!(((d.decay().offset(4_usize as isize).read()) == 5));
        assert!(((d.decay().offset(5_usize as isize).read()) == 6));
        assert!(((d.decay().offset(6_usize as isize).read()) == 1));
        assert!(((d.decay().offset(7_usize as isize).read()) == 1));
        assert!(((d.decay().offset(8_usize as isize).read()) == 1));
        assert!(((d.decay().offset(9_usize as isize).read()) == 1));
        assert!(((d.decay().offset(10_usize as isize).read()) == 1));
        assert!(((d.decay().offset(11_usize as isize).read()) == 1));
        assert!(((d.decay().offset(12_usize as isize).read()) == 1));
        assert!(((d.decay().offset(13_usize as isize).read()) == 1));
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
    ({ tests_5() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__;
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
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
