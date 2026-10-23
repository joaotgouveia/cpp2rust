extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
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
            if operator_lt_0(
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
            } else if operator_lt_0(
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
            !(operator_lt_0(
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
            )) && !(operator_lt_0(
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
            ))
        }
    }
}
impl std::cmp::Eq for A {}
pub fn operator_lt_0(L: Ptr<A>, R: Ptr<A>) -> () {
    return ({ L.with(|__s| __s.data) } < { R.with(|__s| __s.data) });
}
pub fn test_1() -> () {
    let mut a: A = <A>::default();
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
    ({ test_1() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
