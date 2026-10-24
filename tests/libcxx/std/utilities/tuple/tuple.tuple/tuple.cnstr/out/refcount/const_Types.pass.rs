extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign)]
#[byte_size(4)]
pub struct NoValueCtor {
    #[offset(0)]
    pub id: (),
}
impl NoValueCtor {
    pub fn new() -> Self {
        Self {
            id: (*count_0.with(Value::clone).borrow_mut()).prefix_inc(),
        }
    }
    pub fn copy_from(other: Ptr<NoValueCtor>) -> Self {
        let __this: NoValueCtor = Self {
            id: other.with(|__s| __s.id),
        };
        (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for NoValueCtor {
    fn clone(&self) -> Self {
        let __src: Value<NoValueCtor> = Rc::new(RefCell::new(NoValueCtor {
            id: self.id.clone(),
        }));
        NoValueCtor::copy_from(__src.as_pointer())
    }
}
impl Default for NoValueCtor {
    fn default() -> Self {
        { NoValueCtor::new() }
    }
}
thread_local!(
    pub static count_0: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign)]
#[byte_size(1)]
pub struct NoValueCtorEmpty {}
impl NoValueCtorEmpty {
    pub fn new() -> Self {
        Self {}
    }
    pub fn copy_from(_a0: Ptr<NoValueCtorEmpty>) -> Self {
        Self {}
    }
}
impl Clone for NoValueCtorEmpty {
    fn clone(&self) -> Self {
        let __src: Value<NoValueCtorEmpty> = Rc::new(RefCell::new(NoValueCtorEmpty {}));
        NoValueCtorEmpty::copy_from(__src.as_pointer())
    }
}
impl Default for NoValueCtorEmpty {
    fn default() -> Self {
        { NoValueCtorEmpty::new() }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Default)]
#[byte_size(1)]
pub struct ImplicitCopy {}
impl ImplicitCopy {
    pub fn new(mut _a0: ()) -> Self {
        Self {}
    }
    pub fn copy_from(_a0: Ptr<ImplicitCopy>) -> Self {
        Self {}
    }
}
impl Clone for ImplicitCopy {
    fn clone(&self) -> Self {
        let __src: Value<ImplicitCopy> = Rc::new(RefCell::new(ImplicitCopy {}));
        ImplicitCopy::copy_from(__src.as_pointer())
    }
}
pub fn testImplicitCopy1_1() -> () {
    let mut i: ImplicitCopy = ImplicitCopy::new({ 42_() });
    return unimplemented!();
}
pub fn testImplicitCopy2_2() -> () {
    let mut i: ImplicitCopy = ImplicitCopy::new({ 42_() });
    return unimplemented!();
}
pub fn testImplicitCopy3_3() -> () {
    let i: Value<ImplicitCopy> = Rc::new(RefCell::new(ImplicitCopy::new({ 42_() })));
    return (*i.borrow_mut()).clone();
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
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "std::get<0>(t) == 2");;
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "std::get<0>(t) == 0");;
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "std::get<0>(t) == 2");;
        const _: () = assert!(true, "std::get<1>(t) == nullptr");;
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub trait ImplicitCopyImpl {
    fn copy_assign(&self, _a0: Ptr<ImplicitCopy>) -> Ptr<ImplicitCopy>;
}
impl ImplicitCopyImpl for Ptr<ImplicitCopy> {
    fn copy_assign(&self, _a0: Ptr<ImplicitCopy>) -> Ptr<ImplicitCopy> {
        return (*self).clone();
    }
}
pub trait NoValueCtorImpl {
    fn copy_assign(&self, _a0: Ptr<NoValueCtor>) -> Ptr<NoValueCtor>;
}
impl NoValueCtorImpl for Ptr<NoValueCtor> {
    fn copy_assign(&self, _a0: Ptr<NoValueCtor>) -> Ptr<NoValueCtor> {
        field!((*self), id).write({ _a0.with(|__s| __s.id) });
        return (*self).clone();
    }
}
pub trait NoValueCtorEmptyImpl {
    fn copy_assign(&self, _a0: Ptr<NoValueCtorEmpty>) -> Ptr<NoValueCtorEmpty>;
}
impl NoValueCtorEmptyImpl for Ptr<NoValueCtorEmpty> {
    fn copy_assign(&self, _a0: Ptr<NoValueCtorEmpty>) -> Ptr<NoValueCtorEmpty> {
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = count_0.with(|_| ());
}
