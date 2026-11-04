extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i8, new_1, value)]
#[byte_size(1)]
pub struct char_ascii {
    #[offset(0)]
    pub char_: i8,
}
impl char_ascii {
    pub fn new_1(mut ch: i8) -> Self {
        let __this: char_ascii = Self { char_: ch };
        assert!(((ch as i32) <= 127));
        __this
    }
}
impl CppMove for char_ascii {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    let str: Value<Vec<char_ascii>> = Rc::new(RefCell::new(vec![char_ascii::default()]));
    let ch: Value<char_ascii> = Rc::new(RefCell::new(char_ascii::new_1({ ('A' as i8) })));
    ({
        let ___p: RandomAccessIterator<char_ascii> =
            <RandomAccessIterator<char_ascii>>::from(RandomAccessIterator::Vec(str.decay()));
        let ___beg: Ptr<char_ascii> = (ch.as_pointer());
        let ___end: Ptr<char_ascii> = (ch.as_pointer()).offset((1) as isize);
        (*str.borrow_mut()).insert_124_struct_char_ascii_ptr_void(___p, ___beg, ___end)
    });
    assert!(({ (((*str.borrow()).len() - 1) == 1_usize) }));
    assert!(
        ((str.decay().offset(0_usize as isize).with(|__s| __s.char_) as i32)
            == (('A' as i8) as i32))
    );
    const _: () = assert!(true, "");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
