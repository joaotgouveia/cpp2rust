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
    let N: Value<u32> = Rc::new(RefCell::new(20_u32));
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new((0..20).map(|_| 0_i8).collect::<Box<[i8]>>()));
    let mut r: AnyPtr = AnyPtr::default();
    let mut p: AnyPtr = (((buf.as_pointer() as Ptr<i8>).offset(0)) as Ptr<i8>).to_any();
    let mut s: usize = ((*N.borrow()) as usize);
    r = unimplemented!();
    assert!(
        ({ (p).clone() } == { (((buf.as_pointer() as Ptr<i8>).offset(0)) as Ptr<i8>).to_any() })
    );
    assert!(({ (r).clone() } == { (p).clone() }));
    assert!((s == ((*N.borrow()) as usize)));
    p = (((buf.as_pointer() as Ptr<i8>).offset(1)) as Ptr<i8>).to_any();
    s = ((*N.borrow()) as usize);
    r = unimplemented!();
    assert!(
        ({ (p).clone() } == { (((buf.as_pointer() as Ptr<i8>).offset(4)) as Ptr<i8>).to_any() })
    );
    assert!(({ (r).clone() } == { (p).clone() }));
    assert!((s == (((*N.borrow()).wrapping_sub(3_u32)) as usize)));
    p = (((buf.as_pointer() as Ptr<i8>).offset(2)) as Ptr<i8>).to_any();
    s = ((*N.borrow()) as usize);
    r = unimplemented!();
    assert!(
        ({ (p).clone() } == { (((buf.as_pointer() as Ptr<i8>).offset(4)) as Ptr<i8>).to_any() })
    );
    assert!(({ (r).clone() } == { (p).clone() }));
    assert!((s == (((*N.borrow()).wrapping_sub(2_u32)) as usize)));
    p = (((buf.as_pointer() as Ptr<i8>).offset(3)) as Ptr<i8>).to_any();
    s = ((*N.borrow()) as usize);
    r = unimplemented!();
    assert!(
        ({ (p).clone() } == { (((buf.as_pointer() as Ptr<i8>).offset(4)) as Ptr<i8>).to_any() })
    );
    assert!(({ (r).clone() } == { (p).clone() }));
    assert!((s == (((*N.borrow()).wrapping_sub(1_u32)) as usize)));
    p = (((buf.as_pointer() as Ptr<i8>).offset(4)) as Ptr<i8>).to_any();
    s = ((*N.borrow()) as usize);
    r = unimplemented!();
    assert!(
        ({ (p).clone() } == { (((buf.as_pointer() as Ptr<i8>).offset(4)) as Ptr<i8>).to_any() })
    );
    assert!(({ (r).clone() } == { (p).clone() }));
    assert!((s == ((*N.borrow()) as usize)));
    p = (((buf.as_pointer() as Ptr<i8>).offset(0)) as Ptr<i8>).to_any();
    s = ((*N.borrow()) as usize);
    r = unimplemented!();
    assert!(
        ({ (p).clone() } == { (((buf.as_pointer() as Ptr<i8>).offset(0)) as Ptr<i8>).to_any() })
    );
    assert!(({ (r).clone() } == { (p).clone() }));
    assert!((s == ((*N.borrow()) as usize)));
    p = (((buf.as_pointer() as Ptr<i8>).offset(1)) as Ptr<i8>).to_any();
    s = (((*N.borrow()).wrapping_sub(1_u32)) as usize);
    r = unimplemented!();
    assert!(
        ({ (p).clone() } == { (((buf.as_pointer() as Ptr<i8>).offset(4)) as Ptr<i8>).to_any() })
    );
    assert!(({ (r).clone() } == { (p).clone() }));
    assert!((s == (((*N.borrow()).wrapping_sub(4_u32)) as usize)));
    p = (((buf.as_pointer() as Ptr<i8>).offset(1)) as Ptr<i8>).to_any();
    s = (((*N.borrow()).wrapping_sub(1_u32)) as usize);
    r = unimplemented!();
    assert!(
        ({ (p).clone() } == { (((buf.as_pointer() as Ptr<i8>).offset(1)) as Ptr<i8>).to_any() })
    );
    assert!((r).is_null());
    assert!((s == (((*N.borrow()).wrapping_sub(1_u32)) as usize)));
    p = (((buf.as_pointer() as Ptr<i8>).offset(0)) as Ptr<i8>).to_any();
    s = ((*N.borrow()) as usize);
    r = unimplemented!();
    assert!(
        ({ (p).clone() } == { (((buf.as_pointer() as Ptr<i8>).offset(0)) as Ptr<i8>).to_any() })
    );
    assert!((r).is_null());
    assert!((s == ((*N.borrow()) as usize)));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
