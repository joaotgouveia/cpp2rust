extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_constexpr_0() -> () {
    let mut p: Ptr<()> = Ptr::<()>::from_string_literal(&[49 as (), 50 as (), 51 as (), 0 as ()]);
    return ((({
        ({
            let ___s: Ptr<()> = (p).clone();
            let ___a: Value<()> = Rc::new(RefCell::new((49 as ())));
            std_char_traits_char16_t_::find(___s, 3_(), ___a.as_pointer())
        })
    } == { (p).clone() })
        && ({
            ({
                let ___s: Ptr<()> = (p).clone();
                let ___a: Value<()> = Rc::new(RefCell::new((50 as ())));
                std_char_traits_char16_t_::find(___s, 3_(), ___a.as_pointer())
            })
        } == { p.offset((1_()) as isize) }))
        && ({
            ({
                let ___s: Ptr<()> = (p).clone();
                let ___a: Value<()> = Rc::new(RefCell::new((51 as ())));
                std_char_traits_char16_t_::find(___s, 3_(), ___a.as_pointer())
            })
        } == { p.offset((2_()) as isize) }))
        && (({
            let ___s: Ptr<()> = (p).clone();
            let ___a: Value<()> = Rc::new(RefCell::new((52 as ())));
            std_char_traits_char16_t_::find(___s, 3_(), ___a.as_pointer())
        })
        .is_null());
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
    let s1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "test_constexpr()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
