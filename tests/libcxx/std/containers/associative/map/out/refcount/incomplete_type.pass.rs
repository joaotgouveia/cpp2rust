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
#[byte_size(1)]
pub struct complete_type_allocator_std_pair_const_int__int__ {}
impl complete_type_allocator_std_pair_const_int__int__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(72)]
pub struct A {
    #[offset(0)]
    pub data: (),
    #[offset(8)]
    #[byte_size(48)]
    pub m: (),
    #[offset(56)]
    #[byte_size(8)]
    pub it: (),
    #[offset(64)]
    #[byte_size(8)]
    pub cit: (),
}
impl A {
    pub fn move_from(_a0: Ptr<A>) -> Self {
        Self {
            data: { (*_a0.upgrade().deref()).data },
            m: unimplemented!(),
            it: field!(_a0, it).with_mut(std::mem::take),
            cit: field!(_a0, cit).with_mut(std::mem::take),
        }
    }
}
impl CppLess for A {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_4(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for A {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_5(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_eq_5(L: Ptr<A>, R: Ptr<A>) -> () {
    return ({ (L).clone() } == { (R).clone() });
}
pub fn operator_lt_4(L: Ptr<A>, R: Ptr<A>) -> () {
    return ({ L.with(|__s| __s.data) } < { R.with(|__s| __s.data) });
}
pub fn test_6() -> () {
    let mut a: A = <A>::default();
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
    ({ test_6() });
    return 0_();
}
pub trait AImpl {
    fn move_assign(&self, _a0: Ptr<A>) -> Ptr<A>;
}
impl AImpl for Ptr<A> {
    fn move_assign(&self, _a0: Ptr<A>) -> Ptr<A> {
        field!((*self), data).write({ { (*_a0.upgrade().deref()).data } });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        return (*self).clone();
    }
}
pub trait complete_type_allocator_std_pair_const_int__int__Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!()
    }
}
impl complete_type_allocator_std_pair_const_int__int__Impl
    for Ptr<complete_type_allocator_std_pair_const_int__int__>
{
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
