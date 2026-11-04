extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(x: Ptr<X>, y: Ptr<X>) -> bool {
    return ({ x.with(|__s| __s.i_) } == { y.with(|__s| __s.i_) })
        && ({ x.with(|__s| __s.j_) } == { y.with(|__s| __s.j_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Destructor)]
#[cpp_from(bool, new_1, mut_ptr)]
#[byte_size(16)]
pub struct X {
    #[offset(0)]
    i_: i32,
    #[offset(4)]
    j_: i32,
    #[offset(8)]
    #[byte_size(8)]
    dtor_called_: Ptr<bool>,
}
impl X {
    pub fn new_1(dtor_called: Ptr<bool>) -> Self {
        Self {
            i_: 0,
            j_: 0,
            dtor_called_: (dtor_called).clone(),
        }
    }
    pub fn new_2(mut i: i32, dtor_called: Ptr<bool>) -> Self {
        Self {
            i_: i,
            j_: 0,
            dtor_called_: (dtor_called).clone(),
        }
    }
    pub fn new_3(il: Vec<i32>, dtor_called: Ptr<bool>) -> Self {
        let il: Value<Vec<i32>> = Rc::new(RefCell::new(il));
        Self {
            i_: (elem!(unimplemented!(), 0).read()),
            j_: (elem!(unimplemented!(), 1).read()),
            dtor_called_: (dtor_called).clone(),
        }
    }
}
impl CppEq for X {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_0(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for X {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for X {
    fn default() -> Self {
        {
            X {
                i_: 0_i32,
                j_: 0,
                dtor_called_: Ptr::<bool>::null(),
            }
        }
    }
}
pub fn operator_eq_1(x: Ptr<Y>, y: Ptr<Y>) -> bool {
    return ({ x.with(|__s| __s.i_) } == { y.with(|__s| __s.i_) })
        && ({ x.with(|__s| __s.j_) } == { y.with(|__s| __s.j_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(i32, new_1, value)]
#[cpp_from(Vec<i32  >  , new_2, value)]
#[byte_size(8)]
pub struct Y {
    #[offset(0)]
    i_: i32,
    #[offset(4)]
    j_: i32,
}
impl Y {
    pub fn new_3() -> Self {
        Self { i_: 0, j_: 0 }
    }
    pub fn new_1(mut i: i32) -> Self {
        Self { i_: i, j_: 0 }
    }
    pub fn new_2(il: Vec<i32>) -> Self {
        let il: Value<Vec<i32>> = Rc::new(RefCell::new(il));
        Self {
            i_: (elem!(unimplemented!(), 0).read()),
            j_: (elem!(unimplemented!(), 1).read()),
        }
    }
}
impl CppEq for Y {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_1(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for Y {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for Y {
    fn default() -> Self {
        { Y::new_3() }
    }
}
pub fn operator_eq_3(x: Ptr<Z>, y: Ptr<Z>) -> bool {
    return ({ x.with(|__s| __s.i_) } == { y.with(|__s| __s.i_) })
        && ({ x.with(|__s| __s.j_) } == { y.with(|__s| __s.j_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Destructor)]
#[cpp_from(i32, new_1, value)]
#[cpp_from(Vec<i32  >  , new_2, value)]
#[byte_size(8)]
pub struct Z {
    #[offset(0)]
    i_: i32,
    #[offset(4)]
    j_: i32,
}
impl Z {
    pub fn new_3() -> Self {
        Self { i_: 0, j_: 0 }
    }
    pub fn new_1(mut i: i32) -> Self {
        Self { i_: i, j_: 0 }
    }
    pub fn new_2(il: Vec<i32>) -> Self {
        let il: Value<Vec<i32>> = Rc::new(RefCell::new(il));
        let __this: Z = Self {
            i_: (elem!(unimplemented!(), 0).read()),
            j_: (elem!(unimplemented!(), 1).read()),
        };
        std::process::abort();
        __this
    }
}
impl CppEq for Z {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_3(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for Z {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for Z {
    fn default() -> Self {
        { Z::new_3() }
    }
}
thread_local!(
    pub static dtor_called_2: Value<bool> = Rc::new(RefCell::new(false));
);
pub fn check_X_4() -> bool {
    let dtor_called: Value<bool> = Rc::new(RefCell::new(false));
    let x: Value<X> = Rc::new(RefCell::new(X::new_1({ dtor_called.as_pointer() })));
    let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
    let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(({ (((*dtor_called.borrow()) as i32) == (false as i32)) }));
    let v: Ptr<X> = unimplemented!();
    const _: () = assert!(true, "std::is_same_v<X&, decltype(v)>");;
    assert!(({ (*dtor_called.borrow()) }));
    assert!(
        ({
            let _x: Ptr<X> = unimplemented!();
            let _y: Value<X> = Rc::new(RefCell::new(X::new_3({ Vec::from([1, 2]) }, {
                dtor_called.as_pointer()
            })));
            operator_eq_0(_x, _y.as_pointer())
        })
    );
    assert!(({ (v).clone() } == { (unimplemented!()) }));
    return true;
}
pub fn check_Y_5() -> bool {
    let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let v: Ptr<Y> = unimplemented!();
    const _: () = assert!(true, "std::is_same_v<Y&, decltype(v)>");;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(
        ({
            let _x: Ptr<Y> = unimplemented!();
            let _y: Value<Y> = Rc::new(RefCell::new(Y::new_2({ Vec::from([1, 2]) })));
            operator_eq_1(_x, _y.as_pointer())
        })
    );
    assert!(({ (v).clone() } == { (unimplemented!()) }));
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
    {
        ({ check_X_4() });
        const _: () = assert!(true, "check_X()");;
    }
    {
        let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v: Ptr<Vec<i32>> = unimplemented!();
        const _: () = assert!(true, "std::is_same_v<std::vector<int>&, decltype(v)>");;
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(unimplemented!());
        assert!(({ (v).clone() } == { (unimplemented!()) }));
    }
    {
        ({ check_Y_5() });
        const _: () = assert!(true, "check_Y()");;
    }
    return 0;
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
