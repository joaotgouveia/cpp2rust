extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> bool {
    {
        const _: () = assert!(
            true,
            "std::three_way_comparable<std::ranges::iterator_t<R>>"
        );;
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let iter1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let iter2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "std::same_as<decltype(iter1 == iter2), bool>");;
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!((*iter1.borrow()) < (*iter1.borrow())));
        assert!((*iter1.borrow()) < (*iter2.borrow()));
        assert!(!((*iter2.borrow()) < (*iter1.borrow())));
        assert!((*iter1.borrow()) <= (*iter1.borrow()));
        assert!((*iter1.borrow()) <= (*iter2.borrow()));
        assert!(!((*iter2.borrow()) <= (*iter1.borrow())));
        assert!(!((*iter1.borrow()) > (*iter1.borrow())));
        assert!(!((*iter1.borrow()) > (*iter2.borrow())));
        assert!((*iter2.borrow()) > (*iter1.borrow()));
        assert!((*iter1.borrow()) >= (*iter1.borrow()));
        assert!(!((*iter1.borrow()) >= (*iter2.borrow())));
        assert!((*iter2.borrow()) >= (*iter1.borrow()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!((*iter1.borrow()) != (*iter1.borrow())));
        assert!((*iter1.borrow()) != (*iter2.borrow()));
        assert!(!((*iter2.borrow()) != (*iter2.borrow())));
        assert!((unimplemented!()) == std::cmp::Ordering::Less);
        assert!((unimplemented!()) == std::cmp::Ordering::Equal);
        assert!((unimplemented!()) == std::cmp::Ordering::Greater);
        const _: () = assert!(
            true,
            "std::same_as<decltype(iter1 <=> iter2), std::strong_ordering>"
        );;
    }
    {
        const _: () = assert!(
            true,
            "std::three_way_comparable<std::ranges::iterator_t<R>>"
        );;
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let iter1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let iter2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "std::same_as<decltype(iter1 == iter2), bool>");;
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!((*iter1.borrow()) < (*iter1.borrow())));
        assert!((*iter1.borrow()) < (*iter2.borrow()));
        assert!(!((*iter2.borrow()) < (*iter1.borrow())));
        assert!((*iter1.borrow()) <= (*iter1.borrow()));
        assert!((*iter1.borrow()) <= (*iter2.borrow()));
        assert!(!((*iter2.borrow()) <= (*iter1.borrow())));
        assert!(!((*iter1.borrow()) > (*iter1.borrow())));
        assert!(!((*iter1.borrow()) > (*iter2.borrow())));
        assert!((*iter2.borrow()) > (*iter1.borrow()));
        assert!((*iter1.borrow()) >= (*iter1.borrow()));
        assert!(!((*iter1.borrow()) >= (*iter2.borrow())));
        assert!((*iter2.borrow()) >= (*iter1.borrow()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!((*iter1.borrow()) != (*iter1.borrow())));
        assert!((*iter1.borrow()) != (*iter2.borrow()));
        assert!(!((*iter2.borrow()) != (*iter2.borrow())));
        assert!((unimplemented!()) == std::cmp::Ordering::Less);
        assert!((unimplemented!()) == std::cmp::Ordering::Equal);
        assert!((unimplemented!()) == std::cmp::Ordering::Greater);
    }
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
pub fn __cpp2rust_init_globals() {}
