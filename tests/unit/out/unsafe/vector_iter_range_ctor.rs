extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: i64,
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
#[derive(VaArg, FnPtrArg)]
pub struct __fsid_t {
    pub __val: [i32; 2],
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut std_integral_constant_bool__true__value_1: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl integral_constant_0 {
    pub unsafe fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl integral_constant_0 {}
impl integral_constant_0 {}
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut std_integral_constant_bool__false__value_3: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_8 {}
impl Clone for __type_identity_8 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_8 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_9 {}
impl Clone for __type_identity_9 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_9 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_10 {}
impl Clone for __type_identity_10 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_10 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_11 {}
impl Clone for __type_identity_11 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_11 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_52 {}
impl Clone for is_reference_52 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_52 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_53 {}
impl Clone for is_reference_53 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_53 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_83 {}
impl Clone for remove_cv_83 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_83 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_84 {}
impl Clone for remove_cv_84 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_84 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_85 {}
impl Clone for remove_cv_85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_85 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_91 {}
impl Clone for is_const_91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_91 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_92 {}
impl Clone for is_const_92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_96 {}
impl Clone for __is_array_unknown_bounds_96 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_96 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_97 {}
impl Clone for __is_array_unknown_bounds_97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_97 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_116 {}
impl Clone for is_integral_116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_116 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_117 {}
impl Clone for is_integral_117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_117 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_118 {}
impl Clone for is_integral_118 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_118 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_arithmetic_122 {}
impl Clone for is_arithmetic_122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_122 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_arithmetic_123 {}
impl Clone for is_arithmetic_123 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_123 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_arithmetic_124 {}
impl Clone for is_arithmetic_124 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_124 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_arithmetic_125 {}
impl Clone for is_arithmetic_125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_125 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_scalar_126 {}
impl Clone for is_scalar_126 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_126 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_scalar_127 {}
impl Clone for is_scalar_127 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_127 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_scalar_128 {}
impl Clone for is_scalar_128 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_128 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_scalar_129 {}
impl Clone for is_scalar_129 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_129 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_134 {}
impl Clone for is_volatile_134 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_134 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_destructible_safe_139 {}
impl Clone for __is_destructible_safe_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_139 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_destructible_safe_140 {}
impl Clone for __is_destructible_safe_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_140 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_destructible_safe_141 {}
impl Clone for __is_destructible_safe_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_destructible_safe_142 {}
impl Clone for __is_destructible_safe_142 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_142 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_148 {}
impl Clone for is_nothrow_copy_constructible_148 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_148 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_trivially_destructible_149 {}
impl Clone for is_trivially_destructible_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_trivially_destructible_150 {}
impl Clone for is_trivially_destructible_150 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_150 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_trivially_destructible_151 {}
impl Clone for is_trivially_destructible_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_trivially_destructible_152 {}
impl Clone for is_trivially_destructible_152 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_152 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_154 {}
impl Clone for remove_reference_154 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_154 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_155 {}
impl Clone for remove_reference_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_156 {}
impl Clone for remove_reference_156 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_156 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_157 {}
impl Clone for remove_reference_157 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_157 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_158 {}
impl Clone for __cv_selector_158 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_158 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_159 {}
impl Clone for __cv_selector_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_159 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_160 {}
impl Clone for __cv_selector_160 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_160 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_161 {}
impl Clone for __cv_selector_161 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_161 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_162 {}
impl Clone for __match_cv_qualifiers_162 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_162 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_163 {}
impl Clone for __match_cv_qualifiers_163 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_163 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_164 {}
impl Clone for __match_cv_qualifiers_164 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_164 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_165 {}
impl Clone for __match_cv_qualifiers_165 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_165 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_166 {}
impl Clone for __match_cv_qualifiers_166 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_166 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_167 {}
impl Clone for __make_unsigned_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_167 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_168 {}
impl Clone for __make_unsigned_168 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_168 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_169 {}
impl Clone for __make_unsigned_169 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_169 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_170 {}
impl Clone for __make_unsigned_selector_170 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_170 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_171 {}
impl Clone for __make_unsigned_selector_171 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_171 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_172 {}
impl Clone for __make_unsigned_selector_172 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_172 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_173 {}
impl Clone for __make_unsigned_selector_173 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_173 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_174 {}
impl Clone for __make_unsigned_selector_174 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_174 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_175 {}
impl Clone for __make_unsigned_selector_175 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_175 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_179 {}
impl Clone for _List_179 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_179 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_180 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_181 {}
impl Clone for _List_181 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_181 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_182 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_183 {}
impl Clone for _List_183 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_183 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_184 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_185 {}
impl Clone for _List_185 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_185 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_186 {}
impl Clone for _List_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_187 {}
impl Clone for _List_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_187 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_188 {}
impl Clone for __select_188 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_188 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_189 {}
impl Clone for __select_189 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_189 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_190 {}
impl Clone for __select_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_190 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_191 {}
impl Clone for __select_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_191 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_192 {}
impl Clone for __select_192 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_192 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_193 {}
impl Clone for __make_signed_193 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_193 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_194 {}
impl Clone for __make_signed_194 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_194 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_195 {}
impl Clone for __make_signed_selector_195 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_195 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_196 {}
impl Clone for __make_signed_selector_196 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_196 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_197 {}
impl Clone for __make_signed_selector_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_197 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_198 {}
impl Clone for __make_signed_selector_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_199 {}
impl Clone for __make_signed_selector_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_199 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_pointer_200 {}
impl Clone for remove_pointer_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_pointer_200 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_pointer_201 {}
impl Clone for remove_pointer_201 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_pointer_201 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_pointer_202 {}
impl Clone for remove_pointer_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_pointer_202 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct common_type_203 {}
impl Clone for common_type_203 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_203 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std___nonesuchbase,
    ) -> *mut std___nonesuchbase {
        unimplemented!()
    }
}
impl std___nonesuchbase {
    pub unsafe fn move_assign(&mut self, _a0: *mut std___nonesuchbase) -> *mut std___nonesuchbase {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_204 {}
impl Clone for __are_same_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_204 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_205 = u32;
pub const anon_205___value: anon_205 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_206 {}
impl Clone for __are_same_206 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_206 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_207 = u32;
pub const anon_207___value: anon_207 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_208 {}
impl Clone for __are_same_208 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_208 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_209 = u32;
pub const anon_209___value: anon_209 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_210 {}
impl Clone for __are_same_210 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_210 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_211 = u32;
pub const anon_211___value: anon_211 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_212 {}
impl Clone for __are_same_212 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_212 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_213 = u32;
pub const anon_213___value: anon_213 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_214 {}
impl Clone for __are_same_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_214 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_215 = u32;
pub const anon_215___value: anon_215 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_216 {}
impl Clone for __is_integer_216 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_216 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_217 = u32;
pub const anon_217___value: anon_217 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_218 {}
impl Clone for __is_integer_218 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_218 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_219 = u32;
pub const anon_219___value: anon_219 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_220 {}
impl Clone for __is_integer_220 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_220 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_221 = u32;
pub const anon_221___value: anon_221 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_222 {}
impl Clone for __is_integer_222 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_222 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_223 = u32;
pub const anon_223___value: anon_223 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_224 {}
impl Clone for __is_integer_224 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_224 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_225 = u32;
pub const anon_225___value: anon_225 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_226 {}
impl Clone for __is_integer_226 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_226 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_227 = u32;
pub const anon_227___value: anon_227 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_228 {}
impl Clone for __is_integer_228 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_228 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_229 = u32;
pub const anon_229___value: anon_229 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_nonvolatile_trivially_copyable_230 {}
impl Clone for __is_nonvolatile_trivially_copyable_230 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nonvolatile_trivially_copyable_230 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_231 = u32;
pub const anon_231___value: anon_231 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_nonvolatile_trivially_copyable_232 {}
impl Clone for __is_nonvolatile_trivially_copyable_232 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nonvolatile_trivially_copyable_232 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_233 = u32;
pub const anon_233___value: anon_233 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_234 {}
impl Clone for __memcpyable_234 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_234 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_235 {}
impl Clone for __memcpyable_235 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_235 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_236 = u32;
pub const anon_236___value: anon_236 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_237 {}
impl Clone for __memcpyable_237 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_237 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_238 = u32;
pub const anon_238___value: anon_238 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_239 {}
impl Clone for __memcpyable_239 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_239 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_240 {}
impl Clone for __memcpyable_240 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_240 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_integer_241 {}
impl Clone for __memcpyable_integer_241 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_241 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_integer_242 {}
impl Clone for __memcpyable_integer_242 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_242 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_243 = u32;
pub const anon_243___width: anon_243 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_integer_244 {}
impl Clone for __memcpyable_integer_244 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_244 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_245 = u32;
pub const anon_245___width: anon_245 = 32;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_integer_246 {}
impl Clone for __memcpyable_integer_246 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_246 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_247 = u32;
pub const anon_247___width: anon_247 = 32;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_248 {}
impl Clone for __is_integer_nonstrict_248 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_248 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_249 = u32;
pub const anon_249___width: anon_249 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_250 {}
impl Clone for __is_integer_nonstrict_250 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_250 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_251 = u32;
pub const anon_251___width: anon_251 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_252 {}
impl Clone for __numeric_traits_integer_252 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_252 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_long____is_signed_253: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_long____digits_254: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_long____max_255: std::cell::LazyCell<i64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_256 {}
impl Clone for __numeric_traits_integer_256 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_256 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_257:
    std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____digits_258: std::cell::LazyCell<
    i32,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____max_259: std::cell::LazyCell<
    u64,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_260 {}
impl Clone for __numeric_traits_floating_260 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_260 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_261 {}
impl Clone for __numeric_traits_floating_261 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_261 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_262 {}
impl Clone for __numeric_traits_floating_262 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_262 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_263 {}
impl Clone for __numeric_traits_263 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_263 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut piecewise_construct_264: std::cell::LazyCell<std_piecewise_construct_t> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct tuple_265 {}
impl Clone for tuple_265 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_265 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_265 {}
impl tuple_265 {}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_266 {}
impl Clone for iterator_266 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_266 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_267 {}
impl Clone for iterator_267 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_267 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_267 {}
impl iterator_267 {}
impl iterator_267 {}
impl iterator_267 {
    pub unsafe fn copy_assign(&mut self, _a0: *const iterator_267) -> *mut iterator_267 {
        unimplemented!()
    }
}
impl iterator_267 {
    pub unsafe fn move_assign(&mut self, _a0: *mut iterator_267) -> *mut iterator_267 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_268 {}
impl Clone for iterator_268 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_268 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_268 {
    pub unsafe fn copy_assign(&mut self, _a0: *const iterator_268) -> *mut iterator_268 {
        unimplemented!()
    }
}
impl iterator_268 {
    pub unsafe fn move_assign(&mut self, _a0: *mut iterator_268) -> *mut iterator_268 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_269 {}
impl Clone for iterator_269 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_269 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_269 {
    pub unsafe fn copy_assign(&mut self, _a0: *const iterator_269) -> *mut iterator_269 {
        unimplemented!()
    }
}
impl iterator_269 {
    pub unsafe fn move_assign(&mut self, _a0: *mut iterator_269) -> *mut iterator_269 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_270 {}
impl Clone for iterator_traits_270 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_270 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_271 {}
impl Clone for iterator_traits_271 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_271 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_272 {}
impl Clone for iterator_traits_272 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_272 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_273 {}
impl Clone for iterator_traits_273 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_273 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_274 {}
impl Clone for iterator_traits_274 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_274 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_275 {}
impl Clone for iterator_traits_275 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_275 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_276 {}
impl Clone for iterator_traits_276 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_276 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_277 {}
impl Clone for iterator_traits_277 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_277 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __iterator_traits_278 {}
impl Clone for __iterator_traits_278 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_278 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __iterator_traits_279 {}
impl Clone for __iterator_traits_279 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_279 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct reverse_iterator_280 {}
impl Clone for reverse_iterator_280 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_280 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_280 {
    pub unsafe fn copy_from(__x: *const reverse_iterator_280) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_280 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const reverse_iterator_280,
    ) -> *mut reverse_iterator_280 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct reverse_iterator_281 {}
impl Clone for reverse_iterator_281 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_281 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_281 {
    pub unsafe fn copy_from(__x: *const reverse_iterator_281) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_281 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const reverse_iterator_281,
    ) -> *mut reverse_iterator_281 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub unsafe fn copy_from(_a0: *const std_exception) -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub unsafe fn copy_assign(&mut self, _a0: *const std_exception) -> *mut std_exception {
        unimplemented!()
    }
}
impl std_exception {
    pub unsafe fn move_from(_a0: *mut std_exception) -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub unsafe fn move_assign(&mut self, _a0: *mut std_exception) -> *mut std_exception {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl std_bad_alloc {
    pub unsafe fn copy_from(_a0: *const std_bad_alloc) -> Self {
        unimplemented!()
    }
}
impl std_bad_alloc {
    pub unsafe fn copy_assign(&mut self, _a0: *const std_bad_alloc) -> *mut std_bad_alloc {
        unimplemented!()
    }
}
pub type std_align_val_t = usize;
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_282 {}
impl Clone for __new_allocator_282 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_282 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_282 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_282) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_282 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_282,
    ) -> *mut __new_allocator_282 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_283 {}
impl Clone for __new_allocator_283 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_283 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_283 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_283) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_283 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_283,
    ) -> *mut __new_allocator_283 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_284 {}
impl Clone for __new_allocator_284 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_284 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_284 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_284 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_284) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_284 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_284,
    ) -> *mut __new_allocator_284 {
        unimplemented!()
    }
}
impl __new_allocator_284 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut u32 {
        unimplemented!()
    }
}
impl __new_allocator_284 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut u32, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_284 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_285 {}
impl Clone for __new_allocator_285 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_285 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_285 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_285 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_285) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_285 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_285,
    ) -> *mut __new_allocator_285 {
        unimplemented!()
    }
}
impl __new_allocator_285 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut u64 {
        unimplemented!()
    }
}
impl __new_allocator_285 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut u64, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_285 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_286 {}
impl Clone for __new_allocator_286 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_286 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_286 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_286 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_286) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_286 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_286,
    ) -> *mut __new_allocator_286 {
        unimplemented!()
    }
}
impl __new_allocator_286 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut i32 {
        unimplemented!()
    }
}
impl __new_allocator_286 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut i32, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_286 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_287 {}
impl Clone for __new_allocator_287 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_287 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_287 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_287 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_287) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_287 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_287,
    ) -> *mut __new_allocator_287 {
        unimplemented!()
    }
}
impl __new_allocator_287 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut u8 {
        unimplemented!()
    }
}
impl __new_allocator_287 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut u8, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_287 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_288 {}
impl Clone for allocator_288 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_288 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_288 {}
impl allocator_288 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_289 {}
impl Clone for allocator_289 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_289 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_289 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_289 {
    pub unsafe fn copy_from(__a: *const allocator_289) -> Self {
        unimplemented!()
    }
}
impl allocator_289 {
    pub unsafe fn copy_assign(&mut self, _a0: *const allocator_289) -> *mut allocator_289 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
impl allocator_290 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_290 {
    pub unsafe fn copy_from(__a: *const allocator_290) -> Self {
        unimplemented!()
    }
}
impl allocator_290 {
    pub unsafe fn copy_assign(&mut self, _a0: *const allocator_290) -> *mut allocator_290 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_291 {
    pub unsafe fn copy_from(__a: *const allocator_291) -> Self {
        unimplemented!()
    }
}
impl allocator_291 {
    pub unsafe fn copy_assign(&mut self, _a0: *const allocator_291) -> *mut allocator_291 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_292 {
    pub unsafe fn copy_from(__a: *const allocator_292) -> Self {
        unimplemented!()
    }
}
impl allocator_292 {
    pub unsafe fn copy_assign(&mut self, _a0: *const allocator_292) -> *mut allocator_292 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_traits_293 {}
impl Clone for allocator_traits_293 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_293 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_293 {
    pub unsafe fn allocate_1(__a: *mut allocator_289, mut __n: u64) -> *mut u32 {
        unimplemented!()
    }
}
impl allocator_traits_293 {
    pub unsafe fn deallocate(__a: *mut allocator_289, mut __p: *mut u32, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_293 {
    pub unsafe fn max_size(__a: *const allocator_289) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_traits_294 {}
impl Clone for allocator_traits_294 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_294 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_294 {
    pub unsafe fn allocate_1(__a: *mut allocator_290, mut __n: u64) -> *mut u64 {
        unimplemented!()
    }
}
impl allocator_traits_294 {
    pub unsafe fn deallocate(__a: *mut allocator_290, mut __p: *mut u64, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_294 {
    pub unsafe fn max_size(__a: *const allocator_290) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn allocate_1(__a: *mut allocator_291, mut __n: u64) -> *mut i32 {
        unimplemented!()
    }
}
impl allocator_traits_295 {
    pub unsafe fn deallocate(__a: *mut allocator_291, mut __p: *mut i32, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_295 {
    pub unsafe fn max_size(__a: *const allocator_291) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn allocate_1(__a: *mut allocator_292, mut __n: u64) -> *mut u8 {
        unimplemented!()
    }
}
impl allocator_traits_296 {
    pub unsafe fn deallocate(__a: *mut allocator_292, mut __p: *mut u8, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_296 {
    pub unsafe fn max_size(__a: *const allocator_292) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_298 {}
impl Clone for __alloc_traits_298 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_298 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_299 {}
impl Clone for rebind_299 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_299 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_300 {}
impl Clone for __alloc_traits_300 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_300 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_301 {}
impl Clone for rebind_301 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_301 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_302 {}
impl Clone for __alloc_traits_302 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_302 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_303 {}
impl Clone for rebind_303 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_303 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_304 {}
impl Clone for __alloc_traits_304 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_304 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_305 {}
impl Clone for rebind_305 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_305 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _UninitDestroyGuard_306 {
    pub _M_first: *mut u64,
    pub _M_cur: *mut *mut u64,
}
impl Clone for _UninitDestroyGuard_306 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _UninitDestroyGuard_306 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_306 {
    pub unsafe fn new(__first: *mut *mut u64) -> Self {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_306 {
    pub unsafe fn release(&mut self) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_307 {}
impl Clone for initializer_list_307 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_307 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_307 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl initializer_list_307 {
    pub unsafe fn begin(&self) -> *const bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_308 {}
impl Clone for initializer_list_308 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_308 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_308 {}
impl initializer_list_308 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_309 {}
impl Clone for initializer_list_309 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_309 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_309 {}
impl initializer_list_309 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_310 {}
impl Clone for initializer_list_310 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_310 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_310 {}
impl initializer_list_310 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_311 {}
impl Clone for initializer_list_311 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_311 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_311 {}
impl initializer_list_311 {}
#[derive(VaArg, FnPtrArg)]
pub struct _Vector_base_312 {
    pub _M_impl: std__Vector_base_unsigned_int__std_allocator_unsigned_int____Vector_impl,
}
impl Clone for _Vector_base_312 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_312 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std__Vector_base_unsigned_int__std_allocator_unsigned_int____Vector_impl_data {
    pub _M_start: *mut u32,
    pub _M_finish: *mut u32,
    pub _M_end_of_storage: *mut u32,
}
impl Clone for std__Vector_base_unsigned_int__std_allocator_unsigned_int____Vector_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_unsigned_int__std_allocator_unsigned_int____Vector_impl_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_unsigned_int__std_allocator_unsigned_int____Vector_impl_data {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_unsigned_int__std_allocator_unsigned_int____Vector_impl_data {
    pub unsafe fn move_from(
        __x: *mut std__Vector_base_unsigned_int__std_allocator_unsigned_int____Vector_impl_data,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std__Vector_base_unsigned_int__std_allocator_unsigned_int____Vector_impl {}
impl Clone for std__Vector_base_unsigned_int__std_allocator_unsigned_int____Vector_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_unsigned_int__std_allocator_unsigned_int____Vector_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_unsigned_int__std_allocator_unsigned_int____Vector_impl {
    pub unsafe fn new_1(__a: *const allocator_289) -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_unsigned_int__std_allocator_unsigned_int____Vector_impl {
    pub unsafe fn move_from(
        __x: *mut std__Vector_base_unsigned_int__std_allocator_unsigned_int____Vector_impl,
    ) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_312 {
    pub unsafe fn _M_get_Tp_allocator_1(&mut self) -> *mut allocator_289 {
        unimplemented!()
    }
}
impl _Vector_base_312 {
    pub unsafe fn new_1(__a: *const allocator_289) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_312 {
    pub unsafe fn move_from(_a0: *mut _Vector_base_312) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_312 {
    pub unsafe fn _M_allocate(&mut self, mut __n: usize) -> *mut u32 {
        unimplemented!()
    }
}
impl _Vector_base_312 {
    pub unsafe fn _M_deallocate(&mut self, mut __p: *mut u32, mut __n: usize) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _Vector_base_313 {
    pub _M_impl: std__Vector_base_unsigned_long__std_allocator_unsigned_long____Vector_impl,
}
impl Clone for _Vector_base_313 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_313 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std__Vector_base_unsigned_long__std_allocator_unsigned_long____Vector_impl_data {
    pub _M_start: *mut u64,
    pub _M_finish: *mut u64,
    pub _M_end_of_storage: *mut u64,
}
impl Clone for std__Vector_base_unsigned_long__std_allocator_unsigned_long____Vector_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_unsigned_long__std_allocator_unsigned_long____Vector_impl_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_unsigned_long__std_allocator_unsigned_long____Vector_impl_data {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_unsigned_long__std_allocator_unsigned_long____Vector_impl_data {
    pub unsafe fn move_from(
        __x: *mut std__Vector_base_unsigned_long__std_allocator_unsigned_long____Vector_impl_data,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std__Vector_base_unsigned_long__std_allocator_unsigned_long____Vector_impl {}
impl Clone for std__Vector_base_unsigned_long__std_allocator_unsigned_long____Vector_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_unsigned_long__std_allocator_unsigned_long____Vector_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_unsigned_long__std_allocator_unsigned_long____Vector_impl {
    pub unsafe fn new_1(__a: *const allocator_290) -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_unsigned_long__std_allocator_unsigned_long____Vector_impl {
    pub unsafe fn move_from(
        __x: *mut std__Vector_base_unsigned_long__std_allocator_unsigned_long____Vector_impl,
    ) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_313 {
    pub unsafe fn _M_get_Tp_allocator_1(&mut self) -> *mut allocator_290 {
        unimplemented!()
    }
}
impl _Vector_base_313 {
    pub unsafe fn new_1(__a: *const allocator_290) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_313 {
    pub unsafe fn move_from(_a0: *mut _Vector_base_313) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_313 {
    pub unsafe fn _M_allocate(&mut self, mut __n: usize) -> *mut u64 {
        unimplemented!()
    }
}
impl _Vector_base_313 {
    pub unsafe fn _M_deallocate(&mut self, mut __p: *mut u64, mut __n: usize) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _Vector_base_314 {
    pub _M_impl: std__Vector_base_int__std_allocator_int____Vector_impl,
}
impl Clone for _Vector_base_314 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_314 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std__Vector_base_int__std_allocator_int____Vector_impl_data {
    pub _M_start: *mut i32,
    pub _M_finish: *mut i32,
    pub _M_end_of_storage: *mut i32,
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
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__std_allocator_int____Vector_impl_data {
    pub unsafe fn move_from(
        __x: *mut std__Vector_base_int__std_allocator_int____Vector_impl_data,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new_1(__a: *const allocator_291) -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__std_allocator_int____Vector_impl {
    pub unsafe fn move_from(
        __x: *mut std__Vector_base_int__std_allocator_int____Vector_impl,
    ) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_314 {
    pub unsafe fn _M_get_Tp_allocator_1(&mut self) -> *mut allocator_291 {
        unimplemented!()
    }
}
impl _Vector_base_314 {
    pub unsafe fn new_1(__a: *const allocator_291) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_314 {
    pub unsafe fn move_from(_a0: *mut _Vector_base_314) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_314 {
    pub unsafe fn _M_allocate(&mut self, mut __n: usize) -> *mut i32 {
        unimplemented!()
    }
}
impl _Vector_base_314 {
    pub unsafe fn _M_deallocate(&mut self, mut __p: *mut i32, mut __n: usize) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _Vector_base_315 {
    pub _M_impl: std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl,
}
impl Clone for _Vector_base_315 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_315 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl_data {
    pub _M_start: *mut u8,
    pub _M_finish: *mut u8,
    pub _M_end_of_storage: *mut u8,
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
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl_data {
    pub unsafe fn move_from(
        __x: *mut std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl_data,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new_1(__a: *const allocator_292) -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl {
    pub unsafe fn move_from(
        __x: *mut std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl,
    ) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_315 {
    pub unsafe fn _M_get_Tp_allocator_1(&mut self) -> *mut allocator_292 {
        unimplemented!()
    }
}
impl _Vector_base_315 {
    pub unsafe fn new_1(__a: *const allocator_292) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_315 {
    pub unsafe fn move_from(_a0: *mut _Vector_base_315) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_315 {
    pub unsafe fn _M_allocate(&mut self, mut __n: usize) -> *mut u8 {
        unimplemented!()
    }
}
impl _Vector_base_315 {
    pub unsafe fn _M_deallocate(&mut self, mut __p: *mut u8, mut __n: usize) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct vector_316 {}
impl Clone for vector_316 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_316 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl vector_316 {
    pub unsafe fn copy_from(__x: *const vector_316) -> Self {
        unimplemented!()
    }
}
impl vector_316 {
    pub unsafe fn move_from(_a0: *mut vector_316) -> Self {
        unimplemented!()
    }
}
impl vector_316 {
    pub unsafe fn copy_assign(&mut self, __x: *const vector_316) -> *mut vector_316 {
        unimplemented!()
    }
}
impl vector_316 {
    pub unsafe fn move_assign(&mut self, __x: *mut vector_316) -> *mut vector_316 {
        unimplemented!()
    }
}
impl vector_316 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl vector_316 {
    pub unsafe fn operator_index_46(&mut self, mut __n: u64) -> *mut u32 {
        unimplemented!()
    }
}
impl vector_316 {
    pub unsafe fn new_1(
        mut __first: *mut u32,
        mut __last: *mut u32,
        __a: Option<*const allocator_289>,
    ) -> Self {
        unimplemented!()
    }
}
impl vector_316 {
    pub unsafe fn new_2(
        mut __first: *const u32,
        mut __last: *const u32,
        __a: Option<*const allocator_289>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct vector_317 {}
impl Clone for vector_317 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_317 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl vector_317 {
    pub unsafe fn copy_from(__x: *const vector_317) -> Self {
        unimplemented!()
    }
}
impl vector_317 {
    pub unsafe fn move_from(_a0: *mut vector_317) -> Self {
        unimplemented!()
    }
}
impl vector_317 {
    pub unsafe fn copy_assign(&mut self, __x: *const vector_317) -> *mut vector_317 {
        unimplemented!()
    }
}
impl vector_317 {
    pub unsafe fn move_assign(&mut self, __x: *mut vector_317) -> *mut vector_317 {
        unimplemented!()
    }
}
impl vector_317 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl vector_317 {
    pub unsafe fn operator_index_46(&mut self, mut __n: u64) -> *mut u64 {
        unimplemented!()
    }
}
impl vector_317 {
    pub unsafe fn new_1(
        mut __first: *mut u32,
        mut __last: *mut u32,
        __a: Option<*const allocator_290>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct vector_318 {}
impl Clone for vector_318 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_318 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl vector_318 {
    pub unsafe fn copy_from(__x: *const vector_318) -> Self {
        unimplemented!()
    }
}
impl vector_318 {
    pub unsafe fn move_from(_a0: *mut vector_318) -> Self {
        unimplemented!()
    }
}
impl vector_318 {
    pub unsafe fn copy_assign(&mut self, __x: *const vector_318) -> *mut vector_318 {
        unimplemented!()
    }
}
impl vector_318 {
    pub unsafe fn move_assign(&mut self, __x: *mut vector_318) -> *mut vector_318 {
        unimplemented!()
    }
}
impl vector_318 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl vector_318 {
    pub unsafe fn operator_index_46(&mut self, mut __n: u64) -> *mut i32 {
        unimplemented!()
    }
}
impl vector_318 {
    pub unsafe fn new_1(
        mut __first: *mut u32,
        mut __last: *mut u32,
        __a: Option<*const allocator_291>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct vector_319 {}
impl Clone for vector_319 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_319 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl vector_319 {
    pub unsafe fn copy_from(__x: *const vector_319) -> Self {
        unimplemented!()
    }
}
impl vector_319 {
    pub unsafe fn move_from(_a0: *mut vector_319) -> Self {
        unimplemented!()
    }
}
impl vector_319 {
    pub unsafe fn copy_assign(&mut self, __x: *const vector_319) -> *mut vector_319 {
        unimplemented!()
    }
}
impl vector_319 {
    pub unsafe fn move_assign(&mut self, __x: *mut vector_319) -> *mut vector_319 {
        unimplemented!()
    }
}
impl vector_319 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl vector_319 {
    pub unsafe fn operator_index_46(&mut self, mut __n: u64) -> *mut u8 {
        unimplemented!()
    }
}
impl vector_319 {
    pub unsafe fn new_1(
        mut __first: *const u8,
        mut __last: *const u8,
        __a: Option<*const allocator_292>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_320 {}
impl Clone for __hash_base_320 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_320 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_321 {}
impl Clone for __hash_base_321 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_321 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_322 {}
impl Clone for __hash_base_322 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_322 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_323 {}
impl Clone for __hash_base_323 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_323 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_324 {}
impl Clone for __hash_base_324 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_324 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_325 {}
impl Clone for __hash_base_325 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_325 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_326 {}
impl Clone for __hash_base_326 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_326 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_327 {}
impl Clone for __hash_base_327 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_327 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_328 {}
impl Clone for __hash_base_328 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_328 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_329 {}
impl Clone for __hash_base_329 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_329 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_330 {}
impl Clone for __hash_base_330 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_330 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_331 {}
impl Clone for __hash_base_331 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_331 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_332 {}
impl Clone for __hash_base_332 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_332 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_333 {}
impl Clone for __hash_base_333 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_333 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_334 {}
impl Clone for __hash_base_334 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_334 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_335 {}
impl Clone for __hash_base_335 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_335 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_336 {}
impl Clone for __hash_base_336 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_336 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_337 {}
impl Clone for __hash_base_337 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_337 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_338 {}
impl Clone for __hash_base_338 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_338 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_339 {}
impl Clone for __hash_base_339 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_339 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_340 {}
impl Clone for __hash_base_340 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_340 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct hash_341 {}
impl Clone for hash_341 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_341 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn hash(
        mut __ptr: *const ::libc::c_void,
        mut __clength: usize,
        mut __seed: Option<usize>,
    ) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub unsafe fn hash_342(__val: *const f32) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub unsafe fn hash_343(__val: *const f64) -> usize {
        unimplemented!()
    }
}
pub type anon_344 = u32;
pub const anon_344__S_word_bit: anon_344 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn copy_from(_a0: *const std__Bit_reference) -> Self {
        unimplemented!()
    }
}
impl std__Bit_reference {
    pub unsafe fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl std__Bit_reference {
    pub unsafe fn operator_assign_5(&mut self, mut __x: bool) -> *mut std__Bit_reference {
        unimplemented!()
    }
}
impl std__Bit_reference {
    pub unsafe fn copy_assign(
        &mut self,
        __x: *const std__Bit_reference,
    ) -> *mut std__Bit_reference {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std__Bit_iterator_base {
    pub _M_p: *mut u64,
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
    pub unsafe fn _M_assume_normalized(&self) {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub unsafe fn new(mut __x: *mut u64, mut __y: u32) -> Self {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub unsafe fn _M_bump_up(&mut self) {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub unsafe fn _M_bump_down(&mut self) {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub unsafe fn _M_incr(&mut self, mut __i: i64) {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {}
impl std__Bit_iterator_base {}
impl std__Bit_iterator_base {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std__Bit_iterator_base,
    ) -> *mut std__Bit_iterator_base {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut std__Bit_iterator_base,
    ) -> *mut std__Bit_iterator_base {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new_1(mut __x: *mut u64, mut __y: u32) -> Self {
        unimplemented!()
    }
}
impl std__Bit_iterator {
    pub unsafe fn operator_deref(&self) -> std__Bit_reference {
        unimplemented!()
    }
}
impl std__Bit_iterator {
    pub unsafe fn operator_add_assign(&mut self, mut __i: i64) -> *mut std__Bit_iterator {
        unimplemented!()
    }
}
impl std__Bit_iterator {
    pub unsafe fn operator_sub_assign(&mut self, mut __i: i64) -> *mut std__Bit_iterator {
        unimplemented!()
    }
}
impl std__Bit_iterator {}
impl std__Bit_iterator {}
impl std__Bit_iterator {
    pub unsafe fn copy_assign(&mut self, _a0: *const std__Bit_iterator) -> *mut std__Bit_iterator {
        unimplemented!()
    }
}
impl std__Bit_iterator {
    pub unsafe fn move_assign(&mut self, _a0: *mut std__Bit_iterator) -> *mut std__Bit_iterator {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
impl std__Bit_const_iterator {
    pub unsafe fn operator_deref(&self) -> bool {
        unimplemented!()
    }
}
impl std__Bit_const_iterator {
    pub unsafe fn operator_add_assign(&mut self, mut __i: i64) -> *mut std__Bit_const_iterator {
        unimplemented!()
    }
}
impl std__Bit_const_iterator {
    pub unsafe fn operator_sub_assign(&mut self, mut __i: i64) -> *mut std__Bit_const_iterator {
        unimplemented!()
    }
}
impl std__Bit_const_iterator {}
impl std__Bit_const_iterator {}
impl std__Bit_const_iterator {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std__Bit_const_iterator,
    ) -> *mut std__Bit_const_iterator {
        unimplemented!()
    }
}
impl std__Bit_const_iterator {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut std__Bit_const_iterator,
    ) -> *mut std__Bit_const_iterator {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct binary_function_345 {}
impl Clone for binary_function_345 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_345 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct greater_346 {}
impl Clone for greater_346 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_346 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct less_347 {}
impl Clone for less_347 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_347 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct greater_equal_348 {}
impl Clone for greater_equal_348 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_348 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct less_equal_349 {}
impl Clone for less_equal_349 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_349 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut allocator_arg_350: std::cell::LazyCell<std_allocator_arg_t> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg)]
pub struct std___uses_alloc0 {
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
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut src: [u32; 3] = [1_u32, 2_u32, 3_u32];
    let mut v1: vector_316 = vector_316::new_1(
        { src.as_mut_ptr() },
        { src.as_mut_ptr().offset((3) as isize) },
        None,
    );
    (if ((unsafe { vector_316::size(&v1) }) == (3_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_351(
                c"v1.size() == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { vector_316::operator_index_46(&mut v1, 0_u64) })) == (1_u32))
        && ((*(unsafe { vector_316::operator_index_46(&mut v1, 1_u64) })) == (2_u32)))
        && ((*(unsafe { vector_316::operator_index_46(&mut v1, 2_u64) })) == (3_u32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_351(
                c"v1[0] == 1 && v1[1] == 2 && v1[2] == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut v2: vector_317 = vector_317::new_1(
        { src.as_mut_ptr() },
        { src.as_mut_ptr().offset((3) as isize) },
        None,
    );
    (if ((unsafe { vector_317::size(&v2) }) == (3_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_351(
                c"v2.size() == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { vector_317::operator_index_46(&mut v2, 0_u64) })) == (1_u64))
        && ((*(unsafe { vector_317::operator_index_46(&mut v2, 1_u64) })) == (2_u64)))
        && ((*(unsafe { vector_317::operator_index_46(&mut v2, 2_u64) })) == (3_u64))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_351(
                c"v2[0] == 1 && v2[1] == 2 && v2[2] == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut v3: vector_318 = vector_318::new_1(
        { src.as_mut_ptr() },
        { src.as_mut_ptr().offset((3) as isize) },
        None,
    );
    (if ((unsafe { vector_318::size(&v3) }) == (3_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_351(
                c"v3.size() == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { vector_318::operator_index_46(&mut v3, 0_u64) })) == (1))
        && ((*(unsafe { vector_318::operator_index_46(&mut v3, 1_u64) })) == (2)))
        && ((*(unsafe { vector_318::operator_index_46(&mut v3, 2_u64) })) == (3))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_351(
                c"v3[0] == 1 && v3[1] == 2 && v3[2] == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let src1: [u32; 3] = [1_u32, 2_u32, 3_u32];
    let mut v4: vector_316 =
        vector_316::new_2({ src1.as_ptr() }, { (unsafe { end_352(&src1) }) }, None);
    (if ((unsafe { vector_316::size(&v4) }) == (3_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_351(
                c"v4.size() == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { vector_316::operator_index_46(&mut v4, 0_u64) })) == (1_u32))
        && ((*(unsafe { vector_316::operator_index_46(&mut v4, 1_u64) })) == (2_u32)))
        && ((*(unsafe { vector_316::operator_index_46(&mut v4, 2_u64) })) == (3_u32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_351(
                c"v4[0] == 1 && v4[1] == 2 && v4[2] == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut buf: [u8; 5] = [10_u8, 20_u8, 30_u8, 40_u8, 50_u8];
    let mut start: *const u8 = (buf.as_mut_ptr()).cast_const();
    let mut len: usize = 5_usize;
    let mut v5: vector_319 = vector_319::new_1({ start }, { start.offset((len) as isize) }, None);
    (if ((unsafe { vector_319::size(&v5) }) == (5_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_351(
                c"v5.size() == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { vector_319::operator_index_46(&mut v5, 0_u64) })) as i32) == (10))
        && (((*(unsafe { vector_319::operator_index_46(&mut v5, 4_u64) })) as i32) == (50))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_351(
                c"v5[0] == 10 && v5[4] == 50".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct std___uses_alloc0__Sink;
pub unsafe fn __assert_fail_351(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn end_352(__arr: *const [u32; 3]) -> *const u32 {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
