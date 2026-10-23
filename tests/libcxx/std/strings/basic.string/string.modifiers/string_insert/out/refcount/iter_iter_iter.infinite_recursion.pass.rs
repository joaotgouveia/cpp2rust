extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn is_string_asan_correct_0(_a0: Ptr<()>) -> () {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct char_ascii {
    #[offset(0)]
    pub char_: (),
}
impl char_ascii {
    pub fn new_1(mut ch: ()) -> Self {
        let __this: char_ascii = Self { char_: ch };
        unimplemented!();
        __this
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
    let str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ch: Value<char_ascii> = Rc::new(RefCell::new(char_ascii::new_1({ ('A' as ()) })));
    ({
        let ___pos: () = {
            let __tmp_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ascii_ptr_::new_1({ __tmp_0.as_pointer() })
        };
        let ___first: Ptr<char_ascii> = (ch.as_pointer());
        let ___last: Ptr<char_ascii> = (ch.as_pointer()).offset((1_()) as isize);
        (*str.borrow()).insert_109_struct_char_ascii_ptr_0(___pos, ___first, ___last)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
