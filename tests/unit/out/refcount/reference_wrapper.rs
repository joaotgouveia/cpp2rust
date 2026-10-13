extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct reference_wrapper_7 {}
impl Clone for reference_wrapper_7 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reference_wrapper_7 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reference_wrapper_7 {
    pub fn copy_from(_a0: Ptr<reference_wrapper_7>) -> Self {
        unimplemented!()
    }
}
impl reference_wrapper_7 {
    pub fn copy_assign(&self, _a0: Ptr<reference_wrapper_7>) -> Ptr<reference_wrapper_7> {
        unimplemented!()
    }
}
impl reference_wrapper_7 {
    pub fn new(__uref: Ptr<i32>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct reference_wrapper_8 {}
impl Clone for reference_wrapper_8 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reference_wrapper_8 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reference_wrapper_8 {
    pub fn copy_from(_a0: Ptr<reference_wrapper_8>) -> Self {
        unimplemented!()
    }
}
impl reference_wrapper_8 {
    pub fn copy_assign(&self, _a0: Ptr<reference_wrapper_8>) -> Ptr<reference_wrapper_8> {
        unimplemented!()
    }
}
impl reference_wrapper_8 {
    pub fn new(__uref: Ptr<Point>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_9 {}
impl Clone for integral_constant_9 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_9 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_integral_constant_bool__true__value_10: Value<bool> = unimplemented!();
);
impl integral_constant_9 {}
impl integral_constant_9 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_11 {}
impl Clone for integral_constant_11 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_11 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_integral_constant_bool__false__value_12: Value<bool> = unimplemented!();
);
impl integral_constant_11 {}
impl integral_constant_11 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_13 {}
impl Clone for integral_constant_13 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_13 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_14 {}
impl Clone for integral_constant_14 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_14 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_15 {}
impl Clone for integral_constant_15 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_15 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_16 {}
impl Clone for enable_if_16 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_16 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_17 {}
impl Clone for enable_if_17 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_17 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_18 {}
impl Clone for enable_if_18 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_18 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_19 {}
impl Clone for enable_if_19 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_19 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_20 {}
impl Clone for enable_if_20 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_20 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_21 {}
impl Clone for enable_if_21 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_21 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_22 {}
impl Clone for enable_if_22 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_22 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_23 {}
impl Clone for enable_if_23 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_23 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_24 {}
impl Clone for enable_if_24 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_24 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __conditional_25 {}
impl Clone for __conditional_25 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_25 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __conditional_26 {}
impl Clone for __conditional_26 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_26 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_27 {}
impl Clone for __type_identity_27 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_27 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_28 {}
impl Clone for __type_identity_28 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_28 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_29 {}
impl Clone for __type_identity_29 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_29 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_30 {}
impl Clone for __type_identity_30 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_30 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_31 {}
impl Clone for __type_identity_31 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_31 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_32 {}
impl Clone for __type_identity_32 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_32 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_33 {}
impl Clone for __type_identity_33 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_33 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_34 {}
impl Clone for __type_identity_34 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_34 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_35 {}
impl Clone for __type_identity_35 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_35 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_36 {}
impl Clone for __type_identity_36 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_36 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_37 {}
impl Clone for __type_identity_37 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_37 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_38 {}
impl Clone for __type_identity_38 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_38 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_39 {}
impl Clone for __type_identity_39 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_39 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_40 {}
impl Clone for __type_identity_40 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_40 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_41 {}
impl Clone for __type_identity_41 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_41 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_42 {}
impl Clone for __type_identity_42 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_42 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__43 {}
impl Clone for __or__43 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__43 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__44 {}
impl Clone for __or__44 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__44 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__45 {}
impl Clone for __or__45 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__45 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__46 {}
impl Clone for __or__46 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__46 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__47 {}
impl Clone for __or__47 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__47 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__48 {}
impl Clone for __or__48 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__48 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__49 {}
impl Clone for __or__49 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__49 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__50 {}
impl Clone for __or__50 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__50 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__51 {}
impl Clone for __or__51 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__51 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__52 {}
impl Clone for __or__52 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__52 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__53 {}
impl Clone for __or__53 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__53 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__54 {}
impl Clone for __or__54 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__54 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__55 {}
impl Clone for __or__55 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__55 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__56 {}
impl Clone for __or__56 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__56 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__57 {}
impl Clone for __or__57 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__57 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__58 {}
impl Clone for __or__58 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__58 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__59 {}
impl Clone for __or__59 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__59 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__60 {}
impl Clone for __or__60 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__60 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__61 {}
impl Clone for __or__61 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__61 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__62 {}
impl Clone for __or__62 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__62 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__63 {}
impl Clone for __or__63 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__63 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__64 {}
impl Clone for __or__64 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__64 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__65 {}
impl Clone for __and__65 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__65 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__66 {}
impl Clone for __and__66 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__66 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__67 {}
impl Clone for __and__67 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__67 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__68 {}
impl Clone for __and__68 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__68 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__69 {}
impl Clone for __and__69 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__69 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__70 {}
impl Clone for __and__70 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__70 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__71 {}
impl Clone for __and__71 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__71 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__72 {}
impl Clone for __and__72 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__72 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__73 {}
impl Clone for __and__73 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__73 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__74 {}
impl Clone for __and__74 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__74 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__75 {}
impl Clone for __and__75 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__75 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__76 {}
impl Clone for __and__76 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__76 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__77 {}
impl Clone for __and__77 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__77 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__78 {}
impl Clone for __and__78 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__78 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__79 {}
impl Clone for __and__79 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__79 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__80 {}
impl Clone for __and__80 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__80 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__81 {}
impl Clone for __and__81 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__81 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__82 {}
impl Clone for __and__82 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__82 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__83 {}
impl Clone for __and__83 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__83 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__84 {}
impl Clone for __and__84 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__84 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__85 {}
impl Clone for __and__85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__85 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__86 {}
impl Clone for __and__86 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__86 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__87 {}
impl Clone for __and__87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__88 {}
impl Clone for __and__88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__88 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__89 {}
impl Clone for __and__89 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__89 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__90 {}
impl Clone for __and__90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__90 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__91 {}
impl Clone for __and__91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__91 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__92 {}
impl Clone for __and__92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__93 {}
impl Clone for __and__93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__93 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__94 {}
impl Clone for __not__94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__94 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__95 {}
impl Clone for __not__95 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__95 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__96 {}
impl Clone for __not__96 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__96 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__97 {}
impl Clone for __not__97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__97 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__98 {}
impl Clone for __not__98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__98 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__99 {}
impl Clone for __not__99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__99 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__100 {}
impl Clone for __not__100 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__100 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__101 {}
impl Clone for __not__101 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__101 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__102 {}
impl Clone for __not__102 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__102 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__103 {}
impl Clone for __not__103 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__103 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_104 {}
impl Clone for is_reference_104 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_104 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_105 {}
impl Clone for is_reference_105 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_105 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_106 {}
impl Clone for is_reference_106 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_106 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_107 {}
impl Clone for is_reference_107 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_107 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_108 {}
impl Clone for is_reference_108 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_108 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_109 {}
impl Clone for is_reference_109 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_109 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_110 {}
impl Clone for is_reference_110 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_110 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_111 {}
impl Clone for is_reference_111 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_111 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_112 {}
impl Clone for is_reference_112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_112 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_113 {}
impl Clone for is_reference_113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_113 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_114 {}
impl Clone for is_reference_114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_114 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_115 {}
impl Clone for is_reference_115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_115 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_116 {}
impl Clone for is_reference_116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_116 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_117 {}
impl Clone for is_reference_117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_117 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_118 {}
impl Clone for is_reference_118 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_118 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_119 {}
impl Clone for is_reference_119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_119 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_120 {}
impl Clone for is_function_120 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_120 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_121 {}
impl Clone for is_function_121 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_121 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_122 {}
impl Clone for is_function_122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_122 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_123 {}
impl Clone for is_function_123 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_123 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_124 {}
impl Clone for is_function_124 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_124 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_125 {}
impl Clone for is_function_125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_125 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_126 {}
impl Clone for is_function_126 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_126 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_127 {}
impl Clone for is_function_127 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_127 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_128 {}
impl Clone for is_function_128 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_128 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_129 {}
impl Clone for is_function_129 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_129 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_130 {}
impl Clone for is_function_130 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_130 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_131 {}
impl Clone for is_function_131 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_131 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_132 {}
impl Clone for is_function_132 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_132 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_133 {}
impl Clone for is_function_133 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_133 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_134 {}
impl Clone for is_function_134 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_134 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_135 {}
impl Clone for is_function_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_135 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_136 {}
impl Clone for is_void_136 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_136 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_137 {}
impl Clone for is_void_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_137 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_138 {}
impl Clone for is_void_138 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_138 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_139 {}
impl Clone for is_void_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_139 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_140 {}
impl Clone for is_void_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_140 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_141 {}
impl Clone for is_void_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_142 {}
impl Clone for is_void_142 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_142 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_143 {}
impl Clone for is_void_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_143 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_144 {}
impl Clone for is_void_144 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_144 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_145 {}
impl Clone for is_void_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_145 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_146 {}
impl Clone for is_void_146 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_146 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_147 {}
impl Clone for is_void_147 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_147 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_148 {}
impl Clone for is_void_148 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_148 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_149 {}
impl Clone for is_void_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_150 {}
impl Clone for is_void_150 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_150 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_151 {}
impl Clone for is_void_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_152 {}
impl Clone for is_void_152 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_152 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_153 {}
impl Clone for is_void_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_154 {}
impl Clone for is_void_154 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_154 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_155 {}
impl Clone for is_void_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_156 {}
impl Clone for is_void_156 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_156 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_157 {}
impl Clone for is_void_157 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_157 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_158 {}
impl Clone for is_void_158 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_158 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_159 {}
impl Clone for is_void_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_159 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_160 {}
impl Clone for is_void_160 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_160 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_161 {}
impl Clone for remove_cv_161 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_161 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_162 {}
impl Clone for remove_cv_162 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_162 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_163 {}
impl Clone for remove_cv_163 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_163 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_164 {}
impl Clone for remove_cv_164 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_164 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_165 {}
impl Clone for remove_cv_165 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_165 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_166 {}
impl Clone for remove_cv_166 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_166 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_167 {}
impl Clone for remove_cv_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_167 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_168 {}
impl Clone for remove_cv_168 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_168 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_169 {}
impl Clone for remove_cv_169 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_169 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_170 {}
impl Clone for remove_cv_170 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_170 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_171 {}
impl Clone for remove_cv_171 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_171 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_172 {}
impl Clone for remove_cv_172 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_172 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_173 {}
impl Clone for is_const_173 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_173 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_174 {}
impl Clone for is_const_174 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_174 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_175 {}
impl Clone for is_const_175 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_175 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_176 {}
impl Clone for is_const_176 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_176 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_177 {}
impl Clone for is_const_177 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_177 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_178 {}
impl Clone for __is_array_unknown_bounds_178 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_178 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_179 {}
impl Clone for __is_array_unknown_bounds_179 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_179 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_180 {}
impl Clone for __is_array_unknown_bounds_180 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_180 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_181 {}
impl Clone for __is_array_unknown_bounds_181 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_181 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_182 {}
impl Clone for __is_array_unknown_bounds_182 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_182 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_183 {}
impl Clone for __is_array_unknown_bounds_183 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_183 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_184 {}
impl Clone for __is_array_unknown_bounds_184 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_184 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_185 {}
impl Clone for __is_array_unknown_bounds_185 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_185 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_186 {}
impl Clone for __is_array_unknown_bounds_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_187 {}
impl Clone for __is_array_unknown_bounds_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_187 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_188 {}
impl Clone for __is_array_unknown_bounds_188 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_188 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_189 {}
impl Clone for __is_array_unknown_bounds_189 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_189 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_190 {}
impl Clone for __is_array_unknown_bounds_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_190 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_191 {}
impl Clone for __is_array_unknown_bounds_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_191 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_192 {}
impl Clone for __is_array_unknown_bounds_192 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_192 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_193 {}
impl Clone for __is_array_unknown_bounds_193 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_193 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_194 {}
impl Clone for __is_integral_helper_194 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_194 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_195 {}
impl Clone for __is_integral_helper_195 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_195 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_196 {}
impl Clone for __is_integral_helper_196 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_196 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_197 {}
impl Clone for __is_integral_helper_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_197 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_198 {}
impl Clone for __is_integral_helper_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_199 {}
impl Clone for is_integral_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_199 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_200 {}
impl Clone for is_integral_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_200 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_201 {}
impl Clone for is_integral_201 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_201 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_202 {}
impl Clone for is_integral_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_202 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_203 {}
impl Clone for is_integral_203 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_203 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_204 {}
impl Clone for is_pointer_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_204 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_205 {}
impl Clone for is_pointer_205 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_205 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_member_object_pointer_206 {}
impl Clone for is_member_object_pointer_206 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_object_pointer_206 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_member_function_pointer_207 {}
impl Clone for is_member_function_pointer_207 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_function_pointer_207 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_member_function_pointer_208 {}
impl Clone for is_member_function_pointer_208 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_function_pointer_208 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_member_function_pointer_209 {}
impl Clone for is_member_function_pointer_209 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_function_pointer_209 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_class_210 {}
impl Clone for is_class_210 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_class_210 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_211 {}
impl Clone for is_same_211 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_211 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_212 {}
impl Clone for is_same_212 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_212 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_213 {}
impl Clone for is_same_213 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_213 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_214 {}
impl Clone for is_same_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_214 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_215 {}
impl Clone for is_same_215 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_215 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_216 {}
impl Clone for is_same_216 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_216 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_217 {}
impl Clone for is_same_217 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_217 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_218 {}
impl Clone for is_same_218 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_218 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_219 {}
impl Clone for is_same_219 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_219 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_220 {}
impl Clone for is_same_220 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_220 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_221 {}
impl Clone for is_same_221 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_221 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_222 {}
impl Clone for is_same_222 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_222 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_223 {}
impl Clone for is_same_223 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_223 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_224 {}
impl Clone for is_same_224 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_224 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_225 {}
impl Clone for is_same_225 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_225 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_226 {}
impl Clone for is_same_226 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_226 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_227 {}
impl Clone for is_same_227 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_227 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_228 {}
impl Clone for is_same_228 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_228 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_229 {}
impl Clone for is_same_229 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_229 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_230 {}
impl Clone for is_same_230 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_230 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_231 {}
impl Clone for is_same_231 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_231 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_232 {}
impl Clone for is_same_232 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_232 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_233 {}
impl Clone for is_same_233 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_233 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_234 {}
impl Clone for is_same_234 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_234 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_235 {}
impl Clone for is_same_235 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_235 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_236 {}
impl Clone for is_volatile_236 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_236 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_237 {}
impl Clone for is_volatile_237 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_237 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_238 {}
impl Clone for is_volatile_238 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_238 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_239 {}
impl Clone for is_volatile_239 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_239 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_240 {}
impl Clone for is_volatile_240 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_240 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___do_is_destructible_impl {}
impl Clone for std___do_is_destructible_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___do_is_destructible_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___do_is_nt_destructible_impl {}
impl Clone for std___do_is_nt_destructible_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___do_is_nt_destructible_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_241 {}
impl Clone for is_constructible_241 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_241 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_242 {}
impl Clone for is_constructible_242 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_242 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_243 {}
impl Clone for is_constructible_243 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_243 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_244 {}
impl Clone for is_constructible_244 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_244 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_245 {}
impl Clone for is_constructible_245 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_245 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_default_constructible_246 {}
impl Clone for is_default_constructible_246 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_default_constructible_246 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_default_constructible_247 {}
impl Clone for is_default_constructible_247 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_default_constructible_247 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_default_constructible_248 {}
impl Clone for is_default_constructible_248 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_default_constructible_248 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_copy_constructible_249 {}
impl Clone for is_copy_constructible_249 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_constructible_249 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_250 {}
impl Clone for is_move_constructible_250 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_250 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_251 {}
impl Clone for is_move_constructible_251 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_251 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_252 {}
impl Clone for is_move_constructible_252 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_252 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_253 {}
impl Clone for is_nothrow_default_constructible_253 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_253 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_254 {}
impl Clone for is_nothrow_default_constructible_254 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_254 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_255 {}
impl Clone for is_nothrow_default_constructible_255 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_255 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_256 {}
impl Clone for is_nothrow_default_constructible_256 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_256 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_257 {}
impl Clone for is_nothrow_copy_constructible_257 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_257 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_258 {}
impl Clone for is_nothrow_copy_constructible_258 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_258 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_259 {}
impl Clone for is_nothrow_copy_constructible_259 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_259 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_260 {}
impl Clone for is_nothrow_copy_constructible_260 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_260 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_261 {}
impl Clone for is_nothrow_copy_constructible_261 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_261 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_constructible_262 {}
impl Clone for is_nothrow_move_constructible_262 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_262 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_constructible_263 {}
impl Clone for is_nothrow_move_constructible_263 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_263 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_constructible_264 {}
impl Clone for is_nothrow_move_constructible_264 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_264 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_copy_assignable_265 {}
impl Clone for is_copy_assignable_265 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_assignable_265 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_copy_assignable_266 {}
impl Clone for is_copy_assignable_266 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_assignable_266 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_267 {}
impl Clone for is_move_assignable_267 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_267 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_268 {}
impl Clone for is_move_assignable_268 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_268 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_269 {}
impl Clone for is_move_assignable_269 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_269 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_270 {}
impl Clone for is_move_assignable_270 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_270 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_assignable_271 {}
impl Clone for is_nothrow_move_assignable_271 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_271 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_assignable_272 {}
impl Clone for is_nothrow_move_assignable_272 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_272 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_assignable_273 {}
impl Clone for is_nothrow_move_assignable_273 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_273 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___do_is_implicitly_default_constructible_impl {}
impl Clone for std___do_is_implicitly_default_constructible_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___do_is_implicitly_default_constructible_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_274 {}
impl Clone for is_convertible_274 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_274 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_275 {}
impl Clone for is_convertible_275 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_275 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_276 {}
impl Clone for is_convertible_276 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_276 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_277 {}
impl Clone for is_convertible_277 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_277 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_278 {}
impl Clone for is_convertible_278 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_278 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_279 {}
impl Clone for is_convertible_279 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_279 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_280 {}
impl Clone for is_convertible_280 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_280 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_281 {}
impl Clone for is_convertible_281 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_281 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_282 {}
impl Clone for is_convertible_282 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_282 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_283 {}
impl Clone for is_convertible_283 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_283 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_284 {}
impl Clone for is_convertible_284 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_284 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_285 {}
impl Clone for is_convertible_285 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_285 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_286 {}
impl Clone for is_convertible_286 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_286 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_287 {}
impl Clone for is_convertible_287 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_287 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_288 {}
impl Clone for is_convertible_288 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_288 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_289 {}
impl Clone for is_convertible_289 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_289 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_290 {}
impl Clone for remove_reference_290 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_290 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_291 {}
impl Clone for remove_reference_291 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_291 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_292 {}
impl Clone for remove_reference_292 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_292 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_293 {}
impl Clone for remove_reference_293 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_293 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_294 {}
impl Clone for remove_reference_294 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_294 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_295 {}
impl Clone for remove_reference_295 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_295 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_296 {}
impl Clone for remove_reference_296 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_296 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_297 {}
impl Clone for remove_reference_297 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_297 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_298 {}
impl Clone for remove_reference_298 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_298 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_299 {}
impl Clone for remove_reference_299 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_299 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_300 {}
impl Clone for remove_reference_300 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_300 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_301 {}
impl Clone for remove_reference_301 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_301 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_302 {}
impl Clone for remove_reference_302 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_302 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_303 {}
impl Clone for remove_reference_303 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_303 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_304 {}
impl Clone for remove_reference_304 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_304 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_305 {}
impl Clone for remove_reference_305 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_305 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_306 {}
impl Clone for remove_reference_306 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_306 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_307 {}
impl Clone for __cv_selector_307 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_307 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_308 {}
impl Clone for __cv_selector_308 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_308 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_309 {}
impl Clone for __cv_selector_309 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_309 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_310 {}
impl Clone for __cv_selector_310 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_310 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_311 {}
impl Clone for __match_cv_qualifiers_311 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_311 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_312 {}
impl Clone for __match_cv_qualifiers_312 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_312 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_313 {}
impl Clone for __match_cv_qualifiers_313 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_313 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_314 {}
impl Clone for __match_cv_qualifiers_314 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_314 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_315 {}
impl Clone for __match_cv_qualifiers_315 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_315 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_316 {}
impl Clone for __make_unsigned_316 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_316 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_317 {}
impl Clone for __make_unsigned_317 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_317 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_318 {}
impl Clone for __make_unsigned_318 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_318 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_319 {}
impl Clone for __make_unsigned_selector_319 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_319 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_320 {}
impl Clone for __make_unsigned_selector_320 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_320 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_321 {}
impl Clone for __make_unsigned_selector_321 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_321 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_322 {}
impl Clone for __make_unsigned_selector_322 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_322 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_323 {}
impl Clone for __make_unsigned_selector_323 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_323 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_324 {}
impl Clone for __make_unsigned_selector_324 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_324 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___make_unsigned_selector_base {}
impl Clone for std___make_unsigned_selector_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___make_unsigned_selector_base {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_328 {}
impl Clone for _List_328 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_328 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_329 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_330 {}
impl Clone for _List_330 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_330 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_331 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_332 {}
impl Clone for _List_332 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_332 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_333 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_334 {}
impl Clone for _List_334 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_334 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_335 {}
impl Clone for _List_335 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_335 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_336 {}
impl Clone for _List_336 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_336 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_337 {}
impl Clone for __select_337 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_337 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_338 {}
impl Clone for __select_338 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_338 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_339 {}
impl Clone for __select_339 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_339 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_340 {}
impl Clone for __select_340 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_340 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_341 {}
impl Clone for __select_341 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_341 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_342 {}
impl Clone for __make_signed_342 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_342 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_343 {}
impl Clone for __make_signed_343 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_343 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_344 {}
impl Clone for __make_signed_selector_344 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_344 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_345 {}
impl Clone for __make_signed_selector_345 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_345 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_346 {}
impl Clone for __make_signed_selector_346 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_346 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_347 {}
impl Clone for __make_signed_selector_347 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_347 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_348 {}
impl Clone for __make_signed_selector_348 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_348 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct add_pointer_349 {}
impl Clone for add_pointer_349 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for add_pointer_349 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct std___aligned_storage_max_align_t {}
impl Clone for std___aligned_storage_max_align_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___aligned_storage_max_align_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct common_type_350 {}
impl Clone for common_type_350 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_350 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __success_type_351 {}
impl Clone for __success_type_351 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __success_type_351 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___failure_type {}
impl Clone for std___failure_type {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___failure_type {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___do_common_type_impl {}
impl Clone for std___do_common_type_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___do_common_type_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___invoke_memfun_ref {}
impl Clone for std___invoke_memfun_ref {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___invoke_memfun_ref {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___invoke_memfun_ref {}
impl std___invoke_memfun_ref {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___invoke_memfun_deref {}
impl Clone for std___invoke_memfun_deref {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___invoke_memfun_deref {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___invoke_memfun_deref {}
impl std___invoke_memfun_deref {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___invoke_memobj_ref {}
impl Clone for std___invoke_memobj_ref {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___invoke_memobj_ref {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___invoke_memobj_ref {}
impl std___invoke_memobj_ref {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___invoke_memobj_deref {}
impl Clone for std___invoke_memobj_deref {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___invoke_memobj_deref {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___invoke_memobj_deref {}
impl std___invoke_memobj_deref {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___invoke_other {}
impl Clone for std___invoke_other {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___invoke_other {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___invoke_other {}
impl std___invoke_other {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __result_of_success_352 {}
impl Clone for __result_of_success_352 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __result_of_success_352 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___result_of_memfun_ref_impl {}
impl Clone for std___result_of_memfun_ref_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___result_of_memfun_ref_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___result_of_memfun_deref_impl {}
impl Clone for std___result_of_memfun_deref_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___result_of_memfun_deref_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___result_of_memobj_ref_impl {}
impl Clone for std___result_of_memobj_ref_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___result_of_memobj_ref_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___result_of_memobj_deref_impl {}
impl Clone for std___result_of_memobj_deref_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___result_of_memobj_deref_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __result_of_impl_353 {}
impl Clone for __result_of_impl_353 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __result_of_impl_353 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___result_of_other_impl {}
impl Clone for std___result_of_other_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___result_of_other_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __invoke_result_354 {}
impl Clone for __invoke_result_354 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __invoke_result_354 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __detector_355 {}
impl Clone for __detector_355 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __detector_355 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_impl_356 {}
impl Clone for __is_tuple_like_impl_356 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_356 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_impl_357 {}
impl Clone for __is_tuple_like_impl_357 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_357 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_impl_358 {}
impl Clone for __is_tuple_like_impl_358 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_358 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_359 {}
impl Clone for __is_tuple_like_359 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_359 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_360 {}
impl Clone for __is_tuple_like_360 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_360 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_361 {}
impl Clone for __is_tuple_like_361 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_361 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___swappable_details___do_is_swappable_impl {}
impl Clone for std___swappable_details___do_is_swappable_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___swappable_details___do_is_swappable_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___swappable_details___do_is_nothrow_swappable_impl {}
impl Clone for std___swappable_details___do_is_nothrow_swappable_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___swappable_details___do_is_nothrow_swappable_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___swappable_with_details___do_is_swappable_with_impl {}
impl Clone for std___swappable_with_details___do_is_swappable_with_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___swappable_with_details___do_is_swappable_with_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___swappable_with_details___do_is_nothrow_swappable_with_impl {}
impl Clone for std___swappable_with_details___do_is_nothrow_swappable_with_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___swappable_with_details___do_is_nothrow_swappable_with_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_invocable_impl_362 {}
impl Clone for __is_invocable_impl_362 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_invocable_impl_362 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_invocable_363 {}
impl Clone for __is_invocable_363 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_invocable_363 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __call_is_nothrow_364 {}
impl Clone for __call_is_nothrow_364 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __call_is_nothrow_364 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_invocable_365 {}
impl Clone for __is_nothrow_invocable_365 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_invocable_365 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___nonesuchbase {}
impl Clone for std___nonesuchbase {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___nonesuchbase {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___nonesuchbase {}
impl std___nonesuchbase {}
impl std___nonesuchbase {
    pub fn copy_assign(&self, _a0: Ptr<std___nonesuchbase>) -> Ptr<std___nonesuchbase> {
        unimplemented!()
    }
}
impl std___nonesuchbase {
    pub fn move_assign(&self, _a0: Ptr<std___nonesuchbase>) -> Ptr<std___nonesuchbase> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___nonesuch {}
impl Clone for std___nonesuch {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___nonesuch {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static is_array_v_366: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_copyable_v_367: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_default_constructible_v_368: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_standard_layout_v_369: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_370: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_array_v_371: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_copyable_v_372: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_default_constructible_v_373: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_standard_layout_v_374: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_375: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_array_v_376: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_copyable_v_377: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_default_constructible_v_378: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_standard_layout_v_379: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_380: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_array_v_381: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_copyable_v_382: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_default_constructible_v_383: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_standard_layout_v_384: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_385: Value<bool> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct binary_function_386 {}
impl Clone for binary_function_386 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_386 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct binary_function_387 {}
impl Clone for binary_function_387 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_387 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl binary_function_387 {}
impl binary_function_387 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct binary_function_388 {}
impl Clone for binary_function_388 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_388 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl binary_function_388 {}
impl binary_function_388 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct binary_function_389 {}
impl Clone for binary_function_389 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_389 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl binary_function_389 {}
impl binary_function_389 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct equal_to_390 {}
impl Clone for equal_to_390 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for equal_to_390 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct equal_to_391 {}
impl Clone for equal_to_391 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for equal_to_391 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl equal_to_391 {}
impl equal_to_391 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_392 {}
impl Clone for greater_392 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_392 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_393 {}
impl Clone for less_393 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_393 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_394 {}
impl Clone for less_394 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_394 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl less_394 {}
impl less_394 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_395 {}
impl Clone for less_395 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_395 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl less_395 {}
impl less_395 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_equal_396 {}
impl Clone for greater_equal_396 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_396 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_equal_397 {}
impl Clone for less_equal_397 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_397 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_in_place_t {}
impl Clone for std_in_place_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_in_place_t {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_in_place_t {}
impl std_in_place_t {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std__Swallow_assign {}
impl Clone for std__Swallow_assign {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Swallow_assign {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_piecewise_construct_t {}
impl Clone for std_piecewise_construct_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_piecewise_construct_t {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_piecewise_construct_t {}
impl std_piecewise_construct_t {}
thread_local!(
    pub static piecewise_construct_398: Value<std_piecewise_construct_t> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct pair_399 {
    #[offset(0)]
    pub first: bool,
    #[offset(8)]
    pub second: u64,
}
impl Clone for pair_399 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pair_399 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl pair_399 {
    pub fn copy_from(_a0: Ptr<pair_399>) -> Self {
        unimplemented!()
    }
}
impl pair_399 {
    pub fn move_from(_a0: Ptr<pair_399>) -> Self {
        unimplemented!()
    }
}
impl pair_399 {
    pub fn new_1(__x: Ptr<bool>, __y: Ptr<u64>) -> Self {
        unimplemented!()
    }
}
impl pair_399 {
    pub fn new_2(__x: Ptr<bool>, __y: Ptr<i32>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_400 {}
impl Clone for tuple_400 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_400 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_400 {}
impl tuple_400 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _PCC_401 {}
impl Clone for _PCC_401 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _PCC_401 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __pair_base_402 {}
impl Clone for __pair_base_402 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pair_base_402 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___erased_type {}
impl Clone for std___erased_type {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___erased_type {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_allocator_arg_t {}
impl Clone for std_allocator_arg_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_allocator_arg_t {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_allocator_arg_t {}
impl std_allocator_arg_t {}
thread_local!(
    pub static allocator_arg_403: Value<std_allocator_arg_t> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___uses_alloc_base {}
impl Clone for std___uses_alloc_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___uses_alloc_base {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___uses_alloc0 {
    #[offset(0)]
    #[byte_size(1)]
    pub _M_a: std___uses_alloc0__Sink,
}
impl Clone for std___uses_alloc0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___uses_alloc0 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_404 {}
impl Clone for __hash_base_404 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_404 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_405 {}
impl Clone for __hash_base_405 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_405 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_406 {}
impl Clone for __hash_base_406 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_406 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_407 {}
impl Clone for __hash_base_407 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_407 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_408 {}
impl Clone for __hash_base_408 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_408 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_409 {}
impl Clone for __hash_base_409 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_409 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_410 {}
impl Clone for __hash_base_410 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_410 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_411 {}
impl Clone for __hash_base_411 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_411 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_412 {}
impl Clone for __hash_base_412 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_412 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __hash_base_412 {}
impl __hash_base_412 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_413 {}
impl Clone for __hash_base_413 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_413 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_414 {}
impl Clone for __hash_base_414 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_414 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_415 {}
impl Clone for __hash_base_415 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_415 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_416 {}
impl Clone for __hash_base_416 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_416 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_417 {}
impl Clone for __hash_base_417 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_417 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_418 {}
impl Clone for __hash_base_418 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_418 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_419 {}
impl Clone for __hash_base_419 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_419 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_420 {}
impl Clone for __hash_base_420 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_420 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_421 {}
impl Clone for __hash_base_421 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_421 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_422 {}
impl Clone for __hash_base_422 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_422 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_423 {}
impl Clone for __hash_base_423 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_423 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_424 {}
impl Clone for __hash_base_424 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_424 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_425 {}
impl Clone for __hash_base_425 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_425 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_426 {}
impl Clone for __hash_base_426 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_426 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_427 {}
impl Clone for __hash_base_427 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_427 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_428 {}
impl Clone for __hash_base_428 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_428 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_429 {}
impl Clone for __hash_base_429 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_429 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_430 {}
impl Clone for __hash_base_430 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_430 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_431 {}
impl Clone for hash_431 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_431 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl hash_431 {}
impl hash_431 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_432 {}
impl Clone for hash_432 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_432 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_433 {}
impl Clone for hash_433 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_433 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_434 {}
impl Clone for hash_434 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_434 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_435 {}
impl Clone for hash_435 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_435 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_436 {}
impl Clone for hash_436 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_436 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std__Hash_impl {}
impl Clone for std__Hash_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Hash_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash(mut __ptr: AnyPtr, mut __clength: usize, __seed: Option<usize>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_437(__val: Ptr<f32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_438(__val: Ptr<f64>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_439(__val: Ptr<i32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn __hash_combine_440(__val: Ptr<Ptr<std_error_category>>, mut __hash: usize) -> usize {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_fast_hash_441 {}
impl Clone for __is_fast_hash_441 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_fast_hash_441 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Maybe_get_result_type_442 {}
impl Clone for _Maybe_get_result_type_442 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Maybe_get_result_type_442 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Maybe_get_result_type_442 {}
impl _Maybe_get_result_type_442 {}
impl _Maybe_get_result_type_442 {
    pub fn copy_assign(
        &self,
        _a0: Ptr<_Maybe_get_result_type_442>,
    ) -> Ptr<_Maybe_get_result_type_442> {
        unimplemented!()
    }
}
impl _Maybe_get_result_type_442 {
    pub fn move_assign(
        &self,
        _a0: Ptr<_Maybe_get_result_type_442>,
    ) -> Ptr<_Maybe_get_result_type_442> {
        unimplemented!()
    }
}
impl _Maybe_get_result_type_442 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Maybe_get_result_type_443 {}
impl Clone for _Maybe_get_result_type_443 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Maybe_get_result_type_443 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Maybe_get_result_type_443 {}
impl _Maybe_get_result_type_443 {}
impl _Maybe_get_result_type_443 {
    pub fn copy_assign(
        &self,
        _a0: Ptr<_Maybe_get_result_type_443>,
    ) -> Ptr<_Maybe_get_result_type_443> {
        unimplemented!()
    }
}
impl _Maybe_get_result_type_443 {
    pub fn move_assign(
        &self,
        _a0: Ptr<_Maybe_get_result_type_443>,
    ) -> Ptr<_Maybe_get_result_type_443> {
        unimplemented!()
    }
}
impl _Maybe_get_result_type_443 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Weak_result_type_impl_444 {}
impl Clone for _Weak_result_type_impl_444 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Weak_result_type_impl_444 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Weak_result_type_impl_444 {}
impl _Weak_result_type_impl_444 {}
impl _Weak_result_type_impl_444 {
    pub fn copy_assign(
        &self,
        _a0: Ptr<_Weak_result_type_impl_444>,
    ) -> Ptr<_Weak_result_type_impl_444> {
        unimplemented!()
    }
}
impl _Weak_result_type_impl_444 {
    pub fn move_assign(
        &self,
        _a0: Ptr<_Weak_result_type_impl_444>,
    ) -> Ptr<_Weak_result_type_impl_444> {
        unimplemented!()
    }
}
impl _Weak_result_type_impl_444 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Weak_result_type_impl_445 {}
impl Clone for _Weak_result_type_impl_445 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Weak_result_type_impl_445 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Weak_result_type_impl_445 {}
impl _Weak_result_type_impl_445 {}
impl _Weak_result_type_impl_445 {
    pub fn copy_assign(
        &self,
        _a0: Ptr<_Weak_result_type_impl_445>,
    ) -> Ptr<_Weak_result_type_impl_445> {
        unimplemented!()
    }
}
impl _Weak_result_type_impl_445 {
    pub fn move_assign(
        &self,
        _a0: Ptr<_Weak_result_type_impl_445>,
    ) -> Ptr<_Weak_result_type_impl_445> {
        unimplemented!()
    }
}
impl _Weak_result_type_impl_445 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Weak_result_type_memfun_446 {}
impl Clone for _Weak_result_type_memfun_446 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Weak_result_type_memfun_446 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Weak_result_type_memfun_446 {}
impl _Weak_result_type_memfun_446 {}
impl _Weak_result_type_memfun_446 {
    pub fn copy_assign(
        &self,
        _a0: Ptr<_Weak_result_type_memfun_446>,
    ) -> Ptr<_Weak_result_type_memfun_446> {
        unimplemented!()
    }
}
impl _Weak_result_type_memfun_446 {
    pub fn move_assign(
        &self,
        _a0: Ptr<_Weak_result_type_memfun_446>,
    ) -> Ptr<_Weak_result_type_memfun_446> {
        unimplemented!()
    }
}
impl _Weak_result_type_memfun_446 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Weak_result_type_memfun_447 {}
impl Clone for _Weak_result_type_memfun_447 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Weak_result_type_memfun_447 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Weak_result_type_memfun_447 {}
impl _Weak_result_type_memfun_447 {}
impl _Weak_result_type_memfun_447 {
    pub fn copy_assign(
        &self,
        _a0: Ptr<_Weak_result_type_memfun_447>,
    ) -> Ptr<_Weak_result_type_memfun_447> {
        unimplemented!()
    }
}
impl _Weak_result_type_memfun_447 {
    pub fn move_assign(
        &self,
        _a0: Ptr<_Weak_result_type_memfun_447>,
    ) -> Ptr<_Weak_result_type_memfun_447> {
        unimplemented!()
    }
}
impl _Weak_result_type_memfun_447 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Weak_result_type_448 {}
impl Clone for _Weak_result_type_448 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Weak_result_type_448 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Weak_result_type_448 {}
impl _Weak_result_type_448 {}
impl _Weak_result_type_448 {
    pub fn copy_assign(&self, _a0: Ptr<_Weak_result_type_448>) -> Ptr<_Weak_result_type_448> {
        unimplemented!()
    }
}
impl _Weak_result_type_448 {
    pub fn move_assign(&self, _a0: Ptr<_Weak_result_type_448>) -> Ptr<_Weak_result_type_448> {
        unimplemented!()
    }
}
impl _Weak_result_type_448 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Weak_result_type_449 {}
impl Clone for _Weak_result_type_449 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Weak_result_type_449 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Weak_result_type_449 {}
impl _Weak_result_type_449 {}
impl _Weak_result_type_449 {
    pub fn copy_assign(&self, _a0: Ptr<_Weak_result_type_449>) -> Ptr<_Weak_result_type_449> {
        unimplemented!()
    }
}
impl _Weak_result_type_449 {
    pub fn move_assign(&self, _a0: Ptr<_Weak_result_type_449>) -> Ptr<_Weak_result_type_449> {
        unimplemented!()
    }
}
impl _Weak_result_type_449 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Refwrap_base_arg1_450 {}
impl Clone for _Refwrap_base_arg1_450 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Refwrap_base_arg1_450 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Refwrap_base_arg1_450 {}
impl _Refwrap_base_arg1_450 {}
impl _Refwrap_base_arg1_450 {
    pub fn copy_assign(&self, _a0: Ptr<_Refwrap_base_arg1_450>) -> Ptr<_Refwrap_base_arg1_450> {
        unimplemented!()
    }
}
impl _Refwrap_base_arg1_450 {
    pub fn move_assign(&self, _a0: Ptr<_Refwrap_base_arg1_450>) -> Ptr<_Refwrap_base_arg1_450> {
        unimplemented!()
    }
}
impl _Refwrap_base_arg1_450 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Refwrap_base_arg1_451 {}
impl Clone for _Refwrap_base_arg1_451 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Refwrap_base_arg1_451 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Refwrap_base_arg1_451 {}
impl _Refwrap_base_arg1_451 {}
impl _Refwrap_base_arg1_451 {
    pub fn copy_assign(&self, _a0: Ptr<_Refwrap_base_arg1_451>) -> Ptr<_Refwrap_base_arg1_451> {
        unimplemented!()
    }
}
impl _Refwrap_base_arg1_451 {
    pub fn move_assign(&self, _a0: Ptr<_Refwrap_base_arg1_451>) -> Ptr<_Refwrap_base_arg1_451> {
        unimplemented!()
    }
}
impl _Refwrap_base_arg1_451 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Refwrap_base_arg2_452 {}
impl Clone for _Refwrap_base_arg2_452 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Refwrap_base_arg2_452 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Refwrap_base_arg2_452 {}
impl _Refwrap_base_arg2_452 {}
impl _Refwrap_base_arg2_452 {
    pub fn copy_assign(&self, _a0: Ptr<_Refwrap_base_arg2_452>) -> Ptr<_Refwrap_base_arg2_452> {
        unimplemented!()
    }
}
impl _Refwrap_base_arg2_452 {
    pub fn move_assign(&self, _a0: Ptr<_Refwrap_base_arg2_452>) -> Ptr<_Refwrap_base_arg2_452> {
        unimplemented!()
    }
}
impl _Refwrap_base_arg2_452 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Refwrap_base_arg2_453 {}
impl Clone for _Refwrap_base_arg2_453 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Refwrap_base_arg2_453 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Refwrap_base_arg2_453 {}
impl _Refwrap_base_arg2_453 {}
impl _Refwrap_base_arg2_453 {
    pub fn copy_assign(&self, _a0: Ptr<_Refwrap_base_arg2_453>) -> Ptr<_Refwrap_base_arg2_453> {
        unimplemented!()
    }
}
impl _Refwrap_base_arg2_453 {
    pub fn move_assign(&self, _a0: Ptr<_Refwrap_base_arg2_453>) -> Ptr<_Refwrap_base_arg2_453> {
        unimplemented!()
    }
}
impl _Refwrap_base_arg2_453 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Reference_wrapper_base_454 {}
impl Clone for _Reference_wrapper_base_454 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Reference_wrapper_base_454 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Reference_wrapper_base_454 {}
impl _Reference_wrapper_base_454 {}
impl _Reference_wrapper_base_454 {
    pub fn copy_assign(
        &self,
        _a0: Ptr<_Reference_wrapper_base_454>,
    ) -> Ptr<_Reference_wrapper_base_454> {
        unimplemented!()
    }
}
impl _Reference_wrapper_base_454 {
    pub fn move_assign(
        &self,
        _a0: Ptr<_Reference_wrapper_base_454>,
    ) -> Ptr<_Reference_wrapper_base_454> {
        unimplemented!()
    }
}
impl _Reference_wrapper_base_454 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Reference_wrapper_base_455 {}
impl Clone for _Reference_wrapper_base_455 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Reference_wrapper_base_455 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Reference_wrapper_base_455 {}
impl _Reference_wrapper_base_455 {}
impl _Reference_wrapper_base_455 {
    pub fn copy_assign(
        &self,
        _a0: Ptr<_Reference_wrapper_base_455>,
    ) -> Ptr<_Reference_wrapper_base_455> {
        unimplemented!()
    }
}
impl _Reference_wrapper_base_455 {
    pub fn move_assign(
        &self,
        _a0: Ptr<_Reference_wrapper_base_455>,
    ) -> Ptr<_Reference_wrapper_base_455> {
        unimplemented!()
    }
}
impl _Reference_wrapper_base_455 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Reference_wrapper_base_memfun_456 {}
impl Clone for _Reference_wrapper_base_memfun_456 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Reference_wrapper_base_memfun_456 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Reference_wrapper_base_memfun_456 {}
impl _Reference_wrapper_base_memfun_456 {}
impl _Reference_wrapper_base_memfun_456 {
    pub fn copy_assign(
        &self,
        _a0: Ptr<_Reference_wrapper_base_memfun_456>,
    ) -> Ptr<_Reference_wrapper_base_memfun_456> {
        unimplemented!()
    }
}
impl _Reference_wrapper_base_memfun_456 {
    pub fn move_assign(
        &self,
        _a0: Ptr<_Reference_wrapper_base_memfun_456>,
    ) -> Ptr<_Reference_wrapper_base_memfun_456> {
        unimplemented!()
    }
}
impl _Reference_wrapper_base_memfun_456 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Reference_wrapper_base_memfun_457 {}
impl Clone for _Reference_wrapper_base_memfun_457 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Reference_wrapper_base_memfun_457 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Reference_wrapper_base_memfun_457 {}
impl _Reference_wrapper_base_memfun_457 {}
impl _Reference_wrapper_base_memfun_457 {
    pub fn copy_assign(
        &self,
        _a0: Ptr<_Reference_wrapper_base_memfun_457>,
    ) -> Ptr<_Reference_wrapper_base_memfun_457> {
        unimplemented!()
    }
}
impl _Reference_wrapper_base_memfun_457 {
    pub fn move_assign(
        &self,
        _a0: Ptr<_Reference_wrapper_base_memfun_457>,
    ) -> Ptr<_Reference_wrapper_base_memfun_457> {
        unimplemented!()
    }
}
impl _Reference_wrapper_base_memfun_457 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct std_exception {}
impl Clone for std_exception {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_exception {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub fn copy_from(_a0: Ptr<std_exception>) -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub fn copy_assign(&self, _a0: Ptr<std_exception>) -> Ptr<std_exception> {
        unimplemented!()
    }
}
impl std_exception {
    pub fn move_from(_a0: Ptr<std_exception>) -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub fn move_assign(&self, _a0: Ptr<std_exception>) -> Ptr<std_exception> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct std_bad_alloc {}
impl Clone for std_bad_alloc {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_bad_alloc {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_bad_alloc {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std_bad_alloc {
    pub fn copy_from(_a0: Ptr<std_bad_alloc>) -> Self {
        unimplemented!()
    }
}
impl std_bad_alloc {
    pub fn copy_assign(&self, _a0: Ptr<std_bad_alloc>) -> Ptr<std_bad_alloc> {
        unimplemented!()
    }
}
pub type std_align_val_t = usize;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_nothrow_t {}
impl Clone for std_nothrow_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_nothrow_t {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_nothrow_t {}
impl std_nothrow_t {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct std_type_info {}
impl Clone for std_type_info {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_type_info {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct std__Nocopy_types {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_object: AnyPtr,
    #[offset(0)]
    #[byte_size(8)]
    pub _M_const_object: AnyPtr,
    #[offset(0)]
    #[byte_size(8)]
    pub _M_function_pointer: FnPtr<fn()>,
}
impl Clone for std__Nocopy_types {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Nocopy_types {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Nocopy_types {}
impl std__Nocopy_types {}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct std__Any_data {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_unused: std__Nocopy_types,
    #[offset(0)]
    #[byte_size(16)]
    pub _M_pod_data: Value<Box<[i8]>>,
}
impl Clone for std__Any_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Any_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Any_data {}
impl std__Any_data {}
impl std__Any_data {}
pub type std__Manager_operation = u32;
pub const std__Manager_operation___get_type_info: std__Manager_operation = 0;
pub const std__Manager_operation___get_functor_ptr: std__Manager_operation = 1;
pub const std__Manager_operation___clone_functor: std__Manager_operation = 2;
pub const std__Manager_operation___destroy_functor: std__Manager_operation = 3;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Function_base {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_functor: std__Any_data,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_manager:
        FnPtr<fn(Ptr<std__Any_data>, Ptr<std__Any_data>, std__Manager_operation) -> bool>,
}
impl Clone for std__Function_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Function_base {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std__Function_base__M_max_size_458: Value<usize> = unimplemented!();
);
thread_local!(
    pub static std__Function_base__M_max_align_459: Value<usize> = unimplemented!();
);
impl std__Function_base {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std__Function_base {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Function_handler_460 {}
impl Clone for _Function_handler_460 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Function_handler_460 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_461 {}
impl Clone for initializer_list_461 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_461 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_462 {}
impl Clone for initializer_list_462 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_462 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_462 {}
impl initializer_list_462 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_463 {}
impl Clone for initializer_list_463 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_463 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_463 {}
impl initializer_list_463 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_464 {}
impl Clone for initializer_list_464 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_464 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_464 {}
impl initializer_list_464 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_465 {}
impl Clone for initializer_list_465 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_465 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_465 {}
impl initializer_list_465 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___true_type {}
impl Clone for std___true_type {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___true_type {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___false_type {}
impl Clone for std___false_type {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___false_type {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___false_type {}
impl std___false_type {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_466 {}
impl Clone for __are_same_466 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_466 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_467 = u32;
pub const anon_467___value: anon_467 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_468 {}
impl Clone for __are_same_468 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_468 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_469 = u32;
pub const anon_469___value: anon_469 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_470 {}
impl Clone for __are_same_470 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_470 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_471 = u32;
pub const anon_471___value: anon_471 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_472 {}
impl Clone for __are_same_472 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_472 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_473 = u32;
pub const anon_473___value: anon_473 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_474 {}
impl Clone for __are_same_474 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_474 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_475 = u32;
pub const anon_475___value: anon_475 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_476 {}
impl Clone for __are_same_476 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_476 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_477 = u32;
pub const anon_477___value: anon_477 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_478 {}
impl Clone for __is_integer_478 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_478 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_479 = u32;
pub const anon_479___value: anon_479 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_480 {}
impl Clone for __is_integer_480 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_480 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_481 = u32;
pub const anon_481___value: anon_481 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_482 {}
impl Clone for __is_integer_482 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_482 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_483 = u32;
pub const anon_483___value: anon_483 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_484 {}
impl Clone for __is_integer_484 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_484 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_485 = u32;
pub const anon_485___value: anon_485 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_486 {}
impl Clone for __is_integer_486 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_486 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_487 = u32;
pub const anon_487___value: anon_487 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_488 {}
impl Clone for __is_integer_488 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_488 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_489 = u32;
pub const anon_489___value: anon_489 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_490 {}
impl Clone for __is_integer_490 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_490 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_491 = u32;
pub const anon_491___value: anon_491 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_492 {}
impl Clone for __is_integer_492 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_492 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_493 = u32;
pub const anon_493___value: anon_493 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_494 {}
impl Clone for __is_integer_494 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_494 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_495 = u32;
pub const anon_495___value: anon_495 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_496 {}
impl Clone for __is_integer_496 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_496 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_497 = u32;
pub const anon_497___value: anon_497 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_498 {}
impl Clone for __memcpyable_integer_498 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_498 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __conditional_type_499 {}
impl Clone for __conditional_type_499 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_type_499 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_500 {}
impl Clone for __is_integer_nonstrict_500 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_500 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_501 = u32;
pub const anon_501___width: anon_501 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_502 {}
impl Clone for __is_integer_nonstrict_502 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_502 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_503 = u32;
pub const anon_503___width: anon_503 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_504 {}
impl Clone for __is_integer_nonstrict_504 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_504 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_505 = u32;
pub const anon_505___width: anon_505 = 32;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_506 {}
impl Clone for __is_integer_nonstrict_506 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_506 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_507 = u32;
pub const anon_507___width: anon_507 = 32;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_508 {}
impl Clone for __is_integer_nonstrict_508 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_508 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_509 = u32;
pub const anon_509___width: anon_509 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_510 {}
impl Clone for __is_integer_nonstrict_510 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_510 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_511 = u32;
pub const anon_511___width: anon_511 = 8;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_512 {}
impl Clone for __is_integer_nonstrict_512 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_512 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_513 = u32;
pub const anon_513___width: anon_513 = 16;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_514 {}
impl Clone for __numeric_traits_integer_514 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_514 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_515: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____digits_516: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____max_517: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_518 {}
impl Clone for __numeric_traits_integer_518 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_518 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____is_signed_519: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____digits_520: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____max_521: Value<i64> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____min_522: Value<i64> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_523 {}
impl Clone for __numeric_traits_integer_523 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_523 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_int____is_signed_524: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_int____digits_525: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_int____max_526: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_int____min_527: Value<i32> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_528 {}
impl Clone for __numeric_traits_integer_528 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_528 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_int____is_signed_529: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_int____digits_530: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_int____max_531: Value<u32> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_532 {}
impl Clone for __numeric_traits_integer_532 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_532 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long_long____is_signed_533: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long_long____digits_534: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long_long____max_535: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_536 {}
impl Clone for __numeric_traits_integer_536 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_536 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_char____is_signed_537: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_char____digits_538: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_char____max_539: Value<i8> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_540 {}
impl Clone for __numeric_traits_integer_540 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_540 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_short____is_signed_541: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_short____digits_542: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_short____max_543: Value<i16> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_short____min_544: Value<i16> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_545 {}
impl Clone for __numeric_traits_floating_545 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_545 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_floating_float____max_exponent10_546: Value<i32> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_547 {}
impl Clone for __numeric_traits_floating_547 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_547 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_floating_double____max_exponent10_548: Value<i32> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_549 {}
impl Clone for __numeric_traits_floating_549 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_549 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_floating_long_double____max_exponent10_550: Value<i32> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_551 {}
impl Clone for __numeric_traits_551 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_551 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_552 {}
impl Clone for __numeric_traits_552 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_552 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_553 {}
impl Clone for __numeric_traits_553 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_553 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_554 {}
impl Clone for __numeric_traits_554 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_554 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_555 {}
impl Clone for __numeric_traits_555 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_555 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_556 {}
impl Clone for __numeric_traits_556 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_556 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_557 {}
impl Clone for __numeric_traits_557 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_557 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_558 {}
impl Clone for __numeric_traits_558 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_558 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_input_iterator_tag {}
impl Clone for std_input_iterator_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_input_iterator_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_input_iterator_tag {}
impl std_input_iterator_tag {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_output_iterator_tag {}
impl Clone for std_output_iterator_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_output_iterator_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_output_iterator_tag {}
impl std_output_iterator_tag {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_forward_iterator_tag {}
impl Clone for std_forward_iterator_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_forward_iterator_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_forward_iterator_tag {}
impl std_forward_iterator_tag {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_bidirectional_iterator_tag {}
impl Clone for std_bidirectional_iterator_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_bidirectional_iterator_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_bidirectional_iterator_tag {}
impl std_bidirectional_iterator_tag {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_random_access_iterator_tag {}
impl Clone for std_random_access_iterator_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_random_access_iterator_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_random_access_iterator_tag {}
impl std_random_access_iterator_tag {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_559 {}
impl Clone for iterator_559 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_559 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_560 {}
impl Clone for iterator_560 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_560 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_560 {}
impl iterator_560 {}
impl iterator_560 {}
impl iterator_560 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_560>) -> Ptr<iterator_560> {
        unimplemented!()
    }
}
impl iterator_560 {
    pub fn move_assign(&self, _a0: Ptr<iterator_560>) -> Ptr<iterator_560> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_561 {}
impl Clone for iterator_561 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_561 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_561 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_561>) -> Ptr<iterator_561> {
        unimplemented!()
    }
}
impl iterator_561 {
    pub fn move_assign(&self, _a0: Ptr<iterator_561>) -> Ptr<iterator_561> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_562 {}
impl Clone for iterator_562 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_562 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_562 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_562>) -> Ptr<iterator_562> {
        unimplemented!()
    }
}
impl iterator_562 {
    pub fn move_assign(&self, _a0: Ptr<iterator_562>) -> Ptr<iterator_562> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_563 {}
impl Clone for iterator_traits_563 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_563 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_564 {}
impl Clone for iterator_traits_564 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_564 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_565 {}
impl Clone for iterator_traits_565 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_565 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_566 {}
impl Clone for iterator_traits_566 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_566 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_567 {}
impl Clone for iterator_traits_567 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_567 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_568 {}
impl Clone for iterator_traits_568 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_568 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_569 {}
impl Clone for iterator_traits_569 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_569 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_570 {}
impl Clone for __iterator_traits_570 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_570 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_571 {}
impl Clone for __iterator_traits_571 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_571 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_572 {}
impl Clone for __ptr_traits_ptr_to_572 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_572 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_572 {
    pub fn pointer_to(__r: Ptr<i8>) -> Ptr<i8> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_573 {}
impl Clone for __ptr_traits_ptr_to_573 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_573 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_573 {
    pub fn pointer_to(__r: Ptr<i8>) -> Ptr<i8> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_574 {}
impl Clone for __ptr_traits_ptr_to_574 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_574 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_574 {
    pub fn pointer_to(__r: Ptr<i32>) -> Ptr<i32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_575 {}
impl Clone for __ptr_traits_ptr_to_575 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_575 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_575 {
    pub fn pointer_to(__r: Ptr<i32>) -> Ptr<i32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_576 {}
impl Clone for __ptr_traits_ptr_to_576 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_576 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_576 {
    pub fn pointer_to(__r: Ptr<u16>) -> Ptr<u16> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_577 {}
impl Clone for __ptr_traits_ptr_to_577 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_577 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_577 {
    pub fn pointer_to(__r: Ptr<u16>) -> Ptr<u16> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_578 {}
impl Clone for __ptr_traits_ptr_to_578 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_578 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_578 {
    pub fn pointer_to(__r: Ptr<u32>) -> Ptr<u32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_579 {}
impl Clone for __ptr_traits_ptr_to_579 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_579 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_579 {
    pub fn pointer_to(__r: Ptr<u32>) -> Ptr<u32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_580 {}
impl Clone for pointer_traits_580 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_580 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_581 {}
impl Clone for pointer_traits_581 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_581 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_582 {}
impl Clone for pointer_traits_582 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_582 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_583 {}
impl Clone for pointer_traits_583 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_583 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_584 {}
impl Clone for pointer_traits_584 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_584 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_585 {}
impl Clone for pointer_traits_585 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_585 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_586 {}
impl Clone for pointer_traits_586 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_586 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_587 {}
impl Clone for pointer_traits_587 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_587 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_588 {}
impl Clone for reverse_iterator_588 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_588 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_588 {
    pub fn copy_from(__x: Ptr<reverse_iterator_588>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_588 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_588>) -> Ptr<reverse_iterator_588> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_589 {}
impl Clone for reverse_iterator_589 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_589 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_589 {
    pub fn copy_from(__x: Ptr<reverse_iterator_589>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_589 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_589>) -> Ptr<reverse_iterator_589> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_590 {}
impl Clone for __normal_iterator_590 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_590 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_590 {}
impl __normal_iterator_590 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __gnu_cxx___ops__Iter_less_iter {}
impl Clone for __gnu_cxx___ops__Iter_less_iter {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___ops__Iter_less_iter {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Iter_less_iter {}
impl __gnu_cxx___ops__Iter_less_iter {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __gnu_cxx___ops__Iter_less_val {}
impl Clone for __gnu_cxx___ops__Iter_less_val {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___ops__Iter_less_val {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Iter_less_val {}
impl __gnu_cxx___ops__Iter_less_val {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __gnu_cxx___ops__Val_less_iter {}
impl Clone for __gnu_cxx___ops__Val_less_iter {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___ops__Val_less_iter {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Val_less_iter {}
impl __gnu_cxx___ops__Val_less_iter {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __gnu_cxx___ops__Iter_equal_to_iter {}
impl Clone for __gnu_cxx___ops__Iter_equal_to_iter {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___ops__Iter_equal_to_iter {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Iter_equal_to_iter {}
impl __gnu_cxx___ops__Iter_equal_to_iter {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __gnu_cxx___ops__Iter_equal_to_val {}
impl Clone for __gnu_cxx___ops__Iter_equal_to_val {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___ops__Iter_equal_to_val {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Iter_equal_to_val {}
impl __gnu_cxx___ops__Iter_equal_to_val {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_591 {}
impl Clone for char_traits_591 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_591 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_591 {
    pub fn assign_1(__c1: Ptr<i8>, __c2: Ptr<i8>) {
        unimplemented!()
    }
}
impl char_traits_591 {
    pub fn lt(__c1: Ptr<i8>, __c2: Ptr<i8>) -> bool {
        unimplemented!()
    }
}
impl char_traits_591 {
    pub fn length(mut __s: Ptr<i8>) -> usize {
        unimplemented!()
    }
}
impl char_traits_591 {
    pub fn move_(mut __s1: Ptr<i8>, mut __s2: Ptr<i8>, mut __n: usize) -> Ptr<i8> {
        unimplemented!()
    }
}
impl char_traits_591 {
    pub fn copy(mut __s1: Ptr<i8>, mut __s2: Ptr<i8>, mut __n: usize) -> Ptr<i8> {
        unimplemented!()
    }
}
impl char_traits_591 {
    pub fn eof() -> i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_592 {}
impl Clone for char_traits_592 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_592 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_592 {
    pub fn assign_1(__c1: Ptr<i32>, __c2: Ptr<i32>) {
        unimplemented!()
    }
}
impl char_traits_592 {
    pub fn copy(mut __s1: Ptr<i32>, mut __s2: Ptr<i32>, mut __n: usize) -> Ptr<i32> {
        unimplemented!()
    }
}
impl char_traits_592 {
    pub fn eq_int_type(__c1: Ptr<u32>, __c2: Ptr<u32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_592 {
    pub fn eof() -> u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_593 {}
impl Clone for char_traits_593 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_593 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_593 {
    pub fn assign_1(__c1: Ptr<u16>, __c2: Ptr<u16>) {
        unimplemented!()
    }
}
impl char_traits_593 {
    pub fn eq(__c1: Ptr<u16>, __c2: Ptr<u16>) -> bool {
        unimplemented!()
    }
}
impl char_traits_593 {
    pub fn lt(__c1: Ptr<u16>, __c2: Ptr<u16>) -> bool {
        unimplemented!()
    }
}
impl char_traits_593 {
    pub fn copy(mut __s1: Ptr<u16>, mut __s2: Ptr<u16>, mut __n: usize) -> Ptr<u16> {
        unimplemented!()
    }
}
impl char_traits_593 {
    pub fn eq_int_type(__c1: Ptr<u16>, __c2: Ptr<u16>) -> bool {
        unimplemented!()
    }
}
impl char_traits_593 {
    pub fn eof() -> u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_594 {}
impl Clone for char_traits_594 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_594 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_594 {
    pub fn assign_1(__c1: Ptr<u32>, __c2: Ptr<u32>) {
        unimplemented!()
    }
}
impl char_traits_594 {
    pub fn eq(__c1: Ptr<u32>, __c2: Ptr<u32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_594 {
    pub fn lt(__c1: Ptr<u32>, __c2: Ptr<u32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_594 {
    pub fn copy(mut __s1: Ptr<u32>, mut __s2: Ptr<u32>, mut __n: usize) -> Ptr<u32> {
        unimplemented!()
    }
}
impl char_traits_594 {
    pub fn eq_int_type(__c1: Ptr<u32>, __c2: Ptr<u32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_594 {
    pub fn eof() -> u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_595 {}
impl Clone for allocator_595 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_595 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_595 {}
impl allocator_595 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_596 {}
impl Clone for allocator_596 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_596 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_596 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_596 {
    pub fn copy_from(__a: Ptr<allocator_596>) -> Self {
        unimplemented!()
    }
}
impl allocator_596 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_596>) -> Ptr<allocator_596> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_597 {}
impl Clone for allocator_597 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_597 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_597 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_597 {
    pub fn copy_from(__a: Ptr<allocator_597>) -> Self {
        unimplemented!()
    }
}
impl allocator_597 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_597>) -> Ptr<allocator_597> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_598 {}
impl Clone for allocator_598 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_598 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_599 {}
impl Clone for rebind_599 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_599 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_600 {}
impl Clone for rebind_600 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_600 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_598 {
    pub fn copy_from(__a: Ptr<allocator_598>) -> Self {
        unimplemented!()
    }
}
impl allocator_598 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_598>) -> Ptr<allocator_598> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_601 {}
impl Clone for allocator_601 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_601 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_602 {}
impl Clone for rebind_602 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_602 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_603 {}
impl Clone for rebind_603 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_603 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_601 {
    pub fn copy_from(__a: Ptr<allocator_601>) -> Self {
        unimplemented!()
    }
}
impl allocator_601 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_601>) -> Ptr<allocator_601> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_604 {}
impl Clone for allocator_604 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_604 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_604 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_604 {
    pub fn copy_from(__a: Ptr<allocator_604>) -> Self {
        unimplemented!()
    }
}
impl allocator_604 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_604>) -> Ptr<allocator_604> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_605 {}
impl Clone for allocator_605 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_605 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_605 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_605 {
    pub fn copy_from(__a: Ptr<allocator_605>) -> Self {
        unimplemented!()
    }
}
impl allocator_605 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_605>) -> Ptr<allocator_605> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_606 {}
impl Clone for allocator_traits_606 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_606 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_607 {}
impl Clone for allocator_traits_607 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_607 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_608 {}
impl Clone for allocator_traits_608 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_608 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_608 {
    pub fn allocate_1(__a: Ptr<allocator_596>, mut __n: u64) -> Ptr<i8> {
        unimplemented!()
    }
}
impl allocator_traits_608 {
    pub fn deallocate(__a: Ptr<allocator_596>, mut __p: Ptr<i8>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_608 {
    pub fn max_size(__a: Ptr<allocator_596>) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_609 {}
impl Clone for allocator_traits_609 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_609 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_609 {
    pub fn allocate_1(__a: Ptr<allocator_597>, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
}
impl allocator_traits_609 {
    pub fn deallocate(__a: Ptr<allocator_597>, mut __p: Ptr<i32>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_609 {
    pub fn max_size(__a: Ptr<allocator_597>) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_610 {}
impl Clone for allocator_traits_610 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_610 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_610 {
    pub fn allocate_1(__a: Ptr<allocator_604>, mut __n: u64) -> Ptr<u16> {
        unimplemented!()
    }
}
impl allocator_traits_610 {
    pub fn deallocate(__a: Ptr<allocator_604>, mut __p: Ptr<u16>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_610 {
    pub fn max_size(__a: Ptr<allocator_604>) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_611 {}
impl Clone for allocator_traits_611 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_611 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_611 {
    pub fn allocate_1(__a: Ptr<allocator_605>, mut __n: u64) -> Ptr<u32> {
        unimplemented!()
    }
}
impl allocator_traits_611 {
    pub fn deallocate(__a: Ptr<allocator_605>, mut __p: Ptr<u32>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_611 {
    pub fn max_size(__a: Ptr<allocator_605>) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_612 {}
impl Clone for __new_allocator_612 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_612 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_612 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_612 {
    pub fn copy_from(_a0: Ptr<__new_allocator_612>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_612 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_612>) -> Ptr<__new_allocator_612> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_613 {}
impl Clone for __new_allocator_613 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_613 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_613 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_613 {
    pub fn copy_from(_a0: Ptr<__new_allocator_613>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_613 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_613>) -> Ptr<__new_allocator_613> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_614 {}
impl Clone for __new_allocator_614 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_614 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_614 {
    pub fn copy_from(_a0: Ptr<__new_allocator_614>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_614 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_614>) -> Ptr<__new_allocator_614> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_615 {}
impl Clone for __new_allocator_615 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_615 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_615 {
    pub fn copy_from(_a0: Ptr<__new_allocator_615>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_615 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_615>) -> Ptr<__new_allocator_615> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_616 {}
impl Clone for __new_allocator_616 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_616 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_616 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_616 {
    pub fn copy_from(_a0: Ptr<__new_allocator_616>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_616 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_616>) -> Ptr<__new_allocator_616> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_617 {}
impl Clone for __new_allocator_617 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_617 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_617 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_617 {
    pub fn copy_from(_a0: Ptr<__new_allocator_617>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_617 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_617>) -> Ptr<__new_allocator_617> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___allocator_traits_base {}
impl Clone for std___allocator_traits_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___allocator_traits_base {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __rebind_619 {}
impl Clone for __rebind_619 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __rebind_619 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __rebind_620 {}
impl Clone for __rebind_620 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __rebind_620 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_allocator_621 {}
impl Clone for __is_allocator_621 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_621 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_allocator_622 {}
impl Clone for __is_allocator_622 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_622 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_allocator_623 {}
impl Clone for __is_allocator_623 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_623 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_allocator_624 {}
impl Clone for __is_allocator_624 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_624 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_625 {}
impl Clone for __alloc_traits_625 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_625 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_626 {}
impl Clone for __alloc_traits_626 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_626 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_627 {}
impl Clone for rebind_627 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_627 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_628 {}
impl Clone for __alloc_traits_628 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_628 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_629 {}
impl Clone for rebind_629 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_629 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_630 {}
impl Clone for __alloc_traits_630 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_630 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_631 {}
impl Clone for rebind_631 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_631 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_632 {}
impl Clone for __alloc_traits_632 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_632 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_633 {}
impl Clone for rebind_633 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_633 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(56)]
pub struct _Hashtable_634 {}
impl Clone for _Hashtable_634 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Hashtable_634 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Hashtable_634 {
    pub fn copy_from(_a0: Ptr<_Hashtable_634>) -> Self {
        unimplemented!()
    }
}
impl _Hashtable_634 {
    pub fn move_from(__ht: Ptr<_Hashtable_634>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(56)]
pub struct _Hashtable_635 {}
impl Clone for _Hashtable_635 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Hashtable_635 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Hashtable_635 {
    pub fn copy_from(_a0: Ptr<_Hashtable_635>) -> Self {
        unimplemented!()
    }
}
impl _Hashtable_635 {
    pub fn move_from(__ht: Ptr<_Hashtable_635>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Hashtable_base_636 {}
impl Clone for _Hashtable_base_636 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Hashtable_base_636 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Hashtable_base_637 {}
impl Clone for _Hashtable_base_637 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Hashtable_base_637 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___detail__Identity {}
impl Clone for std___detail__Identity {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___detail__Identity {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___detail__Select1st {}
impl Clone for std___detail__Select1st {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___detail__Select1st {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __1st_type_641 {}
impl Clone for __1st_type_641 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __1st_type_641 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___detail__Select1st {}
impl std___detail__Select1st {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _NodeBuilder_642 {}
impl Clone for _NodeBuilder_642 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _NodeBuilder_642 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Hashtable_alloc_643 {}
impl Clone for _Hashtable_alloc_643 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Hashtable_alloc_643 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __get_value_type_644 {}
impl Clone for __get_value_type_644 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __get_value_type_644 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Hashtable_alloc_643 {
    pub fn copy_from(_a0: Ptr<_Hashtable_alloc_643>) -> Self {
        unimplemented!()
    }
}
impl _Hashtable_alloc_643 {
    pub fn move_from(_a0: Ptr<_Hashtable_alloc_643>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Hashtable_traits_645 {}
impl Clone for _Hashtable_traits_645 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Hashtable_traits_645 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Hashtable_traits_646 {}
impl Clone for _Hashtable_traits_646 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Hashtable_traits_646 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct std___detail__Hash_node_base {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_nxt: Ptr<std___detail__Hash_node_base>,
}
impl Clone for std___detail__Hash_node_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___detail__Hash_node_base {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct _Hash_node_code_cache_647 {
    #[offset(0)]
    pub _M_hash_code: usize,
}
impl Clone for _Hash_node_code_cache_647 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Hash_node_code_cache_647 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___detail__Mod_range_hashing {}
impl Clone for std___detail__Mod_range_hashing {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___detail__Mod_range_hashing {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___detail__Mod_range_hashing {}
impl std___detail__Mod_range_hashing {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___detail__Default_ranged_hash {}
impl Clone for std___detail__Default_ranged_hash {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___detail__Default_ranged_hash {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct std___detail__Prime_rehash_policy {
    #[offset(0)]
    pub _M_max_load_factor: f32,
    #[offset(8)]
    pub _M_next_resize: usize,
}
impl Clone for std___detail__Prime_rehash_policy {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___detail__Prime_rehash_policy {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct std___detail__Power2_rehash_policy {
    #[offset(0)]
    pub _M_max_load_factor: f32,
    #[offset(8)]
    pub _M_next_resize: usize,
}
impl Clone for std___detail__Power2_rehash_policy {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___detail__Power2_rehash_policy {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___detail__Power2_rehash_policy__S_growth_factor_648: Value<usize> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Map_base_649 {}
impl Clone for _Map_base_649 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Map_base_649 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Map_base_650 {}
impl Clone for _Map_base_650 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Map_base_650 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Rehash_base_651 {}
impl Clone for _Rehash_base_651 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Rehash_base_651 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Rehash_base_652 {}
impl Clone for _Rehash_base_652 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Rehash_base_652 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Hashtable_ebo_helper_653 {
    #[offset(0)]
    #[byte_size(1)]
    pub _M_obj: hash_431,
}
impl Clone for _Hashtable_ebo_helper_653 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Hashtable_ebo_helper_653 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Hashtable_ebo_helper_654 {
    #[offset(0)]
    #[byte_size(1)]
    pub _M_obj: equal_to_391,
}
impl Clone for _Hashtable_ebo_helper_654 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Hashtable_ebo_helper_654 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Hashtable_ebo_helper_655 {
    #[offset(0)]
    #[byte_size(1)]
    pub _M_obj: allocator_601,
}
impl Clone for _Hashtable_ebo_helper_655 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Hashtable_ebo_helper_655 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Hashtable_ebo_helper_655 {}
impl _Hashtable_ebo_helper_655 {}
impl _Hashtable_ebo_helper_655 {
    pub fn copy_assign(
        &self,
        _a0: Ptr<_Hashtable_ebo_helper_655>,
    ) -> Ptr<_Hashtable_ebo_helper_655> {
        unimplemented!()
    }
}
impl _Hashtable_ebo_helper_655 {
    pub fn move_assign(
        &self,
        _a0: Ptr<_Hashtable_ebo_helper_655>,
    ) -> Ptr<_Hashtable_ebo_helper_655> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Hash_code_base_656 {}
impl Clone for _Hash_code_base_656 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Hash_code_base_656 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std__Enable_default_constructor_tag {}
impl Clone for std__Enable_default_constructor_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Enable_default_constructor_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Enable_default_constructor_tag {}
impl std__Enable_default_constructor_tag {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Enable_default_constructor_657 {}
impl Clone for _Enable_default_constructor_657 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Enable_default_constructor_657 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Enable_default_constructor_657 {
    pub fn copy_from(_a0: Ptr<_Enable_default_constructor_657>) -> Self {
        unimplemented!()
    }
}
impl _Enable_default_constructor_657 {
    pub fn move_from(_a0: Ptr<_Enable_default_constructor_657>) -> Self {
        unimplemented!()
    }
}
impl _Enable_default_constructor_657 {
    pub fn copy_assign(
        &self,
        _a0: Ptr<_Enable_default_constructor_657>,
    ) -> Ptr<_Enable_default_constructor_657> {
        unimplemented!()
    }
}
impl _Enable_default_constructor_657 {
    pub fn move_assign(
        &self,
        _a0: Ptr<_Enable_default_constructor_657>,
    ) -> Ptr<_Enable_default_constructor_657> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(56)]
pub struct unordered_multimap_658 {}
impl Clone for unordered_multimap_658 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for unordered_multimap_658 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl unordered_multimap_658 {
    pub fn copy_from(_a0: Ptr<unordered_multimap_658>) -> Self {
        unimplemented!()
    }
}
impl unordered_multimap_658 {
    pub fn move_from(_a0: Ptr<unordered_multimap_658>) -> Self {
        unimplemented!()
    }
}
impl unordered_multimap_658 {
    pub fn copy_assign(&self, _a0: Ptr<unordered_multimap_658>) -> Ptr<unordered_multimap_658> {
        unimplemented!()
    }
}
impl unordered_multimap_658 {
    pub fn move_assign(&self, _a0: Ptr<unordered_multimap_658>) -> Ptr<unordered_multimap_658> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(56)]
pub struct unordered_map_659 {}
impl Clone for unordered_map_659 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for unordered_map_659 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl unordered_map_659 {
    pub fn copy_from(_a0: Ptr<unordered_map_659>) -> Self {
        unimplemented!()
    }
}
impl unordered_map_659 {
    pub fn move_from(_a0: Ptr<unordered_map_659>) -> Self {
        unimplemented!()
    }
}
impl unordered_map_659 {
    pub fn copy_assign(&self, _a0: Ptr<unordered_map_659>) -> Ptr<unordered_map_659> {
        unimplemented!()
    }
}
impl unordered_map_659 {
    pub fn move_assign(&self, _a0: Ptr<unordered_map_659>) -> Ptr<unordered_map_659> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct max_align_t {
    #[offset(0)]
    pub __clang_max_align_nonce1: i64,
    #[offset(16)]
    #[byte_size(16)]
    pub __clang_max_align_nonce2: f64,
}
impl Clone for max_align_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for max_align_t {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type std_byte = u8;
pub type anon_660 = u32;
pub const anon_660__S_word_bit: anon_660 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct std__Bit_reference {}
impl Clone for std__Bit_reference {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Bit_reference {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Bit_reference {
    pub fn copy_from(_a0: Ptr<std__Bit_reference>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct std__Bit_iterator_base {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_p: Ptr<u64>,
    #[offset(8)]
    pub _M_offset: u32,
}
impl Clone for std__Bit_iterator_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Bit_iterator_base {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub fn new(mut __x: Ptr<u64>, mut __y: u32) -> Self {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {}
impl std__Bit_iterator_base {}
impl std__Bit_iterator_base {
    pub fn copy_assign(&self, _a0: Ptr<std__Bit_iterator_base>) -> Ptr<std__Bit_iterator_base> {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub fn move_assign(&self, _a0: Ptr<std__Bit_iterator_base>) -> Ptr<std__Bit_iterator_base> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct std__Bit_iterator {}
impl Clone for std__Bit_iterator {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Bit_iterator {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Bit_iterator {
    pub fn new_1(mut __x: Ptr<u64>, mut __y: u32) -> Self {
        unimplemented!()
    }
}
impl std__Bit_iterator {}
impl std__Bit_iterator {}
impl std__Bit_iterator {
    pub fn copy_assign(&self, _a0: Ptr<std__Bit_iterator>) -> Ptr<std__Bit_iterator> {
        unimplemented!()
    }
}
impl std__Bit_iterator {
    pub fn move_assign(&self, _a0: Ptr<std__Bit_iterator>) -> Ptr<std__Bit_iterator> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct std__Bit_const_iterator {}
impl Clone for std__Bit_const_iterator {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Bit_const_iterator {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Bit_const_iterator {}
impl std__Bit_const_iterator {}
impl std__Bit_const_iterator {
    pub fn copy_assign(&self, _a0: Ptr<std__Bit_const_iterator>) -> Ptr<std__Bit_const_iterator> {
        unimplemented!()
    }
}
impl std__Bit_const_iterator {
    pub fn move_assign(&self, _a0: Ptr<std__Bit_const_iterator>) -> Ptr<std__Bit_const_iterator> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_661 {}
impl Clone for _Placeholder_661 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_661 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_661 {}
impl _Placeholder_661 {}
impl _Placeholder_661 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_662 {}
impl Clone for _Placeholder_662 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_662 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_662 {}
impl _Placeholder_662 {}
impl _Placeholder_662 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_663 {}
impl Clone for _Placeholder_663 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_663 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_663 {}
impl _Placeholder_663 {}
impl _Placeholder_663 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_664 {}
impl Clone for _Placeholder_664 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_664 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_664 {}
impl _Placeholder_664 {}
impl _Placeholder_664 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_665 {}
impl Clone for _Placeholder_665 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_665 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_665 {}
impl _Placeholder_665 {}
impl _Placeholder_665 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_666 {}
impl Clone for _Placeholder_666 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_666 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_666 {}
impl _Placeholder_666 {}
impl _Placeholder_666 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_667 {}
impl Clone for _Placeholder_667 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_667 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_667 {}
impl _Placeholder_667 {}
impl _Placeholder_667 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_668 {}
impl Clone for _Placeholder_668 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_668 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_668 {}
impl _Placeholder_668 {}
impl _Placeholder_668 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_669 {}
impl Clone for _Placeholder_669 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_669 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_669 {}
impl _Placeholder_669 {}
impl _Placeholder_669 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_670 {}
impl Clone for _Placeholder_670 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_670 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_670 {}
impl _Placeholder_670 {}
impl _Placeholder_670 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_671 {}
impl Clone for _Placeholder_671 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_671 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_671 {}
impl _Placeholder_671 {}
impl _Placeholder_671 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_672 {}
impl Clone for _Placeholder_672 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_672 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_672 {}
impl _Placeholder_672 {}
impl _Placeholder_672 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_673 {}
impl Clone for _Placeholder_673 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_673 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_673 {}
impl _Placeholder_673 {}
impl _Placeholder_673 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_674 {}
impl Clone for _Placeholder_674 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_674 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_674 {}
impl _Placeholder_674 {}
impl _Placeholder_674 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_675 {}
impl Clone for _Placeholder_675 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_675 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_675 {}
impl _Placeholder_675 {}
impl _Placeholder_675 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_676 {}
impl Clone for _Placeholder_676 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_676 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_676 {}
impl _Placeholder_676 {}
impl _Placeholder_676 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_677 {}
impl Clone for _Placeholder_677 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_677 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_677 {}
impl _Placeholder_677 {}
impl _Placeholder_677 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_678 {}
impl Clone for _Placeholder_678 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_678 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_678 {}
impl _Placeholder_678 {}
impl _Placeholder_678 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_679 {}
impl Clone for _Placeholder_679 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_679 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_679 {}
impl _Placeholder_679 {}
impl _Placeholder_679 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_680 {}
impl Clone for _Placeholder_680 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_680 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_680 {}
impl _Placeholder_680 {}
impl _Placeholder_680 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_681 {}
impl Clone for _Placeholder_681 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_681 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_681 {}
impl _Placeholder_681 {}
impl _Placeholder_681 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_682 {}
impl Clone for _Placeholder_682 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_682 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_682 {}
impl _Placeholder_682 {}
impl _Placeholder_682 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_683 {}
impl Clone for _Placeholder_683 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_683 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_683 {}
impl _Placeholder_683 {}
impl _Placeholder_683 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_684 {}
impl Clone for _Placeholder_684 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_684 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_684 {}
impl _Placeholder_684 {}
impl _Placeholder_684 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_685 {}
impl Clone for _Placeholder_685 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_685 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_685 {}
impl _Placeholder_685 {}
impl _Placeholder_685 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_686 {}
impl Clone for _Placeholder_686 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_686 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_686 {}
impl _Placeholder_686 {}
impl _Placeholder_686 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_687 {}
impl Clone for _Placeholder_687 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_687 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_687 {}
impl _Placeholder_687 {}
impl _Placeholder_687 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_688 {}
impl Clone for _Placeholder_688 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_688 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_688 {}
impl _Placeholder_688 {}
impl _Placeholder_688 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Placeholder_689 {}
impl Clone for _Placeholder_689 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_689 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_689 {}
impl _Placeholder_689 {}
impl _Placeholder_689 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct basic_string_690 {}
impl Clone for basic_string_690 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_690 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct std_basic_string_char___Alloc_hider {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_p: Ptr<i8>,
}
impl Clone for std_basic_string_char___Alloc_hider {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_basic_string_char___Alloc_hider {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_basic_string_char___Alloc_hider {
    pub fn new_1(mut __dat: Ptr<i8>, __a: Ptr<allocator_596>) -> Self {
        unimplemented!()
    }
}
impl std_basic_string_char___Alloc_hider {
    pub fn new_2(mut __dat: Ptr<i8>, __a: Option<Ptr<allocator_596>>) -> Self {
        unimplemented!()
    }
}
impl std_basic_string_char___Alloc_hider {}
impl std_basic_string_char___Alloc_hider {}
impl std_basic_string_char___Alloc_hider {
    pub fn copy_assign(
        &self,
        _a0: Ptr<std_basic_string_char___Alloc_hider>,
    ) -> Ptr<std_basic_string_char___Alloc_hider> {
        unimplemented!()
    }
}
impl std_basic_string_char___Alloc_hider {
    pub fn move_assign(
        &self,
        _a0: Ptr<std_basic_string_char___Alloc_hider>,
    ) -> Ptr<std_basic_string_char___Alloc_hider> {
        unimplemented!()
    }
}
pub type anon_692 = u32;
pub const anon_692__S_local_capacity: anon_692 = 15;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_691 {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_local_buf: Value<Box<[i8]>>,
    #[offset(0)]
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_691 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_691 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_690 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl basic_string_690 {
    pub fn copy_from(__str: Ptr<basic_string_690>) -> Self {
        unimplemented!()
    }
}
impl basic_string_690 {
    pub fn new_2(mut __s: Ptr<i8>, mut __n: u64, __a: Option<Ptr<allocator_596>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_690 {
    pub fn move_from(__str: Ptr<basic_string_690>) -> Self {
        unimplemented!()
    }
}
impl basic_string_690 {
    pub fn new_3(__beg: Ptr<i8>, mut __end: Ptr<i8>, __a: Option<Ptr<allocator_596>>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct basic_string_700 {}
impl Clone for basic_string_700 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_700 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct std_basic_string_wchar_t___Alloc_hider {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_p: Ptr<i32>,
}
impl Clone for std_basic_string_wchar_t___Alloc_hider {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_basic_string_wchar_t___Alloc_hider {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_basic_string_wchar_t___Alloc_hider {
    pub fn new_1(mut __dat: Ptr<i32>, __a: Ptr<allocator_597>) -> Self {
        unimplemented!()
    }
}
impl std_basic_string_wchar_t___Alloc_hider {}
impl std_basic_string_wchar_t___Alloc_hider {}
impl std_basic_string_wchar_t___Alloc_hider {
    pub fn copy_assign(
        &self,
        _a0: Ptr<std_basic_string_wchar_t___Alloc_hider>,
    ) -> Ptr<std_basic_string_wchar_t___Alloc_hider> {
        unimplemented!()
    }
}
impl std_basic_string_wchar_t___Alloc_hider {
    pub fn move_assign(
        &self,
        _a0: Ptr<std_basic_string_wchar_t___Alloc_hider>,
    ) -> Ptr<std_basic_string_wchar_t___Alloc_hider> {
        unimplemented!()
    }
}
pub type anon_702 = u32;
pub const anon_702__S_local_capacity: anon_702 = 3;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_701 {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_local_buf: Value<Box<[i32]>>,
    #[offset(0)]
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_701 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_701 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_700 {
    pub fn copy_from(__str: Ptr<basic_string_700>) -> Self {
        unimplemented!()
    }
}
impl basic_string_700 {
    pub fn new_1(mut __s: Ptr<i32>, mut __n: u64, __a: Option<Ptr<allocator_597>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_700 {
    pub fn move_from(__str: Ptr<basic_string_700>) -> Self {
        unimplemented!()
    }
}
impl basic_string_700 {
    pub fn new_2(__beg: Ptr<i8>, mut __end: Ptr<i8>, __a: Option<Ptr<allocator_597>>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct basic_string_703 {}
impl Clone for basic_string_703 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_703 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct std_basic_string_char16_t___Alloc_hider {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_p: Ptr<u16>,
}
impl Clone for std_basic_string_char16_t___Alloc_hider {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_basic_string_char16_t___Alloc_hider {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_basic_string_char16_t___Alloc_hider {
    pub fn new_1(mut __dat: Ptr<u16>, __a: Ptr<allocator_604>) -> Self {
        unimplemented!()
    }
}
impl std_basic_string_char16_t___Alloc_hider {}
impl std_basic_string_char16_t___Alloc_hider {}
impl std_basic_string_char16_t___Alloc_hider {
    pub fn copy_assign(
        &self,
        _a0: Ptr<std_basic_string_char16_t___Alloc_hider>,
    ) -> Ptr<std_basic_string_char16_t___Alloc_hider> {
        unimplemented!()
    }
}
impl std_basic_string_char16_t___Alloc_hider {
    pub fn move_assign(
        &self,
        _a0: Ptr<std_basic_string_char16_t___Alloc_hider>,
    ) -> Ptr<std_basic_string_char16_t___Alloc_hider> {
        unimplemented!()
    }
}
pub type anon_705 = u32;
pub const anon_705__S_local_capacity: anon_705 = 7;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_704 {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_local_buf: Value<Box<[u16]>>,
    #[offset(0)]
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_704 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_704 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_703 {
    pub fn copy_from(__str: Ptr<basic_string_703>) -> Self {
        unimplemented!()
    }
}
impl basic_string_703 {
    pub fn new_1(mut __s: Ptr<u16>, mut __n: u64, __a: Option<Ptr<allocator_604>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_703 {
    pub fn move_from(__str: Ptr<basic_string_703>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct basic_string_706 {}
impl Clone for basic_string_706 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_706 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct std_basic_string_char32_t___Alloc_hider {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_p: Ptr<u32>,
}
impl Clone for std_basic_string_char32_t___Alloc_hider {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_basic_string_char32_t___Alloc_hider {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_basic_string_char32_t___Alloc_hider {
    pub fn new_1(mut __dat: Ptr<u32>, __a: Ptr<allocator_605>) -> Self {
        unimplemented!()
    }
}
impl std_basic_string_char32_t___Alloc_hider {}
impl std_basic_string_char32_t___Alloc_hider {}
impl std_basic_string_char32_t___Alloc_hider {
    pub fn copy_assign(
        &self,
        _a0: Ptr<std_basic_string_char32_t___Alloc_hider>,
    ) -> Ptr<std_basic_string_char32_t___Alloc_hider> {
        unimplemented!()
    }
}
impl std_basic_string_char32_t___Alloc_hider {
    pub fn move_assign(
        &self,
        _a0: Ptr<std_basic_string_char32_t___Alloc_hider>,
    ) -> Ptr<std_basic_string_char32_t___Alloc_hider> {
        unimplemented!()
    }
}
pub type anon_708 = u32;
pub const anon_708__S_local_capacity: anon_708 = 3;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_707 {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_local_buf: Value<Box<[u32]>>,
    #[offset(0)]
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_707 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_707 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_706 {
    pub fn copy_from(__str: Ptr<basic_string_706>) -> Self {
        unimplemented!()
    }
}
impl basic_string_706 {
    pub fn new_1(mut __s: Ptr<u32>, mut __n: u64, __a: Option<Ptr<allocator_605>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_706 {
    pub fn move_from(__str: Ptr<basic_string_706>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __mbstate_t {
    #[offset(0)]
    pub __count: i32,
    #[offset(4)]
    #[byte_size(4)]
    pub __value: anon_709,
}
impl Clone for __mbstate_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __mbstate_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct anon_709 {
    #[offset(0)]
    pub __wch: u32,
    #[offset(0)]
    #[byte_size(4)]
    pub __wchb: Value<Box<[i8]>>,
}
impl Clone for anon_709 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_709 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(232)]
pub struct __locale_struct {
    #[offset(0)]
    #[byte_size(104)]
    pub __locales: Value<Box<[Ptr<__locale_data>]>>,
    #[offset(104)]
    #[byte_size(8)]
    pub __ctype_b: Ptr<u16>,
    #[offset(112)]
    #[byte_size(8)]
    pub __ctype_tolower: Ptr<i32>,
    #[offset(120)]
    #[byte_size(8)]
    pub __ctype_toupper: Ptr<i32>,
    #[offset(128)]
    #[byte_size(104)]
    pub __names: Value<Box<[Ptr<i8>]>>,
}
impl Clone for __locale_struct {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __locale_struct {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __locale_struct {}
impl __locale_struct {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(264)]
pub struct basic_ios_710 {}
impl Clone for basic_ios_710 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_ios_710 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(264)]
pub struct basic_ios_711 {}
impl Clone for basic_ios_711 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_ios_711 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(64)]
pub struct basic_streambuf_712 {}
impl Clone for basic_streambuf_712 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_streambuf_712 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(64)]
pub struct basic_streambuf_713 {}
impl Clone for basic_streambuf_713 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_streambuf_713 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(280)]
pub struct basic_istream_714 {}
impl Clone for basic_istream_714 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_istream_714 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(280)]
pub struct basic_istream_715 {}
impl Clone for basic_istream_715 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_istream_715 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(272)]
pub struct basic_ostream_716 {}
impl Clone for basic_ostream_716 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_ostream_716 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(272)]
pub struct basic_ostream_717 {}
impl Clone for basic_ostream_717 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_ostream_717 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(288)]
pub struct basic_iostream_718 {}
impl Clone for basic_iostream_718 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_iostream_718 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(288)]
pub struct basic_iostream_719 {}
impl Clone for basic_iostream_719 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_iostream_719 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct std___exception_ptr_exception_ptr {}
impl Clone for std___exception_ptr_exception_ptr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___exception_ptr_exception_ptr {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptr {
    pub fn copy_from(__other: Ptr<std___exception_ptr_exception_ptr>) -> Self {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptr {
    pub fn move_from(__o: Ptr<std___exception_ptr_exception_ptr>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct std_nested_exception {}
impl Clone for std_nested_exception {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_nested_exception {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_nested_exception {
    pub fn copy_from(_a0: Ptr<std_nested_exception>) -> Self {
        unimplemented!()
    }
}
impl std_nested_exception {
    pub fn copy_assign(&self, _a0: Ptr<std_nested_exception>) -> Ptr<std_nested_exception> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Char_types_720 {}
impl Clone for _Char_types_720 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Char_types_720 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Char_types_721 {}
impl Clone for _Char_types_721 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Char_types_721 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_722 {}
impl Clone for char_traits_722 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_722 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_722 {
    pub fn eq(__c1: Ptr<i8>, __c2: Ptr<i8>) -> bool {
        unimplemented!()
    }
}
impl char_traits_722 {
    pub fn length(mut __p: Ptr<i8>) -> usize {
        unimplemented!()
    }
}
impl char_traits_722 {
    pub fn find(mut __s: Ptr<i8>, mut __n: usize, __a: Ptr<i8>) -> Ptr<i8> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_723 {}
impl Clone for char_traits_723 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_723 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_723 {
    pub fn eq(__c1: Ptr<i32>, __c2: Ptr<i32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_723 {
    pub fn lt(__c1: Ptr<i32>, __c2: Ptr<i32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_723 {
    pub fn compare(mut __s1: Ptr<i32>, mut __s2: Ptr<i32>, mut __n: usize) -> i32 {
        unimplemented!()
    }
}
impl char_traits_723 {
    pub fn length(mut __p: Ptr<i32>) -> usize {
        unimplemented!()
    }
}
impl char_traits_723 {
    pub fn find(mut __s: Ptr<i32>, mut __n: usize, __a: Ptr<i32>) -> Ptr<i32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(96)]
pub struct lconv {
    #[offset(0)]
    #[byte_size(8)]
    pub decimal_point: Ptr<i8>,
    #[offset(8)]
    #[byte_size(8)]
    pub thousands_sep: Ptr<i8>,
    #[offset(16)]
    #[byte_size(8)]
    pub grouping: Ptr<i8>,
    #[offset(24)]
    #[byte_size(8)]
    pub int_curr_symbol: Ptr<i8>,
    #[offset(32)]
    #[byte_size(8)]
    pub currency_symbol: Ptr<i8>,
    #[offset(40)]
    #[byte_size(8)]
    pub mon_decimal_point: Ptr<i8>,
    #[offset(48)]
    #[byte_size(8)]
    pub mon_thousands_sep: Ptr<i8>,
    #[offset(56)]
    #[byte_size(8)]
    pub mon_grouping: Ptr<i8>,
    #[offset(64)]
    #[byte_size(8)]
    pub positive_sign: Ptr<i8>,
    #[offset(72)]
    #[byte_size(8)]
    pub negative_sign: Ptr<i8>,
    #[offset(80)]
    pub int_frac_digits: i8,
    #[offset(81)]
    pub frac_digits: i8,
    #[offset(82)]
    pub p_cs_precedes: i8,
    #[offset(83)]
    pub p_sep_by_space: i8,
    #[offset(84)]
    pub n_cs_precedes: i8,
    #[offset(85)]
    pub n_sep_by_space: i8,
    #[offset(86)]
    pub p_sign_posn: i8,
    #[offset(87)]
    pub n_sign_posn: i8,
    #[offset(88)]
    pub int_p_cs_precedes: i8,
    #[offset(89)]
    pub int_p_sep_by_space: i8,
    #[offset(90)]
    pub int_n_cs_precedes: i8,
    #[offset(91)]
    pub int_n_sep_by_space: i8,
    #[offset(92)]
    pub int_p_sign_posn: i8,
    #[offset(93)]
    pub int_n_sign_posn: i8,
}
impl Clone for lconv {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for lconv {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __fsid_t {
    #[offset(0)]
    #[byte_size(8)]
    pub __val: Value<Box<[i32]>>,
}
impl Clone for __fsid_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __fsid_t {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_724 = u32;
pub const anon_724__ISupper: anon_724 = 256;
pub const anon_724__ISlower: anon_724 = 512;
pub const anon_724__ISalpha: anon_724 = 1024;
pub const anon_724__ISdigit: anon_724 = 2048;
pub const anon_724__ISxdigit: anon_724 = 4096;
pub const anon_724__ISspace: anon_724 = 8192;
pub const anon_724__ISprint: anon_724 = 16384;
pub const anon_724__ISgraph: anon_724 = 32768;
pub const anon_724__ISblank: anon_724 = 1;
pub const anon_724__IScntrl: anon_724 = 2;
pub const anon_724__ISpunct: anon_724 = 4;
pub const anon_724__ISalnum: anon_724 = 8;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(576)]
pub struct ctype_725 {}
impl Clone for ctype_725 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ctype_725 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1344)]
pub struct ctype_726 {}
impl Clone for ctype_726 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ctype_726 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct num_get_727 {}
impl Clone for num_get_727 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for num_get_727 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct num_get_728 {}
impl Clone for num_get_728 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for num_get_728 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct num_put_729 {}
impl Clone for num_put_729 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for num_put_729 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct num_put_730 {}
impl Clone for num_put_730 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for num_put_730 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct numpunct_731 {}
impl Clone for numpunct_731 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numpunct_731 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct numpunct_732 {}
impl Clone for numpunct_732 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numpunct_732 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct collate_733 {}
impl Clone for collate_733 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for collate_733 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct collate_734 {}
impl Clone for collate_734 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for collate_734 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct timespec {
    #[offset(0)]
    pub tv_sec: i64,
    #[offset(8)]
    pub tv_nsec: i64,
}
impl Clone for timespec {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for timespec {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct sched_param {
    #[offset(0)]
    pub sched_priority: i32,
}
impl Clone for sched_param {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for sched_param {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(128)]
pub struct cpu_set_t {
    #[offset(0)]
    #[byte_size(128)]
    pub __bits: Value<Box<[u64]>>,
}
impl Clone for cpu_set_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for cpu_set_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct timeval {
    #[offset(0)]
    pub tv_sec: i64,
    #[offset(8)]
    pub tv_usec: i64,
}
impl Clone for timeval {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for timeval {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(208)]
pub struct timex {
    #[offset(0)]
    pub modes: u32,
    #[offset(8)]
    pub offset: i64,
    #[offset(16)]
    pub freq: i64,
    #[offset(24)]
    pub maxerror: i64,
    #[offset(32)]
    pub esterror: i64,
    #[offset(40)]
    pub status: i32,
    #[offset(48)]
    pub constant: i64,
    #[offset(56)]
    pub precision: i64,
    #[offset(64)]
    pub tolerance: i64,
    #[offset(72)]
    #[byte_size(16)]
    pub time: timeval,
    #[offset(88)]
    pub tick: i64,
    #[offset(96)]
    pub ppsfreq: i64,
    #[offset(104)]
    pub jitter: i64,
    #[offset(112)]
    pub shift: i32,
    #[offset(120)]
    pub stabil: i64,
    #[offset(128)]
    pub jitcnt: i64,
    #[offset(136)]
    pub calcnt: i64,
    #[offset(144)]
    pub errcnt: i64,
    #[offset(152)]
    pub stbcnt: i64,
    #[offset(160)]
    pub tai: i32,
    #[offset(164)]
    pub anon_735: i32,
    #[offset(168)]
    pub anon_736: i32,
    #[offset(172)]
    pub anon_737: i32,
    #[offset(176)]
    pub anon_738: i32,
    #[offset(180)]
    pub anon_739: i32,
    #[offset(184)]
    pub anon_740: i32,
    #[offset(188)]
    pub anon_741: i32,
    #[offset(192)]
    pub anon_742: i32,
    #[offset(196)]
    pub anon_743: i32,
    #[offset(200)]
    pub anon_744: i32,
    #[offset(204)]
    pub anon_745: i32,
}
impl Clone for timex {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for timex {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(56)]
pub struct tm {
    #[offset(0)]
    pub tm_sec: i32,
    #[offset(4)]
    pub tm_min: i32,
    #[offset(8)]
    pub tm_hour: i32,
    #[offset(12)]
    pub tm_mday: i32,
    #[offset(16)]
    pub tm_mon: i32,
    #[offset(20)]
    pub tm_year: i32,
    #[offset(24)]
    pub tm_wday: i32,
    #[offset(28)]
    pub tm_yday: i32,
    #[offset(32)]
    pub tm_isdst: i32,
    #[offset(40)]
    pub tm_gmtoff: i64,
    #[offset(48)]
    #[byte_size(8)]
    pub tm_zone: Ptr<i8>,
}
impl Clone for tm {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tm {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct itimerspec {
    #[offset(0)]
    #[byte_size(16)]
    pub it_interval: timespec,
    #[offset(16)]
    #[byte_size(16)]
    pub it_value: timespec,
}
impl Clone for itimerspec {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for itimerspec {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __atomic_wide_counter {
    #[offset(0)]
    pub __value64: u64,
    #[offset(0)]
    #[byte_size(8)]
    pub __value32: anon_746,
}
impl Clone for __atomic_wide_counter {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __atomic_wide_counter {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct anon_746 {
    #[offset(0)]
    pub __low: u32,
    #[offset(4)]
    pub __high: u32,
}
impl Clone for anon_746 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_746 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct __pthread_internal_list {
    #[offset(0)]
    #[byte_size(8)]
    pub __prev: Ptr<__pthread_internal_list>,
    #[offset(8)]
    #[byte_size(8)]
    pub __next: Ptr<__pthread_internal_list>,
}
impl Clone for __pthread_internal_list {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_internal_list {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __pthread_internal_slist {
    #[offset(0)]
    #[byte_size(8)]
    pub __next: Ptr<__pthread_internal_slist>,
}
impl Clone for __pthread_internal_slist {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_internal_slist {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(40)]
pub struct __pthread_mutex_s {
    #[offset(0)]
    pub __lock: i32,
    #[offset(4)]
    pub __count: u32,
    #[offset(8)]
    pub __owner: i32,
    #[offset(12)]
    pub __nusers: u32,
    #[offset(16)]
    pub __kind: i32,
    #[offset(20)]
    pub __spins: i16,
    #[offset(22)]
    pub __elision: i16,
    #[offset(24)]
    #[byte_size(16)]
    pub __list: __pthread_internal_list,
}
impl Clone for __pthread_mutex_s {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_mutex_s {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(56)]
pub struct __pthread_rwlock_arch_t {
    #[offset(0)]
    pub __readers: u32,
    #[offset(4)]
    pub __writers: u32,
    #[offset(8)]
    pub __wrphase_futex: u32,
    #[offset(12)]
    pub __writers_futex: u32,
    #[offset(16)]
    pub __pad3: u32,
    #[offset(20)]
    pub __pad4: u32,
    #[offset(24)]
    pub __cur_writer: i32,
    #[offset(28)]
    pub __shared: i32,
    #[offset(32)]
    pub __rwelision: i8,
    #[offset(33)]
    #[byte_size(7)]
    pub __pad1: Value<Box<[u8]>>,
    #[offset(40)]
    pub __pad2: u64,
    #[offset(48)]
    pub __flags: u32,
}
impl Clone for __pthread_rwlock_arch_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_rwlock_arch_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(48)]
pub struct __pthread_cond_s {
    #[offset(0)]
    #[byte_size(8)]
    pub __wseq: __atomic_wide_counter,
    #[offset(8)]
    #[byte_size(8)]
    pub __g1_start: __atomic_wide_counter,
    #[offset(16)]
    #[byte_size(8)]
    pub __g_refs: Value<Box<[u32]>>,
    #[offset(24)]
    #[byte_size(8)]
    pub __g_size: Value<Box<[u32]>>,
    #[offset(32)]
    pub __g1_orig_size: u32,
    #[offset(36)]
    pub __wrefs: u32,
    #[offset(40)]
    #[byte_size(8)]
    pub __g_signals: Value<Box<[u32]>>,
}
impl Clone for __pthread_cond_s {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_cond_s {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct __once_flag {
    #[offset(0)]
    pub __data: i32,
}
impl Clone for __once_flag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __once_flag {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct pthread_mutexattr_t {
    #[offset(0)]
    #[byte_size(4)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i32,
}
impl Clone for pthread_mutexattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_mutexattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct pthread_condattr_t {
    #[offset(0)]
    #[byte_size(4)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i32,
}
impl Clone for pthread_condattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_condattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(56)]
pub struct pthread_attr_t {
    #[offset(0)]
    #[byte_size(56)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_attr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_attr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(40)]
pub struct pthread_mutex_t {
    #[offset(0)]
    #[byte_size(40)]
    pub __data: __pthread_mutex_s,
    #[offset(0)]
    #[byte_size(40)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_mutex_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_mutex_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(48)]
pub struct pthread_cond_t {
    #[offset(0)]
    #[byte_size(48)]
    pub __data: __pthread_cond_s,
    #[offset(0)]
    #[byte_size(48)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_cond_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_cond_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(56)]
pub struct pthread_rwlock_t {
    #[offset(0)]
    #[byte_size(56)]
    pub __data: __pthread_rwlock_arch_t,
    #[offset(0)]
    #[byte_size(56)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_rwlock_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_rwlock_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct pthread_rwlockattr_t {
    #[offset(0)]
    #[byte_size(8)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_rwlockattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_rwlockattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct pthread_barrier_t {
    #[offset(0)]
    #[byte_size(32)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_barrier_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_barrier_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct pthread_barrierattr_t {
    #[offset(0)]
    #[byte_size(4)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i32,
}
impl Clone for pthread_barrierattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_barrierattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(128)]
pub struct __sigset_t {
    #[offset(0)]
    #[byte_size(128)]
    pub __val: Value<Box<[u64]>>,
}
impl Clone for __sigset_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __sigset_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(200)]
pub struct __jmp_buf_tag {
    #[offset(0)]
    #[byte_size(64)]
    pub __jmpbuf: Value<Box<[i64]>>,
    #[offset(64)]
    pub __mask_was_saved: i32,
    #[offset(72)]
    #[byte_size(128)]
    pub __saved_mask: __sigset_t,
}
impl Clone for __jmp_buf_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __jmp_buf_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_747 = u32;
pub const anon_747_PTHREAD_MUTEX_TIMED_NP: anon_747 = 0;
pub const anon_747_PTHREAD_MUTEX_RECURSIVE_NP: anon_747 = 1;
pub const anon_747_PTHREAD_MUTEX_ERRORCHECK_NP: anon_747 = 2;
pub const anon_747_PTHREAD_MUTEX_ADAPTIVE_NP: anon_747 = 3;
pub const anon_747_PTHREAD_MUTEX_NORMAL: anon_747 = 0;
pub const anon_747_PTHREAD_MUTEX_RECURSIVE: anon_747 = 1;
pub const anon_747_PTHREAD_MUTEX_ERRORCHECK: anon_747 = 2;
pub const anon_747_PTHREAD_MUTEX_DEFAULT: anon_747 = 0;
pub const anon_747_PTHREAD_MUTEX_FAST_NP: anon_747 = 0;
pub type anon_748 = u32;
pub const anon_748_PTHREAD_MUTEX_STALLED: anon_748 = 0;
pub const anon_748_PTHREAD_MUTEX_STALLED_NP: anon_748 = 0;
pub const anon_748_PTHREAD_MUTEX_ROBUST: anon_748 = 1;
pub const anon_748_PTHREAD_MUTEX_ROBUST_NP: anon_748 = 1;
pub type anon_749 = u32;
pub const anon_749_PTHREAD_RWLOCK_PREFER_READER_NP: anon_749 = 0;
pub const anon_749_PTHREAD_RWLOCK_PREFER_WRITER_NP: anon_749 = 1;
pub const anon_749_PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: anon_749 = 2;
pub const anon_749_PTHREAD_RWLOCK_DEFAULT_NP: anon_749 = 0;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct _pthread_cleanup_buffer {
    #[offset(0)]
    #[byte_size(8)]
    pub __routine: FnPtr<fn(AnyPtr)>,
    #[offset(8)]
    #[byte_size(8)]
    pub __arg: AnyPtr,
    #[offset(16)]
    pub __canceltype: i32,
    #[offset(24)]
    #[byte_size(8)]
    pub __prev: Ptr<_pthread_cleanup_buffer>,
}
impl Clone for _pthread_cleanup_buffer {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _pthread_cleanup_buffer {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_750 = u32;
pub const anon_750_PTHREAD_CANCEL_DEFERRED: anon_750 = 0;
pub const anon_750_PTHREAD_CANCEL_ASYNCHRONOUS: anon_750 = 1;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(72)]
pub struct __cancel_jmp_buf_tag {
    #[offset(0)]
    #[byte_size(64)]
    pub __cancel_jmp_buf: Value<Box<[i64]>>,
    #[offset(64)]
    pub __mask_was_saved: i32,
}
impl Clone for __cancel_jmp_buf_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cancel_jmp_buf_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(104)]
pub struct __pthread_unwind_buf_t {
    #[offset(0)]
    #[byte_size(72)]
    pub __cancel_jmp_buf: Value<Box<[__cancel_jmp_buf_tag]>>,
    #[offset(72)]
    #[byte_size(32)]
    pub __pad: Value<Box<[AnyPtr]>>,
}
impl Clone for __pthread_unwind_buf_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_unwind_buf_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct __pthread_cleanup_frame {
    #[offset(0)]
    #[byte_size(8)]
    pub __cancel_routine: FnPtr<fn(AnyPtr)>,
    #[offset(8)]
    #[byte_size(8)]
    pub __cancel_arg: AnyPtr,
    #[offset(16)]
    pub __do_it: i32,
    #[offset(20)]
    pub __cancel_type: i32,
}
impl Clone for __pthread_cleanup_frame {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_cleanup_frame {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct __pthread_cleanup_class {}
impl Clone for __pthread_cleanup_class {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_cleanup_class {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __libc_single_threaded_751: Value<i8> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct basic_string_view_693 {}
impl Clone for basic_string_view_693 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_693 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_693 {
    pub fn copy_from(_a0: Ptr<basic_string_view_693>) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_693 {
    pub fn new_1(mut __str: Ptr<i8>, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_693 {
    pub fn copy_assign(&self, _a0: Ptr<basic_string_view_693>) -> Ptr<basic_string_view_693> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct basic_string_view_752 {}
impl Clone for basic_string_view_752 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_752 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_752 {
    pub fn copy_from(_a0: Ptr<basic_string_view_752>) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_752 {
    pub fn new_1(mut __str: Ptr<i32>, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_752 {
    pub fn copy_assign(&self, _a0: Ptr<basic_string_view_752>) -> Ptr<basic_string_view_752> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct basic_string_view_753 {}
impl Clone for basic_string_view_753 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_753 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_753 {
    pub fn copy_from(_a0: Ptr<basic_string_view_753>) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_753 {
    pub fn new_1(mut __str: Ptr<u16>, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_753 {
    pub fn copy_assign(&self, _a0: Ptr<basic_string_view_753>) -> Ptr<basic_string_view_753> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct basic_string_view_754 {}
impl Clone for basic_string_view_754 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_754 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_754 {
    pub fn copy_from(_a0: Ptr<basic_string_view_754>) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_754 {
    pub fn new_1(mut __str: Ptr<u32>, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_754 {
    pub fn copy_assign(&self, _a0: Ptr<basic_string_view_754>) -> Ptr<basic_string_view_754> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct div_t {
    #[offset(0)]
    pub quot: i32,
    #[offset(4)]
    pub rem: i32,
}
impl Clone for div_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for div_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct ldiv_t {
    #[offset(0)]
    pub quot: i64,
    #[offset(8)]
    pub rem: i64,
}
impl Clone for ldiv_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ldiv_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct lldiv_t {
    #[offset(0)]
    pub quot: i64,
    #[offset(8)]
    pub rem: i64,
}
impl Clone for lldiv_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for lldiv_t {
    fn default() -> Self {
        unimplemented!()
    }
}
impl lldiv_t {}
impl lldiv_t {}
impl lldiv_t {}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(128)]
pub struct fd_set {
    #[offset(0)]
    #[byte_size(128)]
    pub fds_bits: Value<Box<[i64]>>,
}
impl Clone for fd_set {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for fd_set {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(48)]
pub struct random_data {
    #[offset(0)]
    #[byte_size(8)]
    pub fptr: Ptr<i32>,
    #[offset(8)]
    #[byte_size(8)]
    pub rptr: Ptr<i32>,
    #[offset(16)]
    #[byte_size(8)]
    pub state: Ptr<i32>,
    #[offset(24)]
    pub rand_type: i32,
    #[offset(28)]
    pub rand_deg: i32,
    #[offset(32)]
    pub rand_sep: i32,
    #[offset(40)]
    #[byte_size(8)]
    pub end_ptr: Ptr<i32>,
}
impl Clone for random_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for random_data {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct drand48_data {
    #[offset(0)]
    #[byte_size(6)]
    pub __x: Value<Box<[u16]>>,
    #[offset(6)]
    #[byte_size(6)]
    pub __old_x: Value<Box<[u16]>>,
    #[offset(12)]
    pub __c: u16,
    #[offset(14)]
    pub __init: u16,
    #[offset(16)]
    pub __a: u64,
}
impl Clone for drand48_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for drand48_data {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct _G_fpos_t {
    #[offset(0)]
    pub __pos: i64,
    #[offset(8)]
    #[byte_size(8)]
    pub __state: __mbstate_t,
}
impl Clone for _G_fpos_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _G_fpos_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct _G_fpos64_t {
    #[offset(0)]
    pub __pos: i64,
    #[offset(8)]
    #[byte_size(8)]
    pub __state: __mbstate_t,
}
impl Clone for _G_fpos64_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _G_fpos64_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(216)]
pub struct _IO_FILE {
    #[offset(0)]
    pub _flags: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub _IO_read_ptr: Ptr<i8>,
    #[offset(16)]
    #[byte_size(8)]
    pub _IO_read_end: Ptr<i8>,
    #[offset(24)]
    #[byte_size(8)]
    pub _IO_read_base: Ptr<i8>,
    #[offset(32)]
    #[byte_size(8)]
    pub _IO_write_base: Ptr<i8>,
    #[offset(40)]
    #[byte_size(8)]
    pub _IO_write_ptr: Ptr<i8>,
    #[offset(48)]
    #[byte_size(8)]
    pub _IO_write_end: Ptr<i8>,
    #[offset(56)]
    #[byte_size(8)]
    pub _IO_buf_base: Ptr<i8>,
    #[offset(64)]
    #[byte_size(8)]
    pub _IO_buf_end: Ptr<i8>,
    #[offset(72)]
    #[byte_size(8)]
    pub _IO_save_base: Ptr<i8>,
    #[offset(80)]
    #[byte_size(8)]
    pub _IO_backup_base: Ptr<i8>,
    #[offset(88)]
    #[byte_size(8)]
    pub _IO_save_end: Ptr<i8>,
    #[offset(96)]
    #[byte_size(8)]
    pub _markers: Ptr<_IO_marker>,
    #[offset(104)]
    #[byte_size(8)]
    pub _chain: Ptr<_IO_FILE>,
    #[offset(112)]
    pub _fileno: i32,
    #[offset(116)]
    pub _flags2: i32,
    #[offset(120)]
    pub _old_offset: i64,
    #[offset(128)]
    pub _cur_column: u16,
    #[offset(130)]
    pub _vtable_offset: i8,
    #[offset(131)]
    #[byte_size(1)]
    pub _shortbuf: Value<Box<[i8]>>,
    #[offset(136)]
    #[byte_size(8)]
    pub _lock: AnyPtr,
    #[offset(144)]
    pub _offset: i64,
    #[offset(152)]
    #[byte_size(8)]
    pub _codecvt: Ptr<_IO_codecvt>,
    #[offset(160)]
    #[byte_size(8)]
    pub _wide_data: Ptr<_IO_wide_data>,
    #[offset(168)]
    #[byte_size(8)]
    pub _freeres_list: Ptr<_IO_FILE>,
    #[offset(176)]
    #[byte_size(8)]
    pub _freeres_buf: AnyPtr,
    #[offset(184)]
    pub __pad5: usize,
    #[offset(192)]
    pub _mode: i32,
    #[offset(196)]
    #[byte_size(20)]
    pub _unused2: Value<Box<[i8]>>,
}
impl Clone for _IO_FILE {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _IO_FILE {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct _IO_cookie_io_functions_t {
    #[offset(0)]
    #[byte_size(8)]
    pub read: FnPtr<fn(AnyPtr, Ptr<i8>, usize) -> i64>,
    #[offset(8)]
    #[byte_size(8)]
    pub write: FnPtr<fn(AnyPtr, Ptr<i8>, usize) -> i64>,
    #[offset(16)]
    #[byte_size(8)]
    pub seek: FnPtr<fn(AnyPtr, Ptr<i64>, i32) -> i32>,
    #[offset(24)]
    #[byte_size(8)]
    pub close: FnPtr<fn(AnyPtr) -> i32>,
}
impl Clone for _IO_cookie_io_functions_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _IO_cookie_io_functions_t {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __integer_to_chars_is_unsigned_755: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __integer_to_chars_is_unsigned_756: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __integer_to_chars_is_unsigned_757: Value<bool> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct std_locale {}
impl Clone for std_locale {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_locale {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_locale_ctype_758: Value<i32> = unimplemented!();
);
thread_local!(
    pub static std_locale_numeric_759: Value<i32> = unimplemented!();
);
thread_local!(
    pub static std_locale_collate_760: Value<i32> = unimplemented!();
);
thread_local!(
    pub static std_locale_time_761: Value<i32> = unimplemented!();
);
thread_local!(
    pub static std_locale_monetary_762: Value<i32> = unimplemented!();
);
thread_local!(
    pub static std_locale_messages_763: Value<i32> = unimplemented!();
);
pub type anon_764 = u32;
pub const anon_764__S_categories_size: anon_764 = 12;
impl std_locale {
    pub fn copy_from(__other: Ptr<std_locale>) -> Self {
        unimplemented!()
    }
}
impl std_locale {
    pub fn new_1(mut __s: Ptr<i8>) -> Self {
        unimplemented!()
    }
}
impl std_locale {
    pub fn new_2(__base: Ptr<std_locale>, mut __s: Ptr<i8>, mut __cat: i32) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct std_locale_facet {}
impl Clone for std_locale_facet {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_locale_facet {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct std_locale_id {}
impl Clone for std_locale_id {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_locale_id {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(40)]
pub struct std_locale__Impl {}
impl Clone for std_locale__Impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_locale__Impl {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type std_errc = i32;
pub const std_errc_address_family_not_supported: std_errc = 97;
pub const std_errc_address_in_use: std_errc = 98;
pub const std_errc_address_not_available: std_errc = 99;
pub const std_errc_already_connected: std_errc = 106;
pub const std_errc_argument_list_too_long: std_errc = 7;
pub const std_errc_argument_out_of_domain: std_errc = 33;
pub const std_errc_bad_address: std_errc = 14;
pub const std_errc_bad_file_descriptor: std_errc = 9;
pub const std_errc_bad_message: std_errc = 74;
pub const std_errc_broken_pipe: std_errc = 32;
pub const std_errc_connection_aborted: std_errc = 103;
pub const std_errc_connection_already_in_progress: std_errc = 114;
pub const std_errc_connection_refused: std_errc = 111;
pub const std_errc_connection_reset: std_errc = 104;
pub const std_errc_cross_device_link: std_errc = 18;
pub const std_errc_destination_address_required: std_errc = 89;
pub const std_errc_device_or_resource_busy: std_errc = 16;
pub const std_errc_directory_not_empty: std_errc = 39;
pub const std_errc_executable_format_error: std_errc = 8;
pub const std_errc_file_exists: std_errc = 17;
pub const std_errc_file_too_large: std_errc = 27;
pub const std_errc_filename_too_long: std_errc = 36;
pub const std_errc_function_not_supported: std_errc = 38;
pub const std_errc_host_unreachable: std_errc = 113;
pub const std_errc_identifier_removed: std_errc = 43;
pub const std_errc_illegal_byte_sequence: std_errc = 84;
pub const std_errc_inappropriate_io_control_operation: std_errc = 25;
pub const std_errc_interrupted: std_errc = 4;
pub const std_errc_invalid_argument: std_errc = 22;
pub const std_errc_invalid_seek: std_errc = 29;
pub const std_errc_io_error: std_errc = 5;
pub const std_errc_is_a_directory: std_errc = 21;
pub const std_errc_message_size: std_errc = 90;
pub const std_errc_network_down: std_errc = 100;
pub const std_errc_network_reset: std_errc = 102;
pub const std_errc_network_unreachable: std_errc = 101;
pub const std_errc_no_buffer_space: std_errc = 105;
pub const std_errc_no_child_process: std_errc = 10;
pub const std_errc_no_link: std_errc = 67;
pub const std_errc_no_lock_available: std_errc = 37;
pub const std_errc_no_message_available: std_errc = 61;
pub const std_errc_no_message: std_errc = 42;
pub const std_errc_no_protocol_option: std_errc = 92;
pub const std_errc_no_space_on_device: std_errc = 28;
pub const std_errc_no_stream_resources: std_errc = 63;
pub const std_errc_no_such_device_or_address: std_errc = 6;
pub const std_errc_no_such_device: std_errc = 19;
pub const std_errc_no_such_file_or_directory: std_errc = 2;
pub const std_errc_no_such_process: std_errc = 3;
pub const std_errc_not_a_directory: std_errc = 20;
pub const std_errc_not_a_socket: std_errc = 88;
pub const std_errc_not_a_stream: std_errc = 60;
pub const std_errc_not_connected: std_errc = 107;
pub const std_errc_not_enough_memory: std_errc = 12;
pub const std_errc_not_supported: std_errc = 95;
pub const std_errc_operation_canceled: std_errc = 125;
pub const std_errc_operation_in_progress: std_errc = 115;
pub const std_errc_operation_not_permitted: std_errc = 1;
pub const std_errc_operation_not_supported: std_errc = 95;
pub const std_errc_operation_would_block: std_errc = 11;
pub const std_errc_owner_dead: std_errc = 130;
pub const std_errc_permission_denied: std_errc = 13;
pub const std_errc_protocol_error: std_errc = 71;
pub const std_errc_protocol_not_supported: std_errc = 93;
pub const std_errc_read_only_file_system: std_errc = 30;
pub const std_errc_resource_deadlock_would_occur: std_errc = 35;
pub const std_errc_resource_unavailable_try_again: std_errc = 11;
pub const std_errc_result_out_of_range: std_errc = 34;
pub const std_errc_state_not_recoverable: std_errc = 131;
pub const std_errc_stream_timeout: std_errc = 62;
pub const std_errc_text_file_busy: std_errc = 26;
pub const std_errc_timed_out: std_errc = 110;
pub const std_errc_too_many_files_open_in_system: std_errc = 23;
pub const std_errc_too_many_files_open: std_errc = 24;
pub const std_errc_too_many_links: std_errc = 31;
pub const std_errc_too_many_symbolic_link_levels: std_errc = 40;
pub const std_errc_value_too_large: std_errc = 75;
pub const std_errc_wrong_protocol_type: std_errc = 91;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct std___cow_string {
    #[offset(0)]
    #[byte_size(8)]
    pub anon_765: anon_765,
}
impl Clone for std___cow_string {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___cow_string {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct anon_765 {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_p: Ptr<i8>,
    #[offset(0)]
    #[byte_size(8)]
    pub _M_bytes: Value<Box<[i8]>>,
}
impl Clone for anon_765 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_765 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___cow_string {
    pub fn copy_from(_a0: Ptr<std___cow_string>) -> Self {
        unimplemented!()
    }
}
impl std___cow_string {
    pub fn move_from(_a0: Ptr<std___cow_string>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct std_logic_error {}
impl Clone for std_logic_error {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_logic_error {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_logic_error {
    pub fn move_from(_a0: Ptr<std_logic_error>) -> Self {
        unimplemented!()
    }
}
impl std_logic_error {
    pub fn copy_from(_a0: Ptr<std_logic_error>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct std_runtime_error {}
impl Clone for std_runtime_error {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_runtime_error {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_runtime_error {
    pub fn new_1(__arg: Ptr<basic_string_690>) -> Self {
        unimplemented!()
    }
}
impl std_runtime_error {
    pub fn move_from(_a0: Ptr<std_runtime_error>) -> Self {
        unimplemented!()
    }
}
impl std_runtime_error {
    pub fn copy_from(_a0: Ptr<std_runtime_error>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_code_enum_766 {}
impl Clone for is_error_code_enum_766 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_766 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_code_enum_767 {}
impl Clone for is_error_code_enum_767 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_767 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_code_enum_768 {}
impl Clone for is_error_code_enum_768 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_768 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_code_enum_769 {}
impl Clone for is_error_code_enum_769 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_769 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_code_enum_770 {}
impl Clone for is_error_code_enum_770 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_770 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_code_enum_771 {}
impl Clone for is_error_code_enum_771 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_771 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_condition_enum_772 {}
impl Clone for is_error_condition_enum_772 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_condition_enum_772 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_condition_enum_773 {}
impl Clone for is_error_condition_enum_773 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_condition_enum_773 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct std_error_code {}
impl Clone for std_error_code {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_error_code {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_error_code {
    pub fn new_1(mut __v: i32, __cat: Ptr<std_error_category>) -> Self {
        unimplemented!()
    }
}
impl std_error_code {
    pub fn copy_from(_a0: Ptr<std_error_code>) -> Self {
        unimplemented!()
    }
}
impl std_error_code {
    pub fn copy_assign(&self, _a0: Ptr<std_error_code>) -> Ptr<std_error_code> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct std_error_condition {}
impl Clone for std_error_condition {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_error_condition {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_error_condition {
    pub fn new_1(mut __v: i32, __cat: Ptr<std_error_category>) -> Self {
        unimplemented!()
    }
}
impl std_error_condition {
    pub fn copy_from(_a0: Ptr<std_error_condition>) -> Self {
        unimplemented!()
    }
}
impl std_error_condition {
    pub fn copy_assign(&self, _a0: Ptr<std_error_condition>) -> Ptr<std_error_condition> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct std_system_error {}
impl Clone for std_system_error {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_system_error {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_system_error {
    pub fn new_1(__ec: std_error_code, mut __what: Ptr<i8>) -> Self {
        unimplemented!()
    }
}
impl std_system_error {
    pub fn copy_from(_a0: Ptr<std_system_error>) -> Self {
        unimplemented!()
    }
}
impl std_system_error {
    pub fn copy_assign(&self, _a0: Ptr<std_system_error>) -> Ptr<std_system_error> {
        unimplemented!()
    }
}
pub type std__Ios_Fmtflags = i32;
pub const std__Ios_Fmtflags__S_boolalpha: std__Ios_Fmtflags = 1;
pub const std__Ios_Fmtflags__S_dec: std__Ios_Fmtflags = 2;
pub const std__Ios_Fmtflags__S_fixed: std__Ios_Fmtflags = 4;
pub const std__Ios_Fmtflags__S_hex: std__Ios_Fmtflags = 8;
pub const std__Ios_Fmtflags__S_internal: std__Ios_Fmtflags = 16;
pub const std__Ios_Fmtflags__S_left: std__Ios_Fmtflags = 32;
pub const std__Ios_Fmtflags__S_oct: std__Ios_Fmtflags = 64;
pub const std__Ios_Fmtflags__S_right: std__Ios_Fmtflags = 128;
pub const std__Ios_Fmtflags__S_scientific: std__Ios_Fmtflags = 256;
pub const std__Ios_Fmtflags__S_showbase: std__Ios_Fmtflags = 512;
pub const std__Ios_Fmtflags__S_showpoint: std__Ios_Fmtflags = 1024;
pub const std__Ios_Fmtflags__S_showpos: std__Ios_Fmtflags = 2048;
pub const std__Ios_Fmtflags__S_skipws: std__Ios_Fmtflags = 4096;
pub const std__Ios_Fmtflags__S_unitbuf: std__Ios_Fmtflags = 8192;
pub const std__Ios_Fmtflags__S_uppercase: std__Ios_Fmtflags = 16384;
pub const std__Ios_Fmtflags__S_adjustfield: std__Ios_Fmtflags = 176;
pub const std__Ios_Fmtflags__S_basefield: std__Ios_Fmtflags = 74;
pub const std__Ios_Fmtflags__S_floatfield: std__Ios_Fmtflags = 260;
pub const std__Ios_Fmtflags__S_ios_fmtflags_end: std__Ios_Fmtflags = 65536;
pub const std__Ios_Fmtflags__S_ios_fmtflags_max: std__Ios_Fmtflags = 2147483647;
pub const std__Ios_Fmtflags__S_ios_fmtflags_min: std__Ios_Fmtflags = -2147483648;
pub type std__Ios_Openmode = i32;
pub const std__Ios_Openmode__S_app: std__Ios_Openmode = 1;
pub const std__Ios_Openmode__S_ate: std__Ios_Openmode = 2;
pub const std__Ios_Openmode__S_bin: std__Ios_Openmode = 4;
pub const std__Ios_Openmode__S_in: std__Ios_Openmode = 8;
pub const std__Ios_Openmode__S_out: std__Ios_Openmode = 16;
pub const std__Ios_Openmode__S_trunc: std__Ios_Openmode = 32;
pub const std__Ios_Openmode__S_noreplace: std__Ios_Openmode = 64;
pub const std__Ios_Openmode__S_ios_openmode_end: std__Ios_Openmode = 65536;
pub const std__Ios_Openmode__S_ios_openmode_max: std__Ios_Openmode = 2147483647;
pub const std__Ios_Openmode__S_ios_openmode_min: std__Ios_Openmode = -2147483648;
pub type std__Ios_Iostate = i32;
pub const std__Ios_Iostate__S_goodbit: std__Ios_Iostate = 0;
pub const std__Ios_Iostate__S_badbit: std__Ios_Iostate = 1;
pub const std__Ios_Iostate__S_eofbit: std__Ios_Iostate = 2;
pub const std__Ios_Iostate__S_failbit: std__Ios_Iostate = 4;
pub const std__Ios_Iostate__S_ios_iostate_end: std__Ios_Iostate = 65536;
pub const std__Ios_Iostate__S_ios_iostate_max: std__Ios_Iostate = 2147483647;
pub const std__Ios_Iostate__S_ios_iostate_min: std__Ios_Iostate = -2147483648;
pub type std__Ios_Seekdir = u32;
pub const std__Ios_Seekdir__S_beg: std__Ios_Seekdir = 0;
pub const std__Ios_Seekdir__S_cur: std__Ios_Seekdir = 1;
pub const std__Ios_Seekdir__S_end: std__Ios_Seekdir = 2;
pub const std__Ios_Seekdir__S_ios_seekdir_end: std__Ios_Seekdir = 65536;
pub type std_io_errc = i32;
pub const std_io_errc_stream: std_io_errc = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(216)]
pub struct std_ios_base {}
impl Clone for std_ios_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ios_base {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_ios_base_boolalpha_774: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_dec_775: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_fixed_776: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_hex_777: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_internal_778: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_left_779: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_oct_780: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_right_781: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_scientific_782: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_showbase_783: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_showpoint_784: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_showpos_785: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_skipws_786: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_unitbuf_787: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_uppercase_788: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_adjustfield_789: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_basefield_790: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_floatfield_791: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_badbit_792: Value<std__Ios_Iostate> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_eofbit_793: Value<std__Ios_Iostate> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_failbit_794: Value<std__Ios_Iostate> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_goodbit_795: Value<std__Ios_Iostate> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_in_796: Value<std__Ios_Openmode> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_out_797: Value<std__Ios_Openmode> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_cur_798: Value<std__Ios_Seekdir> = unimplemented!();
);
pub type std_ios_base_event = u32;
pub const std_ios_base_event_erase_event: std_ios_base_event = 0;
pub const std_ios_base_event_imbue_event: std_ios_base_event = 1;
pub const std_ios_base_event_copyfmt_event: std_ios_base_event = 2;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std_ios_base__Callback_list {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_next: Ptr<std_ios_base__Callback_list>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_fn: FnPtr<fn(std_ios_base_event, Ptr<std_ios_base>, i32)>,
    #[offset(16)]
    pub _M_index: i32,
    #[offset(20)]
    pub _M_refcount: i32,
}
impl Clone for std_ios_base__Callback_list {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ios_base__Callback_list {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct std_ios_base__Words {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_pword: AnyPtr,
    #[offset(8)]
    pub _M_iword: i64,
}
impl Clone for std_ios_base__Words {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ios_base__Words {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_ios_base__Words {}
impl std_ios_base__Words {}
impl std_ios_base__Words {
    pub fn copy_assign(&self, _a0: Ptr<std_ios_base__Words>) -> Ptr<std_ios_base__Words> {
        unimplemented!()
    }
}
impl std_ios_base__Words {
    pub fn move_assign(&self, _a0: Ptr<std_ios_base__Words>) -> Ptr<std_ios_base__Words> {
        unimplemented!()
    }
}
pub type anon_799 = u32;
pub const anon_799__S_local_word_size: anon_799 = 8;
pub type anon_800 = i32;
pub const anon_800___ISwupper: anon_800 = 0;
pub const anon_800___ISwlower: anon_800 = 1;
pub const anon_800___ISwalpha: anon_800 = 2;
pub const anon_800___ISwdigit: anon_800 = 3;
pub const anon_800___ISwxdigit: anon_800 = 4;
pub const anon_800___ISwspace: anon_800 = 5;
pub const anon_800___ISwprint: anon_800 = 6;
pub const anon_800___ISwgraph: anon_800 = 7;
pub const anon_800___ISwblank: anon_800 = 8;
pub const anon_800___ISwcntrl: anon_800 = 9;
pub const anon_800___ISwpunct: anon_800 = 10;
pub const anon_800___ISwalnum: anon_800 = 11;
pub const anon_800__ISwupper: anon_800 = 16777216;
pub const anon_800__ISwlower: anon_800 = 33554432;
pub const anon_800__ISwalpha: anon_800 = 67108864;
pub const anon_800__ISwdigit: anon_800 = 134217728;
pub const anon_800__ISwxdigit: anon_800 = 268435456;
pub const anon_800__ISwspace: anon_800 = 536870912;
pub const anon_800__ISwprint: anon_800 = 1073741824;
pub const anon_800__ISwgraph: anon_800 = -2147483648;
pub const anon_800__ISwblank: anon_800 = 65536;
pub const anon_800__ISwcntrl: anon_800 = 131072;
pub const anon_800__ISwpunct: anon_800 = 262144;
pub const anon_800__ISwalnum: anon_800 = 524288;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ctype_base {}
impl Clone for std_ctype_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ctype_base {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_ctype_base_upper_801: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_lower_802: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_alpha_803: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_digit_804: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_xdigit_805: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_space_806: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_print_807: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_graph_808: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_cntrl_809: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_punct_810: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_alnum_811: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_blank_812: Value<u16> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___num_base {}
impl Clone for std___num_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___num_base {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_813 = u32;
pub const anon_813__S_ominus: anon_813 = 0;
pub const anon_813__S_oplus: anon_813 = 1;
pub const anon_813__S_ox: anon_813 = 2;
pub const anon_813__S_oX: anon_813 = 3;
pub const anon_813__S_odigits: anon_813 = 4;
pub const anon_813__S_odigits_end: anon_813 = 20;
pub const anon_813__S_oudigits: anon_813 = 20;
pub const anon_813__S_oudigits_end: anon_813 = 36;
pub const anon_813__S_oe: anon_813 = 18;
pub const anon_813__S_oE: anon_813 = 34;
pub const anon_813__S_oend: anon_813 = 36;
thread_local!(
    pub static std___num_base__S_atoms_out_814: Value<Ptr<i8>> = unimplemented!();
);
thread_local!(
    pub static std___num_base__S_atoms_in_815: Value<Ptr<i8>> = unimplemented!();
);
pub type anon_816 = u32;
pub const anon_816__S_iminus: anon_816 = 0;
pub const anon_816__S_iplus: anon_816 = 1;
pub const anon_816__S_ix: anon_816 = 2;
pub const anon_816__S_iX: anon_816 = 3;
pub const anon_816__S_izero: anon_816 = 4;
pub const anon_816__S_ie: anon_816 = 18;
pub const anon_816__S_iE: anon_816 = 24;
pub const anon_816__S_iend: anon_816 = 26;
thread_local!(
    pub static cout_817: Value<basic_ostream_716> = unimplemented!();
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
pub fn set_818(ref_: reference_wrapper_7, mut val: i32) {
    let ref_: Value<reference_wrapper_7> = Rc::new(RefCell::new(ref_));
    ({ reference_wrapper_7Impl::get(&ref_.as_pointer()) }).write(val);
}
pub fn read_819(ref_: reference_wrapper_7) -> i32 {
    let ref_: Value<reference_wrapper_7> = Rc::new(RefCell::new(ref_));
    let r: Ptr<i32> = ({ reference_wrapper_7Impl::to_pmuti32(&ref_.as_pointer()) });
    return (r.read());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let i1: Value<i32> = Rc::new(RefCell::new(10));
    let ref_1: Value<reference_wrapper_7> =
        Rc::new(RefCell::new(reference_wrapper_7::new({ i1.as_pointer() })));
    ({ reference_wrapper_7Impl::get(&ref_1.as_pointer()) }).write(20);
    let i2: Ptr<i32> = ({ reference_wrapper_7Impl::to_pmuti32(&ref_1.as_pointer()) });
    {
        i2.with_mut(|__v| *__v = *__v + 5)
    };
    ({
        let ___out: Ptr<basic_ostream_716> = ({
            basic_ostream_716Impl::operator_shl_11(
                &cout_817.with(|v| v.as_pointer()),
                (*i1.borrow()),
            )
        });
        operator_shl_820(___out, ('\n' as i8))
    });
    let i3: Value<i32> = Rc::new(RefCell::new(1));
    let i4: Value<i32> = Rc::new(RefCell::new(2));
    let ref_3: Value<reference_wrapper_7> =
        Rc::new(RefCell::new(reference_wrapper_7::new({ i3.as_pointer() })));
    let ref_4: Value<reference_wrapper_7> =
        Rc::new(RefCell::new(reference_wrapper_7::new({ i4.as_pointer() })));
    let __rhs = (({ reference_wrapper_7Impl::get(&ref_4.as_pointer()) }).read());
    ({ reference_wrapper_7Impl::get(&ref_3.as_pointer()) }).write(__rhs);
    ({
        let ___out: Ptr<basic_ostream_716> = ({
            basic_ostream_716Impl::operator_shl_11(
                &({
                    let ___out: Ptr<basic_ostream_716> = ({
                        basic_ostream_716Impl::operator_shl_11(
                            &cout_817.with(|v| v.as_pointer()),
                            (*i3.borrow()),
                        )
                    });
                    operator_shl_820(___out, (' ' as i8))
                }),
                (*i4.borrow()),
            )
        });
        operator_shl_820(___out, ('\n' as i8))
    });
    ({ set_818(reference_wrapper_7::copy_from({ ref_1.as_pointer() }), 99) });
    ({
        let ___out: Ptr<basic_ostream_716> = ({
            basic_ostream_716Impl::operator_shl_11(
                &({
                    let ___out: Ptr<basic_ostream_716> = ({
                        basic_ostream_716Impl::operator_shl_11(
                            &cout_817.with(|v| v.as_pointer()),
                            (*i1.borrow()),
                        )
                    });
                    operator_shl_820(___out, (' ' as i8))
                }),
                ({ read_819(reference_wrapper_7::copy_from({ ref_1.as_pointer() })) }),
            )
        });
        operator_shl_820(___out, ('\n' as i8))
    });
    let point: Value<Point> = Rc::new(RefCell::new(Point { x: 3, y: 4 }));
    let point_ref: Value<reference_wrapper_8> = Rc::new(RefCell::new(reference_wrapper_8::new({
        point.as_pointer()
    })));
    field!(
        ({ reference_wrapper_8Impl::get(&point_ref.as_pointer(),) }),
        x
    )
    .write(30);
    field!(
        ({ reference_wrapper_8Impl::to_pmutPoint(&point_ref.as_pointer(),) }),
        y
    )
    .write(40);
    ({
        let ___out: Ptr<basic_ostream_716> = ({
            let ___n: i32 = { (*point.borrow()).y };
            basic_ostream_716Impl::operator_shl_11(
                &({
                    let ___out: Ptr<basic_ostream_716> = ({
                        basic_ostream_716Impl::operator_shl_11(
                            &cout_817.with(|v| v.as_pointer()),
                            { (*point.borrow()).x },
                        )
                    });
                    operator_shl_820(___out, (' ' as i8))
                }),
                ___n,
            )
        });
        operator_shl_820(___out, ('\n' as i8))
    });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_wide_data;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_codecvt;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_marker;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __locale_data;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct std_error_category;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct std___uses_alloc0__Sink;
pub fn operator_shl_820(__out: Ptr<basic_ostream_716>, __c: i8) -> Ptr<basic_ostream_716> {
    unimplemented!()
}
pub trait _Hashtable_634Impl {
    fn copy_assign(&self, __ht: Ptr<_Hashtable_634>) -> Ptr<_Hashtable_634> {
        unimplemented!()
    }
    fn move_assign(&self, __ht: Ptr<_Hashtable_634>) -> Ptr<_Hashtable_634> {
        unimplemented!()
    }
}
impl _Hashtable_634Impl for Ptr<_Hashtable_634> {}
pub trait _Hashtable_635Impl {
    fn copy_assign(&self, __ht: Ptr<_Hashtable_635>) -> Ptr<_Hashtable_635> {
        unimplemented!()
    }
    fn move_assign(&self, __ht: Ptr<_Hashtable_635>) -> Ptr<_Hashtable_635> {
        unimplemented!()
    }
}
impl _Hashtable_635Impl for Ptr<_Hashtable_635> {}
pub trait __new_allocator_612Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<i8> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<i8>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl __new_allocator_612Impl for Ptr<__new_allocator_612> {}
pub trait __new_allocator_613Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<i32> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<i32>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl __new_allocator_613Impl for Ptr<__new_allocator_613> {}
pub trait __new_allocator_616Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<u16> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<u16>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl __new_allocator_616Impl for Ptr<__new_allocator_616> {}
pub trait __new_allocator_617Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<u32> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<u32>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl __new_allocator_617Impl for Ptr<__new_allocator_617> {}
pub trait basic_ostream_716Impl {
    fn operator_shl_11(&self, mut __n: i32) -> Ptr<basic_ostream_716> {
        unimplemented!()
    }
    fn put(&self, mut __c: i8) -> Ptr<basic_ostream_716> {
        unimplemented!()
    }
}
impl basic_ostream_716Impl for Ptr<basic_ostream_716> {}
pub trait basic_string_690Impl {
    fn copy_assign(&self, __str: Ptr<basic_string_690>) -> Ptr<basic_string_690> {
        unimplemented!()
    }
    fn move_assign(&self, __str: Ptr<basic_string_690>) -> Ptr<basic_string_690> {
        unimplemented!()
    }
    fn to_basic_string_view_693(&self) -> basic_string_view_693 {
        unimplemented!()
    }
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn length(&self) -> u64 {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn capacity(&self) -> u64 {
        unimplemented!()
    }
    fn reserve_79(&self, __res: u64) {
        unimplemented!()
    }
    fn insert_119(&self, mut __pos1: u64, __str: Ptr<basic_string_690>) -> Ptr<basic_string_690> {
        unimplemented!()
    }
    fn insert_122(&self, mut __pos: u64, mut __s: Ptr<i8>) -> Ptr<basic_string_690> {
        unimplemented!()
    }
    fn replace_133(
        &self,
        mut __pos: u64,
        mut __n1: u64,
        mut __s: Ptr<i8>,
        mut __n2: u64,
    ) -> Ptr<basic_string_690> {
        unimplemented!()
    }
    fn c_str(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn data_158(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn __resize_and_overwrite_694(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
    fn __resize_and_overwrite_695(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
    fn __resize_and_overwrite_696(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
    fn __resize_and_overwrite_697(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
    fn __resize_and_overwrite_698(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
    fn __resize_and_overwrite_699(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
}
impl basic_string_690Impl for Ptr<basic_string_690> {}
pub trait basic_string_700Impl {
    fn copy_assign(&self, __str: Ptr<basic_string_700>) -> Ptr<basic_string_700> {
        unimplemented!()
    }
    fn move_assign(&self, __str: Ptr<basic_string_700>) -> Ptr<basic_string_700> {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn c_str(&self) -> Ptr<i32> {
        unimplemented!()
    }
}
impl basic_string_700Impl for Ptr<basic_string_700> {}
pub trait basic_string_703Impl {
    fn copy_assign(&self, __str: Ptr<basic_string_703>) -> Ptr<basic_string_703> {
        unimplemented!()
    }
    fn move_assign(&self, __str: Ptr<basic_string_703>) -> Ptr<basic_string_703> {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_703Impl for Ptr<basic_string_703> {}
pub trait basic_string_706Impl {
    fn copy_assign(&self, __str: Ptr<basic_string_706>) -> Ptr<basic_string_706> {
        unimplemented!()
    }
    fn move_assign(&self, __str: Ptr<basic_string_706>) -> Ptr<basic_string_706> {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_706Impl for Ptr<basic_string_706> {}
pub trait basic_string_view_693Impl {
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn length(&self) -> u64 {
        unimplemented!()
    }
    fn data(&self) -> Ptr<i8> {
        unimplemented!()
    }
}
impl basic_string_view_693Impl for Ptr<basic_string_view_693> {}
pub trait basic_string_view_752Impl {
    fn length(&self) -> u64 {
        unimplemented!()
    }
    fn data(&self) -> Ptr<i32> {
        unimplemented!()
    }
}
impl basic_string_view_752Impl for Ptr<basic_string_view_752> {}
pub trait basic_string_view_753Impl {
    fn length(&self) -> u64 {
        unimplemented!()
    }
    fn data(&self) -> Ptr<u16> {
        unimplemented!()
    }
}
impl basic_string_view_753Impl for Ptr<basic_string_view_753> {}
pub trait basic_string_view_754Impl {
    fn length(&self) -> u64 {
        unimplemented!()
    }
    fn data(&self) -> Ptr<u32> {
        unimplemented!()
    }
}
impl basic_string_view_754Impl for Ptr<basic_string_view_754> {}
pub trait initializer_list_461Impl {
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn begin(&self) -> Ptr<bool> {
        unimplemented!()
    }
}
impl initializer_list_461Impl for Ptr<initializer_list_461> {}
pub trait integral_constant_9Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl integral_constant_9Impl for Ptr<integral_constant_9> {}
pub trait less_394Impl {
    fn operator_call(
        &self,
        mut __x: Ptr<std_error_category>,
        mut __y: Ptr<std_error_category>,
    ) -> bool {
        unimplemented!()
    }
}
impl less_394Impl for Ptr<less_394> {}
pub trait less_395Impl {
    fn operator_call(&self, mut __x: Ptr<i8>, mut __y: Ptr<i8>) -> bool {
        unimplemented!()
    }
}
impl less_395Impl for Ptr<less_395> {}
pub trait pair_399Impl {
    fn copy_assign(&self, __p: Ptr<pair_399>) -> Ptr<pair_399> {
        unimplemented!()
    }
    fn move_assign(&self, __p: Ptr<pair_399>) -> Ptr<pair_399> {
        unimplemented!()
    }
}
impl pair_399Impl for Ptr<pair_399> {}
pub trait reference_wrapper_7Impl {
    fn to_pmuti32(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn get(&self) -> Ptr<i32> {
        unimplemented!()
    }
}
impl reference_wrapper_7Impl for Ptr<reference_wrapper_7> {}
pub trait reference_wrapper_8Impl {
    fn to_pmutPoint(&self) -> Ptr<Point> {
        unimplemented!()
    }
    fn get(&self) -> Ptr<Point> {
        unimplemented!()
    }
}
impl reference_wrapper_8Impl for Ptr<reference_wrapper_8> {}
pub trait std__Bit_const_iteratorImpl {
    fn operator_deref(&self) -> bool {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut __i: i64) -> Ptr<std__Bit_const_iterator> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut __i: i64) -> Ptr<std__Bit_const_iterator> {
        unimplemented!()
    }
}
impl std__Bit_const_iteratorImpl for Ptr<std__Bit_const_iterator> {}
pub trait std__Bit_iteratorImpl {
    fn operator_deref(&self) -> std__Bit_reference {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut __i: i64) -> Ptr<std__Bit_iterator> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut __i: i64) -> Ptr<std__Bit_iterator> {
        unimplemented!()
    }
}
impl std__Bit_iteratorImpl for Ptr<std__Bit_iterator> {}
pub trait std__Bit_iterator_baseImpl {
    fn _M_assume_normalized(&self) {
        unimplemented!()
    }
    fn _M_bump_up(&self) {
        unimplemented!()
    }
    fn _M_bump_down(&self) {
        unimplemented!()
    }
    fn _M_incr(&self, mut __i: i64) {
        unimplemented!()
    }
}
impl std__Bit_iterator_baseImpl for Ptr<std__Bit_iterator_base> {}
pub trait std__Bit_referenceImpl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_assign_5(&self, mut __x: bool) -> Ptr<std__Bit_reference> {
        unimplemented!()
    }
    fn copy_assign(&self, __x: Ptr<std__Bit_reference>) -> Ptr<std__Bit_reference> {
        unimplemented!()
    }
}
impl std__Bit_referenceImpl for Ptr<std__Bit_reference> {}
pub trait std___cow_stringImpl {
    fn copy_assign(&self, _a0: Ptr<std___cow_string>) -> Ptr<std___cow_string> {
        unimplemented!()
    }
    fn move_assign(&self, _a0: Ptr<std___cow_string>) -> Ptr<std___cow_string> {
        unimplemented!()
    }
}
impl std___cow_stringImpl for Ptr<std___cow_string> {}
pub trait std___detail__Power2_rehash_policyImpl {
    fn _M_next_bkt(&self, mut __n: usize) -> usize {
        unimplemented!()
    }
}
impl std___detail__Power2_rehash_policyImpl for Ptr<std___detail__Power2_rehash_policy> {}
pub trait std___exception_ptr_exception_ptrImpl {
    fn copy_assign(
        &self,
        __other: Ptr<std___exception_ptr_exception_ptr>,
    ) -> Ptr<std___exception_ptr_exception_ptr> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        __o: Ptr<std___exception_ptr_exception_ptr>,
    ) -> Ptr<std___exception_ptr_exception_ptr> {
        unimplemented!()
    }
    fn swap(&self, __other: Ptr<std___exception_ptr_exception_ptr>) {
        unimplemented!()
    }
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptrImpl for Ptr<std___exception_ptr_exception_ptr> {}
pub trait std_error_codeImpl {
    fn assign(&self, mut __v: i32, __cat: Ptr<std_error_category>) {
        unimplemented!()
    }
    fn value(&self) -> i32 {
        unimplemented!()
    }
    fn category(&self) -> Ptr<std_error_category> {
        unimplemented!()
    }
    fn message(&self) -> basic_string_690 {
        unimplemented!()
    }
}
impl std_error_codeImpl for Ptr<std_error_code> {}
pub trait std_error_conditionImpl {
    fn assign(&self, mut __v: i32, __cat: Ptr<std_error_category>) {
        unimplemented!()
    }
    fn value(&self) -> i32 {
        unimplemented!()
    }
    fn category(&self) -> Ptr<std_error_category> {
        unimplemented!()
    }
}
impl std_error_conditionImpl for Ptr<std_error_condition> {}
pub trait std_ios_baseImpl {
    fn setf_8(&self, mut __fmtfl: std__Ios_Fmtflags) -> std__Ios_Fmtflags {
        unimplemented!()
    }
    fn setf_9(
        &self,
        mut __fmtfl: std__Ios_Fmtflags,
        mut __mask: std__Ios_Fmtflags,
    ) -> std__Ios_Fmtflags {
        unimplemented!()
    }
    fn unsetf(&self, mut __mask: std__Ios_Fmtflags) {
        unimplemented!()
    }
    fn width_13(&self) -> i64 {
        unimplemented!()
    }
}
impl std_ios_baseImpl for Ptr<std_ios_base> {}
pub trait std_localeImpl {
    fn copy_assign(&self, __other: Ptr<std_locale>) -> Ptr<std_locale> {
        unimplemented!()
    }
    fn operator_eq(&self, __other: Ptr<std_locale>) -> bool {
        unimplemented!()
    }
}
impl std_localeImpl for Ptr<std_locale> {}
pub trait std_logic_errorImpl {
    fn move_assign(&self, _a0: Ptr<std_logic_error>) -> Ptr<std_logic_error> {
        unimplemented!()
    }
    fn copy_assign(&self, _a0: Ptr<std_logic_error>) -> Ptr<std_logic_error> {
        unimplemented!()
    }
}
impl std_logic_errorImpl for Ptr<std_logic_error> {}
pub trait std_runtime_errorImpl {
    fn move_assign(&self, _a0: Ptr<std_runtime_error>) -> Ptr<std_runtime_error> {
        unimplemented!()
    }
    fn copy_assign(&self, _a0: Ptr<std_runtime_error>) -> Ptr<std_runtime_error> {
        unimplemented!()
    }
}
impl std_runtime_errorImpl for Ptr<std_runtime_error> {}
pub trait std_type_infoImpl {
    fn name(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_eq(&self, __arg: Ptr<std_type_info>) -> bool {
        unimplemented!()
    }
}
impl std_type_infoImpl for Ptr<std_type_info> {}
pub fn __cpp2rust_init_globals() {}
