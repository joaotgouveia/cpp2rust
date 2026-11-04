extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(_a0: Ptr<u16>) {}
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
    assert!((space_1.with(|rc| *rc.borrow()) != 0));
    assert!((print_2.with(|rc| *rc.borrow()) != 0));
    assert!((cntrl_3.with(|rc| *rc.borrow()) != 0));
    assert!((upper_4.with(|rc| *rc.borrow()) != 0));
    assert!((lower_5.with(|rc| *rc.borrow()) != 0));
    assert!((alpha_6.with(|rc| *rc.borrow()) != 0));
    assert!((digit_7.with(|rc| *rc.borrow()) != 0));
    assert!((punct_8.with(|rc| *rc.borrow()) != 0));
    assert!((xdigit_9.with(|rc| *rc.borrow()) != 0));
    assert!(
        ((((((((((space_1.with(|rc| *rc.borrow()) as i32)
            & (print_2.with(|rc| *rc.borrow()) as i32))
            & (cntrl_3.with(|rc| *rc.borrow()) as i32))
            & (upper_4.with(|rc| *rc.borrow()) as i32))
            & (lower_5.with(|rc| *rc.borrow()) as i32))
            & (alpha_6.with(|rc| *rc.borrow()) as i32))
            & (digit_7.with(|rc| *rc.borrow()) as i32))
            & (punct_8.with(|rc| *rc.borrow()) as i32))
            & (xdigit_9.with(|rc| *rc.borrow()) as i32))
            == 0)
    );
    assert!(
        ((alnum_10.with(|rc| *rc.borrow()) as i32)
            == ((alpha_6.with(|rc| *rc.borrow()) as i32)
                | (digit_7.with(|rc| *rc.borrow()) as i32)))
    );
    assert!(
        ((graph_11.with(|rc| *rc.borrow()) as i32)
            == ((alnum_10.with(|rc| *rc.borrow()) as i32)
                | (punct_8.with(|rc| *rc.borrow()) as i32)))
    );
    ({ test_0(space_1.with(|v| v.as_pointer())) });
    ({ test_0(print_2.with(|v| v.as_pointer())) });
    ({ test_0(cntrl_3.with(|v| v.as_pointer())) });
    ({ test_0(upper_4.with(|v| v.as_pointer())) });
    ({ test_0(lower_5.with(|v| v.as_pointer())) });
    ({ test_0(alpha_6.with(|v| v.as_pointer())) });
    ({ test_0(digit_7.with(|v| v.as_pointer())) });
    ({ test_0(punct_8.with(|v| v.as_pointer())) });
    ({ test_0(xdigit_9.with(|v| v.as_pointer())) });
    ({ test_0(blank_12.with(|v| v.as_pointer())) });
    ({ test_0(alnum_10.with(|v| v.as_pointer())) });
    ({ test_0(graph_11.with(|v| v.as_pointer())) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
