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
pub struct complete_type_allocator_std___tree_node_std___value_type_int__int___void_ptr__ {}
impl complete_type_allocator_std___tree_node_std___value_type_int__int___void_ptr__ {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for complete_type_allocator_std___tree_node_std___value_type_int__int___void_ptr__ {
    fn default() -> Self {
        { complete_type_allocator_std___tree_node_std___value_type_int__int___void_ptr__::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct complete_type_allocator_std___tree_node_base_void_ptr__ {}
impl complete_type_allocator_std___tree_node_base_void_ptr__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(48)]
pub struct A {
    #[offset(0)]
    pub data: (),
    #[offset(8)]
    #[byte_size(24)]
    pub m: (),
    #[offset(32)]
    #[byte_size(8)]
    pub it: (),
    #[offset(40)]
    #[byte_size(8)]
    pub cit: (),
}
impl std::cmp::Ord for A {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_4(
                Rc::new(RefCell::new(A {
                    data: self.data.clone(),
                    m: self.m.clone(),
                    it: self.it.clone(),
                    cit: self.cit.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(A {
                    data: other.data.clone(),
                    m: other.m.clone(),
                    it: other.it.clone(),
                    cit: other.cit.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_4(
                Rc::new(RefCell::new(A {
                    data: other.data.clone(),
                    m: other.m.clone(),
                    it: other.it.clone(),
                    cit: other.cit.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(A {
                    data: self.data.clone(),
                    m: self.m.clone(),
                    it: self.it.clone(),
                    cit: self.cit.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for A {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for A {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_5(
                Rc::new(RefCell::new(A {
                    data: self.data.clone(),
                    m: self.m.clone(),
                    it: self.it.clone(),
                    cit: self.cit.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(A {
                    data: other.data.clone(),
                    m: other.m.clone(),
                    it: other.it.clone(),
                    cit: other.cit.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for A {}
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
pub trait complete_type_allocator_std___tree_node_base_void_ptr__Impl {
    fn allocate(&self, mut n: ()) -> Ptr<std___tree_node_base_void_ptr_> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<std___tree_node_base_void_ptr_>, mut n: ()) {
        unimplemented!()
    }
}
impl complete_type_allocator_std___tree_node_base_void_ptr__Impl
    for Ptr<complete_type_allocator_std___tree_node_base_void_ptr__>
{
}
pub trait complete_type_allocator_std___tree_node_std___value_type_int__int___void_ptr__Impl {
    fn allocate(&self, mut n: ()) -> Ptr<std___tree_node_std___value_type_int__int___void_ptr_> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<std___tree_node_std___value_type_int__int___void_ptr_>, n: ());
}
impl complete_type_allocator_std___tree_node_std___value_type_int__int___void_ptr__Impl
    for Ptr<complete_type_allocator_std___tree_node_std___value_type_int__int___void_ptr__>
{
    fn deallocate(
        &self,
        mut p: Ptr<std___tree_node_std___value_type_int__int___void_ptr_>,
        mut n: (),
    ) {
        unimplemented!();
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
