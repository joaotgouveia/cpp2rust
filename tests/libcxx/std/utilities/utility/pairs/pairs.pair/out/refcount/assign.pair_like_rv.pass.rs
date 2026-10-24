extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> () {
    {
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Ptr<()> = (unimplemented!());
            unimplemented!();
            unimplemented!();
            unimplemented!();
            const _: () = assert!(
                true,
                "!std::is_assignable_v<std::pair<int, int>&, std::array<int, 1>>"
            );;
            const _: () = assert!(
                true,
                "std::is_assignable_v<std::pair<int, int>&, std::array<int, 2>>"
            );;
            const _: () = assert!(
                true,
                "!std::is_assignable_v<std::pair<int, int>&, std::array<int, 3>>"
            );;
        }
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Ptr<()> = (unimplemented!());
            unimplemented!();
            unimplemented!();
            unimplemented!();
            const _: () = assert!(
                true,
                "!std::is_assignable_v<std::pair<int, int>&, std::tuple<int>>"
            );;
            const _: () = assert!(
                true,
                "std::is_assignable_v<std::pair<int, int>&, std::tuple<int, int>>"
            );;
            const _: () = assert!(
                true,
                "!std::is_assignable_v<std::pair<int, int>&, std::tuple<int, int, int>>"
            );;
        }
        {
            let data: Value<Box<[()]>> =
                Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_()])));
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Ptr<()> = ((*p.borrow_mut()) = ({ (*a.borrow())() }));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Ptr<()> = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(
            true,
            "!std::is_assignable_v<std::pair<long, std::string>&, std::tuple<char*, std::string>>"
        );;
        const _: () = assert!(
            true,
            "!std::is_assignable_v<std::pair<long, std::string>&, std::tuple<long, void*>>"
        );;
        const _: () = assert!(
            true,
            "std::is_assignable_v<std::pair<long, std::string>&, std::tuple<long, std::string>>"
        );;
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Assignable_1 {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
}
impl Assignable_1 {
    pub fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr: (p).clone() }
    }
}
impl Default for Assignable_1 {
    fn default() -> Self {
        {
            Assignable_1 {
                ptr: Ptr::<()>::null(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(1)]
pub struct NoCopy_2 {}
impl NoCopy_2 {
    pub fn move_from(_a0: Ptr<NoCopy_2>) -> Self {
        Self {}
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
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait NoCopy_2Impl {
    fn move_assign(&self, _a0: Ptr<NoCopy_2>) -> Ptr<NoCopy_2>;
}
impl NoCopy_2Impl for Ptr<NoCopy_2> {
    fn move_assign(&self, _a0: Ptr<NoCopy_2>) -> Ptr<NoCopy_2> {
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
