extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(f: Ptr<()>, d: Ptr<()>) {
    {
        const _: () = assert!(true, "(std::is_same<R, ToDuration>::value)");;
        unimplemented!();
    }
}
pub fn test_1(f: Ptr<()>, d: Ptr<()>) {
    {
        const _: () = assert!(true, "(std::is_same<R, ToDuration>::value)");;
        unimplemented!();
    }
}
pub fn test_2(f: Ptr<()>, d: Ptr<()>) {
    {
        const _: () = assert!(true, "(std::is_same<R, ToDuration>::value)");;
        unimplemented!();
    }
}
pub fn test_3(f: Ptr<()>, d: Ptr<()>) {
    {
        const _: () = assert!(true, "(std::is_same<R, ToDuration>::value)");;
        unimplemented!();
    }
}
pub fn test_4(f: Ptr<()>, d: Ptr<()>) {
    {
        const _: () = assert!(true, "(std::is_same<R, ToDuration>::value)");;
        unimplemented!();
    }
}
pub fn test_5(f: Ptr<()>, d: Ptr<()>) {
    {
        const _: () = assert!(true, "(std::is_same<R, ToDuration>::value)");;
        unimplemented!();
    }
}
pub fn test_6(f: Ptr<()>, d: Ptr<()>) {
    {
        const _: () = assert!(true, "(std::is_same<R, ToDuration>::value)");;
        unimplemented!();
    }
}
pub fn test_7(f: Ptr<()>, d: Ptr<()>) {
    {
        const _: () = assert!(true, "(std::is_same<R, ToDuration>::value)");;
        unimplemented!();
    }
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
    ({
        let _f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_f.as_pointer(), _d.as_pointer())
    });
    ({
        let _f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(_f.as_pointer(), _d.as_pointer())
    });
    ({
        let _f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(_f.as_pointer(), _d.as_pointer())
    });
    ({
        let _f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_3(_f.as_pointer(), _d.as_pointer())
    });
    ({
        let _f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(_f.as_pointer(), _d.as_pointer())
    });
    ({
        let _f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(_f.as_pointer(), _d.as_pointer())
    });
    ({
        let _f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_6(_f.as_pointer(), _d.as_pointer())
    });
    ({
        let _f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_7(_f.as_pointer(), _d.as_pointer())
    });
    {
        let h: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "h.count() == 2");;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
