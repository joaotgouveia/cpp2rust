extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign)]
#[byte_size(1)]
pub struct CopyOnly {}
impl CopyOnly {
    pub fn new() -> Self {
        Self {}
    }
    pub fn copy_from(_a0: Ptr<CopyOnly>) -> Self {
        Self {}
    }
}
impl Clone for CopyOnly {
    fn clone(&self) -> Self {
        let __src: Value<CopyOnly> = Rc::new(RefCell::new(CopyOnly {}));
        CopyOnly::copy_from(__src.as_pointer())
    }
}
impl Default for CopyOnly {
    fn default() -> Self {
        { CopyOnly::new() }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(1)]
pub struct MoveOnly {}
impl MoveOnly {
    pub fn new() -> Self {
        Self {}
    }
    pub fn move_from(_a0: Ptr<MoveOnly>) -> Self {
        Self {}
    }
}
impl Default for MoveOnly {
    fn default() -> Self {
        { MoveOnly::new() }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(1)]
pub struct NoexceptMoveOnly {}
impl NoexceptMoveOnly {
    pub fn new() -> Self {
        Self {}
    }
    pub fn move_from(_a0: Ptr<NoexceptMoveOnly>) -> Self {
        Self {}
    }
}
impl Default for NoexceptMoveOnly {
    fn default() -> Self {
        { NoexceptMoveOnly::new() }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveAssign, Default)]
#[byte_size(1)]
pub struct NotMoveConstructible {}
impl NotMoveConstructible {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NotMoveAssignable {}
impl NotMoveAssignable {}
pub fn can_swap_0() -> () {
    return value_1.with(|rc| *rc.borrow());
}
pub fn can_swap_2() -> () {
    return value_1.with(|rc| *rc.borrow());
}
pub fn can_swap_3() -> () {
    return value_1.with(|rc| *rc.borrow());
}
pub fn can_swap_4() -> () {
    return value_5.with(|rc| *rc.borrow());
}
pub fn can_swap_6() -> () {
    return value_5.with(|rc| *rc.borrow());
}
pub fn test_swap_constexpr_7() -> () {
    let mut i: () = 1_();
    let mut j: () = 2_();
    unimplemented!();
    return (i == 2_()) && (j == 1_());
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
        let mut i: () = 1_();
        let mut j: () = 2_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let j: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "can_swap<CopyOnly&>()");;
        const _: () = assert!(true, "can_swap<MoveOnly&>()");;
        const _: () = assert!(true, "can_swap<NoexceptMoveOnly&>()");;
        const _: () = assert!(true, "!can_swap<NotMoveConstructible&>()");;
        const _: () = assert!(true, "!can_swap<NotMoveAssignable&>()");;
        let mut c: CopyOnly = CopyOnly::new();
        let mut m: MoveOnly = MoveOnly::new();
        let mut nm: NoexceptMoveOnly = NoexceptMoveOnly::new();
        const _: () = assert!(true, "!noexcept(std::swap(c, c))");;
        const _: () = assert!(true, "!noexcept(std::swap(m, m))");;
        const _: () = assert!(true, "noexcept(std::swap(nm, nm))");;
    }
    const _: () = assert!(true, "test_swap_constexpr()");;
    return 0_();
}
pub trait CopyOnlyImpl {
    fn copy_assign(&self, _a0: Ptr<CopyOnly>) -> Ptr<CopyOnly>;
}
impl CopyOnlyImpl for Ptr<CopyOnly> {
    fn copy_assign(&self, _a0: Ptr<CopyOnly>) -> Ptr<CopyOnly> {
        return (*self).clone();
    }
}
pub trait MoveOnlyImpl {
    fn move_assign(&self, _a0: Ptr<MoveOnly>) -> Ptr<MoveOnly>;
}
impl MoveOnlyImpl for Ptr<MoveOnly> {
    fn move_assign(&self, _a0: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        return (*self).clone();
    }
}
pub trait NoexceptMoveOnlyImpl {
    fn move_assign(&self, _a0: Ptr<NoexceptMoveOnly>) -> Ptr<NoexceptMoveOnly>;
}
impl NoexceptMoveOnlyImpl for Ptr<NoexceptMoveOnly> {
    fn move_assign(&self, _a0: Ptr<NoexceptMoveOnly>) -> Ptr<NoexceptMoveOnly> {
        return (*self).clone();
    }
}
pub trait NotMoveAssignableImpl {
    fn move_assign(&self, _a0: Ptr<NotMoveAssignable>) -> Ptr<NotMoveAssignable> {
        unimplemented!()
    }
}
impl NotMoveAssignableImpl for Ptr<NotMoveAssignable> {}
pub trait NotMoveConstructibleImpl {
    fn move_assign(&self, _a0: Ptr<NotMoveConstructible>) -> Ptr<NotMoveConstructible>;
}
impl NotMoveConstructibleImpl for Ptr<NotMoveConstructible> {
    fn move_assign(&self, _a0: Ptr<NotMoveConstructible>) -> Ptr<NotMoveConstructible> {
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
