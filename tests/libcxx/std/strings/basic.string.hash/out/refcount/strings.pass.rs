extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() {
    const _: () = assert!(true, "");;
    let h: Value<std_hash_std_basic_string_char__std_char_traits_char___std_allocator_char___> =
        Rc::new(RefCell::new(
            <std_hash_std_basic_string_char__std_char_traits_char___std_allocator_char___>::default(
            ),
        ));
    let g1: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"1234567890").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let g2: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"1234567891").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let s1: Value<Vec<i8>> = Rc::new(RefCell::new(
        match ({ RandomAccessIterator::Vec(g1.decay()) }, {
            RandomAccessIterator::Vec(g1.decay().to_last())
        }) {
            (mut __tmp_0, mut __tmp_1) => {
                let mut __v = Vec::new();
                let mut __it = (__tmp_0).clone();
                while __it != __tmp_1 {
                    __v.push(<i8>::from_i64(CChar::to_i64(__it.iter_deref().read())));
                    __it.iter_inc();
                }
                __v.push(0_i8);
                __v
            }
        },
    ));
    let s2: Value<Vec<i8>> = Rc::new(RefCell::new(
        match ({ RandomAccessIterator::Vec(g2.decay()) }, {
            RandomAccessIterator::Vec(g2.decay().to_last())
        }) {
            (mut __tmp_2, mut __tmp_3) => {
                let mut __v = Vec::new();
                let mut __it = (__tmp_2).clone();
                while __it != __tmp_3 {
                    __v.push(<i8>::from_i64(CChar::to_i64(__it.iter_deref().read())));
                    __it.iter_inc();
                }
                __v.push(0_i8);
                __v
            }
        },
    ));
    assert!(
        ({
            (({
                ( (*h.borrow_mut()) as std___str_hash_base_char__std_allocator_char___std_basic_string_char__std_char_traits_char___std_allocator_char___ ) ( s1 .as_pointer()  , )
            }) != ({
                ( (*h.borrow_mut()) as std___str_hash_base_char__std_allocator_char___std_basic_string_char__std_char_traits_char___std_allocator_char___ ) ( s2 .as_pointer()  , )
            }))
        })
    );
}
pub fn test_1() {
    const _: () = assert!(true, "");;
    let h : Value<std_hash_std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t___ > = Rc::new(RefCell::new(<std_hash_std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t___  >::default() )) ;
    let g1: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"1234567890").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let g2: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"1234567891").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let s1: Value<Vec<u8>> = Rc::new(RefCell::new(
        match ({ RandomAccessIterator::Vec(g1.decay()) }, {
            RandomAccessIterator::Vec(g1.decay().to_last())
        }) {
            (mut __tmp_4, mut __tmp_5) => {
                let mut __v = Vec::new();
                let mut __it = (__tmp_4).clone();
                while __it != __tmp_5 {
                    __v.push(<u8>::from_i64(CChar::to_i64(__it.iter_deref().read())));
                    __it.iter_inc();
                }
                __v.push(0_u8);
                __v
            }
        },
    ));
    let s2: Value<Vec<u8>> = Rc::new(RefCell::new(
        match ({ RandomAccessIterator::Vec(g2.decay()) }, {
            RandomAccessIterator::Vec(g2.decay().to_last())
        }) {
            (mut __tmp_6, mut __tmp_7) => {
                let mut __v = Vec::new();
                let mut __it = (__tmp_6).clone();
                while __it != __tmp_7 {
                    __v.push(<u8>::from_i64(CChar::to_i64(__it.iter_deref().read())));
                    __it.iter_inc();
                }
                __v.push(0_u8);
                __v
            }
        },
    ));
    assert!(
        ({
            (({
                ( (*h.borrow_mut()) as std___str_hash_base_char8_t__std_allocator_char8_t___std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t___ ) ( s1 .as_pointer()  , )
            }) != ({
                ( (*h.borrow_mut()) as std___str_hash_base_char8_t__std_allocator_char8_t___std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t___ ) ( s2 .as_pointer()  , )
            }))
        })
    );
}
pub fn test_2() {
    const _: () = assert!(true, "");;
    let h : Value<std_hash_std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t___ > = Rc::new(RefCell::new(<std_hash_std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t___  >::default() )) ;
    let g1: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"1234567890").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let g2: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"1234567891").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let s1: Value<Vec<u16>> = Rc::new(RefCell::new(
        match ({ RandomAccessIterator::Vec(g1.decay()) }, {
            RandomAccessIterator::Vec(g1.decay().to_last())
        }) {
            (mut __tmp_8, mut __tmp_9) => {
                let mut __v = Vec::new();
                let mut __it = (__tmp_8).clone();
                while __it != __tmp_9 {
                    __v.push(<u16>::from_i64(CChar::to_i64(__it.iter_deref().read())));
                    __it.iter_inc();
                }
                __v.push(0_u16);
                __v
            }
        },
    ));
    let s2: Value<Vec<u16>> = Rc::new(RefCell::new(
        match ({ RandomAccessIterator::Vec(g2.decay()) }, {
            RandomAccessIterator::Vec(g2.decay().to_last())
        }) {
            (mut __tmp_10, mut __tmp_11) => {
                let mut __v = Vec::new();
                let mut __it = (__tmp_10).clone();
                while __it != __tmp_11 {
                    __v.push(<u16>::from_i64(CChar::to_i64(__it.iter_deref().read())));
                    __it.iter_inc();
                }
                __v.push(0_u16);
                __v
            }
        },
    ));
    assert!(
        ({
            (({
                ( (*h.borrow_mut()) as std___str_hash_base_char16_t__std_allocator_char16_t___std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t___ ) ( s1 .as_pointer()  , )
            }) != ({
                ( (*h.borrow_mut()) as std___str_hash_base_char16_t__std_allocator_char16_t___std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t___ ) ( s2 .as_pointer()  , )
            }))
        })
    );
}
pub fn test_3() {
    const _: () = assert!(true, "");;
    let h : Value<std_hash_std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t___ > = Rc::new(RefCell::new(<std_hash_std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t___  >::default() )) ;
    let g1: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"1234567890").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let g2: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"1234567891").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let s1: Value<Vec<u32>> = Rc::new(RefCell::new(
        match ({ RandomAccessIterator::Vec(g1.decay()) }, {
            RandomAccessIterator::Vec(g1.decay().to_last())
        }) {
            (mut __tmp_12, mut __tmp_13) => {
                let mut __v = Vec::new();
                let mut __it = (__tmp_12).clone();
                while __it != __tmp_13 {
                    __v.push(<u32>::from_i64(CChar::to_i64(__it.iter_deref().read())));
                    __it.iter_inc();
                }
                __v.push(0_u32);
                __v
            }
        },
    ));
    let s2: Value<Vec<u32>> = Rc::new(RefCell::new(
        match ({ RandomAccessIterator::Vec(g2.decay()) }, {
            RandomAccessIterator::Vec(g2.decay().to_last())
        }) {
            (mut __tmp_14, mut __tmp_15) => {
                let mut __v = Vec::new();
                let mut __it = (__tmp_14).clone();
                while __it != __tmp_15 {
                    __v.push(<u32>::from_i64(CChar::to_i64(__it.iter_deref().read())));
                    __it.iter_inc();
                }
                __v.push(0_u32);
                __v
            }
        },
    ));
    assert!(
        ({
            (({
                ( (*h.borrow_mut()) as std___str_hash_base_char32_t__std_allocator_char32_t___std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t___ ) ( s1 .as_pointer()  , )
            }) != ({
                ( (*h.borrow_mut()) as std___str_hash_base_char32_t__std_allocator_char32_t___std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t___ ) ( s2 .as_pointer()  , )
            }))
        })
    );
}
pub fn test_4() {
    const _: () = assert!(true, "");;
    let h : Value<std_hash_std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ > = Rc::new(RefCell::new(<std_hash_std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___  >::default() )) ;
    let g1: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"1234567890").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let g2: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"1234567891").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let s1: Value<Vec<i32>> = Rc::new(RefCell::new(
        match ({ RandomAccessIterator::Vec(g1.decay()) }, {
            RandomAccessIterator::Vec(g1.decay().to_last())
        }) {
            (mut __tmp_16, mut __tmp_17) => {
                let mut __v = Vec::new();
                let mut __it = (__tmp_16).clone();
                while __it != __tmp_17 {
                    __v.push(<i32>::from_i64(CChar::to_i64(__it.iter_deref().read())));
                    __it.iter_inc();
                }
                __v.push(0_i32);
                __v
            }
        },
    ));
    let s2: Value<Vec<i32>> = Rc::new(RefCell::new(
        match ({ RandomAccessIterator::Vec(g2.decay()) }, {
            RandomAccessIterator::Vec(g2.decay().to_last())
        }) {
            (mut __tmp_18, mut __tmp_19) => {
                let mut __v = Vec::new();
                let mut __it = (__tmp_18).clone();
                while __it != __tmp_19 {
                    __v.push(<i32>::from_i64(CChar::to_i64(__it.iter_deref().read())));
                    __it.iter_inc();
                }
                __v.push(0_i32);
                __v
            }
        },
    ));
    assert!(
        ({
            (({
                ( (*h.borrow_mut()) as std___str_hash_base_wchar_t__std_allocator_wchar_t___std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ ) ( s1 .as_pointer()  , )
            }) != ({
                ( (*h.borrow_mut()) as std___str_hash_base_wchar_t__std_allocator_wchar_t___std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ ) ( s2 .as_pointer()  , )
            }))
        })
    );
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
    ({ test_0() });
    ({ test_1() });
    ({ test_2() });
    ({ test_3() });
    ({ test_4() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
