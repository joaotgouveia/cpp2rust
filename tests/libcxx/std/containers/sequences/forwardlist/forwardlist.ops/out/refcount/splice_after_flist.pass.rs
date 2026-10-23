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
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_1: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_2: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_3: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
thread_local!(
    pub static t1_4: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
        5_(),
        6_(),
        7_(),
    ])));
);
thread_local!(
    pub static t2_5: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        10_(),
        11_(),
        12_(),
        13_(),
        14_(),
        15_(),
    ])));
);
thread_local!(
    pub static size_t1_6: Value<()> = Rc::new(RefCell::new(
        ((unimplemented!() - unimplemented!()) as () as ()),
    ));
);
thread_local!(
    pub static size_t2_7: Value<()> = Rc::new(RefCell::new(
        ((unimplemented!() - unimplemented!()) as () as ()),
    ));
);
pub fn testd_8(c: Ptr<()>, mut p: (), mut l: ()) {
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut n1: () = 0_();
    'loop_: while (n1 < p) {
        unimplemented!();
        {
            n1.prefix_inc();
            unimplemented!()
        };
    }
    let mut n2: () = 0_();
    'loop_: while (n2 < l) {
        unimplemented!();
        {
            n2.prefix_inc();
            unimplemented!()
        };
    }
    'loop_: while (n1 < size_t1_6.with(|rc| *rc.borrow())) {
        unimplemented!();
        {
            n1.prefix_inc();
            unimplemented!()
        };
    }
    unimplemented!();
}
pub fn testd_9(c: Ptr<()>, mut p: (), mut l: ()) {
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut n1: () = 0_();
    'loop_: while (n1 < p) {
        unimplemented!();
        {
            n1.prefix_inc();
            unimplemented!()
        };
    }
    let mut n2: () = 0_();
    'loop_: while (n2 < l) {
        unimplemented!();
        {
            n2.prefix_inc();
            unimplemented!()
        };
    }
    'loop_: while (n1 < size_t1_6.with(|rc| *rc.borrow())) {
        unimplemented!();
        {
            n1.prefix_inc();
            unimplemented!()
        };
    }
    unimplemented!();
}
pub fn test_10() -> () {
    {
        let mut l: () = 0_();
        'loop_: while (l <= size_t2_7.with(|rc| *rc.borrow())) {
            let mut p: () = 0_();
            'loop_: while (p <= size_t1_6.with(|rc| *rc.borrow())) {
                let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                ({ testd_8(c1.as_pointer(), p, l) });
                p.prefix_inc();
            }
            l.prefix_inc();
        }
    }
    {
        let mut l: () = 0_();
        'loop_: while (l <= size_t2_7.with(|rc| *rc.borrow())) {
            let mut p: () = 0_();
            'loop_: while (p <= size_t1_6.with(|rc| *rc.borrow())) {
                let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                ({ testd_9(c1.as_pointer(), p, l) });
                p.prefix_inc();
            }
            l.prefix_inc();
        }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    unimplemented!();
    return 0_();
}
pub trait min_allocator_int_Impl {
    fn allocate(&self, mut n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {}
pub trait min_pointer_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(&self, mut n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(&self, mut n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
    let _ = t1_4.with(|_| ());
    let _ = t2_5.with(|_| ());
    let _ = size_t1_6.with(|_| ());
    let _ = size_t2_7.with(|_| ());
}
