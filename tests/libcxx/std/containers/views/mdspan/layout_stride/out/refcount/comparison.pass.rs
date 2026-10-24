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
pub struct not_extents_constructible_tag {}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(12)]
pub struct always_convertible_layout_mapping_std_extents_unsigned_int__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
    #[offset(4)]
    offset_: (),
    #[offset(8)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_unsigned_int__ {
    pub fn new_1(ext: Ptr<()>, offset: Option<()>, scaling: Option<()>) -> Self {
        let mut offset: () = offset.unwrap_or_else(|| 0_());
        let mut scaling: () = scaling.unwrap_or_else(|| 1_());
        Self {
            extents_: unimplemented!(),
            offset_: offset,
            scaling_: scaling,
        }
    }
    pub fn is_always_unique() -> () {
        return true;
    }
    pub fn is_always_exhaustive() -> () {
        return true;
    }
    pub fn is_always_strided() -> () {
        return true;
    }
}
impl Default for always_convertible_layout_mapping_std_extents_unsigned_int__ {
    fn default() -> Self {
        {
            always_convertible_layout_mapping_std_extents_unsigned_int__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(12)]
pub struct always_convertible_layout_mapping_std_extents_unsigned_int__5__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
    #[offset(4)]
    offset_: (),
    #[offset(8)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_unsigned_int__5__ {
    pub fn new_1(ext: Ptr<()>, offset: Option<()>, scaling: Option<()>) -> Self {
        let mut offset: () = offset.unwrap_or_else(|| 0_());
        let mut scaling: () = scaling.unwrap_or_else(|| 1_());
        Self {
            extents_: unimplemented!(),
            offset_: offset,
            scaling_: scaling,
        }
    }
    pub fn is_always_unique() -> () {
        return true;
    }
    pub fn is_always_exhaustive() -> () {
        return true;
    }
    pub fn is_always_strided() -> () {
        return true;
    }
}
impl Default for always_convertible_layout_mapping_std_extents_unsigned_int__5__ {
    fn default() -> Self {
        {
            always_convertible_layout_mapping_std_extents_unsigned_int__5__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(16)]
pub struct always_convertible_layout_mapping_std_extents_unsigned_int__18446744073709551615__18446744073709551615__
{
    #[offset(0)]
    #[byte_size(8)]
    extents_: (),
    #[offset(8)]
    offset_: (),
    #[offset(12)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_unsigned_int__18446744073709551615__18446744073709551615__ { pub fn new_1 ( ext : Ptr< ()  > , offset : Option< ()  > , scaling : Option< ()  > , ) -> Self { let mut offset : ()   = offset.unwrap_or_else(|| 0_() ) ;
 let mut scaling : ()   = scaling.unwrap_or_else(|| 1_() ) ;
 Self { extents_ : unimplemented!()  , offset_ : offset  , scaling_ : scaling  , }  } pub fn is_always_unique ( ) -> ()  { return true   ;
 } pub fn is_always_exhaustive ( ) -> ()  { return true   ;
 } pub fn is_always_strided ( ) -> ()  { return true   ;
 } }
impl Default for always_convertible_layout_mapping_std_extents_unsigned_int__18446744073709551615__18446744073709551615__ { fn default() -> Self { { always_convertible_layout_mapping_std_extents_unsigned_int__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , offset_ : unimplemented!() , scaling_ : unimplemented!() , } } } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_convertible_layout {}
pub fn test_comparison_different_rank_0() {
    let mut D: () = dynamic_extent_1.with(|rc| *rc.borrow());
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, D>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, D>, std::extents<T2, 5>>"
    );;
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, 5>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1>, std::extents<T2, 1>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, D>, std::extents<T2>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 1>, std::extents<T2>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, D>, std::extents<T2, D, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, 5, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, 5, 1>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, D, D>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5, D>, std::extents<T2, 5>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5, 1>, std::extents<T2, 5>>"
    );;
}
pub fn test_comparison_different_rank_2() {
    let mut D: () = dynamic_extent_1.with(|rc| *rc.borrow());
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, D>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, D>, std::extents<T2, 5>>"
    );;
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, 5>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1>, std::extents<T2, 1>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, D>, std::extents<T2>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 1>, std::extents<T2>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, D>, std::extents<T2, D, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, 5, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, 5, 1>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, D, D>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5, D>, std::extents<T2, 5>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5, 1>, std::extents<T2, 5>>"
    );;
}
pub fn test_comparison_different_rank_3() {
    let mut D: () = dynamic_extent_1.with(|rc| *rc.borrow());
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, D>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, D>, std::extents<T2, 5>>"
    );;
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, 5>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1>, std::extents<T2, 1>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, D>, std::extents<T2>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 1>, std::extents<T2>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, D>, std::extents<T2, D, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, 5, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, 5, 1>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, D, D>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5, D>, std::extents<T2, 5>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5, 1>, std::extents<T2, 5>>"
    );;
}
pub fn test_comparison_different_rank_4() {
    let mut D: () = dynamic_extent_1.with(|rc| *rc.borrow());
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, D>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, D>, std::extents<T2, 5>>"
    );;
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, 5>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1>, std::extents<T2, 1>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, D>, std::extents<T2>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 1>, std::extents<T2>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, D>, std::extents<T2, D, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, 5, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, 5, 1>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, D, D>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5, D>, std::extents<T2, 5>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5, 1>, std::extents<T2, 5>>"
    );;
}
pub fn test_comparison_5(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_6(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_7(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_8(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_9(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_10(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_11(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_12(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_13(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_14(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_15(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_16(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_17(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_18(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_19(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_20(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_21(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_22(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_23(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_24(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_25(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_26(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_27(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_28(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_29(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_30(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_31(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_32(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_33(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_34(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_35(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_36(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_37(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_38(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_39(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_40(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_41(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_42(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_43(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_44(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_45(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_46(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_47(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_48(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_49(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_50(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_51(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_52(
    mut equal: (),
    dest_exts: (),
    src_exts: (),
    dest_strides: (),
    src_strides: (),
) {
    let dest_exts: Value<()> = Rc::new(RefCell::new(dest_exts));
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let dest_strides: Value<()> = Rc::new(RefCell::new(dest_strides));
    let src_strides: Value<()> = Rc::new(RefCell::new(src_strides));
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn test_comparison_same_rank_53() {
    let mut D: () = dynamic_extent_1.with(|rc| *rc.borrow());
    ({
        test_comparison_5(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_6(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_6(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_7(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_8(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_9(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_7(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_6(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_6(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_7(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_10(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_11(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_12(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_13(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_14(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_14(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_12(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_15(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_16(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub fn test_comparison_same_rank_54() {
    let mut D: () = dynamic_extent_1.with(|rc| *rc.borrow());
    ({
        test_comparison_17(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_18(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_18(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_19(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_20(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_21(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_19(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_18(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_18(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_19(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_22(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_23(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_24(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_25(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_26(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_26(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_24(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_27(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_28(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub fn test_comparison_same_rank_55() {
    let mut D: () = dynamic_extent_1.with(|rc| *rc.borrow());
    ({
        test_comparison_29(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_30(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_30(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_31(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_32(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_33(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_31(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_30(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_30(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_31(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_34(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_35(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_36(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_37(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_38(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_38(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_36(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_39(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_40(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub fn test_comparison_same_rank_56() {
    let mut D: () = dynamic_extent_1.with(|rc| *rc.borrow());
    ({
        test_comparison_41(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_42(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_42(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_43(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_44(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_45(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_43(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_42(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_42(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_43(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_46(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_47(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_48(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_49(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_50(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_50(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_48(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_51(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_52(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub fn test_comparison_with_57(mut expect_equal: (), e1: (), strides: (), e2: ()) {
    let e1: Value<()> = Rc::new(RefCell::new(e1));
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let e2: Value<()> = Rc::new(RefCell::new(e2));
    let map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let other_map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_comparison_with_58(mut expect_equal: (), e1: (), strides: (), e2: ()) {
    let e1: Value<()> = Rc::new(RefCell::new(e1));
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let e2: Value<()> = Rc::new(RefCell::new(e2));
    let map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let other_map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_comparison_with_59(mut expect_equal: (), e1: (), strides: (), e2: ()) {
    let e1: Value<()> = Rc::new(RefCell::new(e1));
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let e2: Value<()> = Rc::new(RefCell::new(e2));
    let map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let other_map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_comparison_with_60(mut expect_equal: (), e1: (), strides: (), e2: ()) {
    let e1: Value<()> = Rc::new(RefCell::new(e1));
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let e2: Value<()> = Rc::new(RefCell::new(e2));
    let map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let other_map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_comparison_with_61(mut expect_equal: (), e1: (), strides: (), e2: ()) {
    let e1: Value<()> = Rc::new(RefCell::new(e1));
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let e2: Value<()> = Rc::new(RefCell::new(e2));
    let map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let other_map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_comparison_with_62(mut expect_equal: (), e1: (), strides: (), e2: ()) {
    let e1: Value<()> = Rc::new(RefCell::new(e1));
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let e2: Value<()> = Rc::new(RefCell::new(e2));
    let map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let other_map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_comparison_with_63(mut expect_equal: (), e1: (), strides: (), e2: ()) {
    let e1: Value<()> = Rc::new(RefCell::new(e1));
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let e2: Value<()> = Rc::new(RefCell::new(e2));
    let map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let other_map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_comparison_with_64(mut expect_equal: (), e1: (), strides: (), e2: ()) {
    let e1: Value<()> = Rc::new(RefCell::new(e1));
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let e2: Value<()> = Rc::new(RefCell::new(e2));
    let map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let other_map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_comparison_with_65(mut expect_equal: (), e1: (), strides: (), e2: ()) {
    let e1: Value<()> = Rc::new(RefCell::new(e1));
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let e2: Value<()> = Rc::new(RefCell::new(e2));
    let map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut other_map: always_convertible_layout_mapping_std_extents_unsigned_int__ =
        always_convertible_layout_mapping_std_extents_unsigned_int__::new_1(
            { e2.as_pointer() },
            None,
            None,
        );
    unimplemented!();
}
pub fn test_comparison_with_66(mut expect_equal: (), e1: (), strides: (), e2: ()) {
    let e1: Value<()> = Rc::new(RefCell::new(e1));
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let e2: Value<()> = Rc::new(RefCell::new(e2));
    let map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut other_map: always_convertible_layout_mapping_std_extents_unsigned_int__5__ =
        always_convertible_layout_mapping_std_extents_unsigned_int__5__::new_1(
            { e2.as_pointer() },
            None,
            None,
        );
    unimplemented!();
}
pub fn test_comparison_with_67(mut expect_equal: (), e1: (), strides: (), e2: ()) {
    let e1: Value<()> = Rc::new(RefCell::new(e1));
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let e2: Value<()> = Rc::new(RefCell::new(e2));
    let map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut other_map: always_convertible_layout_mapping_std_extents_unsigned_int__5__ =
        always_convertible_layout_mapping_std_extents_unsigned_int__5__::new_1(
            { e2.as_pointer() },
            None,
            None,
        );
    unimplemented!();
}
pub fn test_comparison_with_68(mut expect_equal: (), e1: (), strides: (), e2: ()) {
    let e1: Value<()> = Rc::new(RefCell::new(e1));
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let e2: Value<()> = Rc::new(RefCell::new(e2));
    let map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut other_map : always_convertible_layout_mapping_std_extents_unsigned_int__18446744073709551615__18446744073709551615__  = always_convertible_layout_mapping_std_extents_unsigned_int__18446744073709551615__18446744073709551615__ :: new_1 ( { e2 .as_pointer()  } , None , None , )   ;
    unimplemented!();
}
pub fn test_comparison_with_69(
    mut expect_equal: (),
    e1: (),
    strides: (),
    e2: (),
    mut other_args_4: (),
    mut other_args_5: (),
) {
    let e1: Value<()> = Rc::new(RefCell::new(e1));
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let e2: Value<()> = Rc::new(RefCell::new(e2));
    let map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut other_map : always_convertible_layout_mapping_std_extents_unsigned_int__18446744073709551615__18446744073709551615__  = always_convertible_layout_mapping_std_extents_unsigned_int__18446744073709551615__18446744073709551615__ :: new_1 ( { e2 .as_pointer()  } , { Some ( other_args_4  ) } , { Some ( other_args_5  ) } , )   ;
    unimplemented!();
}
pub fn test_comparison_with_70() {
    let mut D: () = dynamic_extent_1.with(|rc| *rc.borrow());
    let mut is_left_based: () =
        (is_same_v_71.with(|rc| *rc.borrow())) || (is_same_v_72.with(|rc| *rc.borrow()));
    ({ test_comparison_with_57(true, unimplemented!(), unimplemented!(), unimplemented!()) });
    ({ test_comparison_with_58(true, unimplemented!(), unimplemented!(), unimplemented!()) });
    ({ test_comparison_with_59(true, unimplemented!(), unimplemented!(), unimplemented!()) });
    ({ test_comparison_with_59(false, unimplemented!(), unimplemented!(), unimplemented!()) });
    ({
        test_comparison_with_60(
            is_left_based,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_with_60(
            !(is_left_based),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({ test_comparison_with_60(false, unimplemented!(), unimplemented!(), unimplemented!()) });
    {}
}
pub fn test_comparison_with_73() {
    let mut D: () = dynamic_extent_1.with(|rc| *rc.borrow());
    let mut is_left_based: () =
        (is_same_v_74.with(|rc| *rc.borrow())) || (is_same_v_75.with(|rc| *rc.borrow()));
    ({ test_comparison_with_61(true, unimplemented!(), unimplemented!(), unimplemented!()) });
    ({ test_comparison_with_62(true, unimplemented!(), unimplemented!(), unimplemented!()) });
    ({ test_comparison_with_63(true, unimplemented!(), unimplemented!(), unimplemented!()) });
    ({ test_comparison_with_63(false, unimplemented!(), unimplemented!(), unimplemented!()) });
    ({
        test_comparison_with_64(
            is_left_based,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_with_64(
            !(is_left_based),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({ test_comparison_with_64(false, unimplemented!(), unimplemented!(), unimplemented!()) });
    {}
}
pub fn test_comparison_with_76() {
    let mut D: () = dynamic_extent_1.with(|rc| *rc.borrow());
    let mut is_left_based: () =
        (is_same_v_77.with(|rc| *rc.borrow())) || (is_same_v_78.with(|rc| *rc.borrow()));
    ({ test_comparison_with_65(true, unimplemented!(), unimplemented!(), unimplemented!()) });
    ({ test_comparison_with_66(true, unimplemented!(), unimplemented!(), unimplemented!()) });
    ({ test_comparison_with_67(true, unimplemented!(), unimplemented!(), unimplemented!()) });
    ({ test_comparison_with_67(false, unimplemented!(), unimplemented!(), unimplemented!()) });
    ({
        test_comparison_with_68(
            is_left_based,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({
        test_comparison_with_68(
            !(is_left_based),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    ({ test_comparison_with_68(false, unimplemented!(), unimplemented!(), unimplemented!()) });
    {
        ({
            test_comparison_with_69(
                true,
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                0_(),
                2_(),
            )
        });
        ({
            test_comparison_with_69(
                false,
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                1_(),
                2_(),
            )
        });
    }
}
pub fn test_comparison_index_type_79() {
    ({ test_comparison_same_rank_53() });
    ({ test_comparison_different_rank_0() });
    ({ test_comparison_with_70() });
    ({ test_comparison_with_73() });
    ({ test_comparison_with_76() });
}
pub fn test_comparison_index_type_80() {
    ({ test_comparison_same_rank_54() });
    ({ test_comparison_different_rank_2() });
    ({ test_comparison_with_70() });
    ({ test_comparison_with_73() });
    ({ test_comparison_with_76() });
}
pub fn test_comparison_index_type_81() {
    ({ test_comparison_same_rank_55() });
    ({ test_comparison_different_rank_3() });
    ({ test_comparison_with_70() });
    ({ test_comparison_with_73() });
    ({ test_comparison_with_76() });
}
pub fn test_comparison_index_type_82() {
    ({ test_comparison_same_rank_56() });
    ({ test_comparison_different_rank_4() });
    ({ test_comparison_with_70() });
    ({ test_comparison_with_73() });
    ({ test_comparison_with_76() });
}
pub fn test_83() -> () {
    ({ test_comparison_index_type_79() });
    ({ test_comparison_index_type_80() });
    ({ test_comparison_index_type_81() });
    ({ test_comparison_index_type_82() });
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_83() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait always_convertible_layout_mapping_std_extents_unsigned_int__Impl {
    fn copy_assign(
        &self,
        other: Ptr<always_convertible_layout_mapping_std_extents_unsigned_int__>,
    ) -> Ptr<always_convertible_layout_mapping_std_extents_unsigned_int__> {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn stride(&self, mut r: ()) -> () {
        unimplemented!()
    }
    fn operator_call_9(&self) -> ();
}
impl always_convertible_layout_mapping_std_extents_unsigned_int__Impl
    for Ptr<always_convertible_layout_mapping_std_extents_unsigned_int__>
{
    fn operator_call_9(&self) -> () {
        let idx_a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return ((*self).with(|__s| __s.offset_)).wrapping_add(
            ((*self).with(|__s| __s.scaling_)).wrapping_mul(
                ({
                    lambda!(
                        Generic,
                        {
                            let this_: Value<
                                Ptr<always_convertible_layout_mapping_std_extents_unsigned_int__>,
                            > = Rc::new(RefCell::new((*self).clone()));
                            let idx_a: Ptr<()> = idx_a.as_pointer();
                        },
                        |_a0: ()| -> () {
                            let _a0: Value<()> = Rc::new(RefCell::new(_a0));
                            let mut res: () = 0_();
                            unimplemented!();
                            return res;
                        }
                    )
                    .spec::<fn(()) -> ()>(0)
                    .call(<()>::default())
                }),
            ),
        );
    }
}
pub trait always_convertible_layout_mapping_std_extents_unsigned_int__18446744073709551615__18446744073709551615__Impl
{
    fn copy_assign ( &self , other : Ptr< always_convertible_layout_mapping_std_extents_unsigned_int__18446744073709551615__18446744073709551615__  > , ) -> Ptr< always_convertible_layout_mapping_std_extents_unsigned_int__18446744073709551615__18446744073709551615__  >{
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int_int(&self, idx_0: (), idx_1: ()) -> ();
}
impl always_convertible_layout_mapping_std_extents_unsigned_int__18446744073709551615__18446744073709551615__Impl for Ptr<always_convertible_layout_mapping_std_extents_unsigned_int__18446744073709551615__18446744073709551615__> {
fn extents ( &self , ) -> Ptr< ()  > { return field_ptr!((* self )  , extents_)   ;
 } fn required_span_size ( &self , ) -> ()  { let mut size : ()  = 1_()  ;
  ;
 ;
 let mut r : ()  = 0_()  ;
  ;
 ;
 'loop_: while ( r < unimplemented!()  ) { { let rhs_0 = ( size  ) . wrapping_mul ( unimplemented!()  )  ;
 size  = rhs_0 } ;
 r  . postfix_inc () ;
 } return (unimplemented!() .read())   ;
 } fn stride ( &self , mut r : ()  , ) -> ()  { let mut s : ()  = 1_()  ;
  ;
 ;
 let mut i : ()  = 0_()  ;
  ;
 ;
 'loop_: while ( i < r ) { { let rhs_0 = ( s  ) . wrapping_mul ( unimplemented!()  )  ;
 s  = rhs_0 } ;
 i  . postfix_inc () ;
 } return ( s ) . wrapping_mul ( (* self ) .with(|__s| __s . scaling_ ) )   ;
 } fn operator_call_9_int_int ( &self , mut idx_0 : ()  , mut idx_1 : ()  , ) -> ()  { let idx_a : Value<() > = Rc::new(RefCell::new(unimplemented!() )) ;
  ;
 ;
 return ( (* self ) .with(|__s| __s . offset_ ) ) . wrapping_add ( ( (* self ) .with(|__s| __s . scaling_ ) ) . wrapping_mul ( ( ( { lambda! ( Generic , { let this_ : Value<Ptr< always_convertible_layout_mapping_std_extents_unsigned_int__18446744073709551615__18446744073709551615__  > > = Rc::new(RefCell::new(((* self ) ).clone())) ;
 let idx_a : Ptr< ()  > = idx_a .as_pointer()  ;
 } , | _a0 : ()  , | -> ()  { let _a0 : Value<() >  = Rc::new(RefCell::new(_a0)) ;
 let mut res : ()  = 0_()  ;
  ;
 ;
 { let __rhs = ( (unimplemented!() .read()) ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = __rhs ;
 let __rhs = ( (unimplemented!() .read()) ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = __rhs } ;
 return res   ;
 } )  .spec::<fn(()  ,) -> ()  >(0) .call ( <()  >::default()  , ) } ) ) ) )   ;
 } }
pub trait always_convertible_layout_mapping_std_extents_unsigned_int__5__Impl {
    fn copy_assign(
        &self,
        other: Ptr<always_convertible_layout_mapping_std_extents_unsigned_int__5__>,
    ) -> Ptr<always_convertible_layout_mapping_std_extents_unsigned_int__5__> {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int(&self, idx: ()) -> ();
}
impl always_convertible_layout_mapping_std_extents_unsigned_int__5__Impl
    for Ptr<always_convertible_layout_mapping_std_extents_unsigned_int__5__>
{
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
    fn required_span_size(&self) -> () {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
            r.postfix_inc();
        }
        return (unimplemented!().read());
    }
    fn stride(&self, mut r: ()) -> () {
        let mut s: () = 1_();
        let mut i: () = 0_();
        'loop_: while (i < r) {
            {
                let rhs_0 = (s).wrapping_mul(unimplemented!());
                s = rhs_0
            };
            i.postfix_inc();
        }
        return (s).wrapping_mul((*self).with(|__s| __s.scaling_));
    }
    fn operator_call_9_int(&self, mut idx: ()) -> () {
        let idx_a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return ((*self).with(|__s| __s.offset_)).wrapping_add(
            ((*self).with(|__s| __s.scaling_)).wrapping_mul(
                ({
                    lambda!(
                        Generic,
                        {
                            let this_: Value<
                                Ptr<
                                    always_convertible_layout_mapping_std_extents_unsigned_int__5__,
                                >,
                            > = Rc::new(RefCell::new((*self).clone()));
                            let idx_a: Ptr<()> = idx_a.as_pointer();
                        },
                        |_a0: ()| -> () {
                            let _a0: Value<()> = Rc::new(RefCell::new(_a0));
                            let mut res: () = 0_();
                            let __rhs = (unimplemented!().read())
                                .wrapping_add((unimplemented!()).wrapping_mul(res));
                            res = __rhs;
                            return res;
                        }
                    )
                    .spec::<fn(()) -> ()>(0)
                    .call(<()>::default())
                }),
            ),
        );
    }
}
pub fn __cpp2rust_init_globals() {}
