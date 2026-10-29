extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn del_at_start_0(c: Vec<i32>) {
    let c: Value<Vec<i32>> = Rc::new(RefCell::new(c));
    let first: Value<RandomAccessIterator<i32>> =
        Rc::new(RefCell::new(({ (*c.borrow_mut()).begin_24() })));
    let it1: Value<RandomAccessIterator<i32>> =
        Rc::new(RefCell::new((*first.borrow()).iter_add(1_isize)));
    let it2: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        ({ (*c.borrow_mut()).end_26() }).iter_add(-1_isize),
    ));
    ({ (*c.borrow_mut()).erase_68(<RandomAccessIterator<i32>>::from((*first.borrow()).clone())) });
    let it3: Value<RandomAccessIterator<i32>> =
        Rc::new(RefCell::new(({ (*c.borrow_mut()).begin_24() })));
    let it4: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        ({ (*c.borrow_mut()).end_26() }).iter_add(-1_isize),
    ));
    assert!((*it1.borrow()) == (*it3.borrow()));
    assert!((((*it1.borrow()).iter_deref().read()) == ((*it3.borrow()).iter_deref().read())));
    assert!((((*it1.borrow()).iter_deref()) == ((*it3.borrow()).iter_deref())));
    assert!((*it2.borrow()) == (*it4.borrow()));
    assert!((((*it2.borrow()).iter_deref().read()) == ((*it4.borrow()).iter_deref().read())));
    assert!((((*it2.borrow()).iter_deref()) == ((*it4.borrow()).iter_deref())));
}
pub fn del_at_end_1(c: Vec<i32>) {
    let c: Value<Vec<i32>> = Rc::new(RefCell::new(c));
    let first: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        ({ (*c.borrow_mut()).end_26() }).iter_add(-1_isize),
    ));
    let it1: Value<RandomAccessIterator<i32>> =
        Rc::new(RefCell::new(({ (*c.borrow_mut()).begin_24() })));
    let it2: Value<RandomAccessIterator<i32>> =
        Rc::new(RefCell::new((*first.borrow()).iter_add(-1_isize)));
    ({ (*c.borrow_mut()).erase_68(<RandomAccessIterator<i32>>::from((*first.borrow()).clone())) });
    let it3: Value<RandomAccessIterator<i32>> =
        Rc::new(RefCell::new(({ (*c.borrow_mut()).begin_24() })));
    let it4: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        ({ (*c.borrow_mut()).end_26() }).iter_add(-1_isize),
    ));
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
    'loop_: while (({ (*queue.borrow()).size() }) > 1_u64) {
        ({ del_at_start_0((*queue.borrow()).clone()) });
        ({ del_at_end_1((*queue.borrow()).clone()) });
        ({ (*queue.borrow_mut()).pop_back() });
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
