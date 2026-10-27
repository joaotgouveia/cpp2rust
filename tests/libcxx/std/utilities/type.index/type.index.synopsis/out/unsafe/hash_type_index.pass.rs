extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool__float__double__long_double___types_type_list_Enum__EnumClass__void_ptr__const_void_ptr__Class_ptr__std_nullptr_t__
{}
pub unsafe fn can_hash_impl_0(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_1(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_2(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_3(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_4(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_5(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_6(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_7(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_8(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_9(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_10(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_11(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_12(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_13() -> () {
    return (unsafe { can_hash_impl_0(0_()) });
}
pub unsafe fn can_hash_14() -> () {
    return (unsafe { can_hash_impl_1(0_()) });
}
pub unsafe fn can_hash_15() -> () {
    return (unsafe { can_hash_impl_2(0_()) });
}
pub unsafe fn can_hash_16() -> () {
    return (unsafe { can_hash_impl_3(0_()) });
}
pub unsafe fn can_hash_17() -> () {
    return (unsafe { can_hash_impl_4(0_()) });
}
pub unsafe fn can_hash_18() -> () {
    return (unsafe { can_hash_impl_5(0_()) });
}
pub unsafe fn can_hash_19() -> () {
    return (unsafe { can_hash_impl_6(0_()) });
}
pub unsafe fn can_hash_20() -> () {
    return (unsafe { can_hash_impl_7(0_()) });
}
pub unsafe fn can_hash_21() -> () {
    return (unsafe { can_hash_impl_8(0_()) });
}
pub unsafe fn can_hash_22() -> () {
    return (unsafe { can_hash_impl_9(0_()) });
}
pub unsafe fn can_hash_23() -> () {
    return (unsafe { can_hash_impl_10(0_()) });
}
pub unsafe fn can_hash_24() -> () {
    return (unsafe { can_hash_impl_11(0_()) });
}
pub unsafe fn can_hash_25() -> () {
    return (unsafe { can_hash_impl_12(0_()) });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_std_type_index_ {}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_std_type_index_ {
    pub to: (),
}
impl Default for ConvertibleTo_std_type_index_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_std_type_index_ {
                to: unimplemented!(),
            }
        }
    }
}
pub unsafe fn test_hash_enabled_26(key: *const ()) {
    const _: () = assert!(true, "std::is_destructible<Hash>::value");;
    const _: () = assert!(true, "std::is_default_constructible<Hash>::value");;
    const _: () = assert!(true, "std::is_copy_constructible<Hash>::value");;
    const _: () = assert!(true, "std::is_move_constructible<Hash>::value");;
    const _: () = assert!(true, "std::is_copy_assignable<Hash>::value");;
    const _: () = assert!(true, "std::is_move_assignable<Hash>::value");;
    const _: () = assert!(true, "std::is_swappable<Hash>::value");;
    const _: () = assert!(true, "can_hash<Hash, Key&>()");;
    const _: () = assert!(true, "can_hash<Hash, Key const&>()");;
    const _: () = assert!(true, "can_hash<Hash, Key&&>()");;
    const _: () = assert!(true, "can_hash<Hash const, Key&>()");;
    const _: () = assert!(true, "can_hash<Hash const, Key const&>()");;
    const _: () = assert!(true, "can_hash<Hash const, Key&&>()");;
    const _: () = assert!(true, "can_hash<Hash, ConvertibleToSimple<Key>&>()");;
    const _: () = assert!(true, "can_hash<Hash, ConvertibleToSimple<Key> const&>()");;
    const _: () = assert!(true, "can_hash<Hash, ConvertibleToSimple<Key>&&>()");;
    const _: () = assert!(true, "can_hash<Hash, ConvertibleTo<Key>&>()");;
    const _: () = assert!(true, "can_hash<Hash, ConvertibleTo<Key> const&>()");;
    const _: () = assert!(true, "can_hash<Hash, ConvertibleTo<Key>&&>()");;
    const _: () = assert!(true, "can_hash<Hash, ConvertibleTo<Key> const&&>()");;
    let h: () = unimplemented!();
    unimplemented!();
}
pub type Enum = ();
pub type EnumClass = ();
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Class {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestHashEnabled {}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    {}
    {
        (unsafe {
            let mut _key: () = unimplemented!();
            test_hash_enabled_26(&mut _key)
        });
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
