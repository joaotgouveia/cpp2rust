extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(x: Ptr<X>, y: Ptr<X>) -> () {
    return ({ x.with(|__s| __s.i_) } == { y.with(|__s| __s.i_) })
        && ({ x.with(|__s| __s.j_) } == { y.with(|__s| __s.j_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct X {
    #[offset(0)]
    i_: (),
    #[offset(4)]
    j_: (),
    #[offset(8)]
    #[byte_size(8)]
    dtor_called_: Ptr<()>,
}
impl X {
    pub fn new_1(dtor_called: Ptr<()>) -> Self {
        Self {
            i_: 0_(),
            j_: 0_(),
            dtor_called_: (dtor_called).clone(),
        }
    }
    pub fn new_2(mut i: (), dtor_called: Ptr<()>) -> Self {
        Self {
            i_: i,
            j_: 0_(),
            dtor_called_: (dtor_called).clone(),
        }
    }
    pub fn new_3(il: (), dtor_called: Ptr<()>) -> Self {
        let il: Value<()> = Rc::new(RefCell::new(il));
        Self {
            i_: (elem!(unimplemented!(), 0_()).read()),
            j_: (elem!(unimplemented!(), 1_()).read()),
            dtor_called_: (dtor_called).clone(),
        }
    }
}
impl CppEq for X {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_0(__lhs.clone(), __rhs.clone())
    }
}
impl Default for X {
    fn default() -> Self {
        {
            X {
                i_: unimplemented!(),
                j_: 0_(),
                dtor_called_: Ptr::<()>::null(),
            }
        }
    }
}
pub fn operator_eq_1(x: Ptr<Y>, y: Ptr<Y>) -> () {
    return ({ x.with(|__s| __s.i_) } == { y.with(|__s| __s.i_) })
        && ({ x.with(|__s| __s.j_) } == { y.with(|__s| __s.j_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Y {
    #[offset(0)]
    i_: (),
    #[offset(4)]
    j_: (),
}
impl Y {
    pub fn new_1() -> Self {
        Self { i_: 0_(), j_: 0_() }
    }
    pub fn new_2(mut i: ()) -> Self {
        Self { i_: i, j_: 0_() }
    }
    pub fn new_3(il: ()) -> Self {
        let il: Value<()> = Rc::new(RefCell::new(il));
        Self {
            i_: (elem!(unimplemented!(), 0_()).read()),
            j_: (elem!(unimplemented!(), 1_()).read()),
        }
    }
}
impl CppEq for Y {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_1(__lhs.clone(), __rhs.clone())
    }
}
impl Default for Y {
    fn default() -> Self {
        { Y::new_1() }
    }
}
pub fn operator_eq_3(x: Ptr<Z>, y: Ptr<Z>) -> () {
    return ({ x.with(|__s| __s.i_) } == { y.with(|__s| __s.i_) })
        && ({ x.with(|__s| __s.j_) } == { y.with(|__s| __s.j_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct Z {
    #[offset(0)]
    i_: (),
    #[offset(4)]
    j_: (),
}
impl Z {
    pub fn new_1() -> Self {
        Self { i_: 0_(), j_: 0_() }
    }
    pub fn new_2(mut i: ()) -> Self {
        Self { i_: i, j_: 0_() }
    }
    pub fn new_3(il: ()) -> Self {
        let il: Value<()> = Rc::new(RefCell::new(il));
        let __this: Z = Self {
            i_: (elem!(unimplemented!(), 0_()).read()),
            j_: (elem!(unimplemented!(), 1_()).read()),
        };
        unimplemented!();
        __this
    }
}
impl CppEq for Z {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_3(__lhs.clone(), __rhs.clone())
    }
}
impl Default for Z {
    fn default() -> Self {
        { Z::new_1() }
    }
}
thread_local!(
    pub static dtor_called_2: Value<()> = Rc::new(RefCell::new(false));
);
pub fn check_X_4() -> () {
    let dtor_called: Value<()> = Rc::new(RefCell::new(false));
    let x: Value<X> = Rc::new(RefCell::new(X::new_1({ dtor_called.as_pointer() })));
    let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
    let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let v: Ptr<X> = unimplemented!();
    const _: () = assert!(true, "std::is_same_v<X&, decltype(v)>");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn check_Y_5() -> () {
    let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let v: Ptr<Y> = unimplemented!();
    const _: () = assert!(true, "std::is_same_v<Y&, decltype(v)>");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    {
        ({ check_X_4() });
        const _: () = assert!(true, "check_X()");;
    }
    {
        let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v: Ptr<()> = unimplemented!();
        const _: () = assert!(true, "std::is_same_v<std::vector<int>&, decltype(v)>");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        ({ check_Y_5() });
        const _: () = assert!(true, "check_Y()");;
    }
    return 0_();
}
pub trait XImpl {
    fn destructor(&self);
}
impl XImpl for Ptr<X> {
    fn destructor(&self) {
        (*self).with(|__s| __s.dtor_called_.clone()).write(true);
    }
}
pub trait ZImpl {
    fn destructor(&self);
}
impl ZImpl for Ptr<Z> {
    fn destructor(&self) {
        dtor_called_2.with(|rc| *rc.borrow_mut() = true);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = dtor_called_2.with(|_| ());
}
