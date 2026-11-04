extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn make_string_0(in_: Ptr<i8>) -> Vec<i8> {
    return match ({ (in_).offset(((3_u64 as u64).wrapping_sub(1_u64)) as isize) },) {
        (mut __tmp_0,) => {
            let mut __v = Vec::new();
            let mut __it = (in_).offset((0) as isize);
            while __it != __tmp_0 {
                __v.push(<i8>::from_i64(CChar::to_i64(__it.iter_deref().read())));
                __it.iter_inc();
            }
            __v.push(0_i8);
            __v
        }
    };
}
pub fn make_string_1(in_: Ptr<i8>) -> Vec<i32> {
    return match ({ (in_).offset(((3_u64 as u64).wrapping_sub(1_u64)) as isize) },) {
        (mut __tmp_1,) => {
            let mut __v = Vec::new();
            let mut __it = (in_).offset((0) as isize);
            while __it != __tmp_1 {
                __v.push(<i32>::from_i64(CChar::to_i64(__it.iter_deref().read())));
                __it.iter_inc();
            }
            __v.push(0_i32);
            __v
        }
    };
}
pub fn make_string_stream_2(
    in_: Ptr<i8>,
) -> std_basic_istringstream_char__std_char_traits_char___std_allocator_char__ {
    return {
        let __tmp_2: Value<Vec<i8>> = Rc::new(RefCell::new(({ make_string_0((in_).clone()) })));
        std_basic_istringstream_char__std_char_traits_char___std_allocator_char__::new_1(
            { __tmp_2.as_pointer() },
            None,
        )
    };
}
pub fn make_string_stream_3(
    in_: Ptr<i8>,
) -> std_basic_istringstream_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ {
    return {
        let __tmp_3: Value<Vec<i32>> = Rc::new(RefCell::new(({ make_string_1((in_).clone()) })));
        std_basic_istringstream_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
            { __tmp_3.as_pointer() },
            None,
        )
    };
}
const _: () = assert!(true, "NoexceptEnd<std::ranges::istream_view<int>>");
const _: () = assert!(true, "NoexceptEnd<const std::ranges::istream_view<int>>");
const _: () = assert!(true, "NoexceptEnd<std::ranges::wistream_view<int>>");
const _: () = assert!(true, "NoexceptEnd<const std::ranges::wistream_view<int>>");
pub fn test_4() {
    let iss: Value<std_basic_istringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            ({ make_string_stream_2(Ptr::<i8>::from_string_literal(b"12")) }),
        ));
    let isv: Value<std_ranges_basic_istream_view_int__char__std_char_traits_char__> =
        Rc::new(RefCell::new(
            std_ranges_basic_istream_view_int__char__std_char_traits_char__::new({
                iss.as_pointer()
            }),
        ));
    let sent: Value<std_default_sentinel_t> = Rc::new(RefCell::new(({ (*isv.borrow()).end() })));
}
pub fn test_5() {
    let iss: Value<
        std_basic_istringstream_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__,
    > = Rc::new(RefCell::new(
        ({ make_string_stream_3(Ptr::<i8>::from_string_literal(b"12")) }),
    ));
    let isv: Value<std_ranges_basic_istream_view_int__wchar_t__std_char_traits_wchar_t__> =
        Rc::new(RefCell::new(
            std_ranges_basic_istream_view_int__wchar_t__std_char_traits_wchar_t__::new({
                iss.as_pointer()
            }),
        ));
    let sent: Value<std_default_sentinel_t> = Rc::new(RefCell::new(({ (*isv.borrow()).end() })));
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
    ({ test_4() });
    ({ test_5() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
