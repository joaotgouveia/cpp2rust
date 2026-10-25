extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_format_context_create_0(out_it: (), args: ()) -> () {
    let out_it: Value<()> = Rc::new(RefCell::new(out_it));
    let args: Value<()> = Rc::new(RefCell::new(args));
    return ({
        __format_context_create_1(
            (*out_it.borrow_mut()).clone(),
            (*args.borrow()).clone(),
            None,
        )
    });
}
pub type color = ();
pub const color_black: color = 0;
pub const color_red: color = 1;
pub const color_gold: color = 2;
thread_local!(
    pub static color_names_2: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<()>::from_string_literal(b"black"),
        Ptr::<()>::from_string_literal(b"red"),
        Ptr::<()>::from_string_literal(b"gold"),
    ])));
);
pub fn test_3(mut expected: (), fmt: (), mut arg: color, mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<std_formatter_color__char_> =
        Rc::new(RefCell::new(<std_formatter_color__char_>::default()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({ test_format_context_create_0((*out.borrow()).clone(), unimplemented!()) }),
    ));
    ({
        std_formatter_color__char_Impl :: format_1_class_std_basic_format_context_class_std_back_insert_iterator_std_string___char_ ( &formatter .as_pointer()  , arg  , format_ctx .as_pointer()  , )
    });
    unimplemented!();
}
pub fn test_termination_condition_4(mut expected: (), mut f: (), mut arg: color) {
    let fmt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    ({ test_3(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    ({ test_3(unimplemented!(), unimplemented!(), arg, 0_()) });
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
    ({ test_termination_condition_4(unimplemented!(), unimplemented!(), color_black) });
    ({ test_termination_condition_4(unimplemented!(), unimplemented!(), color_red) });
    ({ test_termination_condition_4(unimplemented!(), unimplemented!(), color_gold) });
    return 0_();
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct std_formatter_color__char_;
pub fn __cpp2rust_init_globals() {
    let _ = color_names_2.with(|_| ());
}
