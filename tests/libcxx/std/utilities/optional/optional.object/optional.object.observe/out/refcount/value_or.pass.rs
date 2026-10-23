extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Y {
    #[offset(0)]
    pub i_: (),
}
impl Y {
    pub fn new(mut i: ()) -> Self {
        Self { i_: i }
    }
}
pub fn operator_eq_0(x: Ptr<X>, y: Ptr<X>) -> () {
    return ({ x.with(|__s| __s.i_) } == { y.with(|__s| __s.i_) });
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(4)]
pub struct X {
    #[offset(0)]
    pub i_: (),
}
impl X {
    pub fn new_1(mut i: ()) -> Self {
        Self { i_: i }
    }
    pub fn move_from(x: Ptr<X>) -> Self {
        let __this: X = Self {
            i_: x.with(|__s| __s.i_),
        };
        field!(x, i_).write(0_());
        __this
    }
    pub fn new_2(y: Ptr<Y>) -> Self {
        Self {
            i_: y.with(|__s| __s.i_),
        }
    }
    pub fn new_3(y: Ptr<Y>) -> Self {
        Self {
            i_: (y.with(|__s| __s.i_) + 1_()),
        }
    }
}
impl std::cmp::PartialEq for X {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_0(
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
pub fn operator_eq_1(z1: Ptr<Z>, z2: Ptr<Z>) -> () {
    return ({ z1.with(|__s| __s.i_) } == { z2.with(|__s| __s.i_) })
        && ({ z1.with(|__s| __s.j_) } == { z2.with(|__s| __s.j_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Z {
    #[offset(0)]
    pub i_: (),
    #[offset(4)]
    pub j_: (),
}
impl Z {
    pub fn new(mut i: (), mut j: ()) -> Self {
        Self { i_: i, j_: j }
    }
}
impl std::cmp::PartialEq for Z {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_1(
                Rc::new(RefCell::new(Z {
                    i_: self.i_.clone(),
                    j_: self.j_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Z {
                    i_: other.i_.clone(),
                    j_: other.j_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Z {}
pub fn test_2() -> () {
    {
        let opt: Value<()> = Rc::new(RefCell::new({
            let __tmp_0: Value<()> = Rc::new(RefCell::new(2_()));
            std_optional_X_::new_1({ unimplemented!() }, { __tmp_0.as_pointer() })
        }));
        let y: Value<Y> = Rc::new(RefCell::new(Y::new({ 3_() })));
        unimplemented!();
        unimplemented!();
    }
    {
        let opt: Value<()> = Rc::new(RefCell::new({
            let __tmp_1: Value<()> = Rc::new(RefCell::new(2_()));
            std_optional_X_::new_1({ unimplemented!() }, { __tmp_1.as_pointer() })
        }));
        unimplemented!();
        unimplemented!();
    }
    {
        let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let y: Value<Y> = Rc::new(RefCell::new(Y::new({ 3_() })));
        unimplemented!();
        unimplemented!();
    }
    {
        let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    return 0_();
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
    unimplemented!();
    const _: () = assert!(true, "test() == 0");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
