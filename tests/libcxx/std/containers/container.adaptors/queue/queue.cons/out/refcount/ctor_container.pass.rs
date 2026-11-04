extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn make_0(mut n: i32) -> Vec<i32> {
    let c: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        (*c.borrow_mut()).push(i);
        i.prefix_inc();
    }
    return ({ std::mem::take(&mut (*c.borrow_mut())) });
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
    let d: Value<Vec<i32>> = Rc::new(RefCell::new(({ make_0(5) })));
    let q: Value<std_queue_int__std_deque_int__std_allocator_int___> = Rc::new(RefCell::new(
        std_queue_int__std_deque_int__std_allocator_int___::new_1({ d.as_pointer() }),
    ));
    assert!(({ (({ (*q.borrow()).size() }) == 5_u64) }));
    let mut i: usize = 0_usize;
    'loop_: while (i < ({ (*d.borrow()).size() })) {
        assert!(({ ((({ (*q.borrow()).front_15() }).read()) == { (*d.borrow())[i] }) }));
        ({ (*q.borrow()).pop() });
        i.prefix_inc();
    }
    const _: () = assert!(true, "!test_convertible<Q, const Container&>()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
