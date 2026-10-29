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
pub struct min_allocator_bool_ {}
impl min_allocator_unsigned_long_ {
    pub fn new_1(mut _a0: min_allocator_bool_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_unsigned_long_ {}
impl min_allocator_unsigned_long_ {}
pub fn tests_4() -> bool {
    {
        let d: Value<Vec<bool>> = Rc::new(RefCell::new(
            (0..(10_usize) as usize)
                .map(|_| true)
                .collect::<Vec<bool>>(),
        ));
        let i: Value<RandomAccessIterator<bool>> = Rc::new(RefCell::new({
            let __it = ({ (*d.borrow()).cbegin() }).iter_add(2_i64);
            let __off = __it.index();
            (*d.borrow_mut()).splice(__off..__off, Vec::from([false, true, true, false]));
            __it
        }));
        assert!(((*d.borrow()).len() == 14_usize));
        assert!((*i.borrow()) == d.decay().begin().iter_add(2_i64));
        assert!(((({ (*d.borrow())[0_usize] } as bool) as i32) == (true as i32)));
        assert!(((({ (*d.borrow())[1_usize] } as bool) as i32) == (true as i32)));
        assert!(((({ (*d.borrow())[2_usize] } as bool) as i32) == (false as i32)));
        assert!(((({ (*d.borrow())[3_usize] } as bool) as i32) == (true as i32)));
        assert!(((({ (*d.borrow())[4_usize] } as bool) as i32) == (true as i32)));
        assert!(((({ (*d.borrow())[5_usize] } as bool) as i32) == (false as i32)));
        assert!(((({ (*d.borrow())[6_usize] } as bool) as i32) == (true as i32)));
        assert!(((({ (*d.borrow())[7_usize] } as bool) as i32) == (true as i32)));
        assert!(((({ (*d.borrow())[8_usize] } as bool) as i32) == (true as i32)));
        assert!(((({ (*d.borrow())[9_usize] } as bool) as i32) == (true as i32)));
        assert!(((({ (*d.borrow())[10_usize] } as bool) as i32) == (true as i32)));
        assert!(((({ (*d.borrow())[11_usize] } as bool) as i32) == (true as i32)));
        assert!(((({ (*d.borrow())[12_usize] } as bool) as i32) == (true as i32)));
        assert!(((({ (*d.borrow())[13_usize] } as bool) as i32) == (true as i32)));
    }
    {
        let d: Value<Vec<bool>> = Rc::new(RefCell::new(vec![true; 10_usize as usize]));
        let i: Value<RandomAccessIterator<bool>> = Rc::new(RefCell::new({
            let __it = ({ (*d.borrow()).cbegin() }).iter_add(2_i64);
            let __off = __it.index();
            (*d.borrow_mut()).splice(__off..__off, Vec::from([false, true, true, false]));
            __it
        }));
        assert!(((*d.borrow()).len() == 14_usize));
        assert!((*i.borrow()) == d.decay().begin().iter_add(2_i64));
        assert!(((((*d.borrow())[0_usize] as bool) as i32) == (true as i32)));
        assert!(((((*d.borrow())[1_usize] as bool) as i32) == (true as i32)));
        assert!(((((*d.borrow())[2_usize] as bool) as i32) == (false as i32)));
        assert!(((((*d.borrow())[3_usize] as bool) as i32) == (true as i32)));
        assert!(((((*d.borrow())[4_usize] as bool) as i32) == (true as i32)));
        assert!(((((*d.borrow())[5_usize] as bool) as i32) == (false as i32)));
        assert!(((((*d.borrow())[6_usize] as bool) as i32) == (true as i32)));
        assert!(((((*d.borrow())[7_usize] as bool) as i32) == (true as i32)));
        assert!(((((*d.borrow())[8_usize] as bool) as i32) == (true as i32)));
        assert!(((((*d.borrow())[9_usize] as bool) as i32) == (true as i32)));
        assert!(((((*d.borrow())[10_usize] as bool) as i32) == (true as i32)));
        assert!(((((*d.borrow())[11_usize] as bool) as i32) == (true as i32)));
        assert!(((((*d.borrow())[12_usize] as bool) as i32) == (true as i32)));
        assert!(((((*d.borrow())[13_usize] as bool) as i32) == (true as i32)));
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
    ({ tests_4() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_bool__std_integral_constant_unsigned_long__0__;
pub trait min_allocator_bool_Impl {
    fn allocate(&self, mut n: usize) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_bool__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_bool_Impl for Ptr<min_allocator_bool_> {}
pub trait min_allocator_unsigned_long_Impl {
    fn allocate(
        &self,
        n: usize,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__;
    fn deallocate(
        &self,
        p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        n: usize,
    );
}
impl min_allocator_unsigned_long_Impl for Ptr<min_allocator_unsigned_long_> {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        return min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__::new_1({
            ({ <std_allocator_unsigned_long_>::default().allocate(n) })
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        ({
            let ___p: Ptr<u64> = (p.ptr_).clone();
            let ___n: usize = n;
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
