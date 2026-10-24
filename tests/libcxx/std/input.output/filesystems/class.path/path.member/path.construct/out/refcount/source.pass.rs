extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(80)]
pub struct MultiStringType {
    #[offset(0)]
    #[byte_size(8)]
    pub w_: Ptr<()>,
    #[offset(8)]
    pub wn_: (),
    #[offset(16)]
    #[byte_size(8)]
    pub u8_: Ptr<()>,
    #[offset(24)]
    pub u8n_: (),
    #[offset(32)]
    #[byte_size(8)]
    pub u16_: Ptr<()>,
    #[offset(40)]
    pub u16n_: (),
    #[offset(48)]
    #[byte_size(8)]
    pub u32_: Ptr<()>,
    #[offset(56)]
    pub u32n_: (),
    #[offset(64)]
    #[byte_size(8)]
    pub s_: Ptr<()>,
    #[offset(72)]
    pub sn_: (),
}
impl MultiStringType {
    pub fn new(
        mut w: Ptr<()>,
        mut wn: (),
        mut u8: Ptr<()>,
        mut u8n: (),
        mut u16: Ptr<()>,
        mut u16n: (),
        mut u32: Ptr<()>,
        mut u32n: (),
        mut s: Ptr<()>,
        mut sn: (),
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
pub fn PathReserve_0(p: Ptr<()>, mut N: ()) {
    let native_ref: Ptr<()> = unimplemented!();
    unimplemented!();
}
pub fn PathEq_1(LHS: Ptr<()>, RHS: Ptr<()>) -> () {
    return unimplemented!();
}
pub fn PathEqIgnoreSep_2(LHS: (), RHS: ()) -> () {
    let LHS: Value<()> = Rc::new(RefCell::new(LHS));
    let RHS: Value<()> = Rc::new(RefCell::new(RHS));
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn IterEnd_3(B: Ptr<()>) -> Ptr<()> {
    let B: Value<Ptr<()>> = Rc::new(RefCell::new(B));
    'loop_: while (((*B.borrow()).read()) != unimplemented!()) {
        (*B.borrow_mut()).prefix_inc();
    }
    return (*B.borrow()).clone();
}
pub fn IterEnd_4(B: Ptr<()>) -> Ptr<()> {
    let B: Value<Ptr<()>> = Rc::new(RefCell::new(B));
    'loop_: while (((*B.borrow()).read()) != unimplemented!()) {
        (*B.borrow_mut()).prefix_inc();
    }
    return (*B.borrow()).clone();
}
pub fn IterEnd_5(B: Ptr<()>) -> Ptr<()> {
    let B: Value<Ptr<()>> = Rc::new(RefCell::new(B));
    'loop_: while (((*B.borrow()).read()) != unimplemented!()) {
        (*B.borrow_mut()).prefix_inc();
    }
    return (*B.borrow()).clone();
}
pub fn IterEnd_6(B: Ptr<()>) -> Ptr<()> {
    let B: Value<Ptr<()>> = Rc::new(RefCell::new(B));
    'loop_: while (((*B.borrow()).read()) != unimplemented!()) {
        (*B.borrow_mut()).prefix_inc();
    }
    return (*B.borrow()).clone();
}
pub fn IterEnd_7(B: Ptr<()>) -> Ptr<()> {
    let B: Value<Ptr<()>> = Rc::new(RefCell::new(B));
    'loop_: while (((*B.borrow()).read()) != unimplemented!()) {
        (*B.borrow_mut()).prefix_inc();
    }
    return (*B.borrow()).clone();
}
pub fn StrEnd_8(mut P: Ptr<()>) -> Ptr<()> {
    return ({ IterEnd_3(P) });
}
pub fn StrEnd_9(mut P: Ptr<()>) -> Ptr<()> {
    return ({ IterEnd_4(P) });
}
pub fn StrEnd_10(mut P: Ptr<()>) -> Ptr<()> {
    return ({ IterEnd_5(P) });
}
pub fn StrEnd_11(mut P: Ptr<()>) -> Ptr<()> {
    return ({ IterEnd_6(P) });
}
pub fn StrEnd_12(mut P: Ptr<()>) -> Ptr<()> {
    return ({ IterEnd_7(P) });
}
thread_local!(
    pub static PathList_13: Value<Box<[MultiStringType]>> = Rc::new(RefCell::new(Box::new([
        MultiStringType::new(
            { Ptr::<()>::from_string_literal(&[0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 1]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 1]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 1]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 1]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"") },
            { (::std::mem::size_of::<[(); 1]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            { Ptr::<()>::from_string_literal(&[32 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 2]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[32 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 2]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[32 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 2]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[32 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 2]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b" ") },
            { (::std::mem::size_of::<[(); 2]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            { Ptr::<()>::from_string_literal(&[47 as (), 47 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 3]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[47 as (), 47 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 3]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[47 as (), 47 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 3]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[47 as (), 47 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 3]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"//") },
            { (::std::mem::size_of::<[(); 3]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            { Ptr::<()>::from_string_literal(&[46 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 2]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[46 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 2]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[46 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 2]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[46 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 2]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b".") },
            { (::std::mem::size_of::<[(); 2]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            { Ptr::<()>::from_string_literal(&[46 as (), 46 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 3]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[46 as (), 46 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 3]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[46 as (), 46 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 3]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[46 as (), 46 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 3]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"..") },
            { (::std::mem::size_of::<[(); 3]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            { Ptr::<()>::from_string_literal(&[102 as (), 111 as (), 111 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 4]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[102 as (), 111 as (), 111 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 4]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[102 as (), 111 as (), 111 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 4]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[102 as (), 111 as (), 111 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 4]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"foo") },
            { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            { Ptr::<()>::from_string_literal(&[47 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 2]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[47 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 2]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[47 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 2]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[47 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 2]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"/") },
            { (::std::mem::size_of::<[(); 2]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 5]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 5]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 5]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 5]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"/foo") },
            { (::std::mem::size_of::<[(); 5]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 5]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 5]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 5]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 5]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"foo/") },
            { (::std::mem::size_of::<[(); 5]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 102 as (), 111 as (), 111 as (), 47 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 6]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 102 as (), 111 as (), 111 as (), 47 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 6]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 102 as (), 111 as (), 111 as (), 47 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 6]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 102 as (), 111 as (), 111 as (), 47 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 6]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"/foo/") },
            { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 98 as (), 97 as (), 114 as (),
                    0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 8]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 98 as (), 97 as (), 114 as (),
                    0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 8]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 98 as (), 97 as (), 114 as (),
                    0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 8]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 98 as (), 97 as (), 114 as (),
                    0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 8]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"foo/bar") },
            { (::std::mem::size_of::<[(); 8]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 102 as (), 111 as (), 111 as (), 47 as (), 98 as (), 97 as (),
                    114 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 9]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 102 as (), 111 as (), 111 as (), 47 as (), 98 as (), 97 as (),
                    114 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 9]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 102 as (), 111 as (), 111 as (), 47 as (), 98 as (), 97 as (),
                    114 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 9]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 102 as (), 111 as (), 111 as (), 47 as (), 98 as (), 97 as (),
                    114 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 9]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"/foo/bar") },
            { (::std::mem::size_of::<[(); 9]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 47 as (), 110 as (), 101 as (), 116 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 6]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 47 as (), 110 as (), 101 as (), 116 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 6]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 47 as (), 110 as (), 101 as (), 116 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 6]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 47 as (), 110 as (), 101 as (), 116 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 6]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"//net") },
            { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 47 as (), 110 as (), 101 as (), 116 as (), 47 as (), 102 as (),
                    111 as (), 111 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 10]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 47 as (), 110 as (), 101 as (), 116 as (), 47 as (), 102 as (),
                    111 as (), 111 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 10]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 47 as (), 110 as (), 101 as (), 116 as (), 47 as (), 102 as (),
                    111 as (), 111 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 10]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 47 as (), 110 as (), 101 as (), 116 as (), 47 as (), 102 as (),
                    111 as (), 111 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 10]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"//net/foo") },
            { (::std::mem::size_of::<[(); 10]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 47 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                    47 as (), 47 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 10]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 47 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                    47 as (), 47 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 10]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 47 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                    47 as (), 47 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 10]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 47 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                    47 as (), 47 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 10]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"///foo///") },
            { (::std::mem::size_of::<[(); 10]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 47 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                    47 as (), 47 as (), 98 as (), 97 as (), 114 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 13]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 47 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                    47 as (), 47 as (), 98 as (), 97 as (), 114 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 13]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 47 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                    47 as (), 47 as (), 98 as (), 97 as (), 114 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 13]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 47 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                    47 as (), 47 as (), 98 as (), 97 as (), 114 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 13]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"///foo///bar") },
            { (::std::mem::size_of::<[(); 13]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            { Ptr::<()>::from_string_literal(&[47 as (), 46 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 3]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[47 as (), 46 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 3]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[47 as (), 46 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 3]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[47 as (), 46 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 3]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"/.") },
            { (::std::mem::size_of::<[(); 3]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            { Ptr::<()>::from_string_literal(&[46 as (), 47 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 3]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[46 as (), 47 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 3]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[46 as (), 47 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 3]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[46 as (), 47 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 3]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"./") },
            { (::std::mem::size_of::<[(); 3]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            { Ptr::<()>::from_string_literal(&[47 as (), 46 as (), 46 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 4]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[47 as (), 46 as (), 46 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 4]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[47 as (), 46 as (), 46 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 4]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[47 as (), 46 as (), 46 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 4]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"/..") },
            { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            { Ptr::<()>::from_string_literal(&[46 as (), 46 as (), 47 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 4]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[46 as (), 46 as (), 47 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 4]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[46 as (), 46 as (), 47 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 4]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[46 as (), 46 as (), 47 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 4]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"../") },
            { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 6]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 6]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 6]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 6]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"foo/.") },
            { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 7]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 7]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 7]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 7]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"foo/..") },
            { (::std::mem::size_of::<[(); 7]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 47 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 7]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 47 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 7]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 47 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 7]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 47 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 7]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"foo/./") },
            { (::std::mem::size_of::<[(); 7]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 47 as (), 98 as (),
                    97 as (), 114 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 10]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 47 as (), 98 as (),
                    97 as (), 114 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 10]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 47 as (), 98 as (),
                    97 as (), 114 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 10]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 47 as (), 98 as (),
                    97 as (), 114 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 10]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"foo/./bar") },
            { (::std::mem::size_of::<[(); 10]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 47 as (),
                    0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 8]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 47 as (),
                    0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 8]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 47 as (),
                    0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 8]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 47 as (),
                    0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 8]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"foo/../") },
            { (::std::mem::size_of::<[(); 8]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 47 as (),
                    98 as (), 97 as (), 114 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 11]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 47 as (),
                    98 as (), 97 as (), 114 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 11]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 47 as (),
                    98 as (), 97 as (), 114 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 11]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 47 as (),
                    98 as (), 97 as (), 114 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 11]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"foo/../bar") },
            { (::std::mem::size_of::<[(); 11]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            { Ptr::<()>::from_string_literal(&[99 as (), 58 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 3]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[99 as (), 58 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 3]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[99 as (), 58 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 3]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[99 as (), 58 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 3]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"c:") },
            { (::std::mem::size_of::<[(); 3]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            { Ptr::<()>::from_string_literal(&[99 as (), 58 as (), 47 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 4]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[99 as (), 58 as (), 47 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 4]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[99 as (), 58 as (), 47 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 4]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[99 as (), 58 as (), 47 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 4]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"c:/") },
            { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 6]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 6]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 6]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 6]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"c:foo") },
            { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 7]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 7]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 7]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 7]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"c:/foo") },
            { (::std::mem::size_of::<[(); 7]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 47 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 7]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 47 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 7]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 47 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 7]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 47 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 7]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"c:foo/") },
            { (::std::mem::size_of::<[(); 7]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                    0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 8]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                    0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 8]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                    0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 8]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                    0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 8]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"c:/foo/") },
            { (::std::mem::size_of::<[(); 8]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                    98 as (), 97 as (), 114 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 11]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                    98 as (), 97 as (), 114 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 11]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                    98 as (), 97 as (), 114 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 11]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                    98 as (), 97 as (), 114 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 11]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"c:/foo/bar") },
            { (::std::mem::size_of::<[(); 11]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            {
                Ptr::<()>::from_string_literal(&[
                    112 as (), 114 as (), 110 as (), 58 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 5]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    112 as (), 114 as (), 110 as (), 58 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 5]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    112 as (), 114 as (), 110 as (), 58 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 5]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    112 as (), 114 as (), 110 as (), 58 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 5]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"prn:") },
            { (::std::mem::size_of::<[(); 5]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            { Ptr::<()>::from_string_literal(&[99 as (), 58 as (), 92 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 4]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[99 as (), 58 as (), 92 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 4]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[99 as (), 58 as (), 92 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 4]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[99 as (), 58 as (), 92 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 4]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"c:\\") },
            { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 7]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 7]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 7]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 7]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"c:\\foo") },
            { (::std::mem::size_of::<[(); 7]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 92 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 7]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 92 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 7]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 92 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 7]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 92 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 7]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"c:foo\\") },
            { (::std::mem::size_of::<[(); 7]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 92 as (),
                    0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 8]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 92 as (),
                    0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 8]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 92 as (),
                    0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 8]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 92 as (),
                    0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 8]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"c:\\foo\\") },
            { (::std::mem::size_of::<[(); 8]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                    0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 8]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                    0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 8]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                    0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 8]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                    0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 8]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"c:\\foo/") },
            { (::std::mem::size_of::<[(); 8]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 92 as (),
                    98 as (), 97 as (), 114 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 11]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 92 as (),
                    98 as (), 97 as (), 114 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 11]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 92 as (),
                    98 as (), 97 as (), 114 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 11]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 92 as (),
                    98 as (), 97 as (), 114 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 11]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"c:/foo\\bar") },
            { (::std::mem::size_of::<[(); 11]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            { Ptr::<()>::from_string_literal(&[47 as (), 47 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 3]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[47 as (), 47 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 3]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[47 as (), 47 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 3]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(&[47 as (), 47 as (), 0 as ()]) },
            {
                ((::std::mem::size_of::<[(); 3]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { Ptr::<()>::from_string_literal(b"//") },
            { (::std::mem::size_of::<[(); 3]>() as ()).wrapping_sub(1_()) },
        ),
        MultiStringType::new(
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 102 as (), 105 as (), 110 as (), 97 as (), 108 as (), 108 as (),
                    121 as (), 47 as (), 119 as (), 101 as (), 47 as (), 110 as (), 101 as (),
                    101 as (), 100 as (), 47 as (), 111 as (), 110 as (), 101 as (), 47 as (),
                    114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                    114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                    114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                    114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                    114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                    114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                    114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                    108 as (), 111 as (), 110 as (), 103 as (), 47 as (), 115 as (), 116 as (),
                    114 as (), 105 as (), 110 as (), 103 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 82]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 102 as (), 105 as (), 110 as (), 97 as (), 108 as (), 108 as (),
                    121 as (), 47 as (), 119 as (), 101 as (), 47 as (), 110 as (), 101 as (),
                    101 as (), 100 as (), 47 as (), 111 as (), 110 as (), 101 as (), 47 as (),
                    114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                    114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                    114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                    114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                    114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                    114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                    114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                    108 as (), 111 as (), 110 as (), 103 as (), 47 as (), 115 as (), 116 as (),
                    114 as (), 105 as (), 110 as (), 103 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 82]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 102 as (), 105 as (), 110 as (), 97 as (), 108 as (), 108 as (),
                    121 as (), 47 as (), 119 as (), 101 as (), 47 as (), 110 as (), 101 as (),
                    101 as (), 100 as (), 47 as (), 111 as (), 110 as (), 101 as (), 47 as (),
                    114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                    114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                    114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                    114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                    114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                    114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                    114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                    108 as (), 111 as (), 110 as (), 103 as (), 47 as (), 115 as (), 116 as (),
                    114 as (), 105 as (), 110 as (), 103 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 82]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<()>::from_string_literal(&[
                    47 as (), 102 as (), 105 as (), 110 as (), 97 as (), 108 as (), 108 as (),
                    121 as (), 47 as (), 119 as (), 101 as (), 47 as (), 110 as (), 101 as (),
                    101 as (), 100 as (), 47 as (), 111 as (), 110 as (), 101 as (), 47 as (),
                    114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                    114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                    114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                    114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                    114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                    114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                    114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                    108 as (), 111 as (), 110 as (), 103 as (), 47 as (), 115 as (), 116 as (),
                    114 as (), 105 as (), 110 as (), 103 as (), 0 as (),
                ])
            },
            {
                ((::std::mem::size_of::<[(); 82]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                Ptr::<() >::from_string_literal(b"/finally/we/need/one/really/really/really/really/really/really/really/long/string" )
            },
            { (::std::mem::size_of::<[(); 82]>() as ()).wrapping_sub(1_()) },
        ),
    ])));
);
thread_local!(
    pub static PathListSize_14: Value<()> = Rc::new(RefCell::new(
        (3360usize as ()).wrapping_div((80usize as ())),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_16.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_17.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_18.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_15.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_16.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_17.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_15: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_16: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_17: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_18: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(1)]
pub struct support_double_move_tracker {
    #[offset(0)]
    moved_from_: (),
}
impl support_double_move_tracker {
    pub fn new() -> Self {
        Self { moved_from_: false }
    }
    pub fn move_from(other: Ptr<support_double_move_tracker>) -> Self {
        let __this: support_double_move_tracker = Self { moved_from_: false };
        unimplemented!();
        field!(other, moved_from_).write(true);
        __this
    }
}
impl Default for support_double_move_tracker {
    fn default() -> Self {
        { support_double_move_tracker::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct cpp17_output_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_output_iterator_int_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct cpp17_output_iterator_const_char_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_output_iterator_const_char_ptr_ {}
const _: () = assert!(
    true,
    "std::output_iterator<cpp17_output_iterator<int*>, int>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {}
pub fn operator_ne_19(
    x: Ptr<cpp17_input_iterator_const_char_ptr__const_char_ptr_>,
    y: Ptr<cpp17_input_iterator_const_char_ptr__const_char_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(16)]
pub struct cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
    pub fn new(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<cpp17_input_iterator_const_char_ptr__const_char_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct cpp17_input_iterator_const_char_ptr__Traits_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_char_ptr__Traits_ {}
pub fn operator_eq_20(
    x: Ptr<cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_>,
    y: Ptr<cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_21(
    x: Ptr<cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_>,
    y: Ptr<cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(16)]
pub struct cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_ {
    pub fn new(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl std::cmp::PartialEq for cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_20(
                Rc::new(RefCell::new(
                    cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_ {
                        it_: self.it_.clone(),
                        tracker_: support_double_move_tracker {
                            moved_from_: self.tracker_.moved_from_.clone(),
                        },
                    },
                ))
                .as_pointer(),
                Rc::new(RefCell::new(
                    cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_ {
                        it_: other.it_.clone(),
                        tracker_: support_double_move_tracker {
                            moved_from_: other.tracker_.moved_from_.clone(),
                        },
                    },
                ))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_ {}
pub fn operator_eq_22(
    x: Ptr<cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_>,
    y: Ptr<cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_23(
    x: Ptr<cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_>,
    y: Ptr<cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(16)]
pub struct cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_ {
    pub fn new(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl std::cmp::PartialEq for cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_22(
                Rc::new(RefCell::new(
                    cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_ {
                        it_: self.it_.clone(),
                        tracker_: support_double_move_tracker {
                            moved_from_: self.tracker_.moved_from_.clone(),
                        },
                    },
                ))
                .as_pointer(),
                Rc::new(RefCell::new(
                    cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_ {
                        it_: other.it_.clone(),
                        tracker_: support_double_move_tracker {
                            moved_from_: other.tracker_.moved_from_.clone(),
                        },
                    },
                ))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_ {}
pub fn operator_eq_24(
    x: Ptr<cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_>,
    y: Ptr<cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_25(
    x: Ptr<cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_>,
    y: Ptr<cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(16)]
pub struct cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_ {
    pub fn new(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(
        _a0: Ptr<cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_>,
    ) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl std::cmp::PartialEq for cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_24(
                Rc::new(RefCell::new(
                    cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_ {
                        it_: self.it_.clone(),
                        tracker_: support_double_move_tracker {
                            moved_from_: self.tracker_.moved_from_.clone(),
                        },
                    },
                ))
                .as_pointer(),
                Rc::new(RefCell::new(
                    cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_ {
                        it_: other.it_.clone(),
                        tracker_: support_double_move_tracker {
                            moved_from_: other.tracker_.moved_from_.clone(),
                        },
                    },
                ))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_ {}
pub fn operator_eq_26(
    x: Ptr<cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_>,
    y: Ptr<cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_27(
    x: Ptr<cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_>,
    y: Ptr<cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(16)]
pub struct cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_ {
    pub fn new(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(
        _a0: Ptr<cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_>,
    ) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl std::cmp::PartialEq for cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_26(
                Rc::new(RefCell::new(
                    cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_ {
                        it_: self.it_.clone(),
                        tracker_: support_double_move_tracker {
                            moved_from_: self.tracker_.moved_from_.clone(),
                        },
                    },
                ))
                .as_pointer(),
                Rc::new(RefCell::new(
                    cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_ {
                        it_: other.it_.clone(),
                        tracker_: support_double_move_tracker {
                            moved_from_: other.tracker_.moved_from_.clone(),
                        },
                    },
                ))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_ {}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct cpp20_random_access_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp20_random_access_iterator_int_ptr_ {}
const _: () = assert!(
    true,
    "std::random_access_iterator<cpp20_random_access_iterator<int*>>"
);
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp20_input_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_int_ptr_ {
    pub fn move_from(_a0: Ptr<cpp20_input_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
const _: () = assert!(true, "std::input_iterator<cpp20_input_iterator<int*>>");
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp20_output_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp20_output_iterator_int_ptr_ {
    pub fn move_from(_a0: Ptr<cpp20_output_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
const _: () = assert!(
    true,
    "std::output_iterator<cpp20_output_iterator<int*>, int>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct IteratorOpCounts {
    #[offset(0)]
    pub increments: (),
    #[offset(8)]
    pub decrements: (),
    #[offset(16)]
    pub zero_moves: (),
    #[offset(24)]
    pub equal_cmps: (),
}
impl Default for IteratorOpCounts {
    fn default() -> Self {
        {
            IteratorOpCounts {
                increments: 0_(),
                decrements: 0_(),
                zero_moves: 0_(),
                equal_cmps: 0_(),
            }
        }
    }
}
pub fn operator_eq_28(
    _a0: Ptr<forward_sized_iterator_int_ptr_>,
    _a1: Ptr<forward_sized_iterator_int_ptr_>,
) -> () {
    return ({ _a0.with(|__s| __s.it_.clone()) } == { _a1.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct forward_sized_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    pub it_: Ptr<()>,
}
impl forward_sized_iterator_int_ptr_ {}
impl std::cmp::PartialEq for forward_sized_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_28(
                Rc::new(RefCell::new(forward_sized_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(forward_sized_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for forward_sized_iterator_int_ptr_ {}
impl Default for forward_sized_iterator_int_ptr_ {
    fn default() -> Self {
        {
            forward_sized_iterator_int_ptr_ {
                it_: Ptr::<()>::null(),
            }
        }
    }
}
const _: () = assert!(true, "std::forward_iterator<forward_sized_iterator<>>");
const _: () = assert!(
    true,
    "std::sized_sentinel_for<forward_sized_iterator<>, forward_sized_iterator<>>"
);
pub fn operator_add_29(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_30(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_29(_i, n)
    });
}
pub fn iter_swap_31(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_32(iter: adl_Iterator) -> Ptr<()> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return (unimplemented!()).reinterpret_cast::<()>();
}
pub fn operator_eq_33(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_34(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
    return std::cmp::Ord::cmp(
        &(lhs.with(|__s| __s.ptr_.clone())),
        &(rhs.with(|__s| __s.ptr_.clone())),
    );
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct adl_Iterator {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    iter_moves_: Ptr<()>,
    #[offset(16)]
    #[byte_size(8)]
    iter_swaps_: Ptr<()>,
}
impl adl_Iterator {
    fn new_1(mut p: Ptr<()>, mut iter_moves: Ptr<()>, mut iter_swaps: Ptr<()>) -> Self {
        Self {
            ptr_: (p).clone(),
            iter_moves_: (iter_moves).clone(),
            iter_swaps_: (iter_swaps).clone(),
        }
    }
    pub fn TrackMoves(mut p: Ptr<()>, iter_moves: Ptr<()>) -> adl_Iterator {
        return adl_Iterator::new_1({ (p).clone() }, { (iter_moves).clone() }, {
            Ptr::<()>::null()
        });
    }
    pub fn TrackSwaps_4(iter_swaps: Ptr<()>) -> adl_Iterator {
        return adl_Iterator::new_1({ Ptr::<()>::null() }, { Ptr::<()>::null() }, {
            (iter_swaps).clone()
        });
    }
    pub fn TrackSwaps_5(mut p: Ptr<()>, iter_swaps: Ptr<()>) -> adl_Iterator {
        return adl_Iterator::new_1({ (p).clone() }, { Ptr::<()>::null() }, {
            (iter_swaps).clone()
        });
    }
}
impl std::cmp::Ord for adl_Iterator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_34(
                Rc::new(RefCell::new(adl_Iterator {
                    ptr_: self.ptr_.clone(),
                    iter_moves_: self.iter_moves_.clone(),
                    iter_swaps_: self.iter_swaps_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(adl_Iterator {
                    ptr_: other.ptr_.clone(),
                    iter_moves_: other.iter_moves_.clone(),
                    iter_swaps_: other.iter_swaps_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_33(
                Rc::new(RefCell::new(adl_Iterator {
                    ptr_: self.ptr_.clone(),
                    iter_moves_: self.iter_moves_.clone(),
                    iter_swaps_: self.iter_swaps_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(adl_Iterator {
                    ptr_: other.ptr_.clone(),
                    iter_moves_: other.iter_moves_.clone(),
                    iter_swaps_: other.iter_swaps_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for adl_Iterator {}
impl Default for adl_Iterator {
    fn default() -> Self {
        {
            adl_Iterator {
                ptr_: Ptr::<()>::null(),
                iter_moves_: Ptr::<()>::null(),
                iter_swaps_: Ptr::<()>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct rvalue_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<Ptr<()>>,
}
impl rvalue_iterator_int_ptr_ {}
impl std::cmp::Ord for rvalue_iterator_int_ptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            rvalue_iterator_int_ptr_Impl::operator_cmp(
                &Rc::new(RefCell::new(rvalue_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(rvalue_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for rvalue_iterator_int_ptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for rvalue_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            rvalue_iterator_int_ptr_Impl::operator_cmp(
                &Rc::new(RefCell::new(rvalue_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(rvalue_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                }))
                .as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for rvalue_iterator_int_ptr_ {}
const _: () = assert!(true, "std::random_access_iterator<rvalue_iterator<int*>>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct util_iterator_error {}
impl util_iterator_error {
    pub fn new(mut what: Ptr<()>) -> Self {
        Self {}
    }
}
pub fn RunTestCaseImpl_35(MS: Ptr<MultiStringType>) {
    let mut Expect: Ptr<()> = ({ MultiStringTypeImpl::to_pconst_12(&MS) });
    let mut TestPath: Ptr<()> = ({ MultiStringTypeImpl::to_pconst_12(&MS) });
    let mut TestPathEnd: Ptr<()> = ({ StrEnd_8((TestPath).clone()) });
    let mut Size: () = ((TestPathEnd).clone() - (TestPath).clone()) as ();
    let mut SSize: () = (({ StrEnd_8((Expect).clone()) }) - (Expect).clone()) as ();
    unimplemented!();
    {
        let mut S: () = unimplemented!();
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let S: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
}
pub fn RunTestCaseImpl_36(MS: Ptr<MultiStringType>, mut args: ()) {
    let mut Expect: Ptr<()> = ({ MultiStringTypeImpl::to_pconst_12(&MS) });
    let mut TestPath: Ptr<()> = ({ MultiStringTypeImpl::to_pconst_12(&MS) });
    let mut TestPathEnd: Ptr<()> = ({ StrEnd_8((TestPath).clone()) });
    let mut Size: () = ((TestPathEnd).clone() - (TestPath).clone()) as ();
    let mut SSize: () = (({ StrEnd_8((Expect).clone()) }) - (Expect).clone()) as ();
    unimplemented!();
    {
        let mut S: () = unimplemented!();
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let S: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
}
pub fn RunTestCaseImpl_37(MS: Ptr<MultiStringType>) {
    let mut Expect: Ptr<()> = ({ MultiStringTypeImpl::to_pconst_12(&MS) });
    let mut TestPath: Ptr<()> = ({ MultiStringTypeImpl::to_pconst_14(&MS) });
    let mut TestPathEnd: Ptr<()> = ({ StrEnd_9((TestPath).clone()) });
    let mut Size: () = ((TestPathEnd).clone() - (TestPath).clone()) as ();
    let mut SSize: () = (({ StrEnd_8((Expect).clone()) }) - (Expect).clone()) as ();
    unimplemented!();
    {
        let mut S: () = unimplemented!();
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let S: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
}
pub fn RunTestCaseImpl_38(MS: Ptr<MultiStringType>, mut args: ()) {
    let mut Expect: Ptr<()> = ({ MultiStringTypeImpl::to_pconst_12(&MS) });
    let mut TestPath: Ptr<()> = ({ MultiStringTypeImpl::to_pconst_14(&MS) });
    let mut TestPathEnd: Ptr<()> = ({ StrEnd_9((TestPath).clone()) });
    let mut Size: () = ((TestPathEnd).clone() - (TestPath).clone()) as ();
    let mut SSize: () = (({ StrEnd_8((Expect).clone()) }) - (Expect).clone()) as ();
    unimplemented!();
    {
        let mut S: () = unimplemented!();
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let S: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
}
pub fn RunTestCaseImpl_39(MS: Ptr<MultiStringType>) {
    let mut Expect: Ptr<()> = ({ MultiStringTypeImpl::to_pconst_12(&MS) });
    let mut TestPath: Ptr<()> = ({ MultiStringTypeImpl::to_pconst_13(&MS) });
    let mut TestPathEnd: Ptr<()> = ({ StrEnd_10((TestPath).clone()) });
    let mut Size: () = ((TestPathEnd).clone() - (TestPath).clone()) as ();
    let mut SSize: () = (({ StrEnd_8((Expect).clone()) }) - (Expect).clone()) as ();
    unimplemented!();
    {
        let mut S: () = unimplemented!();
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let S: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
}
pub fn RunTestCaseImpl_40(MS: Ptr<MultiStringType>, mut args: ()) {
    let mut Expect: Ptr<()> = ({ MultiStringTypeImpl::to_pconst_12(&MS) });
    let mut TestPath: Ptr<()> = ({ MultiStringTypeImpl::to_pconst_13(&MS) });
    let mut TestPathEnd: Ptr<()> = ({ StrEnd_10((TestPath).clone()) });
    let mut Size: () = ((TestPathEnd).clone() - (TestPath).clone()) as ();
    let mut SSize: () = (({ StrEnd_8((Expect).clone()) }) - (Expect).clone()) as ();
    unimplemented!();
    {
        let mut S: () = unimplemented!();
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let S: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
}
pub fn RunTestCaseImpl_41(MS: Ptr<MultiStringType>) {
    let mut Expect: Ptr<()> = ({ MultiStringTypeImpl::to_pconst_12(&MS) });
    let mut TestPath: Ptr<()> = ({ MultiStringTypeImpl::to_pconst_15(&MS) });
    let mut TestPathEnd: Ptr<()> = ({ StrEnd_11((TestPath).clone()) });
    let mut Size: () = ((TestPathEnd).clone() - (TestPath).clone()) as ();
    let mut SSize: () = (({ StrEnd_8((Expect).clone()) }) - (Expect).clone()) as ();
    unimplemented!();
    {
        let mut S: () = unimplemented!();
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let S: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
}
pub fn RunTestCaseImpl_42(MS: Ptr<MultiStringType>, mut args: ()) {
    let mut Expect: Ptr<()> = ({ MultiStringTypeImpl::to_pconst_12(&MS) });
    let mut TestPath: Ptr<()> = ({ MultiStringTypeImpl::to_pconst_15(&MS) });
    let mut TestPathEnd: Ptr<()> = ({ StrEnd_11((TestPath).clone()) });
    let mut Size: () = ((TestPathEnd).clone() - (TestPath).clone()) as ();
    let mut SSize: () = (({ StrEnd_8((Expect).clone()) }) - (Expect).clone()) as ();
    unimplemented!();
    {
        let mut S: () = unimplemented!();
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let S: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
}
pub fn RunTestCaseImpl_43(MS: Ptr<MultiStringType>) {
    let mut Expect: Ptr<()> = ({ MultiStringTypeImpl::to_pconst_12(&MS) });
    let mut TestPath: Ptr<()> = ({ MultiStringTypeImpl::to_pconst_16(&MS) });
    let mut TestPathEnd: Ptr<()> = ({ StrEnd_12((TestPath).clone()) });
    let mut Size: () = ((TestPathEnd).clone() - (TestPath).clone()) as ();
    let mut SSize: () = (({ StrEnd_8((Expect).clone()) }) - (Expect).clone()) as ();
    unimplemented!();
    {
        let mut S: () = unimplemented!();
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let S: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
}
pub fn RunTestCaseImpl_44(MS: Ptr<MultiStringType>, mut args: ()) {
    let mut Expect: Ptr<()> = ({ MultiStringTypeImpl::to_pconst_12(&MS) });
    let mut TestPath: Ptr<()> = ({ MultiStringTypeImpl::to_pconst_16(&MS) });
    let mut TestPathEnd: Ptr<()> = ({ StrEnd_12((TestPath).clone()) });
    let mut Size: () = ((TestPathEnd).clone() - (TestPath).clone()) as ();
    let mut SSize: () = (({ StrEnd_8((Expect).clone()) }) - (Expect).clone()) as ();
    unimplemented!();
    {
        let mut S: () = unimplemented!();
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let S: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
}
pub fn RunTestCase_45(MS: Ptr<MultiStringType>) {
    ({ RunTestCaseImpl_35((MS).clone()) });
    ({ RunTestCaseImpl_36((MS).clone(), std_filesystem_path_format_auto_format) });
    ({ RunTestCaseImpl_36((MS).clone(), std_filesystem_path_format_native_format) });
    ({ RunTestCaseImpl_36((MS).clone(), std_filesystem_path_format_generic_format) });
    ({ RunTestCaseImpl_36((MS).clone(), std_filesystem_path_format_auto_format) });
    ({ RunTestCaseImpl_36((MS).clone(), std_filesystem_path_format_native_format) });
    ({ RunTestCaseImpl_36((MS).clone(), std_filesystem_path_format_generic_format) });
}
pub fn RunTestCase_46(MS: Ptr<MultiStringType>) {
    ({ RunTestCaseImpl_37((MS).clone()) });
    ({ RunTestCaseImpl_38((MS).clone(), std_filesystem_path_format_auto_format) });
    ({ RunTestCaseImpl_38((MS).clone(), std_filesystem_path_format_native_format) });
    ({ RunTestCaseImpl_38((MS).clone(), std_filesystem_path_format_generic_format) });
    ({ RunTestCaseImpl_38((MS).clone(), std_filesystem_path_format_auto_format) });
    ({ RunTestCaseImpl_38((MS).clone(), std_filesystem_path_format_native_format) });
    ({ RunTestCaseImpl_38((MS).clone(), std_filesystem_path_format_generic_format) });
}
pub fn RunTestCase_47(MS: Ptr<MultiStringType>) {
    ({ RunTestCaseImpl_39((MS).clone()) });
    ({ RunTestCaseImpl_40((MS).clone(), std_filesystem_path_format_auto_format) });
    ({ RunTestCaseImpl_40((MS).clone(), std_filesystem_path_format_native_format) });
    ({ RunTestCaseImpl_40((MS).clone(), std_filesystem_path_format_generic_format) });
    ({ RunTestCaseImpl_40((MS).clone(), std_filesystem_path_format_auto_format) });
    ({ RunTestCaseImpl_40((MS).clone(), std_filesystem_path_format_native_format) });
    ({ RunTestCaseImpl_40((MS).clone(), std_filesystem_path_format_generic_format) });
}
pub fn RunTestCase_48(MS: Ptr<MultiStringType>) {
    ({ RunTestCaseImpl_41((MS).clone()) });
    ({ RunTestCaseImpl_42((MS).clone(), std_filesystem_path_format_auto_format) });
    ({ RunTestCaseImpl_42((MS).clone(), std_filesystem_path_format_native_format) });
    ({ RunTestCaseImpl_42((MS).clone(), std_filesystem_path_format_generic_format) });
    ({ RunTestCaseImpl_42((MS).clone(), std_filesystem_path_format_auto_format) });
    ({ RunTestCaseImpl_42((MS).clone(), std_filesystem_path_format_native_format) });
    ({ RunTestCaseImpl_42((MS).clone(), std_filesystem_path_format_generic_format) });
}
pub fn RunTestCase_49(MS: Ptr<MultiStringType>) {
    ({ RunTestCaseImpl_43((MS).clone()) });
    ({ RunTestCaseImpl_44((MS).clone(), std_filesystem_path_format_auto_format) });
    ({ RunTestCaseImpl_44((MS).clone(), std_filesystem_path_format_native_format) });
    ({ RunTestCaseImpl_44((MS).clone(), std_filesystem_path_format_generic_format) });
    ({ RunTestCaseImpl_44((MS).clone(), std_filesystem_path_format_auto_format) });
    ({ RunTestCaseImpl_44((MS).clone(), std_filesystem_path_format_native_format) });
    ({ RunTestCaseImpl_44((MS).clone(), std_filesystem_path_format_generic_format) });
}
pub fn test_sfinae_50() {
    {
        const _: () = assert!(true, "std::is_constructible<path, It>::value");;
    }
    {
        const _: () = assert!(true, "std::is_constructible<path, It>::value");;
    }
    {
        const _: () = assert!(true, "std::is_constructible<path, It>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_constructible<path, It>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_constructible<path, int*>::value");;
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Traits_51 {}
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
    'loop_: for MS in IterRange::new(
        (PathList_13.with(|v| v.as_pointer()) as Ptr<MultiStringType>),
        (PathList_13.with(|v| v.as_pointer()) as Ptr<MultiStringType>).offset((42_()) as isize),
    ) {
        let MS: Ptr<MultiStringType> = (MS).clone();
        ({ RunTestCase_45((MS).clone()) });
        ({ RunTestCase_46((MS).clone()) });
        ({ RunTestCase_47((MS).clone()) });
        ({ RunTestCase_48((MS).clone()) });
        ({ RunTestCase_49((MS).clone()) });
    }
    ({ test_sfinae_50() });
    return 0_();
}
pub trait MultiStringTypeImpl {
    fn as_ptr_2(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_3(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_4(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_5(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_6(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn length_7(&self, _a0: Ptr<()>) -> ();
    fn length_8(&self, _a0: Ptr<()>) -> ();
    fn length_9(&self, _a0: Ptr<()>) -> ();
    fn length_10(&self, _a0: Ptr<()>) -> ();
    fn length_11(&self, _a0: Ptr<()>) -> ();
    fn to_pconst_12(&self) -> Ptr<()>;
    fn to_pconst_13(&self) -> Ptr<()>;
    fn to_pconst_14(&self) -> Ptr<()>;
    fn to_pconst_15(&self) -> Ptr<()>;
    fn to_pconst_16(&self) -> Ptr<()>;
}
impl MultiStringTypeImpl for Ptr<MultiStringType> {
    fn as_ptr_2(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn as_ptr_3(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn as_ptr_4(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn as_ptr_5(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn as_ptr_6(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u32_.clone());
    }
    fn length_7(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.sn_);
    }
    fn length_8(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.wn_);
    }
    fn length_9(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u8n_);
    }
    fn length_10(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u16n_);
    }
    fn length_11(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u32n_);
    }
    fn to_pconst_12(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn to_pconst_13(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn to_pconst_14(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn to_pconst_15(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn to_pconst_16(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u32_.clone());
    }
}
pub trait adl_IteratorImpl {
    fn iter_moves(&self) -> ();
    fn iter_swaps(&self) -> ();
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_index(&self, n: ()) -> Ptr<()>;
    fn operator_sub_10(&self, n: ()) -> adl_Iterator;
    fn operator_sub_11(&self, rhs: adl_Iterator) -> ();
    fn operator_add_assign(&self, n: ()) -> Ptr<adl_Iterator>;
    fn operator_sub_assign(&self, n: ()) -> Ptr<adl_Iterator>;
    fn operator_inc_14(&self) -> Ptr<adl_Iterator>;
    fn operator_post_inc_15(&self, _a0: ()) -> adl_Iterator;
    fn operator_dec_16(&self) -> Ptr<adl_Iterator>;
    fn operator_post_dec_17(&self, _a0: ()) -> adl_Iterator;
}
impl adl_IteratorImpl for Ptr<adl_Iterator> {
    fn iter_moves(&self) -> () {
        unimplemented!();
        return ((*self).with(|__s| __s.iter_moves_.clone()).read());
    }
    fn iter_swaps(&self) -> () {
        unimplemented!();
        return ((*self).with(|__s| __s.iter_swaps_.clone()).read());
    }
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone()).offset((n) as isize);
    }
    fn operator_sub_10(&self, mut n: ()) -> adl_Iterator {
        return adl_Iterator::new_1(
            { (*self).with(|__s| __s.ptr_.clone()).offset(-((n) as isize)) },
            { (*self).with(|__s| __s.iter_moves_.clone()) },
            { (*self).with(|__s| __s.iter_swaps_.clone()) },
        );
    }
    fn operator_sub_11(&self, mut rhs: adl_Iterator) -> () {
        return (((*self).with(|__s| __s.ptr_.clone()) - (rhs.ptr_).clone()) as () as ());
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<adl_Iterator> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<adl_Iterator> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn operator_inc_14(&self) -> Ptr<adl_Iterator> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_post_inc_15(&self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(*self).upgrade().deref()).clone();
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return prev;
    }
    fn operator_dec_16(&self) -> Ptr<adl_Iterator> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_post_dec_17(&self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(*self).upgrade().deref()).clone();
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return prev;
    }
}
pub trait cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_>;
    fn operator_post_inc_6(
        &self,
        mut _a0: (),
    ) -> cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_ {
        unimplemented!()
    }
}
impl cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_Impl
    for Ptr<cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_>
{
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
}
pub trait cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_>;
    fn operator_post_inc_6(
        &self,
        mut _a0: (),
    ) -> cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_ {
        unimplemented!()
    }
}
impl cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_Impl
    for Ptr<cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_>
{
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
}
pub trait cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_>;
    fn operator_post_inc_6(
        &self,
        mut _a0: (),
    ) -> cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_ {
        unimplemented!()
    }
}
impl cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_Impl
    for Ptr<cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_>
{
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
}
pub trait cpp17_input_iterator_const_char_ptr__Traits_Impl {
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_const_char_ptr__Traits_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp17_input_iterator_const_char_ptr__Traits_ {
        unimplemented!()
    }
}
impl cpp17_input_iterator_const_char_ptr__Traits_Impl
    for Ptr<cpp17_input_iterator_const_char_ptr__Traits_>
{
}
pub trait cpp17_input_iterator_const_char_ptr__const_char_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_const_char_ptr__const_char_ptr_>;
    fn operator_post_inc_6(
        &self,
        mut _a0: (),
    ) -> cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
        unimplemented!()
    }
}
impl cpp17_input_iterator_const_char_ptr__const_char_ptr_Impl
    for Ptr<cpp17_input_iterator_const_char_ptr__const_char_ptr_>
{
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_const_char_ptr__const_char_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
}
pub trait cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_>;
    fn operator_post_inc_6(
        &self,
        mut _a0: (),
    ) -> cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_ {
        unimplemented!()
    }
}
impl cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_Impl
    for Ptr<cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_>
{
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
}
pub trait cpp17_input_iterator_int_ptr__int_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp17_input_iterator_int_ptr__int_ptr_ {
        unimplemented!()
    }
}
impl cpp17_input_iterator_int_ptr__int_ptr_Impl for Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {}
pub trait cpp17_output_iterator_const_char_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_output_iterator_const_char_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp17_output_iterator_const_char_ptr_ {
        unimplemented!()
    }
}
impl cpp17_output_iterator_const_char_ptr_Impl for Ptr<cpp17_output_iterator_const_char_ptr_> {}
pub trait cpp17_output_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_output_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp17_output_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl cpp17_output_iterator_int_ptr_Impl for Ptr<cpp17_output_iterator_int_ptr_> {}
pub trait cpp20_input_iterator_int_ptr_Impl {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_input_iterator_int_ptr_>,
    ) -> Ptr<cpp20_input_iterator_int_ptr_>;
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_inc_5(&self) -> Ptr<cpp20_input_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) {
        unimplemented!()
    }
}
impl cpp20_input_iterator_int_ptr_Impl for Ptr<cpp20_input_iterator_int_ptr_> {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_input_iterator_int_ptr_>,
    ) -> Ptr<cpp20_input_iterator_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone());
    }
}
pub trait cpp20_output_iterator_int_ptr_Impl {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_output_iterator_int_ptr_>,
    ) -> Ptr<cpp20_output_iterator_int_ptr_>;
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_inc_5(&self) -> Ptr<cpp20_output_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp20_output_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl cpp20_output_iterator_int_ptr_Impl for Ptr<cpp20_output_iterator_int_ptr_> {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_output_iterator_int_ptr_>,
    ) -> Ptr<cpp20_output_iterator_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone());
    }
}
pub trait cpp20_random_access_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_index(&self, n: ()) -> Ptr<()>;
    fn operator_inc_7(&self) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_dec_8(&self) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: ()) -> cpp20_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: ()) -> cpp20_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
}
impl cpp20_random_access_iterator_int_ptr_Impl for Ptr<cpp20_random_access_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone()).offset((n) as isize);
    }
}
pub trait forward_sized_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_4(&self) -> Ptr<forward_sized_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_5(&self, mut _a0: ()) -> forward_sized_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl forward_sized_iterator_int_ptr_Impl for Ptr<forward_sized_iterator_int_ptr_> {}
pub trait rvalue_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn operator_inc_4(&self) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_5(&self, mut _a0: ()) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_dec_6(&self) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_dec_7(&self, mut _a0: ()) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add(&self, mut n: ()) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_sub_9(&self, mut n: ()) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_sub_10(&self, other: Ptr<rvalue_iterator_int_ptr_>) -> () {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn operator_cmp(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> ();
    fn operator_eq(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> ();
}
impl rvalue_iterator_int_ptr_Impl for Ptr<rvalue_iterator_int_ptr_> {
    fn operator_cmp(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> () {
        {
            let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.it_.clone())),
                &(_a0.with(|__s| __s.it_.clone())),
            )));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    fn operator_eq(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> () {
        return ({ (*self).with(|__s| __s.it_.clone()) } == { _a0.with(|__s| __s.it_.clone()) });
    }
}
pub trait support_double_move_trackerImpl {
    fn move_assign(
        &self,
        other: Ptr<support_double_move_tracker>,
    ) -> Ptr<support_double_move_tracker>;
}
impl support_double_move_trackerImpl for Ptr<support_double_move_tracker> {
    fn move_assign(
        &self,
        other: Ptr<support_double_move_tracker>,
    ) -> Ptr<support_double_move_tracker> {
        unimplemented!();
        field!(other, moved_from_).write(true);
        field!((*self), moved_from_).write(false);
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = PathList_13.with(|_| ());
    let _ = PathListSize_14.with(|_| ());
    let _ = outstanding_bytes_15.with(|_| ());
    let _ = alloc_count_16.with(|_| ());
    let _ = dealloc_count_17.with(|_| ());
    let _ = disable_default_constructor_18.with(|_| ());
}
