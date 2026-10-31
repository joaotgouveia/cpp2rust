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
pub fn can_swap_0() -> bool {
    return unimplemented!();
}
pub fn can_swap_1() -> bool {
    return unimplemented!();
}
pub fn can_swap_2() -> bool {
    return unimplemented!();
}
pub fn test_unique_ptr_3() {
    let i: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..3).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    let mut k: i32 = 0;
    'loop_: while (k < 3) {
        {
            let _p: Ptr<_> = Ptr::alloc((k + 1));
            (i.as_pointer() as Ptr<Option<Value<i32>>>)
                .offset(k)
                .write(_p.to_owned_opt())
        };
        k.prefix_inc();
    }
    let j: Value<Box<[Option<Value<i32>>]>> = Rc::new(RefCell::new(
        (0..3).map(|_| None).collect::<Box<[Option<Value<i32>>]>>(),
    ));
    let mut k: i32 = 0;
    'loop_: while (k < 3) {
        {
            let _p: Ptr<_> = Ptr::alloc((k + 4));
            (j.as_pointer() as Ptr<Option<Value<i32>>>)
                .offset(k)
                .write(_p.to_owned_opt())
        };
        k.prefix_inc();
    }
    unimplemented!();
    assert!((((*i.borrow())[(0) as usize].as_pointer().read()) == 4));
    assert!((((*i.borrow())[(1) as usize].as_pointer().read()) == 5));
    assert!((((*i.borrow())[(2) as usize].as_pointer().read()) == 6));
    assert!((((*j.borrow())[(0) as usize].as_pointer().read()) == 1));
    assert!((((*j.borrow())[(1) as usize].as_pointer().read()) == 2));
    assert!((((*j.borrow())[(2) as usize].as_pointer().read()) == 3));
}
pub fn test_4() -> bool {
    {
        let mut i: [i32; 3] = [1, 2, 3];
        let mut j: [i32; 3] = [4, 5, 6];
        unimplemented!();
        assert!((i[(0) as usize] == 4));
        assert!((i[(1) as usize] == 5));
        assert!((i[(2) as usize] == 6));
        assert!((j[(0) as usize] == 1));
        assert!((j[(1) as usize] == 2));
        assert!((j[(2) as usize] == 3));
    }
    {
        let a: Value<Box<[Box<[i32]>]>> =
            Rc::new(RefCell::new(Box::new([Box::new([0, 1]), Box::new([2, 3])])));
        let b: Value<Box<[Box<[i32]>]>> =
            Rc::new(RefCell::new(Box::new([Box::new([9, 8]), Box::new([7, 6])])));
        unimplemented!();
        assert!(((*a.borrow())[(0) as usize][(0) as usize] == 9));
        assert!(((*a.borrow())[(0) as usize][(1) as usize] == 8));
        assert!(((*a.borrow())[(1) as usize][(0) as usize] == 7));
        assert!(((*a.borrow())[(1) as usize][(1) as usize] == 6));
        assert!(((*b.borrow())[(0) as usize][(0) as usize] == 0));
        assert!(((*b.borrow())[(0) as usize][(1) as usize] == 1));
        assert!(((*b.borrow())[(1) as usize][(0) as usize] == 2));
        assert!(((*b.borrow())[(1) as usize][(1) as usize] == 3));
    }
    {
        let a: Value<Box<[Box<[i32]>]>> = Rc::new(RefCell::new(Box::new([
            Box::new([0, 1, 2]),
            Box::new([3, 4, 5]),
            Box::new([6, 7, 8]),
        ])));
        let b: Value<Box<[Box<[i32]>]>> = Rc::new(RefCell::new(Box::new([
            Box::new([9, 8, 7]),
            Box::new([6, 5, 4]),
            Box::new([3, 2, 1]),
        ])));
        unimplemented!();
        assert!(((*a.borrow())[(0) as usize][(0) as usize] == 9));
        assert!(((*a.borrow())[(0) as usize][(1) as usize] == 8));
        assert!(((*a.borrow())[(0) as usize][(2) as usize] == 7));
        assert!(((*a.borrow())[(1) as usize][(0) as usize] == 6));
        assert!(((*a.borrow())[(1) as usize][(1) as usize] == 5));
        assert!(((*a.borrow())[(1) as usize][(2) as usize] == 4));
        assert!(((*a.borrow())[(2) as usize][(0) as usize] == 3));
        assert!(((*a.borrow())[(2) as usize][(1) as usize] == 2));
        assert!(((*a.borrow())[(2) as usize][(2) as usize] == 1));
        assert!(((*b.borrow())[(0) as usize][(0) as usize] == 0));
        assert!(((*b.borrow())[(0) as usize][(1) as usize] == 1));
        assert!(((*b.borrow())[(0) as usize][(2) as usize] == 2));
        assert!(((*b.borrow())[(1) as usize][(0) as usize] == 3));
        assert!(((*b.borrow())[(1) as usize][(1) as usize] == 4));
        assert!(((*b.borrow())[(1) as usize][(2) as usize] == 5));
        assert!(((*b.borrow())[(2) as usize][(0) as usize] == 6));
        assert!(((*b.borrow())[(2) as usize][(1) as usize] == 7));
        assert!(((*b.borrow())[(2) as usize][(2) as usize] == 8));
    }
    {
        const _: () = assert!(true, "can_swap<CA&>()");;
        const _: () = assert!(true, "can_swap<MA&>()");;
        const _: () = assert!(true, "!can_swap<NA&>()");;
        let mut ca: [CopyOnly; 42] = std::array::from_fn::<_, 42, _>(|_| CopyOnly::new());
        let mut ma: [NoexceptMoveOnly; 42] =
            std::array::from_fn::<_, 42, _>(|_| NoexceptMoveOnly::new());
        const _: () = assert!(true, "!noexcept(std::swap(ca, ca))");;
        const _: () = assert!(true, "noexcept(std::swap(ma, ma))");;
    }
    {
        ({ test_unique_ptr_3() });
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_4() });
    const _: () = assert!(true, "test()");;
    return 0;
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
