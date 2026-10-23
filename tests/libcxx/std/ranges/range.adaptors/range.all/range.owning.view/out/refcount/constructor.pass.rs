extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct DefaultConstructible {
    #[offset(0)]
    pub i: (),
}
impl DefaultConstructible {
    pub fn new(j: Option<()>) -> Self {
        let mut j: () = j.unwrap_or_else(|| 42_());
        Self { i: j }
    }
}
impl Default for DefaultConstructible {
    fn default() -> Self {
        { DefaultConstructible::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct NotDefaultConstructible {
    #[offset(0)]
    pub i: (),
}
impl NotDefaultConstructible {
    pub fn new(mut j: ()) -> Self {
        Self { i: j }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(4)]
pub struct MoveChecker {
    #[offset(0)]
    pub i: (),
}
impl MoveChecker {
    pub fn new(mut j: ()) -> Self {
        Self { i: j }
    }
    pub fn move_from(v: Ptr<MoveChecker>) -> Self {
        Self {
            i: unimplemented!(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoexceptChecker {}
pub fn test_0() -> () {
    {
        const _: () = assert!(true, "std::is_constructible_v<OwningView>");;
        const _: () = assert!(true, "std::default_initializable<OwningView>");;
        const _: () = assert!(true, "std::movable<OwningView>");;
        const _: () = assert!(true, "std::is_trivially_move_constructible_v<OwningView>");;
        const _: () = assert!(true, "std::is_trivially_move_assignable_v<OwningView>");;
        const _: () = assert!(true, "!std::is_copy_constructible_v<OwningView>");;
        const _: () = assert!(true, "!std::is_copy_assignable_v<OwningView>");;
        const _: () = assert!(true, "!std::is_constructible_v<OwningView, int>");;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<OwningView, DefaultConstructible&>"
        );;
        const _: () = assert!(
            true,
            "std::is_constructible_v<OwningView, DefaultConstructible&&>"
        );;
        const _: () = assert!(true, "!std::is_convertible_v<int, OwningView>");;
        const _: () = assert!(
            true,
            "std::is_convertible_v<DefaultConstructible&&, OwningView>"
        );;
        {
            let ov: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
        }
        {
            let ov: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
        }
    }
    {
        const _: () = assert!(true, "!std::is_constructible_v<OwningView>");;
        const _: () = assert!(!(false), "!std::default_initializable<OwningView>");;
        const _: () = assert!(true, "std::movable<OwningView>");;
        const _: () = assert!(true, "std::is_trivially_move_constructible_v<OwningView>");;
        const _: () = assert!(true, "std::is_trivially_move_assignable_v<OwningView>");;
        const _: () = assert!(true, "!std::is_copy_constructible_v<OwningView>");;
        const _: () = assert!(true, "!std::is_copy_assignable_v<OwningView>");;
        const _: () = assert!(true, "!std::is_constructible_v<OwningView, int>");;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<OwningView, NotDefaultConstructible&>"
        );;
        const _: () = assert!(
            true,
            "std::is_constructible_v<OwningView, NotDefaultConstructible&&>"
        );;
        const _: () = assert!(true, "!std::is_convertible_v<int, OwningView>");;
        const _: () = assert!(
            true,
            "std::is_convertible_v<NotDefaultConstructible&&, OwningView>"
        );;
        {
            let ov: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
        }
    }
    {
        const _: () = assert!(true, "!std::is_constructible_v<OwningView>");;
        const _: () = assert!(!(false), "!std::default_initializable<OwningView>");;
        const _: () = assert!(true, "std::movable<OwningView>");;
        const _: () = assert!(true, "!std::is_trivially_move_constructible_v<OwningView>");;
        const _: () = assert!(true, "!std::is_trivially_move_assignable_v<OwningView>");;
        const _: () = assert!(true, "!std::is_copy_constructible_v<OwningView>");;
        const _: () = assert!(true, "!std::is_copy_assignable_v<OwningView>");;
        const _: () = assert!(true, "!std::is_constructible_v<OwningView, int>");;
        const _: () = assert!(true, "!std::is_constructible_v<OwningView, MoveChecker&>");;
        const _: () = assert!(true, "std::is_constructible_v<OwningView, MoveChecker&&>");;
        const _: () = assert!(true, "!std::is_convertible_v<int, OwningView>");;
        const _: () = assert!(true, "std::is_convertible_v<MoveChecker&&, OwningView>");;
        {
            let m: Value<MoveChecker> = Rc::new(RefCell::new(MoveChecker::new({ 42_() })));
            let ov: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
        }
    }
    {
        const _: () = assert!(true, "std::is_nothrow_constructible_v<NoexceptChecker>");;
        const _: () = assert!(
            true,
            "std::is_nothrow_constructible_v<std::ranges::owning_view<NoexceptChecker>>"
        );;
        const _: () = assert!(
            true,
            "!std::is_nothrow_constructible_v<DefaultConstructible>"
        );;
        const _: () = assert!(
            true,
            "!std::is_nothrow_constructible_v<std::ranges::owning_view<DefaultConstructible>>"
        );;
        const _: () = assert!(
            true,
            "std::is_nothrow_move_constructible_v<NoexceptChecker>"
        );;
        const _: () = assert!(
            true,
            "std::is_nothrow_move_constructible_v<std::ranges::owning_view<NoexceptChecker>>"
        );;
        const _: () = assert!(true, "!std::is_nothrow_move_constructible_v<MoveChecker>");;
        const _: () = assert!(
            true,
            "!std::is_nothrow_move_constructible_v<std::ranges::owning_view<MoveChecker>>"
        );;
    }
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
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait DefaultConstructibleImpl {
    fn begin(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn end(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl DefaultConstructibleImpl for Ptr<DefaultConstructible> {}
pub trait MoveCheckerImpl {
    fn move_assign(&self, _a0: Ptr<MoveChecker>) -> Ptr<MoveChecker> {
        unimplemented!()
    }
    fn begin(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn end(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl MoveCheckerImpl for Ptr<MoveChecker> {}
pub trait NoexceptCheckerImpl {
    fn begin(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn end(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl NoexceptCheckerImpl for Ptr<NoexceptChecker> {}
pub trait NotDefaultConstructibleImpl {
    fn begin(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn end(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl NotDefaultConstructibleImpl for Ptr<NotDefaultConstructible> {}
pub fn __cpp2rust_init_globals() {}
