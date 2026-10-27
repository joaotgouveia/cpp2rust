extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static move_called_1: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(4)]
pub struct A1_int_ {
    #[offset(0)]
    id_: (),
}
impl A1_int_ {
    pub fn new(mut id: ()) -> Self {
        Self { id_: id }
    }
    pub fn move_from(a: Ptr<A1_int_>) -> Self {
        let __this: A1_int_ = Self {
            id_: ({ A1_int_Impl::id(&a) }),
        };
        move_called_1.with(|rc| *rc.borrow_mut() = true);
        __this
    }
}
thread_local!(
    pub static copy_called_4: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct A2_int_ {
    #[offset(0)]
    id_: (),
}
impl A2_int_ {
    pub fn new(id: Option<()>) -> Self {
        let mut id: () = id.unwrap_or_else(|| 0_());
        Self { id_: id }
    }
    pub fn copy_from(a: Ptr<A2_int_>) -> Self {
        let __this: A2_int_ = Self {
            id_: ({ A2_int_Impl::id(&a) }),
        };
        copy_called_4.with(|rc| *rc.borrow_mut() = true);
        __this
    }
}
impl Clone for A2_int_ {
    fn clone(&self) -> Self {
        let __src: Value<A2_int_> = Rc::new(RefCell::new(A2_int_ {
            id_: self.id_.clone(),
        }));
        A2_int_::copy_from(__src.as_pointer())
    }
}
impl Default for A2_int_ {
    fn default() -> Self {
        { A2_int_::new(None) }
    }
}
thread_local!(
    pub static copy_called_7: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct A3_int_ {
    #[offset(0)]
    id_: (),
}
impl A3_int_ {
    pub fn new(id: Option<()>) -> Self {
        let mut id: () = id.unwrap_or_else(|| 0_());
        Self { id_: id }
    }
    pub fn copy_from(a: Ptr<A3_int_>) -> Self {
        let __this: A3_int_ = Self {
            id_: ({ A3_int_Impl::id(&a) }),
        };
        copy_called_7.with(|rc| *rc.borrow_mut() = true);
        __this
    }
}
impl Clone for A3_int_ {
    fn clone(&self) -> Self {
        let __src: Value<A3_int_> = Rc::new(RefCell::new(A3_int_ {
            id_: self.id_.clone(),
        }));
        A3_int_::copy_from(__src.as_pointer())
    }
}
impl Default for A3_int_ {
    fn default() -> Self {
        { A3_int_::new(None) }
    }
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
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    return 0_();
}
pub trait A1_int_Impl {
    fn id(&self) -> ();
    fn copy_assign(&self, a: Ptr<A1_int_>) -> Ptr<A1_int_> {
        unimplemented!()
    }
    fn move_assign(&self, a: Ptr<A1_int_>) -> Ptr<A1_int_> {
        unimplemented!()
    }
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> ();
}
impl A1_int_Impl for Ptr<A1_int_> {
    fn id(&self) -> () {
        return (*self).with(|__s| __s.id_);
    }
    fn max_size(&self) -> () {
        return (*self).with(|__s| __s.id_);
    }
}
pub trait A2_int_Impl {
    fn id(&self) -> ();
    fn copy_assign(&self, a: Ptr<A2_int_>) -> Ptr<A2_int_> {
        unimplemented!()
    }
    fn move_assign(&self, a: Ptr<A2_int_>) -> Ptr<A2_int_> {
        unimplemented!()
    }
    fn allocate(&self, mut _a0: (), mut hint: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl A2_int_Impl for Ptr<A2_int_> {
    fn id(&self) -> () {
        return (*self).with(|__s| __s.id_);
    }
}
pub trait A3_int_Impl {
    fn id(&self) -> ();
    fn copy_assign(&self, a: Ptr<A3_int_>) -> Ptr<A3_int_> {
        unimplemented!()
    }
    fn move_assign(&self, a: Ptr<A3_int_>) -> Ptr<A3_int_> {
        unimplemented!()
    }
    fn select_on_container_copy_construction(&self) -> A3_int_ {
        unimplemented!()
    }
}
impl A3_int_Impl for Ptr<A3_int_> {
    fn id(&self) -> () {
        return (*self).with(|__s| __s.id_);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = move_called_1.with(|_| ());
    let _ = copy_called_4.with(|_| ());
    let _ = copy_called_7.with(|_| ());
}
