extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(Ptr< i8  >  , to_pconsti8)]
#[cpp_into(Ptr< i32  >  , to_pconsti32)]
#[cpp_into(Ptr< u8  >  , to_pconstu8)]
#[cpp_into(Ptr< u16  >  , to_pconstu16)]
#[cpp_into(Ptr< u32  >  , to_pconstu32)]
#[byte_size(80)]
pub struct MultiStringType {
    #[offset(0)]
    #[byte_size(8)]
    pub w_: Ptr<i32>,
    #[offset(8)]
    pub wn_: usize,
    #[offset(16)]
    #[byte_size(8)]
    pub u8_: Ptr<u8>,
    #[offset(24)]
    pub u8n_: usize,
    #[offset(32)]
    #[byte_size(8)]
    pub u16_: Ptr<u16>,
    #[offset(40)]
    pub u16n_: usize,
    #[offset(48)]
    #[byte_size(8)]
    pub u32_: Ptr<u32>,
    #[offset(56)]
    pub u32n_: usize,
    #[offset(64)]
    #[byte_size(8)]
    pub s_: Ptr<i8>,
    #[offset(72)]
    pub sn_: usize,
}
impl MultiStringType {
    pub fn new(
        mut w: Ptr<i32>,
        mut wn: usize,
        mut u8: Ptr<u8>,
        mut u8n: usize,
        mut u16: Ptr<u16>,
        mut u16n: usize,
        mut u32: Ptr<u32>,
        mut u32n: usize,
        mut s: Ptr<i8>,
        mut sn: usize,
    ) -> Self {
        Self {
            w_: (w).clone(),
            wn_: wn,
            u8_: (u8).clone(),
            u8n_: u8n,
            u16_: (u16).clone(),
            u16n_: u16n,
            u32_: (u32).clone(),
            u32n_: u32n,
            s_: (s).clone(),
            sn_: sn,
        }
    }
}
impl CppMove for MultiStringType {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_fmt_0() {
    ({
        test_parse_1(
            std_basic_string_view_char__std_char_traits_char__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_2(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                { Ptr::<i32>::from_string_literal(&[0 as i32]) },
                                {
                                    ((::std::mem::size_of::<[i32; 1]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<u8>::from_string_literal(&[0 as u8]) },
                                {
                                    ((::std::mem::size_of::<[u8; 1]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<u16>::from_string_literal(&[0 as u16]) },
                                {
                                    ((::std::mem::size_of::<[u16; 1]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<u32>::from_string_literal(&[0 as u32]) },
                                {
                                    ((::std::mem::size_of::<[u32; 1]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<i8>::from_string_literal(b"") },
                                {
                                    (::std::mem::size_of::<[i8; 1]>() as usize)
                                        .wrapping_sub(1_usize)
                                },
                            )))
                            .as_pointer(),
                            Ptr::<i8>::null(),
                        )
                    })
                },
                {
                    (({
                        MultiStringTypeImpl::length_7(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                { Ptr::<i32>::from_string_literal(&[0 as i32]) },
                                {
                                    ((::std::mem::size_of::<[i32; 1]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<u8>::from_string_literal(&[0 as u8]) },
                                {
                                    ((::std::mem::size_of::<[u8; 1]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<u16>::from_string_literal(&[0 as u16]) },
                                {
                                    ((::std::mem::size_of::<[u16; 1]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<u32>::from_string_literal(&[0 as u32]) },
                                {
                                    ((::std::mem::size_of::<[u32; 1]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<i8>::from_string_literal(b"") },
                                {
                                    (::std::mem::size_of::<[i8; 1]>() as usize)
                                        .wrapping_sub(1_usize)
                                },
                            )))
                            .as_pointer(),
                            Ptr::<i8>::null(),
                        )
                    }) as u64)
                },
            ),
            0_usize,
        )
    });
    ({
        test_parse_1(
            std_basic_string_view_char__std_char_traits_char__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_2(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<i32>::from_string_literal(&[
                                        58 as i32, 53 as i32, 0 as i32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[i32; 3]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<u8>::from_string_literal(&[58 as u8, 53 as u8, 0 as u8]) },
                                {
                                    ((::std::mem::size_of::<[u8; 3]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u16>::from_string_literal(&[
                                        58 as u16, 53 as u16, 0 as u16,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u16; 3]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u32>::from_string_literal(&[
                                        58 as u32, 53 as u32, 0 as u32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u32; 3]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<i8>::from_string_literal(b":5") },
                                {
                                    (::std::mem::size_of::<[i8; 3]>() as usize)
                                        .wrapping_sub(1_usize)
                                },
                            )))
                            .as_pointer(),
                            Ptr::<i8>::null(),
                        )
                    })
                },
                {
                    (({
                        MultiStringTypeImpl::length_7(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<i32>::from_string_literal(&[
                                        58 as i32, 53 as i32, 0 as i32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[i32; 3]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<u8>::from_string_literal(&[58 as u8, 53 as u8, 0 as u8]) },
                                {
                                    ((::std::mem::size_of::<[u8; 3]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u16>::from_string_literal(&[
                                        58 as u16, 53 as u16, 0 as u16,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u16; 3]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u32>::from_string_literal(&[
                                        58 as u32, 53 as u32, 0 as u32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u32; 3]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<i8>::from_string_literal(b":5") },
                                {
                                    (::std::mem::size_of::<[i8; 3]>() as usize)
                                        .wrapping_sub(1_usize)
                                },
                            )))
                            .as_pointer(),
                            Ptr::<i8>::null(),
                        )
                    }) as u64)
                },
            ),
            0_usize,
        )
    });
    ({
        test_parse_1(
            std_basic_string_view_char__std_char_traits_char__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_2(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                { Ptr::<i32>::from_string_literal(&[125 as i32, 0 as i32]) },
                                {
                                    ((::std::mem::size_of::<[i32; 2]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<u8>::from_string_literal(&[125 as u8, 0 as u8]) },
                                {
                                    ((::std::mem::size_of::<[u8; 2]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<u16>::from_string_literal(&[125 as u16, 0 as u16]) },
                                {
                                    ((::std::mem::size_of::<[u16; 2]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<u32>::from_string_literal(&[125 as u32, 0 as u32]) },
                                {
                                    ((::std::mem::size_of::<[u32; 2]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<i8>::from_string_literal(b"}") },
                                {
                                    (::std::mem::size_of::<[i8; 2]>() as usize)
                                        .wrapping_sub(1_usize)
                                },
                            )))
                            .as_pointer(),
                            Ptr::<i8>::null(),
                        )
                    })
                },
                {
                    (({
                        MultiStringTypeImpl::length_7(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                { Ptr::<i32>::from_string_literal(&[125 as i32, 0 as i32]) },
                                {
                                    ((::std::mem::size_of::<[i32; 2]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<u8>::from_string_literal(&[125 as u8, 0 as u8]) },
                                {
                                    ((::std::mem::size_of::<[u8; 2]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<u16>::from_string_literal(&[125 as u16, 0 as u16]) },
                                {
                                    ((::std::mem::size_of::<[u16; 2]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<u32>::from_string_literal(&[125 as u32, 0 as u32]) },
                                {
                                    ((::std::mem::size_of::<[u32; 2]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<i8>::from_string_literal(b"}") },
                                {
                                    (::std::mem::size_of::<[i8; 2]>() as usize)
                                        .wrapping_sub(1_usize)
                                },
                            )))
                            .as_pointer(),
                            Ptr::<i8>::null(),
                        )
                    }) as u64)
                },
            ),
            1_usize,
        )
    });
    ({
        test_parse_1(
            std_basic_string_view_char__std_char_traits_char__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_2(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<i32>::from_string_literal(&[
                                        58 as i32, 53 as i32, 125 as i32, 0 as i32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[i32; 4]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u8>::from_string_literal(&[
                                        58 as u8, 53 as u8, 125 as u8, 0 as u8,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u8; 4]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u16>::from_string_literal(&[
                                        58 as u16, 53 as u16, 125 as u16, 0 as u16,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u16; 4]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u32>::from_string_literal(&[
                                        58 as u32, 53 as u32, 125 as u32, 0 as u32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u32; 4]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<i8>::from_string_literal(b":5}") },
                                {
                                    (::std::mem::size_of::<[i8; 4]>() as usize)
                                        .wrapping_sub(1_usize)
                                },
                            )))
                            .as_pointer(),
                            Ptr::<i8>::null(),
                        )
                    })
                },
                {
                    (({
                        MultiStringTypeImpl::length_7(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<i32>::from_string_literal(&[
                                        58 as i32, 53 as i32, 125 as i32, 0 as i32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[i32; 4]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u8>::from_string_literal(&[
                                        58 as u8, 53 as u8, 125 as u8, 0 as u8,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u8; 4]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u16>::from_string_literal(&[
                                        58 as u16, 53 as u16, 125 as u16, 0 as u16,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u16; 4]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u32>::from_string_literal(&[
                                        58 as u32, 53 as u32, 125 as u32, 0 as u32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u32; 4]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<i8>::from_string_literal(b":5}") },
                                {
                                    (::std::mem::size_of::<[i8; 4]>() as usize)
                                        .wrapping_sub(1_usize)
                                },
                            )))
                            .as_pointer(),
                            Ptr::<i8>::null(),
                        )
                    }) as u64)
                },
            ),
            1_usize,
        )
    });
}
pub fn test_fmt_2() {
    ({
        test_parse_3(
            std_basic_string_view_wchar_t__std_char_traits_wchar_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_3(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                { Ptr::<i32>::from_string_literal(&[0 as i32]) },
                                {
                                    ((::std::mem::size_of::<[i32; 1]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<u8>::from_string_literal(&[0 as u8]) },
                                {
                                    ((::std::mem::size_of::<[u8; 1]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<u16>::from_string_literal(&[0 as u16]) },
                                {
                                    ((::std::mem::size_of::<[u16; 1]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<u32>::from_string_literal(&[0 as u32]) },
                                {
                                    ((::std::mem::size_of::<[u32; 1]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<i8>::from_string_literal(b"") },
                                {
                                    (::std::mem::size_of::<[i8; 1]>() as usize)
                                        .wrapping_sub(1_usize)
                                },
                            )))
                            .as_pointer(),
                            Ptr::<i32>::null(),
                        )
                    })
                },
                {
                    (({
                        MultiStringTypeImpl::length_8(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                { Ptr::<i32>::from_string_literal(&[0 as i32]) },
                                {
                                    ((::std::mem::size_of::<[i32; 1]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<u8>::from_string_literal(&[0 as u8]) },
                                {
                                    ((::std::mem::size_of::<[u8; 1]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<u16>::from_string_literal(&[0 as u16]) },
                                {
                                    ((::std::mem::size_of::<[u16; 1]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<u32>::from_string_literal(&[0 as u32]) },
                                {
                                    ((::std::mem::size_of::<[u32; 1]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<i8>::from_string_literal(b"") },
                                {
                                    (::std::mem::size_of::<[i8; 1]>() as usize)
                                        .wrapping_sub(1_usize)
                                },
                            )))
                            .as_pointer(),
                            Ptr::<i32>::null(),
                        )
                    }) as u64)
                },
            ),
            0_usize,
        )
    });
    ({
        test_parse_3(
            std_basic_string_view_wchar_t__std_char_traits_wchar_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_3(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<i32>::from_string_literal(&[
                                        58 as i32, 53 as i32, 0 as i32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[i32; 3]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<u8>::from_string_literal(&[58 as u8, 53 as u8, 0 as u8]) },
                                {
                                    ((::std::mem::size_of::<[u8; 3]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u16>::from_string_literal(&[
                                        58 as u16, 53 as u16, 0 as u16,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u16; 3]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u32>::from_string_literal(&[
                                        58 as u32, 53 as u32, 0 as u32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u32; 3]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<i8>::from_string_literal(b":5") },
                                {
                                    (::std::mem::size_of::<[i8; 3]>() as usize)
                                        .wrapping_sub(1_usize)
                                },
                            )))
                            .as_pointer(),
                            Ptr::<i32>::null(),
                        )
                    })
                },
                {
                    (({
                        MultiStringTypeImpl::length_8(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<i32>::from_string_literal(&[
                                        58 as i32, 53 as i32, 0 as i32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[i32; 3]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<u8>::from_string_literal(&[58 as u8, 53 as u8, 0 as u8]) },
                                {
                                    ((::std::mem::size_of::<[u8; 3]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u16>::from_string_literal(&[
                                        58 as u16, 53 as u16, 0 as u16,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u16; 3]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u32>::from_string_literal(&[
                                        58 as u32, 53 as u32, 0 as u32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u32; 3]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<i8>::from_string_literal(b":5") },
                                {
                                    (::std::mem::size_of::<[i8; 3]>() as usize)
                                        .wrapping_sub(1_usize)
                                },
                            )))
                            .as_pointer(),
                            Ptr::<i32>::null(),
                        )
                    }) as u64)
                },
            ),
            0_usize,
        )
    });
    ({
        test_parse_3(
            std_basic_string_view_wchar_t__std_char_traits_wchar_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_3(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                { Ptr::<i32>::from_string_literal(&[125 as i32, 0 as i32]) },
                                {
                                    ((::std::mem::size_of::<[i32; 2]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<u8>::from_string_literal(&[125 as u8, 0 as u8]) },
                                {
                                    ((::std::mem::size_of::<[u8; 2]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<u16>::from_string_literal(&[125 as u16, 0 as u16]) },
                                {
                                    ((::std::mem::size_of::<[u16; 2]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<u32>::from_string_literal(&[125 as u32, 0 as u32]) },
                                {
                                    ((::std::mem::size_of::<[u32; 2]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<i8>::from_string_literal(b"}") },
                                {
                                    (::std::mem::size_of::<[i8; 2]>() as usize)
                                        .wrapping_sub(1_usize)
                                },
                            )))
                            .as_pointer(),
                            Ptr::<i32>::null(),
                        )
                    })
                },
                {
                    (({
                        MultiStringTypeImpl::length_8(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                { Ptr::<i32>::from_string_literal(&[125 as i32, 0 as i32]) },
                                {
                                    ((::std::mem::size_of::<[i32; 2]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<u8>::from_string_literal(&[125 as u8, 0 as u8]) },
                                {
                                    ((::std::mem::size_of::<[u8; 2]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<u16>::from_string_literal(&[125 as u16, 0 as u16]) },
                                {
                                    ((::std::mem::size_of::<[u16; 2]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<u32>::from_string_literal(&[125 as u32, 0 as u32]) },
                                {
                                    ((::std::mem::size_of::<[u32; 2]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<i8>::from_string_literal(b"}") },
                                {
                                    (::std::mem::size_of::<[i8; 2]>() as usize)
                                        .wrapping_sub(1_usize)
                                },
                            )))
                            .as_pointer(),
                            Ptr::<i32>::null(),
                        )
                    }) as u64)
                },
            ),
            1_usize,
        )
    });
    ({
        test_parse_3(
            std_basic_string_view_wchar_t__std_char_traits_wchar_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_3(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<i32>::from_string_literal(&[
                                        58 as i32, 53 as i32, 125 as i32, 0 as i32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[i32; 4]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u8>::from_string_literal(&[
                                        58 as u8, 53 as u8, 125 as u8, 0 as u8,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u8; 4]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u16>::from_string_literal(&[
                                        58 as u16, 53 as u16, 125 as u16, 0 as u16,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u16; 4]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u32>::from_string_literal(&[
                                        58 as u32, 53 as u32, 125 as u32, 0 as u32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u32; 4]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<i8>::from_string_literal(b":5}") },
                                {
                                    (::std::mem::size_of::<[i8; 4]>() as usize)
                                        .wrapping_sub(1_usize)
                                },
                            )))
                            .as_pointer(),
                            Ptr::<i32>::null(),
                        )
                    })
                },
                {
                    (({
                        MultiStringTypeImpl::length_8(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<i32>::from_string_literal(&[
                                        58 as i32, 53 as i32, 125 as i32, 0 as i32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[i32; 4]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u8>::from_string_literal(&[
                                        58 as u8, 53 as u8, 125 as u8, 0 as u8,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u8; 4]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u16>::from_string_literal(&[
                                        58 as u16, 53 as u16, 125 as u16, 0 as u16,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u16; 4]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                {
                                    Ptr::<u32>::from_string_literal(&[
                                        58 as u32, 53 as u32, 125 as u32, 0 as u32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u32; 4]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<i8>::from_string_literal(b":5}") },
                                {
                                    (::std::mem::size_of::<[i8; 4]>() as usize)
                                        .wrapping_sub(1_usize)
                                },
                            )))
                            .as_pointer(),
                            Ptr::<i32>::null(),
                        )
                    }) as u64)
                },
            ),
            1_usize,
        )
    });
}
pub fn test_4() -> bool {
    ({ test_fmt_0() });
    ({ test_fmt_2() });
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
    ({ test_4() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub trait MultiStringTypeImpl {
    fn as_ptr_2(&self, _a0: Ptr<i8>) -> Ptr<i8>;
    fn as_ptr_3(&self, _a0: Ptr<i32>) -> Ptr<i32>;
    fn as_ptr_4(&self, _a0: Ptr<u8>) -> Ptr<u8>;
    fn as_ptr_5(&self, _a0: Ptr<u16>) -> Ptr<u16>;
    fn as_ptr_6(&self, _a0: Ptr<u32>) -> Ptr<u32>;
    fn length_7(&self, _a0: Ptr<i8>) -> usize;
    fn length_8(&self, _a0: Ptr<i32>) -> usize;
    fn length_9(&self, _a0: Ptr<u8>) -> usize;
    fn length_10(&self, _a0: Ptr<u16>) -> usize;
    fn length_11(&self, _a0: Ptr<u32>) -> usize;
    fn to_pconsti8(&self) -> Ptr<i8>;
    fn to_pconsti32(&self) -> Ptr<i32>;
    fn to_pconstu8(&self) -> Ptr<u8>;
    fn to_pconstu16(&self) -> Ptr<u16>;
    fn to_pconstu32(&self) -> Ptr<u32>;
}
impl MultiStringTypeImpl for Ptr<MultiStringType> {
    fn as_ptr_2(&self, mut _a0: Ptr<i8>) -> Ptr<i8> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn as_ptr_3(&self, mut _a0: Ptr<i32>) -> Ptr<i32> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn as_ptr_4(&self, mut _a0: Ptr<u8>) -> Ptr<u8> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn as_ptr_5(&self, mut _a0: Ptr<u16>) -> Ptr<u16> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn as_ptr_6(&self, mut _a0: Ptr<u32>) -> Ptr<u32> {
        return (*self).with(|__s| __s.u32_.clone());
    }
    fn length_7(&self, mut _a0: Ptr<i8>) -> usize {
        return (*self).with(|__s| __s.sn_);
    }
    fn length_8(&self, mut _a0: Ptr<i32>) -> usize {
        return (*self).with(|__s| __s.wn_);
    }
    fn length_9(&self, mut _a0: Ptr<u8>) -> usize {
        return (*self).with(|__s| __s.u8n_);
    }
    fn length_10(&self, mut _a0: Ptr<u16>) -> usize {
        return (*self).with(|__s| __s.u16n_);
    }
    fn length_11(&self, mut _a0: Ptr<u32>) -> usize {
        return (*self).with(|__s| __s.u32n_);
    }
    fn to_pconsti8(&self) -> Ptr<i8> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn to_pconsti32(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn to_pconstu8(&self) -> Ptr<u8> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn to_pconstu16(&self) -> Ptr<u16> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn to_pconstu32(&self) -> Ptr<u32> {
        return (*self).with(|__s| __s.u32_.clone());
    }
}
pub fn __cpp2rust_init_globals() {}
