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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct MyHash_A_ {}
impl MyHash_A_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for MyHash_A_ {
    fn default() -> Self {
        { MyHash_A_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(104)]
pub struct A {
    #[offset(0)]
    #[byte_size(40)]
    pub m: (),
    #[offset(40)]
    #[byte_size(8)]
    pub it: (),
    #[offset(48)]
    #[byte_size(8)]
    pub cit: (),
    #[offset(56)]
    #[byte_size(24)]
    pub lit: (),
    #[offset(80)]
    #[byte_size(24)]
    pub clit: (),
}
impl std::cmp::PartialEq for A {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_4(
                Rc::new(RefCell::new(A {
                    m: self.m.clone(),
                    it: self.it.clone(),
                    cit: self.cit.clone(),
                    lit: self.lit.clone(),
                    clit: self.clit.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(A {
                    m: other.m.clone(),
                    it: other.it.clone(),
                    cit: other.cit.clone(),
                    lit: other.lit.clone(),
                    clit: other.clit.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for A {}
pub fn operator_eq_4(L: Ptr<A>, R: Ptr<A>) -> () {
    return ({ (L).clone() } == { (R).clone() });
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
    let mut a: A = <A>::default();
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    return 0_();
}
pub trait MyHash_A_Impl {
    fn operator_call(&self, _a0: Ptr<A>) -> () {
        unimplemented!()
    }
}
impl MyHash_A_Impl for Ptr<MyHash_A_> {}
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
