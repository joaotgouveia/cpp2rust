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
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
    pub fn new_1(mut _a0: min_allocator_int_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
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
        let c: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        const _: () = assert!(true, "");;
        assert!((({ (*c.borrow()).size() }) == 0_u64));
        const _: () = assert!(true, "");;
        (*c.borrow_mut()).push(2);
        assert!((({ (*c.borrow()).size() }) == 1_u64));
        const _: () = assert!(true, "");;
        (*c.borrow_mut()).push(1);
        assert!((({ (*c.borrow()).size() }) == 2_u64));
        const _: () = assert!(true, "");;
        (*c.borrow_mut()).push(3);
        assert!((({ (*c.borrow()).size() }) == 3_u64));
        const _: () = assert!(true, "");;
        ({
            let ___position: RandomAccessIterator<i32> =
                <RandomAccessIterator<i32>>::from(({ (*c.borrow_mut()).begin_24() }));
            (*c.borrow_mut()).erase_68(___position)
        });
        assert!((({ (*c.borrow()).size() }) == 2_u64));
        const _: () = assert!(true, "");;
        ({
            let ___position: RandomAccessIterator<i32> =
                <RandomAccessIterator<i32>>::from(({ (*c.borrow_mut()).begin_24() }));
            (*c.borrow_mut()).erase_68(___position)
        });
        assert!((({ (*c.borrow()).size() }) == 1_u64));
        const _: () = assert!(true, "");;
        ({
            let ___position: RandomAccessIterator<i32> =
                <RandomAccessIterator<i32>>::from(({ (*c.borrow_mut()).begin_24() }));
            (*c.borrow_mut()).erase_68(___position)
        });
        assert!((({ (*c.borrow()).size() }) == 0_u64));
        const _: () = assert!(true, "");;
    }
    {
        let c: Value<std_deque_int__min_allocator_int__> =
            Rc::new(RefCell::new(<std_deque_int__min_allocator_int__>::default()));
        const _: () = assert!(true, "");;
        assert!((({ (*c.borrow()).size() }) == 0_u64));
        const _: () = assert!(true, "");;
        ({
            let ___x: Value<i32> = Rc::new(RefCell::new(2));
            (*c.borrow()).push_back_55(___x.as_pointer())
        });
        assert!((({ (*c.borrow()).size() }) == 1_u64));
        const _: () = assert!(true, "");;
        ({
            let ___x: Value<i32> = Rc::new(RefCell::new(1));
            (*c.borrow()).push_back_55(___x.as_pointer())
        });
        assert!((({ (*c.borrow()).size() }) == 2_u64));
        const _: () = assert!(true, "");;
        ({
            let ___x: Value<i32> = Rc::new(RefCell::new(3));
            (*c.borrow()).push_back_55(___x.as_pointer())
        });
        assert!((({ (*c.borrow()).size() }) == 3_u64));
        const _: () = assert!(true, "");;
        ({
            let ___position: RandomAccessIterator<i32> =
                <RandomAccessIterator<i32>>::from(({ (*c.borrow()).begin_24() }));
            (*c.borrow()).erase_68(___position)
        });
        assert!((({ (*c.borrow()).size() }) == 2_u64));
        const _: () = assert!(true, "");;
        ({
            let ___position: RandomAccessIterator<i32> =
                <RandomAccessIterator<i32>>::from(({ (*c.borrow()).begin_24() }));
            (*c.borrow()).erase_68(___position)
        });
        assert!((({ (*c.borrow()).size() }) == 1_u64));
        const _: () = assert!(true, "");;
        ({
            let ___position: RandomAccessIterator<i32> =
                <RandomAccessIterator<i32>>::from(({ (*c.borrow()).begin_24() }));
            (*c.borrow()).erase_68(___position)
        });
        assert!((({ (*c.borrow()).size() }) == 0_u64));
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
pub trait min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___Impl {
    fn allocate ( &self , n : usize  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn deallocate(
        &self,
        p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        n: usize,
    );
}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___Impl
    for Ptr<min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___>
{
    fn allocate ( &self , mut n : usize  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { ( { <std_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___  >::default() . allocate ( n  , ) } )   } , )    ;
    }
    fn deallocate(
        &self,
        mut p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        ({
            <std_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___>::default()
                .deallocate((p.ptr_).clone(), n)
        });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
