extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};

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
    let s: Value<std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int__> =
        Rc::new(RefCell::new(
            <std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int__>::default(
            ),
        ));
    let p1: Value<(
        Value<std___detail__Node_iterator_int__true__false_>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        ({
            let ___x: Value<i32> = Rc::new(RefCell::new(1));
            (*s.borrow()).insert_36(___x.as_pointer())
        }),
    ));
    let p2: Value<(
        Value<std___detail__Node_iterator_int__true__false_>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        ({
            let ___x: Value<i32> = Rc::new(RefCell::new(2));
            (*s.borrow()).insert_36(___x.as_pointer())
        }),
    ));
    let cs: Ptr<std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int__> =
        s.as_pointer();
    assert!(
        ({
            let ___x: Ptr<i32> = (*(*p1.borrow()).0.borrow()).iter_deref();
            let ___y: Ptr<i32> = (*(*p1.borrow()).0.borrow()).iter_deref();
            ({ (*cs.upgrade().deref()).key_eq() })(___x, ___y)
        })
    );
    assert!(
        ({
            let ___x: Ptr<i32> = (*(*p2.borrow()).0.borrow()).iter_deref();
            let ___y: Ptr<i32> = (*(*p2.borrow()).0.borrow()).iter_deref();
            ({ (*cs.upgrade().deref()).key_eq() })(___x, ___y)
        })
    );
    assert!(
        !({
            let ___x: Ptr<i32> = (*(*p1.borrow()).0.borrow()).iter_deref();
            let ___y: Ptr<i32> = (*(*p2.borrow()).0.borrow()).iter_deref();
            ({ (*cs.upgrade().deref()).key_eq() })(___x, ___y)
        })
    );
    assert!(
        !({
            let ___x: Ptr<i32> = (*(*p2.borrow()).0.borrow()).iter_deref();
            let ___y: Ptr<i32> = (*(*p1.borrow()).0.borrow()).iter_deref();
            ({ (*cs.upgrade().deref()).key_eq() })(___x, ___y)
        })
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
