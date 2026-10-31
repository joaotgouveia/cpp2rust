extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(rhs: Ptr<()>, is_going_to_throw: Option<bool>) {
    let mut is_going_to_throw: bool = is_going_to_throw.unwrap_or_else(|| false);
    let mut rhs_engaged: bool = unimplemented!();
    if is_going_to_throw {
        return;
    }
    let lhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(((unimplemented!() as i32) == (rhs_engaged as i32)));
}
pub fn test_1(rhs: Ptr<()>, is_going_to_throw: Option<bool>) {
    let mut is_going_to_throw: bool = is_going_to_throw.unwrap_or_else(|| false);
    let mut rhs_engaged: bool = unimplemented!();
    if is_going_to_throw {
        return;
    }
    let lhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(((unimplemented!() as i32) == (rhs_engaged as i32)));
}
pub fn test_2(rhs: Ptr<()>, is_going_to_throw: Option<bool>) {
    let mut is_going_to_throw: bool = is_going_to_throw.unwrap_or_else(|| false);
    let mut rhs_engaged: bool = unimplemented!();
    if is_going_to_throw {
        return;
    }
    let lhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(((unimplemented!() as i32) == (rhs_engaged as i32)));
}
pub fn operator_eq_3(x: Ptr<X>, y: Ptr<X>) -> bool {
    return ({ x.with(|__s| __s.i_) } == { y.with(|__s| __s.i_) });
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, CppFrom, Destructor, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct X {
    #[offset(0)]
    i_: i32,
}
impl X {
    pub fn new(mut i: i32) -> Self {
        Self { i_: i }
    }
    pub fn move_from(x: Ptr<X>) -> Self {
        Self {
            i_: unimplemented!(),
        }
    }
}
impl CppEq for X {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_3(__lhs.clone(), __rhs.clone())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(1)]
pub struct Z {}
impl Z {
    pub fn new(mut _a0: i32) -> Self {
        let __this: Z = Self {};
        std::process::abort();
        __this
    }
}
pub fn test_all_4() -> bool {
    {
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_0(rhs.as_pointer(), None) });
    }
    {
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_0(rhs.as_pointer(), None) });
    }
    return true;
}
pub fn test_all_5() -> bool {
    {
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_1(rhs.as_pointer(), None) });
    }
    {
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_1(rhs.as_pointer(), None) });
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
    ({ test_all_4() });
    ({ test_all_5() });
    const _: () = assert!(true, "test_all<short, int>()");;
    const _: () = assert!(true, "test_all<int, X>()");;
    {
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_2(rhs.as_pointer(), None) });
    }
    {
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_2(rhs.as_pointer(), Some(true)) });
    }
    const _: () = assert!(
        true,
        "!(std::is_constructible<optional<X>, optional<Z>>::value)"
    );;
    return 0;
}
pub trait XImpl {
    fn destructor(&self);
}
impl XImpl for Ptr<X> {
    fn destructor(&self) {
        field!((*self), i_).write(0);
    }
}
pub fn __cpp2rust_init_globals() {}
