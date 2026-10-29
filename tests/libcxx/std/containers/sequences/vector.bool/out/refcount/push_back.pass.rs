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
pub struct min_pointer_bool__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_bool__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_bool_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_unsigned_long_ {}
pub fn tests_4() -> () {
    {
        let mut a: [(); 7] = [
            (0_() != 0),
            (1_() != 0),
            (1_() != 0),
            (0_() != 0),
            (1_() != 0),
            (0_() != 0),
            (0_() != 0),
        ];
        let mut N: () = (::std::mem::size_of::<[(); 7]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let mut c: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < N) {
            unimplemented!();
            unimplemented!();
            let mut j: () = 0_();
            'loop_: while (j < unimplemented!()) {
                unimplemented!();
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let mut a: [(); 7] = [
            (0_() != 0),
            (1_() != 0),
            (1_() != 0),
            (0_() != 0),
            (1_() != 0),
            (0_() != 0),
            (0_() != 0),
        ];
        let mut N: () = (::std::mem::size_of::<[(); 7]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let mut c: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < N) {
            unimplemented!();
            unimplemented!();
            let mut j: () = 0_();
            'loop_: while (j < unimplemented!()) {
                unimplemented!();
                j.prefix_inc();
            }
            i.prefix_inc();
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
    ({ tests_4() });
    const _: () = assert!(true, "tests()");;
    return 0_();
}
pub trait min_allocator_bool_Impl {
    fn allocate(&self, mut n: ()) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_bool__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_bool_Impl for Ptr<min_allocator_bool_> {}
pub trait min_allocator_unsigned_long_Impl {
    fn allocate(
        &self,
        n: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__;
    fn deallocate(
        &self,
        p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        n: (),
    );
}
impl min_allocator_unsigned_long_Impl for Ptr<min_allocator_unsigned_long_> {
    fn allocate(
        &self,
        mut n: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        return min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub trait min_pointer_bool__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_bool__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_bool__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_bool__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_bool__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_bool__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_bool__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> ();
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: (),
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        n: (),
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__>;
    fn operator_add(
        &self,
        n: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__;
    fn operator_sub(
        &self,
        n: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__;
    fn operator_index(&self, n: ()) -> Ptr<()>;
}
impl min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__>
{
    fn to_(&self) -> () {
        return !(((*self).with(|__s| __s.ptr_.clone())).is_null());
    }
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__Impl :: operator_add_assign ( &tmp .as_pointer()  , n  , )
        });
        return (*tmp.borrow()).clone();
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__Impl :: operator_sub_assign ( &tmp .as_pointer()  , n  , )
        });
        return (*tmp.borrow()).clone();
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone()).offset((n) as isize);
    }
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
}
