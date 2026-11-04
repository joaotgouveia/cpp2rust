extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct Y {
    #[offset(0)]
    pub i_: i32,
}
impl Y {
    pub fn new(mut i: i32) -> Self {
        Self { i_: i }
    }
}
impl CppMove for Y {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_0(x: Ptr<X>, y: Ptr<X>) -> bool {
    return ({ x.with(|__s| __s.i_) } == { y.with(|__s| __s.i_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new_1, value)]
#[cpp_from(Y, new_2, ptr)]
#[cpp_from_move(Y, new_3, ptr)]
#[byte_size(4)]
pub struct X {
    #[offset(0)]
    pub i_: i32,
}
impl X {
    pub fn new_1(mut i: i32) -> Self {
        Self { i_: i }
    }
    pub fn new_2(y: Ptr<Y>) -> Self {
        Self {
            i_: y.with(|__s| __s.i_),
        }
    }
    pub fn new_3(y: Ptr<Y>) -> Self {
        Self {
            i_: (y.with(|__s| __s.i_) + 1),
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
        let opt: Value<std_optional_X_> = Rc::new(RefCell::new(2));
        let y: Value<Y> = Rc::new(RefCell::new(Y::new({ 3 })));
        const _: () = assert!(true, "opt.value_or(y) == 2");;
    }
    {
        let opt: Value<std_optional_X_> = Rc::new(RefCell::new(2));
        const _: () = assert!(true, "opt.value_or(Y(3)) == 2");;
    }
    {
        let opt: Value<std_optional_X_> = Rc::new(RefCell::new(<std_optional_X_>::default()));
        let y: Value<Y> = Rc::new(RefCell::new(Y::new({ 3 })));
        const _: () = assert!(true, "opt.value_or(y) == 3");;
    }
    {
        let opt: Value<std_optional_X_> = Rc::new(RefCell::new(<std_optional_X_>::default()));
        const _: () = assert!(true, "opt.value_or(Y(3)) == 4");;
    }
    {
        let opt: Value<std_optional_X_> = Rc::new(RefCell::new(2));
        let y: Value<Y> = Rc::new(RefCell::new(Y::new({ 3 })));
        assert!(
            ({
                let _x: Value<X> =
                    Rc::new(RefCell::new(({ (*opt.borrow()).value_or(y.as_pointer()) })));
                let _y: Value<X> = Rc::new(RefCell::new(X::new_1({ 2 })));
                operator_eq_0(_x.as_pointer(), _y.as_pointer())
            })
        );
    }
    {
        let opt: Value<std_optional_X_> = Rc::new(RefCell::new(2));
        assert!(
            ({
                let _x: Value<X> = Rc::new(RefCell::new(
                    ({
                        let ___u: Value<Y> = Rc::new(RefCell::new(Y::new({ 3 })));
                        (*opt.borrow()).value_or(___u.as_pointer())
                    }),
                ));
                let _y: Value<X> = Rc::new(RefCell::new(X::new_1({ 2 })));
                operator_eq_0(_x.as_pointer(), _y.as_pointer())
            })
        );
    }
    {
        let opt: Value<std_optional_X_> = Rc::new(RefCell::new(<std_optional_X_>::default()));
        let y: Value<Y> = Rc::new(RefCell::new(Y::new({ 3 })));
        assert!(
            ({
                let _x: Value<X> =
                    Rc::new(RefCell::new(({ (*opt.borrow()).value_or(y.as_pointer()) })));
                let _y: Value<X> = Rc::new(RefCell::new(X::new_1({ 3 })));
                operator_eq_0(_x.as_pointer(), _y.as_pointer())
            })
        );
    }
    {
        let opt: Value<std_optional_X_> = Rc::new(RefCell::new(<std_optional_X_>::default()));
        assert!(
            ({
                let _x: Value<X> = Rc::new(RefCell::new(
                    ({
                        let ___u: Value<Y> = Rc::new(RefCell::new(Y::new({ 3 })));
                        (*opt.borrow()).value_or(___u.as_pointer())
                    }),
                ));
                let _y: Value<X> = Rc::new(RefCell::new(X::new_1({ 4 })));
                operator_eq_0(_x.as_pointer(), _y.as_pointer())
            })
        );
    }
    {
        let opt: Value<std_optional_X_> = Rc::new(RefCell::new(<std_optional_X_>::default()));
        assert!(
            ({
                let _x: Value<X> = Rc::new(RefCell::new(
                    ({
                        let ___u: Value<X> = Rc::new(RefCell::new({
                            let __tmp_0: Value<Y> = Rc::new(RefCell::new(Y::new({ 3 })));
                            X::new_3({ __tmp_0.as_pointer() })
                        }));
                        (*opt.borrow()).value_or(___u.as_pointer())
                    }),
                ));
                let _y: Value<X> = Rc::new(RefCell::new(X::new_1({ 4 })));
                operator_eq_0(_x.as_pointer(), _y.as_pointer())
            })
        );
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
