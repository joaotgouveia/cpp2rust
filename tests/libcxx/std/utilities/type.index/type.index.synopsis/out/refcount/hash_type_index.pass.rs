extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool__float__double__long_double___types_type_list_Enum__EnumClass__void_ptr__const_void_ptr__Class_ptr__std_nullptr_t__
{}
pub fn can_hash_impl_0(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_1(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_2(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_3(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_4(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_5(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_6(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_7(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_8(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_9(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_10(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_11(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_12(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_13() -> () {
    return ({ can_hash_impl_0(0_()) });
}
pub fn can_hash_14() -> () {
    return ({ can_hash_impl_1(0_()) });
}
pub fn can_hash_15() -> () {
    return ({ can_hash_impl_2(0_()) });
}
pub fn can_hash_16() -> () {
    return ({ can_hash_impl_3(0_()) });
}
pub fn can_hash_17() -> () {
    return ({ can_hash_impl_4(0_()) });
}
pub fn can_hash_18() -> () {
    return ({ can_hash_impl_5(0_()) });
}
pub fn can_hash_19() -> () {
    return ({ can_hash_impl_6(0_()) });
}
pub fn can_hash_20() -> () {
    return ({ can_hash_impl_7(0_()) });
}
pub fn can_hash_21() -> () {
    return ({ can_hash_impl_8(0_()) });
}
pub fn can_hash_22() -> () {
    return ({ can_hash_impl_9(0_()) });
}
pub fn can_hash_23() -> () {
    return ({ can_hash_impl_10(0_()) });
}
pub fn can_hash_24() -> () {
    return ({ can_hash_impl_11(0_()) });
}
pub fn can_hash_25() -> () {
    return ({ can_hash_impl_12(0_()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_std_type_index_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct ConvertibleTo_std_type_index_ {
    #[offset(0)]
    #[byte_size(8)]
    pub to: (),
}
impl Default for ConvertibleTo_std_type_index_ {
    fn default() -> Self {
        {
            ConvertibleTo_std_type_index_ {
                to: unimplemented!(),
            }
        }
    }
}
pub fn test_hash_enabled_26(key: Ptr<()>) {
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
    let h: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub type Enum = ();
pub type EnumClass = ();
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Class {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestHashEnabled {}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    {}
    {
        ({
            let _key: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_hash_enabled_26(_key.as_pointer())
        });
    }
    return 0_();
}
pub trait ConvertibleToSimple_std_type_index_Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_std_type_index_Impl for Ptr<ConvertibleToSimple_std_type_index_> {}
pub trait ConvertibleTo_std_type_index_Impl {
    fn to_pmut_1(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn to_pconst_2(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn to_pmut_3(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn to_pconst_4(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl ConvertibleTo_std_type_index_Impl for Ptr<ConvertibleTo_std_type_index_> {}
pub fn __cpp2rust_init_globals() {}
