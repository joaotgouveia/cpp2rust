extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(_a0: Ptr<i32>) {}
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
    const _: () = assert!(true, "(std::is_same<std::locale::category, int>::value)");;
    assert!((none_1.with(|rc| *rc.borrow()) == 0));
    assert!((collate_2.with(|rc| *rc.borrow()) != 0));
    assert!((ctype_3.with(|rc| *rc.borrow()) != 0));
    assert!((monetary_4.with(|rc| *rc.borrow()) != 0));
    assert!((numeric_5.with(|rc| *rc.borrow()) != 0));
    assert!((time_6.with(|rc| *rc.borrow()) != 0));
    assert!((messages_7.with(|rc| *rc.borrow()) != 0));
    assert!(
        ((((((collate_2.with(|rc| *rc.borrow()) & ctype_3.with(|rc| *rc.borrow()))
            & monetary_4.with(|rc| *rc.borrow()))
            & numeric_5.with(|rc| *rc.borrow()))
            & time_6.with(|rc| *rc.borrow()))
            & messages_7.with(|rc| *rc.borrow()))
            == 0)
    );
    assert!(
        ((((((collate_2.with(|rc| *rc.borrow()) | ctype_3.with(|rc| *rc.borrow()))
            | monetary_4.with(|rc| *rc.borrow()))
            | numeric_5.with(|rc| *rc.borrow()))
            | time_6.with(|rc| *rc.borrow()))
            | messages_7.with(|rc| *rc.borrow()))
            == all_8.with(|rc| *rc.borrow()))
    );
    ({ test_0(none_1.with(|v| v.as_pointer())) });
    ({ test_0(collate_2.with(|v| v.as_pointer())) });
    ({ test_0(ctype_3.with(|v| v.as_pointer())) });
    ({ test_0(monetary_4.with(|v| v.as_pointer())) });
    ({ test_0(numeric_5.with(|v| v.as_pointer())) });
    ({ test_0(time_6.with(|v| v.as_pointer())) });
    ({ test_0(messages_7.with(|v| v.as_pointer())) });
    ({ test_0(all_8.with(|v| v.as_pointer())) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
