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
    let mut rhs_engaged: () = ({ (*rhs.upgrade().deref())() });
    if is_going_to_throw {
        return;
    }
    let lhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    if rhs_engaged {
        unimplemented!();
    }
}
pub fn test_1(rhs: Ptr<()>, is_going_to_throw: Option<()>) {
    let mut is_going_to_throw: () = is_going_to_throw.unwrap_or_else(|| false);
    let mut rhs_engaged: () = ({ (*rhs.upgrade().deref())() });
    if is_going_to_throw {
        return;
    }
    let lhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    if rhs_engaged {
        unimplemented!();
    }
}
pub fn test_2(rhs: Ptr<()>, is_going_to_throw: Option<()>) {
    let mut is_going_to_throw: () = is_going_to_throw.unwrap_or_else(|| false);
    let mut rhs_engaged: () = ({ (*rhs.upgrade().deref())() });
    if is_going_to_throw {
        return;
    }
    let lhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    if rhs_engaged {
        unimplemented!();
    }
}
pub fn test_3(rhs: Ptr<()>, is_going_to_throw: Option<()>) {
    let mut is_going_to_throw: () = is_going_to_throw.unwrap_or_else(|| false);
    let mut rhs_engaged: () = ({ (*rhs.upgrade().deref())() });
    if is_going_to_throw {
        return;
    }
    let lhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    if rhs_engaged {
        unimplemented!();
    }
}
pub fn operator_eq_4(x: Ptr<X>, y: Ptr<X>) -> () {
    return ({ x.with(|__s| __s.i_) } == { y.with(|__s| __s.i_) });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor, Default)]
#[byte_size(4)]
pub struct X {
    #[offset(0)]
    i_: (),
}
impl X {
    pub fn new(mut i: ()) -> Self {
        Self { i_: i }
    }
    pub fn copy_from(x: Ptr<X>) -> Self {
        Self {
            i_: x.with(|__s| __s.i_),
        }
    }
}
impl std::cmp::PartialEq for X {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_4(
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
impl Clone for X {
    fn clone(&self) -> Self {
        let __src: Value<X> = Rc::new(RefCell::new(X {
            i_: self.i_.clone(),
        }));
        X::copy_from(__src.as_pointer())
    }
}
pub fn operator_eq_5(x: Ptr<Y>, y: Ptr<Y>) -> () {
    return ({ x.with(|__s| __s.i_) } == { y.with(|__s| __s.i_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Y {
    #[offset(0)]
    i_: (),
}
impl Y {
    pub fn new(mut i: ()) -> Self {
        Self { i_: i }
    }
}
impl std::cmp::PartialEq for Y {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_5(
                Rc::new(RefCell::new(Y {
                    i_: self.i_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Y {
                    i_: other.i_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Y {}
thread_local!(
    pub static count_6: Value<()> = Rc::new(RefCell::new(0_()));
);
pub fn operator_eq_7(x: Ptr<Z>, y: Ptr<Z>) -> () {
    return ({ x.with(|__s| __s.i_) } == { y.with(|__s| __s.i_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Z {
    #[offset(0)]
    i_: (),
}
impl Z {
    pub fn new(mut i: ()) -> Self {
        let __this: Z = Self { i_: i };
        unimplemented!();
        __this
    }
}
impl std::cmp::PartialEq for Z {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_7(
                Rc::new(RefCell::new(Z {
                    i_: self.i_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Z {
                    i_: other.i_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Z {}
pub fn test_all_8() -> () {
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
pub fn test_all_9() -> () {
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
pub fn test_all_10() -> () {
    {
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_2(rhs.as_pointer(), None) });
    }
    {
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_2(rhs.as_pointer(), None) });
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
    ({ test_all_8() });
    ({ test_all_9() });
    ({ test_all_10() });
    const _: () = assert!(true, "test_all<int, short>()");;
    const _: () = assert!(true, "test_all<X, int>()");;
    const _: () = assert!(true, "test_all<Y, int>()");;
    {
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_3(rhs.as_pointer(), None) });
    }
    {
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_3(rhs.as_pointer(), Some(true)) });
    }
    const _: () = assert!(
        true,
        "!(std::is_constructible<optional<X>, const optional<Y>&>::value)"
    );;
    return 0_();
}
pub trait XImpl {
    fn destructor(&self);
    fn copy_assign(&self, _a0: Ptr<X>) -> Ptr<X>;
}
impl XImpl for Ptr<X> {
    fn destructor(&self) {
        field!((*self), i_).write(0_());
    }
    fn copy_assign(&self, _a0: Ptr<X>) -> Ptr<X> {
        field!((*self), i_).write({ _a0.with(|__s| __s.i_) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = count_6.with(|_| ());
}
