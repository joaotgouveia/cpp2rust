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
pub struct operator_hijacker {}
impl std::cmp::Ord for operator_hijacker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_hijackerImpl::operator_lt(
                &Rc::new(RefCell::new(operator_hijacker {})).as_pointer(),
                Rc::new(RefCell::new(operator_hijacker {})).as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_hijackerImpl::operator_lt(
                &Rc::new(RefCell::new(operator_hijacker {})).as_pointer(),
                Rc::new(RefCell::new(operator_hijacker {})).as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for operator_hijacker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for operator_hijacker {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_hijackerImpl::operator_eq(
                &Rc::new(RefCell::new(operator_hijacker {})).as_pointer(),
                Rc::new(RefCell::new(operator_hijacker {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for operator_hijacker {}
const _: () = assert!(
    true,
    "std::is_trivially_copyable<operator_hijacker>::value &&     //\n                  std::is_copy_constructible<operator_hijacker>::value && //\n                  std::is_move_constructible<operator_hijacker>::value && //\n                  std::is_copy_assignable<operator_hijacker>::value &&    //\n                  std::is_move_assignable<operator_hijacker>::value"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct operator_hijacker_char_traits_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct operator_hijacker_char_traits_wchar_t_ {}
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
        let fs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    }
    {
        let fs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    }
    {
        let fs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    }
    {
        let fs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    }
    return 0_();
}
pub trait operator_hijackerImpl {
    fn operator_lt(&self, _a0: Ptr<operator_hijacker>) -> ();
    fn operator_eq(&self, _a0: Ptr<operator_hijacker>) -> ();
    fn operator_call(&self) -> ();
}
impl operator_hijackerImpl for Ptr<operator_hijacker> {
    fn operator_lt(&self, _a0: Ptr<operator_hijacker>) -> () {
        return true;
    }
    fn operator_eq(&self, _a0: Ptr<operator_hijacker>) -> () {
        return true;
    }
    fn operator_call(&self) -> () {
        return 42_();
    }
}
pub fn __cpp2rust_init_globals() {}
