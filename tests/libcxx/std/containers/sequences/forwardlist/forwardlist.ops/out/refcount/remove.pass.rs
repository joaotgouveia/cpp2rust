extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_2.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_3.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_4.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_1.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_2.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_3.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_1: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_2: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_3: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_4: Value<()> = Rc::new(RefCell::new(false));
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
pub fn do_remove_5(l: Ptr<()>, value: Ptr<()>, mut expected: ()) {
    let mut old_size: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn do_remove_6(l: Ptr<()>, value: Ptr<S>, mut expected: ()) {
    let mut old_size: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn do_remove_7(l: Ptr<()>, value: Ptr<()>, mut expected: ()) {
    let mut old_size: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor, Default)]
#[byte_size(8)]
pub struct S {
    #[offset(0)]
    #[byte_size(8)]
    pub i_: Ptr<()>,
}
impl S {
    pub fn new(mut i: ()) -> Self {
        Self { i_: Ptr::alloc(i) }
    }
    pub fn copy_from(rhs: Ptr<S>) -> Self {
        Self {
            i_: Ptr::alloc((rhs.with(|__s| __s.i_.clone()).read())),
        }
    }
}
impl CppEq for S {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        SImpl::operator_eq(__lhs, __rhs.clone())
    }
}
impl Clone for S {
    fn clone(&self) -> Self {
        let __src: Value<S> = Rc::new(RefCell::new(S {
            i_: self.i_.clone(),
        }));
        S::copy_from(__src.as_pointer())
    }
}
pub fn test_8() -> () {
    {
        let mut t1: [(); 7] = [0_(), 5_(), 5_(), 0_(), 0_(), 0_(), 5_()];
        let mut t2: [(); 3] = [5_(), 5_(), 5_()];
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let _value: Value<()> = Rc::new(RefCell::new(0_()));
            do_remove_5(c1.as_pointer(), _value.as_pointer(), 4_())
        });
        unimplemented!();
    }
    {
        let mut t1: [(); 4] = [0_(), 0_(), 0_(), 0_()];
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let _value: Value<()> = Rc::new(RefCell::new(0_()));
            do_remove_5(c1.as_pointer(), _value.as_pointer(), 4_())
        });
        unimplemented!();
    }
    {
        let mut t1: [(); 3] = [5_(), 5_(), 5_()];
        let mut t2: [(); 3] = [5_(), 5_(), 5_()];
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let _value: Value<()> = Rc::new(RefCell::new(0_()));
            do_remove_5(c1.as_pointer(), _value.as_pointer(), 0_())
        });
        unimplemented!();
    }
    {
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let _value: Value<()> = Rc::new(RefCell::new(0_()));
            do_remove_5(c1.as_pointer(), _value.as_pointer(), 0_())
        });
        unimplemented!();
    }
    {
        let mut t1: [(); 4] = [5_(), 5_(), 5_(), 0_()];
        let mut t2: [(); 3] = [5_(), 5_(), 5_()];
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let _value: Value<()> = Rc::new(RefCell::new(0_()));
            do_remove_5(c1.as_pointer(), _value.as_pointer(), 1_())
        });
        unimplemented!();
    }
    {
        let mut t1: [(); 7] = [1_(), 2_(), 1_(), 3_(), 5_(), 8_(), 11_()];
        let mut t2: [(); 5] = [2_(), 3_(), 5_(), 8_(), 11_()];
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let _l: Ptr<()> = c1.as_pointer();
            let _value: Ptr<()> = unimplemented!();
            do_remove_5(_l, _value, 2_())
        });
        unimplemented!();
    }
    {
        let mut t1: [(); 8] = [1_(), 2_(), 1_(), 3_(), 5_(), 8_(), 11_(), 1_()];
        let mut t2: [(); 5] = [2_(), 3_(), 5_(), 8_(), 11_()];
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ip: Ptr<()> = unimplemented!();
        'loop_: while ({ (ip).clone() } != { unimplemented!() }) {
            unimplemented!();
        }
        ({
            let _l: Ptr<()> = c.as_pointer();
            let _value: Ptr<S> = unimplemented!();
            do_remove_6(_l, _value, 3_())
        });
        let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ip: Ptr<()> = unimplemented!();
        'loop_: while ({ (ip).clone() } != { unimplemented!() }) {
            unimplemented!();
            unimplemented!();
            {
                ip.prefix_inc();
                unimplemented!()
            };
        }
        unimplemented!();
    }
    {
        let mut t1: [(); 7] = [0_(), 5_(), 5_(), 0_(), 0_(), 0_(), 5_()];
        let mut t2: [(); 3] = [5_(), 5_(), 5_()];
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let _value: Value<()> = Rc::new(RefCell::new(0_()));
            do_remove_7(c1.as_pointer(), _value.as_pointer(), 4_())
        });
        unimplemented!();
    }
    {
        let mut t1: [(); 4] = [0_(), 0_(), 0_(), 0_()];
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let _value: Value<()> = Rc::new(RefCell::new(0_()));
            do_remove_7(c1.as_pointer(), _value.as_pointer(), 4_())
        });
        unimplemented!();
    }
    {
        let mut t1: [(); 3] = [5_(), 5_(), 5_()];
        let mut t2: [(); 3] = [5_(), 5_(), 5_()];
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let _value: Value<()> = Rc::new(RefCell::new(0_()));
            do_remove_7(c1.as_pointer(), _value.as_pointer(), 0_())
        });
        unimplemented!();
    }
    {
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let _value: Value<()> = Rc::new(RefCell::new(0_()));
            do_remove_7(c1.as_pointer(), _value.as_pointer(), 0_())
        });
        unimplemented!();
    }
    {
        let mut t1: [(); 4] = [5_(), 5_(), 5_(), 0_()];
        let mut t2: [(); 3] = [5_(), 5_(), 5_()];
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let _value: Value<()> = Rc::new(RefCell::new(0_()));
            do_remove_7(c1.as_pointer(), _value.as_pointer(), 1_())
        });
        unimplemented!();
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
pub trait SImpl {
    fn copy_assign(&self, rhs: Ptr<S>) -> Ptr<S>;
    fn destructor(&self);
    fn operator_eq(&self, rhs: Ptr<S>) -> ();
    fn get(&self) -> ();
}
impl SImpl for Ptr<S> {
    fn copy_assign(&self, rhs: Ptr<S>) -> Ptr<S> {
        (*self)
            .with(|__s| __s.i_.clone())
            .write({ (rhs.with(|__s| __s.i_.clone()).read()) });
        return (*self).clone();
    }
    fn destructor(&self) {
        (*self).with(|__s| __s.i_.clone()).delete();
        field!((*self), i_).write(Ptr::<()>::null());
    }
    fn operator_eq(&self, rhs: Ptr<S>) -> () {
        return ({ ((*self).with(|__s| __s.i_.clone()).read()) } == {
            (rhs.with(|__s| __s.i_.clone()).read())
        });
    }
    fn get(&self) -> () {
        return ((*self).with(|__s| __s.i_.clone()).read());
    }
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
    let _ = outstanding_bytes_1.with(|_| ());
    let _ = alloc_count_2.with(|_| ());
    let _ = dealloc_count_3.with(|_| ());
    let _ = disable_default_constructor_4.with(|_| ());
}
