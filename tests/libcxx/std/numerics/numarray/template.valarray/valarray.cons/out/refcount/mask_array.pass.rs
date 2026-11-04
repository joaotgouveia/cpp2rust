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
    let a1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    ])));
    let N1: Value<usize> = Rc::new(RefCell::new(
        (::std::mem::size_of::<[i32; 16]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)),
    ));
    let b: Value<Box<[bool]>> = Rc::new(RefCell::new(Box::new([
        true, false, false, true, true, false, false, true, false, false, false, true, false,
        false, false, false,
    ])));
    let v1: Value<std_valarray_int_> = Rc::new(RefCell::new(std_valarray_int_::new_1(
        { (a1.as_pointer() as Ptr<i32>) },
        { (*N1.borrow()) },
    )));
    let vb: Value<std_valarray_bool_> = Rc::new(RefCell::new(std_valarray_bool_::new_1(
        { (b.as_pointer() as Ptr<bool>) },
        { (*N1.borrow()) },
    )));
    let v2: Value<std_valarray_int_> = Rc::new(RefCell::new({
        let __tmp_0: Value<std_mask_array_int_> =
            Rc::new(RefCell::new(((*v1.borrow())[(*vb.borrow())]).clone()));
        std_valarray_int_::new_2({ __tmp_0.as_pointer() })
    }));
    assert!(({ (({ (*v2.borrow()).size() }) == 5_usize) }));
    assert!(({ ({ (*v2.borrow())[0_usize] } == 0) }));
    assert!(({ ({ (*v2.borrow())[1_usize] } == 3) }));
    assert!(({ ({ (*v2.borrow())[2_usize] } == 4) }));
    assert!(({ ({ (*v2.borrow())[3_usize] } == 7) }));
    assert!(({ ({ (*v2.borrow())[4_usize] } == 11) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
