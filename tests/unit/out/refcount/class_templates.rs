extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_0 {}
impl Clone for integral_constant_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_0 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_integral_constant_bool__true__value_1: Value<bool> = unimplemented!();
);
impl integral_constant_0 {}
impl integral_constant_0 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_2 {}
impl Clone for integral_constant_2 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_2 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_integral_constant_bool__false__value_3: Value<bool> = unimplemented!();
);
impl integral_constant_2 {}
impl integral_constant_2 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_4 {}
impl Clone for integral_constant_4 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_4 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_5 {}
impl Clone for integral_constant_5 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_5 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_6 {}
impl Clone for enable_if_6 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_6 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_7 {}
impl Clone for enable_if_7 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_7 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_8 {}
impl Clone for enable_if_8 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_8 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_9 {}
impl Clone for enable_if_9 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_9 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_10 {}
impl Clone for enable_if_10 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_10 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_11 {}
impl Clone for enable_if_11 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_11 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __conditional_12 {}
impl Clone for __conditional_12 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_12 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __conditional_13 {}
impl Clone for __conditional_13 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_13 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_14 {}
impl Clone for __type_identity_14 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_14 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_15 {}
impl Clone for __type_identity_15 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_15 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_16 {}
impl Clone for __type_identity_16 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_16 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_17 {}
impl Clone for __type_identity_17 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_17 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_18 {}
impl Clone for __type_identity_18 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_18 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_19 {}
impl Clone for __type_identity_19 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_19 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_20 {}
impl Clone for __type_identity_20 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_20 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_21 {}
impl Clone for __type_identity_21 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_21 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_22 {}
impl Clone for __type_identity_22 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_22 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_23 {}
impl Clone for __type_identity_23 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_23 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_24 {}
impl Clone for __type_identity_24 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_24 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_25 {}
impl Clone for __type_identity_25 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_25 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_26 {}
impl Clone for __type_identity_26 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_26 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__27 {}
impl Clone for __or__27 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__27 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__28 {}
impl Clone for __or__28 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__28 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__29 {}
impl Clone for __or__29 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__29 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__30 {}
impl Clone for __or__30 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__30 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__31 {}
impl Clone for __or__31 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__31 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__32 {}
impl Clone for __or__32 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__32 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__33 {}
impl Clone for __or__33 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__33 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__34 {}
impl Clone for __or__34 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__34 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__35 {}
impl Clone for __or__35 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__35 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__36 {}
impl Clone for __or__36 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__36 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__37 {}
impl Clone for __or__37 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__37 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__38 {}
impl Clone for __or__38 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__38 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__39 {}
impl Clone for __or__39 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__39 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__40 {}
impl Clone for __or__40 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__40 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__41 {}
impl Clone for __or__41 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__41 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__42 {}
impl Clone for __or__42 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__42 {
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
pub struct __and__57 {}
impl Clone for __and__57 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__57 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__58 {}
impl Clone for __and__58 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__58 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__59 {}
impl Clone for __and__59 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__59 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__60 {}
impl Clone for __and__60 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__60 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__61 {}
impl Clone for __and__61 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__61 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__62 {}
impl Clone for __and__62 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__62 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__63 {}
impl Clone for __and__63 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__63 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__64 {}
impl Clone for __and__64 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__64 {
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
pub struct __not__72 {}
impl Clone for __not__72 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__72 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__73 {}
impl Clone for __not__73 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__73 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__74 {}
impl Clone for __not__74 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__74 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__75 {}
impl Clone for __not__75 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__75 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__76 {}
impl Clone for __not__76 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__76 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_77 {}
impl Clone for is_reference_77 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_77 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_78 {}
impl Clone for is_reference_78 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_78 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_79 {}
impl Clone for is_reference_79 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_79 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_80 {}
impl Clone for is_reference_80 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_80 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_81 {}
impl Clone for is_reference_81 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_81 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_82 {}
impl Clone for is_reference_82 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_82 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_83 {}
impl Clone for is_reference_83 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_83 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_84 {}
impl Clone for is_reference_84 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_84 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_85 {}
impl Clone for is_reference_85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_85 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_86 {}
impl Clone for is_reference_86 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_86 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_87 {}
impl Clone for is_reference_87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_88 {}
impl Clone for is_reference_88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_88 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_89 {}
impl Clone for is_reference_89 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_89 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_90 {}
impl Clone for is_reference_90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_90 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_91 {}
impl Clone for is_function_91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_91 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_92 {}
impl Clone for is_function_92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_93 {}
impl Clone for is_function_93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_93 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_94 {}
impl Clone for is_function_94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_94 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_95 {}
impl Clone for is_function_95 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_95 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_96 {}
impl Clone for is_function_96 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_96 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_97 {}
impl Clone for is_function_97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_97 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_98 {}
impl Clone for is_function_98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_98 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_99 {}
impl Clone for is_function_99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_99 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_100 {}
impl Clone for is_function_100 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_100 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_101 {}
impl Clone for is_function_101 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_101 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_102 {}
impl Clone for is_function_102 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_102 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_103 {}
impl Clone for is_function_103 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_103 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_104 {}
impl Clone for is_void_104 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_104 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_105 {}
impl Clone for is_void_105 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_105 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_106 {}
impl Clone for is_void_106 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_106 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_107 {}
impl Clone for is_void_107 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_107 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_108 {}
impl Clone for is_void_108 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_108 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_109 {}
impl Clone for is_void_109 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_109 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_110 {}
impl Clone for is_void_110 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_110 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_111 {}
impl Clone for is_void_111 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_111 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_112 {}
impl Clone for is_void_112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_112 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_113 {}
impl Clone for is_void_113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_113 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_114 {}
impl Clone for is_void_114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_114 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_115 {}
impl Clone for is_void_115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_115 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_116 {}
impl Clone for is_void_116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_116 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_117 {}
impl Clone for is_void_117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_117 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_118 {}
impl Clone for remove_cv_118 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_118 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_119 {}
impl Clone for remove_cv_119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_119 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_120 {}
impl Clone for remove_cv_120 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_120 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_121 {}
impl Clone for remove_cv_121 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_121 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_122 {}
impl Clone for remove_cv_122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_122 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_123 {}
impl Clone for remove_cv_123 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_123 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_124 {}
impl Clone for remove_cv_124 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_124 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_125 {}
impl Clone for remove_cv_125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_125 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_126 {}
impl Clone for remove_cv_126 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_126 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_127 {}
impl Clone for remove_cv_127 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_127 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_128 {}
impl Clone for is_const_128 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_128 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_129 {}
impl Clone for is_const_129 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_129 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_130 {}
impl Clone for is_const_130 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_130 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_131 {}
impl Clone for is_const_131 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_131 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_132 {}
impl Clone for is_const_132 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_132 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_133 {}
impl Clone for __is_array_unknown_bounds_133 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_133 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_134 {}
impl Clone for __is_array_unknown_bounds_134 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_134 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_135 {}
impl Clone for __is_array_unknown_bounds_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_135 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_136 {}
impl Clone for __is_array_unknown_bounds_136 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_136 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_137 {}
impl Clone for __is_array_unknown_bounds_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_137 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_138 {}
impl Clone for __is_array_unknown_bounds_138 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_138 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_139 {}
impl Clone for __is_array_unknown_bounds_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_139 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_140 {}
impl Clone for __is_array_unknown_bounds_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_140 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_141 {}
impl Clone for __is_array_unknown_bounds_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_142 {}
impl Clone for __is_array_unknown_bounds_142 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_142 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_143 {}
impl Clone for __is_array_unknown_bounds_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_143 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_144 {}
impl Clone for __is_array_unknown_bounds_144 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_144 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_145 {}
impl Clone for __is_array_unknown_bounds_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_145 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_146 {}
impl Clone for __is_integral_helper_146 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_146 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_147 {}
impl Clone for __is_integral_helper_147 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_147 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_148 {}
impl Clone for __is_integral_helper_148 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_148 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_149 {}
impl Clone for __is_integral_helper_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_150 {}
impl Clone for __is_integral_helper_150 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_150 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_151 {}
impl Clone for __is_integral_helper_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_152 {}
impl Clone for __is_integral_helper_152 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_152 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_153 {}
impl Clone for __is_integral_helper_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_154 {}
impl Clone for __is_integral_helper_154 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_154 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_155 {}
impl Clone for __is_integral_helper_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_156 {}
impl Clone for is_integral_156 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_156 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_157 {}
impl Clone for is_integral_157 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_157 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_158 {}
impl Clone for is_integral_158 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_158 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_159 {}
impl Clone for is_integral_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_159 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_160 {}
impl Clone for is_integral_160 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_160 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_161 {}
impl Clone for is_integral_161 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_161 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_162 {}
impl Clone for is_integral_162 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_162 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_163 {}
impl Clone for is_integral_163 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_163 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_164 {}
impl Clone for is_integral_164 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_164 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_165 {}
impl Clone for is_integral_165 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_165 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_floating_point_helper_166 {}
impl Clone for __is_floating_point_helper_166 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_floating_point_helper_166 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_floating_point_helper_167 {}
impl Clone for __is_floating_point_helper_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_floating_point_helper_167 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_floating_point_168 {}
impl Clone for is_floating_point_168 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_floating_point_168 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_floating_point_169 {}
impl Clone for is_floating_point_169 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_floating_point_169 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_170 {}
impl Clone for is_pointer_170 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_170 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_171 {}
impl Clone for is_pointer_171 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_171 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_172 {}
impl Clone for is_pointer_172 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_172 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_173 {}
impl Clone for is_pointer_173 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_173 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_174 {}
impl Clone for is_pointer_174 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_174 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_enum_175 {}
impl Clone for is_enum_175 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_enum_175 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_null_pointer_176 {}
impl Clone for is_null_pointer_176 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_null_pointer_176 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_177 {}
impl Clone for is_arithmetic_177 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_177 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_178 {}
impl Clone for is_arithmetic_178 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_178 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_179 {}
impl Clone for is_arithmetic_179 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_179 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_180 {}
impl Clone for is_arithmetic_180 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_180 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_member_pointer_181 {}
impl Clone for is_member_pointer_181 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_pointer_181 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_182 {}
impl Clone for is_scalar_182 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_182 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_183 {}
impl Clone for is_scalar_183 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_183 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_184 {}
impl Clone for is_scalar_184 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_184 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_185 {}
impl Clone for is_scalar_185 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_185 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_186 {}
impl Clone for is_same_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_187 {}
impl Clone for is_same_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_187 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_188 {}
impl Clone for is_same_188 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_188 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_189 {}
impl Clone for is_same_189 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_189 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_190 {}
impl Clone for is_same_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_190 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_191 {}
impl Clone for is_same_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_191 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_192 {}
impl Clone for is_same_192 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_192 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_193 {}
impl Clone for is_same_193 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_193 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_194 {}
impl Clone for is_volatile_194 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_194 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_195 {}
impl Clone for is_volatile_195 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_195 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_196 {}
impl Clone for is_volatile_196 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_196 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_197 {}
impl Clone for is_volatile_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_197 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_198 {}
impl Clone for is_volatile_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_all_extents_199 {}
impl Clone for remove_all_extents_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_all_extents_199 {
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
pub struct __is_destructible_impl_200 {}
impl Clone for __is_destructible_impl_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_impl_200 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_safe_201 {}
impl Clone for __is_destructible_safe_201 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_201 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_safe_202 {}
impl Clone for __is_destructible_safe_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_202 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_safe_203 {}
impl Clone for __is_destructible_safe_203 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_203 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_safe_204 {}
impl Clone for __is_destructible_safe_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_204 {
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
pub struct __is_nt_destructible_impl_205 {}
impl Clone for __is_nt_destructible_impl_205 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_impl_205 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nt_destructible_safe_206 {}
impl Clone for __is_nt_destructible_safe_206 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_safe_206 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nt_destructible_safe_207 {}
impl Clone for __is_nt_destructible_safe_207 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_safe_207 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nt_destructible_safe_208 {}
impl Clone for __is_nt_destructible_safe_208 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_safe_208 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nt_destructible_safe_209 {}
impl Clone for __is_nt_destructible_safe_209 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_safe_209 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_destructible_210 {}
impl Clone for is_nothrow_destructible_210 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_destructible_210 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_destructible_211 {}
impl Clone for is_nothrow_destructible_211 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_destructible_211 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_destructible_212 {}
impl Clone for is_nothrow_destructible_212 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_destructible_212 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_destructible_213 {}
impl Clone for is_nothrow_destructible_213 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_destructible_213 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_214 {}
impl Clone for is_constructible_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_214 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_215 {}
impl Clone for is_constructible_215 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_215 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_216 {}
impl Clone for is_constructible_216 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_216 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_217 {}
impl Clone for is_constructible_217 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_217 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_218 {}
impl Clone for is_constructible_218 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_218 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_copy_constructible_219 {}
impl Clone for is_copy_constructible_219 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_constructible_219 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_220 {}
impl Clone for is_move_constructible_220 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_220 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_221 {}
impl Clone for is_move_constructible_221 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_221 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_222 {}
impl Clone for is_move_constructible_222 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_222 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_223 {}
impl Clone for is_move_constructible_223 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_223 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_224 {}
impl Clone for is_nothrow_default_constructible_224 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_224 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_225 {}
impl Clone for is_nothrow_default_constructible_225 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_225 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_226 {}
impl Clone for is_nothrow_default_constructible_226 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_226 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_227 {}
impl Clone for is_nothrow_default_constructible_227 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_227 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_228 {}
impl Clone for is_nothrow_copy_constructible_228 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_228 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_229 {}
impl Clone for is_nothrow_copy_constructible_229 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_229 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_230 {}
impl Clone for is_nothrow_copy_constructible_230 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_230 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_231 {}
impl Clone for is_nothrow_copy_constructible_231 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_231 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_232 {}
impl Clone for is_nothrow_copy_constructible_232 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_232 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_constructible_233 {}
impl Clone for is_nothrow_move_constructible_233 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_233 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_copy_assignable_234 {}
impl Clone for is_copy_assignable_234 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_assignable_234 {
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
pub struct is_trivially_destructible_235 {}
impl Clone for is_trivially_destructible_235 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_235 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_trivially_destructible_236 {}
impl Clone for is_trivially_destructible_236 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_236 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_trivially_destructible_237 {}
impl Clone for is_trivially_destructible_237 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_237 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_trivially_destructible_238 {}
impl Clone for is_trivially_destructible_238 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_238 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_239 {}
impl Clone for is_convertible_239 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_239 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_240 {}
impl Clone for is_convertible_240 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_240 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_241 {}
impl Clone for is_convertible_241 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_241 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_242 {}
impl Clone for __is_nothrow_new_constructible_impl_242 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_242 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_243 {}
impl Clone for __is_nothrow_new_constructible_impl_243 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_243 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_244 {}
impl Clone for __is_nothrow_new_constructible_impl_244 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_244 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_245 {}
impl Clone for __is_nothrow_new_constructible_impl_245 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_245 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_246 {}
impl Clone for __is_nothrow_new_constructible_impl_246 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_246 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_247 {}
impl Clone for remove_reference_247 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_247 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_248 {}
impl Clone for remove_reference_248 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_248 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_249 {}
impl Clone for remove_reference_249 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_249 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_250 {}
impl Clone for remove_reference_250 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_250 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_251 {}
impl Clone for remove_reference_251 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_251 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_252 {}
impl Clone for remove_reference_252 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_252 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_253 {}
impl Clone for remove_reference_253 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_253 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_254 {}
impl Clone for remove_reference_254 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_254 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_255 {}
impl Clone for remove_reference_255 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_255 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_256 {}
impl Clone for remove_reference_256 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_256 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_257 {}
impl Clone for remove_reference_257 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_257 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_258 {}
impl Clone for remove_reference_258 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_258 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_259 {}
impl Clone for remove_reference_259 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_259 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_260 {}
impl Clone for __cv_selector_260 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_260 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_261 {}
impl Clone for __cv_selector_261 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_261 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_262 {}
impl Clone for __cv_selector_262 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_262 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_263 {}
impl Clone for __cv_selector_263 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_263 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_264 {}
impl Clone for __match_cv_qualifiers_264 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_264 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_265 {}
impl Clone for __match_cv_qualifiers_265 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_265 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_266 {}
impl Clone for __match_cv_qualifiers_266 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_266 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_267 {}
impl Clone for __match_cv_qualifiers_267 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_267 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_268 {}
impl Clone for __match_cv_qualifiers_268 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_268 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_269 {}
impl Clone for __make_unsigned_269 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_269 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_270 {}
impl Clone for __make_unsigned_270 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_270 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_271 {}
impl Clone for __make_unsigned_271 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_271 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_272 {}
impl Clone for __make_unsigned_selector_272 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_272 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_273 {}
impl Clone for __make_unsigned_selector_273 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_273 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_274 {}
impl Clone for __make_unsigned_selector_274 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_274 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_275 {}
impl Clone for __make_unsigned_selector_275 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_275 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_276 {}
impl Clone for __make_unsigned_selector_276 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_276 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_277 {}
impl Clone for __make_unsigned_selector_277 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_277 {
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
pub struct _List_281 {}
impl Clone for _List_281 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_281 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_282 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_283 {}
impl Clone for _List_283 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_283 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_284 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_285 {}
impl Clone for _List_285 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_285 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_286 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_287 {}
impl Clone for _List_287 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_287 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_288 {}
impl Clone for _List_288 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_288 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_289 {}
impl Clone for _List_289 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_289 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_290 {}
impl Clone for __select_290 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_290 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_291 {}
impl Clone for __select_291 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_291 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_292 {}
impl Clone for __select_292 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_292 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_293 {}
impl Clone for __select_293 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_293 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_294 {}
impl Clone for __select_294 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_294 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_295 {}
impl Clone for __make_signed_295 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_295 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_296 {}
impl Clone for __make_signed_296 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_296 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_297 {}
impl Clone for __make_signed_selector_297 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_297 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_298 {}
impl Clone for __make_signed_selector_298 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_298 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_299 {}
impl Clone for __make_signed_selector_299 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_299 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_300 {}
impl Clone for __make_signed_selector_300 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_300 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_301 {}
impl Clone for __make_signed_selector_301 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_301 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_pointer_302 {}
impl Clone for remove_pointer_302 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_pointer_302 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_pointer_303 {}
impl Clone for remove_pointer_303 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_pointer_303 {
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
pub struct common_type_304 {}
impl Clone for common_type_304 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_304 {
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
    pub static __is_nothrow_new_constructible_305: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __is_nothrow_new_constructible_306: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __is_nothrow_new_constructible_307: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __is_nothrow_new_constructible_308: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __is_nothrow_new_constructible_309: Value<bool> = unimplemented!();
);
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_310 {}
impl Clone for __are_same_310 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_310 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_311 = u32;
pub const anon_311___value: anon_311 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_312 {}
impl Clone for __are_same_312 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_312 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_313 = u32;
pub const anon_313___value: anon_313 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_314 {}
impl Clone for __are_same_314 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_314 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_315 = u32;
pub const anon_315___value: anon_315 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_316 {}
impl Clone for __are_same_316 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_316 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_317 = u32;
pub const anon_317___value: anon_317 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_318 {}
impl Clone for __are_same_318 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_318 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_319 = u32;
pub const anon_319___value: anon_319 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_320 {}
impl Clone for __are_same_320 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_320 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_321 = u32;
pub const anon_321___value: anon_321 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_322 {}
impl Clone for __are_same_322 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_322 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_323 = u32;
pub const anon_323___value: anon_323 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_324 {}
impl Clone for __is_integer_324 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_324 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_325 = u32;
pub const anon_325___value: anon_325 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_326 {}
impl Clone for __is_integer_326 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_326 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_327 = u32;
pub const anon_327___value: anon_327 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_328 {}
impl Clone for __is_integer_328 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_328 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_329 = u32;
pub const anon_329___value: anon_329 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_330 {}
impl Clone for __is_integer_330 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_330 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_331 = u32;
pub const anon_331___value: anon_331 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_332 {}
impl Clone for __is_integer_332 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_332 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_333 = u32;
pub const anon_333___value: anon_333 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_334 {}
impl Clone for __is_integer_334 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_334 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_335 = u32;
pub const anon_335___value: anon_335 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_byte_336 {}
impl Clone for __is_byte_336 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_byte_336 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_337 = u32;
pub const anon_337___value: anon_337 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nonvolatile_trivially_copyable_338 {}
impl Clone for __is_nonvolatile_trivially_copyable_338 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nonvolatile_trivially_copyable_338 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_339 = u32;
pub const anon_339___value: anon_339 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_340 {}
impl Clone for __memcpyable_340 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_340 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_341 {}
impl Clone for __memcpyable_341 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_341 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_342 = u32;
pub const anon_342___value: anon_342 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_343 {}
impl Clone for __memcpyable_343 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_343 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_344 {}
impl Clone for __memcpyable_integer_344 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_344 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_345 {}
impl Clone for __memcpyable_integer_345 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_345 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_346 = u32;
pub const anon_346___width: anon_346 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_move_iterator_347 {}
impl Clone for __is_move_iterator_347 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_iterator_347 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_348 = u32;
pub const anon_348___value: anon_348 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_move_iterator_349 {}
impl Clone for __is_move_iterator_349 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_iterator_349 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_350 = u32;
pub const anon_350___value: anon_350 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_move_iterator_351 {}
impl Clone for __is_move_iterator_351 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_iterator_351 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_352 = u32;
pub const anon_352___value: anon_352 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_move_iterator_353 {}
impl Clone for __is_move_iterator_353 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_iterator_353 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_354 = u32;
pub const anon_354___value: anon_354 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __enable_if_355 {}
impl Clone for __enable_if_355 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __enable_if_355 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __conditional_type_356 {}
impl Clone for __conditional_type_356 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_type_356 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_357 {}
impl Clone for __is_integer_nonstrict_357 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_357 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_358 = u32;
pub const anon_358___width: anon_358 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_359 {}
impl Clone for __is_integer_nonstrict_359 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_359 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_360 = u32;
pub const anon_360___width: anon_360 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_361 {}
impl Clone for __numeric_traits_integer_361 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_361 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____is_signed_362: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____digits_363: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____max_364: Value<i64> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_365 {}
impl Clone for __numeric_traits_integer_365 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_365 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_366: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____digits_367: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____max_368: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_369 {}
impl Clone for __numeric_traits_floating_369 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_369 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_370 {}
impl Clone for __numeric_traits_floating_370 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_370 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_371 {}
impl Clone for __numeric_traits_floating_371 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_371 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_372 {}
impl Clone for __numeric_traits_372 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_372 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __move_if_noexcept_cond_373 {}
impl Clone for __move_if_noexcept_cond_373 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __move_if_noexcept_cond_373 {
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
    pub static piecewise_construct_374: Value<std_piecewise_construct_t> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_375 {}
impl Clone for tuple_375 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_375 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_375 {}
impl tuple_375 {}
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
pub struct iterator_376 {}
impl Clone for iterator_376 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_376 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_377 {}
impl Clone for iterator_377 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_377 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_377 {}
impl iterator_377 {}
impl iterator_377 {}
impl iterator_377 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_377>) -> Ptr<iterator_377> {
        unimplemented!()
    }
}
impl iterator_377 {
    pub fn move_assign(&self, _a0: Ptr<iterator_377>) -> Ptr<iterator_377> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_378 {}
impl Clone for iterator_378 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_378 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_378 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_378>) -> Ptr<iterator_378> {
        unimplemented!()
    }
}
impl iterator_378 {
    pub fn move_assign(&self, _a0: Ptr<iterator_378>) -> Ptr<iterator_378> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_379 {}
impl Clone for iterator_379 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_379 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_379 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_379>) -> Ptr<iterator_379> {
        unimplemented!()
    }
}
impl iterator_379 {
    pub fn move_assign(&self, _a0: Ptr<iterator_379>) -> Ptr<iterator_379> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_380 {}
impl Clone for iterator_380 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_380 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_380 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_380>) -> Ptr<iterator_380> {
        unimplemented!()
    }
}
impl iterator_380 {
    pub fn move_assign(&self, _a0: Ptr<iterator_380>) -> Ptr<iterator_380> {
        unimplemented!()
    }
}
impl iterator_380 {}
impl iterator_380 {}
impl iterator_380 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_381 {}
impl Clone for iterator_381 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_381 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_381 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_381>) -> Ptr<iterator_381> {
        unimplemented!()
    }
}
impl iterator_381 {
    pub fn move_assign(&self, _a0: Ptr<iterator_381>) -> Ptr<iterator_381> {
        unimplemented!()
    }
}
impl iterator_381 {}
impl iterator_381 {}
impl iterator_381 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_382 {}
impl Clone for iterator_traits_382 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_382 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_383 {}
impl Clone for iterator_traits_383 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_383 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_384 {}
impl Clone for iterator_traits_384 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_384 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_385 {}
impl Clone for iterator_traits_385 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_385 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_386 {}
impl Clone for iterator_traits_386 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_386 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_387 {}
impl Clone for iterator_traits_387 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_387 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_388 {}
impl Clone for iterator_traits_388 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_388 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_389 {}
impl Clone for iterator_traits_389 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_389 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_390 {}
impl Clone for iterator_traits_390 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_390 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_391 {}
impl Clone for iterator_traits_391 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_391 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_392 {}
impl Clone for iterator_traits_392 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_392 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_393 {}
impl Clone for iterator_traits_393 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_393 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_394 {}
impl Clone for iterator_traits_394 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_394 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_395 {}
impl Clone for __iterator_traits_395 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_395 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_396 {}
impl Clone for __iterator_traits_396 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_396 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_397 {}
impl Clone for __iterator_traits_397 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_397 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_398 {}
impl Clone for __iterator_traits_398 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_398 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_399 {}
impl Clone for __iterator_traits_399 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_399 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_400 {}
impl Clone for __ptr_traits_ptr_to_400 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_400 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_401 {}
impl Clone for __ptr_traits_ptr_to_401 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_401 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_402 {}
impl Clone for __ptr_traits_ptr_to_402 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_402 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_403 {}
impl Clone for __ptr_traits_ptr_to_403 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_403 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_404 {}
impl Clone for pointer_traits_404 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_404 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_405 {}
impl Clone for pointer_traits_405 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_405 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_406 {}
impl Clone for pointer_traits_406 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_406 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_407 {}
impl Clone for pointer_traits_407 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_407 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_408 {}
impl Clone for reverse_iterator_408 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_408 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_408 {
    pub fn copy_from(__x: Ptr<reverse_iterator_408>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_408 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_408>) -> Ptr<reverse_iterator_408> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_409 {}
impl Clone for reverse_iterator_409 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_409 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_409 {
    pub fn copy_from(__x: Ptr<reverse_iterator_409>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_409 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_409>) -> Ptr<reverse_iterator_409> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct reverse_iterator_410 {}
impl Clone for reverse_iterator_410 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_410 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_410 {
    pub fn new_1(__x: __normal_iterator_411) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_410 {
    pub fn copy_from(__x: Ptr<reverse_iterator_410>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_410 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_410>) -> Ptr<reverse_iterator_410> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct reverse_iterator_412 {}
impl Clone for reverse_iterator_412 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_412 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_412 {
    pub fn new_1(__x: __normal_iterator_413) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_412 {
    pub fn copy_from(__x: Ptr<reverse_iterator_412>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_412 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_412>) -> Ptr<reverse_iterator_412> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_413 {}
impl Clone for __normal_iterator_413 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_413 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_413 {
    pub fn new_1(__i: Ptr<Ptr<Elem>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_413 {}
impl __normal_iterator_413 {}
impl __normal_iterator_413 {
    pub fn copy_assign(&self, _a0: Ptr<__normal_iterator_413>) -> Ptr<__normal_iterator_413> {
        unimplemented!()
    }
}
impl __normal_iterator_413 {
    pub fn move_assign(&self, _a0: Ptr<__normal_iterator_413>) -> Ptr<__normal_iterator_413> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_411 {}
impl Clone for __normal_iterator_411 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_411 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_411 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_411 {
    pub fn new_2(__i: Ptr<Ptr<Elem>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_411 {}
impl __normal_iterator_411 {}
impl __normal_iterator_411 {
    pub fn copy_assign(&self, _a0: Ptr<__normal_iterator_411>) -> Ptr<__normal_iterator_411> {
        unimplemented!()
    }
}
impl __normal_iterator_411 {
    pub fn move_assign(&self, _a0: Ptr<__normal_iterator_411>) -> Ptr<__normal_iterator_411> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_414 {}
impl Clone for __normal_iterator_414 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_414 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_414 {
    pub fn new_1(__i: Ptr<Ptr<i32>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_414 {}
impl __normal_iterator_414 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_415 {}
impl Clone for __normal_iterator_415 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_415 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_415 {
    pub fn new_1(__i: Ptr<Ptr<i32>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_415 {}
impl __normal_iterator_415 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_416 {}
impl Clone for __normal_iterator_416 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_416 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_416 {
    pub fn new_1(__i: Ptr<Ptr<i8>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_416 {}
impl __normal_iterator_416 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_417 {}
impl Clone for __normal_iterator_417 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_417 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_417 {
    pub fn new_1(__i: Ptr<Ptr<i8>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_417 {}
impl __normal_iterator_417 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_418 {}
impl Clone for __normal_iterator_418 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_418 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_418 {
    pub fn new_1(__i: Ptr<Ptr<f32>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_418 {}
impl __normal_iterator_418 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_419 {}
impl Clone for __normal_iterator_419 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_419 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_419 {
    pub fn new_1(__i: Ptr<Ptr<f32>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_419 {}
impl __normal_iterator_419 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct move_iterator_420 {}
impl Clone for move_iterator_420 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for move_iterator_420 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl move_iterator_420 {
    pub fn new_1(__i: __normal_iterator_413) -> Self {
        unimplemented!()
    }
}
impl move_iterator_420 {}
impl move_iterator_420 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct move_iterator_421 {}
impl Clone for move_iterator_421 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for move_iterator_421 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl move_iterator_421 {
    pub fn new_1(__i: Ptr<Elem>) -> Self {
        unimplemented!()
    }
}
impl move_iterator_421 {}
impl move_iterator_421 {}
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
#[byte_size(1)]
pub struct __new_allocator_422 {}
impl Clone for __new_allocator_422 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_422 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_422 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_422 {
    pub fn copy_from(_a0: Ptr<__new_allocator_422>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_422 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_422>) -> Ptr<__new_allocator_422> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_425 {}
impl Clone for __new_allocator_425 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_425 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_425 {
    pub fn copy_from(_a0: Ptr<__new_allocator_425>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_425 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_425>) -> Ptr<__new_allocator_425> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_426 {}
impl Clone for __new_allocator_426 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_426 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_426 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_426 {
    pub fn copy_from(_a0: Ptr<__new_allocator_426>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_426 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_426>) -> Ptr<__new_allocator_426> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_430 {}
impl Clone for __new_allocator_430 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_430 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_430 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_430 {
    pub fn copy_from(_a0: Ptr<__new_allocator_430>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_430 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_430>) -> Ptr<__new_allocator_430> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_433 {}
impl Clone for __new_allocator_433 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_433 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_433 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_433 {
    pub fn copy_from(_a0: Ptr<__new_allocator_433>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_433 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_433>) -> Ptr<__new_allocator_433> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_436 {}
impl Clone for allocator_436 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_436 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_436 {}
impl allocator_436 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_437 {}
impl Clone for allocator_437 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_437 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_437 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_437 {
    pub fn copy_from(__a: Ptr<allocator_437>) -> Self {
        unimplemented!()
    }
}
impl allocator_437 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_437>) -> Ptr<allocator_437> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_438 {}
impl Clone for allocator_438 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_438 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_438 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_438 {
    pub fn copy_from(__a: Ptr<allocator_438>) -> Self {
        unimplemented!()
    }
}
impl allocator_438 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_438>) -> Ptr<allocator_438> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_439 {}
impl Clone for allocator_439 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_439 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_439 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_439 {
    pub fn copy_from(__a: Ptr<allocator_439>) -> Self {
        unimplemented!()
    }
}
impl allocator_439 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_439>) -> Ptr<allocator_439> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_440 {}
impl Clone for allocator_440 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_440 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_440 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_440 {
    pub fn copy_from(__a: Ptr<allocator_440>) -> Self {
        unimplemented!()
    }
}
impl allocator_440 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_440>) -> Ptr<allocator_440> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_441 {}
impl Clone for allocator_traits_441 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_441 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_441 {
    pub fn allocate_1(__a: Ptr<allocator_438>, mut __n: u64) -> Ptr<Elem> {
        unimplemented!()
    }
}
impl allocator_traits_441 {
    pub fn deallocate(__a: Ptr<allocator_438>, mut __p: Ptr<Elem>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_441 {
    pub fn max_size(__a: Ptr<allocator_438>) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_441 {
    pub fn select_on_container_copy_construction(__rhs: Ptr<allocator_438>) -> allocator_438 {
        unimplemented!()
    }
}
impl allocator_traits_441 {
    pub fn construct_442(__a: Ptr<allocator_438>, mut __p: Ptr<Elem>, __args: Ptr<Elem>) {
        unimplemented!()
    }
}
impl allocator_traits_441 {
    pub fn construct_443(__a: Ptr<allocator_438>, mut __p: Ptr<Elem>, __args: Ptr<Elem>) {
        unimplemented!()
    }
}
impl allocator_traits_441 {
    pub fn destroy_444(__a: Ptr<allocator_438>, mut __p: Ptr<Elem>) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_445 {}
impl Clone for allocator_traits_445 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_445 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_445 {
    pub fn allocate_1(__a: Ptr<allocator_439>, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
}
impl allocator_traits_445 {
    pub fn deallocate(__a: Ptr<allocator_439>, mut __p: Ptr<i32>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_445 {
    pub fn max_size(__a: Ptr<allocator_439>) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_445 {
    pub fn construct_446(__a: Ptr<allocator_439>, mut __p: Ptr<i32>, __args: Ptr<i32>) {
        unimplemented!()
    }
}
impl allocator_traits_445 {
    pub fn destroy_447(__a: Ptr<allocator_439>, mut __p: Ptr<i32>) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_448 {}
impl Clone for allocator_traits_448 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_448 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_448 {
    pub fn allocate_1(__a: Ptr<allocator_437>, mut __n: u64) -> Ptr<i8> {
        unimplemented!()
    }
}
impl allocator_traits_448 {
    pub fn deallocate(__a: Ptr<allocator_437>, mut __p: Ptr<i8>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_448 {
    pub fn max_size(__a: Ptr<allocator_437>) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_448 {
    pub fn construct_449(__a: Ptr<allocator_437>, mut __p: Ptr<i8>, __args: Ptr<i8>) {
        unimplemented!()
    }
}
impl allocator_traits_448 {
    pub fn destroy_450(__a: Ptr<allocator_437>, mut __p: Ptr<i8>) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_451 {}
impl Clone for allocator_traits_451 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_451 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_451 {
    pub fn allocate_1(__a: Ptr<allocator_440>, mut __n: u64) -> Ptr<f32> {
        unimplemented!()
    }
}
impl allocator_traits_451 {
    pub fn deallocate(__a: Ptr<allocator_440>, mut __p: Ptr<f32>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_451 {
    pub fn max_size(__a: Ptr<allocator_440>) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_451 {
    pub fn construct_452(__a: Ptr<allocator_440>, mut __p: Ptr<f32>, __args: Ptr<f32>) {
        unimplemented!()
    }
}
impl allocator_traits_451 {
    pub fn destroy_453(__a: Ptr<allocator_440>, mut __p: Ptr<f32>) {
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
pub struct __is_move_insertable_455 {}
impl Clone for __is_move_insertable_455 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_455 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_move_insertable_456 {}
impl Clone for __is_move_insertable_456 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_456 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_move_insertable_457 {}
impl Clone for __is_move_insertable_457 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_457 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_move_insertable_458 {}
impl Clone for __is_move_insertable_458 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_458 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __shrink_to_fit_aux_459 {}
impl Clone for __shrink_to_fit_aux_459 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __shrink_to_fit_aux_459 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __shrink_to_fit_aux_459 {
    pub fn _S_do_it(__c: Ptr<vector_460>) -> bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_461 {}
impl Clone for __alloc_traits_461 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_461 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_462 {}
impl Clone for rebind_462 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_462 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __alloc_traits_461 {
    pub fn _S_select_on_copy(__a: Ptr<allocator_438>) -> allocator_438 {
        unimplemented!()
    }
}
impl __alloc_traits_461 {
    pub fn _S_on_swap(__a: Ptr<allocator_438>, __b: Ptr<allocator_438>) {
        unimplemented!()
    }
}
impl __alloc_traits_461 {
    pub fn _S_propagate_on_copy_assign() -> bool {
        unimplemented!()
    }
}
impl __alloc_traits_461 {
    pub fn _S_propagate_on_move_assign() -> bool {
        unimplemented!()
    }
}
impl __alloc_traits_461 {
    pub fn _S_always_equal() -> bool {
        unimplemented!()
    }
}
impl __alloc_traits_461 {
    pub fn _S_nothrow_move() -> bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_463 {}
impl Clone for __alloc_traits_463 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_463 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_464 {}
impl Clone for rebind_464 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_464 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_465 {}
impl Clone for __alloc_traits_465 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_465 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_466 {}
impl Clone for rebind_466 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_466 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_467 {}
impl Clone for __alloc_traits_467 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_467 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_468 {}
impl Clone for rebind_468 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_468 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct _UninitDestroyGuard_469 {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_first: Ptr<Elem>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_cur: Ptr<Ptr<Elem>>,
}
impl Clone for _UninitDestroyGuard_469 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _UninitDestroyGuard_469 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_469 {
    pub fn new(__first: Ptr<Ptr<Elem>>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __uninitialized_default_n_1_470 {}
impl Clone for __uninitialized_default_n_1_470 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __uninitialized_default_n_1_470 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __uninitialized_default_n_1_470 {
    pub fn __uninit_default_n_471(mut __first: Ptr<Elem>, mut __n: u64) -> Ptr<Elem> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_bitwise_relocatable_472 {}
impl Clone for __is_bitwise_relocatable_472 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_bitwise_relocatable_472 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_bitwise_relocatable_473 {}
impl Clone for __is_bitwise_relocatable_473 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_bitwise_relocatable_473 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_bitwise_relocatable_474 {}
impl Clone for __is_bitwise_relocatable_474 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_bitwise_relocatable_474 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_bitwise_relocatable_475 {}
impl Clone for __is_bitwise_relocatable_475 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_bitwise_relocatable_475 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_476 {}
impl Clone for initializer_list_476 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_476 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_477 {}
impl Clone for initializer_list_477 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_477 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_477 {}
impl initializer_list_477 {}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_478 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_Elem__std_allocator_Elem____Vector_impl,
}
impl Clone for _Vector_base_478 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_478 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_Elem__std_allocator_Elem____Vector_impl_data {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_start: Ptr<Elem>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_finish: Ptr<Elem>,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_end_of_storage: Ptr<Elem>,
}
impl Clone for std__Vector_base_Elem__std_allocator_Elem____Vector_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_Elem__std_allocator_Elem____Vector_impl_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Elem__std_allocator_Elem____Vector_impl_data {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Elem__std_allocator_Elem____Vector_impl_data {
    pub fn move_from(
        __x: Ptr<std__Vector_base_Elem__std_allocator_Elem____Vector_impl_data>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_Elem__std_allocator_Elem____Vector_impl {}
impl Clone for std__Vector_base_Elem__std_allocator_Elem____Vector_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_Elem__std_allocator_Elem____Vector_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Elem__std_allocator_Elem____Vector_impl {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Elem__std_allocator_Elem____Vector_impl {
    pub fn new_2(__a: Ptr<allocator_438>) -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Elem__std_allocator_Elem____Vector_impl {
    pub fn move_from(__x: Ptr<std__Vector_base_Elem__std_allocator_Elem____Vector_impl>) -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Elem__std_allocator_Elem____Vector_impl {
    pub fn new_3(
        __a: Ptr<allocator_438>,
        __rv: Ptr<std__Vector_base_Elem__std_allocator_Elem____Vector_impl>,
    ) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_478 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_478 {
    pub fn new_2(__a: Ptr<allocator_438>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_478 {
    pub fn new_3(mut __n: usize, __a: Ptr<allocator_438>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_478 {
    pub fn move_from(_a0: Ptr<_Vector_base_478>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_478 {
    pub fn new_4(__a: Ptr<allocator_438>, __x: Ptr<_Vector_base_478>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_479 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_int__std_allocator_int____Vector_impl,
}
impl Clone for _Vector_base_479 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_479 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_int__std_allocator_int____Vector_impl_data {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_start: Ptr<i32>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_finish: Ptr<i32>,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_end_of_storage: Ptr<i32>,
}
impl Clone for std__Vector_base_int__std_allocator_int____Vector_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_int__std_allocator_int____Vector_impl_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__std_allocator_int____Vector_impl_data {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__std_allocator_int____Vector_impl_data {
    pub fn move_from(
        __x: Ptr<std__Vector_base_int__std_allocator_int____Vector_impl_data>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_int__std_allocator_int____Vector_impl {}
impl Clone for std__Vector_base_int__std_allocator_int____Vector_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_int__std_allocator_int____Vector_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__std_allocator_int____Vector_impl {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__std_allocator_int____Vector_impl {
    pub fn move_from(__x: Ptr<std__Vector_base_int__std_allocator_int____Vector_impl>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_479 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_479 {
    pub fn move_from(_a0: Ptr<_Vector_base_479>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_480 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_char__std_allocator_char____Vector_impl,
}
impl Clone for _Vector_base_480 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_480 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_char__std_allocator_char____Vector_impl_data {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_start: Ptr<i8>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_finish: Ptr<i8>,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_end_of_storage: Ptr<i8>,
}
impl Clone for std__Vector_base_char__std_allocator_char____Vector_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_char__std_allocator_char____Vector_impl_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_char__std_allocator_char____Vector_impl_data {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_char__std_allocator_char____Vector_impl_data {
    pub fn move_from(
        __x: Ptr<std__Vector_base_char__std_allocator_char____Vector_impl_data>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_char__std_allocator_char____Vector_impl {}
impl Clone for std__Vector_base_char__std_allocator_char____Vector_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_char__std_allocator_char____Vector_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_char__std_allocator_char____Vector_impl {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_char__std_allocator_char____Vector_impl {
    pub fn move_from(__x: Ptr<std__Vector_base_char__std_allocator_char____Vector_impl>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_480 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_480 {
    pub fn move_from(_a0: Ptr<_Vector_base_480>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_481 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_float__std_allocator_float____Vector_impl,
}
impl Clone for _Vector_base_481 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_481 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_float__std_allocator_float____Vector_impl_data {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_start: Ptr<f32>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_finish: Ptr<f32>,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_end_of_storage: Ptr<f32>,
}
impl Clone for std__Vector_base_float__std_allocator_float____Vector_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_float__std_allocator_float____Vector_impl_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_float__std_allocator_float____Vector_impl_data {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_float__std_allocator_float____Vector_impl_data {
    pub fn move_from(
        __x: Ptr<std__Vector_base_float__std_allocator_float____Vector_impl_data>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_float__std_allocator_float____Vector_impl {}
impl Clone for std__Vector_base_float__std_allocator_float____Vector_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_float__std_allocator_float____Vector_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_float__std_allocator_float____Vector_impl {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_float__std_allocator_float____Vector_impl {
    pub fn move_from(__x: Ptr<std__Vector_base_float__std_allocator_float____Vector_impl>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_481 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_481 {
    pub fn move_from(_a0: Ptr<_Vector_base_481>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_460 {}
impl Clone for vector_460 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_460 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std_vector_Elem___Guard_alloc {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_storage: Ptr<Elem>,
    #[offset(8)]
    pub _M_len: u64,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_vect: Ptr<_Vector_base_478>,
}
impl Clone for std_vector_Elem___Guard_alloc {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_vector_Elem___Guard_alloc {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_vector_Elem___Guard_alloc {
    pub fn new(mut __s: Ptr<Elem>, mut __l: u64, __vect: Ptr<_Vector_base_478>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct std_vector_Elem___Temporary_value {}
impl Clone for std_vector_Elem___Temporary_value {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_vector_Elem___Temporary_value {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct std_vector_Elem___Temporary_value__Storage {
    #[offset(0)]
    pub _M_byte: u8,
    #[offset(0)]
    #[byte_size(4)]
    pub _M_val: Elem,
}
impl Clone for std_vector_Elem___Temporary_value__Storage {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_vector_Elem___Temporary_value__Storage {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_vector_Elem___Temporary_value__Storage {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std_vector_Elem___Temporary_value__Storage {}
impl std_vector_Elem___Temporary_value {}
impl std_vector_Elem___Temporary_value {
    pub fn new(mut __vec: Ptr<vector_460>, __args: Ptr<Elem>) -> Self {
        unimplemented!()
    }
}
impl vector_460 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_460 {
    pub fn new_2(__a: Ptr<allocator_438>) -> Self {
        unimplemented!()
    }
}
impl vector_460 {
    pub fn new_3(mut __n: u64, __value: Ptr<Elem>, __a: Ptr<allocator_438>) -> Self {
        unimplemented!()
    }
}
impl vector_460 {
    pub fn copy_from(__x: Ptr<vector_460>) -> Self {
        unimplemented!()
    }
}
impl vector_460 {
    pub fn move_from(_a0: Ptr<vector_460>) -> Self {
        unimplemented!()
    }
}
impl vector_460 {
    pub fn new_4(
        __first: move_iterator_420,
        __last: move_iterator_420,
        __a: Ptr<allocator_438>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_483 {}
impl Clone for vector_483 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_483 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std_vector_int___Guard_alloc {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_storage: Ptr<i32>,
    #[offset(8)]
    pub _M_len: u64,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_vect: Ptr<_Vector_base_479>,
}
impl Clone for std_vector_int___Guard_alloc {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_vector_int___Guard_alloc {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_vector_int___Guard_alloc {
    pub fn new(mut __s: Ptr<i32>, mut __l: u64, __vect: Ptr<_Vector_base_479>) -> Self {
        unimplemented!()
    }
}
impl vector_483 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_483 {
    pub fn copy_from(__x: Ptr<vector_483>) -> Self {
        unimplemented!()
    }
}
impl vector_483 {
    pub fn move_from(_a0: Ptr<vector_483>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_484 {}
impl Clone for vector_484 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_484 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std_vector_char___Guard_alloc {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_storage: Ptr<i8>,
    #[offset(8)]
    pub _M_len: u64,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_vect: Ptr<_Vector_base_480>,
}
impl Clone for std_vector_char___Guard_alloc {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_vector_char___Guard_alloc {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_vector_char___Guard_alloc {
    pub fn new(mut __s: Ptr<i8>, mut __l: u64, __vect: Ptr<_Vector_base_480>) -> Self {
        unimplemented!()
    }
}
impl vector_484 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_484 {
    pub fn copy_from(__x: Ptr<vector_484>) -> Self {
        unimplemented!()
    }
}
impl vector_484 {
    pub fn move_from(_a0: Ptr<vector_484>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_485 {}
impl Clone for vector_485 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_485 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std_vector_float___Guard_alloc {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_storage: Ptr<f32>,
    #[offset(8)]
    pub _M_len: u64,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_vect: Ptr<_Vector_base_481>,
}
impl Clone for std_vector_float___Guard_alloc {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_vector_float___Guard_alloc {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_vector_float___Guard_alloc {
    pub fn new(mut __s: Ptr<f32>, mut __l: u64, __vect: Ptr<_Vector_base_481>) -> Self {
        unimplemented!()
    }
}
impl vector_485 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_485 {
    pub fn copy_from(__x: Ptr<vector_485>) -> Self {
        unimplemented!()
    }
}
impl vector_485 {
    pub fn move_from(_a0: Ptr<vector_485>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_486 {}
impl Clone for __hash_base_486 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_486 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_487 {}
impl Clone for __hash_base_487 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_487 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_488 {}
impl Clone for __hash_base_488 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_488 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_489 {}
impl Clone for __hash_base_489 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_489 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_490 {}
impl Clone for __hash_base_490 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_490 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_491 {}
impl Clone for __hash_base_491 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_491 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_492 {}
impl Clone for __hash_base_492 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_492 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_493 {}
impl Clone for __hash_base_493 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_493 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_494 {}
impl Clone for __hash_base_494 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_494 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_495 {}
impl Clone for __hash_base_495 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_495 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_496 {}
impl Clone for __hash_base_496 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_496 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_497 {}
impl Clone for __hash_base_497 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_497 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_498 {}
impl Clone for __hash_base_498 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_498 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_499 {}
impl Clone for __hash_base_499 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_499 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_500 {}
impl Clone for __hash_base_500 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_500 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_501 {}
impl Clone for __hash_base_501 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_501 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_502 {}
impl Clone for __hash_base_502 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_502 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_503 {}
impl Clone for __hash_base_503 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_503 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_504 {}
impl Clone for __hash_base_504 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_504 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_505 {}
impl Clone for __hash_base_505 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_505 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_506 {}
impl Clone for __hash_base_506 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_506 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_507 {}
impl Clone for hash_507 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_507 {
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
    pub fn hash_508(__val: Ptr<f32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_509(__val: Ptr<f64>) -> usize {
        unimplemented!()
    }
}
pub type anon_510 = u32;
pub const anon_510__S_word_bit: anon_510 = 64;
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
pub struct binary_function_511 {}
impl Clone for binary_function_511 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_511 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_512 {}
impl Clone for greater_512 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_512 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_513 {}
impl Clone for less_513 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_513 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_equal_514 {}
impl Clone for greater_equal_514 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_514 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_equal_515 {}
impl Clone for less_equal_515 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_515 {
    fn default() -> Self {
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
    pub static allocator_arg_516: Value<std_allocator_arg_t> = unimplemented!();
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct MyContainer_517 {
    #[offset(0)]
    #[byte_size(24)]
    vec_: vector_483,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct MyContainer_518 {
    #[offset(0)]
    #[byte_size(24)]
    vec_: vector_484,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct MyContainer_519 {
    #[offset(0)]
    #[byte_size(24)]
    vec_: vector_485,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(2)]
pub struct Boxed_520 {
    #[offset(0)]
    pub value: i16,
}
impl Boxed_520 {
    pub fn twice(mut v: i16) -> i16 {
        return (((v as i32) + (v as i32)) as i16);
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Boxed_521 {
    #[offset(0)]
    pub value: i32,
}
impl Boxed_521 {
    pub fn twice(mut v: i32) -> i32 {
        return (v + v);
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Boxed_522 {
    #[offset(0)]
    pub value: i64,
}
impl Boxed_522 {
    pub fn twice(mut v: i64) -> i64 {
        return (v + v);
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Inner_524 {
    #[offset(0)]
    pub t: i32,
    #[offset(4)]
    pub u: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Outer_523 {
    #[offset(0)]
    pub v: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Inner_526 {
    #[offset(0)]
    pub t: i64,
    #[offset(8)]
    pub u: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Inner_527 {
    #[offset(0)]
    pub t: i64,
    #[offset(8)]
    pub u: i8,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Outer_525 {
    #[offset(0)]
    pub v: i64,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Elem {
    #[offset(0)]
    pub i: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let ev: Value<vector_460> = Rc::new(RefCell::new(<vector_460>::default()));
    ({
        let ___x: Value<Elem> = Rc::new(RefCell::new(Elem { i: 2 }));
        vector_460Impl::push_back_58(&ev.as_pointer(), ___x.as_pointer())
    });
    (if (({ vector_460Impl::operator_index_46(&ev.as_pointer(), 0_u64) }).with(|__s| __s.i) == 2) {
        &(0);
    } else {
        ({
            __assert_fail_528(
                Ptr::<i8>::from_string_literal(b"ev[0].i == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((({ Boxed_520::twice(2_i16) }) as i32) == 4) {
        &(0);
    } else {
        ({
            __assert_fail_528(
                Ptr::<i8>::from_string_literal(b"Boxed<short>::twice(2) == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let oi: Value<Outer_523> = Rc::new(RefCell::new(Outer_523 { v: 3 }));
    (if (({
        Inner_524Impl::sum(
            &Rc::new(RefCell::new(({ Outer_523Impl::with(&oi.as_pointer(), 4) }))).as_pointer(),
        )
    }) == 7)
    {
        &(0);
    } else {
        ({
            __assert_fail_528(
                Ptr::<i8>::from_string_literal(b"oi.with(4).sum() == 7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let ol: Value<Outer_525> = Rc::new(RefCell::new(Outer_525 { v: 5_i64 }));
    let ic: Value<Inner_527> = Rc::new(RefCell::new(Inner_527 {
        t: 6_i64,
        u: ('a' as i8),
    }));
    (if (({
        Inner_526Impl::sum(
            &Rc::new(RefCell::new(({ Outer_525Impl::with(&ol.as_pointer(), 2) }))).as_pointer(),
        )
    }) == 7)
    {
        &(0);
    } else {
        ({
            __assert_fail_528(
                Ptr::<i8>::from_string_literal(b"ol.with(2).sum() == 7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ Inner_527Impl::sum(&ic.as_pointer()) }) == (6 + (('a' as i8) as i32))) {
        &(0);
    } else {
        ({
            __assert_fail_528(
                Ptr::<i8>::from_string_literal(b"ic.sum() == 6 + \'a\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ Boxed_521::twice(3) }) == 6) {
        &(0);
    } else {
        ({
            __assert_fail_528(
                Ptr::<i8>::from_string_literal(b"Boxed<int>::twice(3) == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let bi: Value<Boxed_521> = Rc::new(RefCell::new(Boxed_521 { value: 4 }));
    (if (({ Boxed_521Impl::plus(&bi.as_pointer(), 5) }) == 9) {
        &(0);
    } else {
        ({
            __assert_fail_528(
                Ptr::<i8>::from_string_literal(b"bi.plus(5) == 9"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ Boxed_522::twice(10_i64) }) == 20_i64) {
        &(0);
    } else {
        ({
            __assert_fail_528(
                Ptr::<i8>::from_string_literal(b"Boxed<long>::twice(10) == 20"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let bl: Value<Boxed_522> = Rc::new(RefCell::new(Boxed_522 { value: 7_i64 }));
    (if (({ Boxed_522Impl::plus(&bl.as_pointer(), 1_i64) }) == 8_i64) {
        &(0);
    } else {
        ({
            __assert_fail_528(
                Ptr::<i8>::from_string_literal(b"bl.plus(1) == 8"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let imc: Value<MyContainer_517> = Rc::new(RefCell::new(<MyContainer_517>::default()));
    (if ({ MyContainer_517Impl::empty(&imc.as_pointer()) }) {
        &(0);
    } else {
        ({
            __assert_fail_528(
                Ptr::<i8>::from_string_literal(b"imc.empty()"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({
        let _item: Value<i32> = Rc::new(RefCell::new(1));
        MyContainer_517Impl::push_back(&imc.as_pointer(), _item.as_pointer())
    });
    (if (({ MyContainer_517Impl::size(&imc.as_pointer()) }) == 1_u64)
        && ((({ MyContainer_517Impl::back_4(&imc.as_pointer()) }).read()) == 1)
    {
        &(0);
    } else {
        ({
            __assert_fail_528(
                Ptr::<i8>::from_string_literal(b"imc.size() == 1 && imc.back() == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ MyContainer_517Impl::pop_back(&imc.as_pointer()) });
    (if ({ MyContainer_517Impl::empty(&imc.as_pointer()) }) {
        &(0);
    } else {
        ({
            __assert_fail_528(
                Ptr::<i8>::from_string_literal(b"imc.empty()"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let cmc: Value<MyContainer_518> = Rc::new(RefCell::new(<MyContainer_518>::default()));
    (if ({ MyContainer_518Impl::empty(&cmc.as_pointer()) }) {
        &(0);
    } else {
        ({
            __assert_fail_528(
                Ptr::<i8>::from_string_literal(b"cmc.empty()"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({
        let _item: Value<i8> = Rc::new(RefCell::new(('a' as i8)));
        MyContainer_518Impl::push_back(&cmc.as_pointer(), _item.as_pointer())
    });
    (if (({ MyContainer_518Impl::size(&cmc.as_pointer()) }) == 1_u64)
        && (((({ MyContainer_518Impl::back_4(&cmc.as_pointer()) }).read()) as i32)
            == (('a' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_528(
                Ptr::<i8>::from_string_literal(b"cmc.size() == 1 && cmc.back() == \'a\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ MyContainer_518Impl::pop_back(&cmc.as_pointer()) });
    (if ({ MyContainer_518Impl::empty(&cmc.as_pointer()) }) {
        &(0);
    } else {
        ({
            __assert_fail_528(
                Ptr::<i8>::from_string_literal(b"cmc.empty()"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let fmc: Value<MyContainer_519> = Rc::new(RefCell::new(<MyContainer_519>::default()));
    (if ({ MyContainer_519Impl::empty(&fmc.as_pointer()) }) {
        &(0);
    } else {
        ({
            __assert_fail_528(
                Ptr::<i8>::from_string_literal(b"fmc.empty()"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({
        let _item: Value<f32> = Rc::new(RefCell::new((1.0E+0 as f32)));
        MyContainer_519Impl::push_back(&fmc.as_pointer(), _item.as_pointer())
    });
    (if (({ MyContainer_519Impl::size(&fmc.as_pointer()) }) == 1_u64)
        && (((({ MyContainer_519Impl::back_4(&fmc.as_pointer()) }).read()) as f64) == 1.0E+0)
    {
        &(0);
    } else {
        ({
            __assert_fail_528(
                Ptr::<i8>::from_string_literal(b"fmc.size() == 1 && fmc.back() == 1.0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ MyContainer_519Impl::pop_back(&fmc.as_pointer()) });
    (if ({ MyContainer_519Impl::empty(&fmc.as_pointer()) }) {
        &(0);
    } else {
        ({
            __assert_fail_528(
                Ptr::<i8>::from_string_literal(b"fmc.empty()"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct std___uses_alloc0__Sink;
pub fn __assert_fail_528(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub trait Boxed_520Impl {
    fn plus(&self, other: i16) -> i16;
}
impl Boxed_520Impl for Ptr<Boxed_520> {
    fn plus(&self, mut other: i16) -> i16 {
        return ((((*self).with(|__s| __s.value) as i32) + (other as i32)) as i16);
    }
}
pub trait Boxed_521Impl {
    fn plus(&self, other: i32) -> i32;
}
impl Boxed_521Impl for Ptr<Boxed_521> {
    fn plus(&self, mut other: i32) -> i32 {
        return ((*self).with(|__s| __s.value) + other);
    }
}
pub trait Boxed_522Impl {
    fn plus(&self, other: i64) -> i64;
}
impl Boxed_522Impl for Ptr<Boxed_522> {
    fn plus(&self, mut other: i64) -> i64 {
        return ((*self).with(|__s| __s.value) + other);
    }
}
pub trait Inner_524Impl {
    fn sum(&self) -> i32;
}
impl Inner_524Impl for Ptr<Inner_524> {
    fn sum(&self) -> i32 {
        return (((*self).with(|__s| __s.t) as i32) + ((*self).with(|__s| __s.u) as i32));
    }
}
pub trait Inner_526Impl {
    fn sum(&self) -> i32;
}
impl Inner_526Impl for Ptr<Inner_526> {
    fn sum(&self) -> i32 {
        return (((*self).with(|__s| __s.t) as i32) + ((*self).with(|__s| __s.u) as i32));
    }
}
pub trait Inner_527Impl {
    fn sum(&self) -> i32;
}
impl Inner_527Impl for Ptr<Inner_527> {
    fn sum(&self) -> i32 {
        return (((*self).with(|__s| __s.t) as i32) + ((*self).with(|__s| __s.u) as i32));
    }
}
pub trait MyContainer_517Impl {
    fn empty(&self) -> bool;
    fn size(&self) -> u64;
    fn back_3(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn back_4(&self) -> Ptr<i32>;
    fn pop_back(&self);
    fn push_back(&self, item: Ptr<i32>);
}
impl MyContainer_517Impl for Ptr<MyContainer_517> {
    fn empty(&self) -> bool {
        return ({ vector_483Impl::empty(&field_ptr!((*self), vec_)) });
    }
    fn size(&self) -> u64 {
        return ({ vector_483Impl::size(&field_ptr!((*self), vec_)) });
    }
    fn back_4(&self) -> Ptr<i32> {
        return ({ vector_483Impl::back_53(&field_ptr!((*self), vec_)) });
    }
    fn pop_back(&self) {
        ({ vector_483Impl::pop_back(&field_ptr!((*self), vec_)) });
        return;
    }
    fn push_back(&self, item: Ptr<i32>) {
        ({
            let ___x: Ptr<i32> = (item).clone();
            vector_483Impl::push_back_57(&field_ptr!((*self), vec_), ___x)
        });
    }
}
pub trait MyContainer_518Impl {
    fn empty(&self) -> bool;
    fn size(&self) -> u64;
    fn back_3(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn back_4(&self) -> Ptr<i8>;
    fn pop_back(&self);
    fn push_back(&self, item: Ptr<i8>);
}
impl MyContainer_518Impl for Ptr<MyContainer_518> {
    fn empty(&self) -> bool {
        return ({ vector_484Impl::empty(&field_ptr!((*self), vec_)) });
    }
    fn size(&self) -> u64 {
        return ({ vector_484Impl::size(&field_ptr!((*self), vec_)) });
    }
    fn back_4(&self) -> Ptr<i8> {
        return ({ vector_484Impl::back_53(&field_ptr!((*self), vec_)) });
    }
    fn pop_back(&self) {
        ({ vector_484Impl::pop_back(&field_ptr!((*self), vec_)) });
        return;
    }
    fn push_back(&self, item: Ptr<i8>) {
        ({
            let ___x: Ptr<i8> = (item).clone();
            vector_484Impl::push_back_57(&field_ptr!((*self), vec_), ___x)
        });
    }
}
pub trait MyContainer_519Impl {
    fn empty(&self) -> bool;
    fn size(&self) -> u64;
    fn back_3(&self) -> Ptr<f32> {
        unimplemented!()
    }
    fn back_4(&self) -> Ptr<f32>;
    fn pop_back(&self);
    fn push_back(&self, item: Ptr<f32>);
}
impl MyContainer_519Impl for Ptr<MyContainer_519> {
    fn empty(&self) -> bool {
        return ({ vector_485Impl::empty(&field_ptr!((*self), vec_)) });
    }
    fn size(&self) -> u64 {
        return ({ vector_485Impl::size(&field_ptr!((*self), vec_)) });
    }
    fn back_4(&self) -> Ptr<f32> {
        return ({ vector_485Impl::back_53(&field_ptr!((*self), vec_)) });
    }
    fn pop_back(&self) {
        ({ vector_485Impl::pop_back(&field_ptr!((*self), vec_)) });
        return;
    }
    fn push_back(&self, item: Ptr<f32>) {
        ({
            let ___x: Ptr<f32> = (item).clone();
            vector_485Impl::push_back_57(&field_ptr!((*self), vec_), ___x)
        });
    }
}
pub trait Outer_523Impl {
    fn with(&self, n: i32) -> Inner_524;
}
impl Outer_523Impl for Ptr<Outer_523> {
    fn with(&self, mut n: i32) -> Inner_524 {
        return Inner_524 {
            t: (*self).with(|__s| __s.v),
            u: n,
        };
    }
}
pub trait Outer_525Impl {
    fn with(&self, n: i32) -> Inner_526;
}
impl Outer_525Impl for Ptr<Outer_525> {
    fn with(&self, mut n: i32) -> Inner_526 {
        return Inner_526 {
            t: (*self).with(|__s| __s.v),
            u: n,
        };
    }
}
pub trait _UninitDestroyGuard_469Impl {
    fn release(&self) {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_469Impl for Ptr<_UninitDestroyGuard_469> {}
pub trait _Vector_base_478Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_438> {
        unimplemented!()
    }
    fn _M_get_Tp_allocator_2(&self) -> Ptr<allocator_438> {
        unimplemented!()
    }
    fn get_allocator(&self) -> allocator_438 {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<Elem> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<Elem>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_478Impl for Ptr<_Vector_base_478> {}
pub trait _Vector_base_479Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_439> {
        unimplemented!()
    }
    fn _M_get_Tp_allocator_2(&self) -> Ptr<allocator_439> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<i32> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<i32>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_479Impl for Ptr<_Vector_base_479> {}
pub trait _Vector_base_480Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_437> {
        unimplemented!()
    }
    fn _M_get_Tp_allocator_2(&self) -> Ptr<allocator_437> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<i8> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<i8>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_480Impl for Ptr<_Vector_base_480> {}
pub trait _Vector_base_481Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_440> {
        unimplemented!()
    }
    fn _M_get_Tp_allocator_2(&self) -> Ptr<allocator_440> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<f32> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<f32>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_481Impl for Ptr<_Vector_base_481> {}
pub trait __new_allocator_422Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<i8> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<i8>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn construct_423(&self, mut __p: Ptr<i8>, __args: Ptr<i8>) {
        unimplemented!()
    }
    fn destroy_424(&self, mut __p: Ptr<i8>) {
        unimplemented!()
    }
}
impl __new_allocator_422Impl for Ptr<__new_allocator_422> {}
pub trait __new_allocator_426Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<Elem> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<Elem>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn construct_427(&self, mut __p: Ptr<Elem>, __args: Ptr<Elem>) {
        unimplemented!()
    }
    fn construct_428(&self, mut __p: Ptr<Elem>, __args: Ptr<Elem>) {
        unimplemented!()
    }
    fn destroy_429(&self, mut __p: Ptr<Elem>) {
        unimplemented!()
    }
}
impl __new_allocator_426Impl for Ptr<__new_allocator_426> {}
pub trait __new_allocator_430Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<i32> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<i32>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn construct_431(&self, mut __p: Ptr<i32>, __args: Ptr<i32>) {
        unimplemented!()
    }
    fn destroy_432(&self, mut __p: Ptr<i32>) {
        unimplemented!()
    }
}
impl __new_allocator_430Impl for Ptr<__new_allocator_430> {}
pub trait __new_allocator_433Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<f32> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<f32>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn construct_434(&self, mut __p: Ptr<f32>, __args: Ptr<f32>) {
        unimplemented!()
    }
    fn destroy_435(&self, mut __p: Ptr<f32>) {
        unimplemented!()
    }
}
impl __new_allocator_433Impl for Ptr<__new_allocator_433> {}
pub trait __normal_iterator_411Impl {
    fn operator_deref(&self) -> Ptr<Elem> {
        unimplemented!()
    }
    fn operator_sub(&self, mut __n: i64) -> __normal_iterator_411 {
        unimplemented!()
    }
    fn base(&self) -> Ptr<Ptr<Elem>> {
        unimplemented!()
    }
}
impl __normal_iterator_411Impl for Ptr<__normal_iterator_411> {}
pub trait __normal_iterator_413Impl {
    fn operator_deref(&self) -> Ptr<Elem> {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<__normal_iterator_413> {
        unimplemented!()
    }
    fn operator_dec_8(&self) -> Ptr<__normal_iterator_413> {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut __n: i64) -> Ptr<__normal_iterator_413> {
        unimplemented!()
    }
    fn operator_add(&self, mut __n: i64) -> __normal_iterator_413 {
        unimplemented!()
    }
    fn operator_sub(&self, mut __n: i64) -> __normal_iterator_413 {
        unimplemented!()
    }
    fn base(&self) -> Ptr<Ptr<Elem>> {
        unimplemented!()
    }
}
impl __normal_iterator_413Impl for Ptr<__normal_iterator_413> {}
pub trait __normal_iterator_414Impl {
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_sub(&self, mut __n: i64) -> __normal_iterator_414 {
        unimplemented!()
    }
    fn base(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl __normal_iterator_414Impl for Ptr<__normal_iterator_414> {}
pub trait __normal_iterator_415Impl {
    fn base(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl __normal_iterator_415Impl for Ptr<__normal_iterator_415> {}
pub trait __normal_iterator_416Impl {
    fn operator_deref(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_sub(&self, mut __n: i64) -> __normal_iterator_416 {
        unimplemented!()
    }
    fn base(&self) -> Ptr<Ptr<i8>> {
        unimplemented!()
    }
}
impl __normal_iterator_416Impl for Ptr<__normal_iterator_416> {}
pub trait __normal_iterator_417Impl {
    fn base(&self) -> Ptr<Ptr<i8>> {
        unimplemented!()
    }
}
impl __normal_iterator_417Impl for Ptr<__normal_iterator_417> {}
pub trait __normal_iterator_418Impl {
    fn operator_deref(&self) -> Ptr<f32> {
        unimplemented!()
    }
    fn operator_sub(&self, mut __n: i64) -> __normal_iterator_418 {
        unimplemented!()
    }
    fn base(&self) -> Ptr<Ptr<f32>> {
        unimplemented!()
    }
}
impl __normal_iterator_418Impl for Ptr<__normal_iterator_418> {}
pub trait __normal_iterator_419Impl {
    fn base(&self) -> Ptr<Ptr<f32>> {
        unimplemented!()
    }
}
impl __normal_iterator_419Impl for Ptr<__normal_iterator_419> {}
pub trait initializer_list_476Impl {
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn begin(&self) -> Ptr<bool> {
        unimplemented!()
    }
}
impl initializer_list_476Impl for Ptr<initializer_list_476> {}
pub trait initializer_list_477Impl {
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn begin(&self) -> Ptr<Elem> {
        unimplemented!()
    }
    fn end(&self) -> Ptr<Elem> {
        unimplemented!()
    }
}
impl initializer_list_477Impl for Ptr<initializer_list_477> {}
pub trait integral_constant_0Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl integral_constant_0Impl for Ptr<integral_constant_0> {}
pub trait move_iterator_420Impl {
    fn base(&self) -> __normal_iterator_413 {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<Elem> {
        unimplemented!()
    }
    fn operator_inc_8(&self) -> Ptr<move_iterator_420> {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<move_iterator_420> {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut __n: i64) -> Ptr<move_iterator_420> {
        unimplemented!()
    }
}
impl move_iterator_420Impl for Ptr<move_iterator_420> {}
pub trait move_iterator_421Impl {
    fn base(&self) -> Ptr<Elem> {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<Elem> {
        unimplemented!()
    }
    fn operator_inc_8(&self) -> Ptr<move_iterator_421> {
        unimplemented!()
    }
}
impl move_iterator_421Impl for Ptr<move_iterator_421> {}
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
pub trait std__Vector_base_Elem__std_allocator_Elem____Vector_impl_dataImpl {
    fn _M_copy_data(
        &self,
        __x: Ptr<std__Vector_base_Elem__std_allocator_Elem____Vector_impl_data>,
    ) {
        unimplemented!()
    }
    fn _M_swap_data(
        &self,
        __x: Ptr<std__Vector_base_Elem__std_allocator_Elem____Vector_impl_data>,
    ) {
        unimplemented!()
    }
}
impl std__Vector_base_Elem__std_allocator_Elem____Vector_impl_dataImpl
    for Ptr<std__Vector_base_Elem__std_allocator_Elem____Vector_impl_data>
{
}
pub trait std_vector_Elem___Guard_allocImpl {
    fn _M_release(&self) -> Ptr<Elem> {
        unimplemented!()
    }
}
impl std_vector_Elem___Guard_allocImpl for Ptr<std_vector_Elem___Guard_alloc> {}
pub trait std_vector_Elem___Temporary_valueImpl {
    fn _M_val(&self) -> Ptr<Elem> {
        unimplemented!()
    }
}
impl std_vector_Elem___Temporary_valueImpl for Ptr<std_vector_Elem___Temporary_value> {}
pub trait vector_460Impl {
    fn copy_assign(&self, __x: Ptr<vector_460>) -> Ptr<vector_460> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_460>) -> Ptr<vector_460> {
        unimplemented!()
    }
    fn begin_26(&self) -> __normal_iterator_413 {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_411 {
        unimplemented!()
    }
    fn end_28(&self) -> __normal_iterator_413 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_411 {
        unimplemented!()
    }
    fn cbegin(&self) -> __normal_iterator_411 {
        unimplemented!()
    }
    fn cend(&self) -> __normal_iterator_411 {
        unimplemented!()
    }
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn capacity(&self) -> u64 {
        unimplemented!()
    }
    fn empty(&self) -> bool {
        unimplemented!()
    }
    fn operator_index_46(&self, mut __n: u64) -> Ptr<Elem> {
        unimplemented!()
    }
    fn operator_index_47(&self, mut __n: u64) -> Ptr<Elem> {
        unimplemented!()
    }
    fn back_53(&self) -> Ptr<Elem> {
        unimplemented!()
    }
    fn push_back_58(&self, __x: Ptr<Elem>) {
        unimplemented!()
    }
    fn swap(&self, __x: Ptr<vector_460>) {
        unimplemented!()
    }
    fn clear(&self) {
        unimplemented!()
    }
    fn emplace_back_482(&self, __args: Ptr<Elem>) -> Ptr<Elem> {
        unimplemented!()
    }
}
impl vector_460Impl for Ptr<vector_460> {}
pub trait vector_483Impl {
    fn copy_assign(&self, __x: Ptr<vector_483>) -> Ptr<vector_483> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_483>) -> Ptr<vector_483> {
        unimplemented!()
    }
    fn begin_26(&self) -> __normal_iterator_414 {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_415 {
        unimplemented!()
    }
    fn end_28(&self) -> __normal_iterator_414 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_415 {
        unimplemented!()
    }
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn empty(&self) -> bool {
        unimplemented!()
    }
    fn back_53(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn push_back_57(&self, __x: Ptr<i32>) {
        unimplemented!()
    }
    fn pop_back(&self) {
        unimplemented!()
    }
}
impl vector_483Impl for Ptr<vector_483> {}
pub trait vector_484Impl {
    fn copy_assign(&self, __x: Ptr<vector_484>) -> Ptr<vector_484> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_484>) -> Ptr<vector_484> {
        unimplemented!()
    }
    fn begin_26(&self) -> __normal_iterator_416 {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_417 {
        unimplemented!()
    }
    fn end_28(&self) -> __normal_iterator_416 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_417 {
        unimplemented!()
    }
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn empty(&self) -> bool {
        unimplemented!()
    }
    fn back_53(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn push_back_57(&self, __x: Ptr<i8>) {
        unimplemented!()
    }
    fn pop_back(&self) {
        unimplemented!()
    }
}
impl vector_484Impl for Ptr<vector_484> {}
pub trait vector_485Impl {
    fn copy_assign(&self, __x: Ptr<vector_485>) -> Ptr<vector_485> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_485>) -> Ptr<vector_485> {
        unimplemented!()
    }
    fn begin_26(&self) -> __normal_iterator_418 {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_419 {
        unimplemented!()
    }
    fn end_28(&self) -> __normal_iterator_418 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_419 {
        unimplemented!()
    }
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn empty(&self) -> bool {
        unimplemented!()
    }
    fn back_53(&self) -> Ptr<f32> {
        unimplemented!()
    }
    fn push_back_57(&self, __x: Ptr<f32>) {
        unimplemented!()
    }
    fn pop_back(&self) {
        unimplemented!()
    }
}
impl vector_485Impl for Ptr<vector_485> {}
pub fn __cpp2rust_init_globals() {}
