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
    let l: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let mut x1: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { Ptr::<()>::from_string_literal(b"1234") },
            None,
        );
        let mut x2: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { Ptr::<()>::from_string_literal(b"12345") },
            None,
        );
        let f: Ptr<()> = ({ use_facet_0(l.as_pointer()) });
        unimplemented!();
    }
    {
        let mut x1: () =
            std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
                {
                    Ptr::<()>::from_string_literal(&[
                        49 as (), 50 as (), 51 as (), 52 as (), 0 as (),
                    ])
                },
                None,
            );
        let mut x2: () =
            std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
                {
                    Ptr::<()>::from_string_literal(&[
                        49 as (), 50 as (), 51 as (), 52 as (), 53 as (), 0 as (),
                    ])
                },
                None,
            );
        let f: Ptr<()> = ({ use_facet_1(l.as_pointer()) });
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
