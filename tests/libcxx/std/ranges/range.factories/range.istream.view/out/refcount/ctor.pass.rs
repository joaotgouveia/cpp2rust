extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn make_string_0(in_: Ptr<()>) -> () {
    return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { (in_).offset((0_()) as isize) },
        { (in_).offset(((4_() as ()).wrapping_sub(1_())) as isize) },
        None,
    );
}
pub fn make_string_1(in_: Ptr<()>) -> () {
    return std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
        { (in_).offset((0_()) as isize) },
        { (in_).offset(((4_() as ()).wrapping_sub(1_())) as isize) },
        None,
    );
}
pub fn make_string_stream_2(in_: Ptr<()>) -> () {
    return unimplemented!();
}
pub fn make_string_stream_3(in_: Ptr<()>) -> () {
    return unimplemented!();
}
const _: () = assert!(
    true,
    "std::constructible_from<std::ranges::istream_view<int>, std::istream&>"
);
const _: () = assert!(
    !(false),
    "!std::convertible_to<std::istream&, std::ranges::istream_view<int>>"
);
const _: () = assert!(
    true,
    "std::constructible_from<std::ranges::wistream_view<int>, std::wistream&>"
);
const _: () = assert!(
    !(false),
    "!std::convertible_to<std::wistream&, std::ranges::wistream_view<int>>"
);
pub fn test_4() {
    {
        let iss: Value<()> = Rc::new(RefCell::new(
            ({ make_string_stream_2(Ptr::<()>::from_string_literal(b"123")) }),
        ));
        let isv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let it: Value<std_ranges_basic_istream_view_int__char____iterator> =
            Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let iss: Value<()> = Rc::new(RefCell::new(
            ({ make_string_stream_2(Ptr::<()>::from_string_literal(b"123")) }),
        ));
        let isv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let iter: Value<std_ranges_basic_istream_view_int__char____iterator> =
            Rc::new(RefCell::new(
                std_ranges_basic_istream_view_int__char____iterator::new({ isv.as_pointer() }),
            ));
        unimplemented!();
    }
}
pub fn test_5() {
    {
        let iss: Value<()> = Rc::new(RefCell::new(
            ({ make_string_stream_3(Ptr::<()>::from_string_literal(b"123")) }),
        ));
        let isv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let it: Value<std_ranges_basic_istream_view_int__wchar_t____iterator> =
            Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let iss: Value<()> = Rc::new(RefCell::new(
            ({ make_string_stream_3(Ptr::<()>::from_string_literal(b"123")) }),
        ));
        let isv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let iter: Value<std_ranges_basic_istream_view_int__wchar_t____iterator> =
            Rc::new(RefCell::new(
                std_ranges_basic_istream_view_int__wchar_t____iterator::new({ isv.as_pointer() }),
            ));
        unimplemented!();
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
    ({ test_4() });
    ({ test_5() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
