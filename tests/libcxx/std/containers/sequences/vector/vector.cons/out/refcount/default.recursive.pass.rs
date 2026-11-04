extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct X {
    #[offset(0)]
    #[byte_size(24)]
    pub q: std_vector_X__std_allocator_X__,
}
impl X {
    pub fn move_from(_a0: Ptr<X>) -> Self {
        Self {
            q: field!(_a0, q).with_mut(std::mem::take),
        }
    }
}
impl CppMove for X {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    return 0;
}
pub trait XImpl {
    fn move_assign(&self, _a0: Ptr<X>) -> Ptr<X>;
}
impl XImpl for Ptr<X> {
    fn move_assign(&self, _a0: Ptr<X>) -> Ptr<X> {
        field!((*self), q).write({ { (*_a0.upgrade().deref()).q.clone() } });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
