extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(mut s: Ptr<i8>, mut len: usize) {
    {
        let sv1: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
            std_basic_string_view_char__std_char_traits_char__::new_1({ (s).clone() }),
        ));
        assert!(({ ((({ (*sv1.borrow()).size() }) as usize) == len) }));
        assert!(({ ({ ({ (*sv1.borrow()).data() }) } == { (s).clone() }) }));
        if (len > 0_usize) {
            ({ (*sv1.borrow()).remove_prefix(1_u64) });
            assert!(
                ({ ((({ (*sv1.borrow()).size() }) as usize) == ((len).wrapping_sub(1_usize))) })
            );
            assert!(({ ({ ({ (*sv1.borrow()).data() }) } == { (s.offset((1) as isize)) }) }));
            ({ (*sv1.borrow()).remove_prefix(((len).wrapping_sub(1_usize) as u64)) });
        }
        assert!(({ (({ (*sv1.borrow()).size() }) == 0_u64) }));
        ({ (*sv1.borrow()).remove_prefix(0_u64) });
        assert!(({ (({ (*sv1.borrow()).size() }) == 0_u64) }));
    }
}
pub fn test_1(mut s: Ptr<i32>, mut len: usize) {
    {
        let sv1: Value<std_basic_string_view_wchar_t__std_char_traits_wchar_t__> =
            Rc::new(RefCell::new(
                std_basic_string_view_wchar_t__std_char_traits_wchar_t__::new_1({ (s).clone() }),
            ));
        assert!(({ ((({ (*sv1.borrow()).size() }) as usize) == len) }));
        assert!(({ ({ ({ (*sv1.borrow()).data() }) } == { (s).clone() }) }));
        if (len > 0_usize) {
            ({ (*sv1.borrow()).remove_prefix(1_u64) });
            assert!(
                ({ ((({ (*sv1.borrow()).size() }) as usize) == ((len).wrapping_sub(1_usize))) })
            );
            assert!(({ ({ ({ (*sv1.borrow()).data() }) } == { (s.offset((1) as isize)) }) }));
            ({ (*sv1.borrow()).remove_prefix(((len).wrapping_sub(1_usize) as u64)) });
        }
        assert!(({ (({ (*sv1.borrow()).size() }) == 0_u64) }));
        ({ (*sv1.borrow()).remove_prefix(0_u64) });
        assert!(({ (({ (*sv1.borrow()).size() }) == 0_u64) }));
    }
}
pub fn test_2(mut s: Ptr<u16>, mut len: usize) {
    {
        let sv1: Value<std_basic_string_view_char16_t__std_char_traits_char16_t__> =
            Rc::new(RefCell::new(
                std_basic_string_view_char16_t__std_char_traits_char16_t__::new_1({ (s).clone() }),
            ));
        assert!(({ ((({ (*sv1.borrow()).size() }) as usize) == len) }));
        assert!(({ ({ ({ (*sv1.borrow()).data() }) } == { (s).clone() }) }));
        if (len > 0_usize) {
            ({ (*sv1.borrow()).remove_prefix(1_u64) });
            assert!(
                ({ ((({ (*sv1.borrow()).size() }) as usize) == ((len).wrapping_sub(1_usize))) })
            );
            assert!(({ ({ ({ (*sv1.borrow()).data() }) } == { (s.offset((1) as isize)) }) }));
            ({ (*sv1.borrow()).remove_prefix(((len).wrapping_sub(1_usize) as u64)) });
        }
        assert!(({ (({ (*sv1.borrow()).size() }) == 0_u64) }));
        ({ (*sv1.borrow()).remove_prefix(0_u64) });
        assert!(({ (({ (*sv1.borrow()).size() }) == 0_u64) }));
    }
}
pub fn test_3(mut s: Ptr<u32>, mut len: usize) {
    {
        let sv1: Value<std_basic_string_view_char32_t__std_char_traits_char32_t__> =
            Rc::new(RefCell::new(
                std_basic_string_view_char32_t__std_char_traits_char32_t__::new_1({ (s).clone() }),
            ));
        assert!(({ ((({ (*sv1.borrow()).size() }) as usize) == len) }));
        assert!(({ ({ ({ (*sv1.borrow()).data() }) } == { (s).clone() }) }));
        if (len > 0_usize) {
            ({ (*sv1.borrow()).remove_prefix(1_u64) });
            assert!(
                ({ ((({ (*sv1.borrow()).size() }) as usize) == ((len).wrapping_sub(1_usize))) })
            );
            assert!(({ ({ ({ (*sv1.borrow()).data() }) } == { (s.offset((1) as isize)) }) }));
            ({ (*sv1.borrow()).remove_prefix(((len).wrapping_sub(1_usize) as u64)) });
        }
        assert!(({ (({ (*sv1.borrow()).size() }) == 0_u64) }));
        ({ (*sv1.borrow()).remove_prefix(0_u64) });
        assert!(({ (({ (*sv1.borrow()).size() }) == 0_u64) }));
    }
}
pub fn test_ce_4(mut n: usize, mut k: usize) -> usize {
    let sv1: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_basic_string_view_char__std_char_traits_char__::new_2(
            { Ptr::<i8>::from_string_literal(b"ABCDEFGHIJKL") },
            { (n as u64) },
        ),
    ));
    ({ (*sv1.borrow()).remove_prefix((k as u64)) });
    return (({ (*sv1.borrow()).size() }) as usize);
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
    ({ test_0(Ptr::<i8>::from_string_literal(b"ABCDE"), 5_usize) });
    ({ test_0(Ptr::<i8>::from_string_literal(b"a"), 1_usize) });
    ({ test_0(Ptr::<i8>::from_string_literal(b""), 0_usize) });
    ({
        test_1(
            Ptr::<i32>::from_string_literal(&[
                65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 0 as i32,
            ]),
            5_usize,
        )
    });
    ({
        test_1(
            Ptr::<i32>::from_string_literal(&[97 as i32, 0 as i32]),
            1_usize,
        )
    });
    ({ test_1(Ptr::<i32>::from_string_literal(&[0 as i32]), 0_usize) });
    ({
        test_2(
            Ptr::<u16>::from_string_literal(&[
                65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 0 as u16,
            ]),
            5_usize,
        )
    });
    ({
        test_2(
            Ptr::<u16>::from_string_literal(&[97 as u16, 0 as u16]),
            1_usize,
        )
    });
    ({ test_2(Ptr::<u16>::from_string_literal(&[0 as u16]), 0_usize) });
    ({
        test_3(
            Ptr::<u32>::from_string_literal(&[
                65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 0 as u32,
            ]),
            5_usize,
        )
    });
    ({
        test_3(
            Ptr::<u32>::from_string_literal(&[97 as u32, 0 as u32]),
            1_usize,
        )
    });
    ({ test_3(Ptr::<u32>::from_string_literal(&[0 as u32]), 0_usize) });
    {
        const _: () = assert!(true, "test_ce(5, 0) == 5");;
        const _: () = assert!(true, "test_ce(5, 1) == 4");;
        const _: () = assert!(true, "test_ce(5, 5) == 0");;
        const _: () = assert!(true, "test_ce(9, 3) == 6");;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
