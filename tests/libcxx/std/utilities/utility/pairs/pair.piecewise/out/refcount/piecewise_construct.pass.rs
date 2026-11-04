extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct A {
    #[offset(0)]
    i_: i32,
    #[offset(4)]
    c_: i8,
}
impl A {
    pub fn new(mut i: i32, mut c: i8) -> Self {
        Self { i_: i, c_: c }
    }
}
impl CppMove for A {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct B {
    #[offset(0)]
    d_: f64,
    #[offset(8)]
    u1_: u32,
    #[offset(12)]
    u2_: u32,
}
impl B {
    pub fn new(mut d: f64, mut u1: u32, mut u2: u32) -> Self {
        Self {
            d_: d,
            u1_: u1,
            u2_: u2,
        }
    }
}
impl CppMove for B {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_0() -> bool {
    let p: Value<(Value<A>, Value<B>)> = Rc::new(RefCell::new(std_pair_A__B_::new_1(
        { (*piecewise_construct_1.with(Value::clone).borrow()).clone() },
        {
            ({
                let ___args_0: Value<i32> = Rc::new(RefCell::new(4));
                let ___args_1: Value<i8> = Rc::new(RefCell::new(('a' as i8)));
                make_tuple_2(___args_0.as_pointer(), ___args_1.as_pointer())
            })
        },
        {
            ({
                let ___args_0: Value<f64> = Rc::new(RefCell::new(3.5_f64));
                let ___args_1: Value<u32> = Rc::new(RefCell::new(6_u32));
                let ___args_2: Value<u32> = Rc::new(RefCell::new(2_u32));
                make_tuple_3(
                    ___args_0.as_pointer(),
                    ___args_1.as_pointer(),
                    ___args_2.as_pointer(),
                )
            })
        },
    )));
    assert!(({ (({ AImpl::get_i(&(*p.borrow()).0.as_pointer(),) }) == 4) }));
    assert!(
        ({ ((({ AImpl::get_c(&(*p.borrow()).0.as_pointer(),) }) as i32) == (('a' as i8) as i32)) })
    );
    assert!(({ (({ BImpl::get_d(&(*p.borrow()).1.as_pointer(),) }) == 3.5_f64) }));
    assert!(({ (({ BImpl::get_u1(&(*p.borrow()).1.as_pointer(),) }) == 6_u32) }));
    assert!(({ (({ BImpl::get_u2(&(*p.borrow()).1.as_pointer(),) }) == 2_u32) }));
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
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub trait AImpl {
    fn get_i(&self) -> i32;
    fn get_c(&self) -> i8;
}
impl AImpl for Ptr<A> {
    fn get_i(&self) -> i32 {
        return (*self).with(|__s| __s.i_);
    }
    fn get_c(&self) -> i8 {
        return (*self).with(|__s| __s.c_);
    }
}
pub trait BImpl {
    fn get_d(&self) -> f64;
    fn get_u1(&self) -> u32;
    fn get_u2(&self) -> u32;
}
impl BImpl for Ptr<B> {
    fn get_d(&self) -> f64 {
        return (*self).with(|__s| __s.d_);
    }
    fn get_u1(&self) -> u32 {
        return (*self).with(|__s| __s.u1_);
    }
    fn get_u2(&self) -> u32 {
        return (*self).with(|__s| __s.u2_);
    }
}
pub fn __cpp2rust_init_globals() {}
