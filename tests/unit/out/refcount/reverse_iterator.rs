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
pub struct __and__42 {}
impl Clone for __and__42 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__42 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__43 {}
impl Clone for __and__43 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__43 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__44 {}
impl Clone for __and__44 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__44 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__45 {}
impl Clone for __and__45 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__45 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__46 {}
impl Clone for __and__46 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__46 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__47 {}
impl Clone for __and__47 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__47 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__48 {}
impl Clone for __not__48 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__48 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__49 {}
impl Clone for __not__49 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__49 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__50 {}
impl Clone for __not__50 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__50 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__51 {}
impl Clone for __not__51 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__51 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__52 {}
impl Clone for __not__52 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__52 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__53 {}
impl Clone for __not__53 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__53 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_54 {}
impl Clone for is_reference_54 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_54 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_55 {}
impl Clone for is_reference_55 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_55 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_56 {}
impl Clone for is_reference_56 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_56 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_57 {}
impl Clone for is_reference_57 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_57 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_58 {}
impl Clone for is_reference_58 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_58 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_59 {}
impl Clone for is_reference_59 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_59 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_60 {}
impl Clone for is_reference_60 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_60 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_61 {}
impl Clone for is_reference_61 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_61 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_62 {}
impl Clone for is_function_62 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_62 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_63 {}
impl Clone for is_function_63 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_63 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_64 {}
impl Clone for is_function_64 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_64 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_65 {}
impl Clone for is_function_65 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_65 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_66 {}
impl Clone for is_function_66 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_66 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_67 {}
impl Clone for is_function_67 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_67 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_68 {}
impl Clone for is_function_68 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_68 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_69 {}
impl Clone for is_function_69 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_69 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_70 {}
impl Clone for is_void_70 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_70 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_71 {}
impl Clone for is_void_71 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_71 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_72 {}
impl Clone for is_void_72 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_72 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_73 {}
impl Clone for is_void_73 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_73 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_74 {}
impl Clone for is_void_74 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_74 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_75 {}
impl Clone for is_void_75 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_75 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_76 {}
impl Clone for is_void_76 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_76 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_77 {}
impl Clone for is_void_77 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_77 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_78 {}
impl Clone for is_void_78 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_78 {
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
pub struct is_void_86 {}
impl Clone for is_void_86 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_86 {
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
pub struct is_const_93 {}
impl Clone for is_const_93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_93 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_94 {}
impl Clone for is_const_94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_94 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_95 {}
impl Clone for is_const_95 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_95 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_96 {}
impl Clone for is_const_96 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_96 {
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
pub struct __is_array_unknown_bounds_98 {}
impl Clone for __is_array_unknown_bounds_98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_98 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_99 {}
impl Clone for __is_array_unknown_bounds_99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_99 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_100 {}
impl Clone for __is_array_unknown_bounds_100 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_100 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_101 {}
impl Clone for __is_array_unknown_bounds_101 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_101 {
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
pub struct __is_integral_helper_106 {}
impl Clone for __is_integral_helper_106 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_106 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_107 {}
impl Clone for __is_integral_helper_107 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_107 {
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
pub struct is_integral_111 {}
impl Clone for is_integral_111 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_111 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_112 {}
impl Clone for is_integral_112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_112 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_113 {}
impl Clone for is_integral_113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_113 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_114 {}
impl Clone for is_integral_114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_114 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_115 {}
impl Clone for is_integral_115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_115 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_116 {}
impl Clone for is_same_116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_116 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_117 {}
impl Clone for is_same_117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_117 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_118 {}
impl Clone for is_same_118 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_118 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_119 {}
impl Clone for is_same_119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_119 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_120 {}
impl Clone for is_same_120 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_120 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_121 {}
impl Clone for is_same_121 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_121 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_122 {}
impl Clone for is_same_122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_122 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_123 {}
impl Clone for is_same_123 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_123 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_124 {}
impl Clone for is_same_124 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_124 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_125 {}
impl Clone for is_same_125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_125 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_126 {}
impl Clone for is_same_126 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_126 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_127 {}
impl Clone for is_same_127 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_127 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_128 {}
impl Clone for is_same_128 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_128 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_129 {}
impl Clone for is_same_129 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_129 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_130 {}
impl Clone for is_same_130 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_130 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_131 {}
impl Clone for is_same_131 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_131 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_132 {}
impl Clone for is_same_132 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_132 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_133 {}
impl Clone for is_same_133 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_133 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_134 {}
impl Clone for is_same_134 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_134 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_135 {}
impl Clone for is_volatile_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_135 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_136 {}
impl Clone for is_volatile_136 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_136 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_137 {}
impl Clone for is_volatile_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_137 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_138 {}
impl Clone for is_volatile_138 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_138 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_139 {}
impl Clone for is_volatile_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_139 {
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
pub struct is_move_constructible_140 {}
impl Clone for is_move_constructible_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_140 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_141 {}
impl Clone for is_nothrow_default_constructible_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_142 {}
impl Clone for is_nothrow_default_constructible_142 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_142 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_143 {}
impl Clone for is_nothrow_copy_constructible_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_143 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_144 {}
impl Clone for is_nothrow_copy_constructible_144 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_144 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_145 {}
impl Clone for is_nothrow_copy_constructible_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_145 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_146 {}
impl Clone for is_nothrow_copy_constructible_146 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_146 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_147 {}
impl Clone for is_nothrow_copy_constructible_147 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_147 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_constructible_148 {}
impl Clone for is_nothrow_move_constructible_148 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_148 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_149 {}
impl Clone for is_move_assignable_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_assignable_150 {}
impl Clone for is_nothrow_move_assignable_150 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_150 {
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
pub struct is_convertible_151 {}
impl Clone for is_convertible_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_152 {}
impl Clone for is_convertible_152 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_152 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_153 {}
impl Clone for is_convertible_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_154 {}
impl Clone for is_convertible_154 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_154 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_155 {}
impl Clone for is_convertible_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_156 {}
impl Clone for is_convertible_156 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_156 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_157 {}
impl Clone for is_convertible_157 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_157 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_158 {}
impl Clone for is_convertible_158 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_158 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_159 {}
impl Clone for is_convertible_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_159 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_160 {}
impl Clone for is_convertible_160 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_160 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_161 {}
impl Clone for remove_reference_161 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_161 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_162 {}
impl Clone for remove_reference_162 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_162 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_163 {}
impl Clone for remove_reference_163 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_163 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_164 {}
impl Clone for remove_reference_164 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_164 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_165 {}
impl Clone for remove_reference_165 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_165 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_166 {}
impl Clone for remove_reference_166 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_166 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_167 {}
impl Clone for remove_reference_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_167 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_168 {}
impl Clone for remove_reference_168 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_168 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_169 {}
impl Clone for remove_reference_169 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_169 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_170 {}
impl Clone for __cv_selector_170 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_170 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_171 {}
impl Clone for __cv_selector_171 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_171 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_172 {}
impl Clone for __cv_selector_172 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_172 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_173 {}
impl Clone for __cv_selector_173 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_173 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_174 {}
impl Clone for __match_cv_qualifiers_174 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_174 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_175 {}
impl Clone for __match_cv_qualifiers_175 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_175 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_176 {}
impl Clone for __match_cv_qualifiers_176 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_176 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_177 {}
impl Clone for __match_cv_qualifiers_177 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_177 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_178 {}
impl Clone for __match_cv_qualifiers_178 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_178 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_179 {}
impl Clone for __make_unsigned_179 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_179 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_180 {}
impl Clone for __make_unsigned_180 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_180 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_181 {}
impl Clone for __make_unsigned_181 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_181 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_182 {}
impl Clone for __make_unsigned_selector_182 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_182 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_183 {}
impl Clone for __make_unsigned_selector_183 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_183 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_184 {}
impl Clone for __make_unsigned_selector_184 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_184 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_185 {}
impl Clone for __make_unsigned_selector_185 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_185 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_186 {}
impl Clone for __make_unsigned_selector_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_187 {}
impl Clone for __make_unsigned_selector_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_187 {
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
pub struct _List_191 {}
impl Clone for _List_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_191 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_192 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_193 {}
impl Clone for _List_193 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_193 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_194 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_195 {}
impl Clone for _List_195 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_195 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_196 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_197 {}
impl Clone for _List_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_197 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_198 {}
impl Clone for _List_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_199 {}
impl Clone for _List_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_199 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_200 {}
impl Clone for __select_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_200 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_201 {}
impl Clone for __select_201 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_201 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_202 {}
impl Clone for __select_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_202 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_203 {}
impl Clone for __select_203 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_203 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_204 {}
impl Clone for __select_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_204 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_205 {}
impl Clone for __make_signed_205 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_205 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_206 {}
impl Clone for __make_signed_206 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_206 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_207 {}
impl Clone for __make_signed_selector_207 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_207 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_208 {}
impl Clone for __make_signed_selector_208 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_208 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_209 {}
impl Clone for __make_signed_selector_209 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_209 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_210 {}
impl Clone for __make_signed_selector_210 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_210 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_211 {}
impl Clone for __make_signed_selector_211 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_211 {
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
pub struct common_type_212 {}
impl Clone for common_type_212 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_212 {
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
pub struct __is_tuple_like_impl_213 {}
impl Clone for __is_tuple_like_impl_213 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_213 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_214 {}
impl Clone for __is_tuple_like_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_214 {
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
    pub static is_array_v_215: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_copyable_v_216: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_default_constructible_v_217: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_standard_layout_v_218: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_219: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_array_v_220: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_copyable_v_221: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_default_constructible_v_222: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_standard_layout_v_223: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_224: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_array_v_225: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_copyable_v_226: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_default_constructible_v_227: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_standard_layout_v_228: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_229: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_array_v_230: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_copyable_v_231: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_trivially_default_constructible_v_232: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_standard_layout_v_233: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_234: Value<bool> = unimplemented!();
);
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
pub struct iterator_235 {}
impl Clone for iterator_235 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_235 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_236 {}
impl Clone for iterator_236 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_236 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_236 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_236>) -> Ptr<iterator_236> {
        unimplemented!()
    }
}
impl iterator_236 {
    pub fn move_assign(&self, _a0: Ptr<iterator_236>) -> Ptr<iterator_236> {
        unimplemented!()
    }
}
impl iterator_236 {}
impl iterator_236 {}
impl iterator_236 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_237 {}
impl Clone for iterator_traits_237 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_237 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_238 {}
impl Clone for iterator_traits_238 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_238 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_239 {}
impl Clone for iterator_traits_239 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_239 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_240 {}
impl Clone for iterator_traits_240 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_240 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_241 {}
impl Clone for iterator_traits_241 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_241 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_242 {}
impl Clone for iterator_traits_242 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_242 {
    fn default() -> Self {
        unimplemented!()
    }
}
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
pub struct __are_same_243 {}
impl Clone for __are_same_243 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_243 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_244 = u32;
pub const anon_244___value: anon_244 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_245 {}
impl Clone for __are_same_245 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_245 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_246 = u32;
pub const anon_246___value: anon_246 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_247 {}
impl Clone for __are_same_247 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_247 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_248 = u32;
pub const anon_248___value: anon_248 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_249 {}
impl Clone for __are_same_249 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_249 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_250 = u32;
pub const anon_250___value: anon_250 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_251 {}
impl Clone for __are_same_251 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_251 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_252 = u32;
pub const anon_252___value: anon_252 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_253 {}
impl Clone for __are_same_253 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_253 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_254 = u32;
pub const anon_254___value: anon_254 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_255 {}
impl Clone for __is_integer_255 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_255 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_256 = u32;
pub const anon_256___value: anon_256 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_257 {}
impl Clone for __is_integer_257 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_257 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_258 = u32;
pub const anon_258___value: anon_258 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_259 {}
impl Clone for __is_integer_259 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_259 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_260 = u32;
pub const anon_260___value: anon_260 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_261 {}
impl Clone for __is_integer_261 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_261 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_262 = u32;
pub const anon_262___value: anon_262 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_263 {}
impl Clone for __is_integer_263 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_263 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_264 = u32;
pub const anon_264___value: anon_264 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_265 {}
impl Clone for __is_integer_265 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_265 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_266 = u32;
pub const anon_266___value: anon_266 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_267 {}
impl Clone for __is_integer_267 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_267 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_268 = u32;
pub const anon_268___value: anon_268 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_269 {}
impl Clone for __is_integer_269 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_269 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_270 = u32;
pub const anon_270___value: anon_270 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_271 {}
impl Clone for __memcpyable_integer_271 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_271 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_272 {}
impl Clone for __ptr_traits_ptr_to_272 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_272 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_272 {
    pub fn pointer_to(__r: Ptr<i8>) -> Ptr<i8> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_273 {}
impl Clone for __ptr_traits_ptr_to_273 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_273 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_273 {
    pub fn pointer_to(__r: Ptr<i8>) -> Ptr<i8> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_274 {}
impl Clone for __ptr_traits_ptr_to_274 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_274 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_274 {
    pub fn pointer_to(__r: Ptr<i32>) -> Ptr<i32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_275 {}
impl Clone for __ptr_traits_ptr_to_275 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_275 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_275 {
    pub fn pointer_to(__r: Ptr<i32>) -> Ptr<i32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_276 {}
impl Clone for __ptr_traits_ptr_to_276 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_276 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_276 {
    pub fn pointer_to(__r: Ptr<u16>) -> Ptr<u16> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_277 {}
impl Clone for __ptr_traits_ptr_to_277 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_277 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_277 {
    pub fn pointer_to(__r: Ptr<u16>) -> Ptr<u16> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_278 {}
impl Clone for __ptr_traits_ptr_to_278 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_278 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_278 {
    pub fn pointer_to(__r: Ptr<u32>) -> Ptr<u32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_279 {}
impl Clone for __ptr_traits_ptr_to_279 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_279 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_279 {
    pub fn pointer_to(__r: Ptr<u32>) -> Ptr<u32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_280 {}
impl Clone for pointer_traits_280 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_280 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_281 {}
impl Clone for pointer_traits_281 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_281 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_282 {}
impl Clone for pointer_traits_282 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_282 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_283 {}
impl Clone for pointer_traits_283 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_283 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_284 {}
impl Clone for pointer_traits_284 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_284 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_285 {}
impl Clone for pointer_traits_285 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_285 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_286 {}
impl Clone for pointer_traits_286 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_286 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_287 {}
impl Clone for pointer_traits_287 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_287 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct reverse_iterator_288 {}
impl Clone for reverse_iterator_288 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_288 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_288 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_288 {
    pub fn new_2(mut __x: Ptr<Foo>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_288 {
    pub fn copy_from(__x: Ptr<reverse_iterator_288>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_288 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_288>) -> Ptr<reverse_iterator_288> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_289 {}
impl Clone for __normal_iterator_289 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_289 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_289 {}
impl __normal_iterator_289 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_290 {}
impl Clone for allocator_290 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_290 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_290 {}
impl allocator_290 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_291 {}
impl Clone for allocator_291 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_291 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_291 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_291 {
    pub fn copy_from(__a: Ptr<allocator_291>) -> Self {
        unimplemented!()
    }
}
impl allocator_291 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_291>) -> Ptr<allocator_291> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_292 {}
impl Clone for allocator_292 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_292 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_292 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_292 {
    pub fn copy_from(__a: Ptr<allocator_292>) -> Self {
        unimplemented!()
    }
}
impl allocator_292 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_292>) -> Ptr<allocator_292> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_293 {}
impl Clone for allocator_293 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_293 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_293 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_293 {
    pub fn copy_from(__a: Ptr<allocator_293>) -> Self {
        unimplemented!()
    }
}
impl allocator_293 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_293>) -> Ptr<allocator_293> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_294 {}
impl Clone for allocator_294 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_294 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_294 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_294 {
    pub fn copy_from(__a: Ptr<allocator_294>) -> Self {
        unimplemented!()
    }
}
impl allocator_294 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_294>) -> Ptr<allocator_294> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_295 {}
impl Clone for allocator_traits_295 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_295 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_295 {
    pub fn allocate_1(__a: Ptr<allocator_291>, mut __n: u64) -> Ptr<i8> {
        unimplemented!()
    }
}
impl allocator_traits_295 {
    pub fn deallocate(__a: Ptr<allocator_291>, mut __p: Ptr<i8>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_295 {
    pub fn max_size(__a: Ptr<allocator_291>) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_296 {}
impl Clone for allocator_traits_296 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_296 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_296 {
    pub fn allocate_1(__a: Ptr<allocator_292>, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
}
impl allocator_traits_296 {
    pub fn deallocate(__a: Ptr<allocator_292>, mut __p: Ptr<i32>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_296 {
    pub fn max_size(__a: Ptr<allocator_292>) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_297 {}
impl Clone for allocator_traits_297 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_297 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_297 {
    pub fn allocate_1(__a: Ptr<allocator_293>, mut __n: u64) -> Ptr<u16> {
        unimplemented!()
    }
}
impl allocator_traits_297 {
    pub fn deallocate(__a: Ptr<allocator_293>, mut __p: Ptr<u16>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_297 {
    pub fn max_size(__a: Ptr<allocator_293>) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_298 {}
impl Clone for allocator_traits_298 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_298 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_298 {
    pub fn allocate_1(__a: Ptr<allocator_294>, mut __n: u64) -> Ptr<u32> {
        unimplemented!()
    }
}
impl allocator_traits_298 {
    pub fn deallocate(__a: Ptr<allocator_294>, mut __p: Ptr<u32>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_298 {
    pub fn max_size(__a: Ptr<allocator_294>) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_299 {}
impl Clone for char_traits_299 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_299 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_299 {
    pub fn assign_1(__c1: Ptr<i8>, __c2: Ptr<i8>) {
        unimplemented!()
    }
}
impl char_traits_299 {
    pub fn lt(__c1: Ptr<i8>, __c2: Ptr<i8>) -> bool {
        unimplemented!()
    }
}
impl char_traits_299 {
    pub fn length(mut __s: Ptr<i8>) -> usize {
        unimplemented!()
    }
}
impl char_traits_299 {
    pub fn move_(mut __s1: Ptr<i8>, mut __s2: Ptr<i8>, mut __n: usize) -> Ptr<i8> {
        unimplemented!()
    }
}
impl char_traits_299 {
    pub fn copy(mut __s1: Ptr<i8>, mut __s2: Ptr<i8>, mut __n: usize) -> Ptr<i8> {
        unimplemented!()
    }
}
impl char_traits_299 {
    pub fn eof() -> i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_300 {}
impl Clone for char_traits_300 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_300 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_300 {
    pub fn assign_1(__c1: Ptr<i32>, __c2: Ptr<i32>) {
        unimplemented!()
    }
}
impl char_traits_300 {
    pub fn copy(mut __s1: Ptr<i32>, mut __s2: Ptr<i32>, mut __n: usize) -> Ptr<i32> {
        unimplemented!()
    }
}
impl char_traits_300 {
    pub fn eq_int_type(__c1: Ptr<u32>, __c2: Ptr<u32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_300 {
    pub fn eof() -> u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_301 {}
impl Clone for char_traits_301 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_301 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_301 {
    pub fn assign_1(__c1: Ptr<u16>, __c2: Ptr<u16>) {
        unimplemented!()
    }
}
impl char_traits_301 {
    pub fn eq(__c1: Ptr<u16>, __c2: Ptr<u16>) -> bool {
        unimplemented!()
    }
}
impl char_traits_301 {
    pub fn lt(__c1: Ptr<u16>, __c2: Ptr<u16>) -> bool {
        unimplemented!()
    }
}
impl char_traits_301 {
    pub fn copy(mut __s1: Ptr<u16>, mut __s2: Ptr<u16>, mut __n: usize) -> Ptr<u16> {
        unimplemented!()
    }
}
impl char_traits_301 {
    pub fn eq_int_type(__c1: Ptr<u16>, __c2: Ptr<u16>) -> bool {
        unimplemented!()
    }
}
impl char_traits_301 {
    pub fn eof() -> u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_302 {}
impl Clone for char_traits_302 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_302 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_302 {
    pub fn assign_1(__c1: Ptr<u32>, __c2: Ptr<u32>) {
        unimplemented!()
    }
}
impl char_traits_302 {
    pub fn eq(__c1: Ptr<u32>, __c2: Ptr<u32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_302 {
    pub fn lt(__c1: Ptr<u32>, __c2: Ptr<u32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_302 {
    pub fn copy(mut __s1: Ptr<u32>, mut __s2: Ptr<u32>, mut __n: usize) -> Ptr<u32> {
        unimplemented!()
    }
}
impl char_traits_302 {
    pub fn eq_int_type(__c1: Ptr<u32>, __c2: Ptr<u32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_302 {
    pub fn eof() -> u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct basic_string_303 {}
impl Clone for basic_string_303 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_303 {
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
    pub fn new_1(mut __dat: Ptr<i8>, __a: Ptr<allocator_291>) -> Self {
        unimplemented!()
    }
}
impl std_basic_string_char___Alloc_hider {
    pub fn new_2(mut __dat: Ptr<i8>, __a: Option<Ptr<allocator_291>>) -> Self {
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
pub type anon_305 = u32;
pub const anon_305__S_local_capacity: anon_305 = 15;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_304 {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_local_buf: Value<Box<[i8]>>,
    #[offset(0)]
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_304 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_304 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_303 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl basic_string_303 {
    pub fn copy_from(__str: Ptr<basic_string_303>) -> Self {
        unimplemented!()
    }
}
impl basic_string_303 {
    pub fn new_2(mut __s: Ptr<i8>, mut __n: u64, __a: Option<Ptr<allocator_291>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_303 {
    pub fn move_from(__str: Ptr<basic_string_303>) -> Self {
        unimplemented!()
    }
}
impl basic_string_303 {
    pub fn new_3(__beg: Ptr<i8>, mut __end: Ptr<i8>, __a: Option<Ptr<allocator_291>>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct basic_string_313 {}
impl Clone for basic_string_313 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_313 {
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
    pub fn new_1(mut __dat: Ptr<i32>, __a: Ptr<allocator_292>) -> Self {
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
pub type anon_315 = u32;
pub const anon_315__S_local_capacity: anon_315 = 3;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_314 {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_local_buf: Value<Box<[i32]>>,
    #[offset(0)]
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_314 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_314 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_313 {
    pub fn copy_from(__str: Ptr<basic_string_313>) -> Self {
        unimplemented!()
    }
}
impl basic_string_313 {
    pub fn new_1(mut __s: Ptr<i32>, mut __n: u64, __a: Option<Ptr<allocator_292>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_313 {
    pub fn move_from(__str: Ptr<basic_string_313>) -> Self {
        unimplemented!()
    }
}
impl basic_string_313 {
    pub fn new_2(__beg: Ptr<i8>, mut __end: Ptr<i8>, __a: Option<Ptr<allocator_292>>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct basic_string_316 {}
impl Clone for basic_string_316 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_316 {
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
    pub fn new_1(mut __dat: Ptr<u16>, __a: Ptr<allocator_293>) -> Self {
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
pub type anon_318 = u32;
pub const anon_318__S_local_capacity: anon_318 = 7;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_317 {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_local_buf: Value<Box<[u16]>>,
    #[offset(0)]
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_317 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_317 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_316 {
    pub fn copy_from(__str: Ptr<basic_string_316>) -> Self {
        unimplemented!()
    }
}
impl basic_string_316 {
    pub fn new_1(mut __s: Ptr<u16>, mut __n: u64, __a: Option<Ptr<allocator_293>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_316 {
    pub fn move_from(__str: Ptr<basic_string_316>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct basic_string_319 {}
impl Clone for basic_string_319 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_319 {
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
    pub fn new_1(mut __dat: Ptr<u32>, __a: Ptr<allocator_294>) -> Self {
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
pub type anon_321 = u32;
pub const anon_321__S_local_capacity: anon_321 = 3;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct anon_320 {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_local_buf: Value<Box<[u32]>>,
    #[offset(0)]
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_320 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_320 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_319 {
    pub fn copy_from(__str: Ptr<basic_string_319>) -> Self {
        unimplemented!()
    }
}
impl basic_string_319 {
    pub fn new_1(mut __s: Ptr<u32>, mut __n: u64, __a: Option<Ptr<allocator_294>>) -> Self {
        unimplemented!()
    }
}
impl basic_string_319 {
    pub fn move_from(__str: Ptr<basic_string_319>) -> Self {
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
    pub __value: anon_322,
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
pub struct anon_322 {
    #[offset(0)]
    pub __wch: u32,
    #[offset(0)]
    #[byte_size(4)]
    pub __wchb: Value<Box<[i8]>>,
}
impl Clone for anon_322 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_322 {
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(64)]
pub struct basic_streambuf_323 {}
impl Clone for basic_streambuf_323 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_streambuf_323 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(64)]
pub struct basic_streambuf_324 {}
impl Clone for basic_streambuf_324 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_streambuf_324 {
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
#[byte_size(24)]
pub struct collate_325 {}
impl Clone for collate_325 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for collate_325 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct collate_326 {}
impl Clone for collate_326 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for collate_326 {
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
    pub anon_327: i32,
    #[offset(168)]
    pub anon_328: i32,
    #[offset(172)]
    pub anon_329: i32,
    #[offset(176)]
    pub anon_330: i32,
    #[offset(180)]
    pub anon_331: i32,
    #[offset(184)]
    pub anon_332: i32,
    #[offset(188)]
    pub anon_333: i32,
    #[offset(192)]
    pub anon_334: i32,
    #[offset(196)]
    pub anon_335: i32,
    #[offset(200)]
    pub anon_336: i32,
    #[offset(204)]
    pub anon_337: i32,
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
    pub __value32: anon_338,
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
pub struct anon_338 {
    #[offset(0)]
    pub __low: u32,
    #[offset(4)]
    pub __high: u32,
}
impl Clone for anon_338 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_338 {
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
pub type anon_339 = u32;
pub const anon_339_PTHREAD_MUTEX_TIMED_NP: anon_339 = 0;
pub const anon_339_PTHREAD_MUTEX_RECURSIVE_NP: anon_339 = 1;
pub const anon_339_PTHREAD_MUTEX_ERRORCHECK_NP: anon_339 = 2;
pub const anon_339_PTHREAD_MUTEX_ADAPTIVE_NP: anon_339 = 3;
pub const anon_339_PTHREAD_MUTEX_NORMAL: anon_339 = 0;
pub const anon_339_PTHREAD_MUTEX_RECURSIVE: anon_339 = 1;
pub const anon_339_PTHREAD_MUTEX_ERRORCHECK: anon_339 = 2;
pub const anon_339_PTHREAD_MUTEX_DEFAULT: anon_339 = 0;
pub const anon_339_PTHREAD_MUTEX_FAST_NP: anon_339 = 0;
pub type anon_340 = u32;
pub const anon_340_PTHREAD_MUTEX_STALLED: anon_340 = 0;
pub const anon_340_PTHREAD_MUTEX_STALLED_NP: anon_340 = 0;
pub const anon_340_PTHREAD_MUTEX_ROBUST: anon_340 = 1;
pub const anon_340_PTHREAD_MUTEX_ROBUST_NP: anon_340 = 1;
pub type anon_341 = u32;
pub const anon_341_PTHREAD_RWLOCK_PREFER_READER_NP: anon_341 = 0;
pub const anon_341_PTHREAD_RWLOCK_PREFER_WRITER_NP: anon_341 = 1;
pub const anon_341_PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: anon_341 = 2;
pub const anon_341_PTHREAD_RWLOCK_DEFAULT_NP: anon_341 = 0;
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
pub type anon_342 = u32;
pub const anon_342_PTHREAD_CANCEL_DEFERRED: anon_342 = 0;
pub const anon_342_PTHREAD_CANCEL_ASYNCHRONOUS: anon_342 = 1;
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
    pub static __libc_single_threaded_343: Value<i8> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Char_types_344 {}
impl Clone for _Char_types_344 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Char_types_344 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Char_types_345 {}
impl Clone for _Char_types_345 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Char_types_345 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_346 {}
impl Clone for char_traits_346 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_346 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_346 {
    pub fn eq(__c1: Ptr<i8>, __c2: Ptr<i8>) -> bool {
        unimplemented!()
    }
}
impl char_traits_346 {
    pub fn length(mut __p: Ptr<i8>) -> usize {
        unimplemented!()
    }
}
impl char_traits_346 {
    pub fn find(mut __s: Ptr<i8>, mut __n: usize, __a: Ptr<i8>) -> Ptr<i8> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct char_traits_347 {}
impl Clone for char_traits_347 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_347 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_347 {
    pub fn eq(__c1: Ptr<i32>, __c2: Ptr<i32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_347 {
    pub fn lt(__c1: Ptr<i32>, __c2: Ptr<i32>) -> bool {
        unimplemented!()
    }
}
impl char_traits_347 {
    pub fn compare(mut __s1: Ptr<i32>, mut __s2: Ptr<i32>, mut __n: usize) -> i32 {
        unimplemented!()
    }
}
impl char_traits_347 {
    pub fn length(mut __p: Ptr<i32>) -> usize {
        unimplemented!()
    }
}
impl char_traits_347 {
    pub fn find(mut __s: Ptr<i32>, mut __n: usize, __a: Ptr<i32>) -> Ptr<i32> {
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
pub struct __new_allocator_348 {}
impl Clone for __new_allocator_348 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_348 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_348 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_348 {
    pub fn copy_from(_a0: Ptr<__new_allocator_348>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_348 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_348>) -> Ptr<__new_allocator_348> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_349 {}
impl Clone for __new_allocator_349 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_349 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_349 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_349 {
    pub fn copy_from(_a0: Ptr<__new_allocator_349>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_349 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_349>) -> Ptr<__new_allocator_349> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_350 {}
impl Clone for __new_allocator_350 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_350 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_350 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_350 {
    pub fn copy_from(_a0: Ptr<__new_allocator_350>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_350 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_350>) -> Ptr<__new_allocator_350> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_351 {}
impl Clone for __new_allocator_351 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_351 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_351 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_351 {
    pub fn copy_from(_a0: Ptr<__new_allocator_351>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_351 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_351>) -> Ptr<__new_allocator_351> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct binary_function_352 {}
impl Clone for binary_function_352 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_352 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct binary_function_353 {}
impl Clone for binary_function_353 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_353 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl binary_function_353 {}
impl binary_function_353 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct binary_function_354 {}
impl Clone for binary_function_354 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_354 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl binary_function_354 {}
impl binary_function_354 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_355 {}
impl Clone for greater_355 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_355 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_356 {}
impl Clone for less_356 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_356 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_357 {}
impl Clone for less_357 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_357 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl less_357 {}
impl less_357 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_358 {}
impl Clone for less_358 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_358 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl less_358 {}
impl less_358 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_equal_359 {}
impl Clone for greater_equal_359 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_359 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_equal_360 {}
impl Clone for less_equal_360 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_360 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_361 {}
impl Clone for __is_integer_nonstrict_361 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_361 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_362 = u32;
pub const anon_362___width: anon_362 = 32;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_363 {}
impl Clone for __is_integer_nonstrict_363 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_363 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_364 = u32;
pub const anon_364___width: anon_364 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_365 {}
impl Clone for __is_integer_nonstrict_365 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_365 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_366 = u32;
pub const anon_366___width: anon_366 = 32;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_367 {}
impl Clone for __is_integer_nonstrict_367 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_367 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_368 = u32;
pub const anon_368___width: anon_368 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_369 {}
impl Clone for __is_integer_nonstrict_369 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_369 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_370 = u32;
pub const anon_370___width: anon_370 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_371 {}
impl Clone for __numeric_traits_integer_371 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_371 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_int____is_signed_372: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_int____digits_373: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_int____max_374: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_int____min_375: Value<i32> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_376 {}
impl Clone for __numeric_traits_integer_376 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_376 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____is_signed_377: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____digits_378: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____max_379: Value<i64> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_380 {}
impl Clone for __numeric_traits_integer_380 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_380 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_int____is_signed_381: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_int____digits_382: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_int____max_383: Value<u32> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_384 {}
impl Clone for __numeric_traits_integer_384 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_384 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_385: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____digits_386: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____max_387: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_388 {}
impl Clone for __numeric_traits_integer_388 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_388 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long_long____is_signed_389: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long_long____digits_390: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long_long____max_391: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_392 {}
impl Clone for __numeric_traits_floating_392 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_392 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_floating_float____max_exponent10_393: Value<i32> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_394 {}
impl Clone for __numeric_traits_floating_394 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_394 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_floating_double____max_exponent10_395: Value<i32> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_396 {}
impl Clone for __numeric_traits_floating_396 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_396 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_floating_long_double____max_exponent10_397: Value<i32> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_398 {}
impl Clone for __numeric_traits_398 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_398 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_399 {}
impl Clone for __numeric_traits_399 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_399 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_400 {}
impl Clone for __numeric_traits_400 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_400 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_401 {}
impl Clone for __numeric_traits_401 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_401 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_402 {}
impl Clone for __numeric_traits_402 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_402 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_403 {}
impl Clone for __numeric_traits_403 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_403 {
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
    pub static piecewise_construct_404: Value<std_piecewise_construct_t> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_405 {}
impl Clone for tuple_405 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_405 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_405 {}
impl tuple_405 {}
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
#[byte_size(16)]
pub struct initializer_list_406 {}
impl Clone for initializer_list_406 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_406 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_406 {}
impl initializer_list_406 {}
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
impl initializer_list_407 {}
impl initializer_list_407 {}
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
pub struct __is_allocator_411 {}
impl Clone for __is_allocator_411 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_411 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_allocator_412 {}
impl Clone for __is_allocator_412 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_412 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_allocator_413 {}
impl Clone for __is_allocator_413 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_413 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_allocator_414 {}
impl Clone for __is_allocator_414 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_414 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_415 {}
impl Clone for __alloc_traits_415 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_415 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_416 {}
impl Clone for rebind_416 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_416 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_417 {}
impl Clone for __alloc_traits_417 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_417 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_418 {}
impl Clone for rebind_418 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_418 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_419 {}
impl Clone for __alloc_traits_419 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_419 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_420 {}
impl Clone for rebind_420 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_420 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_421 {}
impl Clone for __alloc_traits_421 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_421 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_422 {}
impl Clone for rebind_422 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_422 {
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
pub struct __hash_base_445 {}
impl Clone for __hash_base_445 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_445 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_446 {}
impl Clone for __hash_base_446 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_446 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_447 {}
impl Clone for __hash_base_447 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_447 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_448 {}
impl Clone for __hash_base_448 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_448 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_449 {}
impl Clone for __hash_base_449 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_449 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_450 {}
impl Clone for hash_450 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_450 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_451 {}
impl Clone for hash_451 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_451 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_452 {}
impl Clone for hash_452 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_452 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_453 {}
impl Clone for hash_453 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_453 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_454 {}
impl Clone for hash_454 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_454 {
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
    pub fn hash_455(__val: Ptr<f32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_456(__val: Ptr<f64>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_457(__val: Ptr<i32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn __hash_combine_458(__val: Ptr<Ptr<std_error_category>>, mut __hash: usize) -> usize {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct basic_string_view_306 {}
impl Clone for basic_string_view_306 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_306 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_306 {
    pub fn copy_from(_a0: Ptr<basic_string_view_306>) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_306 {
    pub fn new_1(mut __str: Ptr<i8>, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_306 {
    pub fn copy_assign(&self, _a0: Ptr<basic_string_view_306>) -> Ptr<basic_string_view_306> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct basic_string_view_459 {}
impl Clone for basic_string_view_459 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_459 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_459 {
    pub fn copy_from(_a0: Ptr<basic_string_view_459>) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_459 {
    pub fn new_1(mut __str: Ptr<i32>, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_459 {
    pub fn copy_assign(&self, _a0: Ptr<basic_string_view_459>) -> Ptr<basic_string_view_459> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct basic_string_view_460 {}
impl Clone for basic_string_view_460 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_460 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_460 {
    pub fn copy_from(_a0: Ptr<basic_string_view_460>) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_460 {
    pub fn new_1(mut __str: Ptr<u16>, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_460 {
    pub fn copy_assign(&self, _a0: Ptr<basic_string_view_460>) -> Ptr<basic_string_view_460> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct basic_string_view_461 {}
impl Clone for basic_string_view_461 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_461 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_461 {
    pub fn copy_from(_a0: Ptr<basic_string_view_461>) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_461 {
    pub fn new_1(mut __str: Ptr<u32>, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_461 {
    pub fn copy_assign(&self, _a0: Ptr<basic_string_view_461>) -> Ptr<basic_string_view_461> {
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
    pub static __integer_to_chars_is_unsigned_462: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __integer_to_chars_is_unsigned_463: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __integer_to_chars_is_unsigned_464: Value<bool> = unimplemented!();
);
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
    pub static allocator_arg_465: Value<std_allocator_arg_t> = unimplemented!();
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
    pub static std_locale_ctype_466: Value<i32> = unimplemented!();
);
thread_local!(
    pub static std_locale_numeric_467: Value<i32> = unimplemented!();
);
thread_local!(
    pub static std_locale_collate_468: Value<i32> = unimplemented!();
);
thread_local!(
    pub static std_locale_time_469: Value<i32> = unimplemented!();
);
thread_local!(
    pub static std_locale_monetary_470: Value<i32> = unimplemented!();
);
thread_local!(
    pub static std_locale_messages_471: Value<i32> = unimplemented!();
);
pub type anon_472 = u32;
pub const anon_472__S_categories_size: anon_472 = 12;
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
    pub anon_473: anon_473,
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
pub struct anon_473 {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_p: Ptr<i8>,
    #[offset(0)]
    #[byte_size(8)]
    pub _M_bytes: Value<Box<[i8]>>,
}
impl Clone for anon_473 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_473 {
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
    pub fn new_1(__arg: Ptr<basic_string_303>) -> Self {
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
pub struct is_error_code_enum_474 {}
impl Clone for is_error_code_enum_474 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_474 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_code_enum_475 {}
impl Clone for is_error_code_enum_475 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_475 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_code_enum_476 {}
impl Clone for is_error_code_enum_476 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_476 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_code_enum_477 {}
impl Clone for is_error_code_enum_477 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_477 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_code_enum_478 {}
impl Clone for is_error_code_enum_478 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_478 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_code_enum_479 {}
impl Clone for is_error_code_enum_479 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_479 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_condition_enum_480 {}
impl Clone for is_error_condition_enum_480 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_condition_enum_480 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_condition_enum_481 {}
impl Clone for is_error_condition_enum_481 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_condition_enum_481 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_error_condition_enum_482 {}
impl Clone for is_error_condition_enum_482 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_condition_enum_482 {
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
    pub static std_ios_base_boolalpha_483: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_dec_484: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_fixed_485: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_hex_486: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_internal_487: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_left_488: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_oct_489: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_right_490: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_scientific_491: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_showbase_492: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_showpoint_493: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_showpos_494: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_skipws_495: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_unitbuf_496: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_uppercase_497: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_adjustfield_498: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_basefield_499: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_floatfield_500: Value<std__Ios_Fmtflags> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_in_501: Value<std__Ios_Openmode> = unimplemented!();
);
thread_local!(
    pub static std_ios_base_out_502: Value<std__Ios_Openmode> = unimplemented!();
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
pub type anon_503 = u32;
pub const anon_503__S_local_word_size: anon_503 = 8;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Foo {
    #[offset(0)]
    pub v: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a1: Value<Box<[Foo]>> = Rc::new(RefCell::new(Box::new([
        Foo { v: 10 },
        Foo { v: 20 },
        Foo { v: 30 },
        Foo { v: 40 },
        Foo { v: 50 },
    ])));
    let def: Value<reverse_iterator_288> = Rc::new(RefCell::new(reverse_iterator_288::new_1()));
    let first: Value<reverse_iterator_288> = Rc::new(RefCell::new(reverse_iterator_288::new_2({
        (a1.as_pointer() as Ptr<Foo>).offset((5) as isize)
    })));
    (if (({ reverse_iterator_288Impl::base(&first.as_pointer()) })
        == (a1.as_pointer() as Ptr<Foo>).offset((5) as isize))
    {
        &(0);
    } else {
        ({
            __assert_fail_504(
                Ptr::<i8>::from_string_literal(b"first.base() == a1 + 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let ref_: Ptr<Foo> = ({ reverse_iterator_288Impl::operator_deref(&first.as_pointer()) });
    (if (({ FooImpl::get(&ref_) }) == 50) {
        &(0);
    } else {
        ({
            __assert_fail_504(
                Ptr::<i8>::from_string_literal(b"ref.get() == 50"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ FooImpl::get(&({ reverse_iterator_288Impl::operator_arrow(&first.as_pointer()) })) })
        == 50)
    {
        &(0);
    } else {
        ({
            __assert_fail_504(
                Ptr::<i8>::from_string_literal(b"first->get() == 50"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ reverse_iterator_288Impl::operator_inc_10(&first.as_pointer()) });
    (if (({ FooImpl::get(&({ reverse_iterator_288Impl::operator_arrow(&first.as_pointer()) })) })
        == 40)
    {
        &(0);
    } else {
        ({
            __assert_fail_504(
                Ptr::<i8>::from_string_literal(b"first->get() == 40"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let inc: Value<reverse_iterator_288> = Rc::new(RefCell::new(
        ({ reverse_iterator_288Impl::operator_post_inc_11(&first.as_pointer(), 0) }),
    ));
    (if (({ FooImpl::get(&({ reverse_iterator_288Impl::operator_arrow(&inc.as_pointer()) })) })
        == 40)
    {
        &(0);
    } else {
        ({
            __assert_fail_504(
                Ptr::<i8>::from_string_literal(b"inc->get() == 40"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ FooImpl::get(&({ reverse_iterator_288Impl::operator_arrow(&first.as_pointer()) })) })
        == 30)
    {
        &(0);
    } else {
        ({
            __assert_fail_504(
                Ptr::<i8>::from_string_literal(b"first->get() == 30"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut n: i64 = 2_i64;
    let plus: Value<reverse_iterator_288> = Rc::new(RefCell::new(
        ({ reverse_iterator_288Impl::operator_add(&first.as_pointer(), n) }),
    ));
    (if (({ FooImpl::get(&({ reverse_iterator_288Impl::operator_arrow(&plus.as_pointer()) })) })
        == 10)
    {
        &(0);
    } else {
        ({
            __assert_fail_504(
                Ptr::<i8>::from_string_literal(b"plus->get() == 10"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let minus: Value<reverse_iterator_288> = Rc::new(RefCell::new(
        ({ reverse_iterator_288Impl::operator_sub(&plus.as_pointer(), n) }),
    ));
    (if (({ FooImpl::get(&({ reverse_iterator_288Impl::operator_arrow(&minus.as_pointer()) })) })
        == 30)
    {
        &(0);
    } else {
        ({
            __assert_fail_504(
                Ptr::<i8>::from_string_literal(b"minus->get() == 30"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        let ___x: Ptr<reverse_iterator_288> = minus.as_pointer();
        operator_eq_505(___x, first.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_504(
                Ptr::<i8>::from_string_literal(b"minus == first"),
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
pub struct _IO_codecvt;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_marker;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct std_error_category;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_wide_data;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __locale_data;
pub fn __assert_fail_504(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn operator_eq_505(__x: Ptr<reverse_iterator_288>, __y: Ptr<reverse_iterator_288>) -> bool {
    unimplemented!()
}
pub trait FooImpl {
    fn get(&self) -> i32;
}
impl FooImpl for Ptr<Foo> {
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.v);
    }
}
pub trait __new_allocator_348Impl {
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
impl __new_allocator_348Impl for Ptr<__new_allocator_348> {}
pub trait __new_allocator_349Impl {
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
impl __new_allocator_349Impl for Ptr<__new_allocator_349> {}
pub trait __new_allocator_350Impl {
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
impl __new_allocator_350Impl for Ptr<__new_allocator_350> {}
pub trait __new_allocator_351Impl {
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
impl __new_allocator_351Impl for Ptr<__new_allocator_351> {}
pub trait basic_string_303Impl {
    fn copy_assign(&self, __str: Ptr<basic_string_303>) -> Ptr<basic_string_303> {
        unimplemented!()
    }
    fn move_assign(&self, __str: Ptr<basic_string_303>) -> Ptr<basic_string_303> {
        unimplemented!()
    }
    fn to_basic_string_view_306(&self) -> basic_string_view_306 {
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
    fn insert_119(&self, mut __pos1: u64, __str: Ptr<basic_string_303>) -> Ptr<basic_string_303> {
        unimplemented!()
    }
    fn insert_122(&self, mut __pos: u64, mut __s: Ptr<i8>) -> Ptr<basic_string_303> {
        unimplemented!()
    }
    fn replace_133(
        &self,
        mut __pos: u64,
        mut __n1: u64,
        mut __s: Ptr<i8>,
        mut __n2: u64,
    ) -> Ptr<basic_string_303> {
        unimplemented!()
    }
    fn c_str(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn data_158(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn __resize_and_overwrite_307(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
    fn __resize_and_overwrite_308(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
    fn __resize_and_overwrite_309(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
    fn __resize_and_overwrite_310(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
    fn __resize_and_overwrite_311(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
    fn __resize_and_overwrite_312(&self, mut __n: u64, __op: FnPtr<fn(Ptr<i8>, usize) -> usize>) {
        unimplemented!()
    }
}
impl basic_string_303Impl for Ptr<basic_string_303> {}
pub trait basic_string_313Impl {
    fn copy_assign(&self, __str: Ptr<basic_string_313>) -> Ptr<basic_string_313> {
        unimplemented!()
    }
    fn move_assign(&self, __str: Ptr<basic_string_313>) -> Ptr<basic_string_313> {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn c_str(&self) -> Ptr<i32> {
        unimplemented!()
    }
}
impl basic_string_313Impl for Ptr<basic_string_313> {}
pub trait basic_string_316Impl {
    fn copy_assign(&self, __str: Ptr<basic_string_316>) -> Ptr<basic_string_316> {
        unimplemented!()
    }
    fn move_assign(&self, __str: Ptr<basic_string_316>) -> Ptr<basic_string_316> {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_316Impl for Ptr<basic_string_316> {}
pub trait basic_string_319Impl {
    fn copy_assign(&self, __str: Ptr<basic_string_319>) -> Ptr<basic_string_319> {
        unimplemented!()
    }
    fn move_assign(&self, __str: Ptr<basic_string_319>) -> Ptr<basic_string_319> {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_319Impl for Ptr<basic_string_319> {}
pub trait basic_string_view_306Impl {
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
impl basic_string_view_306Impl for Ptr<basic_string_view_306> {}
pub trait basic_string_view_459Impl {
    fn length(&self) -> u64 {
        unimplemented!()
    }
    fn data(&self) -> Ptr<i32> {
        unimplemented!()
    }
}
impl basic_string_view_459Impl for Ptr<basic_string_view_459> {}
pub trait basic_string_view_460Impl {
    fn length(&self) -> u64 {
        unimplemented!()
    }
    fn data(&self) -> Ptr<u16> {
        unimplemented!()
    }
}
impl basic_string_view_460Impl for Ptr<basic_string_view_460> {}
pub trait basic_string_view_461Impl {
    fn length(&self) -> u64 {
        unimplemented!()
    }
    fn data(&self) -> Ptr<u32> {
        unimplemented!()
    }
}
impl basic_string_view_461Impl for Ptr<basic_string_view_461> {}
pub trait integral_constant_7Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl integral_constant_7Impl for Ptr<integral_constant_7> {}
pub trait less_357Impl {
    fn operator_call(
        &self,
        mut __x: Ptr<std_error_category>,
        mut __y: Ptr<std_error_category>,
    ) -> bool {
        unimplemented!()
    }
}
impl less_357Impl for Ptr<less_357> {}
pub trait less_358Impl {
    fn operator_call(&self, mut __x: Ptr<i8>, mut __y: Ptr<i8>) -> bool {
        unimplemented!()
    }
}
impl less_358Impl for Ptr<less_358> {}
pub trait reverse_iterator_288Impl {
    fn base(&self) -> Ptr<Foo> {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<Foo> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<Foo> {
        unimplemented!()
    }
    fn operator_inc_10(&self) -> Ptr<reverse_iterator_288> {
        unimplemented!()
    }
    fn operator_post_inc_11(&self, mut _a0: i32) -> reverse_iterator_288 {
        unimplemented!()
    }
    fn operator_add(&self, mut __n: i64) -> reverse_iterator_288 {
        unimplemented!()
    }
    fn operator_sub(&self, mut __n: i64) -> reverse_iterator_288 {
        unimplemented!()
    }
}
impl reverse_iterator_288Impl for Ptr<reverse_iterator_288> {}
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
    fn message(&self) -> basic_string_303 {
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
