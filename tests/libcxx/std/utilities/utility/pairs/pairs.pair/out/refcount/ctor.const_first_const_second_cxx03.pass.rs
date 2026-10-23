extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct A {
    #[offset(0)]
    data_: (),
}
impl A {
    pub fn new(mut data: ()) -> Self {
        Self { data_: data }
    }
}
impl std::cmp::PartialEq for A {
    fn eq(&self, other: &Self) -> bool {
        {
            AImpl::operator_eq(
                &Rc::new(RefCell::new(A {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(A {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for A {}
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
        let p: Value<()> = Rc::new(RefCell::new({
            let __tmp_0: Value<()> = Rc::new(RefCell::new(3.5_f32));
            let __tmp_1: Value<Ptr<()>> = Rc::new(RefCell::new(Ptr::<()>::null()));
            std_pair_float__short_ptr_::new_1({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
        }));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub trait AImpl {
    fn operator_eq(&self, a: Ptr<A>) -> ();
}
impl AImpl for Ptr<A> {
    fn operator_eq(&self, a: Ptr<A>) -> () {
        return ({ (*self).with(|__s| __s.data_) } == { a.with(|__s| __s.data_) });
    }
}
pub fn __cpp2rust_init_globals() {}
