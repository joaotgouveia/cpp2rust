extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn assert_contiguous_0(c: Ptr<()>) {
    let mut i: () = 0_();
    'loop_: while ({ i } < { unimplemented!() }) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn assert_contiguous_1(c: Ptr<()>) {
    let mut i: () = 0_();
    'loop_: while ({ i } < { unimplemented!() }) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn assert_contiguous_2(c: Ptr<()>) {
    let mut i: () = 0_();
    'loop_: while ({ i } < { unimplemented!() }) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn assert_contiguous_3(c: Ptr<()>) {
    let mut i: () = 0_();
    'loop_: while ({ i } < { unimplemented!() }) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn assert_contiguous_4(c: Ptr<()>) {
    let mut i: () = 0_();
    'loop_: while ({ i } < { unimplemented!() }) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn assert_contiguous_5(c: Ptr<()>) {
    let mut i: () = 0_();
    'loop_: while ({ i } < { unimplemented!() }) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn assert_contiguous_6(c: Ptr<()>) {
    let mut i: () = 0_();
    'loop_: while ({ i } < { unimplemented!() }) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn assert_contiguous_7(c: Ptr<()>) {
    let mut i: () = 0_();
    'loop_: while ({ i } < { unimplemented!() }) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn tests_8() -> () {
    ({
        let _c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert_contiguous_0(_c.as_pointer())
    });
    ({
        let _c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert_contiguous_1(_c.as_pointer())
    });
    ({
        let _c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert_contiguous_2(_c.as_pointer())
    });
    ({
        let _c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert_contiguous_3(_c.as_pointer())
    });
    ({
        let _c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert_contiguous_4(_c.as_pointer())
    });
    ({
        let _c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert_contiguous_5(_c.as_pointer())
    });
    ({
        let _c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert_contiguous_6(_c.as_pointer())
    });
    ({
        let _c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert_contiguous_7(_c.as_pointer())
    });
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ tests_8() });
    const _: () = assert!(true, "tests()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
