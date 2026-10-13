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
pub struct __conditional_11 {}
impl Clone for __conditional_11 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_11 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_12 {}
impl Clone for __type_identity_12 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_12 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_13 {}
impl Clone for __type_identity_13 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_13 {
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
pub struct __or__22 {}
impl Clone for __or__22 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__22 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__23 {}
impl Clone for __or__23 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__23 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__24 {}
impl Clone for __or__24 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__24 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__25 {}
impl Clone for __or__25 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__25 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__26 {}
impl Clone for __or__26 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__26 {
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
pub struct __and__48 {}
impl Clone for __and__48 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__48 {
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
pub struct is_reference_67 {}
impl Clone for is_reference_67 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_67 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_68 {}
impl Clone for is_reference_68 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_68 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_69 {}
impl Clone for is_reference_69 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_69 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_70 {}
impl Clone for is_reference_70 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_70 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_71 {}
impl Clone for is_reference_71 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_71 {
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
pub struct is_function_73 {}
impl Clone for is_function_73 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_73 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_74 {}
impl Clone for is_function_74 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_74 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_75 {}
impl Clone for is_function_75 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_75 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_76 {}
impl Clone for is_function_76 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_76 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_77 {}
impl Clone for is_function_77 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_77 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_78 {}
impl Clone for is_function_78 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_78 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_79 {}
impl Clone for is_void_79 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_79 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_80 {}
impl Clone for is_void_80 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_80 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_81 {}
impl Clone for is_void_81 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_81 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_82 {}
impl Clone for is_void_82 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_82 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_83 {}
impl Clone for is_void_83 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_83 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_84 {}
impl Clone for is_void_84 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_84 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_85 {}
impl Clone for is_void_85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_85 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_86 {}
impl Clone for remove_cv_86 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_86 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_87 {}
impl Clone for remove_cv_87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_88 {}
impl Clone for remove_cv_88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_88 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_89 {}
impl Clone for remove_cv_89 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_89 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_90 {}
impl Clone for remove_cv_90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_90 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_91 {}
impl Clone for remove_cv_91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_91 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_92 {}
impl Clone for remove_cv_92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_93 {}
impl Clone for remove_cv_93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_93 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_94 {}
impl Clone for remove_cv_94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_94 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_95 {}
impl Clone for remove_cv_95 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_95 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_96 {}
impl Clone for remove_cv_96 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_96 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_97 {}
impl Clone for is_const_97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_97 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_98 {}
impl Clone for is_const_98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_98 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_99 {}
impl Clone for is_const_99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_99 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_100 {}
impl Clone for is_const_100 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_100 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_101 {}
impl Clone for is_const_101 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_101 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_102 {}
impl Clone for __is_array_unknown_bounds_102 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_102 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_103 {}
impl Clone for __is_array_unknown_bounds_103 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_103 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_104 {}
impl Clone for __is_array_unknown_bounds_104 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_104 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_105 {}
impl Clone for __is_array_unknown_bounds_105 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_105 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_106 {}
impl Clone for __is_array_unknown_bounds_106 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_106 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_107 {}
impl Clone for __is_array_unknown_bounds_107 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_107 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_108 {}
impl Clone for __is_integral_helper_108 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_108 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_109 {}
impl Clone for __is_integral_helper_109 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_109 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_110 {}
impl Clone for __is_integral_helper_110 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_110 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_111 {}
impl Clone for __is_integral_helper_111 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_111 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_112 {}
impl Clone for __is_integral_helper_112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_112 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_113 {}
impl Clone for __is_integral_helper_113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_113 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_114 {}
impl Clone for __is_integral_helper_114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_114 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_115 {}
impl Clone for __is_integral_helper_115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_115 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_116 {}
impl Clone for __is_integral_helper_116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_116 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_117 {}
impl Clone for __is_integral_helper_117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_117 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_118 {}
impl Clone for __is_integral_helper_118 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_118 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_119 {}
impl Clone for is_integral_119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_119 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_120 {}
impl Clone for is_integral_120 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_120 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_121 {}
impl Clone for is_integral_121 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_121 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_122 {}
impl Clone for is_integral_122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_122 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_123 {}
impl Clone for is_integral_123 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_123 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_124 {}
impl Clone for is_integral_124 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_124 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_125 {}
impl Clone for is_integral_125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_125 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_126 {}
impl Clone for is_integral_126 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_126 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_127 {}
impl Clone for is_integral_127 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_127 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_128 {}
impl Clone for is_integral_128 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_128 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_129 {}
impl Clone for is_integral_129 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_129 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_floating_point_helper_130 {}
impl Clone for __is_floating_point_helper_130 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_floating_point_helper_130 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_floating_point_helper_131 {}
impl Clone for __is_floating_point_helper_131 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_floating_point_helper_131 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_floating_point_helper_132 {}
impl Clone for __is_floating_point_helper_132 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_floating_point_helper_132 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_floating_point_helper_133 {}
impl Clone for __is_floating_point_helper_133 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_floating_point_helper_133 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_floating_point_134 {}
impl Clone for is_floating_point_134 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_floating_point_134 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_floating_point_135 {}
impl Clone for is_floating_point_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_floating_point_135 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_floating_point_136 {}
impl Clone for is_floating_point_136 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_floating_point_136 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_floating_point_137 {}
impl Clone for is_floating_point_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_floating_point_137 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_138 {}
impl Clone for is_pointer_138 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_138 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_139 {}
impl Clone for is_pointer_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_139 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_140 {}
impl Clone for is_pointer_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_140 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_enum_141 {}
impl Clone for is_enum_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_enum_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_enum_142 {}
impl Clone for is_enum_142 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_enum_142 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_enum_143 {}
impl Clone for is_enum_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_enum_143 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_null_pointer_144 {}
impl Clone for is_null_pointer_144 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_null_pointer_144 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_145 {}
impl Clone for is_arithmetic_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_145 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_146 {}
impl Clone for is_arithmetic_146 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_146 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_147 {}
impl Clone for is_arithmetic_147 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_147 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_148 {}
impl Clone for is_arithmetic_148 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_148 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_149 {}
impl Clone for is_arithmetic_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_member_pointer_150 {}
impl Clone for is_member_pointer_150 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_pointer_150 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_151 {}
impl Clone for is_scalar_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_152 {}
impl Clone for is_scalar_152 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_152 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_153 {}
impl Clone for is_scalar_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_154 {}
impl Clone for is_scalar_154 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_154 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_155 {}
impl Clone for is_scalar_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_155 {
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
pub struct is_volatile_161 {}
impl Clone for is_volatile_161 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_161 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_162 {}
impl Clone for is_volatile_162 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_162 {
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
pub struct remove_all_extents_166 {}
impl Clone for remove_all_extents_166 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_all_extents_166 {
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
pub struct __is_destructible_impl_167 {}
impl Clone for __is_destructible_impl_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_impl_167 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_safe_168 {}
impl Clone for __is_destructible_safe_168 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_168 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_safe_169 {}
impl Clone for __is_destructible_safe_169 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_169 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_safe_170 {}
impl Clone for __is_destructible_safe_170 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_170 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_safe_171 {}
impl Clone for __is_destructible_safe_171 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_171 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_safe_172 {}
impl Clone for __is_destructible_safe_172 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_172 {
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
pub struct __is_nt_destructible_safe_173 {}
impl Clone for __is_nt_destructible_safe_173 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_safe_173 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_destructible_174 {}
impl Clone for is_nothrow_destructible_174 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_destructible_174 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_175 {}
impl Clone for is_constructible_175 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_175 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_default_constructible_176 {}
impl Clone for is_default_constructible_176 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_default_constructible_176 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_default_constructible_177 {}
impl Clone for is_default_constructible_177 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_default_constructible_177 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_178 {}
impl Clone for is_move_constructible_178 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_178 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_179 {}
impl Clone for is_nothrow_default_constructible_179 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_179 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_180 {}
impl Clone for is_nothrow_copy_constructible_180 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_180 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_181 {}
impl Clone for is_nothrow_copy_constructible_181 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_181 {
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
pub struct is_copy_assignable_183 {}
impl Clone for is_copy_assignable_183 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_assignable_183 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_copy_assignable_184 {}
impl Clone for is_copy_assignable_184 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_assignable_184 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_copy_assignable_185 {}
impl Clone for is_copy_assignable_185 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_assignable_185 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_186 {}
impl Clone for is_move_assignable_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_187 {}
impl Clone for is_move_assignable_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_187 {
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
pub struct __is_implicitly_default_constructible_impl_188 {}
impl Clone for __is_implicitly_default_constructible_impl_188 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_impl_188 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_implicitly_default_constructible_impl_189 {}
impl Clone for __is_implicitly_default_constructible_impl_189 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_impl_189 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_implicitly_default_constructible_safe_190 {}
impl Clone for __is_implicitly_default_constructible_safe_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_safe_190 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_implicitly_default_constructible_safe_191 {}
impl Clone for __is_implicitly_default_constructible_safe_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_safe_191 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_implicitly_default_constructible_192 {}
impl Clone for __is_implicitly_default_constructible_192 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_192 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_implicitly_default_constructible_193 {}
impl Clone for __is_implicitly_default_constructible_193 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_193 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_trivially_destructible_194 {}
impl Clone for is_trivially_destructible_194 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_194 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_trivially_destructible_195 {}
impl Clone for is_trivially_destructible_195 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_195 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_trivially_destructible_196 {}
impl Clone for is_trivially_destructible_196 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_196 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_trivially_destructible_197 {}
impl Clone for is_trivially_destructible_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_197 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_trivially_destructible_198 {}
impl Clone for is_trivially_destructible_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_198 {
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
pub struct is_convertible_201 {}
impl Clone for is_convertible_201 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_201 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_202 {}
impl Clone for __is_nothrow_new_constructible_impl_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_202 {
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
pub struct __cv_selector_207 {}
impl Clone for __cv_selector_207 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_207 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_208 {}
impl Clone for __cv_selector_208 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_208 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_209 {}
impl Clone for __cv_selector_209 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_209 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_210 {}
impl Clone for __cv_selector_210 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_210 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_211 {}
impl Clone for __match_cv_qualifiers_211 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_211 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_212 {}
impl Clone for __match_cv_qualifiers_212 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_212 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_213 {}
impl Clone for __match_cv_qualifiers_213 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_213 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_214 {}
impl Clone for __match_cv_qualifiers_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_214 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_215 {}
impl Clone for __match_cv_qualifiers_215 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_215 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_216 {}
impl Clone for __make_unsigned_216 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_216 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_217 {}
impl Clone for __make_unsigned_217 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_217 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_218 {}
impl Clone for __make_unsigned_218 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_218 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_219 {}
impl Clone for __make_unsigned_selector_219 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_219 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_220 {}
impl Clone for __make_unsigned_selector_220 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_220 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_221 {}
impl Clone for __make_unsigned_selector_221 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_221 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_222 {}
impl Clone for __make_unsigned_selector_222 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_222 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_223 {}
impl Clone for __make_unsigned_selector_223 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_223 {
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
pub struct _List_228 {}
impl Clone for _List_228 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_228 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_229 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_230 {}
impl Clone for _List_230 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_230 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_231 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_232 {}
impl Clone for _List_232 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_232 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_233 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_234 {}
impl Clone for _List_234 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_234 {
    fn default() -> Self {
        unimplemented!()
    }
}
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_236 {}
impl Clone for _List_236 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_236 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_237 {}
impl Clone for __select_237 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_237 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_238 {}
impl Clone for __select_238 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_238 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_239 {}
impl Clone for __select_239 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_239 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_240 {}
impl Clone for __select_240 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_240 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_241 {}
impl Clone for __select_241 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_241 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_242 {}
impl Clone for __make_signed_242 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_242 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_243 {}
impl Clone for __make_signed_243 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_243 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_244 {}
impl Clone for __make_signed_selector_244 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_244 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_245 {}
impl Clone for __make_signed_selector_245 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_245 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_246 {}
impl Clone for __make_signed_selector_246 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_246 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_247 {}
impl Clone for __make_signed_selector_247 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_247 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_248 {}
impl Clone for __make_signed_selector_248 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_248 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_pointer_249 {}
impl Clone for remove_pointer_249 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_pointer_249 {
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
pub struct common_type_250 {}
impl Clone for common_type_250 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_250 {
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
    pub static __is_nothrow_new_constructible_251: Value<bool> = unimplemented!();
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
pub struct __are_same_252 {}
impl Clone for __are_same_252 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_252 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_253 = u32;
pub const anon_253___value: anon_253 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_254 {}
impl Clone for __are_same_254 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_254 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_255 = u32;
pub const anon_255___value: anon_255 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_256 {}
impl Clone for __are_same_256 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_256 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_257 = u32;
pub const anon_257___value: anon_257 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_258 {}
impl Clone for __are_same_258 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_258 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_259 = u32;
pub const anon_259___value: anon_259 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_260 {}
impl Clone for __are_same_260 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_260 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_261 = u32;
pub const anon_261___value: anon_261 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_262 {}
impl Clone for __are_same_262 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_262 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_263 = u32;
pub const anon_263___value: anon_263 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_264 {}
impl Clone for __are_same_264 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_264 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_265 = u32;
pub const anon_265___value: anon_265 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_266 {}
impl Clone for __are_same_266 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_266 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_267 = u32;
pub const anon_267___value: anon_267 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_268 {}
impl Clone for __are_same_268 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_268 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_269 = u32;
pub const anon_269___value: anon_269 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_270 {}
impl Clone for __is_integer_270 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_270 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_271 = u32;
pub const anon_271___value: anon_271 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_272 {}
impl Clone for __is_integer_272 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_272 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_273 = u32;
pub const anon_273___value: anon_273 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_274 {}
impl Clone for __is_integer_274 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_274 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_275 = u32;
pub const anon_275___value: anon_275 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_276 {}
impl Clone for __is_integer_276 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_276 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_277 = u32;
pub const anon_277___value: anon_277 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_278 {}
impl Clone for __is_integer_278 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_278 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_279 = u32;
pub const anon_279___value: anon_279 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_280 {}
impl Clone for __is_integer_280 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_280 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_281 = u32;
pub const anon_281___value: anon_281 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_282 {}
impl Clone for __is_integer_282 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_282 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_283 = u32;
pub const anon_283___value: anon_283 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_284 {}
impl Clone for __is_integer_284 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_284 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_285 = u32;
pub const anon_285___value: anon_285 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_byte_286 {}
impl Clone for __is_byte_286 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_byte_286 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_287 = u32;
pub const anon_287___value: anon_287 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_byte_288 {}
impl Clone for __is_byte_288 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_byte_288 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_289 = u32;
pub const anon_289___value: anon_289 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_byte_290 {}
impl Clone for __is_byte_290 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_byte_290 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_291 = u32;
pub const anon_291___value: anon_291 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_byte_292 {}
impl Clone for __is_byte_292 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_byte_292 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_293 = u32;
pub const anon_293___value: anon_293 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nonvolatile_trivially_copyable_294 {}
impl Clone for __is_nonvolatile_trivially_copyable_294 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nonvolatile_trivially_copyable_294 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_295 = u32;
pub const anon_295___value: anon_295 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_296 {}
impl Clone for __memcpyable_296 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_296 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_297 {}
impl Clone for __memcpyable_297 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_297 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_298 {}
impl Clone for __memcpyable_integer_298 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_298 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_299 {}
impl Clone for __memcpyable_integer_299 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_299 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_300 = u32;
pub const anon_300___width: anon_300 = 32;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_301 {}
impl Clone for __memcpyable_integer_301 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_301 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_302 = u32;
pub const anon_302___width: anon_302 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_303 {}
impl Clone for __memcpyable_integer_303 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_303 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_304 = u32;
pub const anon_304___width: anon_304 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __enable_if_305 {}
impl Clone for __enable_if_305 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __enable_if_305 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __conditional_type_306 {}
impl Clone for __conditional_type_306 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_type_306 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __conditional_type_307 {}
impl Clone for __conditional_type_307 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_type_307 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __conditional_type_308 {}
impl Clone for __conditional_type_308 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_type_308 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_309 {}
impl Clone for __is_integer_nonstrict_309 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_309 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_310 = u32;
pub const anon_310___width: anon_310 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_311 {}
impl Clone for __is_integer_nonstrict_311 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_311 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_312 = u32;
pub const anon_312___width: anon_312 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_313 {}
impl Clone for __numeric_traits_integer_313 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_313 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____is_signed_314: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____digits_315: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____max_316: Value<i64> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_317 {}
impl Clone for __numeric_traits_integer_317 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_317 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_318: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____digits_319: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____max_320: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_321 {}
impl Clone for __numeric_traits_floating_321 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_321 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_322 {}
impl Clone for __numeric_traits_floating_322 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_322 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_323 {}
impl Clone for __numeric_traits_floating_323 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_323 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_324 {}
impl Clone for __numeric_traits_324 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_324 {
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
    pub static piecewise_construct_325: Value<std_piecewise_construct_t> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct pair_326 {
    #[offset(0)]
    #[byte_size(8)]
    pub first: Ptr<i32>,
    #[offset(8)]
    pub second: i32,
}
impl Clone for pair_326 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pair_326 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl pair_326 {
    pub fn copy_from(_a0: Ptr<pair_326>) -> Self {
        unimplemented!()
    }
}
impl pair_326 {
    pub fn move_from(_a0: Ptr<pair_326>) -> Self {
        unimplemented!()
    }
}
impl pair_326 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_327 {}
impl Clone for tuple_327 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_327 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_327 {}
impl tuple_327 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _PCC_328 {}
impl Clone for _PCC_328 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _PCC_328 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __pair_base_329 {}
impl Clone for __pair_base_329 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pair_base_329 {
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
pub struct iterator_330 {}
impl Clone for iterator_330 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_330 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_331 {}
impl Clone for iterator_331 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_331 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_331 {}
impl iterator_331 {}
impl iterator_331 {}
impl iterator_331 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_331>) -> Ptr<iterator_331> {
        unimplemented!()
    }
}
impl iterator_331 {
    pub fn move_assign(&self, _a0: Ptr<iterator_331>) -> Ptr<iterator_331> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_332 {}
impl Clone for iterator_332 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_332 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_332 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_332>) -> Ptr<iterator_332> {
        unimplemented!()
    }
}
impl iterator_332 {
    pub fn move_assign(&self, _a0: Ptr<iterator_332>) -> Ptr<iterator_332> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_333 {}
impl Clone for iterator_333 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_333 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_333 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_333>) -> Ptr<iterator_333> {
        unimplemented!()
    }
}
impl iterator_333 {
    pub fn move_assign(&self, _a0: Ptr<iterator_333>) -> Ptr<iterator_333> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_334 {}
impl Clone for iterator_traits_334 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_334 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_335 {}
impl Clone for iterator_traits_335 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_335 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_336 {}
impl Clone for iterator_traits_336 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_336 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_337 {}
impl Clone for iterator_traits_337 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_337 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_338 {}
impl Clone for iterator_traits_338 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_338 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_339 {}
impl Clone for iterator_traits_339 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_339 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_340 {}
impl Clone for iterator_traits_340 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_340 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_341 {}
impl Clone for iterator_traits_341 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_341 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_342 {}
impl Clone for iterator_traits_342 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_342 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_343 {}
impl Clone for iterator_traits_343 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_343 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_344 {}
impl Clone for iterator_traits_344 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_344 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_345 {}
impl Clone for iterator_traits_345 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_345 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_346 {}
impl Clone for __iterator_traits_346 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_346 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_347 {}
impl Clone for __iterator_traits_347 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_347 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_348 {}
impl Clone for __iterator_traits_348 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_348 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_349 {}
impl Clone for __ptr_traits_ptr_to_349 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_349 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_350 {}
impl Clone for __ptr_traits_ptr_to_350 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_350 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_351 {}
impl Clone for pointer_traits_351 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_351 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_352 {}
impl Clone for pointer_traits_352 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_352 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_353 {}
impl Clone for reverse_iterator_353 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_353 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_353 {
    pub fn copy_from(__x: Ptr<reverse_iterator_353>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_353 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_353>) -> Ptr<reverse_iterator_353> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_354 {}
impl Clone for reverse_iterator_354 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_354 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_354 {
    pub fn copy_from(__x: Ptr<reverse_iterator_354>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_354 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_354>) -> Ptr<reverse_iterator_354> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_355 {}
impl Clone for __normal_iterator_355 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_355 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_355 {
    pub fn new_1(__i: Ptr<Ptr<i32>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_355 {}
impl __normal_iterator_355 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_356 {}
impl Clone for __normal_iterator_356 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_356 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_356 {
    pub fn new_1(__i: Ptr<Ptr<i32>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_356 {}
impl __normal_iterator_356 {}
impl __normal_iterator_356 {
    pub fn new_2(__i: Ptr<__normal_iterator_355>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_357 {}
impl Clone for __normal_iterator_357 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_357 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_357 {
    pub fn new_1(__i: Ptr<Ptr<Ptr<i32>>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_357 {}
impl __normal_iterator_357 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_358 {}
impl Clone for __normal_iterator_358 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_358 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_358 {
    pub fn new_1(__i: Ptr<Ptr<Ptr<i32>>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_358 {}
impl __normal_iterator_358 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_359 {}
impl Clone for __normal_iterator_359 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_359 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_359 {
    pub fn new_1(__i: Ptr<Ptr<Ptr<i32>>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_359 {}
impl __normal_iterator_359 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_360 {}
impl Clone for __normal_iterator_360 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_360 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_360 {
    pub fn new_1(__i: Ptr<Ptr<Ptr<i32>>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_360 {}
impl __normal_iterator_360 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_361 {}
impl Clone for __normal_iterator_361 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_361 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_361 {
    pub fn new_1(__i: Ptr<Ptr<f64>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_361 {}
impl __normal_iterator_361 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_362 {}
impl Clone for __normal_iterator_362 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_362 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_362 {
    pub fn new_1(__i: Ptr<Ptr<f64>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_362 {}
impl __normal_iterator_362 {}
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
pub struct __new_allocator_363 {}
impl Clone for __new_allocator_363 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_363 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_363 {
    pub fn copy_from(_a0: Ptr<__new_allocator_363>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_363 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_363>) -> Ptr<__new_allocator_363> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_364 {}
impl Clone for __new_allocator_364 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_364 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_364 {
    pub fn copy_from(_a0: Ptr<__new_allocator_364>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_364 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_364>) -> Ptr<__new_allocator_364> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_365 {}
impl Clone for __new_allocator_365 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_365 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_365 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_365 {
    pub fn copy_from(_a0: Ptr<__new_allocator_365>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_365 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_365>) -> Ptr<__new_allocator_365> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_368 {}
impl Clone for __new_allocator_368 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_368 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_368 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_368 {
    pub fn copy_from(_a0: Ptr<__new_allocator_368>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_368 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_368>) -> Ptr<__new_allocator_368> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_369 {}
impl Clone for __new_allocator_369 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_369 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_369 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_369 {
    pub fn copy_from(_a0: Ptr<__new_allocator_369>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_369 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_369>) -> Ptr<__new_allocator_369> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_370 {}
impl Clone for __new_allocator_370 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_370 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_370 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_370 {
    pub fn copy_from(_a0: Ptr<__new_allocator_370>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_370 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_370>) -> Ptr<__new_allocator_370> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_371 {}
impl Clone for __new_allocator_371 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_371 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_371 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_371 {
    pub fn copy_from(_a0: Ptr<__new_allocator_371>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_371 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_371>) -> Ptr<__new_allocator_371> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_372 {}
impl Clone for allocator_372 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_372 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_372 {}
impl allocator_372 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_373 {}
impl Clone for allocator_373 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_373 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_373 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_373 {
    pub fn copy_from(__a: Ptr<allocator_373>) -> Self {
        unimplemented!()
    }
}
impl allocator_373 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_373>) -> Ptr<allocator_373> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_374 {}
impl Clone for allocator_374 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_374 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_374 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_374 {
    pub fn copy_from(__a: Ptr<allocator_374>) -> Self {
        unimplemented!()
    }
}
impl allocator_374 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_374>) -> Ptr<allocator_374> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_375 {}
impl Clone for allocator_375 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_375 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_375 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_375 {
    pub fn copy_from(__a: Ptr<allocator_375>) -> Self {
        unimplemented!()
    }
}
impl allocator_375 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_375>) -> Ptr<allocator_375> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_376 {}
impl Clone for allocator_376 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_376 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_376 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_376 {
    pub fn copy_from(__a: Ptr<allocator_376>) -> Self {
        unimplemented!()
    }
}
impl allocator_376 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_376>) -> Ptr<allocator_376> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_377 {}
impl Clone for allocator_377 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_377 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_377 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_377 {
    pub fn copy_from(__a: Ptr<allocator_377>) -> Self {
        unimplemented!()
    }
}
impl allocator_377 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_377>) -> Ptr<allocator_377> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_378 {}
impl Clone for allocator_traits_378 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_378 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_378 {
    pub fn allocate_1(__a: Ptr<allocator_373>, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
}
impl allocator_traits_378 {
    pub fn deallocate(__a: Ptr<allocator_373>, mut __p: Ptr<i32>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_378 {
    pub fn max_size(__a: Ptr<allocator_373>) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_378 {
    pub fn select_on_container_copy_construction(__rhs: Ptr<allocator_373>) -> allocator_373 {
        unimplemented!()
    }
}
impl allocator_traits_378 {
    pub fn construct_379(__a: Ptr<allocator_373>, mut __p: Ptr<i32>, __args: Ptr<i32>) {
        unimplemented!()
    }
}
impl allocator_traits_378 {
    pub fn destroy_380(__a: Ptr<allocator_373>, mut __p: Ptr<i32>) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_381 {}
impl Clone for allocator_traits_381 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_381 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_381 {
    pub fn allocate_1(__a: Ptr<allocator_374>, mut __n: u64) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl allocator_traits_381 {
    pub fn deallocate(__a: Ptr<allocator_374>, mut __p: Ptr<Ptr<i32>>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_381 {
    pub fn max_size(__a: Ptr<allocator_374>) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_382 {}
impl Clone for allocator_traits_382 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_382 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_382 {
    pub fn allocate_1(__a: Ptr<allocator_375>, mut __n: u64) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl allocator_traits_382 {
    pub fn deallocate(__a: Ptr<allocator_375>, mut __p: Ptr<Ptr<i32>>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_382 {
    pub fn max_size(__a: Ptr<allocator_375>) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_383 {}
impl Clone for allocator_traits_383 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_383 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_383 {
    pub fn allocate_1(__a: Ptr<allocator_376>, mut __n: u64) -> Ptr<f64> {
        unimplemented!()
    }
}
impl allocator_traits_383 {
    pub fn deallocate(__a: Ptr<allocator_376>, mut __p: Ptr<f64>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_383 {
    pub fn max_size(__a: Ptr<allocator_376>) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_384 {}
impl Clone for allocator_traits_384 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_384 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_384 {
    pub fn allocate_1(__a: Ptr<allocator_377>, mut __n: u64) -> Ptr<pair_326> {
        unimplemented!()
    }
}
impl allocator_traits_384 {
    pub fn deallocate(__a: Ptr<allocator_377>, mut __p: Ptr<pair_326>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_384 {
    pub fn max_size(__a: Ptr<allocator_377>) -> u64 {
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
pub struct __is_move_insertable_386 {}
impl Clone for __is_move_insertable_386 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_386 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_387 {}
impl Clone for __alloc_traits_387 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_387 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_388 {}
impl Clone for rebind_388 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_388 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __alloc_traits_387 {
    pub fn _S_select_on_copy(__a: Ptr<allocator_373>) -> allocator_373 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_389 {}
impl Clone for __alloc_traits_389 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_389 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_390 {}
impl Clone for rebind_390 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_390 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_391 {}
impl Clone for __alloc_traits_391 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_391 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_392 {}
impl Clone for rebind_392 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_392 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_393 {}
impl Clone for __alloc_traits_393 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_393 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_394 {}
impl Clone for rebind_394 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_394 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_395 {}
impl Clone for __alloc_traits_395 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_395 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_396 {}
impl Clone for rebind_396 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_396 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct _UninitDestroyGuard_397 {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_first: Ptr<i32>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_cur: Ptr<Ptr<i32>>,
}
impl Clone for _UninitDestroyGuard_397 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _UninitDestroyGuard_397 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_397 {
    pub fn new(__first: Ptr<Ptr<i32>>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct _UninitDestroyGuard_398 {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_first: Ptr<f64>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_cur: Ptr<Ptr<f64>>,
}
impl Clone for _UninitDestroyGuard_398 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _UninitDestroyGuard_398 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_398 {
    pub fn new(__first: Ptr<Ptr<f64>>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct _UninitDestroyGuard_399 {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_first: Ptr<pair_326>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_cur: Ptr<Ptr<pair_326>>,
}
impl Clone for _UninitDestroyGuard_399 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _UninitDestroyGuard_399 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_399 {
    pub fn new(__first: Ptr<Ptr<pair_326>>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __uninitialized_default_n_1_400 {}
impl Clone for __uninitialized_default_n_1_400 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __uninitialized_default_n_1_400 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __uninitialized_default_n_1_400 {
    pub fn __uninit_default_n_401(mut __first: Ptr<i32>, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
}
impl __uninitialized_default_n_1_400 {
    pub fn __uninit_default_n_402(mut __first: Ptr<Ptr<i32>>, mut __n: u64) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl __uninitialized_default_n_1_400 {
    pub fn __uninit_default_n_403(mut __first: Ptr<Ptr<i32>>, mut __n: u64) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __uninitialized_default_n_1_404 {}
impl Clone for __uninitialized_default_n_1_404 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __uninitialized_default_n_1_404 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __uninitialized_default_n_1_404 {
    pub fn __uninit_default_n_405(__first: Ptr<pair_326>, mut __n: u64) -> Ptr<pair_326> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_bitwise_relocatable_406 {}
impl Clone for __is_bitwise_relocatable_406 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_bitwise_relocatable_406 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_407 {}
impl Clone for initializer_list_407 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_407 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_408 {}
impl Clone for initializer_list_408 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_408 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_408 {}
impl initializer_list_408 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_409 {}
impl Clone for initializer_list_409 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_409 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_409 {}
impl initializer_list_409 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_410 {}
impl Clone for initializer_list_410 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_410 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_410 {}
impl initializer_list_410 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_411 {}
impl Clone for initializer_list_411 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_411 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_411 {}
impl initializer_list_411 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_412 {}
impl Clone for initializer_list_412 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_412 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_412 {}
impl initializer_list_412 {}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_413 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_int__std_allocator_int____Vector_impl,
}
impl Clone for _Vector_base_413 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_413 {
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
    pub fn new_2(__a: Ptr<allocator_373>) -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__std_allocator_int____Vector_impl {
    pub fn move_from(__x: Ptr<std__Vector_base_int__std_allocator_int____Vector_impl>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_413 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_413 {
    pub fn new_2(mut __n: usize, __a: Ptr<allocator_373>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_413 {
    pub fn move_from(_a0: Ptr<_Vector_base_413>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_414 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl,
}
impl Clone for _Vector_base_414 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_414 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl_data {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_start: Ptr<Ptr<i32>>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_finish: Ptr<Ptr<i32>>,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_end_of_storage: Ptr<Ptr<i32>>,
}
impl Clone for std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl_data {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl_data {
    pub fn move_from(
        __x: Ptr<std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl_data>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl {}
impl Clone for std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl {
    pub fn new_1(__a: Ptr<allocator_374>) -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl {
    pub fn move_from(
        __x: Ptr<std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl>,
    ) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_414 {
    pub fn new_1(mut __n: usize, __a: Ptr<allocator_374>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_414 {
    pub fn move_from(_a0: Ptr<_Vector_base_414>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_415 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_const_int_ptr__std_allocator_const_int_ptr____Vector_impl,
}
impl Clone for _Vector_base_415 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_415 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_const_int_ptr__std_allocator_const_int_ptr____Vector_impl_data {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_start: Ptr<Ptr<i32>>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_finish: Ptr<Ptr<i32>>,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_end_of_storage: Ptr<Ptr<i32>>,
}
impl Clone for std__Vector_base_const_int_ptr__std_allocator_const_int_ptr____Vector_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_const_int_ptr__std_allocator_const_int_ptr____Vector_impl_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_const_int_ptr__std_allocator_const_int_ptr____Vector_impl_data {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_const_int_ptr__std_allocator_const_int_ptr____Vector_impl_data {
    pub fn move_from(
        __x: Ptr<std__Vector_base_const_int_ptr__std_allocator_const_int_ptr____Vector_impl_data>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_const_int_ptr__std_allocator_const_int_ptr____Vector_impl {}
impl Clone for std__Vector_base_const_int_ptr__std_allocator_const_int_ptr____Vector_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_const_int_ptr__std_allocator_const_int_ptr____Vector_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_const_int_ptr__std_allocator_const_int_ptr____Vector_impl {
    pub fn new_1(__a: Ptr<allocator_375>) -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_const_int_ptr__std_allocator_const_int_ptr____Vector_impl {
    pub fn move_from(
        __x: Ptr<std__Vector_base_const_int_ptr__std_allocator_const_int_ptr____Vector_impl>,
    ) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_415 {
    pub fn new_1(mut __n: usize, __a: Ptr<allocator_375>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_415 {
    pub fn move_from(_a0: Ptr<_Vector_base_415>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_416 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_double__std_allocator_double____Vector_impl,
}
impl Clone for _Vector_base_416 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_416 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_double__std_allocator_double____Vector_impl_data {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_start: Ptr<f64>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_finish: Ptr<f64>,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_end_of_storage: Ptr<f64>,
}
impl Clone for std__Vector_base_double__std_allocator_double____Vector_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_double__std_allocator_double____Vector_impl_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_double__std_allocator_double____Vector_impl_data {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_double__std_allocator_double____Vector_impl_data {
    pub fn move_from(
        __x: Ptr<std__Vector_base_double__std_allocator_double____Vector_impl_data>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_double__std_allocator_double____Vector_impl {}
impl Clone for std__Vector_base_double__std_allocator_double____Vector_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_double__std_allocator_double____Vector_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_double__std_allocator_double____Vector_impl {
    pub fn new_1(__a: Ptr<allocator_376>) -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_double__std_allocator_double____Vector_impl {
    pub fn move_from(
        __x: Ptr<std__Vector_base_double__std_allocator_double____Vector_impl>,
    ) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_416 {
    pub fn new_1(mut __n: usize, __a: Ptr<allocator_376>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_416 {
    pub fn move_from(_a0: Ptr<_Vector_base_416>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_417 { #[offset(0)] #[byte_size(24)] pub _M_impl : std__Vector_base_std_pair_const_int_ptr__int___std_allocator_std_pair_const_int_ptr__int_____Vector_impl  , }
impl Clone for _Vector_base_417 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_417 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_std_pair_const_int_ptr__int___std_allocator_std_pair_const_int_ptr__int_____Vector_impl_data
{
    #[offset(0)]
    #[byte_size(8)]
    pub _M_start: Ptr<pair_326>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_finish: Ptr<pair_326>,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_end_of_storage: Ptr<pair_326>,
}
impl Clone for std__Vector_base_std_pair_const_int_ptr__int___std_allocator_std_pair_const_int_ptr__int_____Vector_impl_data { fn clone(&self) -> Self { unimplemented!() } }
impl Default for std__Vector_base_std_pair_const_int_ptr__int___std_allocator_std_pair_const_int_ptr__int_____Vector_impl_data { fn default() -> Self { unimplemented!() } }
impl std__Vector_base_std_pair_const_int_ptr__int___std_allocator_std_pair_const_int_ptr__int_____Vector_impl_data { pub fn new ( ) -> Self { unimplemented!() } }
impl std__Vector_base_std_pair_const_int_ptr__int___std_allocator_std_pair_const_int_ptr__int_____Vector_impl_data { pub fn move_from ( __x : Ptr< std__Vector_base_std_pair_const_int_ptr__int___std_allocator_std_pair_const_int_ptr__int_____Vector_impl_data  > , ) -> Self { unimplemented!() } }
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_std_pair_const_int_ptr__int___std_allocator_std_pair_const_int_ptr__int_____Vector_impl
{}
impl Clone for std__Vector_base_std_pair_const_int_ptr__int___std_allocator_std_pair_const_int_ptr__int_____Vector_impl { fn clone(&self) -> Self { unimplemented!() } }
impl Default for std__Vector_base_std_pair_const_int_ptr__int___std_allocator_std_pair_const_int_ptr__int_____Vector_impl { fn default() -> Self { unimplemented!() } }
impl std__Vector_base_std_pair_const_int_ptr__int___std_allocator_std_pair_const_int_ptr__int_____Vector_impl { pub fn new_1 ( __a : Ptr< allocator_377  > , ) -> Self { unimplemented!() } }
impl std__Vector_base_std_pair_const_int_ptr__int___std_allocator_std_pair_const_int_ptr__int_____Vector_impl { pub fn move_from ( __x : Ptr< std__Vector_base_std_pair_const_int_ptr__int___std_allocator_std_pair_const_int_ptr__int_____Vector_impl  > , ) -> Self { unimplemented!() } }
impl _Vector_base_417 {
    pub fn new_1(mut __n: usize, __a: Ptr<allocator_377>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_417 {
    pub fn move_from(_a0: Ptr<_Vector_base_417>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_418 {}
impl Clone for vector_418 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_418 {
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
    pub _M_vect: Ptr<_Vector_base_413>,
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
    pub fn new(mut __s: Ptr<i32>, mut __l: u64, __vect: Ptr<_Vector_base_413>) -> Self {
        unimplemented!()
    }
}
impl vector_418 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_418 {
    pub fn new_2(mut __n: u64, __value: Ptr<i32>, __a: Option<Ptr<allocator_373>>) -> Self {
        unimplemented!()
    }
}
impl vector_418 {
    pub fn copy_from(__x: Ptr<vector_418>) -> Self {
        unimplemented!()
    }
}
impl vector_418 {
    pub fn move_from(_a0: Ptr<vector_418>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_420 {}
impl Clone for vector_420 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_420 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl vector_420 {
    pub fn new_1(mut __n: u64, __a: Option<Ptr<allocator_374>>) -> Self {
        unimplemented!()
    }
}
impl vector_420 {
    pub fn copy_from(__x: Ptr<vector_420>) -> Self {
        unimplemented!()
    }
}
impl vector_420 {
    pub fn move_from(_a0: Ptr<vector_420>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_421 {}
impl Clone for vector_421 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_421 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl vector_421 {
    pub fn new_1(mut __n: u64, __a: Option<Ptr<allocator_375>>) -> Self {
        unimplemented!()
    }
}
impl vector_421 {
    pub fn copy_from(__x: Ptr<vector_421>) -> Self {
        unimplemented!()
    }
}
impl vector_421 {
    pub fn move_from(_a0: Ptr<vector_421>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_422 {}
impl Clone for vector_422 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_422 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl vector_422 {
    pub fn new_1(mut __n: u64, __value: Ptr<f64>, __a: Option<Ptr<allocator_376>>) -> Self {
        unimplemented!()
    }
}
impl vector_422 {
    pub fn copy_from(__x: Ptr<vector_422>) -> Self {
        unimplemented!()
    }
}
impl vector_422 {
    pub fn move_from(_a0: Ptr<vector_422>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_423 {}
impl Clone for vector_423 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_423 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl vector_423 {
    pub fn new_1(mut __n: u64, __a: Option<Ptr<allocator_377>>) -> Self {
        unimplemented!()
    }
}
impl vector_423 {
    pub fn copy_from(__x: Ptr<vector_423>) -> Self {
        unimplemented!()
    }
}
impl vector_423 {
    pub fn move_from(_a0: Ptr<vector_423>) -> Self {
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
pub struct __hash_base_431 {}
impl Clone for __hash_base_431 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_431 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_432 {}
impl Clone for __hash_base_432 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_432 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_433 {}
impl Clone for __hash_base_433 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_433 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_434 {}
impl Clone for __hash_base_434 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_434 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_435 {}
impl Clone for __hash_base_435 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_435 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_436 {}
impl Clone for __hash_base_436 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_436 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_437 {}
impl Clone for __hash_base_437 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_437 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_438 {}
impl Clone for __hash_base_438 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_438 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_439 {}
impl Clone for __hash_base_439 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_439 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_440 {}
impl Clone for __hash_base_440 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_440 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_441 {}
impl Clone for __hash_base_441 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_441 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_442 {}
impl Clone for __hash_base_442 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_442 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_443 {}
impl Clone for __hash_base_443 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_443 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_444 {}
impl Clone for __hash_base_444 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_444 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_445 {}
impl Clone for hash_445 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_445 {
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
    pub fn hash_446(__val: Ptr<f32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_447(__val: Ptr<f64>) -> usize {
        unimplemented!()
    }
}
pub type anon_448 = u32;
pub const anon_448__S_word_bit: anon_448 = 64;
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
pub struct binary_function_449 {}
impl Clone for binary_function_449 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_449 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_450 {}
impl Clone for greater_450 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_450 {
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_equal_452 {}
impl Clone for greater_equal_452 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_452 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_equal_453 {}
impl Clone for less_equal_453 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_453 {
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
    pub static allocator_arg_454: Value<std_allocator_arg_t> = unimplemented!();
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
pub fn copy_455(mut copy_vector: vector_418) {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v1: Value<vector_418> = Rc::new(RefCell::new(<vector_418>::default()));
    (if (({ vector_418Impl::size(&v1.as_pointer()) }) == 0_u64) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v1.size() == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ vector_418Impl::empty(&v1.as_pointer()) }) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v1.empty()"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(1));
        vector_418Impl::push_back_58(&v1.as_pointer(), ___x.as_pointer())
    });
    (if !({ vector_418Impl::empty(&v1.as_pointer()) }) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"!v1.empty()"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ vector_418Impl::pop_back(&v1.as_pointer()) });
    (if ({ vector_418Impl::empty(&v1.as_pointer()) }) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v1.empty()"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut s1: u64 = ({ vector_418Impl::size(&v1.as_pointer()) });
    ({ vector_418Impl::resize_40(&v1.as_pointer(), 100_u64) });
    (if (({ vector_418Impl::size(&v1.as_pointer()) }) == 100_u64) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v1.size() == 100"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((({ vector_418Impl::operator_index_46(&v1.as_pointer(), 99_u64) }).read()) == 0) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v1[99] == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ vector_418Impl::operator_index_46(&v1.as_pointer(), 0_u64) }).write(40);
    ({ vector_418Impl::operator_index_46(&v1.as_pointer(), 99_u64) }).write(50);
    (if ((({ vector_418Impl::operator_index_46(&v1.as_pointer(), 0_u64) }).read()) == 40) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v1[0] == 40"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((({ vector_418Impl::operator_index_46(&v1.as_pointer(), 99_u64) }).read()) == 50) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v1[99] == 50"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let v2: Value<vector_418> = Rc::new(RefCell::new(<vector_418>::default()));
    (if (({ vector_418Impl::size(&v2.as_pointer()) }) == 0_u64) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v2.size() == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(1));
        vector_418Impl::push_back_58(&v2.as_pointer(), ___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(2));
        vector_418Impl::push_back_58(&v2.as_pointer(), ___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(3));
        vector_418Impl::push_back_58(&v2.as_pointer(), ___x.as_pointer())
    });
    (if (({ vector_418Impl::size(&v2.as_pointer()) }) == 3_u64) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v2.size() == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({
        let ___position: __normal_iterator_356 = {
            let __tmp_0: Value<__normal_iterator_355> = Rc::new(RefCell::new(
                ({ vector_418Impl::begin_26(&v2.as_pointer()) }),
            ));
            __normal_iterator_356::new_2({ __tmp_0.as_pointer() })
        };
        vector_418Impl::erase_67(&v2.as_pointer(), ___position)
    });
    (if (({ vector_418Impl::size(&v2.as_pointer()) }) == 2_u64) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v2.size() == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((({ vector_418Impl::operator_index_46(&v2.as_pointer(), 0_u64) }).read()) == 2) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v2[0] == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((({ vector_418Impl::operator_index_46(&v2.as_pointer(), 1_u64) }).read()) == 3) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v2[1] == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({
        let ___position: __normal_iterator_356 = {
            let __tmp_1: Value<__normal_iterator_355> = Rc::new(RefCell::new(
                ({ vector_418Impl::begin_26(&v2.as_pointer()) }),
            ));
            __normal_iterator_356::new_2({ __tmp_1.as_pointer() })
        };
        let ___x: Value<i32> = Rc::new(RefCell::new(100));
        vector_418Impl::insert_63(&v2.as_pointer(), ___position, ___x.as_pointer())
    });
    ({ copy_455(vector_418::copy_from({ v2.as_pointer() })) });
    (if (({ vector_418Impl::size(&v2.as_pointer()) }) == 3_u64) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v2.size() == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((({ vector_418Impl::operator_index_46(&v2.as_pointer(), 0_u64) }).read()) == 100) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v2[0] == 100"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((({ vector_418Impl::operator_index_46(&v2.as_pointer(), 1_u64) }).read()) == 2) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v2[1] == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((({ vector_418Impl::operator_index_46(&v2.as_pointer(), 2_u64) }).read()) == 3) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v2[2] == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut s2: u64 = ({ vector_418Impl::size(&v2.as_pointer()) });
    let v3: Value<vector_418> = Rc::new(RefCell::new({
        let __tmp_2: Value<i32> = Rc::new(RefCell::new(1));
        vector_418::new_2({ 100_u64 }, { __tmp_2.as_pointer() }, None)
    }));
    (if (({ vector_418Impl::size(&v3.as_pointer()) }) == 100_u64) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v3.size() == 100"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut i: i32 = 0;
    'loop_: while (i < 100) {
        (if ((({ vector_418Impl::operator_index_46(&v3.as_pointer(), (i as u64)) }).read()) == 1) {
            &(0);
        } else {
            ({
                __assert_fail_456(
                    Ptr::<i8>::from_string_literal(b"v3[i] == 1"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
        i.prefix_inc();
    }
    let v4: Value<vector_420> = Rc::new(RefCell::new(vector_420::new_1({ 100_u64 }, None)));
    (if (({ vector_420Impl::size(&v4.as_pointer()) }) == 100_u64) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v4.size() == 100"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut i: u32 = 0_u32;
    'loop_: while ((i as u64) < ({ vector_420Impl::size(&v4.as_pointer()) })) {
        (if (({ vector_420Impl::operator_index_46(&v4.as_pointer(), (i as u64)) }).read()).is_null()
        {
            &(0);
        } else {
            ({
                __assert_fail_456(
                    Ptr::<i8>::from_string_literal(b"v4[i] == nullptr"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
        i.prefix_inc();
    }
    let v5: Value<vector_421> = Rc::new(RefCell::new(vector_421::new_1({ 100_u64 }, None)));
    (if (({ vector_421Impl::size(&v5.as_pointer()) }) == 100_u64) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v5.size() == 100"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut i: u32 = 0_u32;
    'loop_: while ((i as u64) < ({ vector_421Impl::size(&v5.as_pointer()) })) {
        (if (({ vector_421Impl::operator_index_46(&v5.as_pointer(), (i as u64)) }).read()).is_null()
        {
            &(0);
        } else {
            ({
                __assert_fail_456(
                    Ptr::<i8>::from_string_literal(b"v5[i] == nullptr"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
        i.prefix_inc();
    }
    let v6: Value<vector_422> = Rc::new(RefCell::new({
        let __tmp_3: Value<f64> = Rc::new(RefCell::new(2.0E+0));
        vector_422::new_1({ s2 }, { __tmp_3.as_pointer() }, None)
    }));
    (if (({ vector_422Impl::size(&v6.as_pointer()) }) == s2) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v6.size() == s2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut i: u32 = 0_u32;
    'loop_: while ((i as u64) < s2) {
        (if ((({ vector_422Impl::operator_index_46(&v6.as_pointer(), (i as u64)) }).read())
            == 2.0E+0)
        {
            &(0);
        } else {
            ({
                __assert_fail_456(
                    Ptr::<i8>::from_string_literal(b"v6[i] == 2.0"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
        i.prefix_inc();
    }
    let v7: Value<vector_423> = Rc::new(RefCell::new(vector_423::new_1({ 200_u64 }, None)));
    (if (({ vector_423Impl::size(&v7.as_pointer()) }) == 200_u64) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v7.size() == 200"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut i: u32 = 0_u32;
    'loop_: while (i < 200_u32) {
        (if ((({ vector_423Impl::operator_index_46(&v7.as_pointer(), (i as u64)) })
            .with(|__s| __s.first.clone()))
        .is_null())
            && (({ vector_423Impl::operator_index_46(&v7.as_pointer(), (i as u64)) })
                .with(|__s| __s.second)
                == 0)
        {
            &(0);
        } else {
            ({
                __assert_fail_456(
                    Ptr::<i8>::from_string_literal(b"v7[i].first == nullptr && v7[i].second == 0"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
        i.prefix_inc();
    }
    let mut p1: Ptr<f64> = ({ vector_422Impl::data_55(&v6.as_pointer()) });
    (if ((p1.read()) == 2.0E+0) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"*p1 == 2.0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut p2: Ptr<i32> = ({ vector_418Impl::data_55(&v3.as_pointer()) });
    (if ((p2.read()) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"*p2 == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((({ vector_418Impl::operator_index_46(&v3.as_pointer(), 0_u64) }).read()) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v3[0] == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((({ vector_418Impl::operator_index_46(&v3.as_pointer(), 1_u64) }).read()) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v3[1] == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    p2.write((9.9E+1 as i32));
    (if ((p2.read()) == 99) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"*p2 == 99"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((({ vector_418Impl::operator_index_46(&v3.as_pointer(), 0_u64) }).read()) == 99) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v3[0] == 99"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((({ vector_418Impl::operator_index_46(&v3.as_pointer(), 1_u64) }).read()) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v3[1] == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    p2.prefix_inc();
    p2.write(98);
    (if ((({ vector_418Impl::operator_index_46(&v3.as_pointer(), 0_u64) }).read()) == 99) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v3[0] == 99"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((({ vector_418Impl::operator_index_46(&v3.as_pointer(), 1_u64) }).read()) == 98) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v3[1] == 98"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ vector_418Impl::capacity(&v3.as_pointer()) }) == 100_u64) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v3.capacity() == 100"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ vector_418Impl::size(&v3.as_pointer()) }) == 100_u64) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v3.size() == 100"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ vector_418Impl::reserve(&v3.as_pointer(), 200_u64) });
    (if (({ vector_418Impl::capacity(&v3.as_pointer()) }) == 200_u64) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v3.capacity() == 200"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ vector_418Impl::size(&v3.as_pointer()) }) == 100_u64) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v3.size() == 100"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ vector_418Impl::reserve(&v3.as_pointer(), 50_u64) });
    (if (({ vector_418Impl::capacity(&v3.as_pointer()) }) == 200_u64) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v3.capacity() == 200"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ vector_418Impl::size(&v3.as_pointer()) }) == 100_u64) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v3.size() == 100"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ vector_418Impl::reserve(&v3.as_pointer(), 200_u64) });
    (if (({ vector_418Impl::capacity(&v3.as_pointer()) }) == 200_u64) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v3.capacity() == 200"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ vector_418Impl::size(&v3.as_pointer()) }) == 100_u64) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v3.size() == 100"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ vector_418Impl::reserve(&v3.as_pointer(), 201_u64) });
    (if (({ vector_418Impl::capacity(&v3.as_pointer()) }) == 201_u64) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v3.capacity() == 201"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ vector_418Impl::size(&v3.as_pointer()) }) == 100_u64) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v3.size() == 100"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((({ vector_418Impl::back_53(&v2.as_pointer()) }).read()) == 3) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v2.back() == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((({ vector_418Impl::back_53(&v3.as_pointer()) }).read()) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v3.back() == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ vector_420Impl::back_53(&v4.as_pointer()) }).read()).is_null() {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v4.back() == nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ vector_421Impl::back_53(&v5.as_pointer()) }).read()).is_null() {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v5.back() == nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((({ vector_422Impl::back_53(&v6.as_pointer()) }).read()) == 2.0E+0) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v6.back() == 2.0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let ref0: Ptr<f64> = ({ vector_422Impl::back_53(&v6.as_pointer()) });
    ref0.write(5.0E+0);
    (if ((({ vector_422Impl::back_53(&v6.as_pointer()) }).read()) == 5.0E+0) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v6.back() == 5.0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut x0: f64 = (({ vector_422Impl::back_53(&v6.as_pointer()) }).read());
    (if (x0 == 5.0E+0) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"x0 == 5.0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    x0 = 6.0E+0;
    (if ((({ vector_422Impl::back_53(&v6.as_pointer()) }).read()) == 5.0E+0) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v6.back() == 5.0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut idx: i32 = 0;
    (if ((({ vector_422Impl::at_49(&v6.as_pointer(), (idx as u64)) }).read()) == 2.0E+0) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v6.at(idx) == 2.0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((({ vector_422Impl::at_49(&v6.as_pointer(), (s2).wrapping_sub(1_u64)) }).read()) == 5.0E+0)
    {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v6.at(s2 - 1) == 5.0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let ref1: Ptr<f64> = ({ vector_422Impl::at_49(&v6.as_pointer(), (s2).wrapping_sub(1_u64)) });
    {
        ref1.with_mut(|__v| *__v = *__v + 1.5E+0)
    };
    (if ((({ vector_422Impl::at_49(&v6.as_pointer(), (s2).wrapping_sub(1_u64)) }).read()) == 6.5E+0)
    {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v6.at(s2 - 1) == 6.5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut x1: f64 =
        (({ vector_422Impl::at_49(&v6.as_pointer(), (s2).wrapping_sub(1_u64)) }).read());
    (if (x1 == 6.5E+0) {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"x1 == 6.5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    x1 -= 1.5E+0;
    (if ((({ vector_422Impl::at_49(&v6.as_pointer(), (s2).wrapping_sub(1_u64)) }).read()) == 6.5E+0)
    {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"v6.at(s2 - 1) == 6.5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((s1).wrapping_add(s2))
        .wrapping_add(((({ vector_418Impl::at_49(&v2.as_pointer(), 0_u64) }).read()) as u64))
        == 103_u64)
    {
        &(0);
    } else {
        ({
            __assert_fail_456(
                Ptr::<i8>::from_string_literal(b"s1 + s2 + v2.at(0) == 103"),
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
pub fn __assert_fail_456(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub trait _UninitDestroyGuard_397Impl {
    fn release(&self) {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_397Impl for Ptr<_UninitDestroyGuard_397> {}
pub trait _UninitDestroyGuard_398Impl {
    fn release(&self) {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_398Impl for Ptr<_UninitDestroyGuard_398> {}
pub trait _UninitDestroyGuard_399Impl {
    fn release(&self) {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_399Impl for Ptr<_UninitDestroyGuard_399> {}
pub trait _Vector_base_413Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_373> {
        unimplemented!()
    }
    fn _M_get_Tp_allocator_2(&self) -> Ptr<allocator_373> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<i32> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<i32>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_413Impl for Ptr<_Vector_base_413> {}
pub trait _Vector_base_414Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_374> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<Ptr<i32>>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_414Impl for Ptr<_Vector_base_414> {}
pub trait _Vector_base_415Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_375> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<Ptr<i32>>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_415Impl for Ptr<_Vector_base_415> {}
pub trait _Vector_base_416Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_376> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<f64> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<f64>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_416Impl for Ptr<_Vector_base_416> {}
pub trait _Vector_base_417Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_377> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<pair_326> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<pair_326>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_417Impl for Ptr<_Vector_base_417> {}
pub trait __new_allocator_365Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<i32> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<i32>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn construct_366(&self, mut __p: Ptr<i32>, __args: Ptr<i32>) {
        unimplemented!()
    }
    fn destroy_367(&self, mut __p: Ptr<i32>) {
        unimplemented!()
    }
}
impl __new_allocator_365Impl for Ptr<__new_allocator_365> {}
pub trait __new_allocator_368Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<Ptr<i32>>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl __new_allocator_368Impl for Ptr<__new_allocator_368> {}
pub trait __new_allocator_369Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<Ptr<i32>>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl __new_allocator_369Impl for Ptr<__new_allocator_369> {}
pub trait __new_allocator_370Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<f64> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<f64>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl __new_allocator_370Impl for Ptr<__new_allocator_370> {}
pub trait __new_allocator_371Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<pair_326> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<pair_326>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl __new_allocator_371Impl for Ptr<__new_allocator_371> {}
pub trait __normal_iterator_355Impl {
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_add(&self, mut __n: i64) -> __normal_iterator_355 {
        unimplemented!()
    }
    fn operator_sub(&self, mut __n: i64) -> __normal_iterator_355 {
        unimplemented!()
    }
    fn base(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl __normal_iterator_355Impl for Ptr<__normal_iterator_355> {}
pub trait __normal_iterator_356Impl {
    fn base(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl __normal_iterator_356Impl for Ptr<__normal_iterator_356> {}
pub trait __normal_iterator_357Impl {
    fn operator_deref(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn operator_sub(&self, mut __n: i64) -> __normal_iterator_357 {
        unimplemented!()
    }
}
impl __normal_iterator_357Impl for Ptr<__normal_iterator_357> {}
pub trait __normal_iterator_358Impl {
    fn base(&self) -> Ptr<Ptr<Ptr<i32>>> {
        unimplemented!()
    }
}
impl __normal_iterator_358Impl for Ptr<__normal_iterator_358> {}
pub trait __normal_iterator_359Impl {
    fn operator_deref(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn operator_sub(&self, mut __n: i64) -> __normal_iterator_359 {
        unimplemented!()
    }
}
impl __normal_iterator_359Impl for Ptr<__normal_iterator_359> {}
pub trait __normal_iterator_360Impl {
    fn base(&self) -> Ptr<Ptr<Ptr<i32>>> {
        unimplemented!()
    }
}
impl __normal_iterator_360Impl for Ptr<__normal_iterator_360> {}
pub trait __normal_iterator_361Impl {
    fn operator_deref(&self) -> Ptr<f64> {
        unimplemented!()
    }
    fn operator_sub(&self, mut __n: i64) -> __normal_iterator_361 {
        unimplemented!()
    }
}
impl __normal_iterator_361Impl for Ptr<__normal_iterator_361> {}
pub trait __normal_iterator_362Impl {
    fn base(&self) -> Ptr<Ptr<f64>> {
        unimplemented!()
    }
}
impl __normal_iterator_362Impl for Ptr<__normal_iterator_362> {}
pub trait initializer_list_407Impl {
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn begin(&self) -> Ptr<bool> {
        unimplemented!()
    }
}
impl initializer_list_407Impl for Ptr<initializer_list_407> {}
pub trait integral_constant_0Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl integral_constant_0Impl for Ptr<integral_constant_0> {}
pub trait pair_326Impl {
    fn copy_assign(&self, __p: Ptr<pair_326>) -> Ptr<pair_326> {
        unimplemented!()
    }
    fn move_assign(&self, __p: Ptr<pair_326>) -> Ptr<pair_326> {
        unimplemented!()
    }
}
impl pair_326Impl for Ptr<pair_326> {}
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
pub trait vector_418Impl {
    fn copy_assign(&self, __x: Ptr<vector_418>) -> Ptr<vector_418> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_418>) -> Ptr<vector_418> {
        unimplemented!()
    }
    fn begin_26(&self) -> __normal_iterator_355 {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_356 {
        unimplemented!()
    }
    fn end_28(&self) -> __normal_iterator_355 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_356 {
        unimplemented!()
    }
    fn cbegin(&self) -> __normal_iterator_356 {
        unimplemented!()
    }
    fn cend(&self) -> __normal_iterator_356 {
        unimplemented!()
    }
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn resize_40(&self, mut __new_size: u64) {
        unimplemented!()
    }
    fn capacity(&self) -> u64 {
        unimplemented!()
    }
    fn empty(&self) -> bool {
        unimplemented!()
    }
    fn reserve(&self, mut __n: u64) {
        unimplemented!()
    }
    fn operator_index_46(&self, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
    fn at_49(&self, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
    fn back_53(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn data_55(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn push_back_58(&self, __x: Ptr<i32>) {
        unimplemented!()
    }
    fn pop_back(&self) {
        unimplemented!()
    }
    fn insert_63(&self, __position: __normal_iterator_356, __x: Ptr<i32>) -> __normal_iterator_355 {
        unimplemented!()
    }
    fn erase_67(&self, __position: __normal_iterator_356) -> __normal_iterator_355 {
        unimplemented!()
    }
    fn emplace_back_419(&self, __args: Ptr<i32>) -> Ptr<i32> {
        unimplemented!()
    }
}
impl vector_418Impl for Ptr<vector_418> {}
pub trait vector_420Impl {
    fn copy_assign(&self, __x: Ptr<vector_420>) -> Ptr<vector_420> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_420>) -> Ptr<vector_420> {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_358 {
        unimplemented!()
    }
    fn end_28(&self) -> __normal_iterator_357 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_358 {
        unimplemented!()
    }
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn empty(&self) -> bool {
        unimplemented!()
    }
    fn operator_index_46(&self, mut __n: u64) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn back_53(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl vector_420Impl for Ptr<vector_420> {}
pub trait vector_421Impl {
    fn copy_assign(&self, __x: Ptr<vector_421>) -> Ptr<vector_421> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_421>) -> Ptr<vector_421> {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_360 {
        unimplemented!()
    }
    fn end_28(&self) -> __normal_iterator_359 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_360 {
        unimplemented!()
    }
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn empty(&self) -> bool {
        unimplemented!()
    }
    fn operator_index_46(&self, mut __n: u64) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn back_53(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl vector_421Impl for Ptr<vector_421> {}
pub trait vector_422Impl {
    fn copy_assign(&self, __x: Ptr<vector_422>) -> Ptr<vector_422> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_422>) -> Ptr<vector_422> {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_362 {
        unimplemented!()
    }
    fn end_28(&self) -> __normal_iterator_361 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_362 {
        unimplemented!()
    }
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn empty(&self) -> bool {
        unimplemented!()
    }
    fn operator_index_46(&self, mut __n: u64) -> Ptr<f64> {
        unimplemented!()
    }
    fn at_49(&self, mut __n: u64) -> Ptr<f64> {
        unimplemented!()
    }
    fn back_53(&self) -> Ptr<f64> {
        unimplemented!()
    }
    fn data_55(&self) -> Ptr<f64> {
        unimplemented!()
    }
}
impl vector_422Impl for Ptr<vector_422> {}
pub trait vector_423Impl {
    fn copy_assign(&self, __x: Ptr<vector_423>) -> Ptr<vector_423> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_423>) -> Ptr<vector_423> {
        unimplemented!()
    }
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn operator_index_46(&self, mut __n: u64) -> Ptr<pair_326> {
        unimplemented!()
    }
}
impl vector_423Impl for Ptr<vector_423> {}
pub fn __cpp2rust_init_globals() {}
