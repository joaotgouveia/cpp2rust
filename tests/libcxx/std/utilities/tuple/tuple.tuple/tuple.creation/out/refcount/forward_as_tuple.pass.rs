extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test0_0(_a0: Ptr<()>) {
    const _: () = assert!(true, "std::tuple_size<Tuple>::value == 0");;
}
pub fn test1a_1(t: Ptr<()>) {
    const _: () = assert!(true, "std::tuple_size<Tuple>::value == 1");;
    const _: () = assert!(
        true,
        "std::is_same<typename std::tuple_element<0, Tuple>::type, int&&>::value"
    );;
    unimplemented!();
}
pub fn test1b_2(t: Ptr<()>) {
    const _: () = assert!(true, "std::tuple_size<Tuple>::value == 1");;
    const _: () = assert!(
        true,
        "std::is_same<typename std::tuple_element<0, Tuple>::type, int&>::value"
    );;
    unimplemented!();
}
pub fn test2a_3(t: Ptr<()>) {
    const _: () = assert!(true, "std::tuple_size<Tuple>::value == 2");;
    const _: () = assert!(
        true,
        "std::is_same<typename std::tuple_element<0, Tuple>::type, double&>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_same<typename std::tuple_element<1, Tuple>::type, char&>::value"
    );;
    unimplemented!();
    unimplemented!();
}
pub fn test3_4(_a0: Ptr<()>) -> () {
    return value_5.with(|rc| *rc.borrow());
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
    {
        ({
            let _arg0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test0_0(_arg0.as_pointer())
        });
    }
    {
        ({
            let _t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test1a_1(_t.as_pointer())
        });
    }
    {
        let mut i: () = 2_();
        ({
            let _t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test1b_2(_t.as_pointer())
        });
    }
    {
        let mut i: () = 2.5_f64;
        let c: Value<()> = Rc::new(RefCell::new(('a' as ())));
        ({
            let _t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test2a_3(_t.as_pointer())
        });
        const _: () = assert!(true, "test3 (std::forward_as_tuple(i, c)) == 2");;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
