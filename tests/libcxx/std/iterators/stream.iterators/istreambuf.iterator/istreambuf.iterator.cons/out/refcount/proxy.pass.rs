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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    {
        let inf: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let j: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let i: Value<()> = Rc::new(RefCell::new({
            let __tmp_0: Value<std_istreambuf_iterator_char____proxy> =
                Rc::new(RefCell::new(unimplemented!()));
            std_istreambuf_iterator_char__std_char_traits_char__::new_1({ __tmp_0.as_pointer() })
        }));
        unimplemented!();
        unimplemented!();
    }
    {
        let inf: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let j: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let i: Value<()> = Rc::new(RefCell::new({
            let __tmp_1: Value<std_istreambuf_iterator_wchar_t____proxy> =
                Rc::new(RefCell::new(unimplemented!()));
            std_istreambuf_iterator_wchar_t__std_char_traits_wchar_t__::new_1({
                __tmp_1.as_pointer()
            })
        }));
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
