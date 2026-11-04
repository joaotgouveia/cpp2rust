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
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, CppFrom, Default)]
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
    pub fn move_from(x: Ptr<X>) -> Self {
        let __this: X = Self {
            i_: x.with(|__s| __s.i_),
        };
        field!(x, i_).write(0);
        __this
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
        Self::move_from(src)
    }
}
pub fn operator_eq_1(z1: Ptr<Z>, z2: Ptr<Z>) -> bool {
    return ({ z1.with(|__s| __s.i_) } == { z2.with(|__s| __s.i_) })
        && ({ z1.with(|__s| __s.j_) } == { z2.with(|__s| __s.j_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Z {
    #[offset(0)]
    pub i_: i32,
    #[offset(4)]
    pub j_: i32,
}
impl Z {
    pub fn new(mut i: i32, mut j: i32) -> Self {
        Self { i_: i, j_: j }
    }
}
impl CppEq for Z {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_1(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for Z {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_2() -> i32 {
    {
        let opt: Value<std_optional_X_> = Rc::new(RefCell::new({
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(2));
            std_optional_X_::new_1({ (*in_place_3.with(Value::clone).borrow()).clone() }, {
                __tmp_0.as_pointer()
            })
        }));
        let y: Value<Y> = Rc::new(RefCell::new(Y::new({ 3 })));
        assert!(
            ({
                let _x: Value<X> =
                    Rc::new(RefCell::new(({ (*opt.borrow()).value_or(y.as_pointer()) })));
                let _y: Value<X> = Rc::new(RefCell::new(X::new_1({ 2 })));
                operator_eq_0(_x.as_pointer(), _y.as_pointer())
            })
        );
        assert!(
            ({
                let _x: Ptr<X> = opt.as_pointer();
                let _y: Value<X> = Rc::new(RefCell::new(X::new_1({ 0 })));
                operator_eq_0(_x, _y.as_pointer())
            })
        );
    }
    {
        let opt: Value<std_optional_X_> = Rc::new(RefCell::new({
            let __tmp_1: Value<i32> = Rc::new(RefCell::new(2));
            std_optional_X_::new_1({ (*in_place_3.with(Value::clone).borrow()).clone() }, {
                __tmp_1.as_pointer()
            })
        }));
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
        assert!(
            ({
                let _x: Ptr<X> = opt.as_pointer();
                let _y: Value<X> = Rc::new(RefCell::new(X::new_1({ 0 })));
                operator_eq_0(_x, _y.as_pointer())
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
        assert!(({ !({ (*opt.borrow())() }) }));
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
        assert!(({ !({ (*opt.borrow())() }) }));
    }
    {
        let opt: Value<std_optional_X_> = Rc::new(RefCell::new(<std_optional_X_>::default()));
        assert!(
            ({
                let _x: Value<X> = Rc::new(RefCell::new(
                    ({
                        let ___u: Value<X> = Rc::new(RefCell::new({
                            let __tmp_2: Value<Y> = Rc::new(RefCell::new(Y::new({ 3 })));
                            X::new_3({ __tmp_2.as_pointer() })
                        }));
                        (*opt.borrow()).value_or(___u.as_pointer())
                    }),
                ));
                let _y: Value<X> = Rc::new(RefCell::new(X::new_1({ 4 })));
                operator_eq_0(_x.as_pointer(), _y.as_pointer())
            })
        );
        assert!(({ !({ (*opt.borrow())() }) }));
    }
    {
        let opt: Value<std_optional_Z_> = Rc::new(RefCell::new(<std_optional_Z_>::default()));
        assert!(
            ({
                let _z1: Value<Z> = Rc::new(RefCell::new(
                    ({
                        let ___u: Value<Z> = Rc::new(RefCell::new(Z::new({ 2 }, { 3 })));
                        (*opt.borrow()).value_or(___u.as_pointer())
                    }),
                ));
                let _z2: Value<Z> = Rc::new(RefCell::new(Z::new({ 2 }, { 3 })));
                operator_eq_1(_z1.as_pointer(), _z2.as_pointer())
            })
        );
        assert!(({ !({ (*opt.borrow())() }) }));
    }
    return 0;
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
    assert!((({ test_2() }) == 0));
    const _: () = assert!(true, "test() == 0");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
