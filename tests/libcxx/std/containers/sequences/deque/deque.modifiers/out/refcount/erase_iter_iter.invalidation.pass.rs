extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn del_at_start_0(c: Vec<i32>, mut num: usize) {
    let c: Value<Vec<i32>> = Rc::new(RefCell::new(c));
    let first: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let last: Value<()> = Rc::new(RefCell::new((*first.borrow()).iter_add((num as isize))));
    let it1: Value<()> = Rc::new(RefCell::new((*last.borrow()).clone()));
    let it2: Value<()> = Rc::new(RefCell::new(unimplemented!().iter_add(-1_isize)));
    unimplemented!();
    let it3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let it4: Value<()> = Rc::new(RefCell::new(unimplemented!().iter_add(-1_isize)));
    assert!((*it1.borrow()) == (*it3.borrow()));
    assert!((((*it1.borrow()).iter_deref().read()) == ((*it3.borrow()).iter_deref().read())));
    assert!((((*it1.borrow()).iter_deref()) == ((*it3.borrow()).iter_deref())));
    assert!((*it2.borrow()) == (*it4.borrow()));
    assert!((((*it2.borrow()).iter_deref().read()) == ((*it4.borrow()).iter_deref().read())));
    assert!((((*it2.borrow()).iter_deref()) == ((*it4.borrow()).iter_deref())));
    const _: () = assert!(true, "");;
}
pub fn del_at_end_1(c: Vec<i32>, mut num: usize) {
    let c: Value<Vec<i32>> = Rc::new(RefCell::new(c));
    let last: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let first: Value<()> = Rc::new(RefCell::new((*last.borrow()).iter_add(-(num as isize))));
    let it1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let it2: Value<()> = Rc::new(RefCell::new((*first.borrow()).iter_add(-1_isize)));
    unimplemented!();
    let it3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let it4: Value<()> = Rc::new(RefCell::new(unimplemented!().iter_add(-1_isize)));
    assert!((*it1.borrow()) == (*it3.borrow()));
    assert!((((*it1.borrow()).iter_deref().read()) == ((*it3.borrow()).iter_deref().read())));
    assert!((((*it1.borrow()).iter_deref()) == ((*it3.borrow()).iter_deref())));
    assert!((*it2.borrow()) == (*it4.borrow()));
    assert!((((*it2.borrow()).iter_deref().read()) == ((*it4.borrow()).iter_deref().read())));
    assert!((((*it2.borrow()).iter_deref()) == ((*it4.borrow()).iter_deref())));
    const _: () = assert!(true, "");;
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
    let queue: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    let mut i: i32 = 0;
    'loop_: while (i < 20) {
        (*queue.borrow_mut()).push(i);
        i.prefix_inc();
    }
    'loop_: while (unimplemented!() > 1_usize) {
        let mut i: usize = 1_usize;
        'loop_: while (i < unimplemented!()) {
            ({ del_at_start_0((*queue.borrow()).clone(), i) });
            ({ del_at_end_1((*queue.borrow()).clone(), i) });
            i.prefix_inc();
        }
        unimplemented!();
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
