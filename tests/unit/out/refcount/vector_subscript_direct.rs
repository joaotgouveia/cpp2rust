extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_0 {}
impl Clone for initializer_list_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_0 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_1 {}
impl Clone for initializer_list_1 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_1 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_1 {}
impl initializer_list_1 {}
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
    pub static std_integral_constant_bool__true__value_3: Value<bool> = unimplemented!();
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
thread_local!(
    pub static std_integral_constant_bool__false__value_5: Value<bool> = unimplemented!();
);
impl integral_constant_4 {}
impl integral_constant_4 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_6 {}
impl Clone for integral_constant_6 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_6 {
    fn default() -> Self {
        unimplemented!()
    }
}
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
pub struct enable_if_12 {}
impl Clone for enable_if_12 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_12 {
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
pub struct __not__55 {}
impl Clone for __not__55 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__55 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__56 {}
impl Clone for __not__56 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__56 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__57 {}
impl Clone for __not__57 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__57 {
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
pub struct is_reference_62 {}
impl Clone for is_reference_62 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_62 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_63 {}
impl Clone for is_reference_63 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_63 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_64 {}
impl Clone for is_reference_64 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_64 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_65 {}
impl Clone for is_reference_65 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_65 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_66 {}
impl Clone for is_reference_66 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_66 {
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
pub struct is_function_70 {}
impl Clone for is_function_70 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_70 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_71 {}
impl Clone for is_function_71 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_71 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_72 {}
impl Clone for is_function_72 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_72 {
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
pub struct is_void_87 {}
impl Clone for is_void_87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_88 {}
impl Clone for is_void_88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_88 {
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
pub struct remove_cv_97 {}
impl Clone for remove_cv_97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_97 {
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
pub struct is_const_102 {}
impl Clone for is_const_102 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_102 {
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
pub struct __is_array_unknown_bounds_108 {}
impl Clone for __is_array_unknown_bounds_108 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_108 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_109 {}
impl Clone for __is_array_unknown_bounds_109 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_109 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_110 {}
impl Clone for __is_array_unknown_bounds_110 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_110 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_111 {}
impl Clone for __is_array_unknown_bounds_111 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_111 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_112 {}
impl Clone for __is_array_unknown_bounds_112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_112 {
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
pub struct __is_integral_helper_119 {}
impl Clone for __is_integral_helper_119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_119 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_120 {}
impl Clone for __is_integral_helper_120 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_120 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_121 {}
impl Clone for __is_integral_helper_121 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_121 {
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
pub struct is_integral_130 {}
impl Clone for is_integral_130 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_130 {
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
pub struct is_floating_point_133 {}
impl Clone for is_floating_point_133 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_floating_point_133 {
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
pub struct is_pointer_135 {}
impl Clone for is_pointer_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_135 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_136 {}
impl Clone for is_pointer_136 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_136 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_137 {}
impl Clone for is_pointer_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_137 {
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
pub struct is_enum_140 {}
impl Clone for is_enum_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_enum_140 {
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
pub struct is_null_pointer_142 {}
impl Clone for is_null_pointer_142 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_null_pointer_142 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_null_pointer_143 {}
impl Clone for is_null_pointer_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_null_pointer_143 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_144 {}
impl Clone for is_arithmetic_144 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_144 {
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
pub struct is_member_pointer_147 {}
impl Clone for is_member_pointer_147 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_pointer_147 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_member_pointer_148 {}
impl Clone for is_member_pointer_148 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_pointer_148 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_149 {}
impl Clone for is_scalar_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_150 {}
impl Clone for is_scalar_150 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_150 {
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
pub struct is_volatile_158 {}
impl Clone for is_volatile_158 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_158 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_159 {}
impl Clone for is_volatile_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_159 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_160 {}
impl Clone for is_volatile_160 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_160 {
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
pub struct remove_all_extents_163 {}
impl Clone for remove_all_extents_163 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_all_extents_163 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_all_extents_164 {}
impl Clone for remove_all_extents_164 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_all_extents_164 {
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
pub struct __is_destructible_impl_165 {}
impl Clone for __is_destructible_impl_165 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_impl_165 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_impl_166 {}
impl Clone for __is_destructible_impl_166 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_impl_166 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_safe_167 {}
impl Clone for __is_destructible_safe_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_167 {
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
pub struct __is_nt_destructible_impl_170 {}
impl Clone for __is_nt_destructible_impl_170 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_impl_170 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nt_destructible_safe_171 {}
impl Clone for __is_nt_destructible_safe_171 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_safe_171 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_destructible_172 {}
impl Clone for is_nothrow_destructible_172 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_destructible_172 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_173 {}
impl Clone for is_constructible_173 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_173 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_174 {}
impl Clone for is_constructible_174 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_174 {
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
pub struct is_move_constructible_176 {}
impl Clone for is_move_constructible_176 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_176 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_177 {}
impl Clone for is_move_constructible_177 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_177 {
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
pub struct is_nothrow_default_constructible_180 {}
impl Clone for is_nothrow_default_constructible_180 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_180 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_181 {}
impl Clone for is_nothrow_default_constructible_181 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_181 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_182 {}
impl Clone for is_nothrow_copy_constructible_182 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_182 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_183 {}
impl Clone for is_nothrow_copy_constructible_183 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_183 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_184 {}
impl Clone for is_nothrow_copy_constructible_184 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_184 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_185 {}
impl Clone for is_nothrow_copy_constructible_185 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_185 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_copy_assignable_186 {}
impl Clone for is_copy_assignable_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_assignable_186 {
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
pub struct is_trivially_destructible_187 {}
impl Clone for is_trivially_destructible_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_187 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_trivially_destructible_188 {}
impl Clone for is_trivially_destructible_188 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_188 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_trivially_destructible_189 {}
impl Clone for is_trivially_destructible_189 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_189 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_190 {}
impl Clone for __is_nothrow_new_constructible_impl_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_190 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_191 {}
impl Clone for __is_nothrow_new_constructible_impl_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_191 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_192 {}
impl Clone for __is_nothrow_new_constructible_impl_192 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_192 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_193 {}
impl Clone for remove_reference_193 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_193 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_194 {}
impl Clone for remove_reference_194 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_194 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_195 {}
impl Clone for remove_reference_195 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_195 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_196 {}
impl Clone for remove_reference_196 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_196 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_197 {}
impl Clone for remove_reference_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_197 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_198 {}
impl Clone for remove_reference_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_199 {}
impl Clone for remove_reference_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_199 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_200 {}
impl Clone for remove_reference_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_200 {
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
pub struct __cv_selector_202 {}
impl Clone for __cv_selector_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_202 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_203 {}
impl Clone for __cv_selector_203 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_203 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_204 {}
impl Clone for __cv_selector_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_204 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_205 {}
impl Clone for __cv_selector_205 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_205 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_206 {}
impl Clone for __match_cv_qualifiers_206 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_206 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_207 {}
impl Clone for __match_cv_qualifiers_207 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_207 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_208 {}
impl Clone for __match_cv_qualifiers_208 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_208 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_209 {}
impl Clone for __match_cv_qualifiers_209 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_209 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_210 {}
impl Clone for __match_cv_qualifiers_210 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_210 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_211 {}
impl Clone for __make_unsigned_211 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_211 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_212 {}
impl Clone for __make_unsigned_212 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_212 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_213 {}
impl Clone for __make_unsigned_213 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_213 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_214 {}
impl Clone for __make_unsigned_selector_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_214 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_215 {}
impl Clone for __make_unsigned_selector_215 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_215 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_216 {}
impl Clone for __make_unsigned_selector_216 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_216 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_217 {}
impl Clone for __make_unsigned_selector_217 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_217 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_218 {}
impl Clone for __make_unsigned_selector_218 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_218 {
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
pub struct _List_223 {}
impl Clone for _List_223 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_223 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_224 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_225 {}
impl Clone for _List_225 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_225 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_226 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_227 {}
impl Clone for _List_227 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_227 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_228 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_229 {}
impl Clone for _List_229 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_229 {
    fn default() -> Self {
        unimplemented!()
    }
}
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_231 {}
impl Clone for _List_231 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_231 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_232 {}
impl Clone for __select_232 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_232 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_233 {}
impl Clone for __select_233 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_233 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_234 {}
impl Clone for __select_234 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_234 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_235 {}
impl Clone for __select_235 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_235 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_236 {}
impl Clone for __select_236 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_236 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_237 {}
impl Clone for __make_signed_237 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_237 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_238 {}
impl Clone for __make_signed_238 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_238 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_239 {}
impl Clone for __make_signed_selector_239 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_239 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_240 {}
impl Clone for __make_signed_selector_240 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_240 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_241 {}
impl Clone for __make_signed_selector_241 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_241 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_242 {}
impl Clone for __make_signed_selector_242 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_242 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_243 {}
impl Clone for __make_signed_selector_243 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_243 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_pointer_244 {}
impl Clone for remove_pointer_244 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_pointer_244 {
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
pub struct common_type_245 {}
impl Clone for common_type_245 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_245 {
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
    pub static __is_nothrow_new_constructible_246: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __is_nothrow_new_constructible_247: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __is_nothrow_new_constructible_248: Value<bool> = unimplemented!();
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
pub struct __are_same_255 {}
impl Clone for __are_same_255 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_255 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_256 = u32;
pub const anon_256___value: anon_256 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_257 {}
impl Clone for __are_same_257 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_257 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_258 = u32;
pub const anon_258___value: anon_258 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_259 {}
impl Clone for __are_same_259 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_259 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_260 = u32;
pub const anon_260___value: anon_260 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_261 {}
impl Clone for __are_same_261 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_261 {
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
pub const anon_266___value: anon_266 = 1;
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
pub const anon_268___value: anon_268 = 1;
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
pub struct __is_integer_271 {}
impl Clone for __is_integer_271 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_271 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_272 = u32;
pub const anon_272___value: anon_272 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_273 {}
impl Clone for __is_integer_273 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_273 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_274 = u32;
pub const anon_274___value: anon_274 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_byte_275 {}
impl Clone for __is_byte_275 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_byte_275 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_276 = u32;
pub const anon_276___value: anon_276 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nonvolatile_trivially_copyable_277 {}
impl Clone for __is_nonvolatile_trivially_copyable_277 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nonvolatile_trivially_copyable_277 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_278 = u32;
pub const anon_278___value: anon_278 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_279 {}
impl Clone for __memcpyable_279 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_279 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_280 {}
impl Clone for __memcpyable_integer_280 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_280 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_281 {}
impl Clone for __memcpyable_integer_281 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_281 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_282 = u32;
pub const anon_282___width: anon_282 = 32;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __enable_if_283 {}
impl Clone for __enable_if_283 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __enable_if_283 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __conditional_type_284 {}
impl Clone for __conditional_type_284 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_type_284 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_285 {}
impl Clone for __is_integer_nonstrict_285 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_285 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_286 = u32;
pub const anon_286___width: anon_286 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_287 {}
impl Clone for __is_integer_nonstrict_287 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_287 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_288 = u32;
pub const anon_288___width: anon_288 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_289 {}
impl Clone for __numeric_traits_integer_289 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_289 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____is_signed_290: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____digits_291: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____max_292: Value<i64> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_293 {}
impl Clone for __numeric_traits_integer_293 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_293 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_294: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____digits_295: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____max_296: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_297 {}
impl Clone for __numeric_traits_floating_297 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_297 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_298 {}
impl Clone for __numeric_traits_floating_298 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_298 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_299 {}
impl Clone for __numeric_traits_floating_299 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_299 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_300 {}
impl Clone for __numeric_traits_300 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_300 {
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
    pub static piecewise_construct_301: Value<std_piecewise_construct_t> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_302 {}
impl Clone for tuple_302 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_302 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_302 {}
impl tuple_302 {}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(12)]
pub struct array_303 {
    #[offset(0)]
    #[byte_size(12)]
    pub _M_elems: Value<Box<[i32]>>,
}
impl Clone for array_303 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for array_303 {
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
pub struct iterator_304 {}
impl Clone for iterator_304 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_304 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_305 {}
impl Clone for iterator_305 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_305 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_305 {}
impl iterator_305 {}
impl iterator_305 {}
impl iterator_305 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_305>) -> Ptr<iterator_305> {
        unimplemented!()
    }
}
impl iterator_305 {
    pub fn move_assign(&self, _a0: Ptr<iterator_305>) -> Ptr<iterator_305> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_306 {}
impl Clone for iterator_306 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_306 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_306 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_306>) -> Ptr<iterator_306> {
        unimplemented!()
    }
}
impl iterator_306 {
    pub fn move_assign(&self, _a0: Ptr<iterator_306>) -> Ptr<iterator_306> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_307 {}
impl Clone for iterator_307 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_307 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_307 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_307>) -> Ptr<iterator_307> {
        unimplemented!()
    }
}
impl iterator_307 {
    pub fn move_assign(&self, _a0: Ptr<iterator_307>) -> Ptr<iterator_307> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_308 {}
impl Clone for iterator_traits_308 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_308 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_309 {}
impl Clone for iterator_traits_309 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_309 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_310 {}
impl Clone for iterator_traits_310 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_310 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_311 {}
impl Clone for iterator_traits_311 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_311 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_312 {}
impl Clone for iterator_traits_312 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_312 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_313 {}
impl Clone for iterator_traits_313 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_313 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_314 {}
impl Clone for iterator_traits_314 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_314 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_315 {}
impl Clone for iterator_traits_315 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_315 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_316 {}
impl Clone for iterator_traits_316 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_316 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_317 {}
impl Clone for __iterator_traits_317 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_317 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_318 {}
impl Clone for __iterator_traits_318 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_318 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_319 {}
impl Clone for __iterator_traits_319 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_319 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_320 {}
impl Clone for __ptr_traits_ptr_to_320 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_320 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_321 {}
impl Clone for __ptr_traits_ptr_to_321 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_321 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_322 {}
impl Clone for __ptr_traits_ptr_to_322 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_322 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_323 {}
impl Clone for pointer_traits_323 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_323 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_324 {}
impl Clone for pointer_traits_324 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_324 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_325 {}
impl Clone for pointer_traits_325 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_325 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_326 {}
impl Clone for reverse_iterator_326 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_326 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_326 {
    pub fn copy_from(__x: Ptr<reverse_iterator_326>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_326 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_326>) -> Ptr<reverse_iterator_326> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_327 {}
impl Clone for reverse_iterator_327 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_327 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_327 {
    pub fn copy_from(__x: Ptr<reverse_iterator_327>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_327 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_327>) -> Ptr<reverse_iterator_327> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_328 {}
impl Clone for __normal_iterator_328 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_328 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_328 {
    pub fn new_1(__i: Ptr<Ptr<i32>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_328 {}
impl __normal_iterator_328 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_329 {}
impl Clone for __normal_iterator_329 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_329 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_329 {
    pub fn new_1(__i: Ptr<Ptr<i32>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_329 {}
impl __normal_iterator_329 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_330 {}
impl Clone for __normal_iterator_330 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_330 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_330 {
    pub fn new_1(__i: Ptr<Ptr<Point>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_330 {}
impl __normal_iterator_330 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_331 {}
impl Clone for __normal_iterator_331 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_331 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_331 {
    pub fn new_1(__i: Ptr<Ptr<Point>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_331 {}
impl __normal_iterator_331 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_332 {}
impl Clone for __normal_iterator_332 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_332 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_332 {
    pub fn new_1(__i: Ptr<Ptr<vector_333>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_332 {}
impl __normal_iterator_332 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_334 {}
impl Clone for __normal_iterator_334 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_334 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_334 {
    pub fn new_1(__i: Ptr<Ptr<vector_333>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_334 {}
impl __normal_iterator_334 {}
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
pub struct __array_traits_335 {}
impl Clone for __array_traits_335 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __array_traits_335 {
    fn default() -> Self {
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
pub struct __new_allocator_336 {}
impl Clone for __new_allocator_336 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_336 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_336 {
    pub fn copy_from(_a0: Ptr<__new_allocator_336>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_336 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_336>) -> Ptr<__new_allocator_336> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_337 {}
impl Clone for __new_allocator_337 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_337 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_337 {
    pub fn copy_from(_a0: Ptr<__new_allocator_337>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_337 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_337>) -> Ptr<__new_allocator_337> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_338 {}
impl Clone for __new_allocator_338 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_338 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_338 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_338 {
    pub fn copy_from(_a0: Ptr<__new_allocator_338>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_338 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_338>) -> Ptr<__new_allocator_338> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_340 {}
impl Clone for __new_allocator_340 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_340 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_340 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_340 {
    pub fn copy_from(_a0: Ptr<__new_allocator_340>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_340 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_340>) -> Ptr<__new_allocator_340> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_342 {}
impl Clone for __new_allocator_342 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_342 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_342 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_342 {
    pub fn copy_from(_a0: Ptr<__new_allocator_342>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_342 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_342>) -> Ptr<__new_allocator_342> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_345 {}
impl Clone for allocator_345 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_345 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_345 {}
impl allocator_345 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_346 {}
impl Clone for allocator_346 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_346 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_346 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_346 {
    pub fn copy_from(__a: Ptr<allocator_346>) -> Self {
        unimplemented!()
    }
}
impl allocator_346 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_346>) -> Ptr<allocator_346> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_347 {}
impl Clone for allocator_347 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_347 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_347 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_347 {
    pub fn copy_from(__a: Ptr<allocator_347>) -> Self {
        unimplemented!()
    }
}
impl allocator_347 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_347>) -> Ptr<allocator_347> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_348 {}
impl Clone for allocator_348 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_348 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_348 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_348 {
    pub fn copy_from(__a: Ptr<allocator_348>) -> Self {
        unimplemented!()
    }
}
impl allocator_348 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_348>) -> Ptr<allocator_348> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_349 {}
impl Clone for allocator_traits_349 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_349 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_349 {
    pub fn allocate_1(__a: Ptr<allocator_346>, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
}
impl allocator_traits_349 {
    pub fn deallocate(__a: Ptr<allocator_346>, mut __p: Ptr<i32>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_349 {
    pub fn max_size(__a: Ptr<allocator_346>) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_349 {
    pub fn construct_350(__a: Ptr<allocator_346>, mut __p: Ptr<i32>, __args: Ptr<i32>) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_351 {}
impl Clone for allocator_traits_351 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_351 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_351 {
    pub fn allocate_1(__a: Ptr<allocator_347>, mut __n: u64) -> Ptr<Point> {
        unimplemented!()
    }
}
impl allocator_traits_351 {
    pub fn deallocate(__a: Ptr<allocator_347>, mut __p: Ptr<Point>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_351 {
    pub fn max_size(__a: Ptr<allocator_347>) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_351 {
    pub fn construct_352(__a: Ptr<allocator_347>, mut __p: Ptr<Point>, __args: Ptr<Point>) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_353 {}
impl Clone for allocator_traits_353 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_353 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_353 {
    pub fn allocate_1(__a: Ptr<allocator_348>, mut __n: u64) -> Ptr<vector_333> {
        unimplemented!()
    }
}
impl allocator_traits_353 {
    pub fn deallocate(__a: Ptr<allocator_348>, mut __p: Ptr<vector_333>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_353 {
    pub fn max_size(__a: Ptr<allocator_348>) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_353 {
    pub fn construct_354(
        __a: Ptr<allocator_348>,
        mut __p: Ptr<vector_333>,
        __args: Ptr<vector_333>,
    ) {
        unimplemented!()
    }
}
impl allocator_traits_353 {
    pub fn destroy_355(__a: Ptr<allocator_348>, mut __p: Ptr<vector_333>) {
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
pub struct __is_move_insertable_357 {}
impl Clone for __is_move_insertable_357 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_357 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_move_insertable_358 {}
impl Clone for __is_move_insertable_358 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_358 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_move_insertable_359 {}
impl Clone for __is_move_insertable_359 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_359 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_360 {}
impl Clone for __alloc_traits_360 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_360 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_361 {}
impl Clone for rebind_361 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_361 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_362 {}
impl Clone for __alloc_traits_362 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_362 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_363 {}
impl Clone for rebind_363 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_363 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_364 {}
impl Clone for __alloc_traits_364 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_364 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_365 {}
impl Clone for rebind_365 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_365 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct _UninitDestroyGuard_366 {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_first: Ptr<i32>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_cur: Ptr<Ptr<i32>>,
}
impl Clone for _UninitDestroyGuard_366 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _UninitDestroyGuard_366 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_366 {
    pub fn new(__first: Ptr<Ptr<i32>>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __uninitialized_default_n_1_367 {}
impl Clone for __uninitialized_default_n_1_367 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __uninitialized_default_n_1_367 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __uninitialized_default_n_1_367 {
    pub fn __uninit_default_n_368(mut __first: Ptr<i32>, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_bitwise_relocatable_369 {}
impl Clone for __is_bitwise_relocatable_369 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_bitwise_relocatable_369 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_bitwise_relocatable_370 {}
impl Clone for __is_bitwise_relocatable_370 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_bitwise_relocatable_370 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_bitwise_relocatable_371 {}
impl Clone for __is_bitwise_relocatable_371 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_bitwise_relocatable_371 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_372 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_int__std_allocator_int____Vector_impl,
}
impl Clone for _Vector_base_372 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_372 {
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
    pub fn new_2(__a: Ptr<allocator_346>) -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__std_allocator_int____Vector_impl {
    pub fn move_from(__x: Ptr<std__Vector_base_int__std_allocator_int____Vector_impl>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_372 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_372 {
    pub fn new_2(__a: Ptr<allocator_346>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_372 {
    pub fn new_3(mut __n: usize, __a: Ptr<allocator_346>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_372 {
    pub fn move_from(_a0: Ptr<_Vector_base_372>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_373 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_Point__std_allocator_Point____Vector_impl,
}
impl Clone for _Vector_base_373 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_373 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_Point__std_allocator_Point____Vector_impl_data {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_start: Ptr<Point>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_finish: Ptr<Point>,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_end_of_storage: Ptr<Point>,
}
impl Clone for std__Vector_base_Point__std_allocator_Point____Vector_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_Point__std_allocator_Point____Vector_impl_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Point__std_allocator_Point____Vector_impl_data {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Point__std_allocator_Point____Vector_impl_data {
    pub fn move_from(
        __x: Ptr<std__Vector_base_Point__std_allocator_Point____Vector_impl_data>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_Point__std_allocator_Point____Vector_impl {}
impl Clone for std__Vector_base_Point__std_allocator_Point____Vector_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_Point__std_allocator_Point____Vector_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Point__std_allocator_Point____Vector_impl {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Point__std_allocator_Point____Vector_impl {
    pub fn move_from(__x: Ptr<std__Vector_base_Point__std_allocator_Point____Vector_impl>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_373 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_373 {
    pub fn move_from(_a0: Ptr<_Vector_base_373>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_374 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl,
}
impl Clone for _Vector_base_374 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_374 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl_data {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_start: Ptr<vector_333>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_finish: Ptr<vector_333>,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_end_of_storage: Ptr<vector_333>,
}
impl Clone for std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default
    for std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl_data
{
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl_data {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl_data {
    pub fn move_from(
        __x: Ptr<
            std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl_data,
        >,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl {}
impl Clone for std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl {
    pub fn move_from(
        __x: Ptr<std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl>,
    ) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_374 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_374 {
    pub fn move_from(_a0: Ptr<_Vector_base_374>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_333 {}
impl Clone for vector_333 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_333 {
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
    pub _M_vect: Ptr<_Vector_base_372>,
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
    pub fn new(mut __s: Ptr<i32>, mut __l: u64, __vect: Ptr<_Vector_base_372>) -> Self {
        unimplemented!()
    }
}
impl vector_333 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_333 {
    pub fn new_2(mut __n: u64, __value: Ptr<i32>, __a: Option<Ptr<allocator_346>>) -> Self {
        unimplemented!()
    }
}
impl vector_333 {
    pub fn copy_from(__x: Ptr<vector_333>) -> Self {
        unimplemented!()
    }
}
impl vector_333 {
    pub fn move_from(_a0: Ptr<vector_333>) -> Self {
        unimplemented!()
    }
}
impl vector_333 {
    pub fn new_3(__l: initializer_list_1, __a: Option<Ptr<allocator_346>>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_376 {}
impl Clone for vector_376 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_376 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std_vector_Point___Guard_alloc {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_storage: Ptr<Point>,
    #[offset(8)]
    pub _M_len: u64,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_vect: Ptr<_Vector_base_373>,
}
impl Clone for std_vector_Point___Guard_alloc {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_vector_Point___Guard_alloc {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_vector_Point___Guard_alloc {
    pub fn new(mut __s: Ptr<Point>, mut __l: u64, __vect: Ptr<_Vector_base_373>) -> Self {
        unimplemented!()
    }
}
impl vector_376 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_376 {
    pub fn copy_from(__x: Ptr<vector_376>) -> Self {
        unimplemented!()
    }
}
impl vector_376 {
    pub fn move_from(_a0: Ptr<vector_376>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_378 {}
impl Clone for vector_378 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_378 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std_vector_std_vector_int____Guard_alloc {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_storage: Ptr<vector_333>,
    #[offset(8)]
    pub _M_len: u64,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_vect: Ptr<_Vector_base_374>,
}
impl Clone for std_vector_std_vector_int____Guard_alloc {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_vector_std_vector_int____Guard_alloc {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_vector_std_vector_int____Guard_alloc {
    pub fn new(mut __s: Ptr<vector_333>, mut __l: u64, __vect: Ptr<_Vector_base_374>) -> Self {
        unimplemented!()
    }
}
impl vector_378 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_378 {
    pub fn copy_from(__x: Ptr<vector_378>) -> Self {
        unimplemented!()
    }
}
impl vector_378 {
    pub fn move_from(_a0: Ptr<vector_378>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_380 {}
impl Clone for __hash_base_380 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_380 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_381 {}
impl Clone for __hash_base_381 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_381 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_382 {}
impl Clone for __hash_base_382 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_382 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_383 {}
impl Clone for __hash_base_383 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_383 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_384 {}
impl Clone for __hash_base_384 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_384 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_385 {}
impl Clone for __hash_base_385 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_385 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_386 {}
impl Clone for __hash_base_386 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_386 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_387 {}
impl Clone for __hash_base_387 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_387 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_388 {}
impl Clone for __hash_base_388 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_388 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_389 {}
impl Clone for __hash_base_389 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_389 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_390 {}
impl Clone for __hash_base_390 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_390 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_391 {}
impl Clone for __hash_base_391 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_391 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_392 {}
impl Clone for __hash_base_392 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_392 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_393 {}
impl Clone for __hash_base_393 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_393 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_394 {}
impl Clone for __hash_base_394 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_394 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_395 {}
impl Clone for __hash_base_395 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_395 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_396 {}
impl Clone for __hash_base_396 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_396 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_397 {}
impl Clone for __hash_base_397 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_397 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_398 {}
impl Clone for __hash_base_398 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_398 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_399 {}
impl Clone for __hash_base_399 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_399 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_400 {}
impl Clone for __hash_base_400 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_400 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_401 {}
impl Clone for hash_401 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_401 {
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
    pub fn hash_402(__val: Ptr<f32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_403(__val: Ptr<f64>) -> usize {
        unimplemented!()
    }
}
pub type anon_404 = u32;
pub const anon_404__S_word_bit: anon_404 = 64;
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
pub struct binary_function_405 {}
impl Clone for binary_function_405 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_405 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_406 {}
impl Clone for greater_406 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_406 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_407 {}
impl Clone for less_407 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_407 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_equal_408 {}
impl Clone for greater_equal_408 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_408 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_equal_409 {}
impl Clone for less_equal_409 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_409 {
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
    pub static allocator_arg_410: Value<std_allocator_arg_t> = unimplemented!();
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
#[byte_size(8)]
pub struct Point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(48)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(24)]
    pub values: vector_333,
    #[offset(24)]
    #[byte_size(24)]
    pub points: vector_376,
}
pub fn push_and_index_411(v: Ptr<vector_333>) -> i32 {
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(42));
        vector_333Impl::push_back_58(&v, ___x.as_pointer())
    });
    return ((({ vector_333Impl::size(&v) }) as i32) - 1);
}
pub fn sum_ref_412(v: Ptr<vector_333>) -> i32 {
    let mut s: i32 = 0;
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (({ vector_333Impl::size(&v) }) as usize) }) {
        s += (({
            let ___n: u64 = (i as u64);
            vector_333Impl::operator_index_47(&v, ___n)
        })
        .read());
        i.prefix_inc();
    }
    return s;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<vector_333> = Rc::new(RefCell::new(vector_333::new_3(
        { __cpp2rust_unimplemented::<initializer_list_1>() },
        None,
    )));
    ({ vector_333Impl::operator_index_46(&v.as_pointer(), 0_u64) }).write(10);
    (if ((({ vector_333Impl::operator_index_46(&v.as_pointer(), 0_u64) }).read()) == 10) {
        &(0);
    } else {
        ({
            __assert_fail_413(
                Ptr::<i8>::from_string_literal(b"v[0] == 10"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    {
        ({ vector_333Impl::operator_index_46(&v.as_pointer(), 1_u64) })
            .with_mut(|__v| *__v = *__v + 5)
    };
    ({ vector_333Impl::operator_index_46(&v.as_pointer(), 2_u64) })
        .with_mut(|__v| __v.postfix_inc());
    (if ((({ vector_333Impl::operator_index_46(&v.as_pointer(), 1_u64) }).read()) == 7)
        && ((({ vector_333Impl::operator_index_46(&v.as_pointer(), 2_u64) }).read()) == 4)
    {
        &(0);
    } else {
        ({
            __assert_fail_413(
                Ptr::<i8>::from_string_literal(b"v[1] == 7 && v[2] == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let __rhs = (({ vector_333Impl::operator_index_46(&v.as_pointer(), 1_u64) }).read());
    ({ vector_333Impl::operator_index_46(&v.as_pointer(), 0_u64) }).write(__rhs);
    (if ((({ vector_333Impl::operator_index_46(&v.as_pointer(), 0_u64) }).read()) == 7) {
        &(0);
    } else {
        ({
            __assert_fail_413(
                Ptr::<i8>::from_string_literal(b"v[0] == 7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ vector_333Impl::operator_index_46(&v.as_pointer(), 1_u64) }).write(0);
    (if ((({
        let ___n: u64 =
            ((({ vector_333Impl::operator_index_46(&v.as_pointer(), 1_u64) }).read()) as u64);
        vector_333Impl::operator_index_46(&v.as_pointer(), ___n)
    })
    .read())
        == 7)
    {
        &(0);
    } else {
        ({
            __assert_fail_413(
                Ptr::<i8>::from_string_literal(b"v[v[1]] == 7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut i: i32 = 0;
    ({ vector_333Impl::operator_index_46(&v.as_pointer(), (i.postfix_inc() as u64)) }).write(3);
    (if (i == 1) && ((({ vector_333Impl::operator_index_46(&v.as_pointer(), 0_u64) }).read()) == 3)
    {
        &(0);
    } else {
        ({
            __assert_fail_413(
                Ptr::<i8>::from_string_literal(b"i == 1 && v[0] == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((({
        let ___n: u64 = (({ push_and_index_411(v.as_pointer()) }) as u64);
        vector_333Impl::operator_index_46(&v.as_pointer(), ___n)
    })
    .read())
        == 42)
    {
        &(0);
    } else {
        ({
            __assert_fail_413(
                Ptr::<i8>::from_string_literal(b"v[push_and_index(v)] == 42"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({
        let ___n: u64 = (({ push_and_index_411(v.as_pointer()) }) as u64);
        vector_333Impl::operator_index_46(&v.as_pointer(), ___n)
    })
    .write(5);
    (if (({ vector_333Impl::size(&v.as_pointer()) }) == 5_u64)
        && ((({ vector_333Impl::operator_index_46(&v.as_pointer(), 4_u64) }).read()) == 5)
    {
        &(0);
    } else {
        ({
            __assert_fail_413(
                Ptr::<i8>::from_string_literal(b"v.size() == 5 && v[4] == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut p: Ptr<i32> = ({ vector_333Impl::operator_index_46(&v.as_pointer(), 2_u64) });
    p.write(9);
    (if ((({ vector_333Impl::operator_index_46(&v.as_pointer(), 2_u64) }).read()) == 9) {
        &(0);
    } else {
        ({
            __assert_fail_413(
                Ptr::<i8>::from_string_literal(b"v[2] == 9"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ sum_ref_412(v.as_pointer()) }) == ((((3 + 0) + 9) + 42) + 5)) {
        &(0);
    } else {
        ({
            __assert_fail_413(
                Ptr::<i8>::from_string_literal(b"sum_ref(v) == 3 + 0 + 9 + 42 + 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let h: Value<Holder> = Rc::new(RefCell::new(<Holder>::default()));
    ({ vector_333Impl::resize_40(&field_ptr!(h.as_pointer(), values), 2_u64) });
    ({ vector_333Impl::operator_index_46(&field_ptr!(h.as_pointer(), values), 1_u64) }).write(6);
    ({
        let ___x: Value<Point> = Rc::new(RefCell::new(Point { x: 1, y: 2 }));
        vector_376Impl::push_back_58(&field_ptr!(h.as_pointer(), points), ___x.as_pointer())
    });
    field!(
        ({ vector_376Impl::operator_index_46(&field_ptr!(h.as_pointer(), points), 0_u64,) }),
        y
    )
    .write(5);
    (if ((({ vector_333Impl::operator_index_46(&field_ptr!(h.as_pointer(), values), 1_u64) })
        .read())
        == 6)
    {
        &(0);
    } else {
        ({
            __assert_fail_413(
                Ptr::<i8>::from_string_literal(b"h.values[1] == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        PointImpl::sum(
            &({ vector_376Impl::operator_index_46(&field_ptr!(h.as_pointer(), points), 0_u64) }),
        )
    }) == 6)
    {
        &(0);
    } else {
        ({
            __assert_fail_413(
                Ptr::<i8>::from_string_literal(b"h.points[0].sum() == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut hp: Ptr<Holder> = (h.as_pointer());
    let __rhs =
        ((({ vector_333Impl::operator_index_46(&field_ptr!(hp, values), 1_u64) }).read()) + 1);
    ({ vector_333Impl::operator_index_46(&field_ptr!(hp, values), 0_u64) }).write(__rhs);
    let __rhs = (({ vector_333Impl::operator_index_46(&field_ptr!(hp, values), 0_u64) }).read());
    field!(
        ({ vector_376Impl::operator_index_46(&field_ptr!(hp, points), 0_u64,) }),
        x
    )
    .write(__rhs);
    (if ((({ vector_333Impl::operator_index_46(&field_ptr!(h.as_pointer(), values), 0_u64) })
        .read())
        == 7)
        && (({ vector_376Impl::operator_index_46(&field_ptr!(h.as_pointer(), points), 0_u64) })
            .with(|__s| __s.x)
            == 7)
    {
        &(0);
    } else {
        ({
            __assert_fail_413(
                Ptr::<i8>::from_string_literal(b"h.values[0] == 7 && h.points[0].x == 7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut q: Point =
        (*({ vector_376Impl::operator_index_46(&field_ptr!(h.as_pointer(), points), 0_u64) })
            .upgrade()
            .deref())
        .clone();
    q.x = 0;
    (if (({ vector_376Impl::operator_index_46(&field_ptr!(h.as_pointer(), points), 0_u64) })
        .with(|__s| __s.x)
        == 7)
    {
        &(0);
    } else {
        ({
            __assert_fail_413(
                Ptr::<i8>::from_string_literal(b"h.points[0].x == 7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let grid: Value<vector_378> = Rc::new(RefCell::new(<vector_378>::default()));
    ({
        let ___x: Value<vector_333> = Rc::new(RefCell::new({
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(0));
            vector_333::new_2({ 3_u64 }, { __tmp_0.as_pointer() }, None)
        }));
        vector_378Impl::push_back_58(&grid.as_pointer(), ___x.as_pointer())
    });
    ({
        let ___x: Value<vector_333> = Rc::new(RefCell::new({
            let __tmp_1: Value<i32> = Rc::new(RefCell::new(0));
            vector_333::new_2({ 3_u64 }, { __tmp_1.as_pointer() }, None)
        }));
        vector_378Impl::push_back_58(&grid.as_pointer(), ___x.as_pointer())
    });
    ({
        vector_333Impl::operator_index_46(
            &({ vector_378Impl::operator_index_46(&grid.as_pointer(), 1_u64) }),
            2_u64,
        )
    })
    .write(8);
    (if ((({
        vector_333Impl::operator_index_46(
            &({ vector_378Impl::operator_index_46(&grid.as_pointer(), 1_u64) }),
            2_u64,
        )
    })
    .read())
        == 8)
        && ((({
            vector_333Impl::operator_index_46(
                &({ vector_378Impl::operator_index_46(&grid.as_pointer(), 0_u64) }),
                2_u64,
            )
        })
        .read())
            == 0)
    {
        &(0);
    } else {
        ({
            __assert_fail_413(
                Ptr::<i8>::from_string_literal(b"grid[1][2] == 8 && grid[0][2] == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let a: Value<array_303> = Rc::new(RefCell::new(__cpp2rust_unimplemented::<array_303>()));
    let __rhs = ((({ array_303Impl::operator_index_18(&a.as_pointer(), 0_u64) }).read())
        + (({ array_303Impl::operator_index_18(&a.as_pointer(), 2_u64) }).read()));
    ({ array_303Impl::operator_index_18(&a.as_pointer(), 1_u64) }).write(__rhs);
    (if ((({ array_303Impl::operator_index_18(&a.as_pointer(), 1_u64) }).read()) == 10) {
        &(0);
    } else {
        ({
            __assert_fail_413(
                Ptr::<i8>::from_string_literal(b"a[1] == 10"),
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
#[track_caller]
pub fn __cpp2rust_unimplemented<T>() -> T {
    unimplemented!()
}
pub fn __assert_fail_413(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub trait PointImpl {
    fn sum(&self) -> i32;
}
impl PointImpl for Ptr<Point> {
    fn sum(&self) -> i32 {
        return ((*self).with(|__s| __s.x) + (*self).with(|__s| __s.y));
    }
}
pub trait _UninitDestroyGuard_366Impl {
    fn release(&self) {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_366Impl for Ptr<_UninitDestroyGuard_366> {}
pub trait _Vector_base_372Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_346> {
        unimplemented!()
    }
    fn _M_get_Tp_allocator_2(&self) -> Ptr<allocator_346> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<i32> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<i32>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_372Impl for Ptr<_Vector_base_372> {}
pub trait _Vector_base_373Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_347> {
        unimplemented!()
    }
    fn _M_get_Tp_allocator_2(&self) -> Ptr<allocator_347> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<Point> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<Point>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_373Impl for Ptr<_Vector_base_373> {}
pub trait _Vector_base_374Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_348> {
        unimplemented!()
    }
    fn _M_get_Tp_allocator_2(&self) -> Ptr<allocator_348> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<vector_333> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<vector_333>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_374Impl for Ptr<_Vector_base_374> {}
pub trait __new_allocator_338Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<i32> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<i32>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn construct_339(&self, mut __p: Ptr<i32>, __args: Ptr<i32>) {
        unimplemented!()
    }
}
impl __new_allocator_338Impl for Ptr<__new_allocator_338> {}
pub trait __new_allocator_340Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<Point> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<Point>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn construct_341(&self, mut __p: Ptr<Point>, __args: Ptr<Point>) {
        unimplemented!()
    }
}
impl __new_allocator_340Impl for Ptr<__new_allocator_340> {}
pub trait __new_allocator_342Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<vector_333> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<vector_333>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn construct_343(&self, mut __p: Ptr<vector_333>, __args: Ptr<vector_333>) {
        unimplemented!()
    }
    fn destroy_344(&self, mut __p: Ptr<vector_333>) {
        unimplemented!()
    }
}
impl __new_allocator_342Impl for Ptr<__new_allocator_342> {}
pub trait __normal_iterator_328Impl {
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_sub(&self, mut __n: i64) -> __normal_iterator_328 {
        unimplemented!()
    }
    fn base(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl __normal_iterator_328Impl for Ptr<__normal_iterator_328> {}
pub trait __normal_iterator_329Impl {
    fn base(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl __normal_iterator_329Impl for Ptr<__normal_iterator_329> {}
pub trait __normal_iterator_330Impl {
    fn operator_deref(&self) -> Ptr<Point> {
        unimplemented!()
    }
    fn operator_sub(&self, mut __n: i64) -> __normal_iterator_330 {
        unimplemented!()
    }
    fn base(&self) -> Ptr<Ptr<Point>> {
        unimplemented!()
    }
}
impl __normal_iterator_330Impl for Ptr<__normal_iterator_330> {}
pub trait __normal_iterator_331Impl {
    fn base(&self) -> Ptr<Ptr<Point>> {
        unimplemented!()
    }
}
impl __normal_iterator_331Impl for Ptr<__normal_iterator_331> {}
pub trait __normal_iterator_332Impl {
    fn operator_deref(&self) -> Ptr<vector_333> {
        unimplemented!()
    }
    fn operator_sub(&self, mut __n: i64) -> __normal_iterator_332 {
        unimplemented!()
    }
    fn base(&self) -> Ptr<Ptr<vector_333>> {
        unimplemented!()
    }
}
impl __normal_iterator_332Impl for Ptr<__normal_iterator_332> {}
pub trait __normal_iterator_334Impl {
    fn base(&self) -> Ptr<Ptr<vector_333>> {
        unimplemented!()
    }
}
impl __normal_iterator_334Impl for Ptr<__normal_iterator_334> {}
pub trait array_303Impl {
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn operator_index_18(&self, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
}
impl array_303Impl for Ptr<array_303> {}
pub trait initializer_list_0Impl {
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn begin(&self) -> Ptr<bool> {
        unimplemented!()
    }
}
impl initializer_list_0Impl for Ptr<initializer_list_0> {}
pub trait initializer_list_1Impl {
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn begin(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn end(&self) -> Ptr<i32> {
        unimplemented!()
    }
}
impl initializer_list_1Impl for Ptr<initializer_list_1> {}
pub trait integral_constant_2Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl integral_constant_2Impl for Ptr<integral_constant_2> {}
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
pub trait vector_333Impl {
    fn copy_assign(&self, __x: Ptr<vector_333>) -> Ptr<vector_333> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_333>) -> Ptr<vector_333> {
        unimplemented!()
    }
    fn begin_26(&self) -> __normal_iterator_328 {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_329 {
        unimplemented!()
    }
    fn end_28(&self) -> __normal_iterator_328 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_329 {
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
    fn empty(&self) -> bool {
        unimplemented!()
    }
    fn operator_index_46(&self, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_index_47(&self, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
    fn back_53(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn push_back_58(&self, __x: Ptr<i32>) {
        unimplemented!()
    }
    fn emplace_back_375(&self, __args: Ptr<i32>) -> Ptr<i32> {
        unimplemented!()
    }
}
impl vector_333Impl for Ptr<vector_333> {}
pub trait vector_376Impl {
    fn copy_assign(&self, __x: Ptr<vector_376>) -> Ptr<vector_376> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_376>) -> Ptr<vector_376> {
        unimplemented!()
    }
    fn begin_26(&self) -> __normal_iterator_330 {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_331 {
        unimplemented!()
    }
    fn end_28(&self) -> __normal_iterator_330 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_331 {
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
    fn operator_index_46(&self, mut __n: u64) -> Ptr<Point> {
        unimplemented!()
    }
    fn back_53(&self) -> Ptr<Point> {
        unimplemented!()
    }
    fn push_back_58(&self, __x: Ptr<Point>) {
        unimplemented!()
    }
    fn emplace_back_377(&self, __args: Ptr<Point>) -> Ptr<Point> {
        unimplemented!()
    }
}
impl vector_376Impl for Ptr<vector_376> {}
pub trait vector_378Impl {
    fn copy_assign(&self, __x: Ptr<vector_378>) -> Ptr<vector_378> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_378>) -> Ptr<vector_378> {
        unimplemented!()
    }
    fn begin_26(&self) -> __normal_iterator_332 {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_334 {
        unimplemented!()
    }
    fn end_28(&self) -> __normal_iterator_332 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_334 {
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
    fn operator_index_46(&self, mut __n: u64) -> Ptr<vector_333> {
        unimplemented!()
    }
    fn back_53(&self) -> Ptr<vector_333> {
        unimplemented!()
    }
    fn push_back_58(&self, __x: Ptr<vector_333>) {
        unimplemented!()
    }
    fn emplace_back_379(&self, __args: Ptr<vector_333>) -> Ptr<vector_333> {
        unimplemented!()
    }
}
impl vector_378Impl for Ptr<vector_378> {}
pub fn __cpp2rust_init_globals() {}
