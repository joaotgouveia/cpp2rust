extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_basic_string_view_char__std_char_traits_char____std_basic_string_view_char__std_char_traits_char___
{}
impl CppMove for test_macros_detail_is_same_std_basic_string_view_char__std_char_traits_char____std_basic_string_view_char__std_char_traits_char___ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_basic_string_view_wchar_t__constexpr_char_traits_wchar_t____std_basic_string_view_wchar_t__constexpr_char_traits_wchar_t___
{}
impl CppMove for test_macros_detail_is_same_std_basic_string_view_wchar_t__constexpr_char_traits_wchar_t____std_basic_string_view_wchar_t__constexpr_char_traits_wchar_t___ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_basic_string_view_wchar_t__std_char_traits_wchar_t____std_basic_string_view_wchar_t__std_char_traits_wchar_t___
{}
impl CppMove for test_macros_detail_is_same_std_basic_string_view_wchar_t__std_char_traits_wchar_t____std_basic_string_view_wchar_t__std_char_traits_wchar_t___ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct constexpr_char_traits_wchar_t_ {}
impl constexpr_char_traits_wchar_t_ {
    pub fn eq(mut c1: i32, mut c2: i32) -> bool {
        return (c1 == c2);
    }
    pub fn lt(mut c1: i32, mut c2: i32) -> bool {
        return (c1 < c2);
    }
    pub fn compare(mut s1: Ptr<i32>, mut s2: Ptr<i32>, mut n: usize) -> i32 {
        'loop_: while (n != 0) {
            if ({
                let _c1: i32 = (s1.read());
                let _c2: i32 = (s2.read());
                constexpr_char_traits_wchar_t_::lt(_c1, _c2)
            }) {
                return -1_i32;
            }
            if ({
                let _c1: i32 = (s2.read());
                let _c2: i32 = (s1.read());
                constexpr_char_traits_wchar_t_::lt(_c1, _c2)
            }) {
                return 1;
            }
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return 0;
    }
    pub fn length(mut s: Ptr<i32>) -> usize {
        let len: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while !({
            let _c1: i32 = (s.read());
            let _c2: i32 = 0_i32;
            constexpr_char_traits_wchar_t_::eq(_c1, _c2)
        }) {
            (*len.borrow_mut()).prefix_inc();
            s.prefix_inc();
        }
        return (*len.borrow());
    }
}
impl CppMove for constexpr_char_traits_wchar_t_ {
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
    {}
    {
        let sin: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        let win: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let w: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
    }
    {
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        let w: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
    }
    {
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        let w: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
