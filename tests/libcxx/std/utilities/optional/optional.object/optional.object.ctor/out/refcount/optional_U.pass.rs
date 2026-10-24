extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(rhs: Ptr<()>, is_going_to_throw: Option<()>) {
    let mut is_going_to_throw: () = is_going_to_throw.unwrap_or_else(|| false);
    let mut rhs_engaged: () = unimplemented!();
    if is_going_to_throw {
        return;
    }
    let lhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_1(rhs: Ptr<()>, is_going_to_throw: Option<()>) {
    let mut is_going_to_throw: () = is_going_to_throw.unwrap_or_else(|| false);
    let mut rhs_engaged: () = unimplemented!();
    if is_going_to_throw {
        return;
    }
    let lhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_2(rhs: Ptr<()>, is_going_to_throw: Option<()>) {
    let mut is_going_to_throw: () = is_going_to_throw.unwrap_or_else(|| false);
    let mut rhs_engaged: () = unimplemented!();
    if is_going_to_throw {
        return;
    }
    let lhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn operator_eq_3(x: Ptr<X>, y: Ptr<X>) -> () {
    return ({ x.with(|__s| __s.i_) } == { y.with(|__s| __s.i_) });
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Destructor, Default)]
#[byte_size(4)]
pub struct X {
    #[offset(0)]
    i_: (),
}
impl X {
    pub fn new(mut i: ()) -> Self {
        Self { i_: i }
    }
    pub fn move_from(x: Ptr<X>) -> Self {
        Self {
            i_: unimplemented!(),
        }
    }
}
impl std::cmp::PartialEq for X {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_3(
                Rc::new(RefCell::new(X {
                    i_: self.i_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(X {
                    i_: other.i_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for X {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Z {}
impl Z {
    pub fn new(mut _a0: ()) -> Self {
        let __this: Z = Self {};
        unimplemented!();
        __this
    }
}
pub fn test_all_4() -> () {
    {
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_0(unimplemented!(), None) });
    }
    {
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_0(unimplemented!(), None) });
    }
    return true;
}
pub fn test_all_5() -> () {
    {
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_1(unimplemented!(), None) });
    }
    {
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_1(unimplemented!(), None) });
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
    ({ test_all_4() });
    ({ test_all_5() });
    const _: () = assert!(true, "test_all<short, int>()");;
    const _: () = assert!(true, "test_all<int, X>()");;
    {
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_2(unimplemented!(), None) });
    }
    {
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_2(unimplemented!(), Some(true)) });
    }
    const _: () = assert!(
        true,
        "!(std::is_constructible<optional<X>, optional<Z>>::value)"
    );;
    return 0_();
}
pub trait XImpl {
    fn destructor(&self);
}
impl XImpl for Ptr<X> {
    fn destructor(&self) {
        field!((*self), i_).write(0_());
    }
}
pub fn __cpp2rust_init_globals() {}
