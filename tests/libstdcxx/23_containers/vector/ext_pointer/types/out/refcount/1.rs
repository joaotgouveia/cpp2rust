extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N_X {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<N_X>> = Rc::new(RefCell::new(
        (0..(5_usize) as usize)
            .map(|_| <N_X>::default())
            .collect::<Vec<_>>(),
    ));
    let w: Value<Vec<N_X>> = Rc::new(RefCell::new(
        (0..(1_usize) as usize)
            .map(|_| <N_X>::default())
            .collect::<Vec<_>>(),
    ));
    &(v.decay().offset(0_usize as isize));
    &({
        let __idx = 0_usize;
        if __idx as usize >= (*w.as_pointer().upgrade().deref()).len() {
            panic!("out of bounds access")
        } else {
            w.as_pointer().decay().offset(__idx as isize)
        }
    });
    &((*v.borrow()).len());
    &((*v.borrow()).capacity());
    {
        let __a0 = 1_usize as usize;
        (*v.borrow_mut()).resize_with(__a0, || <N_X>::default())
    };
    {
        let __it = <()>::from(v.decay().begin());
        let __off = __it.index();
        (*v.borrow_mut()).insert(__off, <N_X>::default());
        __it
    };
    {
        let __it = <()>::from(v.decay().begin());
        let __off = __it.index();
        (*v.borrow_mut()).splice(__off..__off, std::iter::repeat_n(<N_X>::default(), 1_usize));
        __it
    };
    {
        let __pos = <()>::from(v.decay().begin());
        let __start = __pos.index();
        let mut __tmp: Vec<N_X> = Vec::new();
        let mut __it = w.decay().begin();
        while __it != w.decay().end() {
            __tmp.push(<N_X as CppFrom<N_X>>::cpp_from(__it.iter_deref()));
            __it.iter_inc();
        }
        v.as_pointer().with_mut(|__v: &mut Vec<N_X>| {
            __v.splice(__start..__start, __tmp);
        });
        __pos
    };
    v.as_pointer().write((*w.borrow()).clone());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
