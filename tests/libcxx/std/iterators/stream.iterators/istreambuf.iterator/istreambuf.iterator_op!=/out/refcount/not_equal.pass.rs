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
    {
        let inf1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let inf2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let i1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let i2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let i3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let i4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let i5: Value<()> = Rc::new(RefCell::new(Ptr::<()>::null()));
        assert!(!((*i1.borrow()) != (*i1.borrow())));
        assert!(!((*i1.borrow()) != (*i2.borrow())));
        assert!(((*i1.borrow()) != (*i3.borrow())));
        assert!(((*i1.borrow()) != (*i4.borrow())));
        assert!(((*i1.borrow()) != (*i5.borrow())));
        assert!(!((*i2.borrow()) != (*i1.borrow())));
        assert!(!((*i2.borrow()) != (*i2.borrow())));
        assert!(((*i2.borrow()) != (*i3.borrow())));
        assert!(((*i2.borrow()) != (*i4.borrow())));
        assert!(((*i2.borrow()) != (*i5.borrow())));
        assert!(((*i3.borrow()) != (*i1.borrow())));
        assert!(((*i3.borrow()) != (*i2.borrow())));
        assert!(!((*i3.borrow()) != (*i3.borrow())));
        assert!(!((*i3.borrow()) != (*i4.borrow())));
        assert!(!((*i3.borrow()) != (*i5.borrow())));
        assert!(((*i4.borrow()) != (*i1.borrow())));
        assert!(((*i4.borrow()) != (*i2.borrow())));
        assert!(!((*i4.borrow()) != (*i3.borrow())));
        assert!(!((*i4.borrow()) != (*i4.borrow())));
        assert!(!((*i4.borrow()) != (*i5.borrow())));
        assert!(((*i5.borrow()) != (*i1.borrow())));
        assert!(((*i5.borrow()) != (*i2.borrow())));
        assert!(!((*i5.borrow()) != (*i3.borrow())));
        assert!(!((*i5.borrow()) != (*i4.borrow())));
        assert!(!((*i5.borrow()) != (*i5.borrow())));
    }
    {
        let i1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let i2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
        assert!(!((*i1.borrow()) != unimplemented!()));
        assert!(!((*i2.borrow()) != unimplemented!()));
        assert!(!(unimplemented!() != (*i1.borrow())));
        assert!(!(unimplemented!() != (*i2.borrow())));
        let stream: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let i3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((*i3.borrow()) != unimplemented!());
        assert!(unimplemented!() != (*i3.borrow()));
    }
    {
        let inf1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let inf2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let i1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let i2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let i3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let i4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let i5: Value<()> = Rc::new(RefCell::new(Ptr::<()>::null()));
        assert!(!((*i1.borrow()) != (*i1.borrow())));
        assert!(!((*i1.borrow()) != (*i2.borrow())));
        assert!(((*i1.borrow()) != (*i3.borrow())));
        assert!(((*i1.borrow()) != (*i4.borrow())));
        assert!(((*i1.borrow()) != (*i5.borrow())));
        assert!(!((*i2.borrow()) != (*i1.borrow())));
        assert!(!((*i2.borrow()) != (*i2.borrow())));
        assert!(((*i2.borrow()) != (*i3.borrow())));
        assert!(((*i2.borrow()) != (*i4.borrow())));
        assert!(((*i2.borrow()) != (*i5.borrow())));
        assert!(((*i3.borrow()) != (*i1.borrow())));
        assert!(((*i3.borrow()) != (*i2.borrow())));
        assert!(!((*i3.borrow()) != (*i3.borrow())));
        assert!(!((*i3.borrow()) != (*i4.borrow())));
        assert!(!((*i3.borrow()) != (*i5.borrow())));
        assert!(((*i4.borrow()) != (*i1.borrow())));
        assert!(((*i4.borrow()) != (*i2.borrow())));
        assert!(!((*i4.borrow()) != (*i3.borrow())));
        assert!(!((*i4.borrow()) != (*i4.borrow())));
        assert!(!((*i4.borrow()) != (*i5.borrow())));
        assert!(((*i5.borrow()) != (*i1.borrow())));
        assert!(((*i5.borrow()) != (*i2.borrow())));
        assert!(!((*i5.borrow()) != (*i3.borrow())));
        assert!(!((*i5.borrow()) != (*i4.borrow())));
        assert!(!((*i5.borrow()) != (*i5.borrow())));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
