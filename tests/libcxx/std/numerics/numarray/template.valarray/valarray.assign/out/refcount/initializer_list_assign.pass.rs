extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub x_: (),
}
impl S {
    pub fn new_1() -> Self {
        let __this: S = Self { x_: 0_() };
        default_ctor_called_0.with(|rc| *rc.borrow_mut() = true);
        __this
    }
    pub fn new_2(mut x: ()) -> Self {
        Self { x_: x }
    }
}
impl CppEq for S {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_1(__lhs.clone(), __rhs.clone())
    }
}
impl Default for S {
    fn default() -> Self {
        { S::new_1() }
    }
}
thread_local!(
    pub static default_ctor_called_0: Value<()> = Rc::new(RefCell::new(false));
);
pub fn operator_eq_1(lhs: Ptr<S>, rhs: Ptr<S>) -> () {
    return ({ lhs.with(|__s| __s.x_) } == { rhs.with(|__s| __s.x_) });
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
        let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
        let mut N: () = (::std::mem::size_of::<[(); 5]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 2.5_f64, 3_(), 4.25_f64, 5_()];
        let mut N: () = (::std::mem::size_of::<[(); 5]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ])));
        let mut N: () = (80usize as ()).wrapping_div((16usize as ()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < N) {
            unimplemented!();
            let mut j: () = 0_();
            'loop_: while (j < unimplemented!()) {
                unimplemented!();
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let a: Value<Box<[S]>> = Rc::new(RefCell::new(Box::new([
            S::new_2({ 1_() }),
            S::new_2({ 2_() }),
            S::new_2({ 3_() }),
            S::new_2({ 4_() }),
            S::new_2({ 5_() }),
        ])));
        let mut N: () = (20usize as ()).wrapping_div((4usize as ()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = default_ctor_called_0.with(|_| ());
}
