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
impl integral_constant_2 {}
impl integral_constant_2 {}
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_81 {}
impl Clone for is_function_81 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_81 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_82 {}
impl Clone for is_function_82 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_82 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_106 {}
impl Clone for remove_cv_106 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_106 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_107 {}
impl Clone for remove_cv_107 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_107 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_108 {}
impl Clone for remove_cv_108 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_108 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_109 {}
impl Clone for remove_cv_109 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_109 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_110 {}
impl Clone for remove_cv_110 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_110 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_111 {}
impl Clone for remove_cv_111 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_111 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_116 {}
impl Clone for is_const_116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_116 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_117 {}
impl Clone for is_const_117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_117 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_122 {}
impl Clone for __is_array_unknown_bounds_122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_122 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_138 {}
impl Clone for __is_integral_helper_138 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_138 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_139 {}
impl Clone for __is_integral_helper_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_139 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_140 {}
impl Clone for __is_integral_helper_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_140 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_141 {}
impl Clone for __is_integral_helper_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_142 {}
impl Clone for __is_integral_helper_142 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_142 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_143 {}
impl Clone for __is_integral_helper_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_143 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_144 {}
impl Clone for is_integral_144 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_144 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_145 {}
impl Clone for is_integral_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_145 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_146 {}
impl Clone for is_integral_146 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_146 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_147 {}
impl Clone for is_integral_147 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_147 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_148 {}
impl Clone for is_integral_148 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_148 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_149 {}
impl Clone for is_integral_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_150 {}
impl Clone for is_integral_150 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_150 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_151 {}
impl Clone for is_integral_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_152 {}
impl Clone for is_integral_152 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_152 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_153 {}
impl Clone for is_integral_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_floating_point_helper_154 {}
impl Clone for __is_floating_point_helper_154 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_floating_point_helper_154 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_floating_point_155 {}
impl Clone for is_floating_point_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_floating_point_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_pointer_156 {}
impl Clone for is_pointer_156 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_156 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_pointer_157 {}
impl Clone for is_pointer_157 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_157 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_arithmetic_158 {}
impl Clone for is_arithmetic_158 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_158 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_arithmetic_159 {}
impl Clone for is_arithmetic_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_159 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_arithmetic_160 {}
impl Clone for is_arithmetic_160 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_160 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_arithmetic_161 {}
impl Clone for is_arithmetic_161 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_161 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_arithmetic_162 {}
impl Clone for is_arithmetic_162 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_162 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_scalar_163 {}
impl Clone for is_scalar_163 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_163 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_scalar_164 {}
impl Clone for is_scalar_164 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_164 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_scalar_165 {}
impl Clone for is_scalar_165 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_165 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_scalar_166 {}
impl Clone for is_scalar_166 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_166 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_scalar_167 {}
impl Clone for is_scalar_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_167 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_168 {}
impl Clone for is_same_168 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_168 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_169 {}
impl Clone for is_same_169 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_169 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_170 {}
impl Clone for is_same_170 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_170 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_171 {}
impl Clone for is_same_171 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_171 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_172 {}
impl Clone for is_same_172 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_172 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_173 {}
impl Clone for is_same_173 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_173 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_174 {}
impl Clone for is_same_174 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_174 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_175 {}
impl Clone for is_same_175 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_175 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_176 {}
impl Clone for is_same_176 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_176 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_177 {}
impl Clone for is_volatile_177 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_177 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_178 {}
impl Clone for is_volatile_178 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_178 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_179 {}
impl Clone for is_volatile_179 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_179 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_180 {}
impl Clone for is_volatile_180 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_180 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_181 {}
impl Clone for is_volatile_181 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_181 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_182 {}
impl Clone for is_volatile_182 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_182 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_empty_183 {}
impl Clone for is_empty_183 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_empty_183 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_empty_184 {}
impl Clone for is_empty_184 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_empty_184 {
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
pub struct __is_destructible_safe_185 {}
impl Clone for __is_destructible_safe_185 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_185 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_destructible_safe_186 {}
impl Clone for __is_destructible_safe_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_destructible_safe_187 {}
impl Clone for __is_destructible_safe_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_187 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_destructible_safe_188 {}
impl Clone for __is_destructible_safe_188 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_188 {
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
pub struct __is_nt_destructible_safe_189 {}
impl Clone for __is_nt_destructible_safe_189 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_safe_189 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_nt_destructible_safe_190 {}
impl Clone for __is_nt_destructible_safe_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_safe_190 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_destructible_191 {}
impl Clone for is_nothrow_destructible_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_destructible_191 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_destructible_192 {}
impl Clone for is_nothrow_destructible_192 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_destructible_192 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_constructible_193 {}
impl Clone for is_constructible_193 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_193 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_constructible_194 {}
impl Clone for is_constructible_194 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_194 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_constructible_195 {}
impl Clone for is_constructible_195 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_195 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_constructible_196 {}
impl Clone for is_constructible_196 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_196 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_default_constructible_197 {}
impl Clone for is_nothrow_default_constructible_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_197 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_198 {}
impl Clone for is_nothrow_copy_constructible_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_199 {}
impl Clone for is_nothrow_copy_constructible_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_199 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_200 {}
impl Clone for is_nothrow_copy_constructible_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_200 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_201 {}
impl Clone for is_nothrow_copy_constructible_201 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_201 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_202 {}
impl Clone for is_nothrow_copy_constructible_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_202 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_203 {}
impl Clone for is_nothrow_copy_constructible_203 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_203 {
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
pub struct is_trivially_destructible_204 {}
impl Clone for is_trivially_destructible_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_204 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_trivially_destructible_205 {}
impl Clone for is_trivially_destructible_205 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_205 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_trivially_destructible_206 {}
impl Clone for is_trivially_destructible_206 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_206 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_trivially_destructible_207 {}
impl Clone for is_trivially_destructible_207 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_207 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_208 {}
impl Clone for is_convertible_208 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_208 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_209 {}
impl Clone for is_convertible_209 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_209 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_210 {}
impl Clone for is_convertible_210 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_210 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_211 {}
impl Clone for is_convertible_211 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_211 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_nothrow_new_constructible_impl_212 {}
impl Clone for __is_nothrow_new_constructible_impl_212 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_212 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_nothrow_new_constructible_impl_213 {}
impl Clone for __is_nothrow_new_constructible_impl_213 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_213 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_nothrow_new_constructible_impl_214 {}
impl Clone for __is_nothrow_new_constructible_impl_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_214 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_nothrow_new_constructible_impl_215 {}
impl Clone for __is_nothrow_new_constructible_impl_215 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_215 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_216 {}
impl Clone for remove_reference_216 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_216 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_217 {}
impl Clone for remove_reference_217 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_217 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_218 {}
impl Clone for remove_reference_218 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_218 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_219 {}
impl Clone for remove_reference_219 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_219 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_220 {}
impl Clone for remove_reference_220 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_220 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_221 {}
impl Clone for remove_reference_221 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_221 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_222 {}
impl Clone for remove_reference_222 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_222 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_223 {}
impl Clone for remove_reference_223 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_223 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_224 {}
impl Clone for __cv_selector_224 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_224 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_225 {}
impl Clone for __cv_selector_225 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_225 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_226 {}
impl Clone for __cv_selector_226 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_226 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_227 {}
impl Clone for __cv_selector_227 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_227 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_228 {}
impl Clone for __cv_selector_228 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_228 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_229 {}
impl Clone for __match_cv_qualifiers_229 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_229 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_230 {}
impl Clone for __match_cv_qualifiers_230 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_230 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_231 {}
impl Clone for __match_cv_qualifiers_231 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_231 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_232 {}
impl Clone for __match_cv_qualifiers_232 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_232 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_233 {}
impl Clone for __match_cv_qualifiers_233 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_233 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_234 {}
impl Clone for __match_cv_qualifiers_234 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_234 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_235 {}
impl Clone for __make_unsigned_235 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_235 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_236 {}
impl Clone for __make_unsigned_236 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_236 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_237 {}
impl Clone for __make_unsigned_237 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_237 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_238 {}
impl Clone for __make_unsigned_238 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_238 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_239 {}
impl Clone for __make_unsigned_selector_239 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_239 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_240 {}
impl Clone for __make_unsigned_selector_240 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_240 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_241 {}
impl Clone for __make_unsigned_selector_241 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_241 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_242 {}
impl Clone for __make_unsigned_selector_242 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_242 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_243 {}
impl Clone for __make_unsigned_selector_243 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_243 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_244 {}
impl Clone for __make_unsigned_selector_244 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_244 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_245 {}
impl Clone for __make_unsigned_selector_245 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_245 {
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
pub struct _List_249 {}
impl Clone for _List_249 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_249 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_250 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_251 {}
impl Clone for _List_251 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_251 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_252 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_253 {}
impl Clone for _List_253 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_253 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_254 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_255 {}
impl Clone for _List_255 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_255 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_256 {}
impl Clone for _List_256 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_256 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_257 {}
impl Clone for _List_257 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_257 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_258 {}
impl Clone for __select_258 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_258 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_259 {}
impl Clone for __select_259 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_259 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_260 {}
impl Clone for __select_260 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_260 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_261 {}
impl Clone for __select_261 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_261 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_262 {}
impl Clone for __select_262 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_262 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct make_unsigned_263 {}
impl Clone for make_unsigned_263 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for make_unsigned_263 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_264 {}
impl Clone for __make_signed_264 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_264 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_265 {}
impl Clone for __make_signed_265 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_265 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_266 {}
impl Clone for __make_signed_selector_266 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_266 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_267 {}
impl Clone for __make_signed_selector_267 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_267 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_268 {}
impl Clone for __make_signed_selector_268 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_268 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_269 {}
impl Clone for __make_signed_selector_269 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_269 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_270 {}
impl Clone for __make_signed_selector_270 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_270 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_pointer_271 {}
impl Clone for remove_pointer_271 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_pointer_271 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_pointer_272 {}
impl Clone for remove_pointer_272 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_pointer_272 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_pointer_273 {}
impl Clone for remove_pointer_273 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_pointer_273 {
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
pub struct common_type_274 {}
impl Clone for common_type_274 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_274 {
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
pub struct __detector_275 {}
impl Clone for __detector_275 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __detector_275 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __detector_276 {}
impl Clone for __detector_276 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __detector_276 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __detector_277 {}
impl Clone for __detector_277 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __detector_277 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __detector_278 {}
impl Clone for __detector_278 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __detector_278 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __detector_279 {}
impl Clone for __detector_279 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __detector_279 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __detector_280 {}
impl Clone for __detector_280 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __detector_280 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __detector_281 {}
impl Clone for __detector_281 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __detector_281 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __detector_282 {}
impl Clone for __detector_282 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __detector_282 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __detector_283 {}
impl Clone for __detector_283 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __detector_283 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __detector_284 {}
impl Clone for __detector_284 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __detector_284 {
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
pub static mut __is_nothrow_new_constructible_285: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __is_nothrow_new_constructible_286: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __is_nothrow_new_constructible_287: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __is_nothrow_new_constructible_288: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
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
pub struct __are_same_289 {}
impl Clone for __are_same_289 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_289 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_290 = u32;
pub const anon_290___value: anon_290 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_291 {}
impl Clone for __are_same_291 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_291 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_292 = u32;
pub const anon_292___value: anon_292 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_293 {}
impl Clone for __are_same_293 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_293 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_294 = u32;
pub const anon_294___value: anon_294 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_295 {}
impl Clone for __are_same_295 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_295 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_296 = u32;
pub const anon_296___value: anon_296 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_297 {}
impl Clone for __are_same_297 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_297 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_298 = u32;
pub const anon_298___value: anon_298 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_299 {}
impl Clone for __are_same_299 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_299 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_300 = u32;
pub const anon_300___value: anon_300 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
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
pub const anon_302___value: anon_302 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
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
pub const anon_304___value: anon_304 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_305 {}
impl Clone for __is_integer_305 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_305 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_306 = u32;
pub const anon_306___value: anon_306 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_307 {}
impl Clone for __is_integer_307 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_307 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_308 = u32;
pub const anon_308___value: anon_308 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_309 {}
impl Clone for __is_integer_309 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_309 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_310 = u32;
pub const anon_310___value: anon_310 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_311 {}
impl Clone for __is_integer_311 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_311 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_312 = u32;
pub const anon_312___value: anon_312 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_313 {}
impl Clone for __is_integer_313 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_313 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_314 = u32;
pub const anon_314___value: anon_314 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_nonvolatile_trivially_copyable_315 {}
impl Clone for __is_nonvolatile_trivially_copyable_315 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nonvolatile_trivially_copyable_315 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_316 = u32;
pub const anon_316___value: anon_316 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_nonvolatile_trivially_copyable_317 {}
impl Clone for __is_nonvolatile_trivially_copyable_317 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nonvolatile_trivially_copyable_317 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_318 = u32;
pub const anon_318___value: anon_318 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_nonvolatile_trivially_copyable_319 {}
impl Clone for __is_nonvolatile_trivially_copyable_319 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nonvolatile_trivially_copyable_319 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_320 = u32;
pub const anon_320___value: anon_320 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_321 {}
impl Clone for __memcpyable_321 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_321 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_322 {}
impl Clone for __memcpyable_322 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_322 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_323 {}
impl Clone for __memcpyable_323 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_323 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_324 = u32;
pub const anon_324___value: anon_324 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_325 {}
impl Clone for __memcpyable_325 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_325 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_326 = u32;
pub const anon_326___value: anon_326 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_327 {}
impl Clone for __memcpyable_327 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_327 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_328 {}
impl Clone for __memcpyable_328 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_328 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_integer_329 {}
impl Clone for __memcpyable_integer_329 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_329 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_integer_330 {}
impl Clone for __memcpyable_integer_330 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_330 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_331 = u32;
pub const anon_331___width: anon_331 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_integer_332 {}
impl Clone for __memcpyable_integer_332 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_332 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_333 = u32;
pub const anon_333___width: anon_333 = 32;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_integer_334 {}
impl Clone for __memcpyable_integer_334 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_334 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_335 = u32;
pub const anon_335___width: anon_335 = 32;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_336 {}
impl Clone for __is_integer_nonstrict_336 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_336 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_337 = u32;
pub const anon_337___width: anon_337 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_338 {}
impl Clone for __is_integer_nonstrict_338 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_338 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_339 = u32;
pub const anon_339___width: anon_339 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_340 {}
impl Clone for __numeric_traits_integer_340 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_340 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_long____is_signed_341: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_long____digits_342: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_long____max_343: std::cell::LazyCell<i64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_344 {}
impl Clone for __numeric_traits_integer_344 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_344 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_345:
    std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____digits_346: std::cell::LazyCell<
    i32,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____max_347: std::cell::LazyCell<
    u64,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_348 {}
impl Clone for __numeric_traits_floating_348 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_348 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_349 {}
impl Clone for __numeric_traits_floating_349 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_349 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_350 {}
impl Clone for __numeric_traits_floating_350 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_350 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_351 {}
impl Clone for __numeric_traits_351 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_351 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_352 {}
impl Clone for __numeric_traits_352 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_352 {
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
pub static mut piecewise_construct_353: std::cell::LazyCell<std_piecewise_construct_t> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct tuple_354 {}
impl Clone for tuple_354 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_354 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_354 {}
impl tuple_354 {}
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
pub struct iterator_355 {}
impl Clone for iterator_355 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_355 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_356 {}
impl Clone for iterator_356 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_356 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_356 {}
impl iterator_356 {}
impl iterator_356 {}
impl iterator_356 {
    pub unsafe fn copy_assign(&mut self, _a0: *const iterator_356) -> *mut iterator_356 {
        unimplemented!()
    }
}
impl iterator_356 {
    pub unsafe fn move_assign(&mut self, _a0: *mut iterator_356) -> *mut iterator_356 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_357 {}
impl Clone for iterator_357 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_357 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_357 {
    pub unsafe fn copy_assign(&mut self, _a0: *const iterator_357) -> *mut iterator_357 {
        unimplemented!()
    }
}
impl iterator_357 {
    pub unsafe fn move_assign(&mut self, _a0: *mut iterator_357) -> *mut iterator_357 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_358 {}
impl Clone for iterator_358 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_358 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_358 {
    pub unsafe fn copy_assign(&mut self, _a0: *const iterator_358) -> *mut iterator_358 {
        unimplemented!()
    }
}
impl iterator_358 {
    pub unsafe fn move_assign(&mut self, _a0: *mut iterator_358) -> *mut iterator_358 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_360 {}
impl Clone for iterator_traits_360 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_360 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_361 {}
impl Clone for iterator_traits_361 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_361 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_362 {}
impl Clone for iterator_traits_362 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_362 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_363 {}
impl Clone for iterator_traits_363 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_363 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_364 {}
impl Clone for iterator_traits_364 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_364 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_365 {}
impl Clone for iterator_traits_365 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_365 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_366 {}
impl Clone for iterator_traits_366 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_366 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_367 {}
impl Clone for iterator_traits_367 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_367 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_368 {}
impl Clone for iterator_traits_368 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_368 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_369 {}
impl Clone for iterator_traits_369 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_369 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_370 {}
impl Clone for iterator_traits_370 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_370 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __iterator_traits_371 {}
impl Clone for __iterator_traits_371 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_371 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __iterator_traits_372 {}
impl Clone for __iterator_traits_372 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_372 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __iterator_traits_373 {}
impl Clone for __iterator_traits_373 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_373 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __replace_first_arg_374 {}
impl Clone for __replace_first_arg_374 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __replace_first_arg_374 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __replace_first_arg_375 {}
impl Clone for __replace_first_arg_375 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __replace_first_arg_375 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_376 {}
impl Clone for __ptr_traits_ptr_to_376 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_376 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_377 {}
impl Clone for __ptr_traits_ptr_to_377 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_377 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_378 {}
impl Clone for pointer_traits_378 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_378 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_379 {}
impl Clone for pointer_traits_379 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_379 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct reverse_iterator_380 {}
impl Clone for reverse_iterator_380 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_380 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_380 {
    pub unsafe fn copy_from(__x: *const reverse_iterator_380) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_380 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const reverse_iterator_380,
    ) -> *mut reverse_iterator_380 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct reverse_iterator_381 {}
impl Clone for reverse_iterator_381 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_381 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_381 {
    pub unsafe fn copy_from(__x: *const reverse_iterator_381) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_381 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const reverse_iterator_381,
    ) -> *mut reverse_iterator_381 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __normal_iterator_382 {}
impl Clone for __normal_iterator_382 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_382 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_382 {
    pub unsafe fn new_1(__i: *const *mut i32) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_382 {
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        unimplemented!()
    }
}
impl __normal_iterator_382 {
    pub unsafe fn operator_add(&self, mut __n: i64) -> __normal_iterator_382 {
        unimplemented!()
    }
}
impl __normal_iterator_382 {
    pub unsafe fn operator_sub(&self, mut __n: i64) -> __normal_iterator_382 {
        unimplemented!()
    }
}
impl __normal_iterator_382 {
    pub unsafe fn base(&self) -> *const *mut i32 {
        unimplemented!()
    }
}
impl __normal_iterator_382 {}
impl __normal_iterator_382 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __normal_iterator_383 {}
impl Clone for __normal_iterator_383 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_383 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_383 {
    pub unsafe fn new_1(__i: *const *const i32) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_383 {
    pub unsafe fn operator_deref(&self) -> *const i32 {
        unimplemented!()
    }
}
impl __normal_iterator_383 {
    pub unsafe fn operator_inc_6(&mut self) -> *mut __normal_iterator_383 {
        unimplemented!()
    }
}
impl __normal_iterator_383 {
    pub unsafe fn base(&self) -> *const *const i32 {
        unimplemented!()
    }
}
impl __normal_iterator_383 {}
impl __normal_iterator_383 {}
impl __normal_iterator_383 {
    pub unsafe fn new_2(__i: *const __normal_iterator_382) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __normal_iterator_384 {}
impl Clone for __normal_iterator_384 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_384 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_384 {
    pub unsafe fn new_1(__i: *const *mut f64) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_384 {
    pub unsafe fn operator_deref(&self) -> *mut f64 {
        unimplemented!()
    }
}
impl __normal_iterator_384 {
    pub unsafe fn operator_sub(&self, mut __n: i64) -> __normal_iterator_384 {
        unimplemented!()
    }
}
impl __normal_iterator_384 {}
impl __normal_iterator_384 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __normal_iterator_385 {}
impl Clone for __normal_iterator_385 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_385 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_385 {
    pub unsafe fn new_1(__i: *const *const f64) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_385 {
    pub unsafe fn base(&self) -> *const *const f64 {
        unimplemented!()
    }
}
impl __normal_iterator_385 {}
impl __normal_iterator_385 {}
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
pub struct __new_allocator_386 {}
impl Clone for __new_allocator_386 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_386 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_386 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_386) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_386 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_386,
    ) -> *mut __new_allocator_386 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_387 {}
impl Clone for __new_allocator_387 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_387 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_387 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_387) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_387 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_387,
    ) -> *mut __new_allocator_387 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_388 {}
impl Clone for __new_allocator_388 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_388 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_388 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_388 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_388) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_388 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_388,
    ) -> *mut __new_allocator_388 {
        unimplemented!()
    }
}
impl __new_allocator_388 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut u32 {
        unimplemented!()
    }
}
impl __new_allocator_388 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut u32, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_388 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_389 {}
impl Clone for __new_allocator_389 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_389 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_389 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_389 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_389) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_389 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_389,
    ) -> *mut __new_allocator_389 {
        unimplemented!()
    }
}
impl __new_allocator_389 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut u64 {
        unimplemented!()
    }
}
impl __new_allocator_389 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut u64, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_389 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_390 {}
impl Clone for __new_allocator_390 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_390 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_390 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_390 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_390) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_390 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_390,
    ) -> *mut __new_allocator_390 {
        unimplemented!()
    }
}
impl __new_allocator_390 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut i32 {
        unimplemented!()
    }
}
impl __new_allocator_390 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut i32, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_390 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_391 {}
impl Clone for __new_allocator_391 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_391 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_391 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_391 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_391) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_391 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_391,
    ) -> *mut __new_allocator_391 {
        unimplemented!()
    }
}
impl __new_allocator_391 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut u8 {
        unimplemented!()
    }
}
impl __new_allocator_391 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut u8, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_391 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
impl allocator_392 {}
impl allocator_392 {}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_393 {
    pub unsafe fn copy_from(__a: *const allocator_393) -> Self {
        unimplemented!()
    }
}
impl allocator_393 {
    pub unsafe fn copy_assign(&mut self, _a0: *const allocator_393) -> *mut allocator_393 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_394 {}
impl Clone for allocator_394 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_394 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_394 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_394 {
    pub unsafe fn copy_from(__a: *const allocator_394) -> Self {
        unimplemented!()
    }
}
impl allocator_394 {
    pub unsafe fn copy_assign(&mut self, _a0: *const allocator_394) -> *mut allocator_394 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_395 {}
impl Clone for allocator_395 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_395 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_395 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_395 {
    pub unsafe fn copy_from(__a: *const allocator_395) -> Self {
        unimplemented!()
    }
}
impl allocator_395 {
    pub unsafe fn copy_assign(&mut self, _a0: *const allocator_395) -> *mut allocator_395 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_396 {}
impl Clone for allocator_396 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_396 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_396 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_396 {
    pub unsafe fn copy_from(__a: *const allocator_396) -> Self {
        unimplemented!()
    }
}
impl allocator_396 {
    pub unsafe fn copy_assign(&mut self, _a0: *const allocator_396) -> *mut allocator_396 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Ptr_398 {}
impl Clone for _Ptr_398 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Ptr_398 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Ptr_399 {}
impl Clone for _Ptr_399 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Ptr_399 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Ptr_400 {}
impl Clone for _Ptr_400 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Ptr_400 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Diff_401 {}
impl Clone for _Diff_401 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Diff_401 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Size_402 {}
impl Clone for _Size_402 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Size_402 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_397 {
    pub unsafe fn allocate_1(__a: *mut TestAllocator_403, mut __n: u64) -> *mut i32 {
        unimplemented!()
    }
}
impl allocator_traits_397 {
    pub unsafe fn deallocate(__a: *mut TestAllocator_403, mut __p: *mut i32, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_397 {
    pub unsafe fn max_size(__a: *const TestAllocator_403) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_397 {
    pub unsafe fn select_on_container_copy_construction(
        __rhs: *const TestAllocator_403,
    ) -> TestAllocator_403 {
        unimplemented!()
    }
}
impl allocator_traits_397 {
    pub unsafe fn construct_404(__a: *mut TestAllocator_403, mut __p: *mut i32, __args: *mut i32) {
        unimplemented!()
    }
}
impl allocator_traits_397 {
    pub unsafe fn construct_405(__a: *mut TestAllocator_403, mut __p: *mut i32) {
        unimplemented!()
    }
}
impl allocator_traits_397 {
    pub unsafe fn construct_406(
        __a: *mut TestAllocator_403,
        mut __p: *mut i32,
        __args: *const i32,
    ) {
        unimplemented!()
    }
}
impl allocator_traits_397 {
    pub unsafe fn destroy_407(__a: *mut TestAllocator_403, mut __p: *mut i32) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_traits_408 {}
impl Clone for allocator_traits_408 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_408 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Ptr_409 {}
impl Clone for _Ptr_409 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Ptr_409 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Ptr_410 {}
impl Clone for _Ptr_410 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Ptr_410 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Ptr_411 {}
impl Clone for _Ptr_411 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Ptr_411 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Diff_412 {}
impl Clone for _Diff_412 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Diff_412 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Size_413 {}
impl Clone for _Size_413 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Size_413 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_408 {
    pub unsafe fn allocate_1(__a: *mut TestAllocator_414, mut __n: u64) -> *mut f64 {
        unimplemented!()
    }
}
impl allocator_traits_408 {
    pub unsafe fn deallocate(__a: *mut TestAllocator_414, mut __p: *mut f64, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_408 {
    pub unsafe fn max_size(__a: *const TestAllocator_414) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_408 {
    pub unsafe fn construct_415(
        __a: *mut TestAllocator_414,
        mut __p: *mut f64,
        __args: *const f64,
    ) {
        unimplemented!()
    }
}
impl allocator_traits_408 {
    pub unsafe fn destroy_416(__a: *mut TestAllocator_414, mut __p: *mut f64) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_traits_417 {}
impl Clone for allocator_traits_417 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_417 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_417 {
    pub unsafe fn allocate_1(__a: *mut allocator_393, mut __n: u64) -> *mut u32 {
        unimplemented!()
    }
}
impl allocator_traits_417 {
    pub unsafe fn deallocate(__a: *mut allocator_393, mut __p: *mut u32, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_417 {
    pub unsafe fn max_size(__a: *const allocator_393) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_traits_418 {}
impl Clone for allocator_traits_418 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_418 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_418 {
    pub unsafe fn allocate_1(__a: *mut allocator_394, mut __n: u64) -> *mut u64 {
        unimplemented!()
    }
}
impl allocator_traits_418 {
    pub unsafe fn deallocate(__a: *mut allocator_394, mut __p: *mut u64, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_418 {
    pub unsafe fn max_size(__a: *const allocator_394) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_traits_419 {}
impl Clone for allocator_traits_419 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_419 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_419 {
    pub unsafe fn allocate_1(__a: *mut allocator_395, mut __n: u64) -> *mut i32 {
        unimplemented!()
    }
}
impl allocator_traits_419 {
    pub unsafe fn deallocate(__a: *mut allocator_395, mut __p: *mut i32, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_419 {
    pub unsafe fn max_size(__a: *const allocator_395) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_traits_420 {}
impl Clone for allocator_traits_420 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_420 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_420 {
    pub unsafe fn allocate_1(__a: *mut allocator_396, mut __n: u64) -> *mut u8 {
        unimplemented!()
    }
}
impl allocator_traits_420 {
    pub unsafe fn deallocate(__a: *mut allocator_396, mut __p: *mut u8, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_420 {
    pub unsafe fn max_size(__a: *const allocator_396) -> u64 {
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
pub struct __rebind_422 {}
impl Clone for __rebind_422 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __rebind_422 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __rebind_423 {}
impl Clone for __rebind_423 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __rebind_423 {
    fn default() -> Self {
        unimplemented!()
    }
}
static mut std___allocator_traits_base___can_construct_424: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut std___allocator_traits_base___has_construct_425: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut std___allocator_traits_base___has_construct_impl_426: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut std___allocator_traits_base___has_new_expr_427: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut std___allocator_traits_base___has_destroy_428: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut std___allocator_traits_base___has_max_size_429: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut std___allocator_traits_base___can_construct_430: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut std___allocator_traits_base___has_construct_431: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut std___allocator_traits_base___has_construct_impl_432: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut std___allocator_traits_base___has_new_expr_433: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut std___allocator_traits_base___has_soccc_434: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut std___allocator_traits_base___can_construct_435: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut std___allocator_traits_base___has_construct_436: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut std___allocator_traits_base___has_construct_impl_437: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut std___allocator_traits_base___has_new_expr_438: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut std___allocator_traits_base___has_max_size_439: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut std___allocator_traits_base___can_construct_440: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut std___allocator_traits_base___has_construct_441: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut std___allocator_traits_base___has_construct_impl_442: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut std___allocator_traits_base___has_new_expr_443: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut std___allocator_traits_base___has_destroy_444: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_alloc_insertable_impl_445 {}
impl Clone for __is_alloc_insertable_impl_445 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_alloc_insertable_impl_445 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_move_insertable_446 {}
impl Clone for __is_move_insertable_446 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_446 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_447 {}
impl Clone for __alloc_traits_447 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_447 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_448 {}
impl Clone for rebind_448 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_448 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __alloc_traits_447 {
    pub unsafe fn _S_select_on_copy(__a: *const TestAllocator_403) -> TestAllocator_403 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_449 {}
impl Clone for __alloc_traits_449 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_449 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_450 {}
impl Clone for rebind_450 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_450 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_451 {}
impl Clone for __alloc_traits_451 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_451 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_452 {}
impl Clone for rebind_452 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_452 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_453 {}
impl Clone for __alloc_traits_453 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_453 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_454 {}
impl Clone for rebind_454 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_454 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_455 {}
impl Clone for __alloc_traits_455 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_455 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_456 {}
impl Clone for rebind_456 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_456 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_457 {}
impl Clone for __alloc_traits_457 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_457 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_458 {}
impl Clone for rebind_458 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_458 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _UninitDestroyGuard_459 {}
impl Clone for _UninitDestroyGuard_459 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _UninitDestroyGuard_459 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_459 {
    pub unsafe fn new(__first: *mut *mut i32, __a: *mut TestAllocator_403) -> Self {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_459 {
    pub unsafe fn release(&mut self) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _UninitDestroyGuard_460 {}
impl Clone for _UninitDestroyGuard_460 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _UninitDestroyGuard_460 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_460 {
    pub unsafe fn new(__first: *mut *mut f64, __a: *mut TestAllocator_414) -> Self {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_460 {
    pub unsafe fn release(&mut self) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _UninitDestroyGuard_461 {
    pub _M_first: *mut u64,
    pub _M_cur: *mut *mut u64,
}
impl Clone for _UninitDestroyGuard_461 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _UninitDestroyGuard_461 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_461 {
    pub unsafe fn new(__first: *mut *mut u64) -> Self {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_461 {
    pub unsafe fn release(&mut self) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
impl initializer_list_462 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl initializer_list_462 {
    pub unsafe fn begin(&self) -> *const bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_466 {}
impl Clone for initializer_list_466 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_466 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_466 {}
impl initializer_list_466 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_467 {}
impl Clone for initializer_list_467 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_467 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_467 {}
impl initializer_list_467 {}
#[derive(VaArg, FnPtrArg)]
pub struct _Vector_base_468 {
    pub _M_impl: std__Vector_base_int__TestAllocator_int____Vector_impl,
}
impl Clone for _Vector_base_468 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_468 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std__Vector_base_int__TestAllocator_int____Vector_impl_data {
    pub _M_start: *mut i32,
    pub _M_finish: *mut i32,
    pub _M_end_of_storage: *mut i32,
}
impl Clone for std__Vector_base_int__TestAllocator_int____Vector_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_int__TestAllocator_int____Vector_impl_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__TestAllocator_int____Vector_impl_data {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__TestAllocator_int____Vector_impl_data {
    pub unsafe fn move_from(
        __x: *mut std__Vector_base_int__TestAllocator_int____Vector_impl_data,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std__Vector_base_int__TestAllocator_int____Vector_impl {}
impl Clone for std__Vector_base_int__TestAllocator_int____Vector_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_int__TestAllocator_int____Vector_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__TestAllocator_int____Vector_impl {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__TestAllocator_int____Vector_impl {
    pub unsafe fn new_2(__a: *const TestAllocator_403) -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__TestAllocator_int____Vector_impl {
    pub unsafe fn move_from(
        __x: *mut std__Vector_base_int__TestAllocator_int____Vector_impl,
    ) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_468 {
    pub unsafe fn _M_get_Tp_allocator_1(&mut self) -> *mut TestAllocator_403 {
        unimplemented!()
    }
}
impl _Vector_base_468 {
    pub unsafe fn _M_get_Tp_allocator_2(&self) -> *const TestAllocator_403 {
        unimplemented!()
    }
}
impl _Vector_base_468 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_468 {
    pub unsafe fn new_2(mut __n: usize, __a: *const TestAllocator_403) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_468 {
    pub unsafe fn move_from(_a0: *mut _Vector_base_468) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_468 {
    pub unsafe fn _M_allocate(&mut self, mut __n: usize) -> *mut i32 {
        unimplemented!()
    }
}
impl _Vector_base_468 {
    pub unsafe fn _M_deallocate(&mut self, mut __p: *mut i32, mut __n: usize) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _Vector_base_469 {
    pub _M_impl: std__Vector_base_double__TestAllocator_double____Vector_impl,
}
impl Clone for _Vector_base_469 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_469 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std__Vector_base_double__TestAllocator_double____Vector_impl_data {
    pub _M_start: *mut f64,
    pub _M_finish: *mut f64,
    pub _M_end_of_storage: *mut f64,
}
impl Clone for std__Vector_base_double__TestAllocator_double____Vector_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_double__TestAllocator_double____Vector_impl_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_double__TestAllocator_double____Vector_impl_data {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_double__TestAllocator_double____Vector_impl_data {
    pub unsafe fn move_from(
        __x: *mut std__Vector_base_double__TestAllocator_double____Vector_impl_data,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std__Vector_base_double__TestAllocator_double____Vector_impl {}
impl Clone for std__Vector_base_double__TestAllocator_double____Vector_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_double__TestAllocator_double____Vector_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_double__TestAllocator_double____Vector_impl {
    pub unsafe fn new_1(__a: *const TestAllocator_414) -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_double__TestAllocator_double____Vector_impl {
    pub unsafe fn move_from(
        __x: *mut std__Vector_base_double__TestAllocator_double____Vector_impl,
    ) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_469 {
    pub unsafe fn _M_get_Tp_allocator_1(&mut self) -> *mut TestAllocator_414 {
        unimplemented!()
    }
}
impl _Vector_base_469 {
    pub unsafe fn new_1(mut __n: usize, __a: *const TestAllocator_414) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_469 {
    pub unsafe fn move_from(_a0: *mut _Vector_base_469) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_469 {
    pub unsafe fn _M_allocate(&mut self, mut __n: usize) -> *mut f64 {
        unimplemented!()
    }
}
impl _Vector_base_469 {
    pub unsafe fn _M_deallocate(&mut self, mut __p: *mut f64, mut __n: usize) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _Vector_base_470 {
    pub _M_impl: std__Vector_base_unsigned_int__std_allocator_unsigned_int____Vector_impl,
}
impl Clone for _Vector_base_470 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_470 {
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
    pub unsafe fn new_1(__a: *const allocator_393) -> Self {
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
impl _Vector_base_470 {
    pub unsafe fn _M_get_Tp_allocator_1(&mut self) -> *mut allocator_393 {
        unimplemented!()
    }
}
impl _Vector_base_470 {
    pub unsafe fn new_1(__a: *const allocator_393) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_470 {
    pub unsafe fn move_from(_a0: *mut _Vector_base_470) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_470 {
    pub unsafe fn _M_allocate(&mut self, mut __n: usize) -> *mut u32 {
        unimplemented!()
    }
}
impl _Vector_base_470 {
    pub unsafe fn _M_deallocate(&mut self, mut __p: *mut u32, mut __n: usize) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _Vector_base_471 {
    pub _M_impl: std__Vector_base_unsigned_long__std_allocator_unsigned_long____Vector_impl,
}
impl Clone for _Vector_base_471 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_471 {
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
    pub unsafe fn new_1(__a: *const allocator_394) -> Self {
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
impl _Vector_base_471 {
    pub unsafe fn _M_get_Tp_allocator_1(&mut self) -> *mut allocator_394 {
        unimplemented!()
    }
}
impl _Vector_base_471 {
    pub unsafe fn new_1(__a: *const allocator_394) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_471 {
    pub unsafe fn move_from(_a0: *mut _Vector_base_471) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_471 {
    pub unsafe fn _M_allocate(&mut self, mut __n: usize) -> *mut u64 {
        unimplemented!()
    }
}
impl _Vector_base_471 {
    pub unsafe fn _M_deallocate(&mut self, mut __p: *mut u64, mut __n: usize) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _Vector_base_472 {
    pub _M_impl: std__Vector_base_int__std_allocator_int____Vector_impl,
}
impl Clone for _Vector_base_472 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_472 {
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
    pub unsafe fn new_1(__a: *const allocator_395) -> Self {
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
impl _Vector_base_472 {
    pub unsafe fn _M_get_Tp_allocator_1(&mut self) -> *mut allocator_395 {
        unimplemented!()
    }
}
impl _Vector_base_472 {
    pub unsafe fn new_1(__a: *const allocator_395) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_472 {
    pub unsafe fn move_from(_a0: *mut _Vector_base_472) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_472 {
    pub unsafe fn _M_allocate(&mut self, mut __n: usize) -> *mut i32 {
        unimplemented!()
    }
}
impl _Vector_base_472 {
    pub unsafe fn _M_deallocate(&mut self, mut __p: *mut i32, mut __n: usize) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _Vector_base_473 {
    pub _M_impl: std__Vector_base_unsigned_char__std_allocator_unsigned_char____Vector_impl,
}
impl Clone for _Vector_base_473 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_473 {
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
    pub unsafe fn new_1(__a: *const allocator_396) -> Self {
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
impl _Vector_base_473 {
    pub unsafe fn _M_get_Tp_allocator_1(&mut self) -> *mut allocator_396 {
        unimplemented!()
    }
}
impl _Vector_base_473 {
    pub unsafe fn new_1(__a: *const allocator_396) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_473 {
    pub unsafe fn move_from(_a0: *mut _Vector_base_473) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_473 {
    pub unsafe fn _M_allocate(&mut self, mut __n: usize) -> *mut u8 {
        unimplemented!()
    }
}
impl _Vector_base_473 {
    pub unsafe fn _M_deallocate(&mut self, mut __p: *mut u8, mut __n: usize) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct vector_474 {}
impl Clone for vector_474 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_474 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std_vector_int__TestAllocator_int____Guard_alloc {
    pub _M_storage: *mut i32,
    pub _M_len: u64,
    pub _M_vect: *mut _Vector_base_468,
}
impl Clone for std_vector_int__TestAllocator_int____Guard_alloc {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_vector_int__TestAllocator_int____Guard_alloc {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_vector_int__TestAllocator_int____Guard_alloc {
    pub unsafe fn new(mut __s: *mut i32, mut __l: u64, __vect: *mut _Vector_base_468) -> Self {
        unimplemented!()
    }
}
impl vector_474 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_474 {
    pub unsafe fn new_2(
        mut __n: u64,
        __value: *const i32,
        __a: Option<*const TestAllocator_403>,
    ) -> Self {
        unimplemented!()
    }
}
impl vector_474 {
    pub unsafe fn copy_from(__x: *const vector_474) -> Self {
        unimplemented!()
    }
}
impl vector_474 {
    pub unsafe fn move_from(_a0: *mut vector_474) -> Self {
        unimplemented!()
    }
}
impl vector_474 {
    pub unsafe fn copy_assign(&mut self, __x: *const vector_474) -> *mut vector_474 {
        unimplemented!()
    }
}
impl vector_474 {
    pub unsafe fn move_assign(&mut self, __x: *mut vector_474) -> *mut vector_474 {
        unimplemented!()
    }
}
impl vector_474 {
    pub unsafe fn begin_26(&mut self) -> __normal_iterator_382 {
        unimplemented!()
    }
}
impl vector_474 {
    pub unsafe fn begin_27(&self) -> __normal_iterator_383 {
        unimplemented!()
    }
}
impl vector_474 {
    pub unsafe fn end_28(&mut self) -> __normal_iterator_382 {
        unimplemented!()
    }
}
impl vector_474 {
    pub unsafe fn end_29(&self) -> __normal_iterator_383 {
        unimplemented!()
    }
}
impl vector_474 {
    pub unsafe fn cbegin(&self) -> __normal_iterator_383 {
        unimplemented!()
    }
}
impl vector_474 {
    pub unsafe fn cend(&self) -> __normal_iterator_383 {
        unimplemented!()
    }
}
impl vector_474 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl vector_474 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl vector_474 {
    pub unsafe fn resize_40(&mut self, mut __new_size: u64) {
        unimplemented!()
    }
}
impl vector_474 {
    pub unsafe fn capacity(&self) -> u64 {
        unimplemented!()
    }
}
impl vector_474 {
    pub unsafe fn empty(&self) -> bool {
        unimplemented!()
    }
}
impl vector_474 {
    pub unsafe fn reserve(&mut self, mut __n: u64) {
        unimplemented!()
    }
}
impl vector_474 {
    pub unsafe fn operator_index_46(&mut self, mut __n: u64) -> *mut i32 {
        unimplemented!()
    }
}
impl vector_474 {
    pub unsafe fn at_49(&mut self, mut __n: u64) -> *mut i32 {
        unimplemented!()
    }
}
impl vector_474 {
    pub unsafe fn front_51(&mut self) -> *mut i32 {
        unimplemented!()
    }
}
impl vector_474 {
    pub unsafe fn back_53(&mut self) -> *mut i32 {
        unimplemented!()
    }
}
impl vector_474 {
    pub unsafe fn data_55(&mut self) -> *mut i32 {
        unimplemented!()
    }
}
impl vector_474 {
    pub unsafe fn push_back_58(&mut self, __x: *mut i32) {
        unimplemented!()
    }
}
impl vector_474 {
    pub unsafe fn pop_back(&mut self) {
        unimplemented!()
    }
}
impl vector_474 {
    pub unsafe fn insert_63(
        &mut self,
        mut __position: __normal_iterator_383,
        __x: *mut i32,
    ) -> __normal_iterator_382 {
        unimplemented!()
    }
}
impl vector_474 {
    pub unsafe fn erase_67(
        &mut self,
        mut __position: __normal_iterator_383,
    ) -> __normal_iterator_382 {
        unimplemented!()
    }
}
impl vector_474 {
    pub unsafe fn emplace_back_475(&mut self, __args: *mut i32) -> *mut i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct vector_476 {}
impl Clone for vector_476 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_476 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl vector_476 {
    pub unsafe fn new_1(
        mut __n: u64,
        __value: *const f64,
        __a: Option<*const TestAllocator_414>,
    ) -> Self {
        unimplemented!()
    }
}
impl vector_476 {
    pub unsafe fn copy_from(__x: *const vector_476) -> Self {
        unimplemented!()
    }
}
impl vector_476 {
    pub unsafe fn move_from(_a0: *mut vector_476) -> Self {
        unimplemented!()
    }
}
impl vector_476 {
    pub unsafe fn copy_assign(&mut self, __x: *const vector_476) -> *mut vector_476 {
        unimplemented!()
    }
}
impl vector_476 {
    pub unsafe fn move_assign(&mut self, __x: *mut vector_476) -> *mut vector_476 {
        unimplemented!()
    }
}
impl vector_476 {
    pub unsafe fn begin_27(&self) -> __normal_iterator_385 {
        unimplemented!()
    }
}
impl vector_476 {
    pub unsafe fn end_28(&mut self) -> __normal_iterator_384 {
        unimplemented!()
    }
}
impl vector_476 {
    pub unsafe fn end_29(&self) -> __normal_iterator_385 {
        unimplemented!()
    }
}
impl vector_476 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl vector_476 {
    pub unsafe fn empty(&self) -> bool {
        unimplemented!()
    }
}
impl vector_476 {
    pub unsafe fn operator_index_46(&mut self, mut __n: u64) -> *mut f64 {
        unimplemented!()
    }
}
impl vector_476 {
    pub unsafe fn at_49(&mut self, mut __n: u64) -> *mut f64 {
        unimplemented!()
    }
}
impl vector_476 {
    pub unsafe fn back_53(&mut self) -> *mut f64 {
        unimplemented!()
    }
}
impl vector_476 {
    pub unsafe fn data_55(&mut self) -> *mut f64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct vector_477 {}
impl Clone for vector_477 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_477 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl vector_477 {
    pub unsafe fn copy_from(__x: *const vector_477) -> Self {
        unimplemented!()
    }
}
impl vector_477 {
    pub unsafe fn move_from(_a0: *mut vector_477) -> Self {
        unimplemented!()
    }
}
impl vector_477 {
    pub unsafe fn copy_assign(&mut self, __x: *const vector_477) -> *mut vector_477 {
        unimplemented!()
    }
}
impl vector_477 {
    pub unsafe fn move_assign(&mut self, __x: *mut vector_477) -> *mut vector_477 {
        unimplemented!()
    }
}
impl vector_477 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl vector_477 {
    pub unsafe fn operator_index_46(&mut self, mut __n: u64) -> *mut u32 {
        unimplemented!()
    }
}
impl vector_477 {
    pub unsafe fn new_1(
        mut __first: *mut u32,
        mut __last: *mut u32,
        __a: Option<*const allocator_393>,
    ) -> Self {
        unimplemented!()
    }
}
impl vector_477 {
    pub unsafe fn new_2(
        mut __first: *const u32,
        mut __last: *const u32,
        __a: Option<*const allocator_393>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct vector_478 {}
impl Clone for vector_478 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_478 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl vector_478 {
    pub unsafe fn copy_from(__x: *const vector_478) -> Self {
        unimplemented!()
    }
}
impl vector_478 {
    pub unsafe fn move_from(_a0: *mut vector_478) -> Self {
        unimplemented!()
    }
}
impl vector_478 {
    pub unsafe fn copy_assign(&mut self, __x: *const vector_478) -> *mut vector_478 {
        unimplemented!()
    }
}
impl vector_478 {
    pub unsafe fn move_assign(&mut self, __x: *mut vector_478) -> *mut vector_478 {
        unimplemented!()
    }
}
impl vector_478 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl vector_478 {
    pub unsafe fn operator_index_46(&mut self, mut __n: u64) -> *mut u64 {
        unimplemented!()
    }
}
impl vector_478 {
    pub unsafe fn new_1(
        mut __first: *mut u32,
        mut __last: *mut u32,
        __a: Option<*const allocator_394>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct vector_479 {}
impl Clone for vector_479 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_479 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl vector_479 {
    pub unsafe fn copy_from(__x: *const vector_479) -> Self {
        unimplemented!()
    }
}
impl vector_479 {
    pub unsafe fn move_from(_a0: *mut vector_479) -> Self {
        unimplemented!()
    }
}
impl vector_479 {
    pub unsafe fn copy_assign(&mut self, __x: *const vector_479) -> *mut vector_479 {
        unimplemented!()
    }
}
impl vector_479 {
    pub unsafe fn move_assign(&mut self, __x: *mut vector_479) -> *mut vector_479 {
        unimplemented!()
    }
}
impl vector_479 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl vector_479 {
    pub unsafe fn operator_index_46(&mut self, mut __n: u64) -> *mut i32 {
        unimplemented!()
    }
}
impl vector_479 {
    pub unsafe fn new_1(
        mut __first: *mut u32,
        mut __last: *mut u32,
        __a: Option<*const allocator_395>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct vector_480 {}
impl Clone for vector_480 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_480 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl vector_480 {
    pub unsafe fn copy_from(__x: *const vector_480) -> Self {
        unimplemented!()
    }
}
impl vector_480 {
    pub unsafe fn move_from(_a0: *mut vector_480) -> Self {
        unimplemented!()
    }
}
impl vector_480 {
    pub unsafe fn copy_assign(&mut self, __x: *const vector_480) -> *mut vector_480 {
        unimplemented!()
    }
}
impl vector_480 {
    pub unsafe fn move_assign(&mut self, __x: *mut vector_480) -> *mut vector_480 {
        unimplemented!()
    }
}
impl vector_480 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl vector_480 {
    pub unsafe fn operator_index_46(&mut self, mut __n: u64) -> *mut u8 {
        unimplemented!()
    }
}
impl vector_480 {
    pub unsafe fn new_1(
        mut __first: *const u8,
        mut __last: *const u8,
        __a: Option<*const allocator_396>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct hash_502 {}
impl Clone for hash_502 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_502 {
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
    pub unsafe fn hash_503(__val: *const f32) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub unsafe fn hash_504(__val: *const f64) -> usize {
        unimplemented!()
    }
}
pub type anon_505 = u32;
pub const anon_505__S_word_bit: anon_505 = 64;
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
pub struct binary_function_506 {}
impl Clone for binary_function_506 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_506 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct greater_507 {}
impl Clone for greater_507 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_507 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct less_508 {}
impl Clone for less_508 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_508 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct greater_equal_509 {}
impl Clone for greater_equal_509 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_509 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct less_equal_510 {}
impl Clone for less_equal_510 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_510 {
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
pub static mut allocator_arg_511: std::cell::LazyCell<std_allocator_arg_t> =
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestAllocator_403 {}
impl TestAllocator_403 {
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut i32 {
        return Box::leak((0..n).map(|_| 0_i32).collect::<Box<[i32]>>()).as_mut_ptr();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut i32, mut _a1: usize) {
        {
            let __p = p;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<i32>(),
                )))
            }
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestAllocator_414 {}
impl TestAllocator_414 {
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut f64 {
        return Box::leak((0..n).map(|_| 0_f64).collect::<Box<[f64]>>()).as_mut_ptr();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut f64, mut _a1: usize) {
        {
            let __p = p;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<f64>(),
                )))
            }
        };
    }
}
pub unsafe fn copy_512(mut copy_vector: vector_474) {}
pub unsafe fn fn_513(v: *mut vector_474, mut v3: vector_474) {
    (unsafe {
        let mut ___x: i32 = 20;
        vector_474::push_back_58(&mut (*v), &mut ___x)
    });
    let mut x: i32 = 0_i32;
    let mut v4: *mut vector_474 = (&mut v3 as *mut vector_474);
    let mut v2: vector_474 = <vector_474>::default();
    (unsafe {
        let mut ___x: i32 = 0;
        vector_474::push_back_58(&mut v2, &mut ___x)
    });
    (unsafe {
        let mut ___x: i32 = 1;
        vector_474::push_back_58(&mut v2, &mut ___x)
    });
    (unsafe {
        let mut ___x: i32 = 3;
        vector_474::push_back_58(&mut v2, &mut ___x)
    });
    x = (*(unsafe { vector_474::operator_index_46(&mut (*v), 2_u64) }));
    (*(unsafe { vector_474::operator_index_46(&mut v2, 0_u64) })) = 1;
    (*(unsafe {
        vector_474::operator_index_46(
            &mut (*if true {
                (&mut v3 as *mut vector_474)
            } else {
                (v)
            }),
            0_u64,
        )
    })) = 7;
    (*(unsafe { vector_474::operator_index_46(&mut (*v4), 1_u64) })) = 13;
    (if ((x) == (6)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514 ( c"x == 6" .as_ptr() , concat!(file!(), "\0").as_ptr() as *const libc::c_char  , line!() as u32  , c"void fn(std::vector<int, TestAllocator<int>> &, std::vector<int, TestAllocator<int>>)" .as_ptr() , )
        });
    });
    (if ((*(unsafe { vector_474::front_51(&mut (*v)) })) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514 ( c"v.front() == 4" .as_ptr() , concat!(file!(), "\0").as_ptr() as *const libc::c_char  , line!() as u32  , c"void fn(std::vector<int, TestAllocator<int>> &, std::vector<int, TestAllocator<int>>)" .as_ptr() , )
        });
    });
    (if ((*(unsafe { vector_474::operator_index_46(&mut (*v), 1_u64) })) == (5)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514 ( c"v[1] == 5" .as_ptr() , concat!(file!(), "\0").as_ptr() as *const libc::c_char  , line!() as u32  , c"void fn(std::vector<int, TestAllocator<int>> &, std::vector<int, TestAllocator<int>>)" .as_ptr() , )
        });
    });
    (if ((*(unsafe { vector_474::operator_index_46(&mut (*v), 2_u64) })) == (6)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514 ( c"v[2] == 6" .as_ptr() , concat!(file!(), "\0").as_ptr() as *const libc::c_char  , line!() as u32  , c"void fn(std::vector<int, TestAllocator<int>> &, std::vector<int, TestAllocator<int>>)" .as_ptr() , )
        });
    });
    (if ((*(unsafe { vector_474::back_53(&mut (*v)) })) == (20)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514 ( c"v.back() == 20" .as_ptr() , concat!(file!(), "\0").as_ptr() as *const libc::c_char  , line!() as u32  , c"void fn(std::vector<int, TestAllocator<int>> &, std::vector<int, TestAllocator<int>>)" .as_ptr() , )
        });
    });
    (if ((*(unsafe { vector_474::operator_index_46(&mut v3, 0_u64) })) == (7)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514 ( c"v3[0] == 7" .as_ptr() , concat!(file!(), "\0").as_ptr() as *const libc::c_char  , line!() as u32  , c"void fn(std::vector<int, TestAllocator<int>> &, std::vector<int, TestAllocator<int>>)" .as_ptr() , )
        });
    });
    (if ((*(unsafe { vector_474::operator_index_46(&mut v3, 1_u64) })) == (13)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514 ( c"v3[1] == 13" .as_ptr() , concat!(file!(), "\0").as_ptr() as *const libc::c_char  , line!() as u32  , c"void fn(std::vector<int, TestAllocator<int>> &, std::vector<int, TestAllocator<int>>)" .as_ptr() , )
        });
    });
    (unsafe {
        let mut ___x: i32 = 20;
        vector_474::push_back_58(&mut (*v), &mut ___x)
    });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v1: vector_474 = <vector_474>::default();
    (if ((unsafe { vector_474::size(&v1) }) == (0_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v1.size() == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe { vector_474::empty(&v1) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v1.empty()".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe {
        let mut ___x: i32 = 1;
        vector_474::push_back_58(&mut v1, &mut ___x)
    });
    (if !(unsafe { vector_474::empty(&v1) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"!v1.empty()".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { vector_474::pop_back(&mut v1) });
    (if (unsafe { vector_474::empty(&v1) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v1.empty()".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut s1: u64 = (unsafe { vector_474::size(&v1) });
    (unsafe { vector_474::resize_40(&mut v1, 100_u64) });
    (if ((unsafe { vector_474::size(&v1) }) == (100_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v1.size() == 100".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*(unsafe { vector_474::operator_index_46(&mut v1, 99_u64) })) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v1[99] == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*(unsafe { vector_474::operator_index_46(&mut v1, 0_u64) })) = 40;
    (*(unsafe { vector_474::operator_index_46(&mut v1, 99_u64) })) = 50;
    (if ((*(unsafe { vector_474::operator_index_46(&mut v1, 0_u64) })) == (40)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v1[0] == 40".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*(unsafe { vector_474::operator_index_46(&mut v1, 99_u64) })) == (50)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v1[99] == 50".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut v2: vector_474 = <vector_474>::default();
    (if ((unsafe { vector_474::size(&v2) }) == (0_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v2.size() == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe {
        let mut ___x: i32 = 1;
        vector_474::push_back_58(&mut v2, &mut ___x)
    });
    (unsafe {
        let mut ___x: i32 = 2;
        vector_474::push_back_58(&mut v2, &mut ___x)
    });
    (unsafe {
        let mut ___x: i32 = 3;
        vector_474::push_back_58(&mut v2, &mut ___x)
    });
    (if ((unsafe { vector_474::size(&v2) }) == (3_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v2.size() == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe {
        let ___position: __normal_iterator_383 = {
            let mut __tmp_0: __normal_iterator_382 = (unsafe { vector_474::begin_26(&mut v2) });
            __normal_iterator_383::new_2({ &mut __tmp_0 })
        };
        vector_474::erase_67(&mut v2, ___position)
    });
    (if ((unsafe { vector_474::size(&v2) }) == (2_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v2.size() == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*(unsafe { vector_474::operator_index_46(&mut v2, 0_u64) })) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v2[0] == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*(unsafe { vector_474::operator_index_46(&mut v2, 1_u64) })) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v2[1] == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe {
        let ___position: __normal_iterator_383 = {
            let mut __tmp_1: __normal_iterator_382 = (unsafe { vector_474::begin_26(&mut v2) });
            __normal_iterator_383::new_2({ &mut __tmp_1 })
        };
        let mut ___x: i32 = 100;
        vector_474::insert_63(&mut v2, ___position, &mut ___x)
    });
    (unsafe { copy_512(vector_474::copy_from({ &v2 })) });
    (if ((unsafe { vector_474::size(&v2) }) == (3_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v2.size() == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*(unsafe { vector_474::operator_index_46(&mut v2, 0_u64) })) == (100)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v2[0] == 100".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*(unsafe { vector_474::operator_index_46(&mut v2, 1_u64) })) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v2[1] == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*(unsafe { vector_474::operator_index_46(&mut v2, 2_u64) })) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v2[2] == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut s2: u64 = (unsafe { vector_474::size(&v2) });
    let mut v3: vector_474 = {
        let mut __tmp_2: i32 = 1;
        vector_474::new_2({ 100_u64 }, { &mut __tmp_2 }, None)
    };
    (if ((unsafe { vector_474::size(&v3) }) == (100_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v3.size() == 100".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut i: i32 = 0;
    'loop_: while ((i) < (100)) {
        (if ((*(unsafe { vector_474::operator_index_46(&mut v3, (i as u64)) })) == (1)) {
            &(0);
        } else {
            (unsafe {
                __assert_fail_514(
                    c"v3[i] == 1".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
        i.prefix_inc();
    }
    let mut v6: vector_476 = {
        let mut __tmp_3: f64 = 2.0E+0;
        vector_476::new_1({ s2 }, { &mut __tmp_3 }, None)
    };
    (if ((unsafe { vector_476::size(&v6) }) == (s2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v6.size() == s2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut i: u32 = 0_u32;
    'loop_: while ((i as u64) < (s2)) {
        (if ((*(unsafe { vector_476::operator_index_46(&mut v6, (i as u64)) })) == (2.0E+0)) {
            &(0);
        } else {
            (unsafe {
                __assert_fail_514(
                    c"v6[i] == 2.0".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
        i.prefix_inc();
    }
    let mut p1: *const f64 = (unsafe { vector_476::data_55(&mut v6) }).cast_const();
    (if ((*p1) == (2.0E+0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"*p1 == 2.0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut p2: *mut i32 = (unsafe { vector_474::data_55(&mut v3) });
    (if ((*p2) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"*p2 == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*(unsafe { vector_474::operator_index_46(&mut v3, 0_u64) })) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v3[0] == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*(unsafe { vector_474::operator_index_46(&mut v3, 1_u64) })) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v3[1] == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*p2) = (9.9E+1 as i32);
    (if ((*p2) == (99)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"*p2 == 99".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*(unsafe { vector_474::operator_index_46(&mut v3, 0_u64) })) == (99)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v3[0] == 99".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*(unsafe { vector_474::operator_index_46(&mut v3, 1_u64) })) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v3[1] == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    p2.prefix_inc();
    (*p2) = 98;
    (if ((*(unsafe { vector_474::operator_index_46(&mut v3, 0_u64) })) == (99)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v3[0] == 99".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*(unsafe { vector_474::operator_index_46(&mut v3, 1_u64) })) == (98)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v3[1] == 98".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { vector_474::capacity(&v3) }) == (100_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v3.capacity() == 100".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { vector_474::size(&v3) }) == (100_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v3.size() == 100".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { vector_474::reserve(&mut v3, 200_u64) });
    (if ((unsafe { vector_474::capacity(&v3) }) == (200_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v3.capacity() == 200".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { vector_474::size(&v3) }) == (100_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v3.size() == 100".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { vector_474::reserve(&mut v3, 50_u64) });
    (if ((unsafe { vector_474::capacity(&v3) }) == (200_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v3.capacity() == 200".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { vector_474::size(&v3) }) == (100_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v3.size() == 100".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { vector_474::reserve(&mut v3, 200_u64) });
    (if ((unsafe { vector_474::capacity(&v3) }) == (200_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v3.capacity() == 200".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { vector_474::size(&v3) }) == (100_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v3.size() == 100".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { vector_474::reserve(&mut v3, 201_u64) });
    (if ((unsafe { vector_474::capacity(&v3) }) == (201_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v3.capacity() == 201".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { vector_474::size(&v3) }) == (100_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v3.size() == 100".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*(unsafe { vector_474::back_53(&mut v2) })) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v2.back() == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*(unsafe { vector_474::back_53(&mut v3) })) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v3.back() == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*(unsafe { vector_476::back_53(&mut v6) })) == (2.0E+0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v6.back() == 2.0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let ref0: *mut f64 = (unsafe { vector_476::back_53(&mut v6) });
    (*ref0) = 5.0E+0;
    (if ((*(unsafe { vector_476::back_53(&mut v6) })) == (5.0E+0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v6.back() == 5.0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut x0: f64 = (*(unsafe { vector_476::back_53(&mut v6) }));
    (if ((x0) == (5.0E+0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"x0 == 5.0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    x0 = 6.0E+0;
    (if ((*(unsafe { vector_476::back_53(&mut v6) })) == (5.0E+0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v6.back() == 5.0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut idx: i32 = 0;
    (if ((*(unsafe { vector_476::at_49(&mut v6, (idx as u64)) })) == (2.0E+0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v6.at(idx) == 2.0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*(unsafe { vector_476::at_49(&mut v6, (s2).wrapping_sub(1_u64)) })) == (5.0E+0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v6.at(s2 - 1) == 5.0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let ref1: *mut f64 = (unsafe { vector_476::at_49(&mut v6, (s2).wrapping_sub(1_u64)) });
    (*ref1) += 1.5E+0;
    (if ((*(unsafe { vector_476::at_49(&mut v6, (s2).wrapping_sub(1_u64)) })) == (6.5E+0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v6.at(s2 - 1) == 6.5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut x1: f64 = (*(unsafe { vector_476::at_49(&mut v6, (s2).wrapping_sub(1_u64)) }));
    (if ((x1) == (6.5E+0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"x1 == 6.5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    x1 -= 1.5E+0;
    (if ((*(unsafe { vector_476::at_49(&mut v6, (s2).wrapping_sub(1_u64)) })) == (6.5E+0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v6.at(s2 - 1) == 6.5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut v7: vector_474 = <vector_474>::default();
    let mut v8: vector_474 = <vector_474>::default();
    (unsafe {
        let mut ___x: i32 = 4;
        vector_474::push_back_58(&mut v7, &mut ___x)
    });
    (unsafe {
        let mut ___x: i32 = 5;
        vector_474::push_back_58(&mut v7, &mut ___x)
    });
    (unsafe {
        let mut ___x: i32 = 6;
        vector_474::push_back_58(&mut v7, &mut ___x)
    });
    (unsafe {
        let mut ___x: i32 = 8;
        vector_474::push_back_58(&mut v8, &mut ___x)
    });
    (unsafe {
        let mut ___x: i32 = 9;
        vector_474::push_back_58(&mut v8, &mut ___x)
    });
    (unsafe { fn_513(&mut v7, vector_474::copy_from({ &v8 })) });
    let mut src: [u32; 3] = [1_u32, 2_u32, 3_u32];
    let mut v9: vector_477 = vector_477::new_1(
        { src.as_mut_ptr() },
        { src.as_mut_ptr().offset((3) as isize) },
        None,
    );
    (if ((unsafe { vector_477::size(&v9) }) == (3_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v9.size() == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { vector_477::operator_index_46(&mut v9, 0_u64) })) == (1_u32))
        && ((*(unsafe { vector_477::operator_index_46(&mut v9, 1_u64) })) == (2_u32)))
        && ((*(unsafe { vector_477::operator_index_46(&mut v9, 2_u64) })) == (3_u32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v9[0] == 1 && v9[1] == 2 && v9[2] == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut v10: vector_478 = vector_478::new_1(
        { src.as_mut_ptr() },
        { src.as_mut_ptr().offset((3) as isize) },
        None,
    );
    (if ((unsafe { vector_478::size(&v10) }) == (3_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v10.size() == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { vector_478::operator_index_46(&mut v10, 0_u64) })) == (1_u64))
        && ((*(unsafe { vector_478::operator_index_46(&mut v10, 1_u64) })) == (2_u64)))
        && ((*(unsafe { vector_478::operator_index_46(&mut v10, 2_u64) })) == (3_u64))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v10[0] == 1 && v10[1] == 2 && v10[2] == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut v11: vector_479 = vector_479::new_1(
        { src.as_mut_ptr() },
        { src.as_mut_ptr().offset((3) as isize) },
        None,
    );
    (if ((unsafe { vector_479::size(&v11) }) == (3_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v11.size() == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { vector_479::operator_index_46(&mut v11, 0_u64) })) == (1))
        && ((*(unsafe { vector_479::operator_index_46(&mut v11, 1_u64) })) == (2)))
        && ((*(unsafe { vector_479::operator_index_46(&mut v11, 2_u64) })) == (3))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v11[0] == 1 && v11[1] == 2 && v11[2] == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let src1: [u32; 3] = [1_u32, 2_u32, 3_u32];
    let mut v12: vector_477 =
        vector_477::new_2({ src1.as_ptr() }, { (unsafe { end_515(&src1) }) }, None);
    (if ((unsafe { vector_477::size(&v12) }) == (3_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v12.size() == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { vector_477::operator_index_46(&mut v12, 0_u64) })) == (1_u32))
        && ((*(unsafe { vector_477::operator_index_46(&mut v12, 1_u64) })) == (2_u32)))
        && ((*(unsafe { vector_477::operator_index_46(&mut v12, 2_u64) })) == (3_u32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v12[0] == 1 && v12[1] == 2 && v12[2] == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut buf: [u8; 5] = [10_u8, 20_u8, 30_u8, 40_u8, 50_u8];
    let mut start: *const u8 = (buf.as_mut_ptr()).cast_const();
    let mut len: usize = 5_usize;
    let mut v13: vector_480 = vector_480::new_1({ start }, { start.offset((len) as isize) }, None);
    (if ((unsafe { vector_480::size(&v13) }) == (5_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v13.size() == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { vector_480::operator_index_46(&mut v13, 0_u64) })) as i32) == (10))
        && (((*(unsafe { vector_480::operator_index_46(&mut v13, 4_u64) })) as i32) == (50))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"v13[0] == 10 && v13[4] == 50".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((((s1).wrapping_add(s2))
        .wrapping_add(((*(unsafe { vector_474::at_49(&mut v2, 0_u64) })) as u64)))
        == (103_u64))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_514(
                c"s1 + s2 + v2.at(0) == 103".as_ptr(),
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
pub unsafe fn __assert_fail_514(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn end_515(__arr: *const [u32; 3]) -> *const u32 {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
