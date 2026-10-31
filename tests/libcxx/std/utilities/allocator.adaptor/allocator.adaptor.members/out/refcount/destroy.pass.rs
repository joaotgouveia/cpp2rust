extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(4)]
pub struct A1_B_ {
    #[offset(0)]
    id_: i32,
}
impl A1_B_ {
    pub fn new(id: Option<i32>) -> Self {
        let mut id: i32 = id.unwrap_or_else(|| 0);
        Self { id_: id }
    }
}
impl Default for A1_B_ {
    fn default() -> Self {
        { A1_B_::new(None) }
    }
}
thread_local!(
    pub static constructed_6: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static destroy_called_7: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(4)]
pub struct A3_B_ {
    #[offset(0)]
    id_: i32,
}
impl A3_B_ {
    pub fn new(id: Option<i32>) -> Self {
        let mut id: i32 = id.unwrap_or_else(|| 0);
        Self { id_: id }
    }
}
impl Default for A3_B_ {
    fn default() -> Self {
        { A3_B_::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(1)]
pub struct B {}
impl B {
    pub fn new() -> Self {
        let __this: B = Self {};
        constructed_8.with(|rc| *rc.borrow_mut() = true);
        __this
    }
}
impl Default for B {
    fn default() -> Self {
        { B::new() }
    }
}
thread_local!(
    pub static constructed_8: Value<bool> = Rc::new(RefCell::new(false));
);

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
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let buf: Value<Box<[i8]>> =
            Rc::new(RefCell::new((0..100).map(|_| 0_i8).collect::<Box<[i8]>>()));
        let mut s: Ptr<B> = (buf.as_pointer() as Ptr<i8>).reinterpret_cast::<B>();
        assert!(!(constructed_8.with(|rc| *rc.borrow())));
        unimplemented!();
        assert!(constructed_8.with(|rc| *rc.borrow()));
        unimplemented!();
        assert!(!(constructed_8.with(|rc| *rc.borrow())));
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let buf: Value<Box<[i8]>> =
            Rc::new(RefCell::new((0..100).map(|_| 0_i8).collect::<Box<[i8]>>()));
        let mut s: Ptr<B> = (buf.as_pointer() as Ptr<i8>).reinterpret_cast::<B>();
        assert!(!(constructed_8.with(|rc| *rc.borrow())));
        assert!(!(constructed_6.with(|rc| *rc.borrow())));
        assert!(!(destroy_called_7.with(|rc| *rc.borrow())));
        unimplemented!();
        assert!(constructed_8.with(|rc| *rc.borrow()));
        assert!(constructed_6.with(|rc| *rc.borrow()));
        assert!(!(destroy_called_7.with(|rc| *rc.borrow())));
        unimplemented!();
        assert!(!(constructed_8.with(|rc| *rc.borrow())));
        assert!(constructed_6.with(|rc| *rc.borrow()));
        assert!(destroy_called_7.with(|rc| *rc.borrow()));
    }
    return 0;
}
pub trait A1_B_Impl {
    fn id(&self) -> i32 {
        unimplemented!()
    }
    fn copy_assign(&self, a: Ptr<A1_B_>) -> Ptr<A1_B_> {
        unimplemented!()
    }
    fn move_assign(&self, a: Ptr<A1_B_>) -> Ptr<A1_B_> {
        unimplemented!()
    }
    fn allocate(&self, mut n: usize) -> Ptr<B> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<B>, mut n: usize) {
        unimplemented!()
    }
    fn max_size(&self) -> usize {
        unimplemented!()
    }
}
impl A1_B_Impl for Ptr<A1_B_> {}
pub trait A3_B_Impl {
    fn id(&self) -> i32 {
        unimplemented!()
    }
    fn copy_assign(&self, a: Ptr<A3_B_>) -> Ptr<A3_B_> {
        unimplemented!()
    }
    fn move_assign(&self, a: Ptr<A3_B_>) -> Ptr<A3_B_> {
        unimplemented!()
    }
    fn select_on_container_copy_construction(&self) -> A3_B_ {
        unimplemented!()
    }
    fn construct_7_struct_B(&self, p: Ptr<B>);
    fn destroy_8_struct_B(&self, p: Ptr<B>);
}
impl A3_B_Impl for Ptr<A3_B_> {
    fn construct_7_struct_B(&self, mut p: Ptr<B>) {
        Ptr::alloc(B::new());
        constructed_6.with(|rc| *rc.borrow_mut() = true);
    }
    fn destroy_8_struct_B(&self, mut p: Ptr<B>) {
        ({ BImpl::destructor(&p) });
        destroy_called_7.with(|rc| *rc.borrow_mut() = true);
    }
}
pub trait BImpl {
    fn destructor(&self);
}
impl BImpl for Ptr<B> {
    fn destructor(&self) {
        constructed_8.with(|rc| *rc.borrow_mut() = false);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = constructed_6.with(|_| ());
    let _ = destroy_called_7.with(|_| ());
    let _ = constructed_8.with(|_| ());
}
