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
    let mut expected: Ptr<std_pmr_memory_resource> = ({ null_memory_resource_0() });
    ({ set_default_resource_1((expected).clone()) });
    {
        let buffer: Value<Box<[i8]>> =
            Rc::new(RefCell::new((0..16).map(|_| 0_i8).collect::<Box<[i8]>>()));
        let r1: Value<std_pmr_monotonic_buffer_resource> =
            Rc::new(RefCell::new(<std_pmr_monotonic_buffer_resource>::default()));
        let r2: Value<std_pmr_monotonic_buffer_resource> =
            Rc::new(RefCell::new(std_pmr_monotonic_buffer_resource::new_1({
                16_usize
            })));
        let r3: Value<std_pmr_monotonic_buffer_resource> =
            Rc::new(RefCell::new(std_pmr_monotonic_buffer_resource::new_2(
                { ((buffer.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any() },
                { ::std::mem::size_of::<[i8; 16]>() },
            )));
        assert!(({ ({ ({ (*r1.borrow()).upstream_resource() }) } == { (expected).clone() }) }));
        assert!(({ ({ ({ (*r2.borrow()).upstream_resource() }) } == { (expected).clone() }) }));
        assert!(({ ({ ({ (*r3.borrow()).upstream_resource() }) } == { (expected).clone() }) }));
    }
    expected = ({ new_delete_resource_2() });
    ({ set_default_resource_1((expected).clone()) });
    {
        let buffer: Value<Box<[i8]>> =
            Rc::new(RefCell::new((0..16).map(|_| 0_i8).collect::<Box<[i8]>>()));
        let r1: Value<std_pmr_monotonic_buffer_resource> =
            Rc::new(RefCell::new(<std_pmr_monotonic_buffer_resource>::default()));
        let r2: Value<std_pmr_monotonic_buffer_resource> =
            Rc::new(RefCell::new(std_pmr_monotonic_buffer_resource::new_1({
                16_usize
            })));
        let r3: Value<std_pmr_monotonic_buffer_resource> =
            Rc::new(RefCell::new(std_pmr_monotonic_buffer_resource::new_2(
                { ((buffer.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any() },
                { ::std::mem::size_of::<[i8; 16]>() },
            )));
        assert!(({ ({ ({ (*r1.borrow()).upstream_resource() }) } == { (expected).clone() }) }));
        assert!(({ ({ ({ (*r2.borrow()).upstream_resource() }) } == { (expected).clone() }) }));
        assert!(({ ({ ({ (*r3.borrow()).upstream_resource() }) } == { (expected).clone() }) }));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
