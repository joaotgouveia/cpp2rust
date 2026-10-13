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
pub struct __not__79 {}
impl Clone for __not__79 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__79 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__80 {}
impl Clone for __not__80 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__80 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__81 {}
impl Clone for __not__81 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__81 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__82 {}
impl Clone for __not__82 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__82 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__83 {}
impl Clone for __not__83 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__83 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__84 {}
impl Clone for __not__84 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__84 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__85 {}
impl Clone for __not__85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__85 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__86 {}
impl Clone for __not__86 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__86 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__87 {}
impl Clone for __not__87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__87 {
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
pub struct is_reference_92 {}
impl Clone for is_reference_92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_92 {
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
pub struct is_function_107 {}
impl Clone for is_function_107 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_107 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_108 {}
impl Clone for is_function_108 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_108 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_109 {}
impl Clone for is_function_109 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_109 {
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
pub struct is_void_122 {}
impl Clone for is_void_122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_122 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_123 {}
impl Clone for is_void_123 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_123 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_124 {}
impl Clone for is_void_124 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_124 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_125 {}
impl Clone for is_void_125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_125 {
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
pub struct remove_cv_153 {}
impl Clone for remove_cv_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_154 {}
impl Clone for remove_cv_154 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_154 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_155 {}
impl Clone for remove_cv_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_156 {}
impl Clone for remove_cv_156 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_156 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_157 {}
impl Clone for remove_cv_157 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_157 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_158 {}
impl Clone for remove_cv_158 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_158 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_159 {}
impl Clone for is_const_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_159 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_160 {}
impl Clone for is_const_160 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_160 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_161 {}
impl Clone for is_const_161 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_161 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_162 {}
impl Clone for is_const_162 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_162 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_163 {}
impl Clone for is_const_163 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_163 {
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
pub struct __is_array_unknown_bounds_174 {}
impl Clone for __is_array_unknown_bounds_174 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_174 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_175 {}
impl Clone for __is_array_unknown_bounds_175 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_175 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_176 {}
impl Clone for __is_array_unknown_bounds_176 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_176 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_177 {}
impl Clone for __is_array_unknown_bounds_177 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_177 {
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
pub struct is_class_189 {}
impl Clone for is_class_189 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_class_189 {
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
pub struct is_same_194 {}
impl Clone for is_same_194 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_194 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_195 {}
impl Clone for is_same_195 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_195 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_196 {}
impl Clone for is_same_196 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_196 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_197 {}
impl Clone for is_same_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_197 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_198 {}
impl Clone for is_same_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_199 {}
impl Clone for is_same_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_199 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_200 {}
impl Clone for is_same_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_200 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_201 {}
impl Clone for is_same_201 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_201 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_202 {}
impl Clone for is_same_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_202 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_203 {}
impl Clone for is_same_203 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_203 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_204 {}
impl Clone for is_same_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_204 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_205 {}
impl Clone for is_same_205 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_205 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_206 {}
impl Clone for is_same_206 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_206 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_207 {}
impl Clone for is_same_207 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_207 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_208 {}
impl Clone for is_same_208 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_208 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_209 {}
impl Clone for is_same_209 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_209 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_210 {}
impl Clone for is_volatile_210 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_210 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_211 {}
impl Clone for is_volatile_211 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_211 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_212 {}
impl Clone for is_volatile_212 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_212 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_213 {}
impl Clone for is_volatile_213 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_213 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_214 {}
impl Clone for is_volatile_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_214 {
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
pub struct is_default_constructible_215 {}
impl Clone for is_default_constructible_215 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_default_constructible_215 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_copy_constructible_216 {}
impl Clone for is_copy_constructible_216 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_constructible_216 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_217 {}
impl Clone for is_move_constructible_217 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_217 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_218 {}
impl Clone for is_move_constructible_218 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_218 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_219 {}
impl Clone for is_move_constructible_219 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_219 {
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
pub struct is_move_constructible_224 {}
impl Clone for is_move_constructible_224 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_224 {
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
pub struct is_nothrow_copy_constructible_227 {}
impl Clone for is_nothrow_copy_constructible_227 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_227 {
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
pub struct is_nothrow_move_constructible_234 {}
impl Clone for is_nothrow_move_constructible_234 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_234 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_constructible_235 {}
impl Clone for is_nothrow_move_constructible_235 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_235 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_constructible_236 {}
impl Clone for is_nothrow_move_constructible_236 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_236 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_constructible_237 {}
impl Clone for is_nothrow_move_constructible_237 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_237 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_constructible_238 {}
impl Clone for is_nothrow_move_constructible_238 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_238 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_constructible_239 {}
impl Clone for is_nothrow_move_constructible_239 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_239 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_constructible_240 {}
impl Clone for is_nothrow_move_constructible_240 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_240 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_copy_assignable_241 {}
impl Clone for is_copy_assignable_241 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_assignable_241 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_242 {}
impl Clone for is_move_assignable_242 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_242 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_243 {}
impl Clone for is_move_assignable_243 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_243 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_244 {}
impl Clone for is_move_assignable_244 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_244 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_245 {}
impl Clone for is_move_assignable_245 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_245 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_246 {}
impl Clone for is_move_assignable_246 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_246 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_247 {}
impl Clone for is_move_assignable_247 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_247 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_248 {}
impl Clone for is_move_assignable_248 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_248 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_249 {}
impl Clone for is_move_assignable_249 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_249 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_assignable_250 {}
impl Clone for is_nothrow_move_assignable_250 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_250 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_assignable_251 {}
impl Clone for is_nothrow_move_assignable_251 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_251 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_assignable_252 {}
impl Clone for is_nothrow_move_assignable_252 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_252 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_assignable_253 {}
impl Clone for is_nothrow_move_assignable_253 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_253 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_assignable_254 {}
impl Clone for is_nothrow_move_assignable_254 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_254 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_assignable_255 {}
impl Clone for is_nothrow_move_assignable_255 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_255 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_assignable_256 {}
impl Clone for is_nothrow_move_assignable_256 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_256 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_assignable_257 {}
impl Clone for is_nothrow_move_assignable_257 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_257 {
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
pub struct is_convertible_258 {}
impl Clone for is_convertible_258 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_258 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_259 {}
impl Clone for is_convertible_259 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_259 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_260 {}
impl Clone for is_convertible_260 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_260 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_261 {}
impl Clone for is_convertible_261 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_261 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_262 {}
impl Clone for is_convertible_262 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_262 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_263 {}
impl Clone for is_convertible_263 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_263 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_264 {}
impl Clone for is_convertible_264 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_264 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_265 {}
impl Clone for is_convertible_265 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_265 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_266 {}
impl Clone for is_convertible_266 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_266 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_267 {}
impl Clone for is_convertible_267 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_267 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_268 {}
impl Clone for is_convertible_268 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_268 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_269 {}
impl Clone for is_convertible_269 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_269 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_270 {}
impl Clone for remove_reference_270 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_270 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_271 {}
impl Clone for remove_reference_271 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_271 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_272 {}
impl Clone for remove_reference_272 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_272 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_273 {}
impl Clone for remove_reference_273 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_273 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_274 {}
impl Clone for remove_reference_274 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_274 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_275 {}
impl Clone for remove_reference_275 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_275 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_276 {}
impl Clone for remove_reference_276 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_276 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_277 {}
impl Clone for remove_reference_277 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_277 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_278 {}
impl Clone for remove_reference_278 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_278 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_279 {}
impl Clone for remove_reference_279 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_279 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_280 {}
impl Clone for remove_reference_280 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_280 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_281 {}
impl Clone for remove_reference_281 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_281 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_282 {}
impl Clone for remove_reference_282 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_282 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_283 {}
impl Clone for remove_reference_283 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_283 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_284 {}
impl Clone for remove_reference_284 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_284 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_285 {}
impl Clone for remove_reference_285 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_285 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_286 {}
impl Clone for remove_reference_286 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_286 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_287 {}
impl Clone for remove_reference_287 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_287 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_288 {}
impl Clone for remove_reference_288 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_288 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_289 {}
impl Clone for __cv_selector_289 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_289 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_290 {}
impl Clone for __cv_selector_290 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_290 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_291 {}
impl Clone for __cv_selector_291 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_291 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_292 {}
impl Clone for __cv_selector_292 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_292 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_293 {}
impl Clone for __match_cv_qualifiers_293 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_293 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_294 {}
impl Clone for __match_cv_qualifiers_294 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_294 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_295 {}
impl Clone for __match_cv_qualifiers_295 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_295 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_296 {}
impl Clone for __match_cv_qualifiers_296 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_296 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_297 {}
impl Clone for __match_cv_qualifiers_297 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_297 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_298 {}
impl Clone for __make_unsigned_298 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_298 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_299 {}
impl Clone for __make_unsigned_299 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_299 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_300 {}
impl Clone for __make_unsigned_300 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_300 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_301 {}
impl Clone for __make_unsigned_selector_301 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_301 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_302 {}
impl Clone for __make_unsigned_selector_302 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_302 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_303 {}
impl Clone for __make_unsigned_selector_303 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_303 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_304 {}
impl Clone for __make_unsigned_selector_304 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_304 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_305 {}
impl Clone for __make_unsigned_selector_305 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_305 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_306 {}
impl Clone for __make_unsigned_selector_306 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_306 {
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
pub struct _List_310 {}
impl Clone for _List_310 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_310 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_311 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_312 {}
impl Clone for _List_312 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_312 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_313 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_314 {}
impl Clone for _List_314 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_314 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_315 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_316 {}
impl Clone for _List_316 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_316 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_317 {}
impl Clone for _List_317 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_317 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_318 {}
impl Clone for _List_318 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_318 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_319 {}
impl Clone for __select_319 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_319 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_320 {}
impl Clone for __select_320 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_320 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_321 {}
impl Clone for __select_321 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_321 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_322 {}
impl Clone for __select_322 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_322 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_323 {}
impl Clone for __select_323 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_323 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_324 {}
impl Clone for __make_signed_324 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_324 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_325 {}
impl Clone for __make_signed_325 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_325 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_326 {}
impl Clone for __make_signed_selector_326 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_326 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_327 {}
impl Clone for __make_signed_selector_327 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_327 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_328 {}
impl Clone for __make_signed_selector_328 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_328 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_329 {}
impl Clone for __make_signed_selector_329 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_329 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_330 {}
impl Clone for __make_signed_selector_330 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_330 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct add_pointer_331 {}
impl Clone for add_pointer_331 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for add_pointer_331 {
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
pub struct common_type_332 {}
impl Clone for common_type_332 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_332 {
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
pub struct __is_tuple_like_impl_333 {}
impl Clone for __is_tuple_like_impl_333 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_333 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_impl_334 {}
impl Clone for __is_tuple_like_impl_334 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_334 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_impl_335 {}
impl Clone for __is_tuple_like_impl_335 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_335 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_impl_336 {}
impl Clone for __is_tuple_like_impl_336 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_336 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_impl_337 {}
impl Clone for __is_tuple_like_impl_337 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_337 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_impl_338 {}
impl Clone for __is_tuple_like_impl_338 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_338 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_impl_339 {}
impl Clone for __is_tuple_like_impl_339 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_339 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_impl_340 {}
impl Clone for __is_tuple_like_impl_340 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_340 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_341 {}
impl Clone for __is_tuple_like_341 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_341 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_342 {}
impl Clone for __is_tuple_like_342 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_342 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_343 {}
impl Clone for __is_tuple_like_343 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_343 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_344 {}
impl Clone for __is_tuple_like_344 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_344 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_345 {}
impl Clone for __is_tuple_like_345 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_345 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_346 {}
impl Clone for __is_tuple_like_346 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_346 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_347 {}
impl Clone for __is_tuple_like_347 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_347 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_348 {}
impl Clone for __is_tuple_like_348 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_348 {
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
    pub static is_array_v_349: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_copyable_v_350: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_default_constructible_v_351: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_standard_layout_v_352: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_353: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_array_v_354: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_copyable_v_355: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_default_constructible_v_356: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_standard_layout_v_357: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_358: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_array_v_359: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_copyable_v_360: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_default_constructible_v_361: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_standard_layout_v_362: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_363: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_array_v_364: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_copyable_v_365: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_default_constructible_v_366: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_standard_layout_v_367: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_368: Value<bool> = unimplemented!();
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
pub struct __are_same_369 {}
impl Clone for __are_same_369 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_369 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_370 = u32;
pub const anon_370___value: anon_370 = 1;
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
pub const anon_372___value: anon_372 = 0;
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
pub const anon_376___value: anon_376 = 1;
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
pub const anon_378___value: anon_378 = 0;
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
pub struct __is_integer_381 {}
impl Clone for __is_integer_381 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_381 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_382 = u32;
pub const anon_382___value: anon_382 = 1;
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
pub const anon_388___value: anon_388 = 1;
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
pub const anon_390___value: anon_390 = 1;
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
pub const anon_392___value: anon_392 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_393 {}
impl Clone for __is_integer_393 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_393 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_394 = u32;
pub const anon_394___value: anon_394 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_395 {}
impl Clone for __is_integer_395 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_395 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_396 = u32;
pub const anon_396___value: anon_396 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_397 {}
impl Clone for __is_integer_397 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_397 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_398 = u32;
pub const anon_398___value: anon_398 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_399 {}
impl Clone for __is_integer_399 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_399 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_400 = u32;
pub const anon_400___value: anon_400 = 0;
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
pub struct __conditional_type_404 {}
impl Clone for __conditional_type_404 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_type_404 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_405 {}
impl Clone for __is_integer_nonstrict_405 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_405 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_406 = u32;
pub const anon_406___width: anon_406 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_407 {}
impl Clone for __is_integer_nonstrict_407 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_407 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_408 = u32;
pub const anon_408___width: anon_408 = 32;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_409 {}
impl Clone for __is_integer_nonstrict_409 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_409 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_410 = u32;
pub const anon_410___width: anon_410 = 32;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_411 {}
impl Clone for __is_integer_nonstrict_411 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_411 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_412 = u32;
pub const anon_412___width: anon_412 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_413 {}
impl Clone for __is_integer_nonstrict_413 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_413 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_414 = u32;
pub const anon_414___width: anon_414 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_415 {}
impl Clone for __is_integer_nonstrict_415 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_415 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_416 = u32;
pub const anon_416___width: anon_416 = 8;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_417 {}
impl Clone for __is_integer_nonstrict_417 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_417 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_418 = u32;
pub const anon_418___width: anon_418 = 16;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_419 {}
impl Clone for __numeric_traits_integer_419 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_419 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____is_signed_420: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____digits_421: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____max_422: Value<i64> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____min_423: Value<i64> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_424 {}
impl Clone for __numeric_traits_integer_424 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_424 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_int____is_signed_425: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_int____digits_426: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_int____max_427: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_int____min_428: Value<i32> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_429 {}
impl Clone for __numeric_traits_integer_429 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_429 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_int____is_signed_430: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_int____digits_431: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_int____max_432: Value<u32> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_433 {}
impl Clone for __numeric_traits_integer_433 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_433 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_434: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____digits_435: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____max_436: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_437 {}
impl Clone for __numeric_traits_integer_437 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_437 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long_long____is_signed_438: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long_long____digits_439: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long_long____max_440: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_441 {}
impl Clone for __numeric_traits_integer_441 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_441 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_char____is_signed_442: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_char____digits_443: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_char____max_444: Value<i8> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_445 {}
impl Clone for __numeric_traits_integer_445 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_445 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_short____is_signed_446: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_short____digits_447: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_short____max_448: Value<i16> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_short____min_449: Value<i16> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_450 {}
impl Clone for __numeric_traits_floating_450 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_450 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_floating_float____max_exponent10_451: Value<i32> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_452 {}
impl Clone for __numeric_traits_floating_452 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_452 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_floating_double____max_exponent10_453: Value<i32> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_454 {}
impl Clone for __numeric_traits_floating_454 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_454 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_floating_long_double____max_exponent10_455: Value<i32> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_456 {}
impl Clone for __numeric_traits_456 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_456 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_457 {}
impl Clone for __numeric_traits_457 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_457 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_458 {}
impl Clone for __numeric_traits_458 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_458 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_459 {}
impl Clone for __numeric_traits_459 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_459 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_460 {}
impl Clone for __numeric_traits_460 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_460 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_461 {}
impl Clone for __numeric_traits_461 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_461 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_462 {}
impl Clone for __numeric_traits_462 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_462 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_463 {}
impl Clone for __numeric_traits_463 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_463 {
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
    pub static piecewise_construct_464: Value<std_piecewise_construct_t> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_465 {}
impl Clone for tuple_465 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_465 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_465 {}
impl tuple_465 {}
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
pub struct iterator_466 {}
impl Clone for iterator_466 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_466 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_466 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_466>) -> Ptr<iterator_466> {
        unimplemented!()
    }
}
impl iterator_466 {
    pub fn move_assign(&self, _a0: Ptr<iterator_466>) -> Ptr<iterator_466> {
        unimplemented!()
    }
}
impl iterator_466 {}
impl iterator_466 {}
impl iterator_466 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_467 {}
impl Clone for iterator_traits_467 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_467 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_468 {}
impl Clone for iterator_traits_468 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_468 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_469 {}
impl Clone for iterator_traits_469 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_469 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_470 {}
impl Clone for iterator_traits_470 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_470 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_471 {}
impl Clone for iterator_traits_471 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_471 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_472 {}
impl Clone for __ptr_traits_ptr_to_472 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_472 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_472 {
    pub fn pointer_to(__r: Ptr<i8>) -> Ptr<i8> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_473 {}
impl Clone for __ptr_traits_ptr_to_473 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_473 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_473 {
    pub fn pointer_to(__r: Ptr<i8>) -> Ptr<i8> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_474 {}
impl Clone for __ptr_traits_ptr_to_474 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_474 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_474 {
    pub fn pointer_to(__r: Ptr<i32>) -> Ptr<i32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_475 {}
impl Clone for __ptr_traits_ptr_to_475 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_475 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_475 {
    pub fn pointer_to(__r: Ptr<i32>) -> Ptr<i32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_476 {}
impl Clone for __ptr_traits_ptr_to_476 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_476 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_476 {
    pub fn pointer_to(__r: Ptr<u16>) -> Ptr<u16> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_477 {}
impl Clone for __ptr_traits_ptr_to_477 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_477 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_477 {
    pub fn pointer_to(__r: Ptr<u16>) -> Ptr<u16> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_478 {}
impl Clone for __ptr_traits_ptr_to_478 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_478 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_478 {
    pub fn pointer_to(__r: Ptr<u32>) -> Ptr<u32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_479 {}
impl Clone for __ptr_traits_ptr_to_479 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_479 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_479 {
    pub fn pointer_to(__r: Ptr<u32>) -> Ptr<u32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_480 {}
impl Clone for pointer_traits_480 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_480 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_481 {}
impl Clone for pointer_traits_481 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_481 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_482 {}
impl Clone for pointer_traits_482 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_482 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_483 {}
impl Clone for pointer_traits_483 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_483 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_484 {}
impl Clone for pointer_traits_484 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_484 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_485 {}
impl Clone for pointer_traits_485 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_485 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_486 {}
impl Clone for pointer_traits_486 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_486 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_487 {}
impl Clone for pointer_traits_487 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_487 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_488 {}
impl Clone for __normal_iterator_488 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_488 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_488 {
    pub fn new_1(__i: Ptr<Ptr<i8>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_488 {}
impl __normal_iterator_488 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_489 {}
impl Clone for __normal_iterator_489 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_489 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_489 {}
impl __normal_iterator_489 {}
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
pub struct char_traits_490 {}
impl Clone for char_traits_490 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_490 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_490 {
    pub fn assign_1(__c1: Ptr<i8>, __c2: Ptr<i8>) {
        unimplemented!()
    }
}
impl char_traits_490 {
    pub fn lt(__c1: Ptr<i8>, __c2: Ptr<i8>) -> bool {
        unimplemented!()
    }
}
impl char_traits_490 {
    pub fn length(mut __s: Ptr<i8>) -> usize {
        unimplemented!()
    }
}
impl char_traits_490 {
    pub fn move_(mut __s1: Ptr<i8>, mut __s2: Ptr<i8>, mut __n: usize) -> Ptr<i8> {
        unimplemented!()
    }
}
impl char_traits_490 {
    pub fn copy(mut __s1: Ptr<i8>, mut __s2: Ptr<i8>, mut __n: usize) -> Ptr<i8> {
        unimplemented!()
    }
}
impl char_traits_490 {
    pub fn eof() -> i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_491 {}
impl Clone for char_traits_491 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_491 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_491 {
    pub fn assign_1(__c1: Ptr<i32>, __c2: Ptr<i32>) {
        unimplemented!()
    }
}
impl char_traits_491 {
    pub fn copy(mut __s1: Ptr<i32>, mut __s2: Ptr<i32>, mut __n: usize) -> Ptr<i32> {
        unimplemented!()
    }
}
impl char_traits_491 {
    pub fn eq_int_type(__c1: Ptr<u32>, __c2: Ptr<u32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_491 {
    pub fn eof() -> u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_492 {}
impl Clone for char_traits_492 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_492 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_492 {
    pub fn assign_1(__c1: Ptr<u16>, __c2: Ptr<u16>) {
        unimplemented!()
    }
}
impl char_traits_492 {
    pub fn eq(__c1: Ptr<u16>, __c2: Ptr<u16>) -> bool {
        unimplemented!()
    }
}
impl char_traits_492 {
    pub fn lt(__c1: Ptr<u16>, __c2: Ptr<u16>) -> bool {
        unimplemented!()
    }
}
impl char_traits_492 {
    pub fn copy(mut __s1: Ptr<u16>, mut __s2: Ptr<u16>, mut __n: usize) -> Ptr<u16> {
        unimplemented!()
    }
}
impl char_traits_492 {
    pub fn eq_int_type(__c1: Ptr<u16>, __c2: Ptr<u16>) -> bool {
        unimplemented!()
    }
}
impl char_traits_492 {
    pub fn eof() -> u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_493 {}
impl Clone for char_traits_493 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_493 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_493 {
    pub fn assign_1(__c1: Ptr<u32>, __c2: Ptr<u32>) {
        unimplemented!()
    }
}
impl char_traits_493 {
    pub fn eq(__c1: Ptr<u32>, __c2: Ptr<u32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_493 {
    pub fn lt(__c1: Ptr<u32>, __c2: Ptr<u32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_493 {
    pub fn copy(mut __s1: Ptr<u32>, mut __s2: Ptr<u32>, mut __n: usize) -> Ptr<u32> {
        unimplemented!()
    }
}
impl char_traits_493 {
    pub fn eq_int_type(__c1: Ptr<u32>, __c2: Ptr<u32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_493 {
    pub fn eof() -> u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_494 {}
impl Clone for initializer_list_494 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_494 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_494 {}
impl initializer_list_494 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_495 {}
impl Clone for initializer_list_495 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_495 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_495 {}
impl initializer_list_495 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_496 {}
impl Clone for initializer_list_496 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_496 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_496 {}
impl initializer_list_496 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_497 {}
impl Clone for initializer_list_497 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_497 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_497 {}
impl initializer_list_497 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_498 {}
impl Clone for initializer_list_498 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_498 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_498 {}
impl initializer_list_498 {}
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
impl __locale_struct {}
impl __locale_struct {}
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
    pub __value32: anon_499,
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
pub struct anon_499 {
    #[offset(0)]
    pub __low: u32,
    #[offset(4)]
    pub __high: u32,
}
impl Clone for anon_499 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_499 {
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
impl __pthread_internal_list {}
impl __pthread_internal_list {}
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
impl __pthread_mutex_s {}
impl __pthread_mutex_s {}
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
impl pthread_mutex_t {}
impl pthread_mutex_t {}
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
pub type anon_500 = u32;
pub const anon_500__S_threshold: anon_500 = 16;
pub type anon_501 = u32;
pub const anon_501__S_chunk_size: anon_501 = 7;
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_502 {}
impl Clone for allocator_502 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_502 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_502 {}
impl allocator_502 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_503 {}
impl Clone for allocator_503 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_503 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_503 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_503 {
    pub fn copy_from(__a: Ptr<allocator_503>) -> Self {
        unimplemented!()
    }
}
impl allocator_503 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_503>) -> Ptr<allocator_503> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_504 {}
impl Clone for allocator_504 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_504 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_504 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_504 {
    pub fn copy_from(__a: Ptr<allocator_504>) -> Self {
        unimplemented!()
    }
}
impl allocator_504 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_504>) -> Ptr<allocator_504> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_505 {}
impl Clone for allocator_505 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_505 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_505 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_505 {
    pub fn copy_from(__a: Ptr<allocator_505>) -> Self {
        unimplemented!()
    }
}
impl allocator_505 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_505>) -> Ptr<allocator_505> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_506 {}
impl Clone for allocator_506 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_506 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_506 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_506 {
    pub fn copy_from(__a: Ptr<allocator_506>) -> Self {
        unimplemented!()
    }
}
impl allocator_506 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_506>) -> Ptr<allocator_506> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_507 {}
impl Clone for allocator_traits_507 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_507 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_507 {
    pub fn allocate_1(__a: Ptr<allocator_503>, mut __n: u64) -> Ptr<i8> {
        unimplemented!()
    }
}
impl allocator_traits_507 {
    pub fn deallocate(__a: Ptr<allocator_503>, mut __p: Ptr<i8>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_507 {
    pub fn max_size(__a: Ptr<allocator_503>) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_508 {}
impl Clone for allocator_traits_508 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_508 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_508 {
    pub fn allocate_1(__a: Ptr<allocator_504>, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
}
impl allocator_traits_508 {
    pub fn deallocate(__a: Ptr<allocator_504>, mut __p: Ptr<i32>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_508 {
    pub fn max_size(__a: Ptr<allocator_504>) -> u64 {
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
    pub fn allocate_1(__a: Ptr<allocator_505>, mut __n: u64) -> Ptr<u16> {
        unimplemented!()
    }
}
impl allocator_traits_509 {
    pub fn deallocate(__a: Ptr<allocator_505>, mut __p: Ptr<u16>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_509 {
    pub fn max_size(__a: Ptr<allocator_505>) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_510 {}
impl Clone for allocator_traits_510 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_510 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_510 {
    pub fn allocate_1(__a: Ptr<allocator_506>, mut __n: u64) -> Ptr<u32> {
        unimplemented!()
    }
}
impl allocator_traits_510 {
    pub fn deallocate(__a: Ptr<allocator_506>, mut __p: Ptr<u32>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_510 {
    pub fn max_size(__a: Ptr<allocator_506>) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct basic_string_511 {}
impl Clone for basic_string_511 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_511 {
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
    pub fn new_1(mut __dat: Ptr<i8>, __a: Ptr<allocator_503>) -> Self {
        unimplemented!()
    }
}
impl std_basic_string_char___Alloc_hider {
    pub fn new_2(mut __dat: Ptr<i8>, __a: Option<Ptr<allocator_503>>) -> Self {
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
pub type anon_513 = u32;
pub const anon_513__S_local_capacity: anon_513 = 15;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_512 {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_local_buf: Value<Box<[i8]>>,
    #[offset(0)]
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_512 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_512 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_511 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl basic_string_511 {
    pub fn copy_from(__str: Ptr<basic_string_511>) -> Self {
        unimplemented!()
    }
}
impl basic_string_511 {
    pub fn new_2(mut __s: Ptr<i8>, mut __n: u64, __a: Option<Ptr<allocator_503>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_511 {
    pub fn move_from(__str: Ptr<basic_string_511>) -> Self {
        unimplemented!()
    }
}
impl basic_string_511 {
    pub fn new_3(mut __s: Ptr<i8>, __a: Option<Ptr<allocator_503>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_511 {
    pub fn new_4(__beg: Ptr<i8>, mut __end: Ptr<i8>, __a: Option<Ptr<allocator_503>>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct basic_string_521 {}
impl Clone for basic_string_521 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_521 {
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
    pub fn new_1(mut __dat: Ptr<i32>, __a: Ptr<allocator_504>) -> Self {
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
pub type anon_523 = u32;
pub const anon_523__S_local_capacity: anon_523 = 3;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_522 {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_local_buf: Value<Box<[i32]>>,
    #[offset(0)]
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_522 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_522 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_521 {
    pub fn copy_from(__str: Ptr<basic_string_521>) -> Self {
        unimplemented!()
    }
}
impl basic_string_521 {
    pub fn new_1(mut __s: Ptr<i32>, mut __n: u64, __a: Option<Ptr<allocator_504>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_521 {
    pub fn move_from(__str: Ptr<basic_string_521>) -> Self {
        unimplemented!()
    }
}
impl basic_string_521 {
    pub fn new_2(__beg: Ptr<i8>, mut __end: Ptr<i8>, __a: Option<Ptr<allocator_504>>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct basic_string_524 {}
impl Clone for basic_string_524 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_524 {
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
    pub fn new_1(mut __dat: Ptr<u16>, __a: Ptr<allocator_505>) -> Self {
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
pub type anon_526 = u32;
pub const anon_526__S_local_capacity: anon_526 = 7;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_525 {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_local_buf: Value<Box<[u16]>>,
    #[offset(0)]
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_525 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_525 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_524 {
    pub fn copy_from(__str: Ptr<basic_string_524>) -> Self {
        unimplemented!()
    }
}
impl basic_string_524 {
    pub fn new_1(mut __s: Ptr<u16>, mut __n: u64, __a: Option<Ptr<allocator_505>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_524 {
    pub fn move_from(__str: Ptr<basic_string_524>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct basic_string_527 {}
impl Clone for basic_string_527 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_527 {
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
    pub fn new_1(mut __dat: Ptr<u32>, __a: Ptr<allocator_506>) -> Self {
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
pub type anon_529 = u32;
pub const anon_529__S_local_capacity: anon_529 = 3;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_528 {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_local_buf: Value<Box<[u32]>>,
    #[offset(0)]
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_528 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_528 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_527 {
    pub fn copy_from(__str: Ptr<basic_string_527>) -> Self {
        unimplemented!()
    }
}
impl basic_string_527 {
    pub fn new_1(mut __s: Ptr<u32>, mut __n: u64, __a: Option<Ptr<allocator_506>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_527 {
    pub fn move_from(__str: Ptr<basic_string_527>) -> Self {
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
    pub __value: anon_530,
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
pub struct anon_530 {
    #[offset(0)]
    pub __wch: u32,
    #[offset(0)]
    #[byte_size(4)]
    pub __wchb: Value<Box<[i8]>>,
}
impl Clone for anon_530 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_530 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl anon_530 {
    pub fn copy_assign(&self, _a0: Ptr<anon_530>) -> Ptr<anon_530> {
        unimplemented!()
    }
}
impl anon_530 {
    pub fn move_assign(&self, _a0: Ptr<anon_530>) -> Ptr<anon_530> {
        unimplemented!()
    }
}
impl anon_530 {}
impl anon_530 {}
impl __mbstate_t {
    pub fn copy_assign(&self, _a0: Ptr<__mbstate_t>) -> Ptr<__mbstate_t> {
        unimplemented!()
    }
}
impl __mbstate_t {
    pub fn move_assign(&self, _a0: Ptr<__mbstate_t>) -> Ptr<__mbstate_t> {
        unimplemented!()
    }
}
impl __mbstate_t {}
impl __mbstate_t {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(264)]
pub struct basic_ios_531 {}
impl Clone for basic_ios_531 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_ios_531 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(264)]
pub struct basic_ios_532 {}
impl Clone for basic_ios_532 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_ios_532 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(64)]
pub struct basic_streambuf_533 {}
impl Clone for basic_streambuf_533 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_streambuf_533 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(64)]
pub struct basic_streambuf_534 {}
impl Clone for basic_streambuf_534 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_streambuf_534 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(280)]
pub struct basic_istream_535 {}
impl Clone for basic_istream_535 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_istream_535 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(280)]
pub struct basic_istream_536 {}
impl Clone for basic_istream_536 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_istream_536 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(272)]
pub struct basic_ostream_537 {}
impl Clone for basic_ostream_537 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_ostream_537 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(272)]
pub struct basic_ostream_538 {}
impl Clone for basic_ostream_538 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_ostream_538 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(288)]
pub struct basic_iostream_539 {}
impl Clone for basic_iostream_539 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_iostream_539 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(288)]
pub struct basic_iostream_540 {}
impl Clone for basic_iostream_540 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_iostream_540 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(240)]
pub struct basic_filebuf_541 {}
impl Clone for basic_filebuf_541 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_filebuf_541 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_filebuf_541 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl basic_filebuf_541 {
    pub fn move_from(_a0: Ptr<basic_filebuf_541>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(240)]
pub struct basic_filebuf_542 {}
impl Clone for basic_filebuf_542 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_filebuf_542 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_filebuf_542 {
    pub fn move_from(_a0: Ptr<basic_filebuf_542>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(520)]
pub struct basic_ifstream_543 {}
impl Clone for basic_ifstream_543 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_ifstream_543 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_ifstream_543 {
    pub fn move_from(__rhs: Ptr<basic_ifstream_543>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(520)]
pub struct basic_ifstream_544 {}
impl Clone for basic_ifstream_544 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_ifstream_544 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_ifstream_544 {
    pub fn move_from(__rhs: Ptr<basic_ifstream_544>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(512)]
pub struct basic_ofstream_545 {}
impl Clone for basic_ofstream_545 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_ofstream_545 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_ofstream_545 {
    pub fn new_1(mut __s: Ptr<i8>, mut __mode: std__Ios_Openmode) -> Self {
        unimplemented!()
    }
}
impl basic_ofstream_545 {
    pub fn move_from(__rhs: Ptr<basic_ofstream_545>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(512)]
pub struct basic_ofstream_546 {}
impl Clone for basic_ofstream_546 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_ofstream_546 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_ofstream_546 {
    pub fn move_from(__rhs: Ptr<basic_ofstream_546>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(528)]
pub struct basic_fstream_547 {}
impl Clone for basic_fstream_547 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_fstream_547 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_fstream_547 {
    pub fn move_from(__rhs: Ptr<basic_fstream_547>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(528)]
pub struct basic_fstream_548 {}
impl Clone for basic_fstream_548 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_fstream_548 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_fstream_548 {
    pub fn move_from(__rhs: Ptr<basic_fstream_548>) -> Self {
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Char_types_549 {}
impl Clone for _Char_types_549 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Char_types_549 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Char_types_550 {}
impl Clone for _Char_types_550 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Char_types_550 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_551 {}
impl Clone for char_traits_551 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_551 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_551 {
    pub fn eq(__c1: Ptr<i8>, __c2: Ptr<i8>) -> bool {
        unimplemented!()
    }
}
impl char_traits_551 {
    pub fn length(mut __p: Ptr<i8>) -> usize {
        unimplemented!()
    }
}
impl char_traits_551 {
    pub fn find(mut __s: Ptr<i8>, mut __n: usize, __a: Ptr<i8>) -> Ptr<i8> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_552 {}
impl Clone for char_traits_552 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_552 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_552 {
    pub fn eq(__c1: Ptr<i32>, __c2: Ptr<i32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_552 {
    pub fn lt(__c1: Ptr<i32>, __c2: Ptr<i32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_552 {
    pub fn compare(mut __s1: Ptr<i32>, mut __s2: Ptr<i32>, mut __n: usize) -> i32 {
        unimplemented!()
    }
}
impl char_traits_552 {
    pub fn length(mut __p: Ptr<i32>) -> usize {
        unimplemented!()
    }
}
impl char_traits_552 {
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
pub type anon_553 = u32;
pub const anon_553__ISupper: anon_553 = 256;
pub const anon_553__ISlower: anon_553 = 512;
pub const anon_553__ISalpha: anon_553 = 1024;
pub const anon_553__ISdigit: anon_553 = 2048;
pub const anon_553__ISxdigit: anon_553 = 4096;
pub const anon_553__ISspace: anon_553 = 8192;
pub const anon_553__ISprint: anon_553 = 16384;
pub const anon_553__ISgraph: anon_553 = 32768;
pub const anon_553__ISblank: anon_553 = 1;
pub const anon_553__IScntrl: anon_553 = 2;
pub const anon_553__ISpunct: anon_553 = 4;
pub const anon_553__ISalnum: anon_553 = 8;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(576)]
pub struct ctype_554 {}
impl Clone for ctype_554 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ctype_554 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1344)]
pub struct ctype_555 {}
impl Clone for ctype_555 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ctype_555 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct codecvt_556 {}
impl Clone for codecvt_556 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for codecvt_556 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct codecvt_557 {}
impl Clone for codecvt_557 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for codecvt_557 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct codecvt_558 {}
impl Clone for codecvt_558 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for codecvt_558 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl codecvt_558 {
    pub fn new(__refs: Option<usize>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct codecvt_559 {}
impl Clone for codecvt_559 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for codecvt_559 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl codecvt_559 {
    pub fn new(__refs: Option<usize>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct num_get_560 {}
impl Clone for num_get_560 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for num_get_560 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct num_get_561 {}
impl Clone for num_get_561 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for num_get_561 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct num_put_562 {}
impl Clone for num_put_562 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for num_put_562 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct num_put_563 {}
impl Clone for num_put_563 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for num_put_563 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct numpunct_564 {}
impl Clone for numpunct_564 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numpunct_564 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct numpunct_565 {}
impl Clone for numpunct_565 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numpunct_565 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct collate_566 {}
impl Clone for collate_566 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for collate_566 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct collate_567 {}
impl Clone for collate_567 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for collate_567 {
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
    pub anon_568: i32,
    #[offset(168)]
    pub anon_569: i32,
    #[offset(172)]
    pub anon_570: i32,
    #[offset(176)]
    pub anon_571: i32,
    #[offset(180)]
    pub anon_572: i32,
    #[offset(184)]
    pub anon_573: i32,
    #[offset(188)]
    pub anon_574: i32,
    #[offset(192)]
    pub anon_575: i32,
    #[offset(196)]
    pub anon_576: i32,
    #[offset(200)]
    pub anon_577: i32,
    #[offset(204)]
    pub anon_578: i32,
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
pub type anon_579 = u32;
pub const anon_579_PTHREAD_MUTEX_TIMED_NP: anon_579 = 0;
pub const anon_579_PTHREAD_MUTEX_RECURSIVE_NP: anon_579 = 1;
pub const anon_579_PTHREAD_MUTEX_ERRORCHECK_NP: anon_579 = 2;
pub const anon_579_PTHREAD_MUTEX_ADAPTIVE_NP: anon_579 = 3;
pub const anon_579_PTHREAD_MUTEX_NORMAL: anon_579 = 0;
pub const anon_579_PTHREAD_MUTEX_RECURSIVE: anon_579 = 1;
pub const anon_579_PTHREAD_MUTEX_ERRORCHECK: anon_579 = 2;
pub const anon_579_PTHREAD_MUTEX_DEFAULT: anon_579 = 0;
pub const anon_579_PTHREAD_MUTEX_FAST_NP: anon_579 = 0;
pub type anon_580 = u32;
pub const anon_580_PTHREAD_MUTEX_STALLED: anon_580 = 0;
pub const anon_580_PTHREAD_MUTEX_STALLED_NP: anon_580 = 0;
pub const anon_580_PTHREAD_MUTEX_ROBUST: anon_580 = 1;
pub const anon_580_PTHREAD_MUTEX_ROBUST_NP: anon_580 = 1;
pub type anon_581 = u32;
pub const anon_581_PTHREAD_RWLOCK_PREFER_READER_NP: anon_581 = 0;
pub const anon_581_PTHREAD_RWLOCK_PREFER_WRITER_NP: anon_581 = 1;
pub const anon_581_PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: anon_581 = 2;
pub const anon_581_PTHREAD_RWLOCK_DEFAULT_NP: anon_581 = 0;
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
pub type anon_582 = u32;
pub const anon_582_PTHREAD_CANCEL_DEFERRED: anon_582 = 0;
pub const anon_582_PTHREAD_CANCEL_ASYNCHRONOUS: anon_582 = 1;
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
    pub static __libc_single_threaded_583: Value<i8> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_584 {}
impl Clone for __new_allocator_584 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_584 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_584 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_584 {
    pub fn copy_from(_a0: Ptr<__new_allocator_584>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_584 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_584>) -> Ptr<__new_allocator_584> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_585 {}
impl Clone for __new_allocator_585 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_585 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_585 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_585 {
    pub fn copy_from(_a0: Ptr<__new_allocator_585>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_585 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_585>) -> Ptr<__new_allocator_585> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_586 {}
impl Clone for __new_allocator_586 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_586 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_586 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_586 {
    pub fn copy_from(_a0: Ptr<__new_allocator_586>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_586 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_586>) -> Ptr<__new_allocator_586> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_587 {}
impl Clone for __new_allocator_587 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_587 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_587 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_587 {
    pub fn copy_from(_a0: Ptr<__new_allocator_587>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_587 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_587>) -> Ptr<__new_allocator_587> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct binary_function_588 {}
impl Clone for binary_function_588 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_588 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct binary_function_589 {}
impl Clone for binary_function_589 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_589 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl binary_function_589 {}
impl binary_function_589 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct binary_function_590 {}
impl Clone for binary_function_590 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_590 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl binary_function_590 {}
impl binary_function_590 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_591 {}
impl Clone for greater_591 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_591 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_592 {}
impl Clone for less_592 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_592 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_593 {}
impl Clone for less_593 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_593 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl less_593 {}
impl less_593 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_594 {}
impl Clone for less_594 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_594 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl less_594 {}
impl less_594 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_equal_595 {}
impl Clone for greater_equal_595 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_595 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_equal_596 {}
impl Clone for less_equal_596 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_596 {
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
pub struct __is_allocator_598 {}
impl Clone for __is_allocator_598 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_598 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_allocator_599 {}
impl Clone for __is_allocator_599 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_599 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_allocator_600 {}
impl Clone for __is_allocator_600 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_600 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_allocator_601 {}
impl Clone for __is_allocator_601 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_601 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_602 {}
impl Clone for __alloc_traits_602 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_602 {
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_604 {}
impl Clone for __alloc_traits_604 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_604 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_605 {}
impl Clone for rebind_605 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_605 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_606 {}
impl Clone for __alloc_traits_606 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_606 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_607 {}
impl Clone for rebind_607 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_607 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_608 {}
impl Clone for __alloc_traits_608 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_608 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_609 {}
impl Clone for rebind_609 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_609 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_610 {}
impl Clone for __hash_base_610 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_610 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_611 {}
impl Clone for __hash_base_611 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_611 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_612 {}
impl Clone for __hash_base_612 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_612 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_613 {}
impl Clone for __hash_base_613 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_613 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_614 {}
impl Clone for __hash_base_614 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_614 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_615 {}
impl Clone for __hash_base_615 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_615 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_616 {}
impl Clone for __hash_base_616 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_616 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_617 {}
impl Clone for __hash_base_617 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_617 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_618 {}
impl Clone for __hash_base_618 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_618 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_619 {}
impl Clone for __hash_base_619 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_619 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_620 {}
impl Clone for __hash_base_620 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_620 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_621 {}
impl Clone for __hash_base_621 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_621 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_622 {}
impl Clone for __hash_base_622 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_622 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_623 {}
impl Clone for __hash_base_623 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_623 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_624 {}
impl Clone for __hash_base_624 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_624 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_625 {}
impl Clone for __hash_base_625 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_625 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_626 {}
impl Clone for __hash_base_626 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_626 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_627 {}
impl Clone for __hash_base_627 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_627 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_628 {}
impl Clone for __hash_base_628 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_628 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_629 {}
impl Clone for __hash_base_629 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_629 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_630 {}
impl Clone for __hash_base_630 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_630 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_631 {}
impl Clone for __hash_base_631 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_631 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_632 {}
impl Clone for __hash_base_632 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_632 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_633 {}
impl Clone for __hash_base_633 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_633 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_634 {}
impl Clone for __hash_base_634 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_634 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_635 {}
impl Clone for __hash_base_635 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_635 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_636 {}
impl Clone for __hash_base_636 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_636 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_637 {}
impl Clone for hash_637 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_637 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_638 {}
impl Clone for hash_638 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_638 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_639 {}
impl Clone for hash_639 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_639 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_640 {}
impl Clone for hash_640 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_640 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_641 {}
impl Clone for hash_641 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_641 {
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
    pub fn hash_642(__val: Ptr<f32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_643(__val: Ptr<f64>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_644(__val: Ptr<i32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn __hash_combine_645(__val: Ptr<Ptr<std_error_category>>, mut __hash: usize) -> usize {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct basic_string_view_514 {}
impl Clone for basic_string_view_514 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_514 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_514 {
    pub fn copy_from(_a0: Ptr<basic_string_view_514>) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_514 {
    pub fn new_1(mut __str: Ptr<i8>, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_514 {
    pub fn copy_assign(&self, _a0: Ptr<basic_string_view_514>) -> Ptr<basic_string_view_514> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct basic_string_view_646 {}
impl Clone for basic_string_view_646 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_646 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_646 {
    pub fn copy_from(_a0: Ptr<basic_string_view_646>) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_646 {
    pub fn new_1(mut __str: Ptr<i32>, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_646 {
    pub fn copy_assign(&self, _a0: Ptr<basic_string_view_646>) -> Ptr<basic_string_view_646> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct basic_string_view_647 {}
impl Clone for basic_string_view_647 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_647 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_647 {
    pub fn copy_from(_a0: Ptr<basic_string_view_647>) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_647 {
    pub fn new_1(mut __str: Ptr<u16>, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_647 {
    pub fn copy_assign(&self, _a0: Ptr<basic_string_view_647>) -> Ptr<basic_string_view_647> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct basic_string_view_648 {}
impl Clone for basic_string_view_648 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_648 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_648 {
    pub fn copy_from(_a0: Ptr<basic_string_view_648>) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_648 {
    pub fn new_1(mut __str: Ptr<u32>, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_648 {
    pub fn copy_assign(&self, _a0: Ptr<basic_string_view_648>) -> Ptr<basic_string_view_648> {
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
    pub static __integer_to_chars_is_unsigned_649: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __integer_to_chars_is_unsigned_650: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __integer_to_chars_is_unsigned_651: Value<bool> = unimplemented!();
);
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
    pub static allocator_arg_652: Value<std_allocator_arg_t> = unimplemented!();
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
    pub static std_locale_ctype_653: Value<i32> = unimplemented!();
);
thread_local!(
    pub static std_locale_numeric_654: Value<i32> = unimplemented!();
);
thread_local!(
    pub static std_locale_collate_655: Value<i32> = unimplemented!();
);
thread_local!(
    pub static std_locale_time_656: Value<i32> = unimplemented!();
);
thread_local!(
    pub static std_locale_monetary_657: Value<i32> = unimplemented!();
);
thread_local!(
    pub static std_locale_messages_658: Value<i32> = unimplemented!();
);
pub type anon_659 = u32;
pub const anon_659__S_categories_size: anon_659 = 12;
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
    pub anon_660: anon_660,
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
pub struct anon_660 {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_p: Ptr<i8>,
    #[offset(0)]
    #[byte_size(8)]
    pub _M_bytes: Value<Box<[i8]>>,
}
impl Clone for anon_660 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_660 {
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
    pub fn new_1(__arg: Ptr<basic_string_511>) -> Self {
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
pub struct is_error_code_enum_661 {}
impl Clone for is_error_code_enum_661 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_661 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_code_enum_662 {}
impl Clone for is_error_code_enum_662 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_662 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_code_enum_663 {}
impl Clone for is_error_code_enum_663 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_663 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_code_enum_664 {}
impl Clone for is_error_code_enum_664 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_664 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_code_enum_665 {}
impl Clone for is_error_code_enum_665 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_665 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_code_enum_666 {}
impl Clone for is_error_code_enum_666 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_666 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_code_enum_667 {}
impl Clone for is_error_code_enum_667 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_667 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_condition_enum_668 {}
impl Clone for is_error_condition_enum_668 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_condition_enum_668 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_condition_enum_669 {}
impl Clone for is_error_condition_enum_669 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_condition_enum_669 {
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
    pub static std_ios_base_boolalpha_670: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_dec_671: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_fixed_672: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_hex_673: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_internal_674: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_left_675: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_oct_676: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_right_677: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_scientific_678: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_showbase_679: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_showpoint_680: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_showpos_681: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_skipws_682: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_unitbuf_683: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_uppercase_684: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_adjustfield_685: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_basefield_686: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_floatfield_687: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_badbit_688: Value<std__Ios_Iostate> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_eofbit_689: Value<std__Ios_Iostate> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_failbit_690: Value<std__Ios_Iostate> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_goodbit_691: Value<std__Ios_Iostate> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_app_692: Value<std__Ios_Openmode> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_ate_693: Value<std__Ios_Openmode> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_binary_694: Value<std__Ios_Openmode> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_in_695: Value<std__Ios_Openmode> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_out_696: Value<std__Ios_Openmode> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_beg_697: Value<std__Ios_Seekdir> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_cur_698: Value<std__Ios_Seekdir> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_end_699: Value<std__Ios_Seekdir> = unimplemented!();
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
pub type anon_700 = u32;
pub const anon_700__S_local_word_size: anon_700 = 8;
pub type anon_701 = i32;
pub const anon_701___ISwupper: anon_701 = 0;
pub const anon_701___ISwlower: anon_701 = 1;
pub const anon_701___ISwalpha: anon_701 = 2;
pub const anon_701___ISwdigit: anon_701 = 3;
pub const anon_701___ISwxdigit: anon_701 = 4;
pub const anon_701___ISwspace: anon_701 = 5;
pub const anon_701___ISwprint: anon_701 = 6;
pub const anon_701___ISwgraph: anon_701 = 7;
pub const anon_701___ISwblank: anon_701 = 8;
pub const anon_701___ISwcntrl: anon_701 = 9;
pub const anon_701___ISwpunct: anon_701 = 10;
pub const anon_701___ISwalnum: anon_701 = 11;
pub const anon_701__ISwupper: anon_701 = 16777216;
pub const anon_701__ISwlower: anon_701 = 33554432;
pub const anon_701__ISwalpha: anon_701 = 67108864;
pub const anon_701__ISwdigit: anon_701 = 134217728;
pub const anon_701__ISwxdigit: anon_701 = 268435456;
pub const anon_701__ISwspace: anon_701 = 536870912;
pub const anon_701__ISwprint: anon_701 = 1073741824;
pub const anon_701__ISwgraph: anon_701 = -2147483648;
pub const anon_701__ISwblank: anon_701 = 65536;
pub const anon_701__ISwcntrl: anon_701 = 131072;
pub const anon_701__ISwpunct: anon_701 = 262144;
pub const anon_701__ISwalnum: anon_701 = 524288;
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
    pub static std_ctype_base_upper_702: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_lower_703: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_alpha_704: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_digit_705: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_xdigit_706: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_space_707: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_print_708: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_graph_709: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_cntrl_710: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_punct_711: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_alnum_712: Value<u16> = unimplemented!();
);
thread_local!(
    pub static std_ctype_base_blank_713: Value<u16> = unimplemented!();
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
pub type anon_714 = u32;
pub const anon_714__S_ominus: anon_714 = 0;
pub const anon_714__S_oplus: anon_714 = 1;
pub const anon_714__S_ox: anon_714 = 2;
pub const anon_714__S_oX: anon_714 = 3;
pub const anon_714__S_odigits: anon_714 = 4;
pub const anon_714__S_odigits_end: anon_714 = 20;
pub const anon_714__S_oudigits: anon_714 = 20;
pub const anon_714__S_oudigits_end: anon_714 = 36;
pub const anon_714__S_oe: anon_714 = 18;
pub const anon_714__S_oE: anon_714 = 34;
pub const anon_714__S_oend: anon_714 = 36;
thread_local!(
    pub static std___num_base__S_atoms_out_715: Value<Ptr<i8>> = unimplemented!();
);
thread_local!(
    pub static std___num_base__S_atoms_in_716: Value<Ptr<i8>> = unimplemented!();
);
pub type anon_717 = u32;
pub const anon_717__S_iminus: anon_717 = 0;
pub const anon_717__S_iplus: anon_717 = 1;
pub const anon_717__S_ix: anon_717 = 2;
pub const anon_717__S_iX: anon_717 = 3;
pub const anon_717__S_izero: anon_717 = 4;
pub const anon_717__S_ie: anon_717 = 18;
pub const anon_717__S_iE: anon_717 = 24;
pub const anon_717__S_iend: anon_717 = 26;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_codecvt_base {}
impl Clone for std_codecvt_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_codecvt_base {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type std_codecvt_base_result = u32;
pub const std_codecvt_base_result_ok: std_codecvt_base_result = 0;
pub const std_codecvt_base_result_partial: std_codecvt_base_result = 1;
pub const std_codecvt_base_result_error: std_codecvt_base_result = 2;
pub const std_codecvt_base_result_noconv: std_codecvt_base_result = 3;
impl std_codecvt_base {}
impl std_codecvt_base {}
impl std_codecvt_base {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct __basic_file_718 {}
impl Clone for __basic_file_718 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __basic_file_718 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __basic_file_718 {
    pub fn move_from(__rv: Ptr<__basic_file_718>, _a1: Option<Ptr<pthread_mutex_t>>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct ostream_iterator_719 {}
impl Clone for ostream_iterator_719 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ostream_iterator_719 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl ostream_iterator_719 {
    pub fn new_1(__s: Ptr<basic_ostream_537>) -> Self {
        unimplemented!()
    }
}
impl ostream_iterator_719 {
    pub fn copy_from(__obj: Ptr<ostream_iterator_719>) -> Self {
        unimplemented!()
    }
}
impl ostream_iterator_719 {
    pub fn copy_assign(&self, _a0: Ptr<ostream_iterator_719>) -> Ptr<ostream_iterator_719> {
        unimplemented!()
    }
}
pub type anon_720 = u32;
pub const anon_720__SC_ARG_MAX: anon_720 = 0;
pub const anon_720__SC_CHILD_MAX: anon_720 = 1;
pub const anon_720__SC_CLK_TCK: anon_720 = 2;
pub const anon_720__SC_NGROUPS_MAX: anon_720 = 3;
pub const anon_720__SC_OPEN_MAX: anon_720 = 4;
pub const anon_720__SC_STREAM_MAX: anon_720 = 5;
pub const anon_720__SC_TZNAME_MAX: anon_720 = 6;
pub const anon_720__SC_JOB_CONTROL: anon_720 = 7;
pub const anon_720__SC_SAVED_IDS: anon_720 = 8;
pub const anon_720__SC_REALTIME_SIGNALS: anon_720 = 9;
pub const anon_720__SC_PRIORITY_SCHEDULING: anon_720 = 10;
pub const anon_720__SC_TIMERS: anon_720 = 11;
pub const anon_720__SC_ASYNCHRONOUS_IO: anon_720 = 12;
pub const anon_720__SC_PRIORITIZED_IO: anon_720 = 13;
pub const anon_720__SC_SYNCHRONIZED_IO: anon_720 = 14;
pub const anon_720__SC_FSYNC: anon_720 = 15;
pub const anon_720__SC_MAPPED_FILES: anon_720 = 16;
pub const anon_720__SC_MEMLOCK: anon_720 = 17;
pub const anon_720__SC_MEMLOCK_RANGE: anon_720 = 18;
pub const anon_720__SC_MEMORY_PROTECTION: anon_720 = 19;
pub const anon_720__SC_MESSAGE_PASSING: anon_720 = 20;
pub const anon_720__SC_SEMAPHORES: anon_720 = 21;
pub const anon_720__SC_SHARED_MEMORY_OBJECTS: anon_720 = 22;
pub const anon_720__SC_AIO_LISTIO_MAX: anon_720 = 23;
pub const anon_720__SC_AIO_MAX: anon_720 = 24;
pub const anon_720__SC_AIO_PRIO_DELTA_MAX: anon_720 = 25;
pub const anon_720__SC_DELAYTIMER_MAX: anon_720 = 26;
pub const anon_720__SC_MQ_OPEN_MAX: anon_720 = 27;
pub const anon_720__SC_MQ_PRIO_MAX: anon_720 = 28;
pub const anon_720__SC_VERSION: anon_720 = 29;
pub const anon_720__SC_PAGESIZE: anon_720 = 30;
pub const anon_720__SC_RTSIG_MAX: anon_720 = 31;
pub const anon_720__SC_SEM_NSEMS_MAX: anon_720 = 32;
pub const anon_720__SC_SEM_VALUE_MAX: anon_720 = 33;
pub const anon_720__SC_SIGQUEUE_MAX: anon_720 = 34;
pub const anon_720__SC_TIMER_MAX: anon_720 = 35;
pub const anon_720__SC_BC_BASE_MAX: anon_720 = 36;
pub const anon_720__SC_BC_DIM_MAX: anon_720 = 37;
pub const anon_720__SC_BC_SCALE_MAX: anon_720 = 38;
pub const anon_720__SC_BC_STRING_MAX: anon_720 = 39;
pub const anon_720__SC_COLL_WEIGHTS_MAX: anon_720 = 40;
pub const anon_720__SC_EQUIV_CLASS_MAX: anon_720 = 41;
pub const anon_720__SC_EXPR_NEST_MAX: anon_720 = 42;
pub const anon_720__SC_LINE_MAX: anon_720 = 43;
pub const anon_720__SC_RE_DUP_MAX: anon_720 = 44;
pub const anon_720__SC_CHARCLASS_NAME_MAX: anon_720 = 45;
pub const anon_720__SC_2_VERSION: anon_720 = 46;
pub const anon_720__SC_2_C_BIND: anon_720 = 47;
pub const anon_720__SC_2_C_DEV: anon_720 = 48;
pub const anon_720__SC_2_FORT_DEV: anon_720 = 49;
pub const anon_720__SC_2_FORT_RUN: anon_720 = 50;
pub const anon_720__SC_2_SW_DEV: anon_720 = 51;
pub const anon_720__SC_2_LOCALEDEF: anon_720 = 52;
pub const anon_720__SC_PII: anon_720 = 53;
pub const anon_720__SC_PII_XTI: anon_720 = 54;
pub const anon_720__SC_PII_SOCKET: anon_720 = 55;
pub const anon_720__SC_PII_INTERNET: anon_720 = 56;
pub const anon_720__SC_PII_OSI: anon_720 = 57;
pub const anon_720__SC_POLL: anon_720 = 58;
pub const anon_720__SC_SELECT: anon_720 = 59;
pub const anon_720__SC_UIO_MAXIOV: anon_720 = 60;
pub const anon_720__SC_IOV_MAX: anon_720 = 60;
pub const anon_720__SC_PII_INTERNET_STREAM: anon_720 = 61;
pub const anon_720__SC_PII_INTERNET_DGRAM: anon_720 = 62;
pub const anon_720__SC_PII_OSI_COTS: anon_720 = 63;
pub const anon_720__SC_PII_OSI_CLTS: anon_720 = 64;
pub const anon_720__SC_PII_OSI_M: anon_720 = 65;
pub const anon_720__SC_T_IOV_MAX: anon_720 = 66;
pub const anon_720__SC_THREADS: anon_720 = 67;
pub const anon_720__SC_THREAD_SAFE_FUNCTIONS: anon_720 = 68;
pub const anon_720__SC_GETGR_R_SIZE_MAX: anon_720 = 69;
pub const anon_720__SC_GETPW_R_SIZE_MAX: anon_720 = 70;
pub const anon_720__SC_LOGIN_NAME_MAX: anon_720 = 71;
pub const anon_720__SC_TTY_NAME_MAX: anon_720 = 72;
pub const anon_720__SC_THREAD_DESTRUCTOR_ITERATIONS: anon_720 = 73;
pub const anon_720__SC_THREAD_KEYS_MAX: anon_720 = 74;
pub const anon_720__SC_THREAD_STACK_MIN: anon_720 = 75;
pub const anon_720__SC_THREAD_THREADS_MAX: anon_720 = 76;
pub const anon_720__SC_THREAD_ATTR_STACKADDR: anon_720 = 77;
pub const anon_720__SC_THREAD_ATTR_STACKSIZE: anon_720 = 78;
pub const anon_720__SC_THREAD_PRIORITY_SCHEDULING: anon_720 = 79;
pub const anon_720__SC_THREAD_PRIO_INHERIT: anon_720 = 80;
pub const anon_720__SC_THREAD_PRIO_PROTECT: anon_720 = 81;
pub const anon_720__SC_THREAD_PROCESS_SHARED: anon_720 = 82;
pub const anon_720__SC_NPROCESSORS_CONF: anon_720 = 83;
pub const anon_720__SC_NPROCESSORS_ONLN: anon_720 = 84;
pub const anon_720__SC_PHYS_PAGES: anon_720 = 85;
pub const anon_720__SC_AVPHYS_PAGES: anon_720 = 86;
pub const anon_720__SC_ATEXIT_MAX: anon_720 = 87;
pub const anon_720__SC_PASS_MAX: anon_720 = 88;
pub const anon_720__SC_XOPEN_VERSION: anon_720 = 89;
pub const anon_720__SC_XOPEN_XCU_VERSION: anon_720 = 90;
pub const anon_720__SC_XOPEN_UNIX: anon_720 = 91;
pub const anon_720__SC_XOPEN_CRYPT: anon_720 = 92;
pub const anon_720__SC_XOPEN_ENH_I18N: anon_720 = 93;
pub const anon_720__SC_XOPEN_SHM: anon_720 = 94;
pub const anon_720__SC_2_CHAR_TERM: anon_720 = 95;
pub const anon_720__SC_2_C_VERSION: anon_720 = 96;
pub const anon_720__SC_2_UPE: anon_720 = 97;
pub const anon_720__SC_XOPEN_XPG2: anon_720 = 98;
pub const anon_720__SC_XOPEN_XPG3: anon_720 = 99;
pub const anon_720__SC_XOPEN_XPG4: anon_720 = 100;
pub const anon_720__SC_CHAR_BIT: anon_720 = 101;
pub const anon_720__SC_CHAR_MAX: anon_720 = 102;
pub const anon_720__SC_CHAR_MIN: anon_720 = 103;
pub const anon_720__SC_INT_MAX: anon_720 = 104;
pub const anon_720__SC_INT_MIN: anon_720 = 105;
pub const anon_720__SC_LONG_BIT: anon_720 = 106;
pub const anon_720__SC_WORD_BIT: anon_720 = 107;
pub const anon_720__SC_MB_LEN_MAX: anon_720 = 108;
pub const anon_720__SC_NZERO: anon_720 = 109;
pub const anon_720__SC_SSIZE_MAX: anon_720 = 110;
pub const anon_720__SC_SCHAR_MAX: anon_720 = 111;
pub const anon_720__SC_SCHAR_MIN: anon_720 = 112;
pub const anon_720__SC_SHRT_MAX: anon_720 = 113;
pub const anon_720__SC_SHRT_MIN: anon_720 = 114;
pub const anon_720__SC_UCHAR_MAX: anon_720 = 115;
pub const anon_720__SC_UINT_MAX: anon_720 = 116;
pub const anon_720__SC_ULONG_MAX: anon_720 = 117;
pub const anon_720__SC_USHRT_MAX: anon_720 = 118;
pub const anon_720__SC_NL_ARGMAX: anon_720 = 119;
pub const anon_720__SC_NL_LANGMAX: anon_720 = 120;
pub const anon_720__SC_NL_MSGMAX: anon_720 = 121;
pub const anon_720__SC_NL_NMAX: anon_720 = 122;
pub const anon_720__SC_NL_SETMAX: anon_720 = 123;
pub const anon_720__SC_NL_TEXTMAX: anon_720 = 124;
pub const anon_720__SC_XBS5_ILP32_OFF32: anon_720 = 125;
pub const anon_720__SC_XBS5_ILP32_OFFBIG: anon_720 = 126;
pub const anon_720__SC_XBS5_LP64_OFF64: anon_720 = 127;
pub const anon_720__SC_XBS5_LPBIG_OFFBIG: anon_720 = 128;
pub const anon_720__SC_XOPEN_LEGACY: anon_720 = 129;
pub const anon_720__SC_XOPEN_REALTIME: anon_720 = 130;
pub const anon_720__SC_XOPEN_REALTIME_THREADS: anon_720 = 131;
pub const anon_720__SC_ADVISORY_INFO: anon_720 = 132;
pub const anon_720__SC_BARRIERS: anon_720 = 133;
pub const anon_720__SC_BASE: anon_720 = 134;
pub const anon_720__SC_C_LANG_SUPPORT: anon_720 = 135;
pub const anon_720__SC_C_LANG_SUPPORT_R: anon_720 = 136;
pub const anon_720__SC_CLOCK_SELECTION: anon_720 = 137;
pub const anon_720__SC_CPUTIME: anon_720 = 138;
pub const anon_720__SC_THREAD_CPUTIME: anon_720 = 139;
pub const anon_720__SC_DEVICE_IO: anon_720 = 140;
pub const anon_720__SC_DEVICE_SPECIFIC: anon_720 = 141;
pub const anon_720__SC_DEVICE_SPECIFIC_R: anon_720 = 142;
pub const anon_720__SC_FD_MGMT: anon_720 = 143;
pub const anon_720__SC_FIFO: anon_720 = 144;
pub const anon_720__SC_PIPE: anon_720 = 145;
pub const anon_720__SC_FILE_ATTRIBUTES: anon_720 = 146;
pub const anon_720__SC_FILE_LOCKING: anon_720 = 147;
pub const anon_720__SC_FILE_SYSTEM: anon_720 = 148;
pub const anon_720__SC_MONOTONIC_CLOCK: anon_720 = 149;
pub const anon_720__SC_MULTI_PROCESS: anon_720 = 150;
pub const anon_720__SC_SINGLE_PROCESS: anon_720 = 151;
pub const anon_720__SC_NETWORKING: anon_720 = 152;
pub const anon_720__SC_READER_WRITER_LOCKS: anon_720 = 153;
pub const anon_720__SC_SPIN_LOCKS: anon_720 = 154;
pub const anon_720__SC_REGEXP: anon_720 = 155;
pub const anon_720__SC_REGEX_VERSION: anon_720 = 156;
pub const anon_720__SC_SHELL: anon_720 = 157;
pub const anon_720__SC_SIGNALS: anon_720 = 158;
pub const anon_720__SC_SPAWN: anon_720 = 159;
pub const anon_720__SC_SPORADIC_SERVER: anon_720 = 160;
pub const anon_720__SC_THREAD_SPORADIC_SERVER: anon_720 = 161;
pub const anon_720__SC_SYSTEM_DATABASE: anon_720 = 162;
pub const anon_720__SC_SYSTEM_DATABASE_R: anon_720 = 163;
pub const anon_720__SC_TIMEOUTS: anon_720 = 164;
pub const anon_720__SC_TYPED_MEMORY_OBJECTS: anon_720 = 165;
pub const anon_720__SC_USER_GROUPS: anon_720 = 166;
pub const anon_720__SC_USER_GROUPS_R: anon_720 = 167;
pub const anon_720__SC_2_PBS: anon_720 = 168;
pub const anon_720__SC_2_PBS_ACCOUNTING: anon_720 = 169;
pub const anon_720__SC_2_PBS_LOCATE: anon_720 = 170;
pub const anon_720__SC_2_PBS_MESSAGE: anon_720 = 171;
pub const anon_720__SC_2_PBS_TRACK: anon_720 = 172;
pub const anon_720__SC_SYMLOOP_MAX: anon_720 = 173;
pub const anon_720__SC_STREAMS: anon_720 = 174;
pub const anon_720__SC_2_PBS_CHECKPOINT: anon_720 = 175;
pub const anon_720__SC_V6_ILP32_OFF32: anon_720 = 176;
pub const anon_720__SC_V6_ILP32_OFFBIG: anon_720 = 177;
pub const anon_720__SC_V6_LP64_OFF64: anon_720 = 178;
pub const anon_720__SC_V6_LPBIG_OFFBIG: anon_720 = 179;
pub const anon_720__SC_HOST_NAME_MAX: anon_720 = 180;
pub const anon_720__SC_TRACE: anon_720 = 181;
pub const anon_720__SC_TRACE_EVENT_FILTER: anon_720 = 182;
pub const anon_720__SC_TRACE_INHERIT: anon_720 = 183;
pub const anon_720__SC_TRACE_LOG: anon_720 = 184;
pub const anon_720__SC_LEVEL1_ICACHE_SIZE: anon_720 = 185;
pub const anon_720__SC_LEVEL1_ICACHE_ASSOC: anon_720 = 186;
pub const anon_720__SC_LEVEL1_ICACHE_LINESIZE: anon_720 = 187;
pub const anon_720__SC_LEVEL1_DCACHE_SIZE: anon_720 = 188;
pub const anon_720__SC_LEVEL1_DCACHE_ASSOC: anon_720 = 189;
pub const anon_720__SC_LEVEL1_DCACHE_LINESIZE: anon_720 = 190;
pub const anon_720__SC_LEVEL2_CACHE_SIZE: anon_720 = 191;
pub const anon_720__SC_LEVEL2_CACHE_ASSOC: anon_720 = 192;
pub const anon_720__SC_LEVEL2_CACHE_LINESIZE: anon_720 = 193;
pub const anon_720__SC_LEVEL3_CACHE_SIZE: anon_720 = 194;
pub const anon_720__SC_LEVEL3_CACHE_ASSOC: anon_720 = 195;
pub const anon_720__SC_LEVEL3_CACHE_LINESIZE: anon_720 = 196;
pub const anon_720__SC_LEVEL4_CACHE_SIZE: anon_720 = 197;
pub const anon_720__SC_LEVEL4_CACHE_ASSOC: anon_720 = 198;
pub const anon_720__SC_LEVEL4_CACHE_LINESIZE: anon_720 = 199;
pub const anon_720__SC_IPV6: anon_720 = 235;
pub const anon_720__SC_RAW_SOCKETS: anon_720 = 236;
pub const anon_720__SC_V7_ILP32_OFF32: anon_720 = 237;
pub const anon_720__SC_V7_ILP32_OFFBIG: anon_720 = 238;
pub const anon_720__SC_V7_LP64_OFF64: anon_720 = 239;
pub const anon_720__SC_V7_LPBIG_OFFBIG: anon_720 = 240;
pub const anon_720__SC_SS_REPL_MAX: anon_720 = 241;
pub const anon_720__SC_TRACE_EVENT_NAME_MAX: anon_720 = 242;
pub const anon_720__SC_TRACE_NAME_MAX: anon_720 = 243;
pub const anon_720__SC_TRACE_SYS_MAX: anon_720 = 244;
pub const anon_720__SC_TRACE_USER_EVENT_MAX: anon_720 = 245;
pub const anon_720__SC_XOPEN_STREAMS: anon_720 = 246;
pub const anon_720__SC_THREAD_ROBUST_PRIO_INHERIT: anon_720 = 247;
pub const anon_720__SC_THREAD_ROBUST_PRIO_PROTECT: anon_720 = 248;
pub const anon_720__SC_MINSIGSTKSZ: anon_720 = 249;
pub const anon_720__SC_SIGSTKSZ: anon_720 = 250;
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let str: Value<basic_string_511> = Rc::new(RefCell::new(basic_string_511::new_3(
        { Ptr::<i8>::from_string_literal(b"Hello, world!\n") },
        None,
    )));
    let file: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(
        b"test_stdcopy_ostream.txt\0",
    )));
    {
        let ofs: Value<basic_ofstream_545> = Rc::new(RefCell::new(basic_ofstream_545::new_1(
            { (file.as_pointer() as Ptr<i8>) },
            { std_ios_base_binary_694.with(|rc| *rc.borrow()) },
        )));
        ({
            let ___first: __normal_iterator_488 =
                ({ basic_string_511Impl::begin_59(&str.as_pointer()) });
            let ___last: __normal_iterator_488 =
                ({ basic_string_511Impl::end_61(&str.as_pointer()) });
            copy_721(
                ___first,
                ___last,
                ostream_iterator_719::new_1({
                    __cpp2rust_unimplemented::<Ptr<basic_ostream_537>>()
                }),
            )
        });
    }
    ({ unlink_722((file.as_pointer() as Ptr<i8>)) });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_codecvt;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_marker;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct std_error_category;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct std___uses_alloc0__Sink;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_wide_data;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __locale_data;
pub fn copy_721(
    __first: __normal_iterator_488,
    __last: __normal_iterator_488,
    __result: ostream_iterator_719,
) -> ostream_iterator_719 {
    unimplemented!()
}
#[track_caller]
pub fn __cpp2rust_unimplemented<T>() -> T {
    unimplemented!()
}
pub fn unlink_722(mut __name: Ptr<i8>) -> i32 {
    unimplemented!()
}
pub trait __new_allocator_584Impl {
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
impl __new_allocator_584Impl for Ptr<__new_allocator_584> {}
pub trait __new_allocator_585Impl {
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
impl __new_allocator_585Impl for Ptr<__new_allocator_585> {}
pub trait __new_allocator_586Impl {
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
impl __new_allocator_586Impl for Ptr<__new_allocator_586> {}
pub trait __new_allocator_587Impl {
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
impl __new_allocator_587Impl for Ptr<__new_allocator_587> {}
pub trait __normal_iterator_488Impl {
    fn base(&self) -> Ptr<Ptr<i8>> {
        unimplemented!()
    }
}
impl __normal_iterator_488Impl for Ptr<__normal_iterator_488> {}
pub trait basic_filebuf_541Impl {
    fn move_assign(&self, _a0: Ptr<basic_filebuf_541>) -> Ptr<basic_filebuf_541> {
        unimplemented!()
    }
    fn open_11(&self, mut __s: Ptr<i8>, mut __mode: std__Ios_Openmode) -> Ptr<basic_filebuf_541> {
        unimplemented!()
    }
    fn close(&self) -> Ptr<basic_filebuf_541> {
        unimplemented!()
    }
}
impl basic_filebuf_541Impl for Ptr<basic_filebuf_541> {}
pub trait basic_filebuf_542Impl {
    fn move_assign(&self, _a0: Ptr<basic_filebuf_542>) -> Ptr<basic_filebuf_542> {
        unimplemented!()
    }
}
impl basic_filebuf_542Impl for Ptr<basic_filebuf_542> {}
pub trait basic_fstream_547Impl {
    fn move_assign(&self, __rhs: Ptr<basic_fstream_547>) -> Ptr<basic_fstream_547> {
        unimplemented!()
    }
}
impl basic_fstream_547Impl for Ptr<basic_fstream_547> {}
pub trait basic_fstream_548Impl {
    fn move_assign(&self, __rhs: Ptr<basic_fstream_548>) -> Ptr<basic_fstream_548> {
        unimplemented!()
    }
}
impl basic_fstream_548Impl for Ptr<basic_fstream_548> {}
pub trait basic_ifstream_543Impl {
    fn move_assign(&self, __rhs: Ptr<basic_ifstream_543>) -> Ptr<basic_ifstream_543> {
        unimplemented!()
    }
}
impl basic_ifstream_543Impl for Ptr<basic_ifstream_543> {}
pub trait basic_ifstream_544Impl {
    fn move_assign(&self, __rhs: Ptr<basic_ifstream_544>) -> Ptr<basic_ifstream_544> {
        unimplemented!()
    }
}
impl basic_ifstream_544Impl for Ptr<basic_ifstream_544> {}
pub trait basic_ios_531Impl {
    fn rdstate(&self) -> std__Ios_Iostate {
        unimplemented!()
    }
    fn clear(&self, __state: Option<std__Ios_Iostate>) {
        unimplemented!()
    }
    fn setstate(&self, mut __state: std__Ios_Iostate) {
        unimplemented!()
    }
}
impl basic_ios_531Impl for Ptr<basic_ios_531> {}
pub trait basic_ofstream_545Impl {
    fn move_assign(&self, __rhs: Ptr<basic_ofstream_545>) -> Ptr<basic_ofstream_545> {
        unimplemented!()
    }
    fn open_14(&self, mut __s: Ptr<i8>, mut __mode: std__Ios_Openmode) {
        unimplemented!()
    }
}
impl basic_ofstream_545Impl for Ptr<basic_ofstream_545> {}
pub trait basic_ofstream_546Impl {
    fn move_assign(&self, __rhs: Ptr<basic_ofstream_546>) -> Ptr<basic_ofstream_546> {
        unimplemented!()
    }
}
impl basic_ofstream_546Impl for Ptr<basic_ofstream_546> {}
pub trait basic_ostream_537Impl {
    fn put(&self, mut __c: i8) -> Ptr<basic_ostream_537> {
        unimplemented!()
    }
}
impl basic_ostream_537Impl for Ptr<basic_ostream_537> {}
pub trait basic_string_511Impl {
    fn copy_assign(&self, __str: Ptr<basic_string_511>) -> Ptr<basic_string_511> {
        unimplemented!()
    }
    fn move_assign(&self, __str: Ptr<basic_string_511>) -> Ptr<basic_string_511> {
        unimplemented!()
    }
    fn to_basic_string_view_514(&self) -> basic_string_view_514 {
        unimplemented!()
    }
    fn begin_59(&self) -> __normal_iterator_488 {
        unimplemented!()
    }
    fn end_61(&self) -> __normal_iterator_488 {
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
    fn insert_119(&self, mut __pos1: u64, __str: Ptr<basic_string_511>) -> Ptr<basic_string_511> {
        unimplemented!()
    }
    fn insert_122(&self, mut __pos: u64, mut __s: Ptr<i8>) -> Ptr<basic_string_511> {
        unimplemented!()
    }
    fn replace_133(
        &self,
        mut __pos: u64,
        mut __n1: u64,
        mut __s: Ptr<i8>,
        mut __n2: u64,
    ) -> Ptr<basic_string_511> {
        unimplemented!()
    }
    fn c_str(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn data_158(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn __resize_and_overwrite_515(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
    fn __resize_and_overwrite_516(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
    fn __resize_and_overwrite_517(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
    fn __resize_and_overwrite_518(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
    fn __resize_and_overwrite_519(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
    fn __resize_and_overwrite_520(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
}
impl basic_string_511Impl for Ptr<basic_string_511> {}
pub trait basic_string_521Impl {
    fn copy_assign(&self, __str: Ptr<basic_string_521>) -> Ptr<basic_string_521> {
        unimplemented!()
    }
    fn move_assign(&self, __str: Ptr<basic_string_521>) -> Ptr<basic_string_521> {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn c_str(&self) -> Ptr<i32> {
        unimplemented!()
    }
}
impl basic_string_521Impl for Ptr<basic_string_521> {}
pub trait basic_string_524Impl {
    fn copy_assign(&self, __str: Ptr<basic_string_524>) -> Ptr<basic_string_524> {
        unimplemented!()
    }
    fn move_assign(&self, __str: Ptr<basic_string_524>) -> Ptr<basic_string_524> {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_524Impl for Ptr<basic_string_524> {}
pub trait basic_string_527Impl {
    fn copy_assign(&self, __str: Ptr<basic_string_527>) -> Ptr<basic_string_527> {
        unimplemented!()
    }
    fn move_assign(&self, __str: Ptr<basic_string_527>) -> Ptr<basic_string_527> {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_527Impl for Ptr<basic_string_527> {}
pub trait basic_string_view_514Impl {
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
impl basic_string_view_514Impl for Ptr<basic_string_view_514> {}
pub trait basic_string_view_646Impl {
    fn length(&self) -> u64 {
        unimplemented!()
    }
    fn data(&self) -> Ptr<i32> {
        unimplemented!()
    }
}
impl basic_string_view_646Impl for Ptr<basic_string_view_646> {}
pub trait basic_string_view_647Impl {
    fn length(&self) -> u64 {
        unimplemented!()
    }
    fn data(&self) -> Ptr<u16> {
        unimplemented!()
    }
}
impl basic_string_view_647Impl for Ptr<basic_string_view_647> {}
pub trait basic_string_view_648Impl {
    fn length(&self) -> u64 {
        unimplemented!()
    }
    fn data(&self) -> Ptr<u32> {
        unimplemented!()
    }
}
impl basic_string_view_648Impl for Ptr<basic_string_view_648> {}
pub trait integral_constant_7Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl integral_constant_7Impl for Ptr<integral_constant_7> {}
pub trait less_593Impl {
    fn operator_call(
        &self,
        mut __x: Ptr<std_error_category>,
        mut __y: Ptr<std_error_category>,
    ) -> bool {
        unimplemented!()
    }
}
impl less_593Impl for Ptr<less_593> {}
pub trait less_594Impl {
    fn operator_call(&self, mut __x: Ptr<i8>, mut __y: Ptr<i8>) -> bool {
        unimplemented!()
    }
}
impl less_594Impl for Ptr<less_594> {}
pub trait ostream_iterator_719Impl {
    fn operator_assign_5(&self, __value: Ptr<i8>) -> Ptr<ostream_iterator_719> {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<ostream_iterator_719> {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<ostream_iterator_719> {
        unimplemented!()
    }
}
impl ostream_iterator_719Impl for Ptr<ostream_iterator_719> {}
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
    fn message(&self) -> basic_string_511 {
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
