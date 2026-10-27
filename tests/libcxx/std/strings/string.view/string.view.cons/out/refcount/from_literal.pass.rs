extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct constexpr_char_traits_char_ {}
impl constexpr_char_traits_char_ {
    pub fn eq(mut c1: (), mut c2: ()) -> () {
        return (c1 == c2);
    }
    pub fn length(mut s: Ptr<()>) -> () {
        let len: Value<()> = Rc::new(RefCell::new(0_()));
        'loop_: while !({
            let _c1: () = (s.read());
            let _c2: () = 0_();
            constexpr_char_traits_char_::eq(_c1, _c2)
        }) {
            (*len.borrow_mut()).prefix_inc();
            s.prefix_inc();
        }
        return (*len.borrow());
    }
}
pub fn StrLen_0(mut s: Ptr<()>) -> () {
    let retVal: Value<()> = Rc::new(RefCell::new(0_()));
    'loop_: while ((s.read()) != 0_()) {
        (*retVal.borrow_mut()).prefix_inc();
        s.prefix_inc();
    }
    return (*retVal.borrow());
}
pub fn StrLen_1(mut s: Ptr<()>) -> () {
    let retVal: Value<()> = Rc::new(RefCell::new(0_()));
    'loop_: while ((s.read()) != 0_()) {
        (*retVal.borrow_mut()).prefix_inc();
        s.prefix_inc();
    }
    return (*retVal.borrow());
}
pub fn StrLen_2(mut s: Ptr<()>) -> () {
    let retVal: Value<()> = Rc::new(RefCell::new(0_()));
    'loop_: while ((s.read()) != 0_()) {
        (*retVal.borrow_mut()).prefix_inc();
        s.prefix_inc();
    }
    return (*retVal.borrow());
}
pub fn StrLen_3(mut s: Ptr<()>) -> () {
    let retVal: Value<()> = Rc::new(RefCell::new(0_()));
    'loop_: while ((s.read()) != 0_()) {
        (*retVal.borrow_mut()).prefix_inc();
        s.prefix_inc();
    }
    return (*retVal.borrow());
}
pub fn test_4(mut s: Ptr<()>) {
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_5(mut s: Ptr<()>) {
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_6(mut s: Ptr<()>) {
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_7(mut s: Ptr<()>) {
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
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
    ({ test_4(Ptr::<()>::from_string_literal(b"QBCDE")) });
    ({ test_4(Ptr::<()>::from_string_literal(b"A")) });
    ({ test_4(Ptr::<()>::from_string_literal(b"")) });
    ({
        test_5(Ptr::<()>::from_string_literal(&[
            81 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as (),
        ]))
    });
    ({ test_5(Ptr::<()>::from_string_literal(&[65 as (), 0 as ()])) });
    ({ test_5(Ptr::<()>::from_string_literal(&[0 as ()])) });
    ({
        test_6(Ptr::<()>::from_string_literal(&[
            81 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as (),
        ]))
    });
    ({ test_6(Ptr::<()>::from_string_literal(&[65 as (), 0 as ()])) });
    ({ test_6(Ptr::<()>::from_string_literal(&[0 as ()])) });
    ({
        test_7(Ptr::<()>::from_string_literal(&[
            81 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as (),
        ]))
    });
    ({ test_7(Ptr::<()>::from_string_literal(&[65 as (), 0 as ()])) });
    ({ test_7(Ptr::<()>::from_string_literal(&[0 as ()])) });
    {
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sv1.size() == 5");;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
