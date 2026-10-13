extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_0 {}
impl Clone for allocator_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_0 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_0 {}
impl allocator_0 {}
impl allocator_0 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_1 {}
impl Clone for allocator_1 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_1 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_1 {
    pub unsafe fn copy_from(__a: *const allocator_1) -> Self {
        unimplemented!()
    }
}
impl allocator_1 {
    pub unsafe fn copy_assign(&mut self, _a0: *const allocator_1) -> *mut allocator_1 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_2 {}
impl Clone for allocator_2 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_2 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_2 {
    pub unsafe fn copy_from(__a: *const allocator_2) -> Self {
        unimplemented!()
    }
}
impl allocator_2 {
    pub unsafe fn copy_assign(&mut self, _a0: *const allocator_2) -> *mut allocator_2 {
        unimplemented!()
    }
}
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
pub static mut nothrow_3: std::cell::LazyCell<std_nothrow_t> =
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
pub static mut std_integral_constant_bool__true__value_5: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl integral_constant_4 {
    pub unsafe fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut std_integral_constant_bool__false__value_7: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct integral_constant_8 {}
impl Clone for integral_constant_8 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_8 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __conditional_14 {}
impl Clone for __conditional_14 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_14 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __conditional_15 {}
impl Clone for __conditional_15 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_15 {
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
pub struct __and__36 {}
impl Clone for __and__36 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__36 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__37 {}
impl Clone for __and__37 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__37 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__38 {}
impl Clone for __and__38 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__38 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__39 {}
impl Clone for __and__39 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__39 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__40 {}
impl Clone for __and__40 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__40 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__41 {}
impl Clone for __and__41 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__41 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__54 {}
impl Clone for __not__54 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__54 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub struct remove_cv_75 {}
impl Clone for remove_cv_75 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_75 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_76 {}
impl Clone for remove_cv_76 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_76 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_77 {}
impl Clone for remove_cv_77 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_77 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_78 {}
impl Clone for remove_cv_78 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_78 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_79 {}
impl Clone for remove_cv_79 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_79 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_80 {}
impl Clone for remove_cv_80 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_80 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_81 {}
impl Clone for remove_cv_81 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_81 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_82 {}
impl Clone for is_const_82 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_82 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_83 {}
impl Clone for is_const_83 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_83 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_84 {}
impl Clone for is_const_84 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_84 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_85 {}
impl Clone for is_const_85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_85 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_86 {}
impl Clone for is_const_86 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_86 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_87 {}
impl Clone for __is_array_unknown_bounds_87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_88 {}
impl Clone for __is_array_unknown_bounds_88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_88 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_89 {}
impl Clone for __is_array_unknown_bounds_89 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_89 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_90 {}
impl Clone for __is_array_unknown_bounds_90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_90 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_91 {}
impl Clone for __is_integral_helper_91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_91 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_92 {}
impl Clone for __is_integral_helper_92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_93 {}
impl Clone for __is_integral_helper_93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_93 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_94 {}
impl Clone for __is_integral_helper_94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_94 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_95 {}
impl Clone for __is_integral_helper_95 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_95 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_96 {}
impl Clone for is_integral_96 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_96 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_97 {}
impl Clone for is_integral_97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_97 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_98 {}
impl Clone for is_integral_98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_98 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_99 {}
impl Clone for is_integral_99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_99 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_100 {}
impl Clone for is_integral_100 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_100 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_pointer_101 {}
impl Clone for is_pointer_101 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_101 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_pointer_102 {}
impl Clone for is_pointer_102 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_102 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_rvalue_reference_103 {}
impl Clone for is_rvalue_reference_103 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_rvalue_reference_103 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_rvalue_reference_104 {}
impl Clone for is_rvalue_reference_104 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_rvalue_reference_104 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_105 {}
impl Clone for is_same_105 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_105 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_106 {}
impl Clone for is_same_106 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_106 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_107 {}
impl Clone for is_volatile_107 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_107 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_108 {}
impl Clone for is_volatile_108 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_108 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_109 {}
impl Clone for is_volatile_109 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_109 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_110 {}
impl Clone for is_volatile_110 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_110 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_111 {}
impl Clone for is_volatile_111 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_111 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_empty_112 {}
impl Clone for is_empty_112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_empty_112 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_empty_113 {}
impl Clone for is_empty_113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_empty_113 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_empty_114 {}
impl Clone for is_empty_114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_empty_114 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_empty_115 {}
impl Clone for is_empty_115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_empty_115 {
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
pub struct is_default_constructible_116 {}
impl Clone for is_default_constructible_116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_default_constructible_116 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_default_constructible_117 {}
impl Clone for is_default_constructible_117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_default_constructible_117 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_default_constructible_118 {}
impl Clone for is_default_constructible_118 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_default_constructible_118 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_default_constructible_119 {}
impl Clone for is_default_constructible_119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_default_constructible_119 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_constructible_120 {}
impl Clone for is_move_constructible_120 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_120 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_constructible_121 {}
impl Clone for is_move_constructible_121 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_121 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_default_constructible_122 {}
impl Clone for is_nothrow_default_constructible_122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_122 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_default_constructible_123 {}
impl Clone for is_nothrow_default_constructible_123 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_123 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_default_constructible_124 {}
impl Clone for is_nothrow_default_constructible_124 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_124 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_default_constructible_125 {}
impl Clone for is_nothrow_default_constructible_125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_125 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_assignable_126 {}
impl Clone for is_assignable_126 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_assignable_126 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_assignable_127 {}
impl Clone for is_assignable_127 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_assignable_127 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_assignable_128 {}
impl Clone for is_assignable_128 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_assignable_128 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_assignable_129 {}
impl Clone for is_assignable_129 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_assignable_129 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_assignable_130 {}
impl Clone for is_assignable_130 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_assignable_130 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_assignable_131 {}
impl Clone for is_assignable_131 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_assignable_131 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_assignable_132 {}
impl Clone for is_assignable_132 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_assignable_132 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_assignable_133 {}
impl Clone for is_assignable_133 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_assignable_133 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_assignable_134 {}
impl Clone for is_move_assignable_134 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_134 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_assignable_135 {}
impl Clone for is_move_assignable_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_135 {
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
pub struct __is_implicitly_default_constructible_impl_136 {}
impl Clone for __is_implicitly_default_constructible_impl_136 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_impl_136 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_implicitly_default_constructible_impl_137 {}
impl Clone for __is_implicitly_default_constructible_impl_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_impl_137 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_implicitly_default_constructible_impl_138 {}
impl Clone for __is_implicitly_default_constructible_impl_138 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_impl_138 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_implicitly_default_constructible_impl_139 {}
impl Clone for __is_implicitly_default_constructible_impl_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_impl_139 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_implicitly_default_constructible_safe_140 {}
impl Clone for __is_implicitly_default_constructible_safe_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_safe_140 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_implicitly_default_constructible_safe_141 {}
impl Clone for __is_implicitly_default_constructible_safe_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_safe_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_implicitly_default_constructible_safe_142 {}
impl Clone for __is_implicitly_default_constructible_safe_142 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_safe_142 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_implicitly_default_constructible_safe_143 {}
impl Clone for __is_implicitly_default_constructible_safe_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_safe_143 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_implicitly_default_constructible_144 {}
impl Clone for __is_implicitly_default_constructible_144 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_144 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_implicitly_default_constructible_145 {}
impl Clone for __is_implicitly_default_constructible_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_145 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_implicitly_default_constructible_146 {}
impl Clone for __is_implicitly_default_constructible_146 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_146 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_implicitly_default_constructible_147 {}
impl Clone for __is_implicitly_default_constructible_147 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_147 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_148 {}
impl Clone for is_convertible_148 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_148 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_149 {}
impl Clone for is_convertible_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_150 {}
impl Clone for remove_reference_150 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_150 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_151 {}
impl Clone for remove_reference_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct add_lvalue_reference_152 {}
impl Clone for add_lvalue_reference_152 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for add_lvalue_reference_152 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct add_lvalue_reference_153 {}
impl Clone for add_lvalue_reference_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for add_lvalue_reference_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_154 {}
impl Clone for __cv_selector_154 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_154 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_155 {}
impl Clone for __cv_selector_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_156 {}
impl Clone for __cv_selector_156 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_156 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_157 {}
impl Clone for __cv_selector_157 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_157 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_158 {}
impl Clone for __match_cv_qualifiers_158 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_158 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_159 {}
impl Clone for __match_cv_qualifiers_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_159 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_160 {}
impl Clone for __match_cv_qualifiers_160 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_160 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_161 {}
impl Clone for __match_cv_qualifiers_161 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_161 {
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
pub struct __make_unsigned_163 {}
impl Clone for __make_unsigned_163 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_163 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_164 {}
impl Clone for __make_unsigned_164 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_164 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_165 {}
impl Clone for __make_unsigned_165 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_165 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_166 {}
impl Clone for __make_unsigned_selector_166 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_166 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_167 {}
impl Clone for __make_unsigned_selector_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_167 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_168 {}
impl Clone for __make_unsigned_selector_168 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_168 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_169 {}
impl Clone for __make_unsigned_selector_169 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_169 {
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
pub struct _List_175 {}
impl Clone for _List_175 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_175 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_176 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_177 {}
impl Clone for _List_177 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_177 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_178 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
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
pub static mut std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_180 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_182 {}
impl Clone for _List_182 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_182 {
    fn default() -> Self {
        unimplemented!()
    }
}
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_184 {}
impl Clone for __select_184 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_184 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_185 {}
impl Clone for __select_185 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_185 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_186 {}
impl Clone for __select_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_187 {}
impl Clone for __select_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_187 {
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
pub struct __make_signed_189 {}
impl Clone for __make_signed_189 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_189 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_190 {}
impl Clone for __make_signed_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_190 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_191 {}
impl Clone for __make_signed_selector_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_191 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_192 {}
impl Clone for __make_signed_selector_192 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_192 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_193 {}
impl Clone for __make_signed_selector_193 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_193 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_194 {}
impl Clone for __make_signed_selector_194 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_194 {
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
pub struct remove_extent_196 {}
impl Clone for remove_extent_196 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_extent_196 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_extent_197 {}
impl Clone for remove_extent_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_extent_197 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_pointer_198 {}
impl Clone for remove_pointer_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_pointer_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_pointer_199 {}
impl Clone for remove_pointer_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_pointer_199 {
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
pub struct common_type_200 {}
impl Clone for common_type_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_200 {
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
pub struct __new_allocator_201 {}
impl Clone for __new_allocator_201 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_201 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_201 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_201) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_201 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_201,
    ) -> *mut __new_allocator_201 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_202 {}
impl Clone for __new_allocator_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_202 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_202 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_202) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_202 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_202,
    ) -> *mut __new_allocator_202 {
        unimplemented!()
    }
}
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_203 {}
impl Clone for iterator_203 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_203 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct tuple_element_204 {}
impl Clone for tuple_element_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_element_204 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct tuple_element_205 {}
impl Clone for tuple_element_205 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_element_205 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct tuple_element_206 {}
impl Clone for tuple_element_206 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_element_206 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct tuple_element_207 {}
impl Clone for tuple_element_207 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_element_207 {
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
pub struct _Nth_type_208 {}
impl Clone for _Nth_type_208 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Nth_type_208 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Nth_type_209 {}
impl Clone for _Nth_type_209 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Nth_type_209 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Nth_type_210 {}
impl Clone for _Nth_type_210 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Nth_type_210 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Nth_type_211 {}
impl Clone for _Nth_type_211 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Nth_type_211 {
    fn default() -> Self {
        unimplemented!()
    }
}
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct tuple_212 {}
impl Clone for tuple_212 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_212 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct tuple_213 {}
impl Clone for tuple_213 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_213 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_213 {
    pub unsafe fn copy_from(_a0: *const tuple_213) -> Self {
        unimplemented!()
    }
}
impl tuple_213 {
    pub unsafe fn move_from(_a0: *mut tuple_213) -> Self {
        unimplemented!()
    }
}
impl tuple_213 {
    pub unsafe fn copy_assign(&mut self, __in: *const tuple_213) -> *mut tuple_213 {
        unimplemented!()
    }
}
impl tuple_213 {
    pub unsafe fn move_assign(&mut self, __in: *mut tuple_213) -> *mut tuple_213 {
        unimplemented!()
    }
}
impl tuple_213 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct tuple_214 {}
impl Clone for tuple_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_214 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_214 {
    pub unsafe fn copy_from(_a0: *const tuple_214) -> Self {
        unimplemented!()
    }
}
impl tuple_214 {
    pub unsafe fn move_from(_a0: *mut tuple_214) -> Self {
        unimplemented!()
    }
}
impl tuple_214 {
    pub unsafe fn copy_assign(&mut self, __in: *const tuple_214) -> *mut tuple_214 {
        unimplemented!()
    }
}
impl tuple_214 {
    pub unsafe fn move_assign(&mut self, __in: *mut tuple_214) -> *mut tuple_214 {
        unimplemented!()
    }
}
impl tuple_214 {
    pub unsafe fn new() -> Self {
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
pub struct __are_same_215 {}
impl Clone for __are_same_215 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_215 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_216 = u32;
pub const anon_216___value: anon_216 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_217 {}
impl Clone for __are_same_217 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_217 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_218 = u32;
pub const anon_218___value: anon_218 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_219 {}
impl Clone for __are_same_219 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_219 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_220 = u32;
pub const anon_220___value: anon_220 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_221 {}
impl Clone for __are_same_221 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_221 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_222 = u32;
pub const anon_222___value: anon_222 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_223 {}
impl Clone for __are_same_223 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_223 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_224 = u32;
pub const anon_224___value: anon_224 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_225 {}
impl Clone for __are_same_225 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_225 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_226 = u32;
pub const anon_226___value: anon_226 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_227 {}
impl Clone for __is_integer_227 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_227 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_228 = u32;
pub const anon_228___value: anon_228 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_229 {}
impl Clone for __is_integer_229 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_229 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_230 = u32;
pub const anon_230___value: anon_230 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_231 {}
impl Clone for __is_integer_231 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_231 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_232 = u32;
pub const anon_232___value: anon_232 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_233 {}
impl Clone for __is_integer_233 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_233 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_234 = u32;
pub const anon_234___value: anon_234 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_integer_235 {}
impl Clone for __memcpyable_integer_235 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_235 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_236 {}
impl Clone for __is_integer_nonstrict_236 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_236 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_237 = u32;
pub const anon_237___width: anon_237 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_238 {}
impl Clone for __numeric_traits_integer_238 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_238 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_long____is_signed_239: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_long____digits_240: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_long____max_241: std::cell::LazyCell<i64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_242 {}
impl Clone for __numeric_traits_floating_242 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_242 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_243 {}
impl Clone for __numeric_traits_floating_243 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_243 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_244 {}
impl Clone for __numeric_traits_floating_244 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_244 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_245 {}
impl Clone for __numeric_traits_245 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_245 {
    fn default() -> Self {
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
pub static mut allocator_arg_247: std::cell::LazyCell<std_allocator_arg_t> =
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_empty_non_tuple_248 {}
impl Clone for __is_empty_non_tuple_248 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_empty_non_tuple_248 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_empty_non_tuple_249 {}
impl Clone for __is_empty_non_tuple_249 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_empty_non_tuple_249 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_empty_non_tuple_250 {}
impl Clone for __is_empty_non_tuple_250 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_empty_non_tuple_250 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_empty_non_tuple_251 {}
impl Clone for __is_empty_non_tuple_251 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_empty_non_tuple_251 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _Head_base_252 {
    pub _M_head_impl: default_delete_253,
}
impl Clone for _Head_base_252 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Head_base_252 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Head_base_252 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Head_base_252 {
    pub unsafe fn copy_from(_a0: *const _Head_base_252) -> Self {
        unimplemented!()
    }
}
impl _Head_base_252 {
    pub unsafe fn move_from(_a0: *mut _Head_base_252) -> Self {
        unimplemented!()
    }
}
impl _Head_base_252 {
    pub unsafe fn _M_head_12(__b: *mut _Head_base_252) -> *mut default_delete_253 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _Head_base_254 {
    pub _M_head_impl: *mut Edge,
}
impl Clone for _Head_base_254 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Head_base_254 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Head_base_254 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Head_base_254 {
    pub unsafe fn copy_from(_a0: *const _Head_base_254) -> Self {
        unimplemented!()
    }
}
impl _Head_base_254 {
    pub unsafe fn move_from(_a0: *mut _Head_base_254) -> Self {
        unimplemented!()
    }
}
impl _Head_base_254 {
    pub unsafe fn _M_head_12(__b: *mut _Head_base_254) -> *mut *mut Edge {
        unimplemented!()
    }
}
impl _Head_base_254 {
    pub unsafe fn _M_head_13(__b: *const _Head_base_254) -> *const *mut Edge {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _Head_base_255 {
    pub _M_head_impl: default_delete_256,
}
impl Clone for _Head_base_255 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Head_base_255 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Head_base_255 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Head_base_255 {
    pub unsafe fn copy_from(_a0: *const _Head_base_255) -> Self {
        unimplemented!()
    }
}
impl _Head_base_255 {
    pub unsafe fn move_from(_a0: *mut _Head_base_255) -> Self {
        unimplemented!()
    }
}
impl _Head_base_255 {
    pub unsafe fn _M_head_12(__b: *mut _Head_base_255) -> *mut default_delete_256 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _Head_base_257 {
    pub _M_head_impl: *mut i32,
}
impl Clone for _Head_base_257 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Head_base_257 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Head_base_257 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Head_base_257 {
    pub unsafe fn copy_from(_a0: *const _Head_base_257) -> Self {
        unimplemented!()
    }
}
impl _Head_base_257 {
    pub unsafe fn move_from(_a0: *mut _Head_base_257) -> Self {
        unimplemented!()
    }
}
impl _Head_base_257 {
    pub unsafe fn _M_head_12(__b: *mut _Head_base_257) -> *mut *mut i32 {
        unimplemented!()
    }
}
impl _Head_base_257 {
    pub unsafe fn _M_head_13(__b: *const _Head_base_257) -> *const *mut i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Tuple_impl_258 {}
impl Clone for _Tuple_impl_258 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Tuple_impl_258 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_258 {
    pub unsafe fn _M_head_1(__t: *mut _Tuple_impl_258) -> *mut *mut Edge {
        unimplemented!()
    }
}
impl _Tuple_impl_258 {
    pub unsafe fn _M_head_2(__t: *const _Tuple_impl_258) -> *const *mut Edge {
        unimplemented!()
    }
}
impl _Tuple_impl_258 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_258 {
    pub unsafe fn copy_from(_a0: *const _Tuple_impl_258) -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_258 {
    pub unsafe fn move_from(_a0: *mut _Tuple_impl_258) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Tuple_impl_259 {}
impl Clone for _Tuple_impl_259 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Tuple_impl_259 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_259 {
    pub unsafe fn _M_head_1(__t: *mut _Tuple_impl_259) -> *mut default_delete_253 {
        unimplemented!()
    }
}
impl _Tuple_impl_259 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_259 {
    pub unsafe fn copy_from(_a0: *const _Tuple_impl_259) -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_259 {
    pub unsafe fn move_from(__in: *mut _Tuple_impl_259) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Tuple_impl_260 {}
impl Clone for _Tuple_impl_260 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Tuple_impl_260 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_260 {
    pub unsafe fn _M_head_1(__t: *mut _Tuple_impl_260) -> *mut *mut i32 {
        unimplemented!()
    }
}
impl _Tuple_impl_260 {
    pub unsafe fn _M_head_2(__t: *const _Tuple_impl_260) -> *const *mut i32 {
        unimplemented!()
    }
}
impl _Tuple_impl_260 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_260 {
    pub unsafe fn copy_from(_a0: *const _Tuple_impl_260) -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_260 {
    pub unsafe fn move_from(_a0: *mut _Tuple_impl_260) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Tuple_impl_261 {}
impl Clone for _Tuple_impl_261 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Tuple_impl_261 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_261 {
    pub unsafe fn _M_head_1(__t: *mut _Tuple_impl_261) -> *mut default_delete_256 {
        unimplemented!()
    }
}
impl _Tuple_impl_261 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_261 {
    pub unsafe fn copy_from(_a0: *const _Tuple_impl_261) -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_261 {
    pub unsafe fn move_from(__in: *mut _Tuple_impl_261) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _TupleConstraints_262 {}
impl Clone for _TupleConstraints_262 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _TupleConstraints_262 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _TupleConstraints_262 {
    pub unsafe fn __is_implicitly_default_constructible() -> bool {
        unimplemented!()
    }
}
impl _TupleConstraints_262 {
    pub unsafe fn __is_explicitly_default_constructible() -> bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _TupleConstraints_263 {}
impl Clone for _TupleConstraints_263 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _TupleConstraints_263 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _TupleConstraints_263 {
    pub unsafe fn __is_implicitly_default_constructible() -> bool {
        unimplemented!()
    }
}
impl _TupleConstraints_263 {
    pub unsafe fn __is_explicitly_default_constructible() -> bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct binary_function_264 {}
impl Clone for binary_function_264 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_264 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct greater_265 {}
impl Clone for greater_265 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_265 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct less_266 {}
impl Clone for less_266 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_266 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct greater_equal_267 {}
impl Clone for greater_equal_267 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_267 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct less_equal_268 {}
impl Clone for less_equal_268 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_268 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_269 {}
impl Clone for __hash_base_269 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_269 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_270 {}
impl Clone for __hash_base_270 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_270 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_271 {}
impl Clone for __hash_base_271 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_271 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_272 {}
impl Clone for __hash_base_272 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_272 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_273 {}
impl Clone for __hash_base_273 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_273 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_274 {}
impl Clone for __hash_base_274 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_274 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_275 {}
impl Clone for __hash_base_275 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_275 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_276 {}
impl Clone for __hash_base_276 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_276 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_277 {}
impl Clone for __hash_base_277 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_277 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_278 {}
impl Clone for __hash_base_278 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_278 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_279 {}
impl Clone for __hash_base_279 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_279 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_280 {}
impl Clone for __hash_base_280 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_280 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_281 {}
impl Clone for __hash_base_281 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_281 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_282 {}
impl Clone for __hash_base_282 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_282 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_283 {}
impl Clone for __hash_base_283 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_283 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_284 {}
impl Clone for __hash_base_284 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_284 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_285 {}
impl Clone for __hash_base_285 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_285 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_286 {}
impl Clone for __hash_base_286 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_286 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_287 {}
impl Clone for __hash_base_287 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_287 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_288 {}
impl Clone for __hash_base_288 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_288 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_289 {}
impl Clone for __hash_base_289 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_289 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct hash_290 {}
impl Clone for hash_290 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_290 {
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
    pub unsafe fn hash_291(__val: *const f32) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub unsafe fn hash_292(__val: *const f64) -> usize {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct default_delete_253 {}
impl Clone for default_delete_253 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for default_delete_253 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl default_delete_253 {}
impl default_delete_253 {}
impl default_delete_253 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const default_delete_253,
    ) -> *mut default_delete_253 {
        unimplemented!()
    }
}
impl default_delete_253 {
    pub unsafe fn move_assign(&mut self, _a0: *mut default_delete_253) -> *mut default_delete_253 {
        unimplemented!()
    }
}
impl default_delete_253 {
    pub unsafe fn operator_call_293(&self, mut __ptr: *mut Edge) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct default_delete_256 {}
impl Clone for default_delete_256 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for default_delete_256 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl default_delete_256 {}
impl default_delete_256 {}
impl default_delete_256 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const default_delete_256,
    ) -> *mut default_delete_256 {
        unimplemented!()
    }
}
impl default_delete_256 {
    pub unsafe fn move_assign(&mut self, _a0: *mut default_delete_256) -> *mut default_delete_256 {
        unimplemented!()
    }
}
impl default_delete_256 {
    pub unsafe fn operator_call_294(&self, mut __ptr: *mut i32) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __uniq_ptr_impl_295 {}
impl Clone for __uniq_ptr_impl_295 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __uniq_ptr_impl_295 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Ptr_296 {}
impl Clone for _Ptr_296 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Ptr_296 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_295 {
    pub unsafe fn new_1(mut __p: *mut Edge) -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_295 {
    pub unsafe fn move_from(__u: *mut __uniq_ptr_impl_295) -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_295 {
    pub unsafe fn move_assign(
        &mut self,
        __u: *mut __uniq_ptr_impl_295,
    ) -> *mut __uniq_ptr_impl_295 {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_295 {
    pub unsafe fn _M_ptr_6(&mut self) -> *mut *mut Edge {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_295 {
    pub unsafe fn _M_ptr_7(&self) -> *mut Edge {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_295 {
    pub unsafe fn _M_deleter_8(&mut self) -> *mut default_delete_253 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __uniq_ptr_impl_297 {}
impl Clone for __uniq_ptr_impl_297 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __uniq_ptr_impl_297 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Ptr_298 {}
impl Clone for _Ptr_298 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Ptr_298 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_297 {
    pub unsafe fn new_1(mut __p: *mut i32) -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_297 {
    pub unsafe fn move_from(__u: *mut __uniq_ptr_impl_297) -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_297 {
    pub unsafe fn move_assign(
        &mut self,
        __u: *mut __uniq_ptr_impl_297,
    ) -> *mut __uniq_ptr_impl_297 {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_297 {
    pub unsafe fn _M_ptr_6(&mut self) -> *mut *mut i32 {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_297 {
    pub unsafe fn _M_ptr_7(&self) -> *mut i32 {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_297 {
    pub unsafe fn _M_deleter_8(&mut self) -> *mut default_delete_256 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __uniq_ptr_data_299 {}
impl Clone for __uniq_ptr_data_299 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __uniq_ptr_data_299 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_data_299 {
    pub unsafe fn move_from(_a0: *mut __uniq_ptr_data_299) -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_data_299 {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut __uniq_ptr_data_299,
    ) -> *mut __uniq_ptr_data_299 {
        unimplemented!()
    }
}
impl __uniq_ptr_data_299 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __uniq_ptr_data_300 {}
impl Clone for __uniq_ptr_data_300 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __uniq_ptr_data_300 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_data_300 {
    pub unsafe fn move_from(_a0: *mut __uniq_ptr_data_300) -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_data_300 {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut __uniq_ptr_data_300,
    ) -> *mut __uniq_ptr_data_300 {
        unimplemented!()
    }
}
impl __uniq_ptr_data_300 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct unique_ptr_301 {}
impl Clone for unique_ptr_301 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for unique_ptr_301 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl unique_ptr_301 {
    pub unsafe fn move_from(_a0: *mut unique_ptr_301) -> Self {
        unimplemented!()
    }
}
impl unique_ptr_301 {
    pub unsafe fn move_assign(&mut self, _a0: *mut unique_ptr_301) -> *mut unique_ptr_301 {
        unimplemented!()
    }
}
impl unique_ptr_301 {
    pub unsafe fn operator_index(&self, mut __i: usize) -> *mut Edge {
        unimplemented!()
    }
}
impl unique_ptr_301 {
    pub unsafe fn get(&self) -> *mut Edge {
        unimplemented!()
    }
}
impl unique_ptr_301 {
    pub unsafe fn get_deleter_15(&mut self) -> *mut default_delete_253 {
        unimplemented!()
    }
}
impl unique_ptr_301 {
    pub unsafe fn new(mut __p: *mut Edge) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct unique_ptr_302 {}
impl Clone for unique_ptr_302 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for unique_ptr_302 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl unique_ptr_302 {
    pub unsafe fn move_from(_a0: *mut unique_ptr_302) -> Self {
        unimplemented!()
    }
}
impl unique_ptr_302 {
    pub unsafe fn move_assign(&mut self, _a0: *mut unique_ptr_302) -> *mut unique_ptr_302 {
        unimplemented!()
    }
}
impl unique_ptr_302 {
    pub unsafe fn operator_index(&self, mut __i: usize) -> *mut i32 {
        unimplemented!()
    }
}
impl unique_ptr_302 {
    pub unsafe fn get(&self) -> *mut i32 {
        unimplemented!()
    }
}
impl unique_ptr_302 {
    pub unsafe fn get_deleter_15(&mut self) -> *mut default_delete_256 {
        unimplemented!()
    }
}
impl unique_ptr_302 {
    pub unsafe fn new(mut __p: *mut i32) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _MakeUniq_303 {}
impl Clone for _MakeUniq_303 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _MakeUniq_303 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _MakeUniq_304 {}
impl Clone for _MakeUniq_304 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _MakeUniq_304 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __mbstate_t {
    pub __count: i32,
    pub __value: anon_305,
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
#[derive(VaArg, FnPtrArg)]
pub struct anon_305 {
    pub __wch: u32,
    pub __wchb: [libc::c_char; 4],
}
impl Clone for anon_305 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_305 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13],
    pub __ctype_b: *const u16,
    pub __ctype_tolower: *const i32,
    pub __ctype_toupper: *const i32,
    pub __names: [*const libc::c_char; 13],
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
#[derive(VaArg, FnPtrArg, Copy)]
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
impl std_type_info {
    pub unsafe fn name(&self) -> *const libc::c_char {
        unimplemented!()
    }
}
impl std_type_info {
    pub unsafe fn operator_eq(&self, __arg: *const std_type_info) -> bool {
        unimplemented!()
    }
}
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
#[derive(VaArg, FnPtrArg)]
pub struct timespec {
    pub tv_sec: i64,
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
#[derive(VaArg, FnPtrArg)]
pub struct sched_param {
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
#[derive(VaArg, FnPtrArg)]
pub struct cpu_set_t {
    pub __bits: [u64; 16],
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
#[derive(VaArg, FnPtrArg)]
pub struct timeval {
    pub tv_sec: i64,
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
#[derive(VaArg, FnPtrArg)]
pub struct timex {
    pub modes: u32,
    pub offset: i64,
    pub freq: i64,
    pub maxerror: i64,
    pub esterror: i64,
    pub status: i32,
    pub constant: i64,
    pub precision: i64,
    pub tolerance: i64,
    pub time: timeval,
    pub tick: i64,
    pub ppsfreq: i64,
    pub jitter: i64,
    pub shift: i32,
    pub stabil: i64,
    pub jitcnt: i64,
    pub calcnt: i64,
    pub errcnt: i64,
    pub stbcnt: i64,
    pub tai: i32,
    pub anon_306: i32,
    pub anon_307: i32,
    pub anon_308: i32,
    pub anon_309: i32,
    pub anon_310: i32,
    pub anon_311: i32,
    pub anon_312: i32,
    pub anon_313: i32,
    pub anon_314: i32,
    pub anon_315: i32,
    pub anon_316: i32,
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
#[derive(VaArg, FnPtrArg)]
pub struct tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: i64,
    pub tm_zone: *const libc::c_char,
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
#[derive(VaArg, FnPtrArg)]
pub struct itimerspec {
    pub it_interval: timespec,
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
#[derive(VaArg, FnPtrArg)]
pub struct __atomic_wide_counter {
    pub __value64: u64,
    pub __value32: anon_317,
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
#[derive(VaArg, FnPtrArg)]
pub struct anon_317 {
    pub __low: u32,
    pub __high: u32,
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
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
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
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_internal_slist {
    pub __next: *mut __pthread_internal_slist,
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
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_mutex_s {
    pub __lock: i32,
    pub __count: u32,
    pub __owner: i32,
    pub __nusers: u32,
    pub __kind: i32,
    pub __spins: i16,
    pub __elision: i16,
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
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: u32,
    pub __writers: u32,
    pub __wrphase_futex: u32,
    pub __writers_futex: u32,
    pub __pad3: u32,
    pub __pad4: u32,
    pub __cur_writer: i32,
    pub __shared: i32,
    pub __rwelision: i8,
    pub __pad1: [u8; 7],
    pub __pad2: u64,
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
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [u32; 2],
    pub __g_size: [u32; 2],
    pub __g1_orig_size: u32,
    pub __wrefs: u32,
    pub __g_signals: [u32; 2],
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
#[derive(VaArg, FnPtrArg)]
pub struct __once_flag {
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_condattr_t {
    pub __size: [libc::c_char; 4],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_attr_t {
    pub __size: [libc::c_char; 56],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [libc::c_char; 56],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_rwlockattr_t {
    pub __size: [libc::c_char; 8],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_barrier_t {
    pub __size: [libc::c_char; 32],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_barrierattr_t {
    pub __size: [libc::c_char; 4],
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
#[derive(VaArg, FnPtrArg)]
pub struct __sigset_t {
    pub __val: [u64; 16],
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
#[derive(VaArg, FnPtrArg)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: [i64; 8],
    pub __mask_was_saved: i32,
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
pub type anon_318 = u32;
pub const anon_318_PTHREAD_MUTEX_TIMED_NP: anon_318 = 0;
pub const anon_318_PTHREAD_MUTEX_RECURSIVE_NP: anon_318 = 1;
pub const anon_318_PTHREAD_MUTEX_ERRORCHECK_NP: anon_318 = 2;
pub const anon_318_PTHREAD_MUTEX_ADAPTIVE_NP: anon_318 = 3;
pub const anon_318_PTHREAD_MUTEX_NORMAL: anon_318 = 0;
pub const anon_318_PTHREAD_MUTEX_RECURSIVE: anon_318 = 1;
pub const anon_318_PTHREAD_MUTEX_ERRORCHECK: anon_318 = 2;
pub const anon_318_PTHREAD_MUTEX_DEFAULT: anon_318 = 0;
pub const anon_318_PTHREAD_MUTEX_FAST_NP: anon_318 = 0;
pub type anon_319 = u32;
pub const anon_319_PTHREAD_MUTEX_STALLED: anon_319 = 0;
pub const anon_319_PTHREAD_MUTEX_STALLED_NP: anon_319 = 0;
pub const anon_319_PTHREAD_MUTEX_ROBUST: anon_319 = 1;
pub const anon_319_PTHREAD_MUTEX_ROBUST_NP: anon_319 = 1;
pub type anon_320 = u32;
pub const anon_320_PTHREAD_RWLOCK_PREFER_READER_NP: anon_320 = 0;
pub const anon_320_PTHREAD_RWLOCK_PREFER_WRITER_NP: anon_320 = 1;
pub const anon_320_PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: anon_320 = 2;
pub const anon_320_PTHREAD_RWLOCK_DEFAULT_NP: anon_320 = 0;
#[derive(VaArg, FnPtrArg)]
pub struct _pthread_cleanup_buffer {
    pub __routine: Option<unsafe fn(*mut ::libc::c_void)>,
    pub __arg: *mut ::libc::c_void,
    pub __canceltype: i32,
    pub __prev: *mut _pthread_cleanup_buffer,
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
pub type anon_321 = u32;
pub const anon_321_PTHREAD_CANCEL_DEFERRED: anon_321 = 0;
pub const anon_321_PTHREAD_CANCEL_ASYNCHRONOUS: anon_321 = 1;
#[derive(VaArg, FnPtrArg)]
pub struct __cancel_jmp_buf_tag {
    pub __cancel_jmp_buf: [i64; 8],
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
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_unwind_buf_t {
    pub __cancel_jmp_buf: [__cancel_jmp_buf_tag; 1],
    pub __pad: [*mut ::libc::c_void; 4],
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
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_cleanup_frame {
    pub __cancel_routine: Option<unsafe fn(*mut ::libc::c_void)>,
    pub __cancel_arg: *mut ::libc::c_void,
    pub __do_it: i32,
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut __libc_single_threaded_322: std::cell::LazyCell<libc::c_char> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
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
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn copy_from(__other: *const std___exception_ptr_exception_ptr) -> Self {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptr {
    pub unsafe fn move_from(__o: *mut std___exception_ptr_exception_ptr) -> Self {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptr {
    pub unsafe fn copy_assign(
        &mut self,
        __other: *const std___exception_ptr_exception_ptr,
    ) -> *mut std___exception_ptr_exception_ptr {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptr {
    pub unsafe fn move_assign(
        &mut self,
        __o: *mut std___exception_ptr_exception_ptr,
    ) -> *mut std___exception_ptr_exception_ptr {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptr {
    pub unsafe fn swap(&mut self, __other: *mut std___exception_ptr_exception_ptr) {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptr {
    pub unsafe fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn copy_from(_a0: *const std_nested_exception) -> Self {
        unimplemented!()
    }
}
impl std_nested_exception {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std_nested_exception,
    ) -> *mut std_nested_exception {
        unimplemented!()
    }
}
pub type __gnu_cxx__Lock_policy = u32;
pub const __gnu_cxx__Lock_policy__S_single: __gnu_cxx__Lock_policy = 0;
pub const __gnu_cxx__Lock_policy__S_mutex: __gnu_cxx__Lock_policy = 1;
pub const __gnu_cxx__Lock_policy__S_atomic: __gnu_cxx__Lock_policy = 2;
pub static mut __default_lock_policy_323: std::cell::LazyCell<__gnu_cxx__Lock_policy> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __gnu_cxx___concurrence_lock_error {}
impl Clone for __gnu_cxx___concurrence_lock_error {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___concurrence_lock_error {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_lock_error {
    unsafe fn what(&self) -> *const libc::c_char {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_lock_error {}
impl __gnu_cxx___concurrence_lock_error {}
impl __gnu_cxx___concurrence_lock_error {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __gnu_cxx___concurrence_lock_error,
    ) -> *mut __gnu_cxx___concurrence_lock_error {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_lock_error {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut __gnu_cxx___concurrence_lock_error,
    ) -> *mut __gnu_cxx___concurrence_lock_error {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_lock_error {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __gnu_cxx___concurrence_unlock_error {}
impl Clone for __gnu_cxx___concurrence_unlock_error {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___concurrence_unlock_error {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_unlock_error {
    unsafe fn what(&self) -> *const libc::c_char {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_unlock_error {}
impl __gnu_cxx___concurrence_unlock_error {}
impl __gnu_cxx___concurrence_unlock_error {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __gnu_cxx___concurrence_unlock_error,
    ) -> *mut __gnu_cxx___concurrence_unlock_error {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_unlock_error {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut __gnu_cxx___concurrence_unlock_error,
    ) -> *mut __gnu_cxx___concurrence_unlock_error {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_unlock_error {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __gnu_cxx___concurrence_broadcast_error {}
impl Clone for __gnu_cxx___concurrence_broadcast_error {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___concurrence_broadcast_error {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_broadcast_error {
    unsafe fn what(&self) -> *const libc::c_char {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_broadcast_error {}
impl __gnu_cxx___concurrence_broadcast_error {}
impl __gnu_cxx___concurrence_broadcast_error {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __gnu_cxx___concurrence_broadcast_error,
    ) -> *mut __gnu_cxx___concurrence_broadcast_error {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_broadcast_error {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut __gnu_cxx___concurrence_broadcast_error,
    ) -> *mut __gnu_cxx___concurrence_broadcast_error {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_broadcast_error {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __gnu_cxx___concurrence_wait_error {}
impl Clone for __gnu_cxx___concurrence_wait_error {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___concurrence_wait_error {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_wait_error {
    unsafe fn what(&self) -> *const libc::c_char {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_wait_error {}
impl __gnu_cxx___concurrence_wait_error {}
impl __gnu_cxx___concurrence_wait_error {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __gnu_cxx___concurrence_wait_error,
    ) -> *mut __gnu_cxx___concurrence_wait_error {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_wait_error {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut __gnu_cxx___concurrence_wait_error,
    ) -> *mut __gnu_cxx___concurrence_wait_error {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_wait_error {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __gnu_cxx___mutex {}
impl Clone for __gnu_cxx___mutex {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___mutex {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___mutex {
    pub unsafe fn lock(&mut self) {
        unimplemented!()
    }
}
impl __gnu_cxx___mutex {
    pub unsafe fn unlock(&mut self) {
        unimplemented!()
    }
}
impl __gnu_cxx___mutex {
    pub unsafe fn gthread_mutex(&mut self) -> *mut pthread_mutex_t {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __gnu_cxx___recursive_mutex {}
impl Clone for __gnu_cxx___recursive_mutex {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___recursive_mutex {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___recursive_mutex {
    pub unsafe fn gthread_recursive_mutex(&mut self) -> *mut pthread_mutex_t {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __gnu_cxx___scoped_lock {}
impl Clone for __gnu_cxx___scoped_lock {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___scoped_lock {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___scoped_lock {
    pub unsafe fn new(__name: *mut __gnu_cxx___mutex) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __gnu_cxx___cond {}
impl Clone for __gnu_cxx___cond {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___cond {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_bad_weak_ptr {}
impl Clone for std_bad_weak_ptr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_bad_weak_ptr {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_bad_weak_ptr {}
impl std_bad_weak_ptr {
    pub unsafe fn copy_assign(&mut self, _a0: *const std_bad_weak_ptr) -> *mut std_bad_weak_ptr {
        unimplemented!()
    }
}
impl std_bad_weak_ptr {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Mutex_base_324 {}
impl Clone for _Mutex_base_324 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Mutex_base_324 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_325 = u32;
pub const anon_325__S_need_barriers: anon_325 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Mutex_base_326 {}
impl Clone for _Mutex_base_326 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Mutex_base_326 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Mutex_base_326 {}
impl _Mutex_base_326 {}
impl _Mutex_base_326 {
    pub unsafe fn copy_assign(&mut self, _a0: *const _Mutex_base_326) -> *mut _Mutex_base_326 {
        unimplemented!()
    }
}
impl _Mutex_base_326 {
    pub unsafe fn move_assign(&mut self, _a0: *mut _Mutex_base_326) -> *mut _Mutex_base_326 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Mutex_base_327 {}
impl Clone for _Mutex_base_327 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Mutex_base_327 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_328 = u32;
pub const anon_328__S_need_barriers: anon_328 = 0;
impl _Mutex_base_327 {}
impl _Mutex_base_327 {}
impl _Mutex_base_327 {
    pub unsafe fn copy_assign(&mut self, _a0: *const _Mutex_base_327) -> *mut _Mutex_base_327 {
        unimplemented!()
    }
}
impl _Mutex_base_327 {
    pub unsafe fn move_assign(&mut self, _a0: *mut _Mutex_base_327) -> *mut _Mutex_base_327 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Sp_counted_ptr_329 {}
impl Clone for _Sp_counted_ptr_329 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Sp_counted_ptr_329 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Sp_counted_ptr_330 {}
impl Clone for _Sp_counted_ptr_330 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Sp_counted_ptr_330 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Sp_counted_ptr_331 {}
impl Clone for _Sp_counted_ptr_331 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Sp_counted_ptr_331 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std__Sp_make_shared_tag {}
impl Clone for std__Sp_make_shared_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Sp_make_shared_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _Sp_alloc_shared_tag_332 {
    pub _M_a: *const allocator_0,
}
impl Clone for _Sp_alloc_shared_tag_332 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Sp_alloc_shared_tag_332 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___sp_array_delete {}
impl Clone for std___sp_array_delete {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___sp_array_delete {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Sp_owner_less_333 {}
impl Clone for _Sp_owner_less_333 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Sp_owner_less_333 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type std_memory_order = i32;
pub const std_memory_order_memory_order_relaxed: std_memory_order = 0;
pub const std_memory_order_memory_order_consume: std_memory_order = 1;
pub const std_memory_order_memory_order_acquire: std_memory_order = 2;
pub const std_memory_order_memory_order_release: std_memory_order = 3;
pub const std_memory_order_memory_order_acq_rel: std_memory_order = 4;
pub const std_memory_order_memory_order_seq_cst: std_memory_order = 5;
pub type std___memory_order_modifier = u32;
pub const std___memory_order_modifier___memory_order_mask: std___memory_order_modifier = 65535;
pub const std___memory_order_modifier___memory_order_modifier_mask: std___memory_order_modifier =
    4294901760;
pub const std___memory_order_modifier___memory_order_hle_acquire: std___memory_order_modifier =
    65536;
pub const std___memory_order_modifier___memory_order_hle_release: std___memory_order_modifier =
    131072;
#[derive(VaArg, FnPtrArg)]
pub struct std___atomic_flag_base {
    pub _M_i: bool,
}
impl Clone for std___atomic_flag_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___atomic_flag_base {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___atomic_flag_base {}
impl std___atomic_flag_base {}
impl std___atomic_flag_base {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std___atomic_flag_base,
    ) -> *mut std___atomic_flag_base {
        unimplemented!()
    }
}
impl std___atomic_flag_base {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut std___atomic_flag_base,
    ) -> *mut std___atomic_flag_base {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_atomic_flag {}
impl Clone for std_atomic_flag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_atomic_flag {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std__Sp_locker {}
impl Clone for std__Sp_locker {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Sp_locker {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type std_pointer_safety = i32;
pub const std_pointer_safety_relaxed: std_pointer_safety = 0;
pub const std_pointer_safety_preferred: std_pointer_safety = 1;
pub const std_pointer_safety_strict: std_pointer_safety = 2;
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Edge {
    pub u: i32,
    pub v: i32,
    pub weight: f64,
}
pub unsafe fn partition_334(arr: *mut unique_ptr_301, mut start: i32, mut end: i32) -> i32 {
    let pivot: *mut Edge = &mut (*(unsafe {
        let ___i: usize = (start as usize);
        unique_ptr_301::operator_index(&(*arr), ___i)
    }));
    let mut count: i32 = 0;
    let mut i: i32 = ((start) + (1));
    'loop_: while ((i) <= (end)) {
        if (((*(unsafe {
            let ___i: usize = (i as usize);
            unique_ptr_301::operator_index(&(*arr), ___i)
        }))
        .weight)
            <= ((*pivot).weight))
        {
            count.postfix_inc();
        }
        i.prefix_inc();
    }
    let mut pidx: i32 = ((start) + (count));
    let mut tmp: Edge = Edge {
        u: (*(unsafe {
            let ___i: usize = (pidx as usize);
            unique_ptr_301::operator_index(&(*arr), ___i)
        }))
        .u,
        v: (*(unsafe {
            let ___i: usize = (pidx as usize);
            unique_ptr_301::operator_index(&(*arr), ___i)
        }))
        .v,
        weight: (*(unsafe {
            let ___i: usize = (pidx as usize);
            unique_ptr_301::operator_index(&(*arr), ___i)
        }))
        .weight,
    };
    (*(unsafe {
        let ___i: usize = (pidx as usize);
        unique_ptr_301::operator_index(&(*arr), ___i)
    })) = Edge {
        u: (*(unsafe {
            let ___i: usize = (start as usize);
            unique_ptr_301::operator_index(&(*arr), ___i)
        }))
        .u,
        v: (*(unsafe {
            let ___i: usize = (start as usize);
            unique_ptr_301::operator_index(&(*arr), ___i)
        }))
        .v,
        weight: (*(unsafe {
            let ___i: usize = (start as usize);
            unique_ptr_301::operator_index(&(*arr), ___i)
        }))
        .weight,
    };
    (*(unsafe {
        let ___i: usize = (start as usize);
        unique_ptr_301::operator_index(&(*arr), ___i)
    })) = Edge {
        u: tmp.u,
        v: tmp.v,
        weight: tmp.weight,
    };
    let mut i: i32 = start;
    let mut j: i32 = end;
    'loop_: while ((i) < (pidx)) && ((j) > (pidx)) {
        'loop_: while (((*(unsafe {
            let ___i: usize = (i as usize);
            unique_ptr_301::operator_index(&(*arr), ___i)
        }))
        .weight)
            <= ((*pivot).weight))
        {
            i.prefix_inc();
        }
        'loop_: while (((*(unsafe {
            let ___i: usize = (j as usize);
            unique_ptr_301::operator_index(&(*arr), ___i)
        }))
        .weight)
            > ((*pivot).weight))
        {
            j.prefix_dec();
        }
        if ((i) < (pidx)) && ((j) > (pidx)) {
            tmp = Edge {
                u: (*(unsafe {
                    let ___i: usize = (i as usize);
                    unique_ptr_301::operator_index(&(*arr), ___i)
                }))
                .u,
                v: (*(unsafe {
                    let ___i: usize = (i as usize);
                    unique_ptr_301::operator_index(&(*arr), ___i)
                }))
                .v,
                weight: (*(unsafe {
                    let ___i: usize = (i as usize);
                    unique_ptr_301::operator_index(&(*arr), ___i)
                }))
                .weight,
            };
            (*(unsafe {
                let ___i: usize = (i as usize);
                unique_ptr_301::operator_index(&(*arr), ___i)
            })) = Edge {
                u: (*(unsafe {
                    let ___i: usize = (j as usize);
                    unique_ptr_301::operator_index(&(*arr), ___i)
                }))
                .u,
                v: (*(unsafe {
                    let ___i: usize = (j as usize);
                    unique_ptr_301::operator_index(&(*arr), ___i)
                }))
                .v,
                weight: (*(unsafe {
                    let ___i: usize = (j as usize);
                    unique_ptr_301::operator_index(&(*arr), ___i)
                }))
                .weight,
            };
            (*(unsafe {
                let ___i: usize = (j as usize);
                unique_ptr_301::operator_index(&(*arr), ___i)
            })) = Edge {
                u: tmp.u,
                v: tmp.v,
                weight: tmp.weight,
            };
            i.postfix_inc();
            j.postfix_dec();
        }
    }
    return pidx;
}
pub unsafe fn quicksort_335(arr: *mut unique_ptr_301, mut start: i32, mut end: i32) {
    if ((start) >= (end)) {
        return;
    }
    let mut p: i32 = (unsafe {
        let _arr: *mut unique_ptr_301 = arr;
        let _start: i32 = start;
        let _end: i32 = end;
        partition_334(_arr, _start, _end)
    });
    (unsafe {
        let _arr: *mut unique_ptr_301 = arr;
        let _start: i32 = start;
        let _end: i32 = ((p) - (1));
        quicksort_335(_arr, _start, _end)
    });
    (unsafe {
        let _arr: *mut unique_ptr_301 = arr;
        let _start: i32 = ((p) + (1));
        let _end: i32 = end;
        quicksort_335(_arr, _start, _end)
    });
}
#[repr(C)]
#[derive(MoveCtorUnsafe, FnPtrArg, Default)]
pub struct DisjointSet {
    pub rank: unique_ptr_302,
    pub parent: unique_ptr_302,
    pub n: i32,
}
impl DisjointSet {
    pub unsafe fn makeSet(&mut self) {
        let mut i: i32 = 0;
        'loop_: while ((i) < (self.n)) {
            (*(unsafe { unique_ptr_302::operator_index(&self.parent, (i as usize)) })) = i;
            (*(unsafe { unique_ptr_302::operator_index(&self.rank, (i as usize)) })) = 1;
            i.postfix_inc();
        }
    }
    pub unsafe fn find(&mut self, mut x: i32) -> i32 {
        if ((*(unsafe { unique_ptr_302::operator_index(&self.parent, (x as usize)) })) != (x)) {
            (*(unsafe { unique_ptr_302::operator_index(&self.parent, (x as usize)) })) = (unsafe {
                let _x: i32 =
                    (*(unsafe { unique_ptr_302::operator_index(&self.parent, (x as usize)) }));
                DisjointSet::find(self, _x)
            });
        }
        return (*(unsafe { unique_ptr_302::operator_index(&self.parent, (x as usize)) }));
    }
    pub unsafe fn merge(&mut self, mut x: i32, mut y: i32) {
        let mut xset: i32 = (unsafe { DisjointSet::find(self, x) });
        let mut yset: i32 = (unsafe { DisjointSet::find(self, y) });
        if ((xset) == (yset)) {
            return;
        }
        if ((*(unsafe { unique_ptr_302::operator_index(&self.rank, (xset as usize)) }))
            < (*(unsafe { unique_ptr_302::operator_index(&self.rank, (yset as usize)) })))
        {
            (*(unsafe { unique_ptr_302::operator_index(&self.parent, (xset as usize)) })) = yset;
        } else if ((*(unsafe { unique_ptr_302::operator_index(&self.rank, (xset as usize)) }))
            > (*(unsafe { unique_ptr_302::operator_index(&self.rank, (yset as usize)) })))
        {
            (*(unsafe { unique_ptr_302::operator_index(&self.parent, (yset as usize)) })) = xset;
        } else {
            (*(unsafe { unique_ptr_302::operator_index(&self.parent, (yset as usize)) })) = xset;
            (*(unsafe { unique_ptr_302::operator_index(&self.rank, (xset as usize)) })) =
                ((*(unsafe { unique_ptr_302::operator_index(&self.rank, (xset as usize)) })) + (1));
        }
    }
    pub unsafe fn move_from(_a0: *mut DisjointSet) -> Self {
        let mut this = Self {
            rank: unique_ptr_302::move_from({ &mut (*_a0).rank }),
            parent: unique_ptr_302::move_from({ &mut (*_a0).parent }),
            n: (*_a0).n,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut DisjointSet) -> *mut DisjointSet {
        self.rank = ((*_a0).rank).clone();
        self.parent = ((*_a0).parent).clone();
        self.n = (*_a0).n;
        return &mut (*(self as *mut DisjointSet));
    }
}
#[repr(C)]
#[derive(MoveCtorUnsafe, FnPtrArg, Default)]
pub struct Graph {
    pub edges: unique_ptr_301,
    pub V: i32,
    pub E: i32,
}
impl Graph {
    pub unsafe fn move_from(_a0: *mut Graph) -> Self {
        let mut this = Self {
            edges: unique_ptr_301::move_from({ &mut (*_a0).edges }),
            V: (*_a0).V,
            E: (*_a0).E,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Graph) -> *mut Graph {
        self.edges = ((*_a0).edges).clone();
        self.V = (*_a0).V;
        self.E = (*_a0).E;
        return &mut (*(self as *mut Graph));
    }
}
pub unsafe fn MSTKruskal_336(graph: *mut Graph) -> f64 {
    (unsafe {
        let _arr: *mut unique_ptr_301 = &mut (*graph).edges;
        let _end: i32 = (((*graph).E) - (1));
        quicksort_335(_arr, 0, _end)
    });
    let mut set: DisjointSet = DisjointSet {
        rank: (unsafe { make_unique_337(((*graph).V as usize)) }),
        parent: (unsafe { make_unique_337(((*graph).V as usize)) }),
        n: (*graph).V,
    };
    (unsafe { DisjointSet::makeSet(&mut set) });
    let mut total_weight: f64 = 0_f64;
    let mut i: i32 = 0;
    'loop_: while ((i) < ((*graph).E)) {
        let mut x: i32 =
            (*(unsafe { unique_ptr_301::operator_index(&(*graph).edges, (i as usize)) })).u;
        let mut y: i32 =
            (*(unsafe { unique_ptr_301::operator_index(&(*graph).edges, (i as usize)) })).v;
        let mut w: f64 =
            (*(unsafe { unique_ptr_301::operator_index(&(*graph).edges, (i as usize)) })).weight;
        if ((unsafe { DisjointSet::find(&mut set, x) })
            != (unsafe { DisjointSet::find(&mut set, y) }))
        {
            (unsafe { DisjointSet::merge(&mut set, x, y) });
            total_weight += w;
        }
        i.prefix_inc();
    }
    return total_weight;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut V: i32 = 4;
    let mut E: i32 = 5;
    let mut graph: Graph = Graph {
        edges: (unsafe { make_unique_338((E as usize)) }),
        V: V,
        E: E,
    };
    (*(unsafe { unique_ptr_301::operator_index(&graph.edges, 0_usize) })) = Edge {
        u: 0,
        v: 1,
        weight: 10_f64,
    };
    (*(unsafe { unique_ptr_301::operator_index(&graph.edges, 1_usize) })) = Edge {
        u: 1,
        v: 3,
        weight: 15_f64,
    };
    (*(unsafe { unique_ptr_301::operator_index(&graph.edges, 2_usize) })) = Edge {
        u: 2,
        v: 3,
        weight: 4_f64,
    };
    (*(unsafe { unique_ptr_301::operator_index(&graph.edges, 3_usize) })) = Edge {
        u: 2,
        v: 0,
        weight: 6_f64,
    };
    (*(unsafe { unique_ptr_301::operator_index(&graph.edges, 4_usize) })) = Edge {
        u: 0,
        v: 3,
        weight: 5_f64,
    };
    let mut total_weight: f64 = (unsafe { MSTKruskal_336(&mut graph) });
    (if ((total_weight) == (19_f64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_339(
                c"total_weight == 19".as_ptr(),
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
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct __locale_data;
pub unsafe fn make_unique_337(mut __num: usize) -> unique_ptr_302 {
    unimplemented!()
}
pub unsafe fn make_unique_338(mut __num: usize) -> unique_ptr_301 {
    unimplemented!()
}
pub unsafe fn __assert_fail_339(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
