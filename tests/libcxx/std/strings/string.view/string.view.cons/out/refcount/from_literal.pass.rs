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
    pub fn eq(mut c1: i8, mut c2: i8) -> bool {
        return ((c1 as i32) == (c2 as i32));
    }
    pub fn length(mut s: Ptr<i8>) -> usize {
        let len: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while !({
            let _c1: i8 = (s.read());
            let _c2: i8 = 0_i8;
            constexpr_char_traits_char_::eq(_c1, _c2)
        }) {
            (*len.borrow_mut()).prefix_inc();
            s.prefix_inc();
        }
        return (*len.borrow());
    }
}
impl CppMove for constexpr_char_traits_char_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn StrLen_0(mut s: Ptr<i8>) -> usize {
    let retVal: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while (((s.read()) as i32) != 0) {
        (*retVal.borrow_mut()).prefix_inc();
        s.prefix_inc();
    }
    return (*retVal.borrow());
}
pub fn StrLen_1(mut s: Ptr<i32>) -> usize {
    let retVal: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((s.read()) != 0) {
        (*retVal.borrow_mut()).prefix_inc();
        s.prefix_inc();
    }
    return (*retVal.borrow());
}
pub fn StrLen_2(mut s: Ptr<u16>) -> usize {
    let retVal: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while (((s.read()) as i32) != 0) {
        (*retVal.borrow_mut()).prefix_inc();
        s.prefix_inc();
    }
    return (*retVal.borrow());
}
pub fn StrLen_3(mut s: Ptr<u32>) -> usize {
    let retVal: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((s.read()) != 0_u32) {
        (*retVal.borrow_mut()).prefix_inc();
        s.prefix_inc();
    }
    return (*retVal.borrow());
}
pub fn test_4(mut s: Ptr<i8>) {
    let sv1: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_basic_string_view_char__std_char_traits_char__::new_1({ (s).clone() }),
    ));
    assert!(
        ({ ({ (({ (*sv1.borrow()).size() }) as usize) } == { ({ StrLen_0((s).clone(),) }) }) })
    );
    assert!(({ ({ ({ (*sv1.borrow()).data() }) } == { (s).clone() }) }));
}
pub fn test_5(mut s: Ptr<i32>) {
    let sv1: Value<std_basic_string_view_wchar_t__std_char_traits_wchar_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_wchar_t__std_char_traits_wchar_t__::new_1({ (s).clone() }),
        ));
    assert!(
        ({ ({ (({ (*sv1.borrow()).size() }) as usize) } == { ({ StrLen_1((s).clone(),) }) }) })
    );
    assert!(({ ({ ({ (*sv1.borrow()).data() }) } == { (s).clone() }) }));
}
pub fn test_6(mut s: Ptr<u16>) {
    let sv1: Value<std_basic_string_view_char16_t__std_char_traits_char16_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char16_t__std_char_traits_char16_t__::new_1({ (s).clone() }),
        ));
    assert!(
        ({ ({ (({ (*sv1.borrow()).size() }) as usize) } == { ({ StrLen_2((s).clone(),) }) }) })
    );
    assert!(({ ({ ({ (*sv1.borrow()).data() }) } == { (s).clone() }) }));
}
pub fn test_7(mut s: Ptr<u32>) {
    let sv1: Value<std_basic_string_view_char32_t__std_char_traits_char32_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char32_t__std_char_traits_char32_t__::new_1({ (s).clone() }),
        ));
    assert!(
        ({ ({ (({ (*sv1.borrow()).size() }) as usize) } == { ({ StrLen_3((s).clone(),) }) }) })
    );
    assert!(({ ({ ({ (*sv1.borrow()).data() }) } == { (s).clone() }) }));
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
    ({ test_4(Ptr::<i8>::from_string_literal(b"QBCDE")) });
    ({ test_4(Ptr::<i8>::from_string_literal(b"A")) });
    ({ test_4(Ptr::<i8>::from_string_literal(b"")) });
    ({
        test_5(Ptr::<i32>::from_string_literal(&[
            81 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 0 as i32,
        ]))
    });
    ({ test_5(Ptr::<i32>::from_string_literal(&[65 as i32, 0 as i32])) });
    ({ test_5(Ptr::<i32>::from_string_literal(&[0 as i32])) });
    ({
        test_6(Ptr::<u16>::from_string_literal(&[
            81 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 0 as u16,
        ]))
    });
    ({ test_6(Ptr::<u16>::from_string_literal(&[65 as u16, 0 as u16])) });
    ({ test_6(Ptr::<u16>::from_string_literal(&[0 as u16])) });
    ({
        test_7(Ptr::<u32>::from_string_literal(&[
            81 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 0 as u32,
        ]))
    });
    ({ test_7(Ptr::<u32>::from_string_literal(&[65 as u32, 0 as u32])) });
    ({ test_7(Ptr::<u32>::from_string_literal(&[0 as u32])) });
    {
        let sv1: Value<std_basic_string_view_char__constexpr_char_traits_char__> =
            Rc::new(RefCell::new(
                std_basic_string_view_char__constexpr_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"ABCDE")
                }),
            ));
        const _: () = assert!(true, "sv1.size() == 5");;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
