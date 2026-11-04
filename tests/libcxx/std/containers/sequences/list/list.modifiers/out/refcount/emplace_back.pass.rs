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
pub struct min_allocator_A_ {}
impl CppMove for min_allocator_A_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(16)]
pub struct A {
    #[offset(0)]
    i_: i32,
    #[offset(8)]
    d_: f64,
}
impl A {
    pub fn new(mut i: i32, mut d: f64) -> Self {
        Self { i_: i, d_: d }
    }
}
pub fn test_4() -> bool {
    {
        let c: Value<std_list_A__std_allocator_A__> =
            Rc::new(RefCell::new(<std_list_A__std_allocator_A__>::default()));
        let r1: Ptr<A> = ({
            let ___args_0: Value<i32> = Rc::new(RefCell::new(2));
            let ___args_1: Value<f64> = Rc::new(RefCell::new(3.5_f64));
            (*c.borrow()).emplace_back(___args_0.as_pointer(), ___args_1.as_pointer())
        });
        assert!(({ (({ (*c.borrow()).size() }) == 1_u64) }));
        assert!(({ ({ (r1).clone() } == { ({ (*c.borrow()).back_43() }) }) }));
        assert!(({ (({ AImpl::geti(&({ (*c.borrow()).front_41() }),) }) == 2) }));
        assert!(({ (({ AImpl::getd(&({ (*c.borrow()).front_41() }),) }) == 3.5_f64) }));
        let r2: Ptr<A> = ({
            let ___args_0: Value<i32> = Rc::new(RefCell::new(3));
            let ___args_1: Value<f64> = Rc::new(RefCell::new(4.5_f64));
            (*c.borrow()).emplace_back(___args_0.as_pointer(), ___args_1.as_pointer())
        });
        assert!(({ (({ (*c.borrow()).size() }) == 2_u64) }));
        assert!(({ ({ (r2).clone() } == { ({ (*c.borrow()).back_43() }) }) }));
        assert!(({ (({ AImpl::geti(&({ (*c.borrow()).front_41() }),) }) == 2) }));
        assert!(({ (({ AImpl::getd(&({ (*c.borrow()).front_41() }),) }) == 3.5_f64) }));
        assert!(({ (({ AImpl::geti(&({ (*c.borrow()).back_43() }),) }) == 3) }));
        assert!(({ (({ AImpl::getd(&({ (*c.borrow()).back_43() }),) }) == 4.5_f64) }));
    }
    {
        let c: Value<std_list_A__min_allocator_A__> =
            Rc::new(RefCell::new(<std_list_A__min_allocator_A__>::default()));
        let r1: Ptr<A> = ({
            let ___args_0: Value<i32> = Rc::new(RefCell::new(2));
            let ___args_1: Value<f64> = Rc::new(RefCell::new(3.5_f64));
            (*c.borrow()).emplace_back(___args_0.as_pointer(), ___args_1.as_pointer())
        });
        assert!(({ (({ (*c.borrow()).size() }) == 1_u64) }));
        assert!(({ ({ (r1).clone() } == { ({ (*c.borrow()).back_43() }) }) }));
        assert!(({ (({ AImpl::geti(&({ (*c.borrow()).front_41() }),) }) == 2) }));
        assert!(({ (({ AImpl::getd(&({ (*c.borrow()).front_41() }),) }) == 3.5_f64) }));
        let r2: Ptr<A> = ({
            let ___args_0: Value<i32> = Rc::new(RefCell::new(3));
            let ___args_1: Value<f64> = Rc::new(RefCell::new(4.5_f64));
            (*c.borrow()).emplace_back(___args_0.as_pointer(), ___args_1.as_pointer())
        });
        assert!(({ (({ (*c.borrow()).size() }) == 2_u64) }));
        assert!(({ ({ (r2).clone() } == { ({ (*c.borrow()).back_43() }) }) }));
        assert!(({ (({ AImpl::geti(&({ (*c.borrow()).front_41() }),) }) == 2) }));
        assert!(({ (({ AImpl::getd(&({ (*c.borrow()).front_41() }),) }) == 3.5_f64) }));
        assert!(({ (({ AImpl::geti(&({ (*c.borrow()).back_43() }),) }) == 3) }));
        assert!(({ (({ AImpl::getd(&({ (*c.borrow()).back_43() }),) }) == 4.5_f64) }));
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
    assert!(({ test_4() }));
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_A__std_integral_constant_unsigned_long__0__;
pub trait AImpl {
    fn copy_assign(&self, _a0: Ptr<A>) -> Ptr<A> {
        unimplemented!()
    }
    fn geti(&self) -> i32;
    fn getd(&self) -> f64;
}
impl AImpl for Ptr<A> {
    fn geti(&self) -> i32 {
        return (*self).with(|__s| __s.i_);
    }
    fn getd(&self) -> f64 {
        return (*self).with(|__s| __s.d_);
    }
}
pub trait min_allocator_A_Impl {
    fn allocate(&self, mut n: usize) -> min_pointer_A__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_A__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_A_Impl for Ptr<min_allocator_A_> {}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
