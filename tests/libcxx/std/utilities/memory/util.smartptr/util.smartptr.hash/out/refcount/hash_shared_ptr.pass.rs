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
pub fn can_hash_impl_0(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_1(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_2(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_3(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_4(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_5(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_6(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_7(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_8(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_9(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_10(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_11(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_12(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_13(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_14(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_15(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_16(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_17(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_18(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_19(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_20(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_21(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_22(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_23(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_24(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_25(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_26() -> bool {
    return ({ can_hash_impl_0(0) });
}
pub fn can_hash_27() -> bool {
    return ({ can_hash_impl_1(0) });
}
pub fn can_hash_28() -> bool {
    return ({ can_hash_impl_2(0) });
}
pub fn can_hash_29() -> bool {
    return ({ can_hash_impl_3(0) });
}
pub fn can_hash_30() -> bool {
    return ({ can_hash_impl_4(0) });
}
pub fn can_hash_31() -> bool {
    return ({ can_hash_impl_5(0) });
}
pub fn can_hash_32() -> bool {
    return ({ can_hash_impl_6(0) });
}
pub fn can_hash_33() -> bool {
    return ({ can_hash_impl_7(0) });
}
pub fn can_hash_34() -> bool {
    return ({ can_hash_impl_8(0) });
}
pub fn can_hash_35() -> bool {
    return ({ can_hash_impl_9(0) });
}
pub fn can_hash_36() -> bool {
    return ({ can_hash_impl_10(0) });
}
pub fn can_hash_37() -> bool {
    return ({ can_hash_impl_11(0) });
}
pub fn can_hash_38() -> bool {
    return ({ can_hash_impl_12(0) });
}
pub fn can_hash_39() -> bool {
    return ({ can_hash_impl_13(0) });
}
pub fn can_hash_40() -> bool {
    return ({ can_hash_impl_14(0) });
}
pub fn can_hash_41() -> bool {
    return ({ can_hash_impl_15(0) });
}
pub fn can_hash_42() -> bool {
    return ({ can_hash_impl_16(0) });
}
pub fn can_hash_43() -> bool {
    return ({ can_hash_impl_17(0) });
}
pub fn can_hash_44() -> bool {
    return ({ can_hash_impl_18(0) });
}
pub fn can_hash_45() -> bool {
    return ({ can_hash_impl_19(0) });
}
pub fn can_hash_46() -> bool {
    return ({ can_hash_impl_20(0) });
}
pub fn can_hash_47() -> bool {
    return ({ can_hash_impl_21(0) });
}
pub fn can_hash_48() -> bool {
    return ({ can_hash_impl_22(0) });
}
pub fn can_hash_49() -> bool {
    return ({ can_hash_impl_23(0) });
}
pub fn can_hash_50() -> bool {
    return ({ can_hash_impl_24(0) });
}
pub fn can_hash_51() -> bool {
    return ({ can_hash_impl_25(0) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_std_shared_ptr_int__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_std_shared_ptr_A__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(16)]
pub struct ConvertibleTo_std_shared_ptr_int__ {
    #[offset(0)]
    #[byte_size(16)]
    pub to: (),
}
impl ConvertibleTo_std_shared_ptr_int__ {
    pub fn move_from(_a0: Ptr<ConvertibleTo_std_shared_ptr_int__>) -> Self {
        Self {
            to: unimplemented!(),
        }
    }
}
impl Default for ConvertibleTo_std_shared_ptr_int__ {
    fn default() -> Self {
        {
            ConvertibleTo_std_shared_ptr_int__ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(16)]
pub struct ConvertibleTo_std_shared_ptr_A__ {
    #[offset(0)]
    #[byte_size(16)]
    pub to: (),
}
impl ConvertibleTo_std_shared_ptr_A__ {
    pub fn move_from(_a0: Ptr<ConvertibleTo_std_shared_ptr_A__>) -> Self {
        Self {
            to: unimplemented!(),
        }
    }
}
impl Default for ConvertibleTo_std_shared_ptr_A__ {
    fn default() -> Self {
        {
            ConvertibleTo_std_shared_ptr_A__ {
                to: unimplemented!(),
            }
        }
    }
}
pub fn test_hash_enabled_52(key: Option<Ptr<()>>) {
    let mut __key_default: Option<Value<()>> = None;
    let key: Ptr<()> = key.unwrap_or_else(|| {
        __key_default
            .insert(Rc::new(RefCell::new(unimplemented!())))
            .as_pointer()
    });
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
    assert!((unimplemented!() == unimplemented!()));
}
pub fn test_hash_enabled_53(key: Option<Ptr<()>>) {
    let mut __key_default: Option<Value<()>> = None;
    let key: Ptr<()> = key.unwrap_or_else(|| {
        __key_default
            .insert(Rc::new(RefCell::new(unimplemented!())))
            .as_pointer()
    });
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
    assert!((unimplemented!() == unimplemented!()));
}
pub type Enum = u32;
pub type EnumClass = bool;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Class {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestHashEnabled {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A {}
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    {
        let mut ptr: Ptr<i32> = Ptr::alloc(Default::default());
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut h: usize = unimplemented!();
        assert!(({ h } == { unimplemented!() }));
    }
    {
        ({ test_hash_enabled_52(None) });
        ({ test_hash_enabled_53(None) });
    }
    return 0;
}
pub trait ConvertibleToSimple_std_shared_ptr_A__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_std_shared_ptr_A__Impl for Ptr<ConvertibleToSimple_std_shared_ptr_A__> {}
pub trait ConvertibleToSimple_std_shared_ptr_int__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_std_shared_ptr_int__Impl
    for Ptr<ConvertibleToSimple_std_shared_ptr_int__>
{
}
pub trait ConvertibleTo_std_shared_ptr_A__Impl {
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
    fn move_assign(
        &self,
        _a0: Ptr<ConvertibleTo_std_shared_ptr_A__>,
    ) -> Ptr<ConvertibleTo_std_shared_ptr_A__>;
}
impl ConvertibleTo_std_shared_ptr_A__Impl for Ptr<ConvertibleTo_std_shared_ptr_A__> {
    fn move_assign(
        &self,
        _a0: Ptr<ConvertibleTo_std_shared_ptr_A__>,
    ) -> Ptr<ConvertibleTo_std_shared_ptr_A__> {
        unimplemented!();
        return (*self).clone();
    }
}
pub trait ConvertibleTo_std_shared_ptr_int__Impl {
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
    fn move_assign(
        &self,
        _a0: Ptr<ConvertibleTo_std_shared_ptr_int__>,
    ) -> Ptr<ConvertibleTo_std_shared_ptr_int__>;
}
impl ConvertibleTo_std_shared_ptr_int__Impl for Ptr<ConvertibleTo_std_shared_ptr_int__> {
    fn move_assign(
        &self,
        _a0: Ptr<ConvertibleTo_std_shared_ptr_int__>,
    ) -> Ptr<ConvertibleTo_std_shared_ptr_int__> {
        unimplemented!();
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
