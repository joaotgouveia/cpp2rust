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
pub fn operator_eq_4(mut _a0: min_allocator_int_, mut _a1: min_allocator_int_) -> bool {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
impl CppEq for min_allocator_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_4(__lhs.read(), __rhs.read())
    }
}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
    pub fn new_1(mut _a0: min_allocator_int_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
pub fn test_5(mut n: u32, x: Ptr<i32>, a: Ptr<min_allocator_int_>) {
    let d: Value<std_deque_int__min_allocator_int__> = Rc::new(RefCell::new(
        std_deque_int__min_allocator_int__::new_1({ (n as u64) }, { (x).clone() }, { (a).clone() }),
    ));
    assert!(
        ({
            let _arg0: min_allocator_int_ = ({ (*d.borrow()).get_allocator() });
            let _arg1: min_allocator_int_ = (*a.upgrade().deref()).clone();
            operator_eq_4(_arg0, _arg1)
        })
    );
    assert!((({ (*d.borrow()).size() }) == (n as u64)));
    assert!(
        ((({
            let ___first: RandomAccessIterator<i32> = ({ (*d.borrow()).begin_24() });
            let ___last: RandomAccessIterator<i32> = ({ (*d.borrow()).end_26() });
            distance_6(___first, ___last)
        }) as usize)
            == (({ (*d.borrow()).size() }) as usize))
    );
    const _: () = assert!(true, "");;
    let i: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        <RandomAccessIterator<i32>>::from(({ (*d.borrow()).begin_24() })),
    ));
    let e: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        <RandomAccessIterator<i32>>::from(({ (*d.borrow()).end_26() })),
    ));
    'loop_: while !((*i.borrow()) == (*e.borrow())) {
        assert!(({ ((*i.borrow()).iter_deref().read()) } == { (x.read()) }));
        (*i.borrow_mut()).iter_inc();
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
        let a: Value<std_allocator_int_> = Rc::new(RefCell::new(<std_allocator_int_>::default()));
        ({
            let _x: Value<i32> = Rc::new(RefCell::new(5));
            test_7(0_u32, _x.as_pointer(), a.as_pointer())
        });
        ({
            let _x: Value<i32> = Rc::new(RefCell::new(10));
            test_7(1_u32, _x.as_pointer(), a.as_pointer())
        });
        ({
            let _x: Value<i32> = Rc::new(RefCell::new(11));
            test_7(10_u32, _x.as_pointer(), a.as_pointer())
        });
        ({
            let _x: Value<i32> = Rc::new(RefCell::new(-11_i32));
            test_7(1023_u32, _x.as_pointer(), a.as_pointer())
        });
        ({
            let _x: Value<i32> = Rc::new(RefCell::new(25));
            test_7(1024_u32, _x.as_pointer(), a.as_pointer())
        });
        ({
            let _x: Value<i32> = Rc::new(RefCell::new(0));
            test_7(1025_u32, _x.as_pointer(), a.as_pointer())
        });
        ({
            let _x: Value<i32> = Rc::new(RefCell::new(110));
            test_7(2047_u32, _x.as_pointer(), a.as_pointer())
        });
        ({
            let _x: Value<i32> = Rc::new(RefCell::new(-500_i32));
            test_7(2048_u32, _x.as_pointer(), a.as_pointer())
        });
        ({
            let _x: Value<i32> = Rc::new(RefCell::new(654));
            test_7(2049_u32, _x.as_pointer(), a.as_pointer())
        });
        ({
            let _x: Value<i32> = Rc::new(RefCell::new(78));
            test_7(4095_u32, _x.as_pointer(), a.as_pointer())
        });
        ({
            let _x: Value<i32> = Rc::new(RefCell::new(1165));
            test_7(4096_u32, _x.as_pointer(), a.as_pointer())
        });
        ({
            let _x: Value<i32> = Rc::new(RefCell::new(157));
            test_7(4097_u32, _x.as_pointer(), a.as_pointer())
        });
    }
    {
        let a: Value<min_allocator_int_> = Rc::new(RefCell::new(<min_allocator_int_>::default()));
        ({
            let _x: Value<i32> = Rc::new(RefCell::new(5));
            test_5(0_u32, _x.as_pointer(), a.as_pointer())
        });
        ({
            let _x: Value<i32> = Rc::new(RefCell::new(10));
            test_5(1_u32, _x.as_pointer(), a.as_pointer())
        });
        ({
            let _x: Value<i32> = Rc::new(RefCell::new(11));
            test_5(10_u32, _x.as_pointer(), a.as_pointer())
        });
        ({
            let _x: Value<i32> = Rc::new(RefCell::new(-11_i32));
            test_5(1023_u32, _x.as_pointer(), a.as_pointer())
        });
        ({
            let _x: Value<i32> = Rc::new(RefCell::new(25));
            test_5(1024_u32, _x.as_pointer(), a.as_pointer())
        });
        ({
            let _x: Value<i32> = Rc::new(RefCell::new(0));
            test_5(1025_u32, _x.as_pointer(), a.as_pointer())
        });
        ({
            let _x: Value<i32> = Rc::new(RefCell::new(110));
            test_5(2047_u32, _x.as_pointer(), a.as_pointer())
        });
        ({
            let _x: Value<i32> = Rc::new(RefCell::new(-500_i32));
            test_5(2048_u32, _x.as_pointer(), a.as_pointer())
        });
        ({
            let _x: Value<i32> = Rc::new(RefCell::new(654));
            test_5(2049_u32, _x.as_pointer(), a.as_pointer())
        });
        ({
            let _x: Value<i32> = Rc::new(RefCell::new(78));
            test_5(4095_u32, _x.as_pointer(), a.as_pointer())
        });
        ({
            let _x: Value<i32> = Rc::new(RefCell::new(1165));
            test_5(4096_u32, _x.as_pointer(), a.as_pointer())
        });
        ({
            let _x: Value<i32> = Rc::new(RefCell::new(157));
            test_5(4097_u32, _x.as_pointer(), a.as_pointer())
        });
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
