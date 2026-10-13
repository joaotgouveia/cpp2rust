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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __mbstate_t {
    #[offset(0)]
    pub __count: i32,
    #[offset(4)]
    #[byte_size(4)]
    pub __value: anon_0,
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
pub struct anon_0 {
    #[offset(0)]
    pub __wch: u32,
    #[offset(0)]
    #[byte_size(4)]
    pub __wchb: Value<Box<[i8]>>,
}
impl Clone for anon_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_0 {
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_1 {}
impl Clone for integral_constant_1 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_1 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_integral_constant_bool__true__value_2: Value<bool> = unimplemented!();
);
impl integral_constant_1 {}
impl integral_constant_1 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_3 {}
impl Clone for integral_constant_3 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_3 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_integral_constant_bool__false__value_4: Value<bool> = unimplemented!();
);
impl integral_constant_3 {}
impl integral_constant_3 {}
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
pub struct __or__18 {}
impl Clone for __or__18 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__18 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__19 {}
impl Clone for __or__19 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__19 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__20 {}
impl Clone for __or__20 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__20 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__21 {}
impl Clone for __or__21 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__21 {
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
pub struct __and__33 {}
impl Clone for __and__33 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__33 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__34 {}
impl Clone for __and__34 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__34 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__35 {}
impl Clone for __and__35 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__35 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_38 {}
impl Clone for is_reference_38 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_38 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_39 {}
impl Clone for is_reference_39 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_39 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_40 {}
impl Clone for is_reference_40 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_40 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_41 {}
impl Clone for is_reference_41 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_41 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_42 {}
impl Clone for is_reference_42 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_42 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_43 {}
impl Clone for is_reference_43 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_43 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_44 {}
impl Clone for is_reference_44 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_44 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_45 {}
impl Clone for is_function_45 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_45 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_46 {}
impl Clone for is_function_46 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_46 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_47 {}
impl Clone for is_function_47 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_47 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_48 {}
impl Clone for is_function_48 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_48 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_49 {}
impl Clone for is_function_49 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_49 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_50 {}
impl Clone for is_function_50 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_50 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_51 {}
impl Clone for is_function_51 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_51 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_52 {}
impl Clone for is_void_52 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_52 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_53 {}
impl Clone for is_void_53 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_53 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_54 {}
impl Clone for is_void_54 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_54 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_55 {}
impl Clone for is_void_55 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_55 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_56 {}
impl Clone for is_void_56 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_56 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_57 {}
impl Clone for is_void_57 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_57 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_58 {}
impl Clone for is_void_58 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_58 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_59 {}
impl Clone for is_void_59 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_59 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_60 {}
impl Clone for remove_cv_60 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_60 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_61 {}
impl Clone for remove_cv_61 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_61 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_62 {}
impl Clone for remove_cv_62 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_62 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_63 {}
impl Clone for remove_cv_63 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_63 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_64 {}
impl Clone for remove_cv_64 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_64 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_65 {}
impl Clone for remove_cv_65 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_65 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_66 {}
impl Clone for remove_cv_66 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_66 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_67 {}
impl Clone for remove_cv_67 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_67 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_68 {}
impl Clone for is_const_68 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_68 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_69 {}
impl Clone for is_const_69 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_69 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_70 {}
impl Clone for is_const_70 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_70 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_71 {}
impl Clone for is_const_71 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_71 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_72 {}
impl Clone for is_const_72 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_72 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_73 {}
impl Clone for __is_array_unknown_bounds_73 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_73 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_74 {}
impl Clone for __is_array_unknown_bounds_74 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_74 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_75 {}
impl Clone for __is_array_unknown_bounds_75 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_75 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_76 {}
impl Clone for __is_array_unknown_bounds_76 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_76 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_77 {}
impl Clone for __is_array_unknown_bounds_77 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_77 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_78 {}
impl Clone for __is_array_unknown_bounds_78 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_78 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_79 {}
impl Clone for __is_array_unknown_bounds_79 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_79 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_80 {}
impl Clone for __is_integral_helper_80 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_80 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_81 {}
impl Clone for __is_integral_helper_81 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_81 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_82 {}
impl Clone for __is_integral_helper_82 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_82 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_83 {}
impl Clone for __is_integral_helper_83 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_83 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_84 {}
impl Clone for __is_integral_helper_84 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_84 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_85 {}
impl Clone for __is_integral_helper_85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_85 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_86 {}
impl Clone for __is_integral_helper_86 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_86 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_87 {}
impl Clone for __is_integral_helper_87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_88 {}
impl Clone for is_integral_88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_88 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_89 {}
impl Clone for is_integral_89 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_89 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_90 {}
impl Clone for is_integral_90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_90 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_91 {}
impl Clone for is_integral_91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_91 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_92 {}
impl Clone for is_integral_92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_93 {}
impl Clone for is_integral_93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_93 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_94 {}
impl Clone for is_integral_94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_94 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_95 {}
impl Clone for is_integral_95 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_95 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_floating_point_helper_96 {}
impl Clone for __is_floating_point_helper_96 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_floating_point_helper_96 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_floating_point_97 {}
impl Clone for is_floating_point_97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_floating_point_97 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_98 {}
impl Clone for is_pointer_98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_98 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_enum_99 {}
impl Clone for is_enum_99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_enum_99 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_null_pointer_100 {}
impl Clone for is_null_pointer_100 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_null_pointer_100 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_101 {}
impl Clone for is_arithmetic_101 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_101 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_102 {}
impl Clone for is_arithmetic_102 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_102 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_member_pointer_103 {}
impl Clone for is_member_pointer_103 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_pointer_103 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_104 {}
impl Clone for is_scalar_104 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_104 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_105 {}
impl Clone for is_scalar_105 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_105 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_107 {}
impl Clone for is_same_107 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_107 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_112 {}
impl Clone for is_volatile_112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_112 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_all_extents_113 {}
impl Clone for remove_all_extents_113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_all_extents_113 {
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
pub struct __is_destructible_impl_114 {}
impl Clone for __is_destructible_impl_114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_impl_114 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_safe_115 {}
impl Clone for __is_destructible_safe_115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_115 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_safe_116 {}
impl Clone for __is_destructible_safe_116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_116 {
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
pub struct __is_nt_destructible_impl_117 {}
impl Clone for __is_nt_destructible_impl_117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_impl_117 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nt_destructible_safe_118 {}
impl Clone for __is_nt_destructible_safe_118 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_safe_118 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_destructible_119 {}
impl Clone for is_nothrow_destructible_119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_destructible_119 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_120 {}
impl Clone for is_constructible_120 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_120 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_122 {}
impl Clone for is_move_constructible_122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_122 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_125 {}
impl Clone for is_nothrow_copy_constructible_125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_125 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_126 {}
impl Clone for is_nothrow_copy_constructible_126 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_126 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_127 {}
impl Clone for is_nothrow_copy_constructible_127 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_127 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_copy_assignable_128 {}
impl Clone for is_copy_assignable_128 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_assignable_128 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_copy_assignable_129 {}
impl Clone for is_copy_assignable_129 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_assignable_129 {
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
pub struct is_trivially_destructible_130 {}
impl Clone for is_trivially_destructible_130 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_130 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_trivially_destructible_131 {}
impl Clone for is_trivially_destructible_131 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_131 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_132 {}
impl Clone for is_convertible_132 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_132 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_133 {}
impl Clone for __is_nothrow_new_constructible_impl_133 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_133 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_134 {}
impl Clone for remove_reference_134 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_134 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_135 {}
impl Clone for remove_reference_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_135 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_136 {}
impl Clone for remove_reference_136 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_136 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_137 {}
impl Clone for __cv_selector_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_137 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_138 {}
impl Clone for __cv_selector_138 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_138 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_139 {}
impl Clone for __cv_selector_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_139 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_140 {}
impl Clone for __cv_selector_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_140 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_141 {}
impl Clone for __match_cv_qualifiers_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_142 {}
impl Clone for __match_cv_qualifiers_142 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_142 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_143 {}
impl Clone for __match_cv_qualifiers_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_143 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_144 {}
impl Clone for __match_cv_qualifiers_144 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_144 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_145 {}
impl Clone for __match_cv_qualifiers_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_145 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_146 {}
impl Clone for __make_unsigned_146 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_146 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_147 {}
impl Clone for __make_unsigned_147 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_147 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_148 {}
impl Clone for __make_unsigned_148 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_148 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_149 {}
impl Clone for __make_unsigned_selector_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_150 {}
impl Clone for __make_unsigned_selector_150 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_150 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_151 {}
impl Clone for __make_unsigned_selector_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_152 {}
impl Clone for __make_unsigned_selector_152 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_152 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_153 {}
impl Clone for __make_unsigned_selector_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_154 {}
impl Clone for __make_unsigned_selector_154 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_154 {
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
pub struct _List_158 {}
impl Clone for _List_158 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_158 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_159 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_160 {}
impl Clone for _List_160 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_160 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_161 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_162 {}
impl Clone for _List_162 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_162 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_163 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_164 {}
impl Clone for _List_164 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_164 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_165 {}
impl Clone for _List_165 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_165 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_166 {}
impl Clone for _List_166 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_166 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_167 {}
impl Clone for __select_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_167 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_168 {}
impl Clone for __select_168 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_168 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_169 {}
impl Clone for __select_169 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_169 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_170 {}
impl Clone for __select_170 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_170 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_171 {}
impl Clone for __select_171 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_171 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_172 {}
impl Clone for __make_signed_172 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_172 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_173 {}
impl Clone for __make_signed_173 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_173 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_174 {}
impl Clone for __make_signed_selector_174 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_174 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_175 {}
impl Clone for __make_signed_selector_175 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_175 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_176 {}
impl Clone for __make_signed_selector_176 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_176 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_177 {}
impl Clone for __make_signed_selector_177 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_177 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_178 {}
impl Clone for __make_signed_selector_178 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_178 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_pointer_179 {}
impl Clone for remove_pointer_179 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_pointer_179 {
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
pub struct common_type_180 {}
impl Clone for common_type_180 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_180 {
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
    pub static __is_nothrow_new_constructible_181: Value<bool> = unimplemented!();
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
pub struct __are_same_182 {}
impl Clone for __are_same_182 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_182 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_183 = u32;
pub const anon_183___value: anon_183 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_184 {}
impl Clone for __are_same_184 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_184 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_185 = u32;
pub const anon_185___value: anon_185 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_186 {}
impl Clone for __are_same_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_187 = u32;
pub const anon_187___value: anon_187 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_188 {}
impl Clone for __are_same_188 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_188 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_189 = u32;
pub const anon_189___value: anon_189 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_190 {}
impl Clone for __are_same_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_190 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_191 = u32;
pub const anon_191___value: anon_191 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_192 {}
impl Clone for __are_same_192 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_192 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_193 = u32;
pub const anon_193___value: anon_193 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_194 {}
impl Clone for __are_same_194 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_194 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_195 = u32;
pub const anon_195___value: anon_195 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_196 {}
impl Clone for __is_integer_196 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_196 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_197 = u32;
pub const anon_197___value: anon_197 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_198 {}
impl Clone for __is_integer_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_199 = u32;
pub const anon_199___value: anon_199 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_200 {}
impl Clone for __is_integer_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_200 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_201 = u32;
pub const anon_201___value: anon_201 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_202 {}
impl Clone for __is_integer_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_202 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_203 = u32;
pub const anon_203___value: anon_203 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_204 {}
impl Clone for __is_integer_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_204 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_205 = u32;
pub const anon_205___value: anon_205 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_206 {}
impl Clone for __is_integer_206 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_206 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_207 = u32;
pub const anon_207___value: anon_207 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_byte_208 {}
impl Clone for __is_byte_208 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_byte_208 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_209 = u32;
pub const anon_209___value: anon_209 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nonvolatile_trivially_copyable_210 {}
impl Clone for __is_nonvolatile_trivially_copyable_210 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nonvolatile_trivially_copyable_210 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_211 = u32;
pub const anon_211___value: anon_211 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_212 {}
impl Clone for __memcpyable_212 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_212 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_213 {}
impl Clone for __memcpyable_integer_213 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_213 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_214 {}
impl Clone for __memcpyable_integer_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_214 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_215 = u32;
pub const anon_215___width: anon_215 = 32;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __enable_if_216 {}
impl Clone for __enable_if_216 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __enable_if_216 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __conditional_type_217 {}
impl Clone for __conditional_type_217 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_type_217 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_218 {}
impl Clone for __is_integer_nonstrict_218 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_218 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_219 = u32;
pub const anon_219___width: anon_219 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_220 {}
impl Clone for __is_integer_nonstrict_220 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_220 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_221 = u32;
pub const anon_221___width: anon_221 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_222 {}
impl Clone for __numeric_traits_integer_222 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_222 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____is_signed_223: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____digits_224: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____max_225: Value<i64> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_226 {}
impl Clone for __numeric_traits_integer_226 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_226 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_227: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____digits_228: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____max_229: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_230 {}
impl Clone for __numeric_traits_floating_230 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_230 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_231 {}
impl Clone for __numeric_traits_floating_231 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_231 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_232 {}
impl Clone for __numeric_traits_floating_232 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_232 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_233 {}
impl Clone for __numeric_traits_233 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_233 {
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
    pub static piecewise_construct_234: Value<std_piecewise_construct_t> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_235 {}
impl Clone for tuple_235 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_235 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_235 {}
impl tuple_235 {}
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_237 {}
impl Clone for iterator_237 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_237 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_237 {}
impl iterator_237 {}
impl iterator_237 {}
impl iterator_237 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_237>) -> Ptr<iterator_237> {
        unimplemented!()
    }
}
impl iterator_237 {
    pub fn move_assign(&self, _a0: Ptr<iterator_237>) -> Ptr<iterator_237> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_238 {}
impl Clone for iterator_238 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_238 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_238 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_238>) -> Ptr<iterator_238> {
        unimplemented!()
    }
}
impl iterator_238 {
    pub fn move_assign(&self, _a0: Ptr<iterator_238>) -> Ptr<iterator_238> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_239 {}
impl Clone for iterator_239 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_239 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_239 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_239>) -> Ptr<iterator_239> {
        unimplemented!()
    }
}
impl iterator_239 {
    pub fn move_assign(&self, _a0: Ptr<iterator_239>) -> Ptr<iterator_239> {
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
pub struct iterator_traits_243 {}
impl Clone for iterator_traits_243 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_243 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_244 {}
impl Clone for iterator_traits_244 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_244 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_245 {}
impl Clone for __iterator_traits_245 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_245 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_246 {}
impl Clone for __iterator_traits_246 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_246 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_247 {}
impl Clone for reverse_iterator_247 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_247 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_247 {
    pub fn copy_from(__x: Ptr<reverse_iterator_247>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_247 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_247>) -> Ptr<reverse_iterator_247> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_248 {}
impl Clone for reverse_iterator_248 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_248 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_248 {
    pub fn copy_from(__x: Ptr<reverse_iterator_248>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_248 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_248>) -> Ptr<reverse_iterator_248> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_249 {}
impl Clone for __normal_iterator_249 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_249 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_249 {
    pub fn new_1(__i: Ptr<Ptr<vector_250>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_249 {}
impl __normal_iterator_249 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_251 {}
impl Clone for __normal_iterator_251 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_251 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_251 {
    pub fn new_1(__i: Ptr<Ptr<i32>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_251 {}
impl __normal_iterator_251 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_252 {}
impl Clone for __normal_iterator_252 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_252 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_252 {
    pub fn new_1(__i: Ptr<Ptr<i32>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_252 {}
impl __normal_iterator_252 {}
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
pub struct __new_allocator_253 {}
impl Clone for __new_allocator_253 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_253 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_253 {
    pub fn copy_from(_a0: Ptr<__new_allocator_253>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_253 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_253>) -> Ptr<__new_allocator_253> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_254 {}
impl Clone for __new_allocator_254 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_254 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_254 {
    pub fn copy_from(_a0: Ptr<__new_allocator_254>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_254 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_254>) -> Ptr<__new_allocator_254> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_255 {}
impl Clone for __new_allocator_255 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_255 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_255 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_255 {
    pub fn copy_from(_a0: Ptr<__new_allocator_255>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_255 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_255>) -> Ptr<__new_allocator_255> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_258 {}
impl Clone for __new_allocator_258 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_258 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_258 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_258 {
    pub fn copy_from(_a0: Ptr<__new_allocator_258>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_258 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_258>) -> Ptr<__new_allocator_258> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_259 {}
impl Clone for allocator_259 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_259 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_259 {}
impl allocator_259 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_260 {}
impl Clone for allocator_260 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_260 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_260 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_260 {
    pub fn copy_from(__a: Ptr<allocator_260>) -> Self {
        unimplemented!()
    }
}
impl allocator_260 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_260>) -> Ptr<allocator_260> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_261 {}
impl Clone for allocator_261 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_261 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_261 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_261 {
    pub fn copy_from(__a: Ptr<allocator_261>) -> Self {
        unimplemented!()
    }
}
impl allocator_261 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_261>) -> Ptr<allocator_261> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_262 {}
impl Clone for allocator_traits_262 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_262 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_262 {
    pub fn allocate_1(__a: Ptr<allocator_261>, mut __n: u64) -> Ptr<vector_250> {
        unimplemented!()
    }
}
impl allocator_traits_262 {
    pub fn deallocate(__a: Ptr<allocator_261>, mut __p: Ptr<vector_250>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_262 {
    pub fn max_size(__a: Ptr<allocator_261>) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_262 {
    pub fn construct_263(
        __a: Ptr<allocator_261>,
        mut __p: Ptr<vector_250>,
        __args: Ptr<vector_250>,
    ) {
        unimplemented!()
    }
}
impl allocator_traits_262 {
    pub fn destroy_264(__a: Ptr<allocator_261>, mut __p: Ptr<vector_250>) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_265 {}
impl Clone for allocator_traits_265 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_265 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_265 {
    pub fn allocate_1(__a: Ptr<allocator_260>, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
}
impl allocator_traits_265 {
    pub fn deallocate(__a: Ptr<allocator_260>, mut __p: Ptr<i32>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_265 {
    pub fn max_size(__a: Ptr<allocator_260>) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_265 {
    pub fn select_on_container_copy_construction(__rhs: Ptr<allocator_260>) -> allocator_260 {
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
pub struct __is_move_insertable_267 {}
impl Clone for __is_move_insertable_267 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_267 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_move_insertable_268 {}
impl Clone for __is_move_insertable_268 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_268 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_269 {}
impl Clone for __alloc_traits_269 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_269 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_270 {}
impl Clone for rebind_270 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_270 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_271 {}
impl Clone for __alloc_traits_271 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_271 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_272 {}
impl Clone for rebind_272 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_272 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __alloc_traits_271 {
    pub fn _S_select_on_copy(__a: Ptr<allocator_260>) -> allocator_260 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct _UninitDestroyGuard_273 {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_first: Ptr<vector_250>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_cur: Ptr<Ptr<vector_250>>,
}
impl Clone for _UninitDestroyGuard_273 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _UninitDestroyGuard_273 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_273 {
    pub fn new(__first: Ptr<Ptr<vector_250>>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __uninitialized_default_n_1_274 {}
impl Clone for __uninitialized_default_n_1_274 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __uninitialized_default_n_1_274 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __uninitialized_default_n_1_274 {
    pub fn __uninit_default_n_275(mut __first: Ptr<i32>, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __uninitialized_default_n_1_276 {}
impl Clone for __uninitialized_default_n_1_276 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __uninitialized_default_n_1_276 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __uninitialized_default_n_1_276 {
    pub fn __uninit_default_n_277(__first: Ptr<vector_250>, mut __n: u64) -> Ptr<vector_250> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_bitwise_relocatable_278 {}
impl Clone for __is_bitwise_relocatable_278 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_bitwise_relocatable_278 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_bitwise_relocatable_279 {}
impl Clone for __is_bitwise_relocatable_279 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_bitwise_relocatable_279 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_280 {}
impl Clone for initializer_list_280 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_280 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_281 {}
impl Clone for initializer_list_281 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_281 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_281 {}
impl initializer_list_281 {}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_282 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl,
}
impl Clone for _Vector_base_282 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_282 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl_data {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_start: Ptr<vector_250>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_finish: Ptr<vector_250>,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_end_of_storage: Ptr<vector_250>,
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
impl _Vector_base_282 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_282 {
    pub fn move_from(_a0: Ptr<_Vector_base_282>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_283 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_int__std_allocator_int____Vector_impl,
}
impl Clone for _Vector_base_283 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_283 {
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
    pub fn new_2(__a: Ptr<allocator_260>) -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__std_allocator_int____Vector_impl {
    pub fn move_from(__x: Ptr<std__Vector_base_int__std_allocator_int____Vector_impl>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_283 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_283 {
    pub fn new_2(mut __n: usize, __a: Ptr<allocator_260>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_283 {
    pub fn move_from(_a0: Ptr<_Vector_base_283>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_250 {}
impl Clone for vector_250 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_250 {
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
    pub _M_vect: Ptr<_Vector_base_283>,
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
    pub fn new(mut __s: Ptr<i32>, mut __l: u64, __vect: Ptr<_Vector_base_283>) -> Self {
        unimplemented!()
    }
}
impl vector_250 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_250 {
    pub fn copy_from(__x: Ptr<vector_250>) -> Self {
        unimplemented!()
    }
}
impl vector_250 {
    pub fn move_from(_a0: Ptr<vector_250>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_284 {}
impl Clone for vector_284 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_284 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std_vector_std_vector_int____Guard_alloc {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_storage: Ptr<vector_250>,
    #[offset(8)]
    pub _M_len: u64,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_vect: Ptr<_Vector_base_282>,
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
    pub fn new(mut __s: Ptr<vector_250>, mut __l: u64, __vect: Ptr<_Vector_base_282>) -> Self {
        unimplemented!()
    }
}
impl vector_284 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_284 {
    pub fn copy_from(__x: Ptr<vector_284>) -> Self {
        unimplemented!()
    }
}
impl vector_284 {
    pub fn move_from(_a0: Ptr<vector_284>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_290 {}
impl Clone for __hash_base_290 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_290 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_291 {}
impl Clone for __hash_base_291 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_291 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_292 {}
impl Clone for __hash_base_292 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_292 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_293 {}
impl Clone for __hash_base_293 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_293 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_294 {}
impl Clone for __hash_base_294 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_294 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_295 {}
impl Clone for __hash_base_295 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_295 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_296 {}
impl Clone for __hash_base_296 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_296 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_297 {}
impl Clone for __hash_base_297 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_297 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_298 {}
impl Clone for __hash_base_298 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_298 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_299 {}
impl Clone for __hash_base_299 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_299 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_300 {}
impl Clone for __hash_base_300 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_300 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_301 {}
impl Clone for __hash_base_301 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_301 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_302 {}
impl Clone for __hash_base_302 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_302 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_303 {}
impl Clone for __hash_base_303 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_303 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_304 {}
impl Clone for __hash_base_304 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_304 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_305 {}
impl Clone for __hash_base_305 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_305 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_306 {}
impl Clone for hash_306 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_306 {
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
    pub fn hash_307(__val: Ptr<f32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_308(__val: Ptr<f64>) -> usize {
        unimplemented!()
    }
}
pub type anon_309 = u32;
pub const anon_309__S_word_bit: anon_309 = 64;
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
pub struct binary_function_310 {}
impl Clone for binary_function_310 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_310 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_311 {}
impl Clone for greater_311 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_311 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_312 {}
impl Clone for less_312 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_312 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_equal_313 {}
impl Clone for greater_equal_313 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_313 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_equal_314 {}
impl Clone for less_equal_314 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_314 {
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
    pub static allocator_arg_315: Value<std_allocator_arg_t> = unimplemented!();
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
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<vector_284> = Rc::new(RefCell::new(<vector_284>::default()));
    ({ vector_284Impl::resize_40(&v.as_pointer(), 2_u64) });
    ({
        vector_250Impl::resize_40(
            &({ vector_284Impl::operator_index_46(&v.as_pointer(), 0_u64) }),
            2_u64,
        )
    });
    ({
        vector_250Impl::resize_40(
            &({ vector_284Impl::operator_index_46(&v.as_pointer(), 1_u64) }),
            1_u64,
        )
    });
    ({
        vector_250Impl::operator_index_46(
            &({ vector_284Impl::operator_index_46(&v.as_pointer(), 0_u64) }),
            0_u64,
        )
    })
    .write(1);
    ({
        vector_250Impl::operator_index_46(
            &({ vector_284Impl::operator_index_46(&v.as_pointer(), 0_u64) }),
            1_u64,
        )
    })
    .write(5);
    ({
        vector_250Impl::operator_index_46(
            &({ vector_284Impl::operator_index_46(&v.as_pointer(), 1_u64) }),
            0_u64,
        )
    })
    .write(6);
    {
        let __range1: Ptr<vector_284> = v.as_pointer();
        let __begin1: Value<__normal_iterator_249> =
            Rc::new(RefCell::new(({ vector_284Impl::begin_26(&__range1) })));
        let __end1: Value<__normal_iterator_249> =
            Rc::new(RefCell::new(({ vector_284Impl::end_28(&__range1) })));
        'loop_: while ({
            let ___lhs: Ptr<__normal_iterator_249> = __begin1.as_pointer();
            operator_ne_316(___lhs, __end1.as_pointer())
        }) {
            let v2: Ptr<vector_250> =
                ({ __normal_iterator_249Impl::operator_deref(&__begin1.as_pointer()) });
            {
                let __range2: Ptr<vector_250> = (v2).clone();
                let __begin2: Value<__normal_iterator_251> =
                    Rc::new(RefCell::new(({ vector_250Impl::begin_26(&__range2) })));
                let __end2: Value<__normal_iterator_251> =
                    Rc::new(RefCell::new(({ vector_250Impl::end_28(&__range2) })));
                'loop_: while ({
                    let ___lhs: Ptr<__normal_iterator_251> = __begin2.as_pointer();
                    operator_ne_317(___lhs, __end2.as_pointer())
                }) {
                    let i: Ptr<i32> =
                        ({ __normal_iterator_251Impl::operator_deref(&__begin2.as_pointer()) });
                    i.with_mut(|__v| __v.prefix_inc());
                    ({ __normal_iterator_251Impl::operator_inc_6(&__begin2.as_pointer()) });
                }
            }
            ({ __normal_iterator_249Impl::operator_inc_6(&__begin1.as_pointer()) });
        }
    }
    {
        let __range1: Ptr<vector_284> = v.as_pointer();
        let __begin1: Value<__normal_iterator_249> =
            Rc::new(RefCell::new(({ vector_284Impl::begin_26(&__range1) })));
        let __end1: Value<__normal_iterator_249> =
            Rc::new(RefCell::new(({ vector_284Impl::end_28(&__range1) })));
        'loop_: while ({
            let ___lhs: Ptr<__normal_iterator_249> = __begin1.as_pointer();
            operator_ne_316(___lhs, __end1.as_pointer())
        }) {
            let v2: Ptr<vector_250> =
                ({ __normal_iterator_249Impl::operator_deref(&__begin1.as_pointer()) });
            {
                let __range2: Ptr<vector_250> = (v2).clone();
                let __begin2: Value<__normal_iterator_251> =
                    Rc::new(RefCell::new(({ vector_250Impl::begin_26(&__range2) })));
                let __end2: Value<__normal_iterator_251> =
                    Rc::new(RefCell::new(({ vector_250Impl::end_28(&__range2) })));
                'loop_: while ({
                    let ___lhs: Ptr<__normal_iterator_251> = __begin2.as_pointer();
                    operator_ne_317(___lhs, __end2.as_pointer())
                }) {
                    let mut i: i32 =
                        (({ __normal_iterator_251Impl::operator_deref(&__begin2.as_pointer()) })
                            .read());
                    println!("{}", (i + 3));
                    ({ __normal_iterator_251Impl::operator_inc_6(&__begin2.as_pointer()) });
                }
            }
            ({ __normal_iterator_249Impl::operator_inc_6(&__begin1.as_pointer()) });
        }
    }
    {
        let __range1: Ptr<vector_284> = v.as_pointer();
        let __begin1: Value<__normal_iterator_249> =
            Rc::new(RefCell::new(({ vector_284Impl::begin_26(&__range1) })));
        let __end1: Value<__normal_iterator_249> =
            Rc::new(RefCell::new(({ vector_284Impl::end_28(&__range1) })));
        'loop_: while ({
            let ___lhs: Ptr<__normal_iterator_249> = __begin1.as_pointer();
            operator_ne_316(___lhs, __end1.as_pointer())
        }) {
            let v2: Value<vector_250> = Rc::new(RefCell::new(vector_250::copy_from({
                ({ __normal_iterator_249Impl::operator_deref(&__begin1.as_pointer()) })
            })));
            {
                let __range2: Ptr<vector_250> = v2.as_pointer();
                let __begin2: Value<__normal_iterator_251> =
                    Rc::new(RefCell::new(({ vector_250Impl::begin_26(&__range2) })));
                let __end2: Value<__normal_iterator_251> =
                    Rc::new(RefCell::new(({ vector_250Impl::end_28(&__range2) })));
                'loop_: while ({
                    let ___lhs: Ptr<__normal_iterator_251> = __begin2.as_pointer();
                    operator_ne_317(___lhs, __end2.as_pointer())
                }) {
                    let mut i: i32 =
                        (({ __normal_iterator_251Impl::operator_deref(&__begin2.as_pointer()) })
                            .read());
                    println!("{}", i);
                    ({ __normal_iterator_251Impl::operator_inc_6(&__begin2.as_pointer()) });
                }
            }
            ({ __normal_iterator_249Impl::operator_inc_6(&__begin1.as_pointer()) });
        }
    }
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_wide_data;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct std___uses_alloc0__Sink;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_marker;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_codecvt;
pub fn operator_ne_316(
    __lhs: Ptr<__normal_iterator_249>,
    __rhs: Ptr<__normal_iterator_249>,
) -> bool {
    unimplemented!()
}
pub fn operator_ne_317(
    __lhs: Ptr<__normal_iterator_251>,
    __rhs: Ptr<__normal_iterator_251>,
) -> bool {
    unimplemented!()
}
pub trait _UninitDestroyGuard_273Impl {
    fn release(&self) {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_273Impl for Ptr<_UninitDestroyGuard_273> {}
pub trait _Vector_base_282Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_261> {
        unimplemented!()
    }
    fn _M_get_Tp_allocator_2(&self) -> Ptr<allocator_261> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<vector_250> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<vector_250>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_282Impl for Ptr<_Vector_base_282> {}
pub trait _Vector_base_283Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_260> {
        unimplemented!()
    }
    fn _M_get_Tp_allocator_2(&self) -> Ptr<allocator_260> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<i32> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<i32>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_283Impl for Ptr<_Vector_base_283> {}
pub trait __new_allocator_255Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<vector_250> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<vector_250>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn construct_256(&self, mut __p: Ptr<vector_250>, __args: Ptr<vector_250>) {
        unimplemented!()
    }
    fn destroy_257(&self, mut __p: Ptr<vector_250>) {
        unimplemented!()
    }
}
impl __new_allocator_255Impl for Ptr<__new_allocator_255> {}
pub trait __new_allocator_258Impl {
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
impl __new_allocator_258Impl for Ptr<__new_allocator_258> {}
pub trait __normal_iterator_249Impl {
    fn operator_deref(&self) -> Ptr<vector_250> {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<__normal_iterator_249> {
        unimplemented!()
    }
    fn base(&self) -> Ptr<Ptr<vector_250>> {
        unimplemented!()
    }
}
impl __normal_iterator_249Impl for Ptr<__normal_iterator_249> {}
pub trait __normal_iterator_251Impl {
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<__normal_iterator_251> {
        unimplemented!()
    }
    fn base(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl __normal_iterator_251Impl for Ptr<__normal_iterator_251> {}
pub trait __normal_iterator_252Impl {
    fn base(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl __normal_iterator_252Impl for Ptr<__normal_iterator_252> {}
pub trait initializer_list_280Impl {
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn begin(&self) -> Ptr<bool> {
        unimplemented!()
    }
}
impl initializer_list_280Impl for Ptr<initializer_list_280> {}
pub trait integral_constant_1Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl integral_constant_1Impl for Ptr<integral_constant_1> {}
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
pub trait vector_250Impl {
    fn copy_assign(&self, __x: Ptr<vector_250>) -> Ptr<vector_250> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_250>) -> Ptr<vector_250> {
        unimplemented!()
    }
    fn begin_26(&self) -> __normal_iterator_251 {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_252 {
        unimplemented!()
    }
    fn end_28(&self) -> __normal_iterator_251 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_252 {
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
    fn operator_index_46(&self, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
}
impl vector_250Impl for Ptr<vector_250> {}
pub trait vector_284Impl {
    fn copy_assign(&self, __x: Ptr<vector_284>) -> Ptr<vector_284> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_284>) -> Ptr<vector_284> {
        unimplemented!()
    }
    fn begin_26(&self) -> __normal_iterator_249 {
        unimplemented!()
    }
    fn end_28(&self) -> __normal_iterator_249 {
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
    fn operator_index_46(&self, mut __n: u64) -> Ptr<vector_250> {
        unimplemented!()
    }
}
impl vector_284Impl for Ptr<vector_284> {}
pub fn __cpp2rust_init_globals() {}
