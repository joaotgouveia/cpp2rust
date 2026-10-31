extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(16)]
pub struct Emplaceable {
    #[offset(0)]
    int_: i32,
    #[offset(8)]
    double_: f64,
}
impl Emplaceable {
    pub fn new_1() -> Self {
        Self {
            int_: 0,
            double_: 0_f64,
        }
    }
    pub fn new_2(mut i: i32, mut d: f64) -> Self {
        Self {
            int_: i,
            double_: d,
        }
    }
    pub fn move_from(x: Ptr<Emplaceable>) -> Self {
        let __this: Emplaceable = Self {
            int_: x.with(|__s| __s.int_),
            double_: x.with(|__s| __s.double_),
        };
        field!(x, int_).write(0);
        field!(x, double_).write(0_f64);
        __this
    }
}
impl CppLess for Emplaceable {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        EmplaceableImpl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for Emplaceable {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        EmplaceableImpl::operator_eq(__lhs, __rhs.clone())
    }
}
impl Default for Emplaceable {
    fn default() -> Self {
        { Emplaceable::new_1() }
    }
}
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
#[byte_size(8)]
pub struct min_pointer_std_pair_const_int__Emplaceable___std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<(Value<i32>, Value<Emplaceable>)>,
}
impl min_pointer_std_pair_const_int__Emplaceable___std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: AnyPtr,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_pair_const_int__Emplaceable__ {}
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
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let e: Value<()> = Rc::new(RefCell::new(<()>::from(unimplemented!())));
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == 1_()));
        assert!(((*(*(*r.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3));
        assert!(
            ({
                let _x: Value<Emplaceable> = Rc::new(RefCell::new(Emplaceable::new_1()));
                EmplaceableImpl::operator_eq(
                    &(*(*r.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer(),
                    _x.as_pointer(),
                )
            })
        );
        (*r.borrow_mut()) = unimplemented!();
        assert!((unimplemented!() == 2_()));
        assert!(((*(*(*r.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4));
        assert!(
            ({
                let _x: Value<Emplaceable> =
                    Rc::new(RefCell::new(Emplaceable::new_2({ 5 }, { 6_f64 })));
                EmplaceableImpl::operator_eq(
                    &(*(*r.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer(),
                    _x.as_pointer(),
                )
            })
        );
        (*r.borrow_mut()) = unimplemented!();
        assert!((unimplemented!() == 3_()));
        assert!(((*(*(*r.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 5));
        assert!(
            ({
                let _x: Value<Emplaceable> =
                    Rc::new(RefCell::new(Emplaceable::new_2({ 6 }, { 7_f64 })));
                EmplaceableImpl::operator_eq(
                    &(*(*r.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer(),
                    _x.as_pointer(),
                )
            })
        );
    }
    {
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let e: Value<()> = Rc::new(RefCell::new(<()>::from(unimplemented!())));
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == 1_()));
        assert!(((*(*(*r.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3));
        assert!(
            ({
                let _x: Value<Emplaceable> = Rc::new(RefCell::new(Emplaceable::new_1()));
                EmplaceableImpl::operator_eq(
                    &(*(*r.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer(),
                    _x.as_pointer(),
                )
            })
        );
        (*r.borrow_mut()) = unimplemented!();
        assert!((unimplemented!() == 2_()));
        assert!(((*(*(*r.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4));
        assert!(
            ({
                let _x: Value<Emplaceable> =
                    Rc::new(RefCell::new(Emplaceable::new_2({ 5 }, { 6_f64 })));
                EmplaceableImpl::operator_eq(
                    &(*(*r.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer(),
                    _x.as_pointer(),
                )
            })
        );
        (*r.borrow_mut()) = unimplemented!();
        assert!((unimplemented!() == 3_()));
        assert!(((*(*(*r.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 5));
        assert!(
            ({
                let _x: Value<Emplaceable> =
                    Rc::new(RefCell::new(Emplaceable::new_2({ 6 }, { 7_f64 })));
                EmplaceableImpl::operator_eq(
                    &(*(*r.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer(),
                    _x.as_pointer(),
                )
            })
        );
    }
    return 0;
}
pub trait EmplaceableImpl {
    fn copy_assign(&self, _a0: Ptr<Emplaceable>) -> Ptr<Emplaceable> {
        unimplemented!()
    }
    fn move_assign(&self, x: Ptr<Emplaceable>) -> Ptr<Emplaceable>;
    fn operator_eq(&self, x: Ptr<Emplaceable>) -> bool;
    fn operator_lt(&self, x: Ptr<Emplaceable>) -> bool;
    fn get(&self) -> i32;
}
impl EmplaceableImpl for Ptr<Emplaceable> {
    fn move_assign(&self, x: Ptr<Emplaceable>) -> Ptr<Emplaceable> {
        field!((*self), int_).write({ x.with(|__s| __s.int_) });
        field!(x, int_).write(0);
        field!((*self), double_).write({ x.with(|__s| __s.double_) });
        field!(x, double_).write(0_f64);
        return (*self).clone();
    }
    fn operator_eq(&self, x: Ptr<Emplaceable>) -> bool {
        return ({ (*self).with(|__s| __s.int_) } == { x.with(|__s| __s.int_) })
            && ({ (*self).with(|__s| __s.double_) } == { x.with(|__s| __s.double_) });
    }
    fn operator_lt(&self, x: Ptr<Emplaceable>) -> bool {
        return ({ (*self).with(|__s| __s.int_) } < { x.with(|__s| __s.int_) })
            || (({ (*self).with(|__s| __s.int_) } == { x.with(|__s| __s.int_) })
                && ({ (*self).with(|__s| __s.double_) } < { x.with(|__s| __s.double_) }));
    }
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.int_);
    }
}
pub trait min_allocator_std_pair_const_int__Emplaceable__Impl {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_std_pair_const_int__Emplaceable___std_integral_constant_unsigned_long__0__
    {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p : min_pointer_std_pair_const_int__Emplaceable___std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_std_pair_const_int__Emplaceable__Impl
    for Ptr<min_allocator_std_pair_const_int__Emplaceable__>
{
}
pub trait min_pointer_std_pair_const_int__Emplaceable___std_integral_constant_unsigned_long__0__Impl
{
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<(Value<i32>, Value<Emplaceable>)> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<(Value<i32>, Value<Emplaceable>)> {
        unimplemented!()
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_std_pair_const_int__Emplaceable___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: i32,
    ) -> min_pointer_std_pair_const_int__Emplaceable___std_integral_constant_unsigned_long__0__
    {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_std_pair_const_int__Emplaceable___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: i32,
    ) -> min_pointer_std_pair_const_int__Emplaceable___std_integral_constant_unsigned_long__0__
    {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_std_pair_const_int__Emplaceable___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_std_pair_const_int__Emplaceable___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_std_pair_const_int__Emplaceable___std_integral_constant_unsigned_long__0__
    {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_std_pair_const_int__Emplaceable___std_integral_constant_unsigned_long__0__
    {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<(Value<i32>, Value<Emplaceable>)> {
        unimplemented!()
    }
}
impl min_pointer_std_pair_const_int__Emplaceable___std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_std_pair_const_int__Emplaceable___std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
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
