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
#[derive(Record, ByteRepr, FnPtrArg, MoveAssign)]
#[byte_size(1)]
pub struct NotMoveConstructible {}
impl NotMoveConstructible {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for NotMoveConstructible {
    fn default() -> Self {
        { NotMoveConstructible::new() }
    }
}
pub fn can_swap_0() -> () {
    return value_1.with(|rc| *rc.borrow());
}
pub fn can_swap_2() -> () {
    return value_1.with(|rc| *rc.borrow());
}
pub fn can_swap_3() -> () {
    return value_4.with(|rc| *rc.borrow());
}
pub fn test_unique_ptr_5() {
    let i: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..3).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    let mut k: () = 0_();
    'loop_: while (k < 3_()) {
        unimplemented!();
        k.prefix_inc();
    }
    let j: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..3).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    let mut k: () = 0_();
    'loop_: while (k < 3_()) {
        unimplemented!();
        k.prefix_inc();
    }
    ({ swap_6(i.as_pointer(), j.as_pointer()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_7() -> () {
    {
        let i: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
        let j: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([4_(), 5_(), 6_()])));
        ({ swap_8(i.as_pointer(), j.as_pointer()) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let a: Value<Box<[Box<[()]>]>> = Rc::new(RefCell::new(Box::new([
            Box::new([0_(), 1_()]),
            Box::new([2_(), 3_()]),
        ])));
        let b: Value<Box<[Box<[()]>]>> = Rc::new(RefCell::new(Box::new([
            Box::new([9_(), 8_()]),
            Box::new([7_(), 6_()]),
        ])));
        ({ swap_9(a.as_pointer(), b.as_pointer()) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let a: Value<Box<[Box<[()]>]>> = Rc::new(RefCell::new(Box::new([
            Box::new([0_(), 1_(), 2_()]),
            Box::new([3_(), 4_(), 5_()]),
            Box::new([6_(), 7_(), 8_()]),
        ])));
        let b: Value<Box<[Box<[()]>]>> = Rc::new(RefCell::new(Box::new([
            Box::new([9_(), 8_(), 7_()]),
            Box::new([6_(), 5_(), 4_()]),
            Box::new([3_(), 2_(), 1_()]),
        ])));
        ({ swap_10(a.as_pointer(), b.as_pointer()) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "can_swap<CA&>()");;
        const _: () = assert!(true, "can_swap<MA&>()");;
        const _: () = assert!(true, "!can_swap<NA&>()");;
        let ca: Value<Box<[CopyOnly]>> =
            Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 42, _>(
                |_| CopyOnly::new(),
            ))));
        let ma: Value<Box<[NoexceptMoveOnly]>> = Rc::new(RefCell::new(Box::new(
            std::array::from_fn::<_, 42, _>(|_| NoexceptMoveOnly::new()),
        )));
        const _: () = assert!(true, "!noexcept(std::swap(ca, ca))");;
        const _: () = assert!(true, "noexcept(std::swap(ma, ma))");;
    }
    {
        ({ test_unique_ptr_5() });
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
    ({ test_7() });
    const _: () = assert!(true, "test()");;
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
pub trait NoexceptMoveOnlyImpl {
    fn move_assign(&self, _a0: Ptr<NoexceptMoveOnly>) -> Ptr<NoexceptMoveOnly>;
}
impl NoexceptMoveOnlyImpl for Ptr<NoexceptMoveOnly> {
    fn move_assign(&self, _a0: Ptr<NoexceptMoveOnly>) -> Ptr<NoexceptMoveOnly> {
        return (*self).clone();
    }
}
pub trait NotMoveConstructibleImpl {
    fn move_assign(&self, _a0: Ptr<NotMoveConstructible>) -> Ptr<NotMoveConstructible>;
}
impl NotMoveConstructibleImpl for Ptr<NotMoveConstructible> {
    fn move_assign(&self, _a0: Ptr<NotMoveConstructible>) -> Ptr<NotMoveConstructible> {
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
