extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() {
    let i: Value<()> = Rc::new(RefCell::new(1_()));
    let c: Value<()> = Rc::new(RefCell::new(('c' as ())));
    let p: Value<()> = Rc::new(RefCell::new(Default::default()));
    const _: () = assert!(
        true,
        "!std::is_default_constructible_v<std::basic_format_args<Context>>"
    );;
    {
        let store: Value<std___format_arg_store_std_basic_format_context_char_ptr__char___int_> =
            Rc::new(RefCell::new(({ make_format_args_1(i.as_pointer()) })));
        const _: () = assert!(true, "");;
        let format_args: Value<()> = Rc::new(RefCell::new(
            std_basic_format_args_std_basic_format_context_char_ptr__char__::new_1({
                store.as_pointer()
            }),
        ));
        unimplemented!();
        unimplemented!();
    }
    {
        let store: Value<
            std___format_arg_store_std_basic_format_context_char_ptr__char___int__char_,
        > = Rc::new(RefCell::new(
            ({ make_format_args_2(i.as_pointer(), c.as_pointer()) }),
        ));
        const _: () = assert!(true, "");;
        let format_args: Value<()> = Rc::new(RefCell::new(
            std_basic_format_args_std_basic_format_context_char_ptr__char__::new_2({
                store.as_pointer()
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let store : Value<std___format_arg_store_std_basic_format_context_char_ptr__char___int__char__std_nullptr_t_ > = Rc::new(RefCell::new(( { make_format_args_3 ( i .as_pointer()  , c .as_pointer()  , p .as_pointer()  , ) } )  )) ;
        const _: () = assert!(true, "");;
        let format_args: Value<()> = Rc::new(RefCell::new(
            std_basic_format_args_std_basic_format_context_char_ptr__char__::new_3({
                store.as_pointer()
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_4() {
    let i: Value<()> = Rc::new(RefCell::new(1_()));
    let c: Value<()> = Rc::new(RefCell::new(('c' as ())));
    let p: Value<()> = Rc::new(RefCell::new(Default::default()));
    const _: () = assert!(
        true,
        "!std::is_default_constructible_v<std::basic_format_args<Context>>"
    );;
    {
        let store: Value<
            std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___int_,
        > = Rc::new(RefCell::new(({ make_format_args_5(i.as_pointer()) })));
        const _: () = assert!(true, "");;
        let format_args: Value<()> = Rc::new(RefCell::new(
            std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_1({
                store.as_pointer()
            }),
        ));
        unimplemented!();
        unimplemented!();
    }
    {
        let store: Value<
            std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___int__char_,
        > = Rc::new(RefCell::new(
            ({ make_format_args_6(i.as_pointer(), c.as_pointer()) }),
        ));
        const _: () = assert!(true, "");;
        let format_args: Value<()> = Rc::new(RefCell::new(
            std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_2({
                store.as_pointer()
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let store : Value<std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___int__char__std_nullptr_t_ > = Rc::new(RefCell::new(( { make_format_args_7 ( i .as_pointer()  , c .as_pointer()  , p .as_pointer()  , ) } )  )) ;
        const _: () = assert!(true, "");;
        let format_args: Value<()> = Rc::new(RefCell::new(
            std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_3({
                store.as_pointer()
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_8() {
    ({ test_0() });
    ({ test_4() });
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
    ({ test_8() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
