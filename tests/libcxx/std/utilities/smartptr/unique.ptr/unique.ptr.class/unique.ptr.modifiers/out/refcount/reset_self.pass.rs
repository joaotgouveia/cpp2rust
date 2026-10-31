extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(8)]
pub struct A {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr_: Option<Value<A>>,
}
impl A {
    pub fn new() -> Self {
        let __this: Value<A> = Rc::new(RefCell::new(Self {
            ptr_: this.to_owned_opt(),
        }));
        let this: Ptr<A> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(_a0: Ptr<A>) -> Self {
        Self {
            ptr_: field!(_a0, ptr_).with_mut(|__v: &mut Option<Value<A>>| __v.take()),
        }
    }
}
impl Default for A {
    fn default() -> Self {
        { A::new() }
    }
}
pub fn test_0() -> bool {
    ({ AImpl::reset(&(Ptr::alloc(A::new()))) });
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub trait AImpl {
    fn reset(&self);
    fn move_assign(&self, _a0: Ptr<A>) -> Ptr<A>;
}
impl AImpl for Ptr<A> {
    fn reset(&self) {
        {
            let _p: Ptr<_> = Ptr::null();
            field_ptr!((*self), ptr_).write(_p.to_owned_opt())
        };
    }
    fn move_assign(&self, _a0: Ptr<A>) -> Ptr<A> {
        field_ptr!((*self), ptr_)
            .write(field!(_a0, ptr_).with_mut(|__v: &mut Option<Value<A>>| __v.take()));
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
