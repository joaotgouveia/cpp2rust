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
pub fn test_type_0() {
    let mut ii1: Ptr<i8> = Ptr::<i8>::null();
    let mut ii2: Ptr<i8> = Ptr::<i8>::null();
    let mut ii4: Ptr<i8> = (ii1).clone();
    let mut cii: Ptr<i8> = Ptr::<i8>::null();
    assert!(({ (ii1).clone() } == { (ii2).clone() }));
    assert!(({ (ii1).clone() } == { (ii4).clone() }));
    assert!(({ (ii1).clone() } == { (cii).clone() }));
    assert!(!({ (ii1).clone() } != { (ii2).clone() }));
    assert!(!({ (ii1).clone() } != { (cii).clone() }));
    let c: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_basic_string_view_char__std_char_traits_char__::new_1(
            {
                ({
                    MultiStringTypeImpl::as_ptr_2(
                        &Rc::new(RefCell::new(MultiStringType::new(
                            {
                                Ptr::<i32>::from_string_literal(&[
                                    97 as i32, 98 as i32, 99 as i32, 0 as i32,
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
                                    97 as u8, 98 as u8, 99 as u8, 0 as u8,
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
                                    97 as u16, 98 as u16, 99 as u16, 0 as u16,
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
                                    97 as u32, 98 as u32, 99 as u32, 0 as u32,
                                ])
                            },
                            {
                                ((::std::mem::size_of::<[u32; 4]>() as usize)
                                    .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                    as usize)
                                    .wrapping_sub(1_usize)
                            },
                            { Ptr::<i8>::from_string_literal(b"abc") },
                            { (::std::mem::size_of::<[i8; 4]>() as usize).wrapping_sub(1_usize) },
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
                                    97 as i32, 98 as i32, 99 as i32, 0 as i32,
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
                                    97 as u8, 98 as u8, 99 as u8, 0 as u8,
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
                                    97 as u16, 98 as u16, 99 as u16, 0 as u16,
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
                                    97 as u32, 98 as u32, 99 as u32, 0 as u32,
                                ])
                            },
                            {
                                ((::std::mem::size_of::<[u32; 4]>() as usize)
                                    .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                    as usize)
                                    .wrapping_sub(1_usize)
                            },
                            { Ptr::<i8>::from_string_literal(b"abc") },
                            { (::std::mem::size_of::<[i8; 4]>() as usize).wrapping_sub(1_usize) },
                        )))
                        .as_pointer(),
                        Ptr::<i8>::null(),
                    )
                }) as u64)
            },
        ),
    ));
    assert!(({ (({ (*c.borrow()).begin() }) == ({ begin_1(c.as_pointer(),) })) }));
    assert!(({ (*c.borrow()).rbegin() }) == ({ rbegin_2(c.as_pointer(),) }));
    assert!(({ (({ (*c.borrow()).cbegin() }) == ({ cbegin_3(c.as_pointer(),) })) }));
    assert!(({ (*c.borrow()).crbegin() }) == ({ crbegin_4(c.as_pointer(),) }));
    assert!(({ (({ (*c.borrow()).end() }) == ({ libcc2rs::end_refcount(c.as_pointer(),) })) }));
    assert!(({ (*c.borrow()).rend() }) == ({ rend_5(c.as_pointer(),) }));
    assert!(({ (({ (*c.borrow()).cend() }) == ({ cend_6(c.as_pointer(),) })) }));
    assert!(({ (*c.borrow()).crend() }) == ({ crend_7(c.as_pointer(),) }));
    assert!((({ begin_1(c.as_pointer(),) }) != ({ libcc2rs::end_refcount(c.as_pointer(),) })));
    assert!(({ rbegin_2(c.as_pointer(),) }) != ({ rend_5(c.as_pointer(),) }));
    assert!((({ cbegin_3(c.as_pointer(),) }) != ({ cend_6(c.as_pointer(),) })));
    assert!(({ crbegin_4(c.as_pointer(),) }) != ({ crend_7(c.as_pointer(),) }));
    let r1: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(&(ii1), &(ii2))));
    assert!(({ (*r1.borrow()) }) == ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }));
    let r2: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(&(ii1), &(ii2))));
    assert!(({ (*r2.borrow()) }) == ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }));
}
pub fn test_type_8() {
    let mut ii1: Ptr<i32> = Ptr::<i32>::null();
    let mut ii2: Ptr<i32> = Ptr::<i32>::null();
    let mut ii4: Ptr<i32> = (ii1).clone();
    let mut cii: Ptr<i32> = Ptr::<i32>::null();
    assert!(({ (ii1).clone() } == { (ii2).clone() }));
    assert!(({ (ii1).clone() } == { (ii4).clone() }));
    assert!(({ (ii1).clone() } == { (cii).clone() }));
    assert!(!({ (ii1).clone() } != { (ii2).clone() }));
    assert!(!({ (ii1).clone() } != { (cii).clone() }));
    let c: Value<std_basic_string_view_wchar_t__std_char_traits_wchar_t__> = Rc::new(RefCell::new(
        std_basic_string_view_wchar_t__std_char_traits_wchar_t__::new_1(
            {
                ({
                    MultiStringTypeImpl::as_ptr_3(
                        &Rc::new(RefCell::new(MultiStringType::new(
                            {
                                Ptr::<i32>::from_string_literal(&[
                                    97 as i32, 98 as i32, 99 as i32, 0 as i32,
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
                                    97 as u8, 98 as u8, 99 as u8, 0 as u8,
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
                                    97 as u16, 98 as u16, 99 as u16, 0 as u16,
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
                                    97 as u32, 98 as u32, 99 as u32, 0 as u32,
                                ])
                            },
                            {
                                ((::std::mem::size_of::<[u32; 4]>() as usize)
                                    .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                    as usize)
                                    .wrapping_sub(1_usize)
                            },
                            { Ptr::<i8>::from_string_literal(b"abc") },
                            { (::std::mem::size_of::<[i8; 4]>() as usize).wrapping_sub(1_usize) },
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
                                    97 as i32, 98 as i32, 99 as i32, 0 as i32,
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
                                    97 as u8, 98 as u8, 99 as u8, 0 as u8,
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
                                    97 as u16, 98 as u16, 99 as u16, 0 as u16,
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
                                    97 as u32, 98 as u32, 99 as u32, 0 as u32,
                                ])
                            },
                            {
                                ((::std::mem::size_of::<[u32; 4]>() as usize)
                                    .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                    as usize)
                                    .wrapping_sub(1_usize)
                            },
                            { Ptr::<i8>::from_string_literal(b"abc") },
                            { (::std::mem::size_of::<[i8; 4]>() as usize).wrapping_sub(1_usize) },
                        )))
                        .as_pointer(),
                        Ptr::<i32>::null(),
                    )
                }) as u64)
            },
        ),
    ));
    assert!(({ (({ (*c.borrow()).begin() }) == ({ begin_9(c.as_pointer(),) })) }));
    assert!(({ (*c.borrow()).rbegin() }) == ({ rbegin_10(c.as_pointer(),) }));
    assert!(({ (({ (*c.borrow()).cbegin() }) == ({ cbegin_11(c.as_pointer(),) })) }));
    assert!(({ (*c.borrow()).crbegin() }) == ({ crbegin_12(c.as_pointer(),) }));
    assert!(({ (({ (*c.borrow()).end() }) == ({ libcc2rs::end_refcount(c.as_pointer(),) })) }));
    assert!(({ (*c.borrow()).rend() }) == ({ rend_13(c.as_pointer(),) }));
    assert!(({ (({ (*c.borrow()).cend() }) == ({ cend_14(c.as_pointer(),) })) }));
    assert!(({ (*c.borrow()).crend() }) == ({ crend_15(c.as_pointer(),) }));
    assert!((({ begin_9(c.as_pointer(),) }) != ({ libcc2rs::end_refcount(c.as_pointer(),) })));
    assert!(({ rbegin_10(c.as_pointer(),) }) != ({ rend_13(c.as_pointer(),) }));
    assert!((({ cbegin_11(c.as_pointer(),) }) != ({ cend_14(c.as_pointer(),) })));
    assert!(({ crbegin_12(c.as_pointer(),) }) != ({ crend_15(c.as_pointer(),) }));
    let r1: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(&(ii1), &(ii2))));
    assert!(({ (*r1.borrow()) }) == ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }));
    let r2: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(&(ii1), &(ii2))));
    assert!(({ (*r2.borrow()) }) == ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }));
}
pub fn test_type_16() {
    let mut ii1: Ptr<u8> = Ptr::<u8>::null();
    let mut ii2: Ptr<u8> = Ptr::<u8>::null();
    let mut ii4: Ptr<u8> = (ii1).clone();
    let mut cii: Ptr<u8> = Ptr::<u8>::null();
    assert!(({ (ii1).clone() } == { (ii2).clone() }));
    assert!(({ (ii1).clone() } == { (ii4).clone() }));
    assert!(({ (ii1).clone() } == { (cii).clone() }));
    assert!(!({ (ii1).clone() } != { (ii2).clone() }));
    assert!(!({ (ii1).clone() } != { (cii).clone() }));
    let c: Value<std_basic_string_view_char8_t__std_char_traits_char8_t__> = Rc::new(RefCell::new(
        std_basic_string_view_char8_t__std_char_traits_char8_t__::new_1(
            {
                ({
                    MultiStringTypeImpl::as_ptr_4(
                        &Rc::new(RefCell::new(MultiStringType::new(
                            {
                                Ptr::<i32>::from_string_literal(&[
                                    97 as i32, 98 as i32, 99 as i32, 0 as i32,
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
                                    97 as u8, 98 as u8, 99 as u8, 0 as u8,
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
                                    97 as u16, 98 as u16, 99 as u16, 0 as u16,
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
                                    97 as u32, 98 as u32, 99 as u32, 0 as u32,
                                ])
                            },
                            {
                                ((::std::mem::size_of::<[u32; 4]>() as usize)
                                    .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                    as usize)
                                    .wrapping_sub(1_usize)
                            },
                            { Ptr::<i8>::from_string_literal(b"abc") },
                            { (::std::mem::size_of::<[i8; 4]>() as usize).wrapping_sub(1_usize) },
                        )))
                        .as_pointer(),
                        Ptr::<u8>::null(),
                    )
                })
            },
            {
                (({
                    MultiStringTypeImpl::length_9(
                        &Rc::new(RefCell::new(MultiStringType::new(
                            {
                                Ptr::<i32>::from_string_literal(&[
                                    97 as i32, 98 as i32, 99 as i32, 0 as i32,
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
                                    97 as u8, 98 as u8, 99 as u8, 0 as u8,
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
                                    97 as u16, 98 as u16, 99 as u16, 0 as u16,
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
                                    97 as u32, 98 as u32, 99 as u32, 0 as u32,
                                ])
                            },
                            {
                                ((::std::mem::size_of::<[u32; 4]>() as usize)
                                    .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                    as usize)
                                    .wrapping_sub(1_usize)
                            },
                            { Ptr::<i8>::from_string_literal(b"abc") },
                            { (::std::mem::size_of::<[i8; 4]>() as usize).wrapping_sub(1_usize) },
                        )))
                        .as_pointer(),
                        Ptr::<u8>::null(),
                    )
                }) as u64)
            },
        ),
    ));
    assert!(({ (({ (*c.borrow()).begin() }) == ({ begin_17(c.as_pointer(),) })) }));
    assert!(({ (*c.borrow()).rbegin() }) == ({ rbegin_18(c.as_pointer(),) }));
    assert!(({ (({ (*c.borrow()).cbegin() }) == ({ cbegin_19(c.as_pointer(),) })) }));
    assert!(({ (*c.borrow()).crbegin() }) == ({ crbegin_20(c.as_pointer(),) }));
    assert!(({ (({ (*c.borrow()).end() }) == ({ libcc2rs::end_refcount(c.as_pointer(),) })) }));
    assert!(({ (*c.borrow()).rend() }) == ({ rend_21(c.as_pointer(),) }));
    assert!(({ (({ (*c.borrow()).cend() }) == ({ cend_22(c.as_pointer(),) })) }));
    assert!(({ (*c.borrow()).crend() }) == ({ crend_23(c.as_pointer(),) }));
    assert!((({ begin_17(c.as_pointer(),) }) != ({ libcc2rs::end_refcount(c.as_pointer(),) })));
    assert!(({ rbegin_18(c.as_pointer(),) }) != ({ rend_21(c.as_pointer(),) }));
    assert!((({ cbegin_19(c.as_pointer(),) }) != ({ cend_22(c.as_pointer(),) })));
    assert!(({ crbegin_20(c.as_pointer(),) }) != ({ crend_23(c.as_pointer(),) }));
    let r1: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(&(ii1), &(ii2))));
    assert!(({ (*r1.borrow()) }) == ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }));
    let r2: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(&(ii1), &(ii2))));
    assert!(({ (*r2.borrow()) }) == ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }));
}
pub fn test_type_24() {
    let mut ii1: Ptr<u16> = Ptr::<u16>::null();
    let mut ii2: Ptr<u16> = Ptr::<u16>::null();
    let mut ii4: Ptr<u16> = (ii1).clone();
    let mut cii: Ptr<u16> = Ptr::<u16>::null();
    assert!(({ (ii1).clone() } == { (ii2).clone() }));
    assert!(({ (ii1).clone() } == { (ii4).clone() }));
    assert!(({ (ii1).clone() } == { (cii).clone() }));
    assert!(!({ (ii1).clone() } != { (ii2).clone() }));
    assert!(!({ (ii1).clone() } != { (cii).clone() }));
    let c: Value<std_basic_string_view_char16_t__std_char_traits_char16_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char16_t__std_char_traits_char16_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_5(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<i32>::from_string_literal(&[
                                        97 as i32, 98 as i32, 99 as i32, 0 as i32,
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
                                        97 as u8, 98 as u8, 99 as u8, 0 as u8,
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
                                        97 as u16, 98 as u16, 99 as u16, 0 as u16,
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
                                        97 as u32, 98 as u32, 99 as u32, 0 as u32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u32; 4]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<i8>::from_string_literal(b"abc") },
                                {
                                    (::std::mem::size_of::<[i8; 4]>() as usize)
                                        .wrapping_sub(1_usize)
                                },
                            )))
                            .as_pointer(),
                            Ptr::<u16>::null(),
                        )
                    })
                },
                {
                    (({
                        MultiStringTypeImpl::length_10(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<i32>::from_string_literal(&[
                                        97 as i32, 98 as i32, 99 as i32, 0 as i32,
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
                                        97 as u8, 98 as u8, 99 as u8, 0 as u8,
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
                                        97 as u16, 98 as u16, 99 as u16, 0 as u16,
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
                                        97 as u32, 98 as u32, 99 as u32, 0 as u32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u32; 4]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<i8>::from_string_literal(b"abc") },
                                {
                                    (::std::mem::size_of::<[i8; 4]>() as usize)
                                        .wrapping_sub(1_usize)
                                },
                            )))
                            .as_pointer(),
                            Ptr::<u16>::null(),
                        )
                    }) as u64)
                },
            ),
        ));
    assert!(({ (({ (*c.borrow()).begin() }) == ({ begin_25(c.as_pointer(),) })) }));
    assert!(({ (*c.borrow()).rbegin() }) == ({ rbegin_26(c.as_pointer(),) }));
    assert!(({ (({ (*c.borrow()).cbegin() }) == ({ cbegin_27(c.as_pointer(),) })) }));
    assert!(({ (*c.borrow()).crbegin() }) == ({ crbegin_28(c.as_pointer(),) }));
    assert!(({ (({ (*c.borrow()).end() }) == ({ libcc2rs::end_refcount(c.as_pointer(),) })) }));
    assert!(({ (*c.borrow()).rend() }) == ({ rend_29(c.as_pointer(),) }));
    assert!(({ (({ (*c.borrow()).cend() }) == ({ cend_30(c.as_pointer(),) })) }));
    assert!(({ (*c.borrow()).crend() }) == ({ crend_31(c.as_pointer(),) }));
    assert!((({ begin_25(c.as_pointer(),) }) != ({ libcc2rs::end_refcount(c.as_pointer(),) })));
    assert!(({ rbegin_26(c.as_pointer(),) }) != ({ rend_29(c.as_pointer(),) }));
    assert!((({ cbegin_27(c.as_pointer(),) }) != ({ cend_30(c.as_pointer(),) })));
    assert!(({ crbegin_28(c.as_pointer(),) }) != ({ crend_31(c.as_pointer(),) }));
    let r1: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(&(ii1), &(ii2))));
    assert!(({ (*r1.borrow()) }) == ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }));
    let r2: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(&(ii1), &(ii2))));
    assert!(({ (*r2.borrow()) }) == ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }));
}
pub fn test_type_32() {
    let mut ii1: Ptr<u32> = Ptr::<u32>::null();
    let mut ii2: Ptr<u32> = Ptr::<u32>::null();
    let mut ii4: Ptr<u32> = (ii1).clone();
    let mut cii: Ptr<u32> = Ptr::<u32>::null();
    assert!(({ (ii1).clone() } == { (ii2).clone() }));
    assert!(({ (ii1).clone() } == { (ii4).clone() }));
    assert!(({ (ii1).clone() } == { (cii).clone() }));
    assert!(!({ (ii1).clone() } != { (ii2).clone() }));
    assert!(!({ (ii1).clone() } != { (cii).clone() }));
    let c: Value<std_basic_string_view_char32_t__std_char_traits_char32_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char32_t__std_char_traits_char32_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_6(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<i32>::from_string_literal(&[
                                        97 as i32, 98 as i32, 99 as i32, 0 as i32,
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
                                        97 as u8, 98 as u8, 99 as u8, 0 as u8,
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
                                        97 as u16, 98 as u16, 99 as u16, 0 as u16,
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
                                        97 as u32, 98 as u32, 99 as u32, 0 as u32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u32; 4]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<i8>::from_string_literal(b"abc") },
                                {
                                    (::std::mem::size_of::<[i8; 4]>() as usize)
                                        .wrapping_sub(1_usize)
                                },
                            )))
                            .as_pointer(),
                            Ptr::<u32>::null(),
                        )
                    })
                },
                {
                    (({
                        MultiStringTypeImpl::length_11(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<i32>::from_string_literal(&[
                                        97 as i32, 98 as i32, 99 as i32, 0 as i32,
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
                                        97 as u8, 98 as u8, 99 as u8, 0 as u8,
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
                                        97 as u16, 98 as u16, 99 as u16, 0 as u16,
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
                                        97 as u32, 98 as u32, 99 as u32, 0 as u32,
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[u32; 4]>() as usize)
                                        .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                        as usize)
                                        .wrapping_sub(1_usize)
                                },
                                { Ptr::<i8>::from_string_literal(b"abc") },
                                {
                                    (::std::mem::size_of::<[i8; 4]>() as usize)
                                        .wrapping_sub(1_usize)
                                },
                            )))
                            .as_pointer(),
                            Ptr::<u32>::null(),
                        )
                    }) as u64)
                },
            ),
        ));
    assert!(({ (({ (*c.borrow()).begin() }) == ({ begin_33(c.as_pointer(),) })) }));
    assert!(({ (*c.borrow()).rbegin() }) == ({ rbegin_34(c.as_pointer(),) }));
    assert!(({ (({ (*c.borrow()).cbegin() }) == ({ cbegin_35(c.as_pointer(),) })) }));
    assert!(({ (*c.borrow()).crbegin() }) == ({ crbegin_36(c.as_pointer(),) }));
    assert!(({ (({ (*c.borrow()).end() }) == ({ libcc2rs::end_refcount(c.as_pointer(),) })) }));
    assert!(({ (*c.borrow()).rend() }) == ({ rend_37(c.as_pointer(),) }));
    assert!(({ (({ (*c.borrow()).cend() }) == ({ cend_38(c.as_pointer(),) })) }));
    assert!(({ (*c.borrow()).crend() }) == ({ crend_39(c.as_pointer(),) }));
    assert!((({ begin_33(c.as_pointer(),) }) != ({ libcc2rs::end_refcount(c.as_pointer(),) })));
    assert!(({ rbegin_34(c.as_pointer(),) }) != ({ rend_37(c.as_pointer(),) }));
    assert!((({ cbegin_35(c.as_pointer(),) }) != ({ cend_38(c.as_pointer(),) })));
    assert!(({ crbegin_36(c.as_pointer(),) }) != ({ crend_39(c.as_pointer(),) }));
    let r1: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(&(ii1), &(ii2))));
    assert!(({ (*r1.borrow()) }) == ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }));
    let r2: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(&(ii1), &(ii2))));
    assert!(({ (*r2.borrow()) }) == ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }));
}
pub fn test_40() -> bool {
    ({ test_type_0() });
    ({ test_type_8() });
    ({ test_type_16() });
    ({ test_type_24() });
    ({ test_type_32() });
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
    ({ test_40() });
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
