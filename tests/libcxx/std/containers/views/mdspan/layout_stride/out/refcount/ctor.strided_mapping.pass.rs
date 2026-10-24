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
pub struct always_convertible_layout_mapping_std_extents_int__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
    #[offset(4)]
    offset_: (),
    #[offset(8)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_int__ {
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
impl Default for always_convertible_layout_mapping_std_extents_int__ {
    fn default() -> Self {
        {
            always_convertible_layout_mapping_std_extents_int__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(12)]
pub struct always_convertible_layout_mapping_std_extents_int__18446744073709551615__ {
    #[offset(0)]
    #[byte_size(4)]
    extents_: (),
    #[offset(4)]
    offset_: (),
    #[offset(8)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_int__18446744073709551615__ {
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
impl Default for always_convertible_layout_mapping_std_extents_int__18446744073709551615__ {
    fn default() -> Self {
        {
            always_convertible_layout_mapping_std_extents_int__18446744073709551615__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(12)]
pub struct always_convertible_layout_mapping_std_extents_int__5__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
    #[offset(4)]
    offset_: (),
    #[offset(8)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_int__5__ {
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
impl Default for always_convertible_layout_mapping_std_extents_int__5__ {
    fn default() -> Self {
        {
            always_convertible_layout_mapping_std_extents_int__5__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(16)]
pub struct always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__
{
    #[offset(0)]
    #[byte_size(8)]
    extents_: (),
    #[offset(8)]
    offset_: (),
    #[offset(12)]
    scaling_: (),
}
impl
    always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__
{
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
impl Default for always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__ { fn default() -> Self { { always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , offset_ : unimplemented!() , scaling_ : unimplemented!() , } } } }
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(12)]
pub struct always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__ {
    #[offset(0)]
    #[byte_size(4)]
    extents_: (),
    #[offset(4)]
    offset_: (),
    #[offset(8)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__ {
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
impl Default for always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__ {
    fn default() -> Self {
        {
            always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(12)]
pub struct always_convertible_layout_mapping_std_extents_int__5__7__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
    #[offset(4)]
    offset_: (),
    #[offset(8)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_int__5__7__ {
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
impl Default for always_convertible_layout_mapping_std_extents_int__5__7__ {
    fn default() -> Self {
        {
            always_convertible_layout_mapping_std_extents_int__5__7__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(16)]
pub struct always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__8__9__1__
{
    #[offset(0)]
    #[byte_size(8)]
    extents_: (),
    #[offset(8)]
    offset_: (),
    #[offset(12)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__8__9__1__ { pub fn new_1 ( ext : Ptr< ()  > , offset : Option< ()  > , scaling : Option< ()  > , ) -> Self { let mut offset : ()   = offset.unwrap_or_else(|| 0_() ) ;
 let mut scaling : ()   = scaling.unwrap_or_else(|| 1_() ) ;
 Self { extents_ : unimplemented!()  , offset_ : offset  , scaling_ : scaling  , }  } pub fn is_always_unique ( ) -> ()  { return true   ;
 } pub fn is_always_exhaustive ( ) -> ()  { return true   ;
 } pub fn is_always_strided ( ) -> ()  { return true   ;
 } }
impl Default for always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__8__9__1__ { fn default() -> Self { { always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__8__9__1__ { extents_ : unimplemented!() , offset_ : unimplemented!() , scaling_ : unimplemented!() , } } } }
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(28)]
pub struct always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__
{
    #[offset(0)]
    #[byte_size(20)]
    extents_: (),
    #[offset(20)]
    offset_: (),
    #[offset(24)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { pub fn new_1 ( ext : Ptr< ()  > , offset : Option< ()  > , scaling : Option< ()  > , ) -> Self { let mut offset : ()   = offset.unwrap_or_else(|| 0_() ) ;
 let mut scaling : ()   = scaling.unwrap_or_else(|| 1_() ) ;
 Self { extents_ : unimplemented!()  , offset_ : offset  , scaling_ : scaling  , }  } pub fn is_always_unique ( ) -> ()  { return true   ;
 } pub fn is_always_exhaustive ( ) -> ()  { return true   ;
 } pub fn is_always_strided ( ) -> ()  { return true   ;
 } }
impl Default for always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { fn default() -> Self { { always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , offset_ : unimplemented!() , scaling_ : unimplemented!() , } } } }
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(12)]
pub struct always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__8__9__1__ {
    #[offset(0)]
    #[byte_size(4)]
    extents_: (),
    #[offset(4)]
    offset_: (),
    #[offset(8)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__8__9__1__ {
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
impl Default
    for always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__8__9__1__
{
    fn default() -> Self {
        {
            always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__8__9__1__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(12)]
pub struct always_convertible_layout_mapping_std_extents_int__5__7__8__9__1__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
    #[offset(4)]
    offset_: (),
    #[offset(8)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_int__5__7__8__9__1__ {
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
impl Default for always_convertible_layout_mapping_std_extents_int__5__7__8__9__1__ {
    fn default() -> Self {
        {
            always_convertible_layout_mapping_std_extents_int__5__7__8__9__1__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(24)]
pub struct always_convertible_layout_mapping_std_extents_unsigned_long__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
    #[offset(8)]
    offset_: (),
    #[offset(16)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_unsigned_long__ {
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
impl Default for always_convertible_layout_mapping_std_extents_unsigned_long__ {
    fn default() -> Self {
        {
            always_convertible_layout_mapping_std_extents_unsigned_long__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(24)]
pub struct always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__ {
    #[offset(0)]
    #[byte_size(8)]
    extents_: (),
    #[offset(8)]
    offset_: (),
    #[offset(16)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__ {
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
impl Default
    for always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__
{
    fn default() -> Self {
        {
            always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(24)]
pub struct always_convertible_layout_mapping_std_extents_unsigned_long__5__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
    #[offset(8)]
    offset_: (),
    #[offset(16)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_unsigned_long__5__ {
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
impl Default for always_convertible_layout_mapping_std_extents_unsigned_long__5__ {
    fn default() -> Self {
        {
            always_convertible_layout_mapping_std_extents_unsigned_long__5__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(32)]
pub struct always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__
{
    #[offset(0)]
    #[byte_size(16)]
    extents_: (),
    #[offset(16)]
    offset_: (),
    #[offset(24)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__ { pub fn new_1 ( ext : Ptr< ()  > , offset : Option< ()  > , scaling : Option< ()  > , ) -> Self { let mut offset : ()   = offset.unwrap_or_else(|| 0_() ) ;
 let mut scaling : ()   = scaling.unwrap_or_else(|| 1_() ) ;
 Self { extents_ : unimplemented!()  , offset_ : offset  , scaling_ : scaling  , }  } pub fn is_always_unique ( ) -> ()  { return true   ;
 } pub fn is_always_exhaustive ( ) -> ()  { return true   ;
 } pub fn is_always_strided ( ) -> ()  { return true   ;
 } }
impl Default for always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__ { fn default() -> Self { { always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , offset_ : unimplemented!() , scaling_ : unimplemented!() , } } } }
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(24)]
pub struct always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__ {
    #[offset(0)]
    #[byte_size(8)]
    extents_: (),
    #[offset(8)]
    offset_: (),
    #[offset(16)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__ {
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
impl Default
    for always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__
{
    fn default() -> Self {
        {
            always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(24)]
pub struct always_convertible_layout_mapping_std_extents_unsigned_long__5__7__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
    #[offset(8)]
    offset_: (),
    #[offset(16)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_unsigned_long__5__7__ {
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
impl Default for always_convertible_layout_mapping_std_extents_unsigned_long__5__7__ {
    fn default() -> Self {
        {
            always_convertible_layout_mapping_std_extents_unsigned_long__5__7__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(32)]
pub struct always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__8__9__1__
{
    #[offset(0)]
    #[byte_size(16)]
    extents_: (),
    #[offset(16)]
    offset_: (),
    #[offset(24)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__8__9__1__ { pub fn new_1 ( ext : Ptr< ()  > , offset : Option< ()  > , scaling : Option< ()  > , ) -> Self { let mut offset : ()   = offset.unwrap_or_else(|| 0_() ) ;
 let mut scaling : ()   = scaling.unwrap_or_else(|| 1_() ) ;
 Self { extents_ : unimplemented!()  , offset_ : offset  , scaling_ : scaling  , }  } pub fn is_always_unique ( ) -> ()  { return true   ;
 } pub fn is_always_exhaustive ( ) -> ()  { return true   ;
 } pub fn is_always_strided ( ) -> ()  { return true   ;
 } }
impl Default for always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__8__9__1__ { fn default() -> Self { { always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__8__9__1__ { extents_ : unimplemented!() , offset_ : unimplemented!() , scaling_ : unimplemented!() , } } } }
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(56)]
pub struct always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__
{
    #[offset(0)]
    #[byte_size(40)]
    extents_: (),
    #[offset(40)]
    offset_: (),
    #[offset(48)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { pub fn new_1 ( ext : Ptr< ()  > , offset : Option< ()  > , scaling : Option< ()  > , ) -> Self { let mut offset : ()   = offset.unwrap_or_else(|| 0_() ) ;
 let mut scaling : ()   = scaling.unwrap_or_else(|| 1_() ) ;
 Self { extents_ : unimplemented!()  , offset_ : offset  , scaling_ : scaling  , }  } pub fn is_always_unique ( ) -> ()  { return true   ;
 } pub fn is_always_exhaustive ( ) -> ()  { return true   ;
 } pub fn is_always_strided ( ) -> ()  { return true   ;
 } }
impl Default for always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { fn default() -> Self { { always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , offset_ : unimplemented!() , scaling_ : unimplemented!() , } } } }
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(24)]
pub struct always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__8__9__1__
{
    #[offset(0)]
    #[byte_size(8)]
    extents_: (),
    #[offset(8)]
    offset_: (),
    #[offset(16)]
    scaling_: (),
}
impl
    always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__8__9__1__
{
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
impl Default for always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__8__9__1__ { fn default() -> Self { { always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__8__9__1__ { extents_ : unimplemented!() , offset_ : unimplemented!() , scaling_ : unimplemented!() , } } } }
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(24)]
pub struct always_convertible_layout_mapping_std_extents_unsigned_long__5__7__8__9__1__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
    #[offset(8)]
    offset_: (),
    #[offset(16)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_unsigned_long__5__7__8__9__1__ {
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
impl Default for always_convertible_layout_mapping_std_extents_unsigned_long__5__7__8__9__1__ {
    fn default() -> Self {
        {
            always_convertible_layout_mapping_std_extents_unsigned_long__5__7__8__9__1__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(24)]
pub struct always_convertible_layout_mapping_std_extents_long__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
    #[offset(8)]
    offset_: (),
    #[offset(16)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_long__ {
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
impl Default for always_convertible_layout_mapping_std_extents_long__ {
    fn default() -> Self {
        {
            always_convertible_layout_mapping_std_extents_long__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(24)]
pub struct always_convertible_layout_mapping_std_extents_long__18446744073709551615__ {
    #[offset(0)]
    #[byte_size(8)]
    extents_: (),
    #[offset(8)]
    offset_: (),
    #[offset(16)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_long__18446744073709551615__ {
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
impl Default for always_convertible_layout_mapping_std_extents_long__18446744073709551615__ {
    fn default() -> Self {
        {
            always_convertible_layout_mapping_std_extents_long__18446744073709551615__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(24)]
pub struct always_convertible_layout_mapping_std_extents_long__5__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
    #[offset(8)]
    offset_: (),
    #[offset(16)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_long__5__ {
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
impl Default for always_convertible_layout_mapping_std_extents_long__5__ {
    fn default() -> Self {
        {
            always_convertible_layout_mapping_std_extents_long__5__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(32)]
pub struct always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__
{
    #[offset(0)]
    #[byte_size(16)]
    extents_: (),
    #[offset(16)]
    offset_: (),
    #[offset(24)]
    scaling_: (),
}
impl
    always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__
{
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
impl Default for always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__ { fn default() -> Self { { always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , offset_ : unimplemented!() , scaling_ : unimplemented!() , } } } }
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(24)]
pub struct always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__ {
    #[offset(0)]
    #[byte_size(8)]
    extents_: (),
    #[offset(8)]
    offset_: (),
    #[offset(16)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__ {
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
impl Default for always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__ {
    fn default() -> Self {
        {
            always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(24)]
pub struct always_convertible_layout_mapping_std_extents_long__5__7__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
    #[offset(8)]
    offset_: (),
    #[offset(16)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_long__5__7__ {
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
impl Default for always_convertible_layout_mapping_std_extents_long__5__7__ {
    fn default() -> Self {
        {
            always_convertible_layout_mapping_std_extents_long__5__7__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(32)]
pub struct always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__8__9__1__
{
    #[offset(0)]
    #[byte_size(16)]
    extents_: (),
    #[offset(16)]
    offset_: (),
    #[offset(24)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__8__9__1__ { pub fn new_1 ( ext : Ptr< ()  > , offset : Option< ()  > , scaling : Option< ()  > , ) -> Self { let mut offset : ()   = offset.unwrap_or_else(|| 0_() ) ;
 let mut scaling : ()   = scaling.unwrap_or_else(|| 1_() ) ;
 Self { extents_ : unimplemented!()  , offset_ : offset  , scaling_ : scaling  , }  } pub fn is_always_unique ( ) -> ()  { return true   ;
 } pub fn is_always_exhaustive ( ) -> ()  { return true   ;
 } pub fn is_always_strided ( ) -> ()  { return true   ;
 } }
impl Default for always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__8__9__1__ { fn default() -> Self { { always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__8__9__1__ { extents_ : unimplemented!() , offset_ : unimplemented!() , scaling_ : unimplemented!() , } } } }
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(56)]
pub struct always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__
{
    #[offset(0)]
    #[byte_size(40)]
    extents_: (),
    #[offset(40)]
    offset_: (),
    #[offset(48)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { pub fn new_1 ( ext : Ptr< ()  > , offset : Option< ()  > , scaling : Option< ()  > , ) -> Self { let mut offset : ()   = offset.unwrap_or_else(|| 0_() ) ;
 let mut scaling : ()   = scaling.unwrap_or_else(|| 1_() ) ;
 Self { extents_ : unimplemented!()  , offset_ : offset  , scaling_ : scaling  , }  } pub fn is_always_unique ( ) -> ()  { return true   ;
 } pub fn is_always_exhaustive ( ) -> ()  { return true   ;
 } pub fn is_always_strided ( ) -> ()  { return true   ;
 } }
impl Default for always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { fn default() -> Self { { always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , offset_ : unimplemented!() , scaling_ : unimplemented!() , } } } }
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(24)]
pub struct always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__8__9__1__ {
    #[offset(0)]
    #[byte_size(8)]
    extents_: (),
    #[offset(8)]
    offset_: (),
    #[offset(16)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__8__9__1__ {
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
impl Default
    for always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__8__9__1__
{
    fn default() -> Self {
        {
            always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__8__9__1__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(24)]
pub struct always_convertible_layout_mapping_std_extents_long__5__7__8__9__1__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
    #[offset(8)]
    offset_: (),
    #[offset(16)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_long__5__7__8__9__1__ {
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
impl Default for always_convertible_layout_mapping_std_extents_long__5__7__8__9__1__ {
    fn default() -> Self {
        {
            always_convertible_layout_mapping_std_extents_long__5__7__8__9__1__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(12)]
pub struct always_convertible_layout_mapping_std_extents_int__18446744073709551615__4__ {
    #[offset(0)]
    #[byte_size(4)]
    extents_: (),
    #[offset(4)]
    offset_: (),
    #[offset(8)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_int__18446744073709551615__4__ {
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
impl Default for always_convertible_layout_mapping_std_extents_int__18446744073709551615__4__ {
    fn default() -> Self {
        {
            always_convertible_layout_mapping_std_extents_int__18446744073709551615__4__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(12)]
pub struct always_convertible_layout_mapping_std_extents_int__4__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
    #[offset(4)]
    offset_: (),
    #[offset(8)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_int__4__ {
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
impl Default for always_convertible_layout_mapping_std_extents_int__4__ {
    fn default() -> Self {
        {
            always_convertible_layout_mapping_std_extents_int__4__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(12)]
pub struct always_convertible_layout_mapping_std_extents_int__4__18446744073709551615__ {
    #[offset(0)]
    #[byte_size(4)]
    extents_: (),
    #[offset(4)]
    offset_: (),
    #[offset(8)]
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_int__4__18446744073709551615__ {
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
impl Default for always_convertible_layout_mapping_std_extents_int__4__18446744073709551615__ {
    fn default() -> Self {
        {
            always_convertible_layout_mapping_std_extents_int__4__18446744073709551615__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_convertible_layout {}
pub fn test_conversion_0(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_1(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_2(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_3(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_4(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_5(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_6(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_7(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_8(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_9(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_10(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_11(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_12(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_13(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_14(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_15(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_16(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_17(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_18(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_19(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_20(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_21(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_22(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_23(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_24(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_25(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_26(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_27(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_28(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_29(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_30(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_31(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_32(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_33(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_34(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_35(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_36(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_37(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_38(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_39(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_40(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_41(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_42(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_43(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_44(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_45(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_46(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_47(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_48(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_49(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_50(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_51(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_52(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_53(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_54(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_55(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_56(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_57(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_58(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_59(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_60(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_61(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_62(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_63(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_64(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_65(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_66(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_67(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_68(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_69(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_70(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_71(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_72(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_73(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_74(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_75(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_76(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_77(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_78(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_79(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_80(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_81(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_82(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_83(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_84(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_85(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_86(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_87(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_88(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_89(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_90(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_91(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_92(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_93(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_94(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_95(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_96(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_97(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_98(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_99(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_100(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_101(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_102(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_103(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_104(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_105(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_106(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_107(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_108(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_109(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_110(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_111(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_112(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_113(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_114(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_115(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_116(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_117(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_118(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_119(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_120(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_121(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_122(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_123(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_124(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_125(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_126(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_127(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_128(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_129(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_130(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_131(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_132(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_133(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_134(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_135(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_136(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_137(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_138(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_139(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_140(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_141(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_142(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_143(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let src: Value<()> = Rc::new(RefCell::new(
        ({
            lambda!(
                {
                    let src_exts: Ptr<()> = src_exts.as_pointer();
                },
                || -> () {
                    {
                        let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mut stride: () = 2_();
                        let mut r: () = 0_();
                        'loop_: while (r < unimplemented!()) {
                            unimplemented!().write(stride);
                            {
                                let rhs_0 = (stride).wrapping_mul(unimplemented!());
                                stride = rhs_0
                            };
                            r.postfix_inc();
                        }
                        return unimplemented!();
                    }
                    panic!("ub: non-void function does not return a value")
                }
            )
            .call()
        }),
    ));
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let dest_implicit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_conversion_144(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src: always_convertible_layout_mapping_std_extents_int__ = ({
        lambda!(
            {
                let src_exts: Ptr<()> = src_exts.as_pointer();
            },
            || -> always_convertible_layout_mapping_std_extents_int__ {
                {
                    return always_convertible_layout_mapping_std_extents_int__::new_1(
                        { (src_exts).clone() },
                        None,
                        None,
                    );
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_145(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src: always_convertible_layout_mapping_std_extents_int__18446744073709551615__ = ({
        lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__  { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( )
    });
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_146(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src: always_convertible_layout_mapping_std_extents_int__18446744073709551615__ = ({
        lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__  { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( )
    });
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_147(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src: always_convertible_layout_mapping_std_extents_int__5__ = ({
        lambda!(
            {
                let src_exts: Ptr<()> = src_exts.as_pointer();
            },
            || -> always_convertible_layout_mapping_std_extents_int__5__ {
                {
                    return always_convertible_layout_mapping_std_extents_int__5__::new_1(
                        { (src_exts).clone() },
                        None,
                        None,
                    );
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_148(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src : always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__  = ( { lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__  { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } )  ;
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_149(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src : always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__  = ( { lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__  { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } )  ;
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_150(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src: always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__ = ({
        lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__  { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( )
    });
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_151(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src: always_convertible_layout_mapping_std_extents_int__5__7__ = ({
        lambda!(
            {
                let src_exts: Ptr<()> = src_exts.as_pointer();
            },
            || -> always_convertible_layout_mapping_std_extents_int__5__7__ {
                {
                    return always_convertible_layout_mapping_std_extents_int__5__7__::new_1(
                        { (src_exts).clone() },
                        None,
                        None,
                    );
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_152(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src : always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__8__9__1__  = ( { lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__8__9__1__  { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__8__9__1__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } )  ;
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_153(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src : always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__  = ( { lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__  { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } )  ;
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_154(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src : always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__8__9__1__  = ( { lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__8__9__1__  { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__8__9__1__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } )  ;
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_155(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src: always_convertible_layout_mapping_std_extents_int__5__7__8__9__1__ = ({
        lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__5__7__8__9__1__  { { return always_convertible_layout_mapping_std_extents_int__5__7__8__9__1__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( )
    });
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_156(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src: always_convertible_layout_mapping_std_extents_unsigned_long__ = ({
        lambda!(
            {
                let src_exts: Ptr<()> = src_exts.as_pointer();
            },
            || -> always_convertible_layout_mapping_std_extents_unsigned_long__ {
                {
                    return always_convertible_layout_mapping_std_extents_unsigned_long__::new_1(
                        { (src_exts).clone() },
                        None,
                        None,
                    );
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_157(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src : always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__  = ( { lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__  { { return always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } )  ;
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_158(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src : always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__  = ( { lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__  { { return always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } )  ;
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_159(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src: always_convertible_layout_mapping_std_extents_unsigned_long__5__ = ({
        lambda!(
            {
                let src_exts: Ptr<()> = src_exts.as_pointer();
            },
            || -> always_convertible_layout_mapping_std_extents_unsigned_long__5__ {
                {
                    return always_convertible_layout_mapping_std_extents_unsigned_long__5__::new_1(
                        { (src_exts).clone() },
                        None,
                        None,
                    );
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_160(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src : always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__  = ( { lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__  { { return always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } )  ;
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_161(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src : always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__  = ( { lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__  { { return always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } )  ;
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_162(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src : always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__  = ( { lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__  { { return always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } )  ;
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_163(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src: always_convertible_layout_mapping_std_extents_unsigned_long__5__7__ = ({
        lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_unsigned_long__5__7__  { { return always_convertible_layout_mapping_std_extents_unsigned_long__5__7__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( )
    });
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_164(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src : always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__8__9__1__  = ( { lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__8__9__1__  { { return always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__8__9__1__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } )  ;
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_165(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src : always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__  = ( { lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__  { { return always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } )  ;
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_166(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src : always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__8__9__1__  = ( { lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__8__9__1__  { { return always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__8__9__1__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } )  ;
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_167(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src: always_convertible_layout_mapping_std_extents_unsigned_long__5__7__8__9__1__ = ({
        lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_unsigned_long__5__7__8__9__1__  { { return always_convertible_layout_mapping_std_extents_unsigned_long__5__7__8__9__1__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( )
    });
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_168(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src: always_convertible_layout_mapping_std_extents_int__ = ({
        lambda!(
            {
                let src_exts: Ptr<()> = src_exts.as_pointer();
            },
            || -> always_convertible_layout_mapping_std_extents_int__ {
                {
                    return always_convertible_layout_mapping_std_extents_int__::new_1(
                        { (src_exts).clone() },
                        None,
                        None,
                    );
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_169(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src: always_convertible_layout_mapping_std_extents_int__18446744073709551615__ = ({
        lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__  { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( )
    });
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_170(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src: always_convertible_layout_mapping_std_extents_int__18446744073709551615__ = ({
        lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__  { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( )
    });
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_171(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src: always_convertible_layout_mapping_std_extents_int__5__ = ({
        lambda!(
            {
                let src_exts: Ptr<()> = src_exts.as_pointer();
            },
            || -> always_convertible_layout_mapping_std_extents_int__5__ {
                {
                    return always_convertible_layout_mapping_std_extents_int__5__::new_1(
                        { (src_exts).clone() },
                        None,
                        None,
                    );
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_172(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src : always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__  = ( { lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__  { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } )  ;
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_173(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src : always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__  = ( { lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__  { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } )  ;
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_174(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src: always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__ = ({
        lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__  { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( )
    });
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_175(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src: always_convertible_layout_mapping_std_extents_int__5__7__ = ({
        lambda!(
            {
                let src_exts: Ptr<()> = src_exts.as_pointer();
            },
            || -> always_convertible_layout_mapping_std_extents_int__5__7__ {
                {
                    return always_convertible_layout_mapping_std_extents_int__5__7__::new_1(
                        { (src_exts).clone() },
                        None,
                        None,
                    );
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_176(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src : always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__8__9__1__  = ( { lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__8__9__1__  { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__8__9__1__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } )  ;
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_177(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src : always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__  = ( { lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__  { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } )  ;
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_178(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src : always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__8__9__1__  = ( { lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__8__9__1__  { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__8__9__1__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } )  ;
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_179(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src: always_convertible_layout_mapping_std_extents_int__5__7__8__9__1__ = ({
        lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__5__7__8__9__1__  { { return always_convertible_layout_mapping_std_extents_int__5__7__8__9__1__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( )
    });
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_180(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src: always_convertible_layout_mapping_std_extents_long__ = ({
        lambda!(
            {
                let src_exts: Ptr<()> = src_exts.as_pointer();
            },
            || -> always_convertible_layout_mapping_std_extents_long__ {
                {
                    return always_convertible_layout_mapping_std_extents_long__::new_1(
                        { (src_exts).clone() },
                        None,
                        None,
                    );
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_181(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src: always_convertible_layout_mapping_std_extents_long__18446744073709551615__ = ({
        lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_long__18446744073709551615__  { { return always_convertible_layout_mapping_std_extents_long__18446744073709551615__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( )
    });
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_182(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src: always_convertible_layout_mapping_std_extents_long__18446744073709551615__ = ({
        lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_long__18446744073709551615__  { { return always_convertible_layout_mapping_std_extents_long__18446744073709551615__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( )
    });
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_183(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src: always_convertible_layout_mapping_std_extents_long__5__ = ({
        lambda!(
            {
                let src_exts: Ptr<()> = src_exts.as_pointer();
            },
            || -> always_convertible_layout_mapping_std_extents_long__5__ {
                {
                    return always_convertible_layout_mapping_std_extents_long__5__::new_1(
                        { (src_exts).clone() },
                        None,
                        None,
                    );
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_184(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src : always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__  = ( { lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__  { { return always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } )  ;
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_185(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src : always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__  = ( { lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__  { { return always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } )  ;
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_186(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src: always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__ = ({
        lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__  { { return always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( )
    });
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_187(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src: always_convertible_layout_mapping_std_extents_long__5__7__ = ({
        lambda!(
            {
                let src_exts: Ptr<()> = src_exts.as_pointer();
            },
            || -> always_convertible_layout_mapping_std_extents_long__5__7__ {
                {
                    return always_convertible_layout_mapping_std_extents_long__5__7__::new_1(
                        { (src_exts).clone() },
                        None,
                        None,
                    );
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_188(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src : always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__8__9__1__  = ( { lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__8__9__1__  { { return always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__8__9__1__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } )  ;
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_189(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src : always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__  = ( { lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__  { { return always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } )  ;
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_190(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src : always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__8__9__1__  = ( { lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__8__9__1__  { { return always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__8__9__1__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } )  ;
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_191(src_exts: ()) {
    let src_exts: Value<()> = Rc::new(RefCell::new(src_exts));
    let mut src: always_convertible_layout_mapping_std_extents_long__5__7__8__9__1__ = ({
        lambda! ( { let src_exts : Ptr< ()  > = src_exts .as_pointer()  ;
 } , | | -> always_convertible_layout_mapping_std_extents_long__5__7__8__9__1__  { { return always_convertible_layout_mapping_std_extents_long__5__7__8__9__1__ :: new_1 ( { (src_exts ).clone() } , None , None , )    ;
 } panic!("ub: non-void function does not return a value") } )  .call ( )
    });
    const _: () = assert!(true, "");;
    let dest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_conversion_192() {
    let mut D: () = dynamic_extent_193.with(|rc| *rc.borrow());
    let mut idx_convertible: () = (unimplemented!() >= unimplemented!());
    let mut l_convertible: () = ((is_same_v_194.with(|rc| *rc.borrow()))
        || (is_same_v_195.with(|rc| *rc.borrow())))
        || (is_same_v_196.with(|rc| *rc.borrow()));
    let mut idx_l_convertible: () = (idx_convertible) && (l_convertible);
    ({ test_conversion_0(unimplemented!()) });
    ({ test_conversion_1(unimplemented!()) });
    ({ test_conversion_1(unimplemented!()) });
    ({ test_conversion_2(unimplemented!()) });
    ({ test_conversion_3(unimplemented!()) });
    ({ test_conversion_4(unimplemented!()) });
    ({ test_conversion_5(unimplemented!()) });
    ({ test_conversion_6(unimplemented!()) });
    ({ test_conversion_7(unimplemented!()) });
    ({ test_conversion_8(unimplemented!()) });
    ({ test_conversion_9(unimplemented!()) });
    ({ test_conversion_10(unimplemented!()) });
    ({ test_conversion_11(unimplemented!()) });
}
pub fn test_conversion_197() {
    let mut D: () = dynamic_extent_193.with(|rc| *rc.borrow());
    let mut idx_convertible: () = (unimplemented!() >= (unimplemented!() as ()));
    let mut l_convertible: () = ((is_same_v_194.with(|rc| *rc.borrow()))
        || (is_same_v_195.with(|rc| *rc.borrow())))
        || (is_same_v_196.with(|rc| *rc.borrow()));
    let mut idx_l_convertible: () = (idx_convertible) && (l_convertible);
    ({ test_conversion_12(unimplemented!()) });
    ({ test_conversion_13(unimplemented!()) });
    ({ test_conversion_13(unimplemented!()) });
    ({ test_conversion_14(unimplemented!()) });
    ({ test_conversion_15(unimplemented!()) });
    ({ test_conversion_16(unimplemented!()) });
    ({ test_conversion_17(unimplemented!()) });
    ({ test_conversion_18(unimplemented!()) });
    ({ test_conversion_19(unimplemented!()) });
    ({ test_conversion_20(unimplemented!()) });
    ({ test_conversion_21(unimplemented!()) });
    ({ test_conversion_22(unimplemented!()) });
    ({ test_conversion_23(unimplemented!()) });
}
pub fn test_conversion_198() {
    let mut D: () = dynamic_extent_193.with(|rc| *rc.borrow());
    let mut idx_convertible: () = ((unimplemented!() as ()) >= unimplemented!());
    let mut l_convertible: () = ((is_same_v_194.with(|rc| *rc.borrow()))
        || (is_same_v_195.with(|rc| *rc.borrow())))
        || (is_same_v_196.with(|rc| *rc.borrow()));
    let mut idx_l_convertible: () = (idx_convertible) && (l_convertible);
    ({ test_conversion_24(unimplemented!()) });
    ({ test_conversion_25(unimplemented!()) });
    ({ test_conversion_25(unimplemented!()) });
    ({ test_conversion_26(unimplemented!()) });
    ({ test_conversion_27(unimplemented!()) });
    ({ test_conversion_28(unimplemented!()) });
    ({ test_conversion_29(unimplemented!()) });
    ({ test_conversion_30(unimplemented!()) });
    ({ test_conversion_31(unimplemented!()) });
    ({ test_conversion_32(unimplemented!()) });
    ({ test_conversion_33(unimplemented!()) });
    ({ test_conversion_34(unimplemented!()) });
    ({ test_conversion_35(unimplemented!()) });
}
pub fn test_conversion_199() {
    let mut D: () = dynamic_extent_193.with(|rc| *rc.borrow());
    let mut idx_convertible: () = ((unimplemented!() as ()) >= unimplemented!());
    let mut l_convertible: () = ((is_same_v_194.with(|rc| *rc.borrow()))
        || (is_same_v_195.with(|rc| *rc.borrow())))
        || (is_same_v_196.with(|rc| *rc.borrow()));
    let mut idx_l_convertible: () = (idx_convertible) && (l_convertible);
    ({ test_conversion_36(unimplemented!()) });
    ({ test_conversion_37(unimplemented!()) });
    ({ test_conversion_37(unimplemented!()) });
    ({ test_conversion_38(unimplemented!()) });
    ({ test_conversion_39(unimplemented!()) });
    ({ test_conversion_40(unimplemented!()) });
    ({ test_conversion_41(unimplemented!()) });
    ({ test_conversion_42(unimplemented!()) });
    ({ test_conversion_43(unimplemented!()) });
    ({ test_conversion_44(unimplemented!()) });
    ({ test_conversion_45(unimplemented!()) });
    ({ test_conversion_46(unimplemented!()) });
    ({ test_conversion_47(unimplemented!()) });
}
pub fn test_conversion_200() {
    let mut D: () = dynamic_extent_193.with(|rc| *rc.borrow());
    let mut idx_convertible: () = (unimplemented!() >= unimplemented!());
    let mut l_convertible: () = ((is_same_v_201.with(|rc| *rc.borrow()))
        || (is_same_v_202.with(|rc| *rc.borrow())))
        || (is_same_v_203.with(|rc| *rc.borrow()));
    let mut idx_l_convertible: () = (idx_convertible) && (l_convertible);
    ({ test_conversion_48(unimplemented!()) });
    ({ test_conversion_49(unimplemented!()) });
    ({ test_conversion_49(unimplemented!()) });
    ({ test_conversion_50(unimplemented!()) });
    ({ test_conversion_51(unimplemented!()) });
    ({ test_conversion_52(unimplemented!()) });
    ({ test_conversion_53(unimplemented!()) });
    ({ test_conversion_54(unimplemented!()) });
    ({ test_conversion_55(unimplemented!()) });
    ({ test_conversion_56(unimplemented!()) });
    ({ test_conversion_57(unimplemented!()) });
    ({ test_conversion_58(unimplemented!()) });
    ({ test_conversion_59(unimplemented!()) });
}
pub fn test_conversion_204() {
    let mut D: () = dynamic_extent_193.with(|rc| *rc.borrow());
    let mut idx_convertible: () = (unimplemented!() >= (unimplemented!() as ()));
    let mut l_convertible: () = ((is_same_v_201.with(|rc| *rc.borrow()))
        || (is_same_v_202.with(|rc| *rc.borrow())))
        || (is_same_v_203.with(|rc| *rc.borrow()));
    let mut idx_l_convertible: () = (idx_convertible) && (l_convertible);
    ({ test_conversion_60(unimplemented!()) });
    ({ test_conversion_61(unimplemented!()) });
    ({ test_conversion_61(unimplemented!()) });
    ({ test_conversion_62(unimplemented!()) });
    ({ test_conversion_63(unimplemented!()) });
    ({ test_conversion_64(unimplemented!()) });
    ({ test_conversion_65(unimplemented!()) });
    ({ test_conversion_66(unimplemented!()) });
    ({ test_conversion_67(unimplemented!()) });
    ({ test_conversion_68(unimplemented!()) });
    ({ test_conversion_69(unimplemented!()) });
    ({ test_conversion_70(unimplemented!()) });
    ({ test_conversion_71(unimplemented!()) });
}
pub fn test_conversion_205() {
    let mut D: () = dynamic_extent_193.with(|rc| *rc.borrow());
    let mut idx_convertible: () = ((unimplemented!() as ()) >= unimplemented!());
    let mut l_convertible: () = ((is_same_v_201.with(|rc| *rc.borrow()))
        || (is_same_v_202.with(|rc| *rc.borrow())))
        || (is_same_v_203.with(|rc| *rc.borrow()));
    let mut idx_l_convertible: () = (idx_convertible) && (l_convertible);
    ({ test_conversion_72(unimplemented!()) });
    ({ test_conversion_73(unimplemented!()) });
    ({ test_conversion_73(unimplemented!()) });
    ({ test_conversion_74(unimplemented!()) });
    ({ test_conversion_75(unimplemented!()) });
    ({ test_conversion_76(unimplemented!()) });
    ({ test_conversion_77(unimplemented!()) });
    ({ test_conversion_78(unimplemented!()) });
    ({ test_conversion_79(unimplemented!()) });
    ({ test_conversion_80(unimplemented!()) });
    ({ test_conversion_81(unimplemented!()) });
    ({ test_conversion_82(unimplemented!()) });
    ({ test_conversion_83(unimplemented!()) });
}
pub fn test_conversion_206() {
    let mut D: () = dynamic_extent_193.with(|rc| *rc.borrow());
    let mut idx_convertible: () = ((unimplemented!() as ()) >= unimplemented!());
    let mut l_convertible: () = ((is_same_v_201.with(|rc| *rc.borrow()))
        || (is_same_v_202.with(|rc| *rc.borrow())))
        || (is_same_v_203.with(|rc| *rc.borrow()));
    let mut idx_l_convertible: () = (idx_convertible) && (l_convertible);
    ({ test_conversion_84(unimplemented!()) });
    ({ test_conversion_85(unimplemented!()) });
    ({ test_conversion_85(unimplemented!()) });
    ({ test_conversion_86(unimplemented!()) });
    ({ test_conversion_87(unimplemented!()) });
    ({ test_conversion_88(unimplemented!()) });
    ({ test_conversion_89(unimplemented!()) });
    ({ test_conversion_90(unimplemented!()) });
    ({ test_conversion_91(unimplemented!()) });
    ({ test_conversion_92(unimplemented!()) });
    ({ test_conversion_93(unimplemented!()) });
    ({ test_conversion_94(unimplemented!()) });
    ({ test_conversion_95(unimplemented!()) });
}
pub fn test_conversion_207() {
    let mut D: () = dynamic_extent_193.with(|rc| *rc.borrow());
    let mut idx_convertible: () = (unimplemented!() >= unimplemented!());
    let mut l_convertible: () = ((is_same_v_208.with(|rc| *rc.borrow()))
        || (is_same_v_209.with(|rc| *rc.borrow())))
        || (is_same_v_210.with(|rc| *rc.borrow()));
    let mut idx_l_convertible: () = (idx_convertible) && (l_convertible);
    ({ test_conversion_96(unimplemented!()) });
    ({ test_conversion_97(unimplemented!()) });
    ({ test_conversion_97(unimplemented!()) });
    ({ test_conversion_98(unimplemented!()) });
    ({ test_conversion_99(unimplemented!()) });
    ({ test_conversion_100(unimplemented!()) });
    ({ test_conversion_101(unimplemented!()) });
    ({ test_conversion_102(unimplemented!()) });
    ({ test_conversion_103(unimplemented!()) });
    ({ test_conversion_104(unimplemented!()) });
    ({ test_conversion_105(unimplemented!()) });
    ({ test_conversion_106(unimplemented!()) });
    ({ test_conversion_107(unimplemented!()) });
}
pub fn test_conversion_211() {
    let mut D: () = dynamic_extent_193.with(|rc| *rc.borrow());
    let mut idx_convertible: () = (unimplemented!() >= (unimplemented!() as ()));
    let mut l_convertible: () = ((is_same_v_208.with(|rc| *rc.borrow()))
        || (is_same_v_209.with(|rc| *rc.borrow())))
        || (is_same_v_210.with(|rc| *rc.borrow()));
    let mut idx_l_convertible: () = (idx_convertible) && (l_convertible);
    ({ test_conversion_108(unimplemented!()) });
    ({ test_conversion_109(unimplemented!()) });
    ({ test_conversion_109(unimplemented!()) });
    ({ test_conversion_110(unimplemented!()) });
    ({ test_conversion_111(unimplemented!()) });
    ({ test_conversion_112(unimplemented!()) });
    ({ test_conversion_113(unimplemented!()) });
    ({ test_conversion_114(unimplemented!()) });
    ({ test_conversion_115(unimplemented!()) });
    ({ test_conversion_116(unimplemented!()) });
    ({ test_conversion_117(unimplemented!()) });
    ({ test_conversion_118(unimplemented!()) });
    ({ test_conversion_119(unimplemented!()) });
}
pub fn test_conversion_212() {
    let mut D: () = dynamic_extent_193.with(|rc| *rc.borrow());
    let mut idx_convertible: () = ((unimplemented!() as ()) >= unimplemented!());
    let mut l_convertible: () = ((is_same_v_208.with(|rc| *rc.borrow()))
        || (is_same_v_209.with(|rc| *rc.borrow())))
        || (is_same_v_210.with(|rc| *rc.borrow()));
    let mut idx_l_convertible: () = (idx_convertible) && (l_convertible);
    ({ test_conversion_120(unimplemented!()) });
    ({ test_conversion_121(unimplemented!()) });
    ({ test_conversion_121(unimplemented!()) });
    ({ test_conversion_122(unimplemented!()) });
    ({ test_conversion_123(unimplemented!()) });
    ({ test_conversion_124(unimplemented!()) });
    ({ test_conversion_125(unimplemented!()) });
    ({ test_conversion_126(unimplemented!()) });
    ({ test_conversion_127(unimplemented!()) });
    ({ test_conversion_128(unimplemented!()) });
    ({ test_conversion_129(unimplemented!()) });
    ({ test_conversion_130(unimplemented!()) });
    ({ test_conversion_131(unimplemented!()) });
}
pub fn test_conversion_213() {
    let mut D: () = dynamic_extent_193.with(|rc| *rc.borrow());
    let mut idx_convertible: () = ((unimplemented!() as ()) >= unimplemented!());
    let mut l_convertible: () = ((is_same_v_208.with(|rc| *rc.borrow()))
        || (is_same_v_209.with(|rc| *rc.borrow())))
        || (is_same_v_210.with(|rc| *rc.borrow()));
    let mut idx_l_convertible: () = (idx_convertible) && (l_convertible);
    ({ test_conversion_132(unimplemented!()) });
    ({ test_conversion_133(unimplemented!()) });
    ({ test_conversion_133(unimplemented!()) });
    ({ test_conversion_134(unimplemented!()) });
    ({ test_conversion_135(unimplemented!()) });
    ({ test_conversion_136(unimplemented!()) });
    ({ test_conversion_137(unimplemented!()) });
    ({ test_conversion_138(unimplemented!()) });
    ({ test_conversion_139(unimplemented!()) });
    ({ test_conversion_140(unimplemented!()) });
    ({ test_conversion_141(unimplemented!()) });
    ({ test_conversion_142(unimplemented!()) });
    ({ test_conversion_143(unimplemented!()) });
}
pub fn test_conversion_214() {
    let mut D: () = dynamic_extent_193.with(|rc| *rc.borrow());
    let mut idx_convertible: () = (unimplemented!() >= unimplemented!());
    let mut l_convertible: () = ((is_same_v_215.with(|rc| *rc.borrow()))
        || (is_same_v_216.with(|rc| *rc.borrow())))
        || (is_same_v_217.with(|rc| *rc.borrow()));
    let mut idx_l_convertible: () = (idx_convertible) && (l_convertible);
    ({ test_conversion_144(unimplemented!()) });
    ({ test_conversion_145(unimplemented!()) });
    ({ test_conversion_145(unimplemented!()) });
    ({ test_conversion_146(unimplemented!()) });
    ({ test_conversion_147(unimplemented!()) });
    ({ test_conversion_148(unimplemented!()) });
    ({ test_conversion_149(unimplemented!()) });
    ({ test_conversion_150(unimplemented!()) });
    ({ test_conversion_151(unimplemented!()) });
    ({ test_conversion_152(unimplemented!()) });
    ({ test_conversion_153(unimplemented!()) });
    ({ test_conversion_154(unimplemented!()) });
    ({ test_conversion_155(unimplemented!()) });
}
pub fn test_conversion_218() {
    let mut D: () = dynamic_extent_193.with(|rc| *rc.borrow());
    let mut idx_convertible: () = (unimplemented!() >= (unimplemented!() as ()));
    let mut l_convertible: () = ((is_same_v_215.with(|rc| *rc.borrow()))
        || (is_same_v_216.with(|rc| *rc.borrow())))
        || (is_same_v_217.with(|rc| *rc.borrow()));
    let mut idx_l_convertible: () = (idx_convertible) && (l_convertible);
    ({ test_conversion_156(unimplemented!()) });
    ({ test_conversion_157(unimplemented!()) });
    ({ test_conversion_157(unimplemented!()) });
    ({ test_conversion_158(unimplemented!()) });
    ({ test_conversion_159(unimplemented!()) });
    ({ test_conversion_160(unimplemented!()) });
    ({ test_conversion_161(unimplemented!()) });
    ({ test_conversion_162(unimplemented!()) });
    ({ test_conversion_163(unimplemented!()) });
    ({ test_conversion_164(unimplemented!()) });
    ({ test_conversion_165(unimplemented!()) });
    ({ test_conversion_166(unimplemented!()) });
    ({ test_conversion_167(unimplemented!()) });
}
pub fn test_conversion_219() {
    let mut D: () = dynamic_extent_193.with(|rc| *rc.borrow());
    let mut idx_convertible: () = ((unimplemented!() as ()) >= unimplemented!());
    let mut l_convertible: () = ((is_same_v_215.with(|rc| *rc.borrow()))
        || (is_same_v_216.with(|rc| *rc.borrow())))
        || (is_same_v_217.with(|rc| *rc.borrow()));
    let mut idx_l_convertible: () = (idx_convertible) && (l_convertible);
    ({ test_conversion_168(unimplemented!()) });
    ({ test_conversion_169(unimplemented!()) });
    ({ test_conversion_169(unimplemented!()) });
    ({ test_conversion_170(unimplemented!()) });
    ({ test_conversion_171(unimplemented!()) });
    ({ test_conversion_172(unimplemented!()) });
    ({ test_conversion_173(unimplemented!()) });
    ({ test_conversion_174(unimplemented!()) });
    ({ test_conversion_175(unimplemented!()) });
    ({ test_conversion_176(unimplemented!()) });
    ({ test_conversion_177(unimplemented!()) });
    ({ test_conversion_178(unimplemented!()) });
    ({ test_conversion_179(unimplemented!()) });
}
pub fn test_conversion_220() {
    let mut D: () = dynamic_extent_193.with(|rc| *rc.borrow());
    let mut idx_convertible: () = ((unimplemented!() as ()) >= unimplemented!());
    let mut l_convertible: () = ((is_same_v_215.with(|rc| *rc.borrow()))
        || (is_same_v_216.with(|rc| *rc.borrow())))
        || (is_same_v_217.with(|rc| *rc.borrow()));
    let mut idx_l_convertible: () = (idx_convertible) && (l_convertible);
    ({ test_conversion_180(unimplemented!()) });
    ({ test_conversion_181(unimplemented!()) });
    ({ test_conversion_181(unimplemented!()) });
    ({ test_conversion_182(unimplemented!()) });
    ({ test_conversion_183(unimplemented!()) });
    ({ test_conversion_184(unimplemented!()) });
    ({ test_conversion_185(unimplemented!()) });
    ({ test_conversion_186(unimplemented!()) });
    ({ test_conversion_187(unimplemented!()) });
    ({ test_conversion_188(unimplemented!()) });
    ({ test_conversion_189(unimplemented!()) });
    ({ test_conversion_190(unimplemented!()) });
    ({ test_conversion_191(unimplemented!()) });
}
pub fn test_no_implicit_conversion_221() {
    let mut D: () = dynamic_extent_193.with(|rc| *rc.borrow());
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, D>, FromM<FromL, int, 5>>"
    );;
    const _: () = assert!(
        true,
        "std::is_convertible_v<FromM<FromL, int, 5>, ToM<int, D>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, 5>, FromM<FromL, int, D>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_convertible_v<FromM<FromL, int, D>, ToM<int, 5>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, D, 7>, FromM<FromL, int, 5, 7>>"
    );;
    const _: () = assert!(
        true,
        "std::is_convertible_v<FromM<FromL, int, 5, 7>, ToM<int, D, 7>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, 5, 7>, FromM<FromL, int, D, 7>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_convertible_v<FromM<FromL, int, D, 7>, ToM<int, 5, 7>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<size_t, 5>, FromM<FromL, int, 5>>"
    );;
    const _: () = assert!(
        true,
        "std::is_convertible_v<FromM<FromL, int, 5>, ToM<size_t, 5>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, 5>, FromM<FromL, size_t, 5>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_convertible_v<FromM<FromL, size_t, 5>, ToM<int, 5>>"
    );;
}
pub fn test_no_implicit_conversion_222() {
    let mut D: () = dynamic_extent_193.with(|rc| *rc.borrow());
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, D>, FromM<FromL, int, 5>>"
    );;
    const _: () = assert!(
        true,
        "std::is_convertible_v<FromM<FromL, int, 5>, ToM<int, D>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, 5>, FromM<FromL, int, D>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_convertible_v<FromM<FromL, int, D>, ToM<int, 5>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, D, 7>, FromM<FromL, int, 5, 7>>"
    );;
    const _: () = assert!(
        true,
        "std::is_convertible_v<FromM<FromL, int, 5, 7>, ToM<int, D, 7>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, 5, 7>, FromM<FromL, int, D, 7>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_convertible_v<FromM<FromL, int, D, 7>, ToM<int, 5, 7>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<size_t, 5>, FromM<FromL, int, 5>>"
    );;
    const _: () = assert!(
        true,
        "std::is_convertible_v<FromM<FromL, int, 5>, ToM<size_t, 5>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, 5>, FromM<FromL, size_t, 5>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_convertible_v<FromM<FromL, size_t, 5>, ToM<int, 5>>"
    );;
}
pub fn test_no_implicit_conversion_223() {
    let mut D: () = dynamic_extent_193.with(|rc| *rc.borrow());
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, D>, FromM<FromL, int, 5>>"
    );;
    const _: () = assert!(
        true,
        "std::is_convertible_v<FromM<FromL, int, 5>, ToM<int, D>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, 5>, FromM<FromL, int, D>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_convertible_v<FromM<FromL, int, D>, ToM<int, 5>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, D, 7>, FromM<FromL, int, 5, 7>>"
    );;
    const _: () = assert!(
        true,
        "std::is_convertible_v<FromM<FromL, int, 5, 7>, ToM<int, D, 7>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, 5, 7>, FromM<FromL, int, D, 7>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_convertible_v<FromM<FromL, int, D, 7>, ToM<int, 5, 7>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<size_t, 5>, FromM<FromL, int, 5>>"
    );;
    const _: () = assert!(
        true,
        "std::is_convertible_v<FromM<FromL, int, 5>, ToM<size_t, 5>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, 5>, FromM<FromL, size_t, 5>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_convertible_v<FromM<FromL, size_t, 5>, ToM<int, 5>>"
    );;
}
pub fn test_rank_mismatch_224() {
    let mut D: () = dynamic_extent_193.with(|rc| *rc.borrow());
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D>, FromM<FromL, int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int>, FromM<FromL, int, D, D>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D>, FromM<FromL, int, D, D>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D, D, D>, FromM<FromL, int, D, D>>"
    );;
}
pub fn test_rank_mismatch_225() {
    let mut D: () = dynamic_extent_193.with(|rc| *rc.borrow());
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D>, FromM<FromL, int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int>, FromM<FromL, int, D, D>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D>, FromM<FromL, int, D, D>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D, D, D>, FromM<FromL, int, D, D>>"
    );;
}
pub fn test_rank_mismatch_226() {
    let mut D: () = dynamic_extent_193.with(|rc| *rc.borrow());
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D>, FromM<FromL, int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int>, FromM<FromL, int, D, D>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D>, FromM<FromL, int, D, D>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D, D, D>, FromM<FromL, int, D, D>>"
    );;
}
pub fn test_rank_mismatch_227() {
    let mut D: () = dynamic_extent_193.with(|rc| *rc.borrow());
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D>, FromM<FromL, int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int>, FromM<FromL, int, D, D>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D>, FromM<FromL, int, D, D>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D, D, D>, FromM<FromL, int, D, D>>"
    );;
}
pub fn test_static_extent_mismatch_228() {
    let mut D: () = dynamic_extent_193.with(|rc| *rc.borrow());
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D, 5>, FromM<FromL, int, D, 4>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, 5>, FromM<FromL, int, 4>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, 5, D>, FromM<FromL, int, 4, D>>"
    );;
}
pub fn test_static_extent_mismatch_229() {
    let mut D: () = dynamic_extent_193.with(|rc| *rc.borrow());
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D, 5>, FromM<FromL, int, D, 4>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, 5>, FromM<FromL, int, 4>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, 5, D>, FromM<FromL, int, 4, D>>"
    );;
}
pub fn test_static_extent_mismatch_230() {
    let mut D: () = dynamic_extent_193.with(|rc| *rc.borrow());
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D, 5>, FromM<FromL, int, D, 4>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, 5>, FromM<FromL, int, 4>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, 5, D>, FromM<FromL, int, 4, D>>"
    );;
}
pub fn test_static_extent_mismatch_231() {
    let mut D: () = dynamic_extent_193.with(|rc| *rc.borrow());
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D, 5>, FromM<FromL, int, D, 4>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, 5>, FromM<FromL, int, 4>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, 5, D>, FromM<FromL, int, 4, D>>"
    );;
}
pub fn test_layout_232() {
    ({ test_conversion_192() });
    ({ test_conversion_197() });
    ({ test_conversion_198() });
    ({ test_conversion_199() });
    {
        ({ test_no_implicit_conversion_221() });
    }
    ({ test_rank_mismatch_224() });
    ({ test_static_extent_mismatch_228() });
}
pub fn test_layout_233() {
    ({ test_conversion_200() });
    ({ test_conversion_204() });
    ({ test_conversion_205() });
    ({ test_conversion_206() });
    {
        ({ test_no_implicit_conversion_222() });
    }
    ({ test_rank_mismatch_225() });
    ({ test_static_extent_mismatch_229() });
}
pub fn test_layout_234() {
    ({ test_conversion_207() });
    ({ test_conversion_211() });
    ({ test_conversion_212() });
    ({ test_conversion_213() });
    {
        ({ test_no_implicit_conversion_223() });
    }
    ({ test_rank_mismatch_226() });
    ({ test_static_extent_mismatch_230() });
}
pub fn test_layout_235() {
    ({ test_conversion_214() });
    ({ test_conversion_218() });
    ({ test_conversion_219() });
    ({ test_conversion_220() });
    {}
    ({ test_rank_mismatch_227() });
    ({ test_static_extent_mismatch_231() });
}
pub fn test_236() -> () {
    ({ test_layout_232() });
    ({ test_layout_233() });
    ({ test_layout_234() });
    ({ test_layout_235() });
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
    ({ test_236() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait always_convertible_layout_mapping_std_extents_int__Impl {
    fn copy_assign(
        &self,
        other: Ptr<always_convertible_layout_mapping_std_extents_int__>,
    ) -> Ptr<always_convertible_layout_mapping_std_extents_int__> {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn stride(&self, mut r: ()) -> () {
        unimplemented!()
    }
    fn operator_call_9(&self) -> ();
}
impl always_convertible_layout_mapping_std_extents_int__Impl
    for Ptr<always_convertible_layout_mapping_std_extents_int__>
{
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
    fn operator_call_9(&self) -> () {
        let idx_a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return ((*self).with(|__s| __s.offset_)
            + ((*self).with(|__s| __s.scaling_)
                * ({
                    lambda!(
                        Generic,
                        {
                            let this_: Value<
                                Ptr<always_convertible_layout_mapping_std_extents_int__>,
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
                })));
    }
}
pub trait always_convertible_layout_mapping_std_extents_int__18446744073709551615__Impl {
    fn copy_assign(
        &self,
        other: Ptr<always_convertible_layout_mapping_std_extents_int__18446744073709551615__>,
    ) -> Ptr<always_convertible_layout_mapping_std_extents_int__18446744073709551615__> {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int(&self, idx: ()) -> ();
}
impl always_convertible_layout_mapping_std_extents_int__18446744073709551615__Impl
    for Ptr<always_convertible_layout_mapping_std_extents_int__18446744073709551615__>
{
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
    fn required_span_size(&self) -> () {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            size *= unimplemented!();
            r.postfix_inc();
        }
        return (unimplemented!().read());
    }
    fn stride(&self, mut r: ()) -> () {
        let mut s: () = 1_();
        let mut i: () = 0_();
        'loop_: while (i < r) {
            s *= unimplemented!();
            i.postfix_inc();
        }
        return (s * (*self).with(|__s| __s.scaling_));
    }
    fn operator_call_9_int(&self, mut idx: ()) -> () {
        let idx_a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return ((*self).with(|__s| __s.offset_)
            + ((*self).with(|__s| __s.scaling_)
                * ({
                    lambda! ( Generic , { let this_ : Value<Ptr< always_convertible_layout_mapping_std_extents_int__18446744073709551615__  > > = Rc::new(RefCell::new(((* self ) ).clone())) ;
 let idx_a : Ptr< ()  > = idx_a .as_pointer()  ;
 } , | _a0 : ()  , | -> ()  { let _a0 : Value<() >  = Rc::new(RefCell::new(_a0)) ;
 let mut res : ()  = 0_()  ;
  ;
 ;
 let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 return res   ;
 } )  .spec::<fn(()  ,) -> ()  >(0) .call ( <()  >::default()  , )
                })));
    }
}
pub trait always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__Impl
{
    fn copy_assign ( &self , other : Ptr< always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__  > , ) -> Ptr< always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__  >{
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int_int(&self, idx_0: (), idx_1: ()) -> ();
}
impl always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__Impl for Ptr<always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__> {
fn extents ( &self , ) -> Ptr< ()  > { return field_ptr!((* self )  , extents_)   ;
 } fn required_span_size ( &self , ) -> ()  { let mut size : ()  = 1_()  ;
  ;
 ;
 let mut r : ()  = 0_()  ;
  ;
 ;
 'loop_: while ( r < unimplemented!()  ) { size  *= unimplemented!()   ;
 r  . postfix_inc () ;
 } return (unimplemented!() .read())   ;
 } fn stride ( &self , mut r : ()  , ) -> ()  { let mut s : ()  = 1_()  ;
  ;
 ;
 let mut i : ()  = 0_()  ;
  ;
 ;
 'loop_: while ( i < r ) { s  *= unimplemented!()   ;
 i  . postfix_inc () ;
 } return ( s * (* self ) .with(|__s| __s . scaling_ ) )   ;
 } fn operator_call_9_int_int ( &self , mut idx_0 : ()  , mut idx_1 : ()  , ) -> ()  { let idx_a : Value<() > = Rc::new(RefCell::new(unimplemented!() )) ;
  ;
 ;
 return ( (* self ) .with(|__s| __s . offset_ ) + ( (* self ) .with(|__s| __s . scaling_ ) * ( ( { lambda! ( Generic , { let this_ : Value<Ptr< always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__  > > = Rc::new(RefCell::new(((* self ) ).clone())) ;
 let idx_a : Ptr< ()  > = idx_a .as_pointer()  ;
 } , | _a0 : ()  , | -> ()  { let _a0 : Value<() >  = Rc::new(RefCell::new(_a0)) ;
 let mut res : ()  = 0_()  ;
  ;
 ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs } ;
 return res   ;
 } )  .spec::<fn(()  ,) -> ()  >(0) .call ( <()  >::default()  , ) } ) ) ) )   ;
 } }
pub trait always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__Impl
{
    fn copy_assign ( &self , other : Ptr< always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__  > , ) -> Ptr< always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__  >{
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int_int_int_int_int(
        &self,
        idx_0: (),
        idx_1: (),
        idx_2: (),
        idx_3: (),
        idx_4: (),
    ) -> ();
}
impl always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__Impl for Ptr<always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__> {
fn extents ( &self , ) -> Ptr< ()  > { return field_ptr!((* self )  , extents_)   ;
 } fn required_span_size ( &self , ) -> ()  { let mut size : ()  = 1_()  ;
  ;
 ;
 let mut r : ()  = 0_()  ;
  ;
 ;
 'loop_: while ( r < unimplemented!()  ) { size  *= unimplemented!()   ;
 r  . postfix_inc () ;
 } return (unimplemented!() .read())   ;
 } fn stride ( &self , mut r : ()  , ) -> ()  { let mut s : ()  = 1_()  ;
  ;
 ;
 let mut i : ()  = 0_()  ;
  ;
 ;
 'loop_: while ( i < r ) { s  *= unimplemented!()   ;
 i  . postfix_inc () ;
 } return ( s * (* self ) .with(|__s| __s . scaling_ ) )   ;
 } fn operator_call_9_int_int_int_int_int ( &self , mut idx_0 : ()  , mut idx_1 : ()  , mut idx_2 : ()  , mut idx_3 : ()  , mut idx_4 : ()  , ) -> ()  { let idx_a : Value<() > = Rc::new(RefCell::new(unimplemented!() )) ;
  ;
 ;
 return ( (* self ) .with(|__s| __s . offset_ ) + ( (* self ) .with(|__s| __s . scaling_ ) * ( ( { lambda! ( Generic , { let this_ : Value<Ptr< always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__  > > = Rc::new(RefCell::new(((* self ) ).clone())) ;
 let idx_a : Ptr< ()  > = idx_a .as_pointer()  ;
 } , | _a0 : ()  , | -> ()  { let _a0 : Value<() >  = Rc::new(RefCell::new(_a0)) ;
 let mut res : ()  = 0_()  ;
  ;
 ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs } } } } ;
 return res   ;
 } )  .spec::<fn(()  ,) -> ()  >(0) .call ( <()  >::default()  , ) } ) ) ) )   ;
 } }
pub trait always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__8__9__1__Impl
{
    fn copy_assign ( &self , other : Ptr< always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__8__9__1__  > , ) -> Ptr< always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__8__9__1__  >{
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int_int_int_int_int(
        &self,
        idx_0: (),
        idx_1: (),
        idx_2: (),
        idx_3: (),
        idx_4: (),
    ) -> ();
}
impl always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__8__9__1__Impl for Ptr<always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__8__9__1__> {
fn extents ( &self , ) -> Ptr< ()  > { return field_ptr!((* self )  , extents_)   ;
 } fn required_span_size ( &self , ) -> ()  { let mut size : ()  = 1_()  ;
  ;
 ;
 let mut r : ()  = 0_()  ;
  ;
 ;
 'loop_: while ( r < unimplemented!()  ) { size  *= unimplemented!()   ;
 r  . postfix_inc () ;
 } return (unimplemented!() .read())   ;
 } fn stride ( &self , mut r : ()  , ) -> ()  { let mut s : ()  = 1_()  ;
  ;
 ;
 let mut i : ()  = 0_()  ;
  ;
 ;
 'loop_: while ( i < r ) { s  *= unimplemented!()   ;
 i  . postfix_inc () ;
 } return ( s * (* self ) .with(|__s| __s . scaling_ ) )   ;
 } fn operator_call_9_int_int_int_int_int ( &self , mut idx_0 : ()  , mut idx_1 : ()  , mut idx_2 : ()  , mut idx_3 : ()  , mut idx_4 : ()  , ) -> ()  { let idx_a : Value<() > = Rc::new(RefCell::new(unimplemented!() )) ;
  ;
 ;
 return ( (* self ) .with(|__s| __s . offset_ ) + ( (* self ) .with(|__s| __s . scaling_ ) * ( ( { lambda! ( Generic , { let this_ : Value<Ptr< always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__8__9__1__  > > = Rc::new(RefCell::new(((* self ) ).clone())) ;
 let idx_a : Ptr< ()  > = idx_a .as_pointer()  ;
 } , | _a0 : ()  , | -> ()  { let _a0 : Value<() >  = Rc::new(RefCell::new(_a0)) ;
 let mut res : ()  = 0_()  ;
  ;
 ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs } } } } ;
 return res   ;
 } )  .spec::<fn(()  ,) -> ()  >(0) .call ( <()  >::default()  , ) } ) ) ) )   ;
 } }
pub trait always_convertible_layout_mapping_std_extents_int__18446744073709551615__4__Impl {
    fn copy_assign(
        &self,
        other: Ptr<always_convertible_layout_mapping_std_extents_int__18446744073709551615__4__>,
    ) -> Ptr<always_convertible_layout_mapping_std_extents_int__18446744073709551615__4__> {
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
}
impl always_convertible_layout_mapping_std_extents_int__18446744073709551615__4__Impl
    for Ptr<always_convertible_layout_mapping_std_extents_int__18446744073709551615__4__>
{
}
pub trait always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__Impl {
    fn copy_assign(
        &self,
        other: Ptr<always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__>,
    ) -> Ptr<always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__> {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int_int(&self, idx_0: (), idx_1: ()) -> ();
}
impl always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__Impl
    for Ptr<always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__>
{
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
    fn required_span_size(&self) -> () {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            size *= unimplemented!();
            r.postfix_inc();
        }
        return (unimplemented!().read());
    }
    fn stride(&self, mut r: ()) -> () {
        let mut s: () = 1_();
        let mut i: () = 0_();
        'loop_: while (i < r) {
            s *= unimplemented!();
            i.postfix_inc();
        }
        return (s * (*self).with(|__s| __s.scaling_));
    }
    fn operator_call_9_int_int(&self, mut idx_0: (), mut idx_1: ()) -> () {
        let idx_a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return ((*self).with(|__s| __s.offset_)
            + ((*self).with(|__s| __s.scaling_)
                * ({
                    lambda! ( Generic , { let this_ : Value<Ptr< always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__  > > = Rc::new(RefCell::new(((* self ) ).clone())) ;
 let idx_a : Ptr< ()  > = idx_a .as_pointer()  ;
 } , | _a0 : ()  , | -> ()  { let _a0 : Value<() >  = Rc::new(RefCell::new(_a0)) ;
 let mut res : ()  = 0_()  ;
  ;
 ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs } ;
 return res   ;
 } )  .spec::<fn(()  ,) -> ()  >(0) .call ( <()  >::default()  , )
                })));
    }
}
pub trait always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__8__9__1__Impl
{
    fn copy_assign(
        &self,
        other: Ptr<
            always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__8__9__1__,
        >,
    ) -> Ptr<always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__8__9__1__>
    {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int_int_int_int_int(
        &self,
        idx_0: (),
        idx_1: (),
        idx_2: (),
        idx_3: (),
        idx_4: (),
    ) -> ();
}
impl always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__8__9__1__Impl
    for Ptr<always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__8__9__1__>
{
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
    fn required_span_size(&self) -> () {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            size *= unimplemented!();
            r.postfix_inc();
        }
        return (unimplemented!().read());
    }
    fn stride(&self, mut r: ()) -> () {
        let mut s: () = 1_();
        let mut i: () = 0_();
        'loop_: while (i < r) {
            s *= unimplemented!();
            i.postfix_inc();
        }
        return (s * (*self).with(|__s| __s.scaling_));
    }
    fn operator_call_9_int_int_int_int_int(
        &self,
        mut idx_0: (),
        mut idx_1: (),
        mut idx_2: (),
        mut idx_3: (),
        mut idx_4: (),
    ) -> () {
        let idx_a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return ((*self).with(|__s| __s.offset_)
            + ((*self).with(|__s| __s.scaling_)
                * ({
                    lambda! ( Generic , { let this_ : Value<Ptr< always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__8__9__1__  > > = Rc::new(RefCell::new(((* self ) ).clone())) ;
 let idx_a : Ptr< ()  > = idx_a .as_pointer()  ;
 } , | _a0 : ()  , | -> ()  { let _a0 : Value<() >  = Rc::new(RefCell::new(_a0)) ;
 let mut res : ()  = 0_()  ;
  ;
 ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs } } } } ;
 return res   ;
 } )  .spec::<fn(()  ,) -> ()  >(0) .call ( <()  >::default()  , )
                })));
    }
}
pub trait always_convertible_layout_mapping_std_extents_int__4__Impl {
    fn copy_assign(
        &self,
        other: Ptr<always_convertible_layout_mapping_std_extents_int__4__>,
    ) -> Ptr<always_convertible_layout_mapping_std_extents_int__4__> {
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
}
impl always_convertible_layout_mapping_std_extents_int__4__Impl
    for Ptr<always_convertible_layout_mapping_std_extents_int__4__>
{
}
pub trait always_convertible_layout_mapping_std_extents_int__4__18446744073709551615__Impl {
    fn copy_assign(
        &self,
        other: Ptr<always_convertible_layout_mapping_std_extents_int__4__18446744073709551615__>,
    ) -> Ptr<always_convertible_layout_mapping_std_extents_int__4__18446744073709551615__> {
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
}
impl always_convertible_layout_mapping_std_extents_int__4__18446744073709551615__Impl
    for Ptr<always_convertible_layout_mapping_std_extents_int__4__18446744073709551615__>
{
}
pub trait always_convertible_layout_mapping_std_extents_int__5__Impl {
    fn copy_assign(
        &self,
        other: Ptr<always_convertible_layout_mapping_std_extents_int__5__>,
    ) -> Ptr<always_convertible_layout_mapping_std_extents_int__5__> {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int(&self, idx: ()) -> ();
}
impl always_convertible_layout_mapping_std_extents_int__5__Impl
    for Ptr<always_convertible_layout_mapping_std_extents_int__5__>
{
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
    fn required_span_size(&self) -> () {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            size *= unimplemented!();
            r.postfix_inc();
        }
        return (unimplemented!().read());
    }
    fn stride(&self, mut r: ()) -> () {
        let mut s: () = 1_();
        let mut i: () = 0_();
        'loop_: while (i < r) {
            s *= unimplemented!();
            i.postfix_inc();
        }
        return (s * (*self).with(|__s| __s.scaling_));
    }
    fn operator_call_9_int(&self, mut idx: ()) -> () {
        let idx_a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return ((*self).with(|__s| __s.offset_)
            + ((*self).with(|__s| __s.scaling_)
                * ({
                    lambda!(
                        Generic,
                        {
                            let this_: Value<
                                Ptr<always_convertible_layout_mapping_std_extents_int__5__>,
                            > = Rc::new(RefCell::new((*self).clone()));
                            let idx_a: Ptr<()> = idx_a.as_pointer();
                        },
                        |_a0: ()| -> () {
                            let _a0: Value<()> = Rc::new(RefCell::new(_a0));
                            let mut res: () = 0_();
                            let __rhs = ((unimplemented!().read()) + (unimplemented!() * res));
                            res = __rhs;
                            return res;
                        }
                    )
                    .spec::<fn(()) -> ()>(0)
                    .call(<()>::default())
                })));
    }
}
pub trait always_convertible_layout_mapping_std_extents_int__5__7__Impl {
    fn copy_assign(
        &self,
        other: Ptr<always_convertible_layout_mapping_std_extents_int__5__7__>,
    ) -> Ptr<always_convertible_layout_mapping_std_extents_int__5__7__> {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int_int(&self, idx_0: (), idx_1: ()) -> ();
}
impl always_convertible_layout_mapping_std_extents_int__5__7__Impl
    for Ptr<always_convertible_layout_mapping_std_extents_int__5__7__>
{
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
    fn required_span_size(&self) -> () {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            size *= unimplemented!();
            r.postfix_inc();
        }
        return (unimplemented!().read());
    }
    fn stride(&self, mut r: ()) -> () {
        let mut s: () = 1_();
        let mut i: () = 0_();
        'loop_: while (i < r) {
            s *= unimplemented!();
            i.postfix_inc();
        }
        return (s * (*self).with(|__s| __s.scaling_));
    }
    fn operator_call_9_int_int(&self, mut idx_0: (), mut idx_1: ()) -> () {
        let idx_a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return ((*self).with(|__s| __s.offset_)
            + ((*self).with(|__s| __s.scaling_)
                * ({
                    lambda!(
                        Generic,
                        {
                            let this_: Value<
                                Ptr<always_convertible_layout_mapping_std_extents_int__5__7__>,
                            > = Rc::new(RefCell::new((*self).clone()));
                            let idx_a: Ptr<()> = idx_a.as_pointer();
                        },
                        |_a0: ()| -> () {
                            let _a0: Value<()> = Rc::new(RefCell::new(_a0));
                            let mut res: () = 0_();
                            {
                                let __rhs = ((unimplemented!().read()) + (unimplemented!() * res));
                                res = __rhs;
                                let __rhs = ((unimplemented!().read()) + (unimplemented!() * res));
                                res = __rhs
                            };
                            return res;
                        }
                    )
                    .spec::<fn(()) -> ()>(0)
                    .call(<()>::default())
                })));
    }
}
pub trait always_convertible_layout_mapping_std_extents_int__5__7__8__9__1__Impl {
    fn copy_assign(
        &self,
        other: Ptr<always_convertible_layout_mapping_std_extents_int__5__7__8__9__1__>,
    ) -> Ptr<always_convertible_layout_mapping_std_extents_int__5__7__8__9__1__> {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int_int_int_int_int(
        &self,
        idx_0: (),
        idx_1: (),
        idx_2: (),
        idx_3: (),
        idx_4: (),
    ) -> ();
}
impl always_convertible_layout_mapping_std_extents_int__5__7__8__9__1__Impl
    for Ptr<always_convertible_layout_mapping_std_extents_int__5__7__8__9__1__>
{
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
    fn required_span_size(&self) -> () {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            size *= unimplemented!();
            r.postfix_inc();
        }
        return (unimplemented!().read());
    }
    fn stride(&self, mut r: ()) -> () {
        let mut s: () = 1_();
        let mut i: () = 0_();
        'loop_: while (i < r) {
            s *= unimplemented!();
            i.postfix_inc();
        }
        return (s * (*self).with(|__s| __s.scaling_));
    }
    fn operator_call_9_int_int_int_int_int(
        &self,
        mut idx_0: (),
        mut idx_1: (),
        mut idx_2: (),
        mut idx_3: (),
        mut idx_4: (),
    ) -> () {
        let idx_a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return ((*self).with(|__s| __s.offset_)
            + ((*self).with(|__s| __s.scaling_)
                * ({
                    lambda! ( Generic , { let this_ : Value<Ptr< always_convertible_layout_mapping_std_extents_int__5__7__8__9__1__  > > = Rc::new(RefCell::new(((* self ) ).clone())) ;
 let idx_a : Ptr< ()  > = idx_a .as_pointer()  ;
 } , | _a0 : ()  , | -> ()  { let _a0 : Value<() >  = Rc::new(RefCell::new(_a0)) ;
 let mut res : ()  = 0_()  ;
  ;
 ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs } } } } ;
 return res   ;
 } )  .spec::<fn(()  ,) -> ()  >(0) .call ( <()  >::default()  , )
                })));
    }
}
pub trait always_convertible_layout_mapping_std_extents_long__Impl {
    fn copy_assign(
        &self,
        other: Ptr<always_convertible_layout_mapping_std_extents_long__>,
    ) -> Ptr<always_convertible_layout_mapping_std_extents_long__> {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn stride(&self, mut r: ()) -> () {
        unimplemented!()
    }
    fn operator_call_9(&self) -> ();
}
impl always_convertible_layout_mapping_std_extents_long__Impl
    for Ptr<always_convertible_layout_mapping_std_extents_long__>
{
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
    fn operator_call_9(&self) -> () {
        let idx_a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return ((*self).with(|__s| __s.offset_)
            + ((*self).with(|__s| __s.scaling_)
                * ({
                    lambda!(
                        Generic,
                        {
                            let this_: Value<
                                Ptr<always_convertible_layout_mapping_std_extents_long__>,
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
                })));
    }
}
pub trait always_convertible_layout_mapping_std_extents_long__18446744073709551615__Impl {
    fn copy_assign(
        &self,
        other: Ptr<always_convertible_layout_mapping_std_extents_long__18446744073709551615__>,
    ) -> Ptr<always_convertible_layout_mapping_std_extents_long__18446744073709551615__> {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int(&self, idx: ()) -> ();
}
impl always_convertible_layout_mapping_std_extents_long__18446744073709551615__Impl
    for Ptr<always_convertible_layout_mapping_std_extents_long__18446744073709551615__>
{
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
    fn required_span_size(&self) -> () {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            size *= unimplemented!();
            r.postfix_inc();
        }
        return (unimplemented!().read());
    }
    fn stride(&self, mut r: ()) -> () {
        let mut s: () = 1_();
        let mut i: () = 0_();
        'loop_: while (i < r) {
            s *= unimplemented!();
            i.postfix_inc();
        }
        return (s * (*self).with(|__s| __s.scaling_));
    }
    fn operator_call_9_int(&self, mut idx: ()) -> () {
        let idx_a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return ((*self).with(|__s| __s.offset_)
            + ((*self).with(|__s| __s.scaling_)
                * ({
                    lambda! ( Generic , { let this_ : Value<Ptr< always_convertible_layout_mapping_std_extents_long__18446744073709551615__  > > = Rc::new(RefCell::new(((* self ) ).clone())) ;
 let idx_a : Ptr< ()  > = idx_a .as_pointer()  ;
 } , | _a0 : ()  , | -> ()  { let _a0 : Value<() >  = Rc::new(RefCell::new(_a0)) ;
 let mut res : ()  = 0_()  ;
  ;
 ;
 let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 return res   ;
 } )  .spec::<fn(()  ,) -> ()  >(0) .call ( <()  >::default()  , )
                })));
    }
}
pub trait always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__Impl
{
    fn copy_assign ( &self , other : Ptr< always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__  > , ) -> Ptr< always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__  >{
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int_int(&self, idx_0: (), idx_1: ()) -> ();
}
impl always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__Impl for Ptr<always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__> {
fn extents ( &self , ) -> Ptr< ()  > { return field_ptr!((* self )  , extents_)   ;
 } fn required_span_size ( &self , ) -> ()  { let mut size : ()  = 1_()  ;
  ;
 ;
 let mut r : ()  = 0_()  ;
  ;
 ;
 'loop_: while ( r < unimplemented!()  ) { size  *= unimplemented!()   ;
 r  . postfix_inc () ;
 } return (unimplemented!() .read())   ;
 } fn stride ( &self , mut r : ()  , ) -> ()  { let mut s : ()  = 1_()  ;
  ;
 ;
 let mut i : ()  = 0_()  ;
  ;
 ;
 'loop_: while ( i < r ) { s  *= unimplemented!()   ;
 i  . postfix_inc () ;
 } return ( s * (* self ) .with(|__s| __s . scaling_ ) )   ;
 } fn operator_call_9_int_int ( &self , mut idx_0 : ()  , mut idx_1 : ()  , ) -> ()  { let idx_a : Value<() > = Rc::new(RefCell::new(unimplemented!() )) ;
  ;
 ;
 return ( (* self ) .with(|__s| __s . offset_ ) + ( (* self ) .with(|__s| __s . scaling_ ) * ( ( { lambda! ( Generic , { let this_ : Value<Ptr< always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__  > > = Rc::new(RefCell::new(((* self ) ).clone())) ;
 let idx_a : Ptr< ()  > = idx_a .as_pointer()  ;
 } , | _a0 : ()  , | -> ()  { let _a0 : Value<() >  = Rc::new(RefCell::new(_a0)) ;
 let mut res : ()  = 0_()  ;
  ;
 ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs } ;
 return res   ;
 } )  .spec::<fn(()  ,) -> ()  >(0) .call ( <()  >::default()  , ) } ) ) ) )   ;
 } }
pub trait always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__Impl
{
    fn copy_assign ( &self , other : Ptr< always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__  > , ) -> Ptr< always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__  >{
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int_int_int_int_int(
        &self,
        idx_0: (),
        idx_1: (),
        idx_2: (),
        idx_3: (),
        idx_4: (),
    ) -> ();
}
impl always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__Impl for Ptr<always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__> {
fn extents ( &self , ) -> Ptr< ()  > { return field_ptr!((* self )  , extents_)   ;
 } fn required_span_size ( &self , ) -> ()  { let mut size : ()  = 1_()  ;
  ;
 ;
 let mut r : ()  = 0_()  ;
  ;
 ;
 'loop_: while ( r < unimplemented!()  ) { size  *= unimplemented!()   ;
 r  . postfix_inc () ;
 } return (unimplemented!() .read())   ;
 } fn stride ( &self , mut r : ()  , ) -> ()  { let mut s : ()  = 1_()  ;
  ;
 ;
 let mut i : ()  = 0_()  ;
  ;
 ;
 'loop_: while ( i < r ) { s  *= unimplemented!()   ;
 i  . postfix_inc () ;
 } return ( s * (* self ) .with(|__s| __s . scaling_ ) )   ;
 } fn operator_call_9_int_int_int_int_int ( &self , mut idx_0 : ()  , mut idx_1 : ()  , mut idx_2 : ()  , mut idx_3 : ()  , mut idx_4 : ()  , ) -> ()  { let idx_a : Value<() > = Rc::new(RefCell::new(unimplemented!() )) ;
  ;
 ;
 return ( (* self ) .with(|__s| __s . offset_ ) + ( (* self ) .with(|__s| __s . scaling_ ) * ( ( { lambda! ( Generic , { let this_ : Value<Ptr< always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__  > > = Rc::new(RefCell::new(((* self ) ).clone())) ;
 let idx_a : Ptr< ()  > = idx_a .as_pointer()  ;
 } , | _a0 : ()  , | -> ()  { let _a0 : Value<() >  = Rc::new(RefCell::new(_a0)) ;
 let mut res : ()  = 0_()  ;
  ;
 ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs } } } } ;
 return res   ;
 } )  .spec::<fn(()  ,) -> ()  >(0) .call ( <()  >::default()  , ) } ) ) ) )   ;
 } }
pub trait always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__8__9__1__Impl
{
    fn copy_assign ( &self , other : Ptr< always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__8__9__1__  > , ) -> Ptr< always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__8__9__1__  >{
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int_int_int_int_int(
        &self,
        idx_0: (),
        idx_1: (),
        idx_2: (),
        idx_3: (),
        idx_4: (),
    ) -> ();
}
impl always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__8__9__1__Impl for Ptr<always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__8__9__1__> {
fn extents ( &self , ) -> Ptr< ()  > { return field_ptr!((* self )  , extents_)   ;
 } fn required_span_size ( &self , ) -> ()  { let mut size : ()  = 1_()  ;
  ;
 ;
 let mut r : ()  = 0_()  ;
  ;
 ;
 'loop_: while ( r < unimplemented!()  ) { size  *= unimplemented!()   ;
 r  . postfix_inc () ;
 } return (unimplemented!() .read())   ;
 } fn stride ( &self , mut r : ()  , ) -> ()  { let mut s : ()  = 1_()  ;
  ;
 ;
 let mut i : ()  = 0_()  ;
  ;
 ;
 'loop_: while ( i < r ) { s  *= unimplemented!()   ;
 i  . postfix_inc () ;
 } return ( s * (* self ) .with(|__s| __s . scaling_ ) )   ;
 } fn operator_call_9_int_int_int_int_int ( &self , mut idx_0 : ()  , mut idx_1 : ()  , mut idx_2 : ()  , mut idx_3 : ()  , mut idx_4 : ()  , ) -> ()  { let idx_a : Value<() > = Rc::new(RefCell::new(unimplemented!() )) ;
  ;
 ;
 return ( (* self ) .with(|__s| __s . offset_ ) + ( (* self ) .with(|__s| __s . scaling_ ) * ( ( { lambda! ( Generic , { let this_ : Value<Ptr< always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__8__9__1__  > > = Rc::new(RefCell::new(((* self ) ).clone())) ;
 let idx_a : Ptr< ()  > = idx_a .as_pointer()  ;
 } , | _a0 : ()  , | -> ()  { let _a0 : Value<() >  = Rc::new(RefCell::new(_a0)) ;
 let mut res : ()  = 0_()  ;
  ;
 ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs } } } } ;
 return res   ;
 } )  .spec::<fn(()  ,) -> ()  >(0) .call ( <()  >::default()  , ) } ) ) ) )   ;
 } }
pub trait always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__Impl {
    fn copy_assign(
        &self,
        other: Ptr<always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__>,
    ) -> Ptr<always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__> {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int_int(&self, idx_0: (), idx_1: ()) -> ();
}
impl always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__Impl
    for Ptr<always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__>
{
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
    fn required_span_size(&self) -> () {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            size *= unimplemented!();
            r.postfix_inc();
        }
        return (unimplemented!().read());
    }
    fn stride(&self, mut r: ()) -> () {
        let mut s: () = 1_();
        let mut i: () = 0_();
        'loop_: while (i < r) {
            s *= unimplemented!();
            i.postfix_inc();
        }
        return (s * (*self).with(|__s| __s.scaling_));
    }
    fn operator_call_9_int_int(&self, mut idx_0: (), mut idx_1: ()) -> () {
        let idx_a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return ((*self).with(|__s| __s.offset_)
            + ((*self).with(|__s| __s.scaling_)
                * ({
                    lambda! ( Generic , { let this_ : Value<Ptr< always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__  > > = Rc::new(RefCell::new(((* self ) ).clone())) ;
 let idx_a : Ptr< ()  > = idx_a .as_pointer()  ;
 } , | _a0 : ()  , | -> ()  { let _a0 : Value<() >  = Rc::new(RefCell::new(_a0)) ;
 let mut res : ()  = 0_()  ;
  ;
 ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs } ;
 return res   ;
 } )  .spec::<fn(()  ,) -> ()  >(0) .call ( <()  >::default()  , )
                })));
    }
}
pub trait always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__8__9__1__Impl
{
    fn copy_assign(
        &self,
        other: Ptr<
            always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__8__9__1__,
        >,
    ) -> Ptr<always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__8__9__1__>
    {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int_int_int_int_int(
        &self,
        idx_0: (),
        idx_1: (),
        idx_2: (),
        idx_3: (),
        idx_4: (),
    ) -> ();
}
impl always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__8__9__1__Impl
    for Ptr<always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__8__9__1__>
{
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
    fn required_span_size(&self) -> () {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            size *= unimplemented!();
            r.postfix_inc();
        }
        return (unimplemented!().read());
    }
    fn stride(&self, mut r: ()) -> () {
        let mut s: () = 1_();
        let mut i: () = 0_();
        'loop_: while (i < r) {
            s *= unimplemented!();
            i.postfix_inc();
        }
        return (s * (*self).with(|__s| __s.scaling_));
    }
    fn operator_call_9_int_int_int_int_int(
        &self,
        mut idx_0: (),
        mut idx_1: (),
        mut idx_2: (),
        mut idx_3: (),
        mut idx_4: (),
    ) -> () {
        let idx_a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return ((*self).with(|__s| __s.offset_)
            + ((*self).with(|__s| __s.scaling_)
                * ({
                    lambda! ( Generic , { let this_ : Value<Ptr< always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__8__9__1__  > > = Rc::new(RefCell::new(((* self ) ).clone())) ;
 let idx_a : Ptr< ()  > = idx_a .as_pointer()  ;
 } , | _a0 : ()  , | -> ()  { let _a0 : Value<() >  = Rc::new(RefCell::new(_a0)) ;
 let mut res : ()  = 0_()  ;
  ;
 ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs } } } } ;
 return res   ;
 } )  .spec::<fn(()  ,) -> ()  >(0) .call ( <()  >::default()  , )
                })));
    }
}
pub trait always_convertible_layout_mapping_std_extents_long__5__Impl {
    fn copy_assign(
        &self,
        other: Ptr<always_convertible_layout_mapping_std_extents_long__5__>,
    ) -> Ptr<always_convertible_layout_mapping_std_extents_long__5__> {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int(&self, idx: ()) -> ();
}
impl always_convertible_layout_mapping_std_extents_long__5__Impl
    for Ptr<always_convertible_layout_mapping_std_extents_long__5__>
{
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
    fn required_span_size(&self) -> () {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            size *= unimplemented!();
            r.postfix_inc();
        }
        return (unimplemented!().read());
    }
    fn stride(&self, mut r: ()) -> () {
        let mut s: () = 1_();
        let mut i: () = 0_();
        'loop_: while (i < r) {
            s *= unimplemented!();
            i.postfix_inc();
        }
        return (s * (*self).with(|__s| __s.scaling_));
    }
    fn operator_call_9_int(&self, mut idx: ()) -> () {
        let idx_a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return ((*self).with(|__s| __s.offset_)
            + ((*self).with(|__s| __s.scaling_)
                * ({
                    lambda!(
                        Generic,
                        {
                            let this_: Value<
                                Ptr<always_convertible_layout_mapping_std_extents_long__5__>,
                            > = Rc::new(RefCell::new((*self).clone()));
                            let idx_a: Ptr<()> = idx_a.as_pointer();
                        },
                        |_a0: ()| -> () {
                            let _a0: Value<()> = Rc::new(RefCell::new(_a0));
                            let mut res: () = 0_();
                            let __rhs = ((unimplemented!().read()) + (unimplemented!() * res));
                            res = __rhs;
                            return res;
                        }
                    )
                    .spec::<fn(()) -> ()>(0)
                    .call(<()>::default())
                })));
    }
}
pub trait always_convertible_layout_mapping_std_extents_long__5__7__Impl {
    fn copy_assign(
        &self,
        other: Ptr<always_convertible_layout_mapping_std_extents_long__5__7__>,
    ) -> Ptr<always_convertible_layout_mapping_std_extents_long__5__7__> {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int_int(&self, idx_0: (), idx_1: ()) -> ();
}
impl always_convertible_layout_mapping_std_extents_long__5__7__Impl
    for Ptr<always_convertible_layout_mapping_std_extents_long__5__7__>
{
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
    fn required_span_size(&self) -> () {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            size *= unimplemented!();
            r.postfix_inc();
        }
        return (unimplemented!().read());
    }
    fn stride(&self, mut r: ()) -> () {
        let mut s: () = 1_();
        let mut i: () = 0_();
        'loop_: while (i < r) {
            s *= unimplemented!();
            i.postfix_inc();
        }
        return (s * (*self).with(|__s| __s.scaling_));
    }
    fn operator_call_9_int_int(&self, mut idx_0: (), mut idx_1: ()) -> () {
        let idx_a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return ((*self).with(|__s| __s.offset_)
            + ((*self).with(|__s| __s.scaling_)
                * ({
                    lambda!(
                        Generic,
                        {
                            let this_: Value<
                                Ptr<always_convertible_layout_mapping_std_extents_long__5__7__>,
                            > = Rc::new(RefCell::new((*self).clone()));
                            let idx_a: Ptr<()> = idx_a.as_pointer();
                        },
                        |_a0: ()| -> () {
                            let _a0: Value<()> = Rc::new(RefCell::new(_a0));
                            let mut res: () = 0_();
                            {
                                let __rhs = ((unimplemented!().read()) + (unimplemented!() * res));
                                res = __rhs;
                                let __rhs = ((unimplemented!().read()) + (unimplemented!() * res));
                                res = __rhs
                            };
                            return res;
                        }
                    )
                    .spec::<fn(()) -> ()>(0)
                    .call(<()>::default())
                })));
    }
}
pub trait always_convertible_layout_mapping_std_extents_long__5__7__8__9__1__Impl {
    fn copy_assign(
        &self,
        other: Ptr<always_convertible_layout_mapping_std_extents_long__5__7__8__9__1__>,
    ) -> Ptr<always_convertible_layout_mapping_std_extents_long__5__7__8__9__1__> {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int_int_int_int_int(
        &self,
        idx_0: (),
        idx_1: (),
        idx_2: (),
        idx_3: (),
        idx_4: (),
    ) -> ();
}
impl always_convertible_layout_mapping_std_extents_long__5__7__8__9__1__Impl
    for Ptr<always_convertible_layout_mapping_std_extents_long__5__7__8__9__1__>
{
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
    fn required_span_size(&self) -> () {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            size *= unimplemented!();
            r.postfix_inc();
        }
        return (unimplemented!().read());
    }
    fn stride(&self, mut r: ()) -> () {
        let mut s: () = 1_();
        let mut i: () = 0_();
        'loop_: while (i < r) {
            s *= unimplemented!();
            i.postfix_inc();
        }
        return (s * (*self).with(|__s| __s.scaling_));
    }
    fn operator_call_9_int_int_int_int_int(
        &self,
        mut idx_0: (),
        mut idx_1: (),
        mut idx_2: (),
        mut idx_3: (),
        mut idx_4: (),
    ) -> () {
        let idx_a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return ((*self).with(|__s| __s.offset_)
            + ((*self).with(|__s| __s.scaling_)
                * ({
                    lambda! ( Generic , { let this_ : Value<Ptr< always_convertible_layout_mapping_std_extents_long__5__7__8__9__1__  > > = Rc::new(RefCell::new(((* self ) ).clone())) ;
 let idx_a : Ptr< ()  > = idx_a .as_pointer()  ;
 } , | _a0 : ()  , | -> ()  { let _a0 : Value<() >  = Rc::new(RefCell::new(_a0)) ;
 let mut res : ()  = 0_()  ;
  ;
 ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs ;
 let __rhs = ( (unimplemented!() .read()) + ( unimplemented!()  * res ) )  ;
 res  = __rhs } } } } ;
 return res   ;
 } )  .spec::<fn(()  ,) -> ()  >(0) .call ( <()  >::default()  , )
                })));
    }
}
pub trait always_convertible_layout_mapping_std_extents_unsigned_long__Impl {
    fn copy_assign(
        &self,
        other: Ptr<always_convertible_layout_mapping_std_extents_unsigned_long__>,
    ) -> Ptr<always_convertible_layout_mapping_std_extents_unsigned_long__> {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn stride(&self, mut r: ()) -> () {
        unimplemented!()
    }
    fn operator_call_9(&self) -> ();
}
impl always_convertible_layout_mapping_std_extents_unsigned_long__Impl
    for Ptr<always_convertible_layout_mapping_std_extents_unsigned_long__>
{
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
    fn operator_call_9(&self) -> () {
        let idx_a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return ((*self).with(|__s| __s.offset_)).wrapping_add(
            ((*self).with(|__s| __s.scaling_)).wrapping_mul(
                ({
                    lambda!(
                        Generic,
                        {
                            let this_: Value<
                                Ptr<always_convertible_layout_mapping_std_extents_unsigned_long__>,
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
pub trait always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__Impl {
    fn copy_assign(
        &self,
        other: Ptr<
            always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__,
        >,
    ) -> Ptr<always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__>
    {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int(&self, idx: ()) -> ();
}
impl always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__Impl
    for Ptr<always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__>
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
        return ( (* self ) .with(|__s| __s . offset_ ) ) . wrapping_add ( ( (* self ) .with(|__s| __s . scaling_ ) ) . wrapping_mul ( ( ( { lambda! ( Generic , { let this_ : Value<Ptr< always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__  > > = Rc::new(RefCell::new(((* self ) ).clone())) ;
 let idx_a : Ptr< ()  > = idx_a .as_pointer()  ;
 } , | _a0 : ()  , | -> ()  { let _a0 : Value<() >  = Rc::new(RefCell::new(_a0)) ;
 let mut res : ()  = 0_()  ;
  ;
 ;
 let __rhs = ( (unimplemented!() .read()) ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = __rhs ;
 return res   ;
 } )  .spec::<fn(()  ,) -> ()  >(0) .call ( <()  >::default()  , ) } ) ) ) )   ;
    }
}
pub trait always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__Impl
{
    fn copy_assign ( &self , other : Ptr< always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__  > , ) -> Ptr< always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__  >{
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int_int(&self, idx_0: (), idx_1: ()) -> ();
}
impl always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__Impl for Ptr<always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__> {
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
 return ( (* self ) .with(|__s| __s . offset_ ) ) . wrapping_add ( ( (* self ) .with(|__s| __s . scaling_ ) ) . wrapping_mul ( ( ( { lambda! ( Generic , { let this_ : Value<Ptr< always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__  > > = Rc::new(RefCell::new(((* self ) ).clone())) ;
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
pub trait always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__Impl
{
    fn copy_assign ( &self , other : Ptr< always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__  > , ) -> Ptr< always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__  >{
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int_int_int_int_int(
        &self,
        idx_0: (),
        idx_1: (),
        idx_2: (),
        idx_3: (),
        idx_4: (),
    ) -> ();
}
impl always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__Impl for Ptr<always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__> {
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
 } fn operator_call_9_int_int_int_int_int ( &self , mut idx_0 : ()  , mut idx_1 : ()  , mut idx_2 : ()  , mut idx_3 : ()  , mut idx_4 : ()  , ) -> ()  { let idx_a : Value<() > = Rc::new(RefCell::new(unimplemented!() )) ;
  ;
 ;
 return ( (* self ) .with(|__s| __s . offset_ ) ) . wrapping_add ( ( (* self ) .with(|__s| __s . scaling_ ) ) . wrapping_mul ( ( ( { lambda! ( Generic , { let this_ : Value<Ptr< always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__  > > = Rc::new(RefCell::new(((* self ) ).clone())) ;
 let idx_a : Ptr< ()  > = idx_a .as_pointer()  ;
 } , | _a0 : ()  , | -> ()  { let _a0 : Value<() >  = Rc::new(RefCell::new(_a0)) ;
 let mut res : ()  = 0_()  ;
  ;
 ;
 { let __rhs = ( (unimplemented!() .read()) ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = __rhs ;
 let __rhs = ( (unimplemented!() .read()) ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = __rhs } } } } ;
 return res   ;
 } )  .spec::<fn(()  ,) -> ()  >(0) .call ( <()  >::default()  , ) } ) ) ) )   ;
 } }
pub trait always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__8__9__1__Impl
{
    fn copy_assign ( &self , other : Ptr< always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__8__9__1__  > , ) -> Ptr< always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__8__9__1__  >{
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int_int_int_int_int(
        &self,
        idx_0: (),
        idx_1: (),
        idx_2: (),
        idx_3: (),
        idx_4: (),
    ) -> ();
}
impl always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__8__9__1__Impl for Ptr<always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__8__9__1__> {
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
 } fn operator_call_9_int_int_int_int_int ( &self , mut idx_0 : ()  , mut idx_1 : ()  , mut idx_2 : ()  , mut idx_3 : ()  , mut idx_4 : ()  , ) -> ()  { let idx_a : Value<() > = Rc::new(RefCell::new(unimplemented!() )) ;
  ;
 ;
 return ( (* self ) .with(|__s| __s . offset_ ) ) . wrapping_add ( ( (* self ) .with(|__s| __s . scaling_ ) ) . wrapping_mul ( ( ( { lambda! ( Generic , { let this_ : Value<Ptr< always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__8__9__1__  > > = Rc::new(RefCell::new(((* self ) ).clone())) ;
 let idx_a : Ptr< ()  > = idx_a .as_pointer()  ;
 } , | _a0 : ()  , | -> ()  { let _a0 : Value<() >  = Rc::new(RefCell::new(_a0)) ;
 let mut res : ()  = 0_()  ;
  ;
 ;
 { let __rhs = ( (unimplemented!() .read()) ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = __rhs ;
 let __rhs = ( (unimplemented!() .read()) ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = __rhs } } } } ;
 return res   ;
 } )  .spec::<fn(()  ,) -> ()  >(0) .call ( <()  >::default()  , ) } ) ) ) )   ;
 } }
pub trait always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__Impl
{
    fn copy_assign(
        &self,
        other: Ptr<
            always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__,
        >,
    ) -> Ptr<always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__>
    {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int_int(&self, idx_0: (), idx_1: ()) -> ();
}
impl always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__Impl
    for Ptr<always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__>
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
    fn operator_call_9_int_int(&self, mut idx_0: (), mut idx_1: ()) -> () {
        let idx_a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return ( (* self ) .with(|__s| __s . offset_ ) ) . wrapping_add ( ( (* self ) .with(|__s| __s . scaling_ ) ) . wrapping_mul ( ( ( { lambda! ( Generic , { let this_ : Value<Ptr< always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__  > > = Rc::new(RefCell::new(((* self ) ).clone())) ;
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
    }
}
pub trait always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__8__9__1__Impl
{
    fn copy_assign ( &self , other : Ptr< always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__8__9__1__  > , ) -> Ptr< always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__8__9__1__  >{
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int_int_int_int_int(
        &self,
        idx_0: (),
        idx_1: (),
        idx_2: (),
        idx_3: (),
        idx_4: (),
    ) -> ();
}
impl always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__8__9__1__Impl for Ptr<always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__8__9__1__> {
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
 } fn operator_call_9_int_int_int_int_int ( &self , mut idx_0 : ()  , mut idx_1 : ()  , mut idx_2 : ()  , mut idx_3 : ()  , mut idx_4 : ()  , ) -> ()  { let idx_a : Value<() > = Rc::new(RefCell::new(unimplemented!() )) ;
  ;
 ;
 return ( (* self ) .with(|__s| __s . offset_ ) ) . wrapping_add ( ( (* self ) .with(|__s| __s . scaling_ ) ) . wrapping_mul ( ( ( { lambda! ( Generic , { let this_ : Value<Ptr< always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__8__9__1__  > > = Rc::new(RefCell::new(((* self ) ).clone())) ;
 let idx_a : Ptr< ()  > = idx_a .as_pointer()  ;
 } , | _a0 : ()  , | -> ()  { let _a0 : Value<() >  = Rc::new(RefCell::new(_a0)) ;
 let mut res : ()  = 0_()  ;
  ;
 ;
 { let __rhs = ( (unimplemented!() .read()) ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = __rhs ;
 let __rhs = ( (unimplemented!() .read()) ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = __rhs } } } } ;
 return res   ;
 } )  .spec::<fn(()  ,) -> ()  >(0) .call ( <()  >::default()  , ) } ) ) ) )   ;
 } }
pub trait always_convertible_layout_mapping_std_extents_unsigned_long__5__Impl {
    fn copy_assign(
        &self,
        other: Ptr<always_convertible_layout_mapping_std_extents_unsigned_long__5__>,
    ) -> Ptr<always_convertible_layout_mapping_std_extents_unsigned_long__5__> {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int(&self, idx: ()) -> ();
}
impl always_convertible_layout_mapping_std_extents_unsigned_long__5__Impl
    for Ptr<always_convertible_layout_mapping_std_extents_unsigned_long__5__>
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
        return ((*self).with(|__s| __s.offset_))
            .wrapping_add(((*self).with(|__s| __s.scaling_)).wrapping_mul(
            ({
                lambda!(
                    Generic,
                    {
                        let this_: Value<
                            Ptr<always_convertible_layout_mapping_std_extents_unsigned_long__5__>,
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
        ));
    }
}
pub trait always_convertible_layout_mapping_std_extents_unsigned_long__5__7__Impl {
    fn copy_assign(
        &self,
        other: Ptr<always_convertible_layout_mapping_std_extents_unsigned_long__5__7__>,
    ) -> Ptr<always_convertible_layout_mapping_std_extents_unsigned_long__5__7__> {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int_int(&self, idx_0: (), idx_1: ()) -> ();
}
impl always_convertible_layout_mapping_std_extents_unsigned_long__5__7__Impl
    for Ptr<always_convertible_layout_mapping_std_extents_unsigned_long__5__7__>
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
    fn operator_call_9_int_int(&self, mut idx_0: (), mut idx_1: ()) -> () {
        let idx_a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return ((*self).with(|__s| __s.offset_))
            .wrapping_add(((*self).with(|__s| __s.scaling_)).wrapping_mul(
            ({
                lambda!(
                    Generic,
                    {
                        let this_: Value<
                            Ptr<
                                always_convertible_layout_mapping_std_extents_unsigned_long__5__7__,
                            >,
                        > = Rc::new(RefCell::new((*self).clone()));
                        let idx_a: Ptr<()> = idx_a.as_pointer();
                    },
                    |_a0: ()| -> () {
                        let _a0: Value<()> = Rc::new(RefCell::new(_a0));
                        let mut res: () = 0_();
                        {
                            let __rhs = (unimplemented!().read())
                                .wrapping_add((unimplemented!()).wrapping_mul(res));
                            res = __rhs;
                            let __rhs = (unimplemented!().read())
                                .wrapping_add((unimplemented!()).wrapping_mul(res));
                            res = __rhs
                        };
                        return res;
                    }
                )
                .spec::<fn(()) -> ()>(0)
                .call(<()>::default())
            }),
        ));
    }
}
pub trait always_convertible_layout_mapping_std_extents_unsigned_long__5__7__8__9__1__Impl {
    fn copy_assign(
        &self,
        other: Ptr<always_convertible_layout_mapping_std_extents_unsigned_long__5__7__8__9__1__>,
    ) -> Ptr<always_convertible_layout_mapping_std_extents_unsigned_long__5__7__8__9__1__> {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> ();
    fn stride(&self, r: ()) -> ();
    fn operator_call_9_int_int_int_int_int(
        &self,
        idx_0: (),
        idx_1: (),
        idx_2: (),
        idx_3: (),
        idx_4: (),
    ) -> ();
}
impl always_convertible_layout_mapping_std_extents_unsigned_long__5__7__8__9__1__Impl
    for Ptr<always_convertible_layout_mapping_std_extents_unsigned_long__5__7__8__9__1__>
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
    fn operator_call_9_int_int_int_int_int(
        &self,
        mut idx_0: (),
        mut idx_1: (),
        mut idx_2: (),
        mut idx_3: (),
        mut idx_4: (),
    ) -> () {
        let idx_a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return ( (* self ) .with(|__s| __s . offset_ ) ) . wrapping_add ( ( (* self ) .with(|__s| __s . scaling_ ) ) . wrapping_mul ( ( ( { lambda! ( Generic , { let this_ : Value<Ptr< always_convertible_layout_mapping_std_extents_unsigned_long__5__7__8__9__1__  > > = Rc::new(RefCell::new(((* self ) ).clone())) ;
 let idx_a : Ptr< ()  > = idx_a .as_pointer()  ;
 } , | _a0 : ()  , | -> ()  { let _a0 : Value<() >  = Rc::new(RefCell::new(_a0)) ;
 let mut res : ()  = 0_()  ;
  ;
 ;
 { let __rhs = ( (unimplemented!() .read()) ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = __rhs ;
 let __rhs = ( (unimplemented!() .read()) ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = __rhs } } } } ;
 return res   ;
 } )  .spec::<fn(()  ,) -> ()  >(0) .call ( <()  >::default()  , ) } ) ) ) )   ;
    }
}
pub fn __cpp2rust_init_globals() {}
