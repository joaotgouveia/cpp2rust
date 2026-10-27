extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(1)]
pub struct move_only {}
impl move_only {
    pub fn move_from(_a0: Ptr<move_only>) -> Self {
        Self {}
    }
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for move_only {
    fn default() -> Self {
        { move_only::new() }
    }
}
pub fn source_0() -> move_only {
    return move_only::new();
}
pub fn csource_1() -> move_only {
    return move_only::new();
}
pub fn test_2(mut _a0: move_only) {}
thread_local!(
    pub static x_3: Value<()> = Rc::new(RefCell::new(42_()));
);
thread_local!(
    pub static cx_4: Ptr<()> = x_3.with(|v| v.as_pointer());
);
thread_local!(
    pub static copy_ctor_5: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static move_ctor_6: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor)]
#[byte_size(1)]
pub struct A {}
impl A {
    pub fn new() -> Self {
        Self {}
    }
    pub fn copy_from(_a0: Ptr<A>) -> Self {
        let __this: A = Self {};
        (*copy_ctor_5.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn move_from(_a0: Ptr<A>) -> Self {
        let __this: A = Self {};
        (*move_ctor_6.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for A {
    fn clone(&self) -> Self {
        let __src: Value<A> = Rc::new(RefCell::new(A {}));
        A::copy_from(__src.as_pointer())
    }
}
impl Default for A {
    fn default() -> Self {
        { A::new() }
    }
}
pub fn test_constexpr_move_7() -> () {
    let y: Value<()> = Rc::new(RefCell::new(42_()));
    let mut cy: () = (*y.borrow());
    return ((((unimplemented!().read()) == 42_()) && ((unimplemented!().read()) == 42_()))
        && ((unimplemented!().read()) == 42_()))
        && ((unimplemented!().read()) == 42_());
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
        const _: () = assert!(true, "std::is_same<decltype(std::move(x)), int&&>::value");;
        const _: () = assert!(true, "");;
        const _: () = assert!(
            true,
            "std::is_same<decltype(std::move(cx)), const int&&>::value"
        );;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_same<decltype(std::move(42)), int&&>::value");;
        const _: () = assert!(true, "");;
        const _: () = assert!(
            true,
            "std::is_same<decltype(std::move(get<const int&&>())), const int&&>::value"
        );;
        const _: () = assert!(true, "");;
    }
    {
        let a: Value<A> = Rc::new(RefCell::new(A::new()));
        let ca: Value<A> = Rc::new(RefCell::new(A::new()));
        unimplemented!();
        unimplemented!();
        let mut a2: A = A::copy_from({ a.as_pointer() });
        &(a2);
        unimplemented!();
        unimplemented!();
        let mut a3: A = A::move_from({ unimplemented!() });
        &(a3);
        unimplemented!();
        unimplemented!();
        let mut a4: A = A::copy_from({ ca.as_pointer() });
        &(a4);
        unimplemented!();
        unimplemented!();
        let mut a5: A = A::copy_from({ unimplemented!() });
        &(a5);
        unimplemented!();
        unimplemented!();
    }
    {
        let mut mo: move_only = move_only::new();
        ({ test_2(move_only::move_from({ unimplemented!() })) });
        ({ test_2(({ source_0() })) });
    }
    {
        let mut y: () = 42_();
        const _: () = assert!(true, "std::move(y) == 42");;
        const _: () = assert!(true, "test_constexpr_move()");;
    }
    return 0_();
}
pub trait move_onlyImpl {
    fn copy_assign(&self, _a0: Ptr<move_only>) -> Ptr<move_only> {
        unimplemented!()
    }
    fn move_assign(&self, _a0: Ptr<move_only>) -> Ptr<move_only>;
}
impl move_onlyImpl for Ptr<move_only> {
    fn move_assign(&self, _a0: Ptr<move_only>) -> Ptr<move_only> {
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = x_3.with(|_| ());
    let _ = cx_4.with(|_| ());
    let _ = copy_ctor_5.with(|_| ());
    let _ = move_ctor_6.with(|_| ());
}
