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
    let l: Value<std_locale> = Rc::new(RefCell::new(<std_locale>::default()));
    assert!(!({ isgraph_0((' ' as i8), l.as_pointer(),) }));
    assert!(({ isgraph_0(('<' as i8), l.as_pointer(),) }));
    assert!(!({ isgraph_0(('\x08' as i8), l.as_pointer(),) }));
    assert!(({ isgraph_0(('A' as i8), l.as_pointer(),) }));
    assert!(({ isgraph_0(('a' as i8), l.as_pointer(),) }));
    assert!(({ isgraph_0(('z' as i8), l.as_pointer(),) }));
    assert!(({ isgraph_0(('3' as i8), l.as_pointer(),) }));
    assert!(({ isgraph_0(('.' as i8), l.as_pointer(),) }));
    assert!(({ isgraph_0(('f' as i8), l.as_pointer(),) }));
    assert!(({ isgraph_0(('9' as i8), l.as_pointer(),) }));
    assert!(({ isgraph_0(('+' as i8), l.as_pointer(),) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
