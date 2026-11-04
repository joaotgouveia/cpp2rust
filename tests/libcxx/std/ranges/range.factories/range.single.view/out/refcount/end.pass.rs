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
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
impl CppMove for test_macros_detail_is_same_int_ptr__int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {}
impl CppMove for test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_Empty_ptr__Empty_ptr_ {}
impl CppMove for test_macros_detail_is_same_Empty_ptr__Empty_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_Empty_ptr__const_Empty_ptr_ {}
impl CppMove for test_macros_detail_is_same_const_Empty_ptr__const_Empty_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_4 = u32;
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_BigType_ptr__BigType_ptr_ {}
impl CppMove for test_macros_detail_is_same_BigType_ptr__BigType_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_5 = u32;
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_BigType_ptr__const_BigType_ptr_ {}
impl CppMove for test_macros_detail_is_same_const_BigType_ptr__const_BigType_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Empty {}
impl CppMove for Empty {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(64)]
pub struct BigType {
    #[offset(0)]
    #[byte_size(64)]
    pub buffer: Box<[i8]>,
}
impl CppMove for BigType {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for BigType {
    fn default() -> Self {
        {
            BigType {
                buffer: Box::new([
                    10_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
                    0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
                    0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
                    0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
                    0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
                ]),
            }
        }
    }
}
pub fn test_6() -> bool {
    {
        let sv: Value<std_ranges_single_view_int_> = Rc::new(RefCell::new({
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(42));
            std_ranges_single_view_int_::new_1({ __tmp_0.as_pointer() })
        }));
        assert!(
            ({
                (({ (*sv.borrow()).end_7() })
                    == ({ (*sv.borrow()).begin_5() }).offset((1) as isize))
            })
        );
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "noexcept(sv.end())");;
    }
    {
        let sv: Value<std_ranges_single_view_int_> = Rc::new(RefCell::new({
            let __tmp_1: Value<i32> = Rc::new(RefCell::new(42));
            std_ranges_single_view_int_::new_1({ __tmp_1.as_pointer() })
        }));
        assert!(
            ({
                (({ (*sv.borrow()).end_8() })
                    == ({ (*sv.borrow()).begin_6() }).offset((1) as isize))
            })
        );
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "noexcept(sv.end())");;
    }
    {
        let sv: Value<std_ranges_single_view_Empty_> = Rc::new(RefCell::new({
            let __tmp_2: Value<Empty> = Rc::new(RefCell::new(<Empty>::default()));
            std_ranges_single_view_Empty_::new_1({ __tmp_2.as_pointer() })
        }));
        assert!(
            ({
                (({ (*sv.borrow()).end_7() })
                    == ({ (*sv.borrow()).begin_5() }).offset((1) as isize))
            })
        );
        const _: () = assert!(true, "");;
    }
    {
        let sv: Value<std_ranges_single_view_Empty_> = Rc::new(RefCell::new({
            let __tmp_3: Value<Empty> = Rc::new(RefCell::new(<Empty>::default()));
            std_ranges_single_view_Empty_::new_1({ __tmp_3.as_pointer() })
        }));
        assert!(
            ({
                (({ (*sv.borrow()).end_8() })
                    == ({ (*sv.borrow()).begin_6() }).offset((1) as isize))
            })
        );
        const _: () = assert!(true, "");;
    }
    {
        let sv: Value<std_ranges_single_view_BigType_> = Rc::new(RefCell::new({
            let __tmp_4: Value<BigType> = Rc::new(RefCell::new(<BigType>::default()));
            std_ranges_single_view_BigType_::new_1({ __tmp_4.as_pointer() })
        }));
        assert!(
            ({
                (({ (*sv.borrow()).end_7() })
                    == ({ (*sv.borrow()).begin_5() }).offset((1) as isize))
            })
        );
        const _: () = assert!(true, "");;
    }
    {
        let sv: Value<std_ranges_single_view_BigType_> = Rc::new(RefCell::new({
            let __tmp_5: Value<BigType> = Rc::new(RefCell::new(<BigType>::default()));
            std_ranges_single_view_BigType_::new_1({ __tmp_5.as_pointer() })
        }));
        assert!(
            ({
                (({ (*sv.borrow()).end_8() })
                    == ({ (*sv.borrow()).begin_6() }).offset((1) as isize))
            })
        );
        const _: () = assert!(true, "");;
    }
    return true;
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
    ({ test_6() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
