extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
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
pub struct __or__65 {}
impl Clone for __or__65 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__65 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__66 {}
impl Clone for __or__66 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__66 {
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
pub struct __not__88 {}
impl Clone for __not__88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__88 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__89 {}
impl Clone for __not__89 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__89 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__90 {}
impl Clone for __not__90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__90 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__91 {}
impl Clone for __not__91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__91 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__92 {}
impl Clone for __not__92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_93 {}
impl Clone for is_reference_93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_93 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_94 {}
impl Clone for is_reference_94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_94 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_95 {}
impl Clone for is_reference_95 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_95 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_96 {}
impl Clone for is_reference_96 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_96 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_97 {}
impl Clone for is_reference_97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_97 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_98 {}
impl Clone for is_reference_98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_98 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_99 {}
impl Clone for is_reference_99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_99 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_100 {}
impl Clone for is_reference_100 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_100 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_101 {}
impl Clone for is_reference_101 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_101 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_102 {}
impl Clone for is_reference_102 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_102 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_103 {}
impl Clone for is_reference_103 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_103 {
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
pub struct is_function_110 {}
impl Clone for is_function_110 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_110 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_111 {}
impl Clone for is_function_111 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_111 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_112 {}
impl Clone for is_function_112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_112 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_113 {}
impl Clone for is_function_113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_113 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_114 {}
impl Clone for is_function_114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_114 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_115 {}
impl Clone for is_function_115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_115 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_116 {}
impl Clone for is_function_116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_116 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_117 {}
impl Clone for is_function_117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_117 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_118 {}
impl Clone for is_function_118 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_118 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_119 {}
impl Clone for is_function_119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_119 {
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
pub struct is_void_126 {}
impl Clone for is_void_126 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_126 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_127 {}
impl Clone for is_void_127 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_127 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_128 {}
impl Clone for is_void_128 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_128 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_129 {}
impl Clone for is_void_129 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_129 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_130 {}
impl Clone for is_void_130 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_130 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_131 {}
impl Clone for is_void_131 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_131 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_132 {}
impl Clone for is_void_132 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_132 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_133 {}
impl Clone for is_void_133 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_133 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_134 {}
impl Clone for is_void_134 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_134 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_135 {}
impl Clone for is_void_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_135 {
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
pub struct remove_cv_143 {}
impl Clone for remove_cv_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_143 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_144 {}
impl Clone for remove_cv_144 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_144 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_145 {}
impl Clone for remove_cv_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_145 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_146 {}
impl Clone for remove_cv_146 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_146 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_147 {}
impl Clone for remove_cv_147 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_147 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_148 {}
impl Clone for remove_cv_148 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_148 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_149 {}
impl Clone for remove_cv_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_150 {}
impl Clone for remove_cv_150 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_150 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_151 {}
impl Clone for remove_cv_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_152 {}
impl Clone for remove_cv_152 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_152 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_153 {}
impl Clone for is_const_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_154 {}
impl Clone for is_const_154 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_154 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_155 {}
impl Clone for is_const_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_156 {}
impl Clone for is_const_156 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_156 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_157 {}
impl Clone for is_const_157 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_157 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_158 {}
impl Clone for __is_array_unknown_bounds_158 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_158 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_159 {}
impl Clone for __is_array_unknown_bounds_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_159 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_160 {}
impl Clone for __is_array_unknown_bounds_160 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_160 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_161 {}
impl Clone for __is_array_unknown_bounds_161 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_161 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_162 {}
impl Clone for __is_array_unknown_bounds_162 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_162 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_163 {}
impl Clone for __is_array_unknown_bounds_163 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_163 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_164 {}
impl Clone for __is_array_unknown_bounds_164 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_164 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_165 {}
impl Clone for __is_array_unknown_bounds_165 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_165 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_166 {}
impl Clone for __is_array_unknown_bounds_166 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_166 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_167 {}
impl Clone for __is_array_unknown_bounds_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_167 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_168 {}
impl Clone for __is_array_unknown_bounds_168 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_168 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_169 {}
impl Clone for __is_array_unknown_bounds_169 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_169 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_170 {}
impl Clone for __is_array_unknown_bounds_170 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_170 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_171 {}
impl Clone for __is_array_unknown_bounds_171 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_171 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_172 {}
impl Clone for __is_array_unknown_bounds_172 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_172 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_173 {}
impl Clone for __is_array_unknown_bounds_173 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_173 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_174 {}
impl Clone for __is_integral_helper_174 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_174 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_175 {}
impl Clone for __is_integral_helper_175 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_175 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_176 {}
impl Clone for __is_integral_helper_176 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_176 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_177 {}
impl Clone for __is_integral_helper_177 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_177 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_178 {}
impl Clone for __is_integral_helper_178 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_178 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_179 {}
impl Clone for __is_integral_helper_179 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_179 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_180 {}
impl Clone for __is_integral_helper_180 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_180 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_181 {}
impl Clone for __is_integral_helper_181 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_181 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_182 {}
impl Clone for __is_integral_helper_182 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_182 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_183 {}
impl Clone for __is_integral_helper_183 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_183 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_184 {}
impl Clone for is_integral_184 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_184 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_185 {}
impl Clone for is_integral_185 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_185 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_186 {}
impl Clone for is_integral_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_187 {}
impl Clone for is_integral_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_187 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_188 {}
impl Clone for is_integral_188 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_188 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_189 {}
impl Clone for is_integral_189 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_189 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_190 {}
impl Clone for is_integral_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_190 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_191 {}
impl Clone for is_integral_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_191 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_192 {}
impl Clone for is_integral_192 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_192 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_193 {}
impl Clone for is_integral_193 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_193 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_floating_point_helper_194 {}
impl Clone for __is_floating_point_helper_194 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_floating_point_helper_194 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_floating_point_helper_195 {}
impl Clone for __is_floating_point_helper_195 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_floating_point_helper_195 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_floating_point_helper_196 {}
impl Clone for __is_floating_point_helper_196 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_floating_point_helper_196 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_floating_point_197 {}
impl Clone for is_floating_point_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_floating_point_197 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_floating_point_198 {}
impl Clone for is_floating_point_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_floating_point_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_floating_point_199 {}
impl Clone for is_floating_point_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_floating_point_199 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_200 {}
impl Clone for is_pointer_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_200 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_201 {}
impl Clone for is_pointer_201 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_201 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_202 {}
impl Clone for is_pointer_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_202 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_203 {}
impl Clone for is_pointer_203 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_203 {
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
pub struct is_pointer_206 {}
impl Clone for is_pointer_206 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_206 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_enum_207 {}
impl Clone for is_enum_207 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_enum_207 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_enum_208 {}
impl Clone for is_enum_208 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_enum_208 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_enum_209 {}
impl Clone for is_enum_209 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_enum_209 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_null_pointer_210 {}
impl Clone for is_null_pointer_210 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_null_pointer_210 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_null_pointer_211 {}
impl Clone for is_null_pointer_211 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_null_pointer_211 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_null_pointer_212 {}
impl Clone for is_null_pointer_212 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_null_pointer_212 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_213 {}
impl Clone for is_arithmetic_213 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_213 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_214 {}
impl Clone for is_arithmetic_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_214 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_215 {}
impl Clone for is_arithmetic_215 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_215 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_216 {}
impl Clone for is_arithmetic_216 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_216 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_217 {}
impl Clone for is_arithmetic_217 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_217 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_member_pointer_218 {}
impl Clone for is_member_pointer_218 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_pointer_218 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_member_pointer_219 {}
impl Clone for is_member_pointer_219 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_pointer_219 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_member_pointer_220 {}
impl Clone for is_member_pointer_220 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_pointer_220 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_221 {}
impl Clone for is_scalar_221 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_221 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_222 {}
impl Clone for is_scalar_222 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_222 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_223 {}
impl Clone for is_scalar_223 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_223 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_224 {}
impl Clone for is_scalar_224 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_224 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_225 {}
impl Clone for is_scalar_225 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_225 {
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
pub struct remove_all_extents_241 {}
impl Clone for remove_all_extents_241 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_all_extents_241 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_all_extents_242 {}
impl Clone for remove_all_extents_242 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_all_extents_242 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_all_extents_243 {}
impl Clone for remove_all_extents_243 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_all_extents_243 {
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
pub struct __is_destructible_impl_244 {}
impl Clone for __is_destructible_impl_244 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_impl_244 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_impl_245 {}
impl Clone for __is_destructible_impl_245 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_impl_245 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_impl_246 {}
impl Clone for __is_destructible_impl_246 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_impl_246 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_safe_247 {}
impl Clone for __is_destructible_safe_247 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_247 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_safe_248 {}
impl Clone for __is_destructible_safe_248 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_248 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_safe_249 {}
impl Clone for __is_destructible_safe_249 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_249 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_safe_250 {}
impl Clone for __is_destructible_safe_250 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_250 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_safe_251 {}
impl Clone for __is_destructible_safe_251 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_251 {
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
pub struct __is_nt_destructible_impl_252 {}
impl Clone for __is_nt_destructible_impl_252 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_impl_252 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nt_destructible_impl_253 {}
impl Clone for __is_nt_destructible_impl_253 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_impl_253 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nt_destructible_safe_254 {}
impl Clone for __is_nt_destructible_safe_254 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_safe_254 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nt_destructible_safe_255 {}
impl Clone for __is_nt_destructible_safe_255 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_safe_255 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_destructible_256 {}
impl Clone for is_nothrow_destructible_256 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_destructible_256 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_destructible_257 {}
impl Clone for is_nothrow_destructible_257 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_destructible_257 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_258 {}
impl Clone for is_constructible_258 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_258 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_259 {}
impl Clone for is_constructible_259 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_259 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_260 {}
impl Clone for is_constructible_260 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_260 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_261 {}
impl Clone for is_constructible_261 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_261 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_262 {}
impl Clone for is_constructible_262 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_262 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_263 {}
impl Clone for is_constructible_263 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_263 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_264 {}
impl Clone for is_constructible_264 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_264 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_265 {}
impl Clone for is_constructible_265 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_265 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_266 {}
impl Clone for is_constructible_266 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_266 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_267 {}
impl Clone for is_constructible_267 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_267 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_copy_constructible_268 {}
impl Clone for is_copy_constructible_268 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_constructible_268 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_269 {}
impl Clone for is_move_constructible_269 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_269 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_270 {}
impl Clone for is_move_constructible_270 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_270 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_271 {}
impl Clone for is_move_constructible_271 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_271 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_272 {}
impl Clone for is_move_constructible_272 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_272 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_273 {}
impl Clone for is_nothrow_default_constructible_273 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_273 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_274 {}
impl Clone for is_nothrow_default_constructible_274 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_274 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_275 {}
impl Clone for is_nothrow_default_constructible_275 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_275 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_276 {}
impl Clone for is_nothrow_default_constructible_276 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_276 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_277 {}
impl Clone for is_nothrow_default_constructible_277 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_277 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_278 {}
impl Clone for is_nothrow_copy_constructible_278 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_278 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_279 {}
impl Clone for is_nothrow_copy_constructible_279 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_279 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_280 {}
impl Clone for is_nothrow_copy_constructible_280 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_280 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_281 {}
impl Clone for is_nothrow_copy_constructible_281 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_281 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_282 {}
impl Clone for is_nothrow_copy_constructible_282 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_282 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_283 {}
impl Clone for is_nothrow_copy_constructible_283 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_283 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_constructible_284 {}
impl Clone for is_nothrow_move_constructible_284 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_284 {
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
pub struct is_trivially_destructible_285 {}
impl Clone for is_trivially_destructible_285 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_285 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_trivially_destructible_286 {}
impl Clone for is_trivially_destructible_286 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_286 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_trivially_destructible_287 {}
impl Clone for is_trivially_destructible_287 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_287 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_trivially_destructible_288 {}
impl Clone for is_trivially_destructible_288 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_288 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_trivially_destructible_289 {}
impl Clone for is_trivially_destructible_289 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_289 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_290 {}
impl Clone for is_convertible_290 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_290 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_291 {}
impl Clone for is_convertible_291 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_291 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_292 {}
impl Clone for __is_nothrow_new_constructible_impl_292 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_292 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_293 {}
impl Clone for __is_nothrow_new_constructible_impl_293 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_293 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_294 {}
impl Clone for __is_nothrow_new_constructible_impl_294 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_294 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_295 {}
impl Clone for __is_nothrow_new_constructible_impl_295 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_295 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_296 {}
impl Clone for __is_nothrow_new_constructible_impl_296 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_296 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_297 {}
impl Clone for __is_nothrow_new_constructible_impl_297 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_297 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_298 {}
impl Clone for __is_nothrow_new_constructible_impl_298 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_298 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_299 {}
impl Clone for __is_nothrow_new_constructible_impl_299 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_299 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_300 {}
impl Clone for __is_nothrow_new_constructible_impl_300 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_300 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_301 {}
impl Clone for __is_nothrow_new_constructible_impl_301 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_301 {
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
pub struct remove_reference_307 {}
impl Clone for remove_reference_307 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_307 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_308 {}
impl Clone for remove_reference_308 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_308 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_309 {}
impl Clone for remove_reference_309 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_309 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_310 {}
impl Clone for remove_reference_310 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_310 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_311 {}
impl Clone for remove_reference_311 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_311 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_312 {}
impl Clone for remove_reference_312 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_312 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_313 {}
impl Clone for remove_reference_313 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_313 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_314 {}
impl Clone for remove_reference_314 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_314 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_315 {}
impl Clone for remove_reference_315 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_315 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_316 {}
impl Clone for remove_reference_316 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_316 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_317 {}
impl Clone for __cv_selector_317 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_317 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_318 {}
impl Clone for __cv_selector_318 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_318 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_319 {}
impl Clone for __cv_selector_319 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_319 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_320 {}
impl Clone for __cv_selector_320 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_320 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_321 {}
impl Clone for __match_cv_qualifiers_321 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_321 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_322 {}
impl Clone for __match_cv_qualifiers_322 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_322 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_323 {}
impl Clone for __match_cv_qualifiers_323 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_323 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_324 {}
impl Clone for __match_cv_qualifiers_324 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_324 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_325 {}
impl Clone for __match_cv_qualifiers_325 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_325 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_326 {}
impl Clone for __make_unsigned_326 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_326 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_327 {}
impl Clone for __make_unsigned_327 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_327 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_328 {}
impl Clone for __make_unsigned_328 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_328 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_329 {}
impl Clone for __make_unsigned_selector_329 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_329 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_330 {}
impl Clone for __make_unsigned_selector_330 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_330 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_331 {}
impl Clone for __make_unsigned_selector_331 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_331 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_332 {}
impl Clone for __make_unsigned_selector_332 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_332 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_333 {}
impl Clone for __make_unsigned_selector_333 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_333 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_334 {}
impl Clone for __make_unsigned_selector_334 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_334 {
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
pub struct _List_338 {}
impl Clone for _List_338 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_338 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_339 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_340 {}
impl Clone for _List_340 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_340 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_341 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_342 {}
impl Clone for _List_342 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_342 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_343 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_344 {}
impl Clone for _List_344 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_344 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_345 {}
impl Clone for _List_345 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_345 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_346 {}
impl Clone for _List_346 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_346 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_347 {}
impl Clone for __select_347 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_347 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_348 {}
impl Clone for __select_348 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_348 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_349 {}
impl Clone for __select_349 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_349 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_350 {}
impl Clone for __select_350 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_350 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_351 {}
impl Clone for __select_351 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_351 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_352 {}
impl Clone for __make_signed_352 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_352 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_353 {}
impl Clone for __make_signed_353 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_353 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_354 {}
impl Clone for __make_signed_selector_354 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_354 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_355 {}
impl Clone for __make_signed_selector_355 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_355 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_356 {}
impl Clone for __make_signed_selector_356 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_356 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_357 {}
impl Clone for __make_signed_selector_357 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_357 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_358 {}
impl Clone for __make_signed_selector_358 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_358 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_pointer_359 {}
impl Clone for remove_pointer_359 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_pointer_359 {
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
pub struct common_type_360 {}
impl Clone for common_type_360 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_360 {
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
    pub static __is_nothrow_new_constructible_361: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __is_nothrow_new_constructible_362: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __is_nothrow_new_constructible_363: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __is_nothrow_new_constructible_364: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __is_nothrow_new_constructible_365: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __is_nothrow_new_constructible_366: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __is_nothrow_new_constructible_367: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __is_nothrow_new_constructible_368: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __is_nothrow_new_constructible_369: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __is_nothrow_new_constructible_370: Value<bool> = unimplemented!();
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
pub struct __are_same_371 {}
impl Clone for __are_same_371 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_371 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_372 = u32;
pub const anon_372___value: anon_372 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_373 {}
impl Clone for __are_same_373 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_373 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_374 = u32;
pub const anon_374___value: anon_374 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_375 {}
impl Clone for __are_same_375 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_375 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_376 = u32;
pub const anon_376___value: anon_376 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_377 {}
impl Clone for __are_same_377 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_377 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_378 = u32;
pub const anon_378___value: anon_378 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_379 {}
impl Clone for __are_same_379 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_379 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_380 = u32;
pub const anon_380___value: anon_380 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_381 {}
impl Clone for __are_same_381 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_381 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_382 = u32;
pub const anon_382___value: anon_382 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_383 {}
impl Clone for __is_integer_383 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_383 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_384 = u32;
pub const anon_384___value: anon_384 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_385 {}
impl Clone for __is_integer_385 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_385 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_386 = u32;
pub const anon_386___value: anon_386 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_387 {}
impl Clone for __is_integer_387 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_387 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_388 = u32;
pub const anon_388___value: anon_388 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_389 {}
impl Clone for __is_integer_389 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_389 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_390 = u32;
pub const anon_390___value: anon_390 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_391 {}
impl Clone for __is_integer_391 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_391 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_392 = u32;
pub const anon_392___value: anon_392 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nonvolatile_trivially_copyable_393 {}
impl Clone for __is_nonvolatile_trivially_copyable_393 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nonvolatile_trivially_copyable_393 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_394 = u32;
pub const anon_394___value: anon_394 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nonvolatile_trivially_copyable_395 {}
impl Clone for __is_nonvolatile_trivially_copyable_395 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nonvolatile_trivially_copyable_395 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_396 = u32;
pub const anon_396___value: anon_396 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_397 {}
impl Clone for __memcpyable_397 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_397 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_398 {}
impl Clone for __memcpyable_398 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_398 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_399 = u32;
pub const anon_399___value: anon_399 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_400 {}
impl Clone for __memcpyable_400 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_400 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_401 {}
impl Clone for __memcpyable_integer_401 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_401 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_move_iterator_402 {}
impl Clone for __is_move_iterator_402 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_iterator_402 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_403 = u32;
pub const anon_403___value: anon_403 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_404 {}
impl Clone for __is_integer_nonstrict_404 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_404 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_405 = u32;
pub const anon_405___width: anon_405 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_406 {}
impl Clone for __is_integer_nonstrict_406 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_406 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_407 = u32;
pub const anon_407___width: anon_407 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_408 {}
impl Clone for __numeric_traits_integer_408 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_408 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____is_signed_409: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____digits_410: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____max_411: Value<i64> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_412 {}
impl Clone for __numeric_traits_integer_412 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_412 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_413: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____digits_414: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____max_415: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_416 {}
impl Clone for __numeric_traits_floating_416 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_416 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_417 {}
impl Clone for __numeric_traits_floating_417 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_417 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_418 {}
impl Clone for __numeric_traits_floating_418 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_418 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_419 {}
impl Clone for __numeric_traits_419 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_419 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __move_if_noexcept_cond_420 {}
impl Clone for __move_if_noexcept_cond_420 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __move_if_noexcept_cond_420 {
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
    pub static piecewise_construct_421: Value<std_piecewise_construct_t> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_422 {}
impl Clone for tuple_422 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_422 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_422 {}
impl tuple_422 {}
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
pub struct iterator_423 {}
impl Clone for iterator_423 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_423 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_424 {}
impl Clone for iterator_424 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_424 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_424 {}
impl iterator_424 {}
impl iterator_424 {}
impl iterator_424 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_424>) -> Ptr<iterator_424> {
        unimplemented!()
    }
}
impl iterator_424 {
    pub fn move_assign(&self, _a0: Ptr<iterator_424>) -> Ptr<iterator_424> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_425 {}
impl Clone for iterator_425 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_425 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_425 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_425>) -> Ptr<iterator_425> {
        unimplemented!()
    }
}
impl iterator_425 {
    pub fn move_assign(&self, _a0: Ptr<iterator_425>) -> Ptr<iterator_425> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_426 {}
impl Clone for iterator_426 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_426 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_426 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_426>) -> Ptr<iterator_426> {
        unimplemented!()
    }
}
impl iterator_426 {
    pub fn move_assign(&self, _a0: Ptr<iterator_426>) -> Ptr<iterator_426> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_427 {}
impl Clone for iterator_traits_427 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_427 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_428 {}
impl Clone for iterator_traits_428 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_428 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_429 {}
impl Clone for iterator_traits_429 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_429 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_430 {}
impl Clone for iterator_traits_430 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_430 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_431 {}
impl Clone for iterator_traits_431 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_431 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_432 {}
impl Clone for iterator_traits_432 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_432 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_433 {}
impl Clone for iterator_traits_433 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_433 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_434 {}
impl Clone for iterator_traits_434 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_434 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_435 {}
impl Clone for iterator_traits_435 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_435 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_436 {}
impl Clone for iterator_traits_436 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_436 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_437 {}
impl Clone for iterator_traits_437 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_437 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_438 {}
impl Clone for iterator_traits_438 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_438 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_439 {}
impl Clone for iterator_traits_439 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_439 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_440 {}
impl Clone for iterator_traits_440 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_440 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_441 {}
impl Clone for iterator_traits_441 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_441 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_442 {}
impl Clone for __iterator_traits_442 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_442 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_443 {}
impl Clone for __iterator_traits_443 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_443 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_444 {}
impl Clone for __iterator_traits_444 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_444 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_445 {}
impl Clone for __iterator_traits_445 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_445 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_446 {}
impl Clone for __ptr_traits_ptr_to_446 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_446 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_447 {}
impl Clone for __ptr_traits_ptr_to_447 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_447 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_448 {}
impl Clone for __ptr_traits_ptr_to_448 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_448 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_449 {}
impl Clone for __ptr_traits_ptr_to_449 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_449 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_450 {}
impl Clone for pointer_traits_450 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_450 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_451 {}
impl Clone for pointer_traits_451 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_451 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_452 {}
impl Clone for pointer_traits_452 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_452 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_453 {}
impl Clone for pointer_traits_453 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_453 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_454 {}
impl Clone for reverse_iterator_454 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_454 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_454 {
    pub fn copy_from(__x: Ptr<reverse_iterator_454>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_454 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_454>) -> Ptr<reverse_iterator_454> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_455 {}
impl Clone for reverse_iterator_455 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_455 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_455 {
    pub fn copy_from(__x: Ptr<reverse_iterator_455>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_455 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_455>) -> Ptr<reverse_iterator_455> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_456 {}
impl Clone for __normal_iterator_456 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_456 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_456 {
    pub fn new_1(__i: Ptr<Ptr<vector_457>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_456 {}
impl __normal_iterator_456 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_458 {}
impl Clone for __normal_iterator_458 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_458 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_458 {
    pub fn new_1(__i: Ptr<Ptr<vector_457>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_458 {}
impl __normal_iterator_458 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_459 {}
impl Clone for __normal_iterator_459 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_459 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_459 {
    pub fn new_1(__i: Ptr<Ptr<u8>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_459 {}
impl __normal_iterator_459 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_460 {}
impl Clone for __normal_iterator_460 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_460 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_460 {
    pub fn new_1(__i: Ptr<Ptr<Chunk>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_460 {}
impl __normal_iterator_460 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_461 {}
impl Clone for __normal_iterator_461 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_461 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_461 {
    pub fn new_1(__i: Ptr<Ptr<Chunk>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_461 {}
impl __normal_iterator_461 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_462 {}
impl Clone for __normal_iterator_462 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_462 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_462 {
    pub fn new_1(__i: Ptr<Ptr<Pair>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_462 {}
impl __normal_iterator_462 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_463 {}
impl Clone for __normal_iterator_463 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_463 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_463 {
    pub fn new_1(__i: Ptr<Ptr<Pair>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_463 {}
impl __normal_iterator_463 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_464 {}
impl Clone for __normal_iterator_464 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_464 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_464 {
    pub fn new_1(__i: Ptr<Ptr<i64>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_464 {}
impl __normal_iterator_464 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_465 {}
impl Clone for __normal_iterator_465 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_465 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_465 {
    pub fn new_1(__i: Ptr<Ptr<i64>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_465 {}
impl __normal_iterator_465 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct move_iterator_466 {}
impl Clone for move_iterator_466 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for move_iterator_466 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl move_iterator_466 {
    pub fn new_1(__i: __normal_iterator_460) -> Self {
        unimplemented!()
    }
}
impl move_iterator_466 {}
impl move_iterator_466 {}
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct _Deque_iterator_467 {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_cur: Ptr<Pair>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_first: Ptr<Pair>,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_last: Ptr<Pair>,
    #[offset(24)]
    #[byte_size(8)]
    pub _M_node: Ptr<Ptr<Pair>>,
}
impl Clone for _Deque_iterator_467 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Deque_iterator_467 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Deque_iterator_467 {
    pub fn _S_buffer_size() -> usize {
        unimplemented!()
    }
}
impl _Deque_iterator_467 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Deque_iterator_467 {
    pub fn copy_from(__x: Ptr<_Deque_iterator_467>) -> Self {
        unimplemented!()
    }
}
impl _Deque_iterator_467 {
    pub fn copy_assign(&self, _a0: Ptr<_Deque_iterator_467>) -> Ptr<_Deque_iterator_467> {
        unimplemented!()
    }
}
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
pub struct __new_allocator_468 {}
impl Clone for __new_allocator_468 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_468 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_468 {
    pub fn copy_from(_a0: Ptr<__new_allocator_468>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_468 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_468>) -> Ptr<__new_allocator_468> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_469 {}
impl Clone for __new_allocator_469 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_469 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_469 {
    pub fn copy_from(_a0: Ptr<__new_allocator_469>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_469 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_469>) -> Ptr<__new_allocator_469> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_470 {}
impl Clone for __new_allocator_470 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_470 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_470 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_470 {
    pub fn copy_from(_a0: Ptr<__new_allocator_470>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_470 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_470>) -> Ptr<__new_allocator_470> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_474 {}
impl Clone for __new_allocator_474 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_474 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_474 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_474 {
    pub fn copy_from(_a0: Ptr<__new_allocator_474>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_474 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_474>) -> Ptr<__new_allocator_474> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_475 {}
impl Clone for __new_allocator_475 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_475 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_475 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_475 {
    pub fn copy_from(_a0: Ptr<__new_allocator_475>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_475 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_475>) -> Ptr<__new_allocator_475> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_478 {}
impl Clone for __new_allocator_478 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_478 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_478 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_478 {
    pub fn copy_from(_a0: Ptr<__new_allocator_478>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_478 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_478>) -> Ptr<__new_allocator_478> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_484 {}
impl Clone for __new_allocator_484 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_484 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_484 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_484 {
    pub fn copy_from(_a0: Ptr<__new_allocator_484>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_484 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_484>) -> Ptr<__new_allocator_484> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_485 {}
impl Clone for __new_allocator_485 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_485 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_485 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_485 {
    pub fn copy_from(_a0: Ptr<__new_allocator_485>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_485 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_485>) -> Ptr<__new_allocator_485> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_488 {}
impl Clone for allocator_488 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_488 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_488 {}
impl allocator_488 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_489 {}
impl Clone for allocator_489 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_489 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_489 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_489 {
    pub fn copy_from(__a: Ptr<allocator_489>) -> Self {
        unimplemented!()
    }
}
impl allocator_489 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_489>) -> Ptr<allocator_489> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_490 {}
impl Clone for allocator_490 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_490 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_490 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_490 {
    pub fn copy_from(__a: Ptr<allocator_490>) -> Self {
        unimplemented!()
    }
}
impl allocator_490 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_490>) -> Ptr<allocator_490> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_491 {}
impl Clone for allocator_491 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_491 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_491 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_491 {
    pub fn copy_from(__a: Ptr<allocator_491>) -> Self {
        unimplemented!()
    }
}
impl allocator_491 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_491>) -> Ptr<allocator_491> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_492 {}
impl Clone for allocator_492 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_492 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_492 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_492 {
    pub fn copy_from(__a: Ptr<allocator_492>) -> Self {
        unimplemented!()
    }
}
impl allocator_492 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_492>) -> Ptr<allocator_492> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_493 {}
impl Clone for allocator_493 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_493 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_493 {
    pub fn copy_from(__a: Ptr<allocator_493>) -> Self {
        unimplemented!()
    }
}
impl allocator_493 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_493>) -> Ptr<allocator_493> {
        unimplemented!()
    }
}
impl allocator_493 {
    pub fn new_1(_a0: Ptr<allocator_492>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_494 {}
impl Clone for allocator_494 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_494 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_494 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_494 {
    pub fn copy_from(__a: Ptr<allocator_494>) -> Self {
        unimplemented!()
    }
}
impl allocator_494 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_494>) -> Ptr<allocator_494> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_495 {}
impl Clone for allocator_traits_495 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_495 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_495 {
    pub fn allocate_1(__a: Ptr<allocator_491>, mut __n: u64) -> Ptr<vector_457> {
        unimplemented!()
    }
}
impl allocator_traits_495 {
    pub fn deallocate(__a: Ptr<allocator_491>, mut __p: Ptr<vector_457>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_495 {
    pub fn max_size(__a: Ptr<allocator_491>) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_495 {
    pub fn construct_496(
        __a: Ptr<allocator_491>,
        mut __p: Ptr<vector_457>,
        __args: Ptr<vector_457>,
    ) {
        unimplemented!()
    }
}
impl allocator_traits_495 {
    pub fn construct_497(
        __a: Ptr<allocator_491>,
        mut __p: Ptr<vector_457>,
        __args_2: Ptr<u8>,
        __args_3: Ptr<Ptr<u8>>,
    ) {
        unimplemented!()
    }
}
impl allocator_traits_495 {
    pub fn destroy_498(__a: Ptr<allocator_491>, mut __p: Ptr<vector_457>) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_499 {}
impl Clone for allocator_traits_499 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_499 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_499 {
    pub fn allocate_1(__a: Ptr<allocator_490>, mut __n: u64) -> Ptr<u8> {
        unimplemented!()
    }
}
impl allocator_traits_499 {
    pub fn deallocate(__a: Ptr<allocator_490>, mut __p: Ptr<u8>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_499 {
    pub fn max_size(__a: Ptr<allocator_490>) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_500 {}
impl Clone for allocator_traits_500 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_500 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_500 {
    pub fn allocate_1(__a: Ptr<allocator_489>, mut __n: u64) -> Ptr<Chunk> {
        unimplemented!()
    }
}
impl allocator_traits_500 {
    pub fn deallocate(__a: Ptr<allocator_489>, mut __p: Ptr<Chunk>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_500 {
    pub fn max_size(__a: Ptr<allocator_489>) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_500 {
    pub fn construct_501(__a: Ptr<allocator_489>, mut __p: Ptr<Chunk>, __args: Ptr<Chunk>) {
        unimplemented!()
    }
}
impl allocator_traits_500 {
    pub fn construct_502(__a: Ptr<allocator_489>, mut __p: Ptr<Chunk>, __args: Ptr<Chunk>) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_503 {}
impl Clone for allocator_traits_503 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_503 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_503 {
    pub fn allocate_1(__a: Ptr<allocator_492>, mut __n: u64) -> Ptr<Pair> {
        unimplemented!()
    }
}
impl allocator_traits_503 {
    pub fn deallocate(__a: Ptr<allocator_492>, mut __p: Ptr<Pair>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_503 {
    pub fn max_size(__a: Ptr<allocator_492>) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_503 {
    pub fn construct_504(__a: Ptr<allocator_492>, mut __p: Ptr<Pair>) {
        unimplemented!()
    }
}
impl allocator_traits_503 {
    pub fn construct_505(__a: Ptr<allocator_492>, mut __p: Ptr<Pair>, __args: Ptr<Pair>) {
        unimplemented!()
    }
}
impl allocator_traits_503 {
    pub fn construct_506(__a: Ptr<allocator_492>, mut __p: Ptr<Pair>, __args: Ptr<i32>) {
        unimplemented!()
    }
}
impl allocator_traits_503 {
    pub fn construct_507(
        __a: Ptr<allocator_492>,
        mut __p: Ptr<Pair>,
        __args_2: Ptr<i32>,
        __args_3: Ptr<i32>,
    ) {
        unimplemented!()
    }
}
impl allocator_traits_503 {
    pub fn destroy_508(__a: Ptr<allocator_492>, mut __p: Ptr<Pair>) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_509 {}
impl Clone for allocator_traits_509 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_509 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_509 {
    pub fn allocate_1(__a: Ptr<allocator_494>, mut __n: u64) -> Ptr<i64> {
        unimplemented!()
    }
}
impl allocator_traits_509 {
    pub fn deallocate(__a: Ptr<allocator_494>, mut __p: Ptr<i64>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_509 {
    pub fn max_size(__a: Ptr<allocator_494>) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_509 {
    pub fn construct_510(__a: Ptr<allocator_494>, mut __p: Ptr<i64>) {
        unimplemented!()
    }
}
impl allocator_traits_509 {
    pub fn construct_511(__a: Ptr<allocator_494>, mut __p: Ptr<i64>, __args: Ptr<i32>) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_512 {}
impl Clone for allocator_traits_512 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_512 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_512 {
    pub fn allocate_1(__a: Ptr<allocator_493>, mut __n: u64) -> Ptr<Ptr<Pair>> {
        unimplemented!()
    }
}
impl allocator_traits_512 {
    pub fn deallocate(__a: Ptr<allocator_493>, mut __p: Ptr<Ptr<Pair>>, mut __n: u64) {
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
pub struct __is_move_insertable_514 {}
impl Clone for __is_move_insertable_514 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_514 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_move_insertable_515 {}
impl Clone for __is_move_insertable_515 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_515 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_move_insertable_516 {}
impl Clone for __is_move_insertable_516 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_516 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_move_insertable_517 {}
impl Clone for __is_move_insertable_517 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_517 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __shrink_to_fit_aux_518 {}
impl Clone for __shrink_to_fit_aux_518 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __shrink_to_fit_aux_518 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __shrink_to_fit_aux_518 {
    pub fn _S_do_it(__c: Ptr<vector_519>) -> bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_520 {}
impl Clone for __alloc_traits_520 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_520 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_521 {}
impl Clone for rebind_521 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_521 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_522 {}
impl Clone for __alloc_traits_522 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_522 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_523 {}
impl Clone for rebind_523 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_523 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_524 {}
impl Clone for __alloc_traits_524 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_524 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_525 {}
impl Clone for rebind_525 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_525 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __alloc_traits_524 {
    pub fn _S_on_swap(__a: Ptr<allocator_489>, __b: Ptr<allocator_489>) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_526 {}
impl Clone for __alloc_traits_526 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_526 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_527 {}
impl Clone for rebind_527 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_527 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_528 {}
impl Clone for rebind_528 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_528 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_529 {}
impl Clone for __alloc_traits_529 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_529 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_530 {}
impl Clone for __alloc_traits_530 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_530 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_531 {}
impl Clone for rebind_531 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_531 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct _UninitDestroyGuard_532 {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_first: Ptr<Chunk>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_cur: Ptr<Ptr<Chunk>>,
}
impl Clone for _UninitDestroyGuard_532 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _UninitDestroyGuard_532 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_532 {
    pub fn new(__first: Ptr<Ptr<Chunk>>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_bitwise_relocatable_533 {}
impl Clone for __is_bitwise_relocatable_533 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_bitwise_relocatable_533 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_bitwise_relocatable_534 {}
impl Clone for __is_bitwise_relocatable_534 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_bitwise_relocatable_534 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_bitwise_relocatable_535 {}
impl Clone for __is_bitwise_relocatable_535 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_bitwise_relocatable_535 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_bitwise_relocatable_536 {}
impl Clone for __is_bitwise_relocatable_536 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_bitwise_relocatable_536 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_537 {}
impl Clone for initializer_list_537 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_537 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_538 {}
impl Clone for initializer_list_538 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_538 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_538 {}
impl initializer_list_538 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(80)]
pub struct _Deque_base_539 {}
impl Clone for _Deque_base_539 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Deque_base_539 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(80)]
pub struct std__Deque_base_Pair__std_allocator_Pair____Deque_impl_data {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_map: Ptr<Ptr<Pair>>,
    #[offset(8)]
    pub _M_map_size: usize,
    #[offset(16)]
    #[byte_size(32)]
    pub _M_start: _Deque_iterator_467,
    #[offset(48)]
    #[byte_size(32)]
    pub _M_finish: _Deque_iterator_467,
}
impl Clone for std__Deque_base_Pair__std_allocator_Pair____Deque_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Deque_base_Pair__std_allocator_Pair____Deque_impl_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Deque_base_Pair__std_allocator_Pair____Deque_impl_data {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std__Deque_base_Pair__std_allocator_Pair____Deque_impl_data {
    pub fn copy_from(
        _a0: Ptr<std__Deque_base_Pair__std_allocator_Pair____Deque_impl_data>,
    ) -> Self {
        unimplemented!()
    }
}
impl std__Deque_base_Pair__std_allocator_Pair____Deque_impl_data {
    pub fn copy_assign(
        &self,
        _a0: Ptr<std__Deque_base_Pair__std_allocator_Pair____Deque_impl_data>,
    ) -> Ptr<std__Deque_base_Pair__std_allocator_Pair____Deque_impl_data> {
        unimplemented!()
    }
}
impl std__Deque_base_Pair__std_allocator_Pair____Deque_impl_data {
    pub fn move_from(
        __x: Ptr<std__Deque_base_Pair__std_allocator_Pair____Deque_impl_data>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(80)]
pub struct std__Deque_base_Pair__std_allocator_Pair____Deque_impl {}
impl Clone for std__Deque_base_Pair__std_allocator_Pair____Deque_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Deque_base_Pair__std_allocator_Pair____Deque_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Deque_base_Pair__std_allocator_Pair____Deque_impl {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl std__Deque_base_Pair__std_allocator_Pair____Deque_impl {
    pub fn move_from(_a0: Ptr<std__Deque_base_Pair__std_allocator_Pair____Deque_impl>) -> Self {
        unimplemented!()
    }
}
pub type anon_540 = u32;
pub const anon_540__S_initial_map_size: anon_540 = 8;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(80)]
pub struct deque_541 {}
impl Clone for deque_541 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for deque_541 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl deque_541 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl deque_541 {
    pub fn copy_from(__x: Ptr<deque_541>) -> Self {
        unimplemented!()
    }
}
impl deque_541 {
    pub fn move_from(_a0: Ptr<deque_541>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct binary_function_544 {}
impl Clone for binary_function_544 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_544 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_545 {}
impl Clone for greater_545 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_545 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_546 {}
impl Clone for less_546 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_546 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_equal_547 {}
impl Clone for greater_equal_547 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_547 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_equal_548 {}
impl Clone for less_equal_548 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_548 {
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
    pub static allocator_arg_549: Value<std_allocator_arg_t> = unimplemented!();
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_550 { #[offset(0)] #[byte_size(24)] pub _M_impl : std__Vector_base_std_vector_unsigned_char___std_allocator_std_vector_unsigned_char_____Vector_impl  , }
impl Clone for _Vector_base_550 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_550 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_std_vector_unsigned_char___std_allocator_std_vector_unsigned_char_____Vector_impl_data
{
    #[offset(0)]
    #[byte_size(8)]
    pub _M_start: Ptr<vector_457>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_finish: Ptr<vector_457>,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_end_of_storage: Ptr<vector_457>,
}
impl Clone for std__Vector_base_std_vector_unsigned_char___std_allocator_std_vector_unsigned_char_____Vector_impl_data { fn clone(&self) -> Self { unimplemented!() } }
impl Default for std__Vector_base_std_vector_unsigned_char___std_allocator_std_vector_unsigned_char_____Vector_impl_data { fn default() -> Self { unimplemented!() } }
impl std__Vector_base_std_vector_unsigned_char___std_allocator_std_vector_unsigned_char_____Vector_impl_data { pub fn new ( ) -> Self { unimplemented!() } }
impl std__Vector_base_std_vector_unsigned_char___std_allocator_std_vector_unsigned_char_____Vector_impl_data { pub fn move_from ( __x : Ptr< std__Vector_base_std_vector_unsigned_char___std_allocator_std_vector_unsigned_char_____Vector_impl_data  > , ) -> Self { unimplemented!() } }
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_std_vector_unsigned_char___std_allocator_std_vector_unsigned_char_____Vector_impl
{}
impl Clone for std__Vector_base_std_vector_unsigned_char___std_allocator_std_vector_unsigned_char_____Vector_impl { fn clone(&self) -> Self { unimplemented!() } }
impl Default for std__Vector_base_std_vector_unsigned_char___std_allocator_std_vector_unsigned_char_____Vector_impl { fn default() -> Self { unimplemented!() } }
impl std__Vector_base_std_vector_unsigned_char___std_allocator_std_vector_unsigned_char_____Vector_impl { pub fn new_1 ( ) -> Self { unimplemented!() } }
impl std__Vector_base_std_vector_unsigned_char___std_allocator_std_vector_unsigned_char_____Vector_impl { pub fn move_from ( __x : Ptr< std__Vector_base_std_vector_unsigned_char___std_allocator_std_vector_unsigned_char_____Vector_impl  > , ) -> Self { unimplemented!() } }
impl _Vector_base_550 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_550 {
    pub fn move_from(_a0: Ptr<_Vector_base_550>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_551 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl,
}
impl Clone for _Vector_base_551 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_551 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl_data {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_start: Ptr<u8>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_finish: Ptr<u8>,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_end_of_storage: Ptr<u8>,
}
impl Clone for std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl_data {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl_data {
    pub fn move_from(
        __x: Ptr<std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl_data>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl {}
impl Clone for std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl {
    pub fn new_2(__a: Ptr<allocator_490>) -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl {
    pub fn move_from(
        __x: Ptr<std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl>,
    ) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_551 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_551 {
    pub fn new_2(__a: Ptr<allocator_490>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_551 {
    pub fn move_from(_a0: Ptr<_Vector_base_551>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_552 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_Chunk__std_allocator_Chunk____Vector_impl,
}
impl Clone for _Vector_base_552 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_552 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_Chunk__std_allocator_Chunk____Vector_impl_data {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_start: Ptr<Chunk>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_finish: Ptr<Chunk>,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_end_of_storage: Ptr<Chunk>,
}
impl Clone for std__Vector_base_Chunk__std_allocator_Chunk____Vector_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_Chunk__std_allocator_Chunk____Vector_impl_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Chunk__std_allocator_Chunk____Vector_impl_data {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Chunk__std_allocator_Chunk____Vector_impl_data {
    pub fn move_from(
        __x: Ptr<std__Vector_base_Chunk__std_allocator_Chunk____Vector_impl_data>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_Chunk__std_allocator_Chunk____Vector_impl {}
impl Clone for std__Vector_base_Chunk__std_allocator_Chunk____Vector_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_Chunk__std_allocator_Chunk____Vector_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Chunk__std_allocator_Chunk____Vector_impl {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Chunk__std_allocator_Chunk____Vector_impl {
    pub fn new_2(__a: Ptr<allocator_489>) -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Chunk__std_allocator_Chunk____Vector_impl {
    pub fn move_from(__x: Ptr<std__Vector_base_Chunk__std_allocator_Chunk____Vector_impl>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_552 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_552 {
    pub fn new_2(__a: Ptr<allocator_489>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_552 {
    pub fn move_from(_a0: Ptr<_Vector_base_552>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_553 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_Pair__std_allocator_Pair____Vector_impl,
}
impl Clone for _Vector_base_553 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_553 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_Pair__std_allocator_Pair____Vector_impl_data {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_start: Ptr<Pair>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_finish: Ptr<Pair>,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_end_of_storage: Ptr<Pair>,
}
impl Clone for std__Vector_base_Pair__std_allocator_Pair____Vector_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_Pair__std_allocator_Pair____Vector_impl_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Pair__std_allocator_Pair____Vector_impl_data {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Pair__std_allocator_Pair____Vector_impl_data {
    pub fn move_from(
        __x: Ptr<std__Vector_base_Pair__std_allocator_Pair____Vector_impl_data>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_Pair__std_allocator_Pair____Vector_impl {}
impl Clone for std__Vector_base_Pair__std_allocator_Pair____Vector_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_Pair__std_allocator_Pair____Vector_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Pair__std_allocator_Pair____Vector_impl {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Pair__std_allocator_Pair____Vector_impl {
    pub fn move_from(__x: Ptr<std__Vector_base_Pair__std_allocator_Pair____Vector_impl>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_553 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_553 {
    pub fn move_from(_a0: Ptr<_Vector_base_553>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_554 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_long__std_allocator_long____Vector_impl,
}
impl Clone for _Vector_base_554 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_554 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_long__std_allocator_long____Vector_impl_data {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_start: Ptr<i64>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_finish: Ptr<i64>,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_end_of_storage: Ptr<i64>,
}
impl Clone for std__Vector_base_long__std_allocator_long____Vector_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_long__std_allocator_long____Vector_impl_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_long__std_allocator_long____Vector_impl_data {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_long__std_allocator_long____Vector_impl_data {
    pub fn move_from(
        __x: Ptr<std__Vector_base_long__std_allocator_long____Vector_impl_data>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_long__std_allocator_long____Vector_impl {}
impl Clone for std__Vector_base_long__std_allocator_long____Vector_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_long__std_allocator_long____Vector_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_long__std_allocator_long____Vector_impl {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_long__std_allocator_long____Vector_impl {
    pub fn move_from(__x: Ptr<std__Vector_base_long__std_allocator_long____Vector_impl>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_554 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_554 {
    pub fn move_from(_a0: Ptr<_Vector_base_554>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_519 {}
impl Clone for vector_519 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_519 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std_vector_Chunk___Guard_alloc {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_storage: Ptr<Chunk>,
    #[offset(8)]
    pub _M_len: u64,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_vect: Ptr<_Vector_base_552>,
}
impl Clone for std_vector_Chunk___Guard_alloc {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_vector_Chunk___Guard_alloc {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_vector_Chunk___Guard_alloc {
    pub fn new(mut __s: Ptr<Chunk>, mut __l: u64, __vect: Ptr<_Vector_base_552>) -> Self {
        unimplemented!()
    }
}
impl vector_519 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_519 {
    pub fn copy_from(__x: Ptr<vector_519>) -> Self {
        unimplemented!()
    }
}
impl vector_519 {
    pub fn move_from(_a0: Ptr<vector_519>) -> Self {
        unimplemented!()
    }
}
impl vector_519 {
    pub fn new_2(
        __first: move_iterator_466,
        __last: move_iterator_466,
        __a: Ptr<allocator_489>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_457 {}
impl Clone for vector_457 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_457 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl vector_457 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_457 {
    pub fn copy_from(__x: Ptr<vector_457>) -> Self {
        unimplemented!()
    }
}
impl vector_457 {
    pub fn move_from(_a0: Ptr<vector_457>) -> Self {
        unimplemented!()
    }
}
impl vector_457 {
    pub fn new_2(__first: Ptr<u8>, mut __last: Ptr<u8>, __a: Option<Ptr<allocator_490>>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_556 {}
impl Clone for vector_556 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_556 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std_vector_std_vector_unsigned_char____Guard_alloc {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_storage: Ptr<vector_457>,
    #[offset(8)]
    pub _M_len: u64,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_vect: Ptr<_Vector_base_550>,
}
impl Clone for std_vector_std_vector_unsigned_char____Guard_alloc {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_vector_std_vector_unsigned_char____Guard_alloc {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_vector_std_vector_unsigned_char____Guard_alloc {
    pub fn new(mut __s: Ptr<vector_457>, mut __l: u64, __vect: Ptr<_Vector_base_550>) -> Self {
        unimplemented!()
    }
}
impl vector_556 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_556 {
    pub fn copy_from(__x: Ptr<vector_556>) -> Self {
        unimplemented!()
    }
}
impl vector_556 {
    pub fn move_from(_a0: Ptr<vector_556>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_559 {}
impl Clone for vector_559 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_559 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std_vector_Pair___Guard_alloc {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_storage: Ptr<Pair>,
    #[offset(8)]
    pub _M_len: u64,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_vect: Ptr<_Vector_base_553>,
}
impl Clone for std_vector_Pair___Guard_alloc {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_vector_Pair___Guard_alloc {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_vector_Pair___Guard_alloc {
    pub fn new(mut __s: Ptr<Pair>, mut __l: u64, __vect: Ptr<_Vector_base_553>) -> Self {
        unimplemented!()
    }
}
impl vector_559 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_559 {
    pub fn copy_from(__x: Ptr<vector_559>) -> Self {
        unimplemented!()
    }
}
impl vector_559 {
    pub fn move_from(_a0: Ptr<vector_559>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_563 {}
impl Clone for vector_563 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_563 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std_vector_long___Guard_alloc {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_storage: Ptr<i64>,
    #[offset(8)]
    pub _M_len: u64,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_vect: Ptr<_Vector_base_554>,
}
impl Clone for std_vector_long___Guard_alloc {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_vector_long___Guard_alloc {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_vector_long___Guard_alloc {
    pub fn new(mut __s: Ptr<i64>, mut __l: u64, __vect: Ptr<_Vector_base_554>) -> Self {
        unimplemented!()
    }
}
impl vector_563 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_563 {
    pub fn copy_from(__x: Ptr<vector_563>) -> Self {
        unimplemented!()
    }
}
impl vector_563 {
    pub fn move_from(_a0: Ptr<vector_563>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_566 {}
impl Clone for __hash_base_566 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_566 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_567 {}
impl Clone for __hash_base_567 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_567 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_568 {}
impl Clone for __hash_base_568 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_568 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_569 {}
impl Clone for __hash_base_569 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_569 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_570 {}
impl Clone for __hash_base_570 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_570 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_571 {}
impl Clone for __hash_base_571 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_571 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_572 {}
impl Clone for __hash_base_572 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_572 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_573 {}
impl Clone for __hash_base_573 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_573 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_574 {}
impl Clone for __hash_base_574 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_574 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_575 {}
impl Clone for __hash_base_575 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_575 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_576 {}
impl Clone for __hash_base_576 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_576 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_577 {}
impl Clone for __hash_base_577 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_577 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_578 {}
impl Clone for __hash_base_578 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_578 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_579 {}
impl Clone for __hash_base_579 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_579 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_580 {}
impl Clone for __hash_base_580 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_580 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_581 {}
impl Clone for __hash_base_581 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_581 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_582 {}
impl Clone for __hash_base_582 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_582 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_583 {}
impl Clone for __hash_base_583 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_583 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_584 {}
impl Clone for __hash_base_584 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_584 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_585 {}
impl Clone for __hash_base_585 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_585 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_586 {}
impl Clone for __hash_base_586 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_586 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_587 {}
impl Clone for hash_587 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_587 {
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
    pub fn hash_588(__val: Ptr<f32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_589(__val: Ptr<f64>) -> usize {
        unimplemented!()
    }
}
pub type anon_590 = u32;
pub const anon_590__S_word_bit: anon_590 = 64;
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Chunk {
    #[offset(0)]
    pub data: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Writer {
    #[offset(0)]
    #[byte_size(8)]
    pub output: Ptr<vector_519>,
    #[offset(8)]
    #[byte_size(4)]
    pub chunk: Chunk,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(48)]
pub struct JPEGData {
    #[offset(0)]
    #[byte_size(24)]
    pub com_data: vector_556,
    #[offset(24)]
    #[byte_size(24)]
    pub app_data: vector_556,
}
pub fn push_param_591(mut dest: Ptr<vector_556>) {
    ({
        let ___x: Value<vector_457> = Rc::new(RefCell::new(<vector_457>::default()));
        vector_556Impl::push_back_58(&dest, ___x.as_pointer())
    });
}
pub fn push_local_from_field_592(mut jpg: Ptr<JPEGData>, mut cond: bool) {
    let head: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([1_u8, 2_u8, 3_u8])));
    let mut dest: Ptr<vector_556> = Ptr::<vector_556>::null();
    if cond {
        dest = (field_ptr!(jpg, com_data));
    } else {
        dest = (field_ptr!(jpg, app_data));
    }
    ({
        let ___x: Value<vector_457> = Rc::new(RefCell::new(vector_457::new_2(
            { (head.as_pointer() as Ptr<u8>) },
            { (head.as_pointer() as Ptr<u8>).offset((3) as isize) },
            None,
        )));
        vector_556Impl::push_back_58(&dest, ___x.as_pointer())
    });
}
pub fn shrink_through_ptr_593(mut comps: Ptr<vector_519>) {
    ({ vector_519Impl::shrink_to_fit(&comps) });
}
pub fn nested_push_move_594(mut bw: Ptr<Writer>) {
    ({
        let ___x: Ptr<Chunk> = field_ptr!(bw, chunk);
        vector_519Impl::push_back_58(&bw.with(|__s| __s.output.clone()), ___x)
    });
}
pub fn emplace_local_from_field_595(mut jpg: Ptr<JPEGData>, mut cond: bool) {
    let head: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([1_u8, 2_u8, 3_u8])));
    let mut dest: Ptr<vector_556> = Ptr::<vector_556>::null();
    if cond {
        dest = (field_ptr!(jpg, com_data));
    } else {
        dest = (field_ptr!(jpg, app_data));
    }
    ({
        let ___args_0: Ptr<u8> = head.as_pointer();
        let ___args_1: Value<Ptr<u8>> = Rc::new(RefCell::new(
            (head.as_pointer() as Ptr<u8>).offset((3) as isize),
        ));
        vector_556Impl::emplace_back_557(&dest, ___args_0, ___args_1.as_pointer())
    });
}
pub fn nested_emplace_move_596(mut bw: Ptr<Writer>) {
    ({
        let ___args: Ptr<Chunk> = field_ptr!(bw, chunk);
        vector_519Impl::emplace_back_555(&bw.with(|__s| __s.output.clone()), ___args)
    });
}
pub fn self_ref_push_597(mut comps: Ptr<vector_519>) {
    ({
        let ___x: Ptr<Chunk> = ({ vector_519Impl::front_51(&comps) });
        vector_519Impl::push_back_57(&comps, ___x)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Pair {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
impl Pair {
    pub fn new_1() -> Self {
        Self {
            first: -1_i32,
            second: -1_i32,
        }
    }
    pub fn new_2(mut a: i32) -> Self {
        Self {
            first: a,
            second: 0,
        }
    }
    pub fn new_3(mut a: i32, mut b: i32) -> Self {
        Self {
            first: a,
            second: (b * 2),
        }
    }
}
impl Default for Pair {
    fn default() -> Self {
        { Pair::new_1() }
    }
}
pub fn emplace_ctor_args_598(mut pairs: Ptr<vector_559>) {
    ({ vector_559Impl::emplace_back_560(&pairs) });
    ({
        let ___args: Value<i32> = Rc::new(RefCell::new(3));
        vector_559Impl::emplace_back_561(&pairs, ___args.as_pointer())
    });
    ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(4));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(5));
        vector_559Impl::emplace_back_562(&pairs, ___args_0.as_pointer(), ___args_1.as_pointer())
    });
}
pub fn emplace_deque_599(mut queue: Ptr<deque_541>) {
    ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(6));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(7));
        deque_541Impl::emplace_back_542(&queue, ___args_0.as_pointer(), ___args_1.as_pointer())
    });
    ({ deque_541Impl::emplace_back_543(&queue) });
}
pub fn emplace_scalar_600(mut values: Ptr<vector_563>, x: i32) {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    ({ vector_563Impl::emplace_back_564(&values) });
    ({ vector_563Impl::emplace_back_565(&values, x.as_pointer()) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let vecs: Value<vector_556> = Rc::new(RefCell::new(<vector_556>::default()));
    ({ push_param_591((vecs.as_pointer())) });
    (if (({ vector_556Impl::size(&vecs.as_pointer()) }) == 1_u64) {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(b"vecs.size() == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        vector_457Impl::empty(&({ vector_556Impl::operator_index_46(&vecs.as_pointer(), 0_u64) }))
    }) {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(b"vecs[0].empty()"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let jpg: Value<JPEGData> = Rc::new(RefCell::new(<JPEGData>::default()));
    ({ push_local_from_field_592((jpg.as_pointer()), true) });
    (if (({ vector_556Impl::size(&field_ptr!(jpg.as_pointer(), com_data)) }) == 1_u64) {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(b"jpg.com_data.size() == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        vector_457Impl::size(
            &({
                vector_556Impl::operator_index_46(&field_ptr!(jpg.as_pointer(), com_data), 0_u64)
            }),
        )
    }) == 3_u64)
    {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(b"jpg.com_data[0].size() == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({
        vector_457Impl::operator_index_46(
            &({
                vector_556Impl::operator_index_46(&field_ptr!(jpg.as_pointer(), com_data), 0_u64)
            }),
            0_u64,
        )
    })
    .read()) as i32)
        == 1)
    {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(b"jpg.com_data[0][0] == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({
        vector_457Impl::operator_index_46(
            &({
                vector_556Impl::operator_index_46(&field_ptr!(jpg.as_pointer(), com_data), 0_u64)
            }),
            1_u64,
        )
    })
    .read()) as i32)
        == 2)
    {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(b"jpg.com_data[0][1] == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({
        vector_457Impl::operator_index_46(
            &({
                vector_556Impl::operator_index_46(&field_ptr!(jpg.as_pointer(), com_data), 0_u64)
            }),
            2_u64,
        )
    })
    .read()) as i32)
        == 3)
    {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(b"jpg.com_data[0][2] == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ vector_556Impl::empty(&field_ptr!(jpg.as_pointer(), app_data)) }) {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(b"jpg.app_data.empty()"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let chunks: Value<vector_519> = Rc::new(RefCell::new(<vector_519>::default()));
    ({ shrink_through_ptr_593((chunks.as_pointer())) });
    (if ({ vector_519Impl::empty(&chunks.as_pointer()) }) {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(b"chunks.empty()"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let w: Value<Writer> = Rc::new(RefCell::new(<Writer>::default()));
    (*w.borrow_mut()).chunk.data = 42;
    (*w.borrow_mut()).output = (chunks.as_pointer());
    ({ nested_push_move_594((w.as_pointer())) });
    (if (({ vector_519Impl::size(&chunks.as_pointer()) }) == 1_u64) {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(b"chunks.size() == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ vector_519Impl::operator_index_46(&chunks.as_pointer(), 0_u64) }).with(|__s| __s.data)
        == 42)
    {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(b"chunks[0].data == 42"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ emplace_local_from_field_595((jpg.as_pointer()), false) });
    (if (({ vector_556Impl::size(&field_ptr!(jpg.as_pointer(), app_data)) }) == 1_u64) {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(b"jpg.app_data.size() == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        vector_457Impl::size(
            &({
                vector_556Impl::operator_index_46(&field_ptr!(jpg.as_pointer(), app_data), 0_u64)
            }),
        )
    }) == 3_u64)
    {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(b"jpg.app_data[0].size() == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({
        vector_457Impl::operator_index_46(
            &({
                vector_556Impl::operator_index_46(&field_ptr!(jpg.as_pointer(), app_data), 0_u64)
            }),
            0_u64,
        )
    })
    .read()) as i32)
        == 1)
    {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(b"jpg.app_data[0][0] == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({
        vector_457Impl::operator_index_46(
            &({
                vector_556Impl::operator_index_46(&field_ptr!(jpg.as_pointer(), app_data), 0_u64)
            }),
            2_u64,
        )
    })
    .read()) as i32)
        == 3)
    {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(b"jpg.app_data[0][2] == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ vector_556Impl::size(&field_ptr!(jpg.as_pointer(), com_data)) }) == 1_u64) {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(b"jpg.com_data.size() == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (*w.borrow_mut()).chunk.data = 99;
    (*w.borrow_mut()).output = (chunks.as_pointer());
    ({ nested_emplace_move_596((w.as_pointer())) });
    (if (({ vector_519Impl::size(&chunks.as_pointer()) }) == 2_u64) {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(b"chunks.size() == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ vector_519Impl::operator_index_46(&chunks.as_pointer(), 1_u64) }).with(|__s| __s.data)
        == 99)
    {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(b"chunks[1].data == 99"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ self_ref_push_597((chunks.as_pointer())) });
    (if (({ vector_519Impl::size(&chunks.as_pointer()) }) == 3_u64) {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(b"chunks.size() == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ vector_519Impl::operator_index_46(&chunks.as_pointer(), 2_u64) }).with(|__s| __s.data)
        == 42)
    {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(b"chunks[2].data == 42"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let pairs: Value<vector_559> = Rc::new(RefCell::new(<vector_559>::default()));
    ({ emplace_ctor_args_598((pairs.as_pointer())) });
    (if (({ vector_559Impl::size(&pairs.as_pointer()) }) == 3_u64) {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(b"pairs.size() == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ vector_559Impl::operator_index_46(&pairs.as_pointer(), 0_u64) }).with(|__s| __s.first)
        == -1_i32)
        && (({ vector_559Impl::operator_index_46(&pairs.as_pointer(), 0_u64) })
            .with(|__s| __s.second)
            == -1_i32)
    {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(b"pairs[0].first == -1 && pairs[0].second == -1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ vector_559Impl::operator_index_46(&pairs.as_pointer(), 1_u64) }).with(|__s| __s.first)
        == 3)
        && (({ vector_559Impl::operator_index_46(&pairs.as_pointer(), 1_u64) })
            .with(|__s| __s.second)
            == 0)
    {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(b"pairs[1].first == 3 && pairs[1].second == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ vector_559Impl::operator_index_46(&pairs.as_pointer(), 2_u64) }).with(|__s| __s.first)
        == 4)
        && (({ vector_559Impl::operator_index_46(&pairs.as_pointer(), 2_u64) })
            .with(|__s| __s.second)
            == 10)
    {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(b"pairs[2].first == 4 && pairs[2].second == 10"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let queue: Value<deque_541> = Rc::new(RefCell::new(<deque_541>::default()));
    ({ emplace_deque_599((queue.as_pointer())) });
    (if (({ deque_541Impl::front_45(&queue.as_pointer()) }).with(|__s| __s.first) == 6)
        && (({ deque_541Impl::front_45(&queue.as_pointer()) }).with(|__s| __s.second) == 14)
    {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(
                    b"queue.front().first == 6 && queue.front().second == 14",
                ),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ deque_541Impl::back_47(&queue.as_pointer()) }).with(|__s| __s.first) == -1_i32)
        && (({ deque_541Impl::back_47(&queue.as_pointer()) }).with(|__s| __s.second) == -1_i32)
    {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(
                    b"queue.back().first == -1 && queue.back().second == -1",
                ),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let values: Value<vector_563> = Rc::new(RefCell::new(<vector_563>::default()));
    ({ emplace_scalar_600((values.as_pointer()), 7) });
    (if (({ vector_563Impl::size(&values.as_pointer()) }) == 2_u64) {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(b"values.size() == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((({ vector_563Impl::operator_index_46(&values.as_pointer(), 0_u64) }).read()) == 0_i64) {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(b"values[0] == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((({ vector_563Impl::operator_index_46(&values.as_pointer(), 1_u64) }).read()) == 7_i64) {
        &(0);
    } else {
        ({
            __assert_fail_601(
                Ptr::<i8>::from_string_literal(b"values[1] == 7"),
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
pub fn __assert_fail_601(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub trait _Deque_iterator_467Impl {
    fn operator_deref(&self) -> Ptr<Pair> {
        unimplemented!()
    }
    fn operator_dec_12(&self) -> Ptr<_Deque_iterator_467> {
        unimplemented!()
    }
    fn _M_set_node(&self, mut __new_node: Ptr<Ptr<Pair>>) {
        unimplemented!()
    }
}
impl _Deque_iterator_467Impl for Ptr<_Deque_iterator_467> {}
pub trait _UninitDestroyGuard_532Impl {
    fn release(&self) {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_532Impl for Ptr<_UninitDestroyGuard_532> {}
pub trait _Vector_base_550Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_491> {
        unimplemented!()
    }
    fn _M_get_Tp_allocator_2(&self) -> Ptr<allocator_491> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<vector_457> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<vector_457>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_550Impl for Ptr<_Vector_base_550> {}
pub trait _Vector_base_551Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_490> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<u8> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<u8>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_551Impl for Ptr<_Vector_base_551> {}
pub trait _Vector_base_552Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_489> {
        unimplemented!()
    }
    fn _M_get_Tp_allocator_2(&self) -> Ptr<allocator_489> {
        unimplemented!()
    }
    fn get_allocator(&self) -> allocator_489 {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<Chunk> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<Chunk>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_552Impl for Ptr<_Vector_base_552> {}
pub trait _Vector_base_553Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_492> {
        unimplemented!()
    }
    fn _M_get_Tp_allocator_2(&self) -> Ptr<allocator_492> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<Pair> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<Pair>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_553Impl for Ptr<_Vector_base_553> {}
pub trait _Vector_base_554Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_494> {
        unimplemented!()
    }
    fn _M_get_Tp_allocator_2(&self) -> Ptr<allocator_494> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<i64> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<i64>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_554Impl for Ptr<_Vector_base_554> {}
pub trait __new_allocator_470Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<vector_457> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<vector_457>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn construct_471(&self, mut __p: Ptr<vector_457>, __args: Ptr<vector_457>) {
        unimplemented!()
    }
    fn construct_472(&self, mut __p: Ptr<vector_457>, __args_1: Ptr<u8>, __args_2: Ptr<Ptr<u8>>) {
        unimplemented!()
    }
    fn destroy_473(&self, mut __p: Ptr<vector_457>) {
        unimplemented!()
    }
}
impl __new_allocator_470Impl for Ptr<__new_allocator_470> {}
pub trait __new_allocator_474Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<u8> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<u8>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl __new_allocator_474Impl for Ptr<__new_allocator_474> {}
pub trait __new_allocator_475Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<Chunk> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<Chunk>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn construct_476(&self, mut __p: Ptr<Chunk>, __args: Ptr<Chunk>) {
        unimplemented!()
    }
    fn construct_477(&self, mut __p: Ptr<Chunk>, __args: Ptr<Chunk>) {
        unimplemented!()
    }
}
impl __new_allocator_475Impl for Ptr<__new_allocator_475> {}
pub trait __new_allocator_478Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<Pair> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<Pair>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn construct_479(&self, mut __p: Ptr<Pair>) {
        unimplemented!()
    }
    fn construct_480(&self, mut __p: Ptr<Pair>, __args: Ptr<Pair>) {
        unimplemented!()
    }
    fn construct_481(&self, mut __p: Ptr<Pair>, __args: Ptr<i32>) {
        unimplemented!()
    }
    fn construct_482(&self, mut __p: Ptr<Pair>, __args_1: Ptr<i32>, __args_2: Ptr<i32>) {
        unimplemented!()
    }
    fn destroy_483(&self, mut __p: Ptr<Pair>) {
        unimplemented!()
    }
}
impl __new_allocator_478Impl for Ptr<__new_allocator_478> {}
pub trait __new_allocator_484Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<Ptr<Pair>> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<Ptr<Pair>>, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_484Impl for Ptr<__new_allocator_484> {}
pub trait __new_allocator_485Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<i64> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<i64>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn construct_486(&self, mut __p: Ptr<i64>) {
        unimplemented!()
    }
    fn construct_487(&self, mut __p: Ptr<i64>, __args: Ptr<i32>) {
        unimplemented!()
    }
}
impl __new_allocator_485Impl for Ptr<__new_allocator_485> {}
pub trait __normal_iterator_456Impl {
    fn operator_deref(&self) -> Ptr<vector_457> {
        unimplemented!()
    }
    fn operator_sub(&self, mut __n: i64) -> __normal_iterator_456 {
        unimplemented!()
    }
    fn base(&self) -> Ptr<Ptr<vector_457>> {
        unimplemented!()
    }
}
impl __normal_iterator_456Impl for Ptr<__normal_iterator_456> {}
pub trait __normal_iterator_458Impl {
    fn base(&self) -> Ptr<Ptr<vector_457>> {
        unimplemented!()
    }
}
impl __normal_iterator_458Impl for Ptr<__normal_iterator_458> {}
pub trait __normal_iterator_459Impl {
    fn base(&self) -> Ptr<Ptr<u8>> {
        unimplemented!()
    }
}
impl __normal_iterator_459Impl for Ptr<__normal_iterator_459> {}
pub trait __normal_iterator_460Impl {
    fn operator_deref(&self) -> Ptr<Chunk> {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<__normal_iterator_460> {
        unimplemented!()
    }
    fn operator_sub(&self, mut __n: i64) -> __normal_iterator_460 {
        unimplemented!()
    }
    fn base(&self) -> Ptr<Ptr<Chunk>> {
        unimplemented!()
    }
}
impl __normal_iterator_460Impl for Ptr<__normal_iterator_460> {}
pub trait __normal_iterator_461Impl {
    fn base(&self) -> Ptr<Ptr<Chunk>> {
        unimplemented!()
    }
}
impl __normal_iterator_461Impl for Ptr<__normal_iterator_461> {}
pub trait __normal_iterator_462Impl {
    fn operator_deref(&self) -> Ptr<Pair> {
        unimplemented!()
    }
    fn operator_sub(&self, mut __n: i64) -> __normal_iterator_462 {
        unimplemented!()
    }
    fn base(&self) -> Ptr<Ptr<Pair>> {
        unimplemented!()
    }
}
impl __normal_iterator_462Impl for Ptr<__normal_iterator_462> {}
pub trait __normal_iterator_463Impl {
    fn base(&self) -> Ptr<Ptr<Pair>> {
        unimplemented!()
    }
}
impl __normal_iterator_463Impl for Ptr<__normal_iterator_463> {}
pub trait __normal_iterator_464Impl {
    fn operator_deref(&self) -> Ptr<i64> {
        unimplemented!()
    }
    fn operator_sub(&self, mut __n: i64) -> __normal_iterator_464 {
        unimplemented!()
    }
    fn base(&self) -> Ptr<Ptr<i64>> {
        unimplemented!()
    }
}
impl __normal_iterator_464Impl for Ptr<__normal_iterator_464> {}
pub trait __normal_iterator_465Impl {
    fn base(&self) -> Ptr<Ptr<i64>> {
        unimplemented!()
    }
}
impl __normal_iterator_465Impl for Ptr<__normal_iterator_465> {}
pub trait deque_541Impl {
    fn copy_assign(&self, __x: Ptr<deque_541>) -> Ptr<deque_541> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<deque_541>) -> Ptr<deque_541> {
        unimplemented!()
    }
    fn begin_22(&self) -> _Deque_iterator_467 {
        unimplemented!()
    }
    fn end_24(&self) -> _Deque_iterator_467 {
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
    fn front_45(&self) -> Ptr<Pair> {
        unimplemented!()
    }
    fn back_47(&self) -> Ptr<Pair> {
        unimplemented!()
    }
    fn emplace_back_542(&self, __args_0: Ptr<i32>, __args_1: Ptr<i32>) -> Ptr<Pair> {
        unimplemented!()
    }
    fn emplace_back_543(&self) -> Ptr<Pair> {
        unimplemented!()
    }
}
impl deque_541Impl for Ptr<deque_541> {}
pub trait initializer_list_537Impl {
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn begin(&self) -> Ptr<bool> {
        unimplemented!()
    }
}
impl initializer_list_537Impl for Ptr<initializer_list_537> {}
pub trait integral_constant_0Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl integral_constant_0Impl for Ptr<integral_constant_0> {}
pub trait move_iterator_466Impl {
    fn base(&self) -> __normal_iterator_460 {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<Chunk> {
        unimplemented!()
    }
    fn operator_inc_8(&self) -> Ptr<move_iterator_466> {
        unimplemented!()
    }
}
impl move_iterator_466Impl for Ptr<move_iterator_466> {}
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
pub trait std__Vector_base_Chunk__std_allocator_Chunk____Vector_impl_dataImpl {
    fn _M_copy_data(
        &self,
        __x: Ptr<std__Vector_base_Chunk__std_allocator_Chunk____Vector_impl_data>,
    ) {
        unimplemented!()
    }
    fn _M_swap_data(
        &self,
        __x: Ptr<std__Vector_base_Chunk__std_allocator_Chunk____Vector_impl_data>,
    ) {
        unimplemented!()
    }
}
impl std__Vector_base_Chunk__std_allocator_Chunk____Vector_impl_dataImpl
    for Ptr<std__Vector_base_Chunk__std_allocator_Chunk____Vector_impl_data>
{
}
pub trait vector_457Impl {
    fn copy_assign(&self, __x: Ptr<vector_457>) -> Ptr<vector_457> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_457>) -> Ptr<vector_457> {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_459 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_459 {
        unimplemented!()
    }
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn empty(&self) -> bool {
        unimplemented!()
    }
    fn operator_index_46(&self, mut __n: u64) -> Ptr<u8> {
        unimplemented!()
    }
}
impl vector_457Impl for Ptr<vector_457> {}
pub trait vector_519Impl {
    fn copy_assign(&self, __x: Ptr<vector_519>) -> Ptr<vector_519> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_519>) -> Ptr<vector_519> {
        unimplemented!()
    }
    fn begin_26(&self) -> __normal_iterator_460 {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_461 {
        unimplemented!()
    }
    fn end_28(&self) -> __normal_iterator_460 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_461 {
        unimplemented!()
    }
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn shrink_to_fit(&self) {
        unimplemented!()
    }
    fn capacity(&self) -> u64 {
        unimplemented!()
    }
    fn empty(&self) -> bool {
        unimplemented!()
    }
    fn operator_index_46(&self, mut __n: u64) -> Ptr<Chunk> {
        unimplemented!()
    }
    fn front_51(&self) -> Ptr<Chunk> {
        unimplemented!()
    }
    fn back_53(&self) -> Ptr<Chunk> {
        unimplemented!()
    }
    fn push_back_57(&self, __x: Ptr<Chunk>) {
        unimplemented!()
    }
    fn push_back_58(&self, __x: Ptr<Chunk>) {
        unimplemented!()
    }
    fn swap(&self, __x: Ptr<vector_519>) {
        unimplemented!()
    }
    fn emplace_back_555(&self, __args: Ptr<Chunk>) -> Ptr<Chunk> {
        unimplemented!()
    }
}
impl vector_519Impl for Ptr<vector_519> {}
pub trait vector_556Impl {
    fn copy_assign(&self, __x: Ptr<vector_556>) -> Ptr<vector_556> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_556>) -> Ptr<vector_556> {
        unimplemented!()
    }
    fn begin_26(&self) -> __normal_iterator_456 {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_458 {
        unimplemented!()
    }
    fn end_28(&self) -> __normal_iterator_456 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_458 {
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
    fn operator_index_46(&self, mut __n: u64) -> Ptr<vector_457> {
        unimplemented!()
    }
    fn back_53(&self) -> Ptr<vector_457> {
        unimplemented!()
    }
    fn push_back_58(&self, __x: Ptr<vector_457>) {
        unimplemented!()
    }
    fn emplace_back_557(&self, __args_0: Ptr<u8>, __args_1: Ptr<Ptr<u8>>) -> Ptr<vector_457> {
        unimplemented!()
    }
    fn emplace_back_558(&self, __args: Ptr<vector_457>) -> Ptr<vector_457> {
        unimplemented!()
    }
}
impl vector_556Impl for Ptr<vector_556> {}
pub trait vector_559Impl {
    fn copy_assign(&self, __x: Ptr<vector_559>) -> Ptr<vector_559> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_559>) -> Ptr<vector_559> {
        unimplemented!()
    }
    fn begin_26(&self) -> __normal_iterator_462 {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_463 {
        unimplemented!()
    }
    fn end_28(&self) -> __normal_iterator_462 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_463 {
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
    fn operator_index_46(&self, mut __n: u64) -> Ptr<Pair> {
        unimplemented!()
    }
    fn back_53(&self) -> Ptr<Pair> {
        unimplemented!()
    }
    fn emplace_back_560(&self) -> Ptr<Pair> {
        unimplemented!()
    }
    fn emplace_back_561(&self, __args: Ptr<i32>) -> Ptr<Pair> {
        unimplemented!()
    }
    fn emplace_back_562(&self, __args_0: Ptr<i32>, __args_1: Ptr<i32>) -> Ptr<Pair> {
        unimplemented!()
    }
}
impl vector_559Impl for Ptr<vector_559> {}
pub trait vector_563Impl {
    fn copy_assign(&self, __x: Ptr<vector_563>) -> Ptr<vector_563> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_563>) -> Ptr<vector_563> {
        unimplemented!()
    }
    fn begin_26(&self) -> __normal_iterator_464 {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_465 {
        unimplemented!()
    }
    fn end_28(&self) -> __normal_iterator_464 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_465 {
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
    fn operator_index_46(&self, mut __n: u64) -> Ptr<i64> {
        unimplemented!()
    }
    fn back_53(&self) -> Ptr<i64> {
        unimplemented!()
    }
    fn emplace_back_564(&self) -> Ptr<i64> {
        unimplemented!()
    }
    fn emplace_back_565(&self, __args: Ptr<i32>) -> Ptr<i64> {
        unimplemented!()
    }
}
impl vector_563Impl for Ptr<vector_563> {}
pub fn __cpp2rust_init_globals() {}
