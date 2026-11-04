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
pub fn test_0() {
    let empty: Value<Vec<i8>> = Rc::new(RefCell::new(vec![i8::default()]));
    let str: Value<Vec<i8>> = Rc::new(RefCell::new(
        match (
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
                ({
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
                })
            },
        ) {
            (mut __tmp_0, mut __tmp_1) => {
                let mut __v = Vec::with_capacity(__tmp_1 as usize + 1);
                __tmp_0.with_slice(__tmp_1 as usize, |__s| __v.extend_from_slice(__s));
                __v.push(0_i8);
                __v
            }
        },
    ));
    ({ test_1(true) });
    ({ test_1(false) });
    ({ test_2(('a' as i8)) });
    ({ test_2(('z' as i8)) });
    ({ test_2(('0' as i8)) });
    ({ test_2(('9' as i8)) });
    {
        ({ test_2(('a' as i8)) });
        ({ test_2(('z' as i8)) });
        ({ test_2(('0' as i8)) });
        ({ test_2(('9' as i8)) });
    }
    ({ test_3(i8::MIN) });
    ({ test_3(0_i8) });
    ({ test_3(i8::MAX) });
    ({ test_4(i16::MIN) });
    ({ test_4((i8::MIN as i16)) });
    ({ test_4(0_i16) });
    ({ test_4((i8::MAX as i16)) });
    ({ test_4(i16::MAX) });
    ({ test_5(i32::MIN) });
    ({ test_5((i16::MIN as i32)) });
    ({ test_5((i8::MIN as i32)) });
    ({ test_5(0) });
    ({ test_5((i8::MAX as i32)) });
    ({ test_5((i16::MAX as i32)) });
    ({ test_5(i32::MAX) });
    ({ test_6(i64::MIN) });
    ({ test_6((i32::MIN as i64)) });
    ({ test_6((i16::MIN as i64)) });
    ({ test_6((i8::MIN as i64)) });
    ({ test_6(0_i64) });
    ({ test_6((i8::MAX as i64)) });
    ({ test_6((i16::MAX as i64)) });
    ({ test_6((i32::MAX as i64)) });
    ({ test_6(i64::MAX) });
    ({ test_7(i64::MIN) });
    ({ test_7(i64::MIN) });
    ({ test_7((i32::MIN as i64)) });
    ({ test_7((i16::MIN as i64)) });
    ({ test_7((i8::MIN as i64)) });
    ({ test_7(0_i64) });
    ({ test_7((i8::MAX as i64)) });
    ({ test_7((i16::MAX as i64)) });
    ({ test_7((i32::MAX as i64)) });
    ({ test_7(i64::MAX) });
    ({ test_7(i64::MAX) });
    ({ test_handle_8(0_i128) });
    ({ test_9(0_u8) });
    ({ test_9(u8::MAX) });
    ({ test_10(0_u16) });
    ({ test_10((u8::MAX as u16)) });
    ({ test_10(u16::MAX) });
    ({ test_11(0_u32) });
    ({ test_11((u8::MAX as u32)) });
    ({ test_11((u16::MAX as u32)) });
    ({ test_11(u32::MAX) });
    ({ test_12(0_u64) });
    ({ test_12((u8::MAX as u64)) });
    ({ test_12((u16::MAX as u64)) });
    ({ test_12((u32::MAX as u64)) });
    ({ test_12(u64::MAX) });
    ({ test_13(0_u64) });
    ({ test_13((u8::MAX as u64)) });
    ({ test_13((u16::MAX as u64)) });
    ({ test_13((u32::MAX as u64)) });
    ({ test_13(u64::MAX) });
    ({ test_13(({ std_numeric_limits_unsigned_long_long_::max() })) });
    ({ test_handle_14(0_u128) });
    ({ test_15(-f32::MAX) });
    ({ test_15(-f32::MIN_POSITIVE) });
    ({ test_15((-0_f64 as f32)) });
    ({ test_15((0_f64 as f32)) });
    ({ test_15(f32::MIN_POSITIVE) });
    ({ test_15(f32::MAX) });
    ({ test_16(-f64::MAX) });
    ({ test_16(-f64::MIN_POSITIVE) });
    ({ test_16(-0_f64) });
    ({ test_16(0_f64) });
    ({ test_16(f64::MIN_POSITIVE) });
    ({ test_16(f64::MAX) });
    ({ test_17(-f64::MAX) });
    ({ test_17(-f64::MIN_POSITIVE) });
    ({ test_17(-0_f64) });
    ({ test_17(0_f64) });
    ({ test_17(f64::MIN_POSITIVE) });
    ({ test_17(f64::MAX) });
    ({ test_18(empty.decay()) });
    ({ test_18(str.decay()) });
    ({ test_19(<std_basic_string_view_char__std_char_traits_char__>::default()) });
    ({ test_19(({ (*empty.borrow())() })) });
    ({ test_19(({ (*str.borrow())() })) });
    ({ test_20(vec![i8::default()]) });
    ({ test_20((*empty.borrow()).clone()) });
    ({ test_20((*str.borrow()).clone()) });
    ({ test_21(Default::default()) });
}
pub fn test_22() {
    let empty: Value<Vec<i32>> = Rc::new(RefCell::new(vec![i32::default()]));
    let str: Value<Vec<i32>> = Rc::new(RefCell::new(
        match (
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
                ({
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
                })
            },
        ) {
            (mut __tmp_2, mut __tmp_3) => {
                let mut __v = Vec::with_capacity(__tmp_3 as usize + 1);
                __tmp_2.with_slice(__tmp_3 as usize, |__s| __v.extend_from_slice(__s));
                __v.push(0_i32);
                __v
            }
        },
    ));
    ({ test_23(true) });
    ({ test_23(false) });
    ({ test_24((('a' as i8) as i32)) });
    ({ test_24((('z' as i8) as i32)) });
    ({ test_24((('0' as i8) as i32)) });
    ({ test_24((('9' as i8) as i32)) });
    {
        {
            ({ test_25(('a' as i8)) });
            ({ test_25(('z' as i8)) });
            ({ test_25(('0' as i8)) });
            ({ test_25(('9' as i8)) });
        }
    }
    ({ test_26(i8::MIN) });
    ({ test_26(0_i8) });
    ({ test_26(i8::MAX) });
    ({ test_27(i16::MIN) });
    ({ test_27((i8::MIN as i16)) });
    ({ test_27(0_i16) });
    ({ test_27((i8::MAX as i16)) });
    ({ test_27(i16::MAX) });
    ({ test_28(i32::MIN) });
    ({ test_28((i16::MIN as i32)) });
    ({ test_28((i8::MIN as i32)) });
    ({ test_28(0) });
    ({ test_28((i8::MAX as i32)) });
    ({ test_28((i16::MAX as i32)) });
    ({ test_28(i32::MAX) });
    ({ test_29(i64::MIN) });
    ({ test_29((i32::MIN as i64)) });
    ({ test_29((i16::MIN as i64)) });
    ({ test_29((i8::MIN as i64)) });
    ({ test_29(0_i64) });
    ({ test_29((i8::MAX as i64)) });
    ({ test_29((i16::MAX as i64)) });
    ({ test_29((i32::MAX as i64)) });
    ({ test_29(i64::MAX) });
    ({ test_30(i64::MIN) });
    ({ test_30(i64::MIN) });
    ({ test_30((i32::MIN as i64)) });
    ({ test_30((i16::MIN as i64)) });
    ({ test_30((i8::MIN as i64)) });
    ({ test_30(0_i64) });
    ({ test_30((i8::MAX as i64)) });
    ({ test_30((i16::MAX as i64)) });
    ({ test_30((i32::MAX as i64)) });
    ({ test_30(i64::MAX) });
    ({ test_30(i64::MAX) });
    ({ test_handle_31(0_i128) });
    ({ test_32(0_u8) });
    ({ test_32(u8::MAX) });
    ({ test_33(0_u16) });
    ({ test_33((u8::MAX as u16)) });
    ({ test_33(u16::MAX) });
    ({ test_34(0_u32) });
    ({ test_34((u8::MAX as u32)) });
    ({ test_34((u16::MAX as u32)) });
    ({ test_34(u32::MAX) });
    ({ test_35(0_u64) });
    ({ test_35((u8::MAX as u64)) });
    ({ test_35((u16::MAX as u64)) });
    ({ test_35((u32::MAX as u64)) });
    ({ test_35(u64::MAX) });
    ({ test_36(0_u64) });
    ({ test_36((u8::MAX as u64)) });
    ({ test_36((u16::MAX as u64)) });
    ({ test_36((u32::MAX as u64)) });
    ({ test_36(u64::MAX) });
    ({ test_36(({ std_numeric_limits_unsigned_long_long_::max() })) });
    ({ test_handle_37(0_u128) });
    ({ test_38(-f32::MAX) });
    ({ test_38(-f32::MIN_POSITIVE) });
    ({ test_38((-0_f64 as f32)) });
    ({ test_38((0_f64 as f32)) });
    ({ test_38(f32::MIN_POSITIVE) });
    ({ test_38(f32::MAX) });
    ({ test_39(-f64::MAX) });
    ({ test_39(-f64::MIN_POSITIVE) });
    ({ test_39(-0_f64) });
    ({ test_39(0_f64) });
    ({ test_39(f64::MIN_POSITIVE) });
    ({ test_39(f64::MAX) });
    ({ test_40(-f64::MAX) });
    ({ test_40(-f64::MIN_POSITIVE) });
    ({ test_40(-0_f64) });
    ({ test_40(0_f64) });
    ({ test_40(f64::MIN_POSITIVE) });
    ({ test_40(f64::MAX) });
    ({ test_41(empty.decay()) });
    ({ test_41(str.decay()) });
    ({ test_42(<std_basic_string_view_wchar_t__std_char_traits_wchar_t__>::default()) });
    ({ test_42(({ (*empty.borrow())() })) });
    ({ test_42(({ (*str.borrow())() })) });
    ({ test_43(vec![i32::default()]) });
    ({ test_43((*empty.borrow()).clone()) });
    ({ test_43((*str.borrow()).clone()) });
    ({ test_44(Default::default()) });
}
pub fn test_45() {
    ({ test_0() });
    ({ test_22() });
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
    ({ test_45() });
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
