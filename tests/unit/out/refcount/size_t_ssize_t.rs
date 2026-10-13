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
pub struct integral_constant_7 {}
impl Clone for integral_constant_7 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_7 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_integral_constant_bool__true__value_8: Value<bool> = unimplemented!();
);
impl integral_constant_7 {}
impl integral_constant_7 {}
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
    pub static std_integral_constant_bool__false__value_10: Value<bool> = unimplemented!();
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_12 {}
impl Clone for integral_constant_12 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_12 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_13 {}
impl Clone for enable_if_13 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_13 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_14 {}
impl Clone for enable_if_14 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_14 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_15 {}
impl Clone for enable_if_15 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_15 {
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
pub struct __conditional_22 {}
impl Clone for __conditional_22 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_22 {
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
pub struct __and__49 {}
impl Clone for __and__49 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__49 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__50 {}
impl Clone for __and__50 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__50 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__51 {}
impl Clone for __and__51 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__51 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__52 {}
impl Clone for __and__52 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__52 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__53 {}
impl Clone for __and__53 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__53 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__54 {}
impl Clone for __and__54 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__54 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__55 {}
impl Clone for __and__55 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__55 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__56 {}
impl Clone for __and__56 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__56 {
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
pub struct __not__65 {}
impl Clone for __not__65 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__65 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__66 {}
impl Clone for __not__66 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__66 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__67 {}
impl Clone for __not__67 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__67 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__68 {}
impl Clone for __not__68 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__68 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__69 {}
impl Clone for __not__69 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__69 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__70 {}
impl Clone for __not__70 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__70 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__71 {}
impl Clone for __not__71 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__71 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_72 {}
impl Clone for is_reference_72 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_72 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_73 {}
impl Clone for is_reference_73 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_73 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_74 {}
impl Clone for is_reference_74 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_74 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_75 {}
impl Clone for is_reference_75 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_75 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_76 {}
impl Clone for is_reference_76 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_76 {
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
pub struct is_function_83 {}
impl Clone for is_function_83 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_83 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_84 {}
impl Clone for is_function_84 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_84 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_85 {}
impl Clone for is_function_85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_85 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_86 {}
impl Clone for is_function_86 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_86 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_87 {}
impl Clone for is_function_87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_88 {}
impl Clone for is_function_88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_88 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_89 {}
impl Clone for is_function_89 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_89 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_90 {}
impl Clone for is_function_90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_90 {
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
pub struct is_void_93 {}
impl Clone for is_void_93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_93 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_94 {}
impl Clone for is_void_94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_94 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_95 {}
impl Clone for is_void_95 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_95 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_96 {}
impl Clone for is_void_96 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_96 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_97 {}
impl Clone for is_void_97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_97 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_98 {}
impl Clone for is_void_98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_98 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_99 {}
impl Clone for is_void_99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_99 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_100 {}
impl Clone for is_void_100 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_100 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_101 {}
impl Clone for is_void_101 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_101 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_102 {}
impl Clone for is_void_102 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_102 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_103 {}
impl Clone for is_void_103 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_103 {
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
pub struct remove_cv_112 {}
impl Clone for remove_cv_112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_112 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_113 {}
impl Clone for remove_cv_113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_113 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_114 {}
impl Clone for remove_cv_114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_114 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_115 {}
impl Clone for remove_cv_115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_115 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_116 {}
impl Clone for remove_cv_116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_116 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_117 {}
impl Clone for remove_cv_117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_117 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_118 {}
impl Clone for is_const_118 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_118 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_119 {}
impl Clone for is_const_119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_119 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_120 {}
impl Clone for is_const_120 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_120 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_121 {}
impl Clone for is_const_121 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_121 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_122 {}
impl Clone for is_const_122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_122 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_123 {}
impl Clone for __is_array_unknown_bounds_123 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_123 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_124 {}
impl Clone for __is_array_unknown_bounds_124 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_124 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_125 {}
impl Clone for __is_array_unknown_bounds_125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_125 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_126 {}
impl Clone for __is_array_unknown_bounds_126 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_126 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_127 {}
impl Clone for __is_array_unknown_bounds_127 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_127 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_128 {}
impl Clone for __is_array_unknown_bounds_128 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_128 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_129 {}
impl Clone for __is_array_unknown_bounds_129 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_129 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_130 {}
impl Clone for __is_array_unknown_bounds_130 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_130 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_131 {}
impl Clone for __is_array_unknown_bounds_131 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_131 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_132 {}
impl Clone for __is_array_unknown_bounds_132 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_132 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_133 {}
impl Clone for __is_integral_helper_133 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_133 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_134 {}
impl Clone for __is_integral_helper_134 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_134 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_135 {}
impl Clone for __is_integral_helper_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_135 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_136 {}
impl Clone for __is_integral_helper_136 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_136 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_137 {}
impl Clone for __is_integral_helper_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_137 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_138 {}
impl Clone for is_integral_138 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_138 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_139 {}
impl Clone for is_integral_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_139 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_140 {}
impl Clone for is_integral_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_140 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_141 {}
impl Clone for is_integral_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_142 {}
impl Clone for is_integral_142 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_142 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_143 {}
impl Clone for is_pointer_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_143 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_144 {}
impl Clone for is_same_144 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_144 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_145 {}
impl Clone for is_same_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_145 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_146 {}
impl Clone for is_same_146 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_146 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_147 {}
impl Clone for is_same_147 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_147 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_148 {}
impl Clone for is_same_148 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_148 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_149 {}
impl Clone for is_same_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_150 {}
impl Clone for is_same_150 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_150 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_151 {}
impl Clone for is_same_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_152 {}
impl Clone for is_same_152 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_152 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_153 {}
impl Clone for is_same_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_154 {}
impl Clone for is_same_154 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_154 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_155 {}
impl Clone for is_same_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_156 {}
impl Clone for is_same_156 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_156 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_157 {}
impl Clone for is_same_157 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_157 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_158 {}
impl Clone for is_same_158 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_158 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_159 {}
impl Clone for is_same_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_159 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_160 {}
impl Clone for is_same_160 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_160 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_161 {}
impl Clone for is_same_161 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_161 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_162 {}
impl Clone for is_same_162 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_162 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_163 {}
impl Clone for is_volatile_163 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_163 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_164 {}
impl Clone for is_volatile_164 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_164 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_165 {}
impl Clone for is_volatile_165 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_165 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_166 {}
impl Clone for is_volatile_166 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_166 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_167 {}
impl Clone for is_volatile_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_167 {
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
pub struct is_constructible_168 {}
impl Clone for is_constructible_168 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_168 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_169 {}
impl Clone for is_constructible_169 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_169 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_170 {}
impl Clone for is_constructible_170 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_170 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_171 {}
impl Clone for is_constructible_171 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_171 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_172 {}
impl Clone for is_move_constructible_172 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_172 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_173 {}
impl Clone for is_nothrow_default_constructible_173 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_173 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_174 {}
impl Clone for is_nothrow_default_constructible_174 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_174 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_175 {}
impl Clone for is_nothrow_copy_constructible_175 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_175 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_176 {}
impl Clone for is_nothrow_copy_constructible_176 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_176 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_177 {}
impl Clone for is_nothrow_copy_constructible_177 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_177 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_178 {}
impl Clone for is_nothrow_copy_constructible_178 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_178 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_179 {}
impl Clone for is_nothrow_copy_constructible_179 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_179 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_constructible_180 {}
impl Clone for is_nothrow_move_constructible_180 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_180 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_copy_assignable_181 {}
impl Clone for is_copy_assignable_181 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_assignable_181 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_copy_assignable_182 {}
impl Clone for is_copy_assignable_182 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_assignable_182 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_183 {}
impl Clone for is_move_assignable_183 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_183 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_184 {}
impl Clone for is_move_assignable_184 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_184 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_185 {}
impl Clone for is_move_assignable_185 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_185 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_assignable_186 {}
impl Clone for is_nothrow_move_assignable_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_186 {
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
pub struct is_convertible_187 {}
impl Clone for is_convertible_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_187 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_188 {}
impl Clone for is_convertible_188 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_188 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_189 {}
impl Clone for is_convertible_189 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_189 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_190 {}
impl Clone for is_convertible_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_190 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_191 {}
impl Clone for is_convertible_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_191 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_192 {}
impl Clone for is_convertible_192 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_192 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_193 {}
impl Clone for is_convertible_193 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_193 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_194 {}
impl Clone for is_convertible_194 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_194 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_195 {}
impl Clone for is_convertible_195 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_195 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_196 {}
impl Clone for is_convertible_196 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_196 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_197 {}
impl Clone for is_convertible_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_197 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_198 {}
impl Clone for is_convertible_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_199 {}
impl Clone for is_convertible_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_199 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_200 {}
impl Clone for is_convertible_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_200 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_201 {}
impl Clone for remove_reference_201 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_201 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_202 {}
impl Clone for remove_reference_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_202 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_203 {}
impl Clone for remove_reference_203 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_203 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_204 {}
impl Clone for remove_reference_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_204 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_205 {}
impl Clone for remove_reference_205 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_205 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_206 {}
impl Clone for remove_reference_206 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_206 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_207 {}
impl Clone for remove_reference_207 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_207 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_208 {}
impl Clone for remove_reference_208 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_208 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_209 {}
impl Clone for remove_reference_209 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_209 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_210 {}
impl Clone for remove_reference_210 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_210 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_211 {}
impl Clone for remove_reference_211 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_211 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_212 {}
impl Clone for __cv_selector_212 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_212 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_213 {}
impl Clone for __cv_selector_213 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_213 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_214 {}
impl Clone for __cv_selector_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_214 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_215 {}
impl Clone for __cv_selector_215 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_215 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_216 {}
impl Clone for __match_cv_qualifiers_216 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_216 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_217 {}
impl Clone for __match_cv_qualifiers_217 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_217 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_218 {}
impl Clone for __match_cv_qualifiers_218 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_218 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_219 {}
impl Clone for __match_cv_qualifiers_219 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_219 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_220 {}
impl Clone for __match_cv_qualifiers_220 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_220 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_221 {}
impl Clone for __make_unsigned_221 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_221 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_222 {}
impl Clone for __make_unsigned_222 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_222 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_223 {}
impl Clone for __make_unsigned_223 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_223 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_224 {}
impl Clone for __make_unsigned_selector_224 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_224 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_225 {}
impl Clone for __make_unsigned_selector_225 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_225 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_226 {}
impl Clone for __make_unsigned_selector_226 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_226 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_227 {}
impl Clone for __make_unsigned_selector_227 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_227 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_228 {}
impl Clone for __make_unsigned_selector_228 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_228 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_229 {}
impl Clone for __make_unsigned_selector_229 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_229 {
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
pub struct _List_233 {}
impl Clone for _List_233 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_233 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_234 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_235 {}
impl Clone for _List_235 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_235 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_236 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_237 {}
impl Clone for _List_237 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_237 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_238 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_239 {}
impl Clone for _List_239 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_239 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_240 {}
impl Clone for _List_240 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_240 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_241 {}
impl Clone for _List_241 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_241 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_242 {}
impl Clone for __select_242 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_242 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_243 {}
impl Clone for __select_243 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_243 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_244 {}
impl Clone for __select_244 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_244 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_245 {}
impl Clone for __select_245 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_245 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_246 {}
impl Clone for __select_246 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_246 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_247 {}
impl Clone for __make_signed_247 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_247 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_248 {}
impl Clone for __make_signed_248 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_248 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_249 {}
impl Clone for __make_signed_selector_249 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_249 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_250 {}
impl Clone for __make_signed_selector_250 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_250 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_251 {}
impl Clone for __make_signed_selector_251 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_251 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_252 {}
impl Clone for __make_signed_selector_252 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_252 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_253 {}
impl Clone for __make_signed_selector_253 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_253 {
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
pub struct common_type_254 {}
impl Clone for common_type_254 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_254 {
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
pub struct __is_tuple_like_impl_255 {}
impl Clone for __is_tuple_like_impl_255 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_255 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_256 {}
impl Clone for __is_tuple_like_256 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_256 {
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
    pub static is_array_v_257: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_copyable_v_258: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_default_constructible_v_259: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_standard_layout_v_260: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_261: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_array_v_262: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_copyable_v_263: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_default_constructible_v_264: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_standard_layout_v_265: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_266: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_array_v_267: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_copyable_v_268: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_default_constructible_v_269: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_standard_layout_v_270: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_271: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_array_v_272: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_copyable_v_273: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_default_constructible_v_274: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_standard_layout_v_275: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_276: Value<bool> = unimplemented!();
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
impl std___false_type {}
impl std___false_type {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_277 {}
impl Clone for __are_same_277 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_277 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_278 = u32;
pub const anon_278___value: anon_278 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_279 {}
impl Clone for __are_same_279 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_279 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_280 = u32;
pub const anon_280___value: anon_280 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_281 {}
impl Clone for __are_same_281 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_281 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_282 = u32;
pub const anon_282___value: anon_282 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_283 {}
impl Clone for __are_same_283 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_283 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_284 = u32;
pub const anon_284___value: anon_284 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_285 {}
impl Clone for __are_same_285 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_285 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_286 = u32;
pub const anon_286___value: anon_286 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_287 {}
impl Clone for __are_same_287 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_287 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_288 = u32;
pub const anon_288___value: anon_288 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_289 {}
impl Clone for __is_integer_289 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_289 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_290 = u32;
pub const anon_290___value: anon_290 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_291 {}
impl Clone for __is_integer_291 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_291 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_292 = u32;
pub const anon_292___value: anon_292 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_293 {}
impl Clone for __is_integer_293 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_293 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_294 = u32;
pub const anon_294___value: anon_294 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_295 {}
impl Clone for __is_integer_295 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_295 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_296 = u32;
pub const anon_296___value: anon_296 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_297 {}
impl Clone for __is_integer_297 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_297 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_298 = u32;
pub const anon_298___value: anon_298 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_299 {}
impl Clone for __is_integer_299 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_299 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_300 = u32;
pub const anon_300___value: anon_300 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_301 {}
impl Clone for __is_integer_301 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_301 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_302 = u32;
pub const anon_302___value: anon_302 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_303 {}
impl Clone for __is_integer_303 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_303 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_304 = u32;
pub const anon_304___value: anon_304 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_305 {}
impl Clone for __memcpyable_integer_305 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_305 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_306 {}
impl Clone for __is_integer_nonstrict_306 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_306 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_307 = u32;
pub const anon_307___width: anon_307 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_308 {}
impl Clone for __is_integer_nonstrict_308 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_308 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_309 = u32;
pub const anon_309___width: anon_309 = 32;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_310 {}
impl Clone for __is_integer_nonstrict_310 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_310 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_311 = u32;
pub const anon_311___width: anon_311 = 32;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_312 {}
impl Clone for __is_integer_nonstrict_312 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_312 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_313 = u32;
pub const anon_313___width: anon_313 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_314 {}
impl Clone for __is_integer_nonstrict_314 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_314 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_315 = u32;
pub const anon_315___width: anon_315 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_316 {}
impl Clone for __numeric_traits_integer_316 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_316 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____is_signed_317: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____digits_318: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____max_319: Value<i64> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_320 {}
impl Clone for __numeric_traits_integer_320 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_320 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_int____is_signed_321: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_int____digits_322: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_int____max_323: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_int____min_324: Value<i32> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_325 {}
impl Clone for __numeric_traits_integer_325 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_325 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_int____is_signed_326: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_int____digits_327: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_int____max_328: Value<u32> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_329 {}
impl Clone for __numeric_traits_integer_329 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_329 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_330: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____digits_331: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____max_332: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_333 {}
impl Clone for __numeric_traits_integer_333 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_333 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long_long____is_signed_334: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long_long____digits_335: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long_long____max_336: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_337 {}
impl Clone for __numeric_traits_floating_337 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_337 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_floating_float____max_exponent10_338: Value<i32> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_339 {}
impl Clone for __numeric_traits_floating_339 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_339 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_floating_double____max_exponent10_340: Value<i32> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_341 {}
impl Clone for __numeric_traits_floating_341 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_341 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_floating_long_double____max_exponent10_342: Value<i32> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_343 {}
impl Clone for __numeric_traits_343 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_343 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_344 {}
impl Clone for __numeric_traits_344 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_344 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_345 {}
impl Clone for __numeric_traits_345 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_345 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_346 {}
impl Clone for __numeric_traits_346 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_346 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_347 {}
impl Clone for __numeric_traits_347 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_347 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_348 {}
impl Clone for __numeric_traits_348 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_348 {
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
    pub static piecewise_construct_349: Value<std_piecewise_construct_t> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct pair_350 {
    #[offset(0)]
    pub first: u64,
    #[offset(8)]
    pub second: i32,
}
impl Clone for pair_350 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pair_350 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl pair_350 {
    pub fn copy_from(_a0: Ptr<pair_350>) -> Self {
        unimplemented!()
    }
}
impl pair_350 {
    pub fn move_from(_a0: Ptr<pair_350>) -> Self {
        unimplemented!()
    }
}
impl pair_350 {
    pub fn new(__x: Ptr<u64>, __y: Ptr<i32>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_351 {}
impl Clone for tuple_351 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_351 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_351 {}
impl tuple_351 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _PCC_352 {}
impl Clone for _PCC_352 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _PCC_352 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __pair_base_353 {}
impl Clone for __pair_base_353 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pair_base_353 {
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
pub struct iterator_354 {}
impl Clone for iterator_354 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_354 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_355 {}
impl Clone for iterator_traits_355 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_355 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_356 {}
impl Clone for iterator_traits_356 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_356 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_357 {}
impl Clone for iterator_traits_357 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_357 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_358 {}
impl Clone for iterator_traits_358 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_358 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_359 {}
impl Clone for iterator_traits_359 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_359 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_360 {}
impl Clone for __ptr_traits_ptr_to_360 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_360 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_360 {
    pub fn pointer_to(__r: Ptr<i8>) -> Ptr<i8> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_361 {}
impl Clone for __ptr_traits_ptr_to_361 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_361 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_361 {
    pub fn pointer_to(__r: Ptr<i8>) -> Ptr<i8> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_362 {}
impl Clone for __ptr_traits_ptr_to_362 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_362 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_362 {
    pub fn pointer_to(__r: Ptr<i32>) -> Ptr<i32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_363 {}
impl Clone for __ptr_traits_ptr_to_363 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_363 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_363 {
    pub fn pointer_to(__r: Ptr<i32>) -> Ptr<i32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_364 {}
impl Clone for __ptr_traits_ptr_to_364 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_364 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_364 {
    pub fn pointer_to(__r: Ptr<u16>) -> Ptr<u16> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_365 {}
impl Clone for __ptr_traits_ptr_to_365 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_365 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_365 {
    pub fn pointer_to(__r: Ptr<u16>) -> Ptr<u16> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_366 {}
impl Clone for __ptr_traits_ptr_to_366 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_366 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_366 {
    pub fn pointer_to(__r: Ptr<u32>) -> Ptr<u32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_367 {}
impl Clone for __ptr_traits_ptr_to_367 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_367 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_367 {
    pub fn pointer_to(__r: Ptr<u32>) -> Ptr<u32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_368 {}
impl Clone for pointer_traits_368 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_368 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_369 {}
impl Clone for pointer_traits_369 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_369 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_370 {}
impl Clone for pointer_traits_370 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_370 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_371 {}
impl Clone for pointer_traits_371 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_371 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_372 {}
impl Clone for pointer_traits_372 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_372 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_373 {}
impl Clone for pointer_traits_373 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_373 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_374 {}
impl Clone for pointer_traits_374 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_374 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_375 {}
impl Clone for pointer_traits_375 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_375 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_376 {}
impl Clone for __normal_iterator_376 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_376 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_376 {}
impl __normal_iterator_376 {}
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
impl __gnu_cxx___ops__Iter_less_val {
    pub fn new_1() -> Self {
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
pub struct char_traits_377 {}
impl Clone for char_traits_377 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_377 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_377 {
    pub fn assign_1(__c1: Ptr<i8>, __c2: Ptr<i8>) {
        unimplemented!()
    }
}
impl char_traits_377 {
    pub fn lt(__c1: Ptr<i8>, __c2: Ptr<i8>) -> bool {
        unimplemented!()
    }
}
impl char_traits_377 {
    pub fn length(mut __s: Ptr<i8>) -> usize {
        unimplemented!()
    }
}
impl char_traits_377 {
    pub fn move_(mut __s1: Ptr<i8>, mut __s2: Ptr<i8>, mut __n: usize) -> Ptr<i8> {
        unimplemented!()
    }
}
impl char_traits_377 {
    pub fn copy(mut __s1: Ptr<i8>, mut __s2: Ptr<i8>, mut __n: usize) -> Ptr<i8> {
        unimplemented!()
    }
}
impl char_traits_377 {
    pub fn eof() -> i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_378 {}
impl Clone for char_traits_378 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_378 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_378 {
    pub fn assign_1(__c1: Ptr<i32>, __c2: Ptr<i32>) {
        unimplemented!()
    }
}
impl char_traits_378 {
    pub fn copy(mut __s1: Ptr<i32>, mut __s2: Ptr<i32>, mut __n: usize) -> Ptr<i32> {
        unimplemented!()
    }
}
impl char_traits_378 {
    pub fn eq_int_type(__c1: Ptr<u32>, __c2: Ptr<u32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_378 {
    pub fn eof() -> u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_379 {}
impl Clone for char_traits_379 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_379 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_379 {
    pub fn assign_1(__c1: Ptr<u16>, __c2: Ptr<u16>) {
        unimplemented!()
    }
}
impl char_traits_379 {
    pub fn eq(__c1: Ptr<u16>, __c2: Ptr<u16>) -> bool {
        unimplemented!()
    }
}
impl char_traits_379 {
    pub fn lt(__c1: Ptr<u16>, __c2: Ptr<u16>) -> bool {
        unimplemented!()
    }
}
impl char_traits_379 {
    pub fn copy(mut __s1: Ptr<u16>, mut __s2: Ptr<u16>, mut __n: usize) -> Ptr<u16> {
        unimplemented!()
    }
}
impl char_traits_379 {
    pub fn eq_int_type(__c1: Ptr<u16>, __c2: Ptr<u16>) -> bool {
        unimplemented!()
    }
}
impl char_traits_379 {
    pub fn eof() -> u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_380 {}
impl Clone for char_traits_380 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_380 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_380 {
    pub fn assign_1(__c1: Ptr<u32>, __c2: Ptr<u32>) {
        unimplemented!()
    }
}
impl char_traits_380 {
    pub fn eq(__c1: Ptr<u32>, __c2: Ptr<u32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_380 {
    pub fn lt(__c1: Ptr<u32>, __c2: Ptr<u32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_380 {
    pub fn copy(mut __s1: Ptr<u32>, mut __s2: Ptr<u32>, mut __n: usize) -> Ptr<u32> {
        unimplemented!()
    }
}
impl char_traits_380 {
    pub fn eq_int_type(__c1: Ptr<u32>, __c2: Ptr<u32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_380 {
    pub fn eof() -> u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_381 {}
impl Clone for initializer_list_381 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_381 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_381 {}
impl initializer_list_381 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_382 {}
impl Clone for initializer_list_382 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_382 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_382 {}
impl initializer_list_382 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_383 {}
impl Clone for initializer_list_383 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_383 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_383 {}
impl initializer_list_383 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_384 {}
impl Clone for initializer_list_384 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_384 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_384 {}
impl initializer_list_384 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_385 {}
impl Clone for initializer_list_385 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_385 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_385 {}
impl initializer_list_385 {}
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
#[byte_size(8)]
pub struct __atomic_wide_counter {
    #[offset(0)]
    pub __value64: u64,
    #[offset(0)]
    #[byte_size(8)]
    pub __value32: anon_386,
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
pub struct anon_386 {
    #[offset(0)]
    pub __low: u32,
    #[offset(4)]
    pub __high: u32,
}
impl Clone for anon_386 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_386 {
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
pub type anon_387 = u32;
pub const anon_387__S_threshold: anon_387 = 16;
pub type anon_388 = u32;
pub const anon_388__S_chunk_size: anon_388 = 7;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __pstl_execution_sequenced_policy {}
impl Clone for __pstl_execution_sequenced_policy {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pstl_execution_sequenced_policy {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __pstl_execution_parallel_policy {}
impl Clone for __pstl_execution_parallel_policy {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pstl_execution_parallel_policy {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __pstl_execution_parallel_unsequenced_policy {}
impl Clone for __pstl_execution_parallel_unsequenced_policy {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pstl_execution_parallel_unsequenced_policy {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __pstl_execution_unsequenced_policy {}
impl Clone for __pstl_execution_unsequenced_policy {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pstl_execution_unsequenced_policy {
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
pub struct allocator_389 {}
impl Clone for allocator_389 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_389 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_389 {}
impl allocator_389 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_390 {}
impl Clone for allocator_390 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_390 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_390 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_390 {
    pub fn copy_from(__a: Ptr<allocator_390>) -> Self {
        unimplemented!()
    }
}
impl allocator_390 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_390>) -> Ptr<allocator_390> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_391 {}
impl Clone for allocator_391 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_391 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_391 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_391 {
    pub fn copy_from(__a: Ptr<allocator_391>) -> Self {
        unimplemented!()
    }
}
impl allocator_391 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_391>) -> Ptr<allocator_391> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_392 {}
impl Clone for allocator_392 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_392 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_392 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_392 {
    pub fn copy_from(__a: Ptr<allocator_392>) -> Self {
        unimplemented!()
    }
}
impl allocator_392 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_392>) -> Ptr<allocator_392> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_393 {}
impl Clone for allocator_393 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_393 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_393 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_393 {
    pub fn copy_from(__a: Ptr<allocator_393>) -> Self {
        unimplemented!()
    }
}
impl allocator_393 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_393>) -> Ptr<allocator_393> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_394 {}
impl Clone for allocator_traits_394 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_394 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_394 {
    pub fn allocate_1(__a: Ptr<allocator_390>, mut __n: u64) -> Ptr<i8> {
        unimplemented!()
    }
}
impl allocator_traits_394 {
    pub fn deallocate(__a: Ptr<allocator_390>, mut __p: Ptr<i8>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_394 {
    pub fn max_size(__a: Ptr<allocator_390>) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_395 {}
impl Clone for allocator_traits_395 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_395 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_395 {
    pub fn allocate_1(__a: Ptr<allocator_391>, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
}
impl allocator_traits_395 {
    pub fn deallocate(__a: Ptr<allocator_391>, mut __p: Ptr<i32>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_395 {
    pub fn max_size(__a: Ptr<allocator_391>) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_396 {}
impl Clone for allocator_traits_396 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_396 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_396 {
    pub fn allocate_1(__a: Ptr<allocator_392>, mut __n: u64) -> Ptr<u16> {
        unimplemented!()
    }
}
impl allocator_traits_396 {
    pub fn deallocate(__a: Ptr<allocator_392>, mut __p: Ptr<u16>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_396 {
    pub fn max_size(__a: Ptr<allocator_392>) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_397 {}
impl Clone for allocator_traits_397 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_397 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_397 {
    pub fn allocate_1(__a: Ptr<allocator_393>, mut __n: u64) -> Ptr<u32> {
        unimplemented!()
    }
}
impl allocator_traits_397 {
    pub fn deallocate(__a: Ptr<allocator_393>, mut __p: Ptr<u32>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_397 {
    pub fn max_size(__a: Ptr<allocator_393>) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct basic_string_398 {}
impl Clone for basic_string_398 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_398 {
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
    pub fn new_1(mut __dat: Ptr<i8>, __a: Ptr<allocator_390>) -> Self {
        unimplemented!()
    }
}
impl std_basic_string_char___Alloc_hider {
    pub fn new_2(mut __dat: Ptr<i8>, __a: Option<Ptr<allocator_390>>) -> Self {
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
pub type anon_400 = u32;
pub const anon_400__S_local_capacity: anon_400 = 15;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_399 {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_local_buf: Value<Box<[i8]>>,
    #[offset(0)]
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_399 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_399 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_398 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl basic_string_398 {
    pub fn copy_from(__str: Ptr<basic_string_398>) -> Self {
        unimplemented!()
    }
}
impl basic_string_398 {
    pub fn new_2(mut __s: Ptr<i8>, mut __n: u64, __a: Option<Ptr<allocator_390>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_398 {
    pub fn move_from(__str: Ptr<basic_string_398>) -> Self {
        unimplemented!()
    }
}
impl basic_string_398 {
    pub fn new_3(__beg: Ptr<i8>, mut __end: Ptr<i8>, __a: Option<Ptr<allocator_390>>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct basic_string_408 {}
impl Clone for basic_string_408 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_408 {
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
    pub fn new_1(mut __dat: Ptr<i32>, __a: Ptr<allocator_391>) -> Self {
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
pub type anon_410 = u32;
pub const anon_410__S_local_capacity: anon_410 = 3;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_409 {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_local_buf: Value<Box<[i32]>>,
    #[offset(0)]
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_409 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_409 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_408 {
    pub fn copy_from(__str: Ptr<basic_string_408>) -> Self {
        unimplemented!()
    }
}
impl basic_string_408 {
    pub fn new_1(mut __s: Ptr<i32>, mut __n: u64, __a: Option<Ptr<allocator_391>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_408 {
    pub fn move_from(__str: Ptr<basic_string_408>) -> Self {
        unimplemented!()
    }
}
impl basic_string_408 {
    pub fn new_2(__beg: Ptr<i8>, mut __end: Ptr<i8>, __a: Option<Ptr<allocator_391>>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct basic_string_411 {}
impl Clone for basic_string_411 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_411 {
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
    pub fn new_1(mut __dat: Ptr<u16>, __a: Ptr<allocator_392>) -> Self {
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
pub type anon_413 = u32;
pub const anon_413__S_local_capacity: anon_413 = 7;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_412 {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_local_buf: Value<Box<[u16]>>,
    #[offset(0)]
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_412 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_412 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_411 {
    pub fn copy_from(__str: Ptr<basic_string_411>) -> Self {
        unimplemented!()
    }
}
impl basic_string_411 {
    pub fn new_1(mut __s: Ptr<u16>, mut __n: u64, __a: Option<Ptr<allocator_392>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_411 {
    pub fn move_from(__str: Ptr<basic_string_411>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct basic_string_414 {}
impl Clone for basic_string_414 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_414 {
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
    pub fn new_1(mut __dat: Ptr<u32>, __a: Ptr<allocator_393>) -> Self {
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
pub type anon_416 = u32;
pub const anon_416__S_local_capacity: anon_416 = 3;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_415 {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_local_buf: Value<Box<[u32]>>,
    #[offset(0)]
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_415 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_415 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_414 {
    pub fn copy_from(__str: Ptr<basic_string_414>) -> Self {
        unimplemented!()
    }
}
impl basic_string_414 {
    pub fn new_1(mut __s: Ptr<u32>, mut __n: u64, __a: Option<Ptr<allocator_393>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_414 {
    pub fn move_from(__str: Ptr<basic_string_414>) -> Self {
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
    pub __value: anon_417,
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
pub struct anon_417 {
    #[offset(0)]
    pub __wch: u32,
    #[offset(0)]
    #[byte_size(4)]
    pub __wchb: Value<Box<[i8]>>,
}
impl Clone for anon_417 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_417 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(64)]
pub struct basic_streambuf_418 {}
impl Clone for basic_streambuf_418 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_streambuf_418 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(64)]
pub struct basic_streambuf_419 {}
impl Clone for basic_streambuf_419 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_streambuf_419 {
    fn default() -> Self {
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct collate_420 {}
impl Clone for collate_420 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for collate_420 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct collate_421 {}
impl Clone for collate_421 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for collate_421 {
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
    pub anon_422: i32,
    #[offset(168)]
    pub anon_423: i32,
    #[offset(172)]
    pub anon_424: i32,
    #[offset(176)]
    pub anon_425: i32,
    #[offset(180)]
    pub anon_426: i32,
    #[offset(184)]
    pub anon_427: i32,
    #[offset(188)]
    pub anon_428: i32,
    #[offset(192)]
    pub anon_429: i32,
    #[offset(196)]
    pub anon_430: i32,
    #[offset(200)]
    pub anon_431: i32,
    #[offset(204)]
    pub anon_432: i32,
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
pub type anon_433 = u32;
pub const anon_433_PTHREAD_MUTEX_TIMED_NP: anon_433 = 0;
pub const anon_433_PTHREAD_MUTEX_RECURSIVE_NP: anon_433 = 1;
pub const anon_433_PTHREAD_MUTEX_ERRORCHECK_NP: anon_433 = 2;
pub const anon_433_PTHREAD_MUTEX_ADAPTIVE_NP: anon_433 = 3;
pub const anon_433_PTHREAD_MUTEX_NORMAL: anon_433 = 0;
pub const anon_433_PTHREAD_MUTEX_RECURSIVE: anon_433 = 1;
pub const anon_433_PTHREAD_MUTEX_ERRORCHECK: anon_433 = 2;
pub const anon_433_PTHREAD_MUTEX_DEFAULT: anon_433 = 0;
pub const anon_433_PTHREAD_MUTEX_FAST_NP: anon_433 = 0;
pub type anon_434 = u32;
pub const anon_434_PTHREAD_MUTEX_STALLED: anon_434 = 0;
pub const anon_434_PTHREAD_MUTEX_STALLED_NP: anon_434 = 0;
pub const anon_434_PTHREAD_MUTEX_ROBUST: anon_434 = 1;
pub const anon_434_PTHREAD_MUTEX_ROBUST_NP: anon_434 = 1;
pub type anon_435 = u32;
pub const anon_435_PTHREAD_RWLOCK_PREFER_READER_NP: anon_435 = 0;
pub const anon_435_PTHREAD_RWLOCK_PREFER_WRITER_NP: anon_435 = 1;
pub const anon_435_PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: anon_435 = 2;
pub const anon_435_PTHREAD_RWLOCK_DEFAULT_NP: anon_435 = 0;
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
pub type anon_436 = u32;
pub const anon_436_PTHREAD_CANCEL_DEFERRED: anon_436 = 0;
pub const anon_436_PTHREAD_CANCEL_ASYNCHRONOUS: anon_436 = 1;
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
    pub static __libc_single_threaded_437: Value<i8> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Char_types_438 {}
impl Clone for _Char_types_438 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Char_types_438 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Char_types_439 {}
impl Clone for _Char_types_439 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Char_types_439 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_440 {}
impl Clone for char_traits_440 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_440 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_440 {
    pub fn eq(__c1: Ptr<i8>, __c2: Ptr<i8>) -> bool {
        unimplemented!()
    }
}
impl char_traits_440 {
    pub fn length(mut __p: Ptr<i8>) -> usize {
        unimplemented!()
    }
}
impl char_traits_440 {
    pub fn find(mut __s: Ptr<i8>, mut __n: usize, __a: Ptr<i8>) -> Ptr<i8> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_441 {}
impl Clone for char_traits_441 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_441 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_441 {
    pub fn eq(__c1: Ptr<i32>, __c2: Ptr<i32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_441 {
    pub fn lt(__c1: Ptr<i32>, __c2: Ptr<i32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_441 {
    pub fn compare(mut __s1: Ptr<i32>, mut __s2: Ptr<i32>, mut __n: usize) -> i32 {
        unimplemented!()
    }
}
impl char_traits_441 {
    pub fn length(mut __p: Ptr<i32>) -> usize {
        unimplemented!()
    }
}
impl char_traits_441 {
    pub fn find(mut __s: Ptr<i32>, mut __n: usize, __a: Ptr<i32>) -> Ptr<i32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_442 {}
impl Clone for __new_allocator_442 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_442 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_442 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_442 {
    pub fn copy_from(_a0: Ptr<__new_allocator_442>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_442 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_442>) -> Ptr<__new_allocator_442> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_443 {}
impl Clone for __new_allocator_443 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_443 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_443 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_443 {
    pub fn copy_from(_a0: Ptr<__new_allocator_443>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_443 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_443>) -> Ptr<__new_allocator_443> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_444 {}
impl Clone for __new_allocator_444 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_444 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_444 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_444 {
    pub fn copy_from(_a0: Ptr<__new_allocator_444>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_444 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_444>) -> Ptr<__new_allocator_444> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_445 {}
impl Clone for __new_allocator_445 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_445 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_445 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_445 {
    pub fn copy_from(_a0: Ptr<__new_allocator_445>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_445 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_445>) -> Ptr<__new_allocator_445> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct binary_function_446 {}
impl Clone for binary_function_446 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_446 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct binary_function_447 {}
impl Clone for binary_function_447 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_447 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl binary_function_447 {}
impl binary_function_447 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct binary_function_448 {}
impl Clone for binary_function_448 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_448 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl binary_function_448 {}
impl binary_function_448 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_449 {}
impl Clone for greater_449 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_449 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_450 {}
impl Clone for less_450 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_450 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_451 {}
impl Clone for less_451 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_451 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl less_451 {}
impl less_451 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_452 {}
impl Clone for less_452 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_452 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl less_452 {}
impl less_452 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_equal_453 {}
impl Clone for greater_equal_453 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_453 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_equal_454 {}
impl Clone for less_equal_454 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_454 {
    fn default() -> Self {
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
pub struct __is_allocator_456 {}
impl Clone for __is_allocator_456 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_456 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_allocator_457 {}
impl Clone for __is_allocator_457 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_457 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_allocator_458 {}
impl Clone for __is_allocator_458 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_458 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_allocator_459 {}
impl Clone for __is_allocator_459 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_459 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_460 {}
impl Clone for __alloc_traits_460 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_460 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_461 {}
impl Clone for rebind_461 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_461 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_462 {}
impl Clone for __alloc_traits_462 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_462 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_463 {}
impl Clone for rebind_463 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_463 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_464 {}
impl Clone for __alloc_traits_464 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_464 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_465 {}
impl Clone for rebind_465 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_465 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_466 {}
impl Clone for __alloc_traits_466 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_466 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_467 {}
impl Clone for rebind_467 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_467 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_468 {}
impl Clone for __hash_base_468 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_468 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_469 {}
impl Clone for __hash_base_469 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_469 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_470 {}
impl Clone for __hash_base_470 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_470 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_471 {}
impl Clone for __hash_base_471 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_471 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_472 {}
impl Clone for __hash_base_472 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_472 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_473 {}
impl Clone for __hash_base_473 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_473 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_474 {}
impl Clone for __hash_base_474 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_474 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_475 {}
impl Clone for __hash_base_475 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_475 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_476 {}
impl Clone for __hash_base_476 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_476 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_477 {}
impl Clone for __hash_base_477 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_477 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_478 {}
impl Clone for __hash_base_478 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_478 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_479 {}
impl Clone for __hash_base_479 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_479 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_480 {}
impl Clone for __hash_base_480 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_480 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_481 {}
impl Clone for __hash_base_481 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_481 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_482 {}
impl Clone for __hash_base_482 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_482 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_483 {}
impl Clone for __hash_base_483 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_483 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_484 {}
impl Clone for __hash_base_484 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_484 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_485 {}
impl Clone for __hash_base_485 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_485 {
    fn default() -> Self {
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
pub struct hash_495 {}
impl Clone for hash_495 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_495 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_496 {}
impl Clone for hash_496 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_496 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_497 {}
impl Clone for hash_497 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_497 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_498 {}
impl Clone for hash_498 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_498 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_499 {}
impl Clone for hash_499 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_499 {
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
    pub fn hash_500(__val: Ptr<f32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_501(__val: Ptr<f64>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_502(__val: Ptr<i32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn __hash_combine_503(__val: Ptr<Ptr<std_error_category>>, mut __hash: usize) -> usize {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct basic_string_view_401 {}
impl Clone for basic_string_view_401 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_401 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_401 {
    pub fn copy_from(_a0: Ptr<basic_string_view_401>) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_401 {
    pub fn new_1(mut __str: Ptr<i8>, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_401 {
    pub fn copy_assign(&self, _a0: Ptr<basic_string_view_401>) -> Ptr<basic_string_view_401> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct basic_string_view_504 {}
impl Clone for basic_string_view_504 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_504 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_504 {
    pub fn copy_from(_a0: Ptr<basic_string_view_504>) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_504 {
    pub fn new_1(mut __str: Ptr<i32>, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_504 {
    pub fn copy_assign(&self, _a0: Ptr<basic_string_view_504>) -> Ptr<basic_string_view_504> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct basic_string_view_505 {}
impl Clone for basic_string_view_505 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_505 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_505 {
    pub fn copy_from(_a0: Ptr<basic_string_view_505>) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_505 {
    pub fn new_1(mut __str: Ptr<u16>, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_505 {
    pub fn copy_assign(&self, _a0: Ptr<basic_string_view_505>) -> Ptr<basic_string_view_505> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct basic_string_view_506 {}
impl Clone for basic_string_view_506 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_506 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_506 {
    pub fn copy_from(_a0: Ptr<basic_string_view_506>) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_506 {
    pub fn new_1(mut __str: Ptr<u32>, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_506 {
    pub fn copy_assign(&self, _a0: Ptr<basic_string_view_506>) -> Ptr<basic_string_view_506> {
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
    pub static __integer_to_chars_is_unsigned_507: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __integer_to_chars_is_unsigned_508: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __integer_to_chars_is_unsigned_509: Value<bool> = unimplemented!();
);
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
    pub static allocator_arg_510: Value<std_allocator_arg_t> = unimplemented!();
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
    pub static std_locale_ctype_511: Value<i32> = unimplemented!();
);
thread_local!(
    pub static std_locale_numeric_512: Value<i32> = unimplemented!();
);
thread_local!(
    pub static std_locale_collate_513: Value<i32> = unimplemented!();
);
thread_local!(
    pub static std_locale_time_514: Value<i32> = unimplemented!();
);
thread_local!(
    pub static std_locale_monetary_515: Value<i32> = unimplemented!();
);
thread_local!(
    pub static std_locale_messages_516: Value<i32> = unimplemented!();
);
pub type anon_517 = u32;
pub const anon_517__S_categories_size: anon_517 = 12;
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct std___cow_string {
    #[offset(0)]
    #[byte_size(8)]
    pub anon_518: anon_518,
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
pub struct anon_518 {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_p: Ptr<i8>,
    #[offset(0)]
    #[byte_size(8)]
    pub _M_bytes: Value<Box<[i8]>>,
}
impl Clone for anon_518 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_518 {
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
    pub fn new_1(__arg: Ptr<basic_string_398>) -> Self {
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
pub struct is_error_code_enum_519 {}
impl Clone for is_error_code_enum_519 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_519 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_code_enum_520 {}
impl Clone for is_error_code_enum_520 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_520 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_code_enum_521 {}
impl Clone for is_error_code_enum_521 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_521 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_code_enum_522 {}
impl Clone for is_error_code_enum_522 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_522 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_code_enum_523 {}
impl Clone for is_error_code_enum_523 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_523 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_condition_enum_524 {}
impl Clone for is_error_condition_enum_524 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_condition_enum_524 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_condition_enum_525 {}
impl Clone for is_error_condition_enum_525 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_condition_enum_525 {
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
    pub static std_ios_base_boolalpha_526: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_dec_527: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_fixed_528: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_hex_529: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_internal_530: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_left_531: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_oct_532: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_right_533: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_scientific_534: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_showbase_535: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_showpoint_536: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_showpos_537: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_skipws_538: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_unitbuf_539: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_uppercase_540: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_adjustfield_541: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_basefield_542: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_floatfield_543: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_in_544: Value<std__Ios_Openmode> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_out_545: Value<std__Ios_Openmode> = unimplemented!();
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
pub type anon_546 = u32;
pub const anon_546__S_local_word_size: anon_546 = 8;
pub fn add_sizes_547(mut a: usize, mut b: usize) -> usize {
    return (a).wrapping_add(b);
}
pub fn take_ulong_548(mut x: u64) -> u64 {
    return x;
}
pub fn sub_signed_549(mut a: isize, mut b: isize) -> isize {
    return (a - b);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut n: usize = (4usize as usize).wrapping_add(4_usize);
    (if (n == (4usize as usize).wrapping_add(4_usize)) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"n == sizeof(int) + 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let ul: Value<u64> = Rc::new(RefCell::new(10_u64));
    let sz: Value<usize> = Rc::new(RefCell::new(20_usize));
    let mut mixed: usize = (((*sz.borrow()) as u64).wrapping_add((*ul.borrow())) as usize);
    (if (mixed == 30_usize) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"mixed == 30"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((*sz.borrow()) > ((*ul.borrow()) as usize)) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"sz > ul"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((*ul.borrow()) as usize) < (*sz.borrow())) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"ul < sz"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if !((*sz.borrow()) == ((*ul.borrow()) as usize)) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"!(sz == ul)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut chain: usize = (((((*sz.borrow()) as u64).wrapping_add((*ul.borrow())))
        .wrapping_add(5_u64))
    .wrapping_add((8usize as u64)) as usize);
    (if (chain == (((20 + 10) + 5) as usize).wrapping_add((8usize as usize))) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"chain == 20 + 10 + 5 + sizeof(long)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut acc: usize = 100_usize;
    acc = { ((acc as u64).wrapping_add((8usize as u64))) as usize };
    acc = { (acc).wrapping_mul(2_usize) };
    acc = { ((acc as u64).wrapping_sub((*ul.borrow()))) as usize };
    (if (acc
        == ((((100_usize).wrapping_add((8usize as usize))) as usize).wrapping_mul(2_usize)
            as usize)
            .wrapping_sub(10_usize))
    {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"acc == (100 + sizeof(double)) * 2 - 10"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (*sz.borrow_mut()) = { (*sz.borrow()).wrapping_add(1_usize) };
    (if ((*sz.borrow()) == 21_usize) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"sz == 21"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut fr: usize = ({
        add_sizes_547(
            ((4usize as u64).wrapping_add(((*sz.borrow()) as u64)) as usize),
            ((*ul.borrow()) as usize),
        )
    });
    (if (fr == ((4usize as usize).wrapping_add(21_usize) as usize).wrapping_add(10_usize)) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"fr == sizeof(int) + 21 + 10"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut fr2: u64 = ({ take_ulong_548(((*sz.borrow()) as u64)) });
    (if (fr2 == 21_u64) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"fr2 == 21"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut lo: usize = ((({
        let ___a: Value<u64> = Rc::new(RefCell::new(((*sz.borrow()) as u64)));
        let ___b: Value<u64> = Rc::new(RefCell::new((8usize as u64).wrapping_add((*ul.borrow()))));
        min_551(___a.as_pointer(), ___b.as_pointer())
    })
    .read()) as usize);
    let mut hi: usize = ((({
        let ___a: Value<u64> = Rc::new(RefCell::new(
            (4usize as u64).wrapping_add(((*sz.borrow()) as u64)),
        ));
        max_552(___a.as_pointer(), ul.as_pointer())
    })
    .read()) as usize);
    (if (lo == (8usize as usize).wrapping_add(10_usize)) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"lo == sizeof(long) + 10"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (hi == (4usize as usize).wrapping_add(21_usize)) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"hi == sizeof(int) + 21"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut bound: usize = ((({
        let ___a: Value<u64> = Rc::new(RefCell::new(((*sz.borrow()) as u64)));
        let ___b: Value<u64> = Rc::new(RefCell::new((4_usize as u64)));
        min_551(___a.as_pointer(), ___b.as_pointer())
    })
    .read()) as usize);
    (if (bound == 4_usize) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"bound == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut data: [i32; 8] = [0_i32; 8];
    let mut count: usize = (32usize as usize).wrapping_div((4usize as usize));
    let mut i: usize = 0_usize;
    'loop_: while (i < count) {
        data[(i) as usize] = { (((i).wrapping_mul(2_usize)) as i32) };
        i.postfix_inc();
    }
    let mut total: usize = 0_usize;
    let mut i: usize = 0_usize;
    'loop_: while (i < count) {
        total = { (total).wrapping_add((data[(i) as usize] as usize)) };
        i.postfix_inc();
    }
    (if (total == 56_usize) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"total == 56"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut cond: usize = (if ((*sz.borrow()) > ((*ul.borrow()) as usize)) {
        ((*sz.borrow()) as u64).wrapping_add((4usize as u64))
    } else {
        (*ul.borrow())
    } as usize);
    (if (cond == (21_usize).wrapping_add((4usize as usize))) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"cond == 21 + sizeof(int)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut arr: [usize; 4] = [0_usize, 1_usize, 2_usize, 3_usize];
    let mut idx: usize = (if (4usize > 2_usize) { 2 } else { 0 } as usize);
    (if (arr[(idx) as usize] == 2_usize) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"arr[idx] == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut s1: isize = 5_isize;
    let mut s2: isize = 12_isize;
    let sd: Value<isize> = Rc::new(RefCell::new(({ sub_signed_549(s1, s2) })));
    (if ((*sd.borrow()) == (-7_i32 as isize)) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"sd == -7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((*sd.borrow()) < 0_isize) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"sd < 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut l: i64 = 3_i64;
    let sm: Value<isize> = Rc::new(RefCell::new((((s2 as i64) + l) as isize)));
    (if ((*sm.borrow()) == 15_isize) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"sm == 15"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((*sm.borrow()) > (l as isize)) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"sm > l"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut smin: isize = ((({
        let ___a: Value<i64> = Rc::new(RefCell::new(((*sd.borrow()) as i64)));
        let ___b: Value<i64> = Rc::new(RefCell::new(((*sm.borrow()) as i64)));
        min_553(___a.as_pointer(), ___b.as_pointer())
    })
    .read()) as isize);
    let mut smax: isize = ((({
        let ___a: Value<i64> = Rc::new(RefCell::new(((*sd.borrow()) as i64)));
        let ___b: Value<i64> = Rc::new(RefCell::new(((*sm.borrow()) as i64)));
        max_554(___a.as_pointer(), ___b.as_pointer())
    })
    .read()) as isize);
    (if (smin == (-7_i32 as isize)) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"smin == -7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (smax == 15_isize) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"smax == 15"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut delta: isize = (((*sz.borrow()) as isize) - ((*ul.borrow()) as isize));
    (if (delta == 11_isize) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"delta == 11"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut a64: i64 = 100_i64;
    let mut b: isize = 30_isize;
    a64 -= (b as i64);
    (if (a64 == 70_i64) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"a64 == 70"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    a64 += (b as i64);
    (if (a64 == 100_i64) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"a64 == 100"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut c: isize = (-20_i32 as isize);
    a64 -= (c as i64);
    (if (a64 == 120_i64) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"a64 == 120"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((((n).wrapping_rem(7_usize)) as i32) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"(int)(n % 7) == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mx: Value<usize> = Rc::new(RefCell::new(5_usize));
    let mut mins: [usize; 4] = [
        0_usize,
        ((({
            let ___a: Value<u64> = Rc::new(RefCell::new(1_u64));
            let ___b: Value<u64> = Rc::new(RefCell::new(((*mx.borrow()) as u64)));
            min_551(___a.as_pointer(), ___b.as_pointer())
        })
        .read()) as usize),
        ((({
            let ___a: Value<u64> = Rc::new(RefCell::new(((*mx.borrow()) as u64)));
            let ___b: Value<u64> =
                Rc::new(RefCell::new(((*mx.borrow()).wrapping_sub(3_usize) as u64)));
            min_551(___a.as_pointer(), ___b.as_pointer())
        })
        .read()) as usize),
        (*mx.borrow()),
    ];
    (if (mins[(1) as usize] == 1_usize) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"mins[1] == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (mins[(2) as usize] == 2_usize) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"mins[2] == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let pr: Value<pair_350> = Rc::new(RefCell::new({
        let __tmp_0: Value<i32> = Rc::new(RefCell::new(1));
        pair_350::new({ (sz.as_pointer()).reinterpret_cast::<u64>() }, {
            __tmp_0.as_pointer()
        })
    }));
    (if ({ (*pr.borrow()).first } == 21_u64) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"pr.first == 21"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let sizes: Value<Box<[usize]>> =
        Rc::new(RefCell::new(Box::new([1_usize, 2_usize, 3_usize, 4_usize])));
    let mut sizes_end: Ptr<usize> =
        ({ end_555((sizes.as_pointer() as Ptr<usize>).reinterpret_cast::<u64>()) })
            .reinterpret_cast::<usize>();
    (if (((sizes_end).clone() - (sizes.as_pointer() as Ptr<usize>)) as i64 == 4_i64) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"sizes_end - sizes == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((sizes_end.offset(-((1) as isize))).read()) == 4_usize) {
        &(0);
    } else {
        ({
            __assert_fail_550(
                Ptr::<i8>::from_string_literal(b"*(sizes_end - 1) == 4"),
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
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_wide_data;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_codecvt;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct std_error_category;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __locale_data;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_marker;
pub fn __assert_fail_550(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn min_551(__a: Ptr<u64>, __b: Ptr<u64>) -> Ptr<u64> {
    unimplemented!()
}
pub fn max_552(__a: Ptr<u64>, __b: Ptr<u64>) -> Ptr<u64> {
    unimplemented!()
}
pub fn min_553(__a: Ptr<i64>, __b: Ptr<i64>) -> Ptr<i64> {
    unimplemented!()
}
pub fn max_554(__a: Ptr<i64>, __b: Ptr<i64>) -> Ptr<i64> {
    unimplemented!()
}
pub fn end_555(__arr: Ptr<u64>) -> Ptr<u64> {
    unimplemented!()
}
pub trait __new_allocator_442Impl {
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
impl __new_allocator_442Impl for Ptr<__new_allocator_442> {}
pub trait __new_allocator_443Impl {
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
impl __new_allocator_443Impl for Ptr<__new_allocator_443> {}
pub trait __new_allocator_444Impl {
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
impl __new_allocator_444Impl for Ptr<__new_allocator_444> {}
pub trait __new_allocator_445Impl {
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
impl __new_allocator_445Impl for Ptr<__new_allocator_445> {}
pub trait basic_string_398Impl {
    fn copy_assign(&self, __str: Ptr<basic_string_398>) -> Ptr<basic_string_398> {
        unimplemented!()
    }
    fn move_assign(&self, __str: Ptr<basic_string_398>) -> Ptr<basic_string_398> {
        unimplemented!()
    }
    fn to_basic_string_view_401(&self) -> basic_string_view_401 {
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
    fn insert_119(&self, mut __pos1: u64, __str: Ptr<basic_string_398>) -> Ptr<basic_string_398> {
        unimplemented!()
    }
    fn insert_122(&self, mut __pos: u64, mut __s: Ptr<i8>) -> Ptr<basic_string_398> {
        unimplemented!()
    }
    fn replace_133(
        &self,
        mut __pos: u64,
        mut __n1: u64,
        mut __s: Ptr<i8>,
        mut __n2: u64,
    ) -> Ptr<basic_string_398> {
        unimplemented!()
    }
    fn c_str(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn data_158(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn __resize_and_overwrite_402(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
    fn __resize_and_overwrite_403(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
    fn __resize_and_overwrite_404(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
    fn __resize_and_overwrite_405(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
    fn __resize_and_overwrite_406(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
    fn __resize_and_overwrite_407(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
}
impl basic_string_398Impl for Ptr<basic_string_398> {}
pub trait basic_string_408Impl {
    fn copy_assign(&self, __str: Ptr<basic_string_408>) -> Ptr<basic_string_408> {
        unimplemented!()
    }
    fn move_assign(&self, __str: Ptr<basic_string_408>) -> Ptr<basic_string_408> {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn c_str(&self) -> Ptr<i32> {
        unimplemented!()
    }
}
impl basic_string_408Impl for Ptr<basic_string_408> {}
pub trait basic_string_411Impl {
    fn copy_assign(&self, __str: Ptr<basic_string_411>) -> Ptr<basic_string_411> {
        unimplemented!()
    }
    fn move_assign(&self, __str: Ptr<basic_string_411>) -> Ptr<basic_string_411> {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_411Impl for Ptr<basic_string_411> {}
pub trait basic_string_414Impl {
    fn copy_assign(&self, __str: Ptr<basic_string_414>) -> Ptr<basic_string_414> {
        unimplemented!()
    }
    fn move_assign(&self, __str: Ptr<basic_string_414>) -> Ptr<basic_string_414> {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_414Impl for Ptr<basic_string_414> {}
pub trait basic_string_view_401Impl {
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
impl basic_string_view_401Impl for Ptr<basic_string_view_401> {}
pub trait basic_string_view_504Impl {
    fn length(&self) -> u64 {
        unimplemented!()
    }
    fn data(&self) -> Ptr<i32> {
        unimplemented!()
    }
}
impl basic_string_view_504Impl for Ptr<basic_string_view_504> {}
pub trait basic_string_view_505Impl {
    fn length(&self) -> u64 {
        unimplemented!()
    }
    fn data(&self) -> Ptr<u16> {
        unimplemented!()
    }
}
impl basic_string_view_505Impl for Ptr<basic_string_view_505> {}
pub trait basic_string_view_506Impl {
    fn length(&self) -> u64 {
        unimplemented!()
    }
    fn data(&self) -> Ptr<u32> {
        unimplemented!()
    }
}
impl basic_string_view_506Impl for Ptr<basic_string_view_506> {}
pub trait integral_constant_7Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl integral_constant_7Impl for Ptr<integral_constant_7> {}
pub trait less_451Impl {
    fn operator_call(
        &self,
        mut __x: Ptr<std_error_category>,
        mut __y: Ptr<std_error_category>,
    ) -> bool {
        unimplemented!()
    }
}
impl less_451Impl for Ptr<less_451> {}
pub trait less_452Impl {
    fn operator_call(&self, mut __x: Ptr<i8>, mut __y: Ptr<i8>) -> bool {
        unimplemented!()
    }
}
impl less_452Impl for Ptr<less_452> {}
pub trait pair_350Impl {
    fn copy_assign(&self, __p: Ptr<pair_350>) -> Ptr<pair_350> {
        unimplemented!()
    }
    fn move_assign(&self, __p: Ptr<pair_350>) -> Ptr<pair_350> {
        unimplemented!()
    }
}
impl pair_350Impl for Ptr<pair_350> {}
pub trait std___cow_stringImpl {
    fn copy_assign(&self, _a0: Ptr<std___cow_string>) -> Ptr<std___cow_string> {
        unimplemented!()
    }
    fn move_assign(&self, _a0: Ptr<std___cow_string>) -> Ptr<std___cow_string> {
        unimplemented!()
    }
}
impl std___cow_stringImpl for Ptr<std___cow_string> {}
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
    fn message(&self) -> basic_string_398 {
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
