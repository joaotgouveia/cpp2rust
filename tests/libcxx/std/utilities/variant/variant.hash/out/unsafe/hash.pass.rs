extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
pub unsafe fn operator_eq_1(LHS: *const TypeID, RHS: *const TypeID) -> () {
    return (((*LHS).m_id) == ((*RHS).m_id));
}
pub unsafe fn operator_ne_2(LHS: *const TypeID, RHS: *const TypeID) -> () {
    return (((*LHS).m_id) != ((*RHS).m_id));
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct TypeID {
    m_id: *const (),
}
impl TypeID {
    pub unsafe fn name(&self) -> () {
        return unimplemented!();
    }
    unsafe fn new(mut xid: *const ()) -> Self {
        let mut this = Self { m_id: xid };
        this
    }
}
impl std::cmp::PartialEq for TypeID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_1(self as *const TypeID, other as *const TypeID) }
    }
}
impl std::cmp::Eq for TypeID {}
pub type CallType = ();
pub const CallType_CT_None: CallType = 0;
pub const CallType_CT_NonConst: CallType = 1;
pub const CallType_CT_Const: CallType = 2;
pub const CallType_CT_LValue: CallType = 4;
pub const CallType_CT_RValue: CallType = 8;
pub unsafe fn operator_bitor_3(mut LHS: CallType, mut RHS: CallType) -> CallType {
    return (((LHS as ()) | (RHS as ())) as CallType);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ForwardingCallObject {}
impl ForwardingCallObject {
    pub unsafe fn to_(&self) -> () {
        return 0_();
    }
}
pub static mut last_call_type_4: std::cell::LazyCell<CallType> =
    std::cell::LazyCell::new(|| unsafe { CallType_CT_None });
pub static mut last_call_args_5: std::cell::LazyCell<*const TypeID> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnFirst {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnArity {}
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
pub unsafe fn can_hash_impl_13(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_14(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_15(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_16(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_17(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_18(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_19(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_20(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_21(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_22(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_23(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_24(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_25(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_26(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_27(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_28(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_29(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_30(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_31(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_32(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_33(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_34(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_35(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_36(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_37(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_38(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_39(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_40(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_41(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_42(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_43(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_44(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_45(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_46(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_47(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_48(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_49(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_50(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_51(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_52(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_53(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_54(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_55(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_56(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_57(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_58(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_59(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_60(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_61(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_62(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_63(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_64(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_65(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_66(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_67(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_68(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_69(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_70(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_71(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn can_hash_impl_72(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn can_hash_impl_73(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn can_hash_impl_74(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn can_hash_impl_75(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn can_hash_impl_76(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn can_hash_impl_77(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn can_hash_impl_78(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn can_hash_impl_79(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn can_hash_impl_80(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn can_hash_impl_81(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn can_hash_impl_82(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn can_hash_impl_83(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn can_hash_impl_84(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn can_hash_impl_85(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn can_hash_impl_86(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn can_hash_impl_87(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn can_hash_impl_88(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn can_hash_impl_89(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn can_hash_impl_90(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn can_hash_impl_91(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn can_hash_impl_92(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn can_hash_impl_93(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn can_hash_impl_94(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn can_hash_impl_95(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn can_hash_impl_96(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn can_hash_97() -> () {
    return (unsafe { can_hash_impl_6(0_()) });
}
pub unsafe fn can_hash_98() -> () {
    return (unsafe { can_hash_impl_7(0_()) });
}
pub unsafe fn can_hash_99() -> () {
    return (unsafe { can_hash_impl_8(0_()) });
}
pub unsafe fn can_hash_100() -> () {
    return (unsafe { can_hash_impl_9(0_()) });
}
pub unsafe fn can_hash_101() -> () {
    return (unsafe { can_hash_impl_10(0_()) });
}
pub unsafe fn can_hash_102() -> () {
    return (unsafe { can_hash_impl_11(0_()) });
}
pub unsafe fn can_hash_103() -> () {
    return (unsafe { can_hash_impl_12(0_()) });
}
pub unsafe fn can_hash_104() -> () {
    return (unsafe { can_hash_impl_13(0_()) });
}
pub unsafe fn can_hash_105() -> () {
    return (unsafe { can_hash_impl_14(0_()) });
}
pub unsafe fn can_hash_106() -> () {
    return (unsafe { can_hash_impl_15(0_()) });
}
pub unsafe fn can_hash_107() -> () {
    return (unsafe { can_hash_impl_16(0_()) });
}
pub unsafe fn can_hash_108() -> () {
    return (unsafe { can_hash_impl_17(0_()) });
}
pub unsafe fn can_hash_109() -> () {
    return (unsafe { can_hash_impl_18(0_()) });
}
pub unsafe fn can_hash_110() -> () {
    return (unsafe { can_hash_impl_19(0_()) });
}
pub unsafe fn can_hash_111() -> () {
    return (unsafe { can_hash_impl_20(0_()) });
}
pub unsafe fn can_hash_112() -> () {
    return (unsafe { can_hash_impl_21(0_()) });
}
pub unsafe fn can_hash_113() -> () {
    return (unsafe { can_hash_impl_22(0_()) });
}
pub unsafe fn can_hash_114() -> () {
    return (unsafe { can_hash_impl_23(0_()) });
}
pub unsafe fn can_hash_115() -> () {
    return (unsafe { can_hash_impl_24(0_()) });
}
pub unsafe fn can_hash_116() -> () {
    return (unsafe { can_hash_impl_25(0_()) });
}
pub unsafe fn can_hash_117() -> () {
    return (unsafe { can_hash_impl_26(0_()) });
}
pub unsafe fn can_hash_118() -> () {
    return (unsafe { can_hash_impl_27(0_()) });
}
pub unsafe fn can_hash_119() -> () {
    return (unsafe { can_hash_impl_28(0_()) });
}
pub unsafe fn can_hash_120() -> () {
    return (unsafe { can_hash_impl_29(0_()) });
}
pub unsafe fn can_hash_121() -> () {
    return (unsafe { can_hash_impl_30(0_()) });
}
pub unsafe fn can_hash_122() -> () {
    return (unsafe { can_hash_impl_31(0_()) });
}
pub unsafe fn can_hash_123() -> () {
    return (unsafe { can_hash_impl_32(0_()) });
}
pub unsafe fn can_hash_124() -> () {
    return (unsafe { can_hash_impl_33(0_()) });
}
pub unsafe fn can_hash_125() -> () {
    return (unsafe { can_hash_impl_34(0_()) });
}
pub unsafe fn can_hash_126() -> () {
    return (unsafe { can_hash_impl_35(0_()) });
}
pub unsafe fn can_hash_127() -> () {
    return (unsafe { can_hash_impl_36(0_()) });
}
pub unsafe fn can_hash_128() -> () {
    return (unsafe { can_hash_impl_37(0_()) });
}
pub unsafe fn can_hash_129() -> () {
    return (unsafe { can_hash_impl_38(0_()) });
}
pub unsafe fn can_hash_130() -> () {
    return (unsafe { can_hash_impl_39(0_()) });
}
pub unsafe fn can_hash_131() -> () {
    return (unsafe { can_hash_impl_40(0_()) });
}
pub unsafe fn can_hash_132() -> () {
    return (unsafe { can_hash_impl_41(0_()) });
}
pub unsafe fn can_hash_133() -> () {
    return (unsafe { can_hash_impl_42(0_()) });
}
pub unsafe fn can_hash_134() -> () {
    return (unsafe { can_hash_impl_43(0_()) });
}
pub unsafe fn can_hash_135() -> () {
    return (unsafe { can_hash_impl_44(0_()) });
}
pub unsafe fn can_hash_136() -> () {
    return (unsafe { can_hash_impl_45(0_()) });
}
pub unsafe fn can_hash_137() -> () {
    return (unsafe { can_hash_impl_46(0_()) });
}
pub unsafe fn can_hash_138() -> () {
    return (unsafe { can_hash_impl_47(0_()) });
}
pub unsafe fn can_hash_139() -> () {
    return (unsafe { can_hash_impl_48(0_()) });
}
pub unsafe fn can_hash_140() -> () {
    return (unsafe { can_hash_impl_49(0_()) });
}
pub unsafe fn can_hash_141() -> () {
    return (unsafe { can_hash_impl_50(0_()) });
}
pub unsafe fn can_hash_142() -> () {
    return (unsafe { can_hash_impl_51(0_()) });
}
pub unsafe fn can_hash_143() -> () {
    return (unsafe { can_hash_impl_52(0_()) });
}
pub unsafe fn can_hash_144() -> () {
    return (unsafe { can_hash_impl_53(0_()) });
}
pub unsafe fn can_hash_145() -> () {
    return (unsafe { can_hash_impl_54(0_()) });
}
pub unsafe fn can_hash_146() -> () {
    return (unsafe { can_hash_impl_55(0_()) });
}
pub unsafe fn can_hash_147() -> () {
    return (unsafe { can_hash_impl_56(0_()) });
}
pub unsafe fn can_hash_148() -> () {
    return (unsafe { can_hash_impl_57(0_()) });
}
pub unsafe fn can_hash_149() -> () {
    return (unsafe { can_hash_impl_58(0_()) });
}
pub unsafe fn can_hash_150() -> () {
    return (unsafe { can_hash_impl_59(0_()) });
}
pub unsafe fn can_hash_151() -> () {
    return (unsafe { can_hash_impl_60(0_()) });
}
pub unsafe fn can_hash_152() -> () {
    return (unsafe { can_hash_impl_61(0_()) });
}
pub unsafe fn can_hash_153() -> () {
    return (unsafe { can_hash_impl_62(0_()) });
}
pub unsafe fn can_hash_154() -> () {
    return (unsafe { can_hash_impl_63(0_()) });
}
pub unsafe fn can_hash_155() -> () {
    return (unsafe { can_hash_impl_64(0_()) });
}
pub unsafe fn can_hash_156() -> () {
    return (unsafe { can_hash_impl_65(0_()) });
}
pub unsafe fn can_hash_157() -> () {
    return (unsafe { can_hash_impl_66(0_()) });
}
pub unsafe fn can_hash_158() -> () {
    return (unsafe { can_hash_impl_67(0_()) });
}
pub unsafe fn can_hash_159() -> () {
    return (unsafe { can_hash_impl_68(0_()) });
}
pub unsafe fn can_hash_160() -> () {
    return (unsafe { can_hash_impl_69(0_()) });
}
pub unsafe fn can_hash_161() -> () {
    return (unsafe { can_hash_impl_70(0_()) });
}
pub unsafe fn can_hash_162() -> () {
    return (unsafe { can_hash_impl_71(0_()) });
}
pub unsafe fn can_hash_163() -> () {
    return (unsafe { can_hash_impl_72(0_()) });
}
pub unsafe fn can_hash_164() -> () {
    return (unsafe { can_hash_impl_73(0_()) });
}
pub unsafe fn can_hash_165() -> () {
    return (unsafe { can_hash_impl_74(0_()) });
}
pub unsafe fn can_hash_166() -> () {
    return (unsafe { can_hash_impl_75(0_()) });
}
pub unsafe fn can_hash_167() -> () {
    return (unsafe { can_hash_impl_76(0_()) });
}
pub unsafe fn can_hash_168() -> () {
    return (unsafe { can_hash_impl_77(0_()) });
}
pub unsafe fn can_hash_169() -> () {
    return (unsafe { can_hash_impl_78(0_()) });
}
pub unsafe fn can_hash_170() -> () {
    return (unsafe { can_hash_impl_79(0_()) });
}
pub unsafe fn can_hash_171() -> () {
    return (unsafe { can_hash_impl_80(0_()) });
}
pub unsafe fn can_hash_172() -> () {
    return (unsafe { can_hash_impl_81(0_()) });
}
pub unsafe fn can_hash_173() -> () {
    return (unsafe { can_hash_impl_82(0_()) });
}
pub unsafe fn can_hash_174() -> () {
    return (unsafe { can_hash_impl_83(0_()) });
}
pub unsafe fn can_hash_175() -> () {
    return (unsafe { can_hash_impl_84(0_()) });
}
pub unsafe fn can_hash_176() -> () {
    return (unsafe { can_hash_impl_85(0_()) });
}
pub unsafe fn can_hash_177() -> () {
    return (unsafe { can_hash_impl_86(0_()) });
}
pub unsafe fn can_hash_178() -> () {
    return (unsafe { can_hash_impl_87(0_()) });
}
pub unsafe fn can_hash_179() -> () {
    return (unsafe { can_hash_impl_88(0_()) });
}
pub unsafe fn can_hash_180() -> () {
    return (unsafe { can_hash_impl_89(0_()) });
}
pub unsafe fn can_hash_181() -> () {
    return (unsafe { can_hash_impl_90(0_()) });
}
pub unsafe fn can_hash_182() -> () {
    return (unsafe { can_hash_impl_91(0_()) });
}
pub unsafe fn can_hash_183() -> () {
    return (unsafe { can_hash_impl_92(0_()) });
}
pub unsafe fn can_hash_184() -> () {
    return (unsafe { can_hash_impl_93(0_()) });
}
pub unsafe fn can_hash_185() -> () {
    return (unsafe { can_hash_impl_94(0_()) });
}
pub unsafe fn can_hash_186() -> () {
    return (unsafe { can_hash_impl_95(0_()) });
}
pub unsafe fn can_hash_187() -> () {
    return (unsafe { can_hash_impl_96(0_()) });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_std_monostate_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_std_variant_int__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_std_variant_int_ptr__long__double__const_int__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_std_variant_int__B__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_std_variant_const_B__int__ {}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_std_monostate_ {
    pub to: (),
}
impl Default for ConvertibleTo_std_monostate_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_std_monostate_ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_std_variant_int__ {
    pub to: (),
}
impl Default for ConvertibleTo_std_variant_int__ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_std_variant_int__ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_std_variant_int_ptr__long__double__const_int__ {
    pub to: (),
}
impl Default for ConvertibleTo_std_variant_int_ptr__long__double__const_int__ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_std_variant_int_ptr__long__double__const_int__ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_std_variant_int__B__ {
    pub to: (),
}
impl Default for ConvertibleTo_std_variant_int__B__ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_std_variant_int__B__ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_std_variant_const_B__int__ {
    pub to: (),
}
impl Default for ConvertibleTo_std_variant_const_B__int__ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_std_variant_const_B__int__ {
                to: unimplemented!(),
            }
        }
    }
}
pub unsafe fn test_hash_enabled_188(key: Option<*const ()>) {
    let mut __key_default: Option<()> = None;
    let mut key: *const () =
        key.unwrap_or_else(|| unsafe { __key_default.insert(unimplemented!()) as *const () });
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
pub unsafe fn test_hash_enabled_189(key: Option<*const ()>) {
    let mut __key_default: Option<()> = None;
    let mut key: *const () =
        key.unwrap_or_else(|| unsafe { __key_default.insert(unimplemented!()) as *const () });
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
pub unsafe fn test_hash_enabled_190(key: Option<*const ()>) {
    let mut __key_default: Option<()> = None;
    let mut key: *const () =
        key.unwrap_or_else(|| unsafe { __key_default.insert(unimplemented!()) as *const () });
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
pub unsafe fn test_hash_enabled_191(key: Option<*const ()>) {
    let mut __key_default: Option<()> = None;
    let mut key: *const () =
        key.unwrap_or_else(|| unsafe { __key_default.insert(unimplemented!()) as *const () });
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
pub unsafe fn test_hash_enabled_192(key: Option<*const ()>) {
    let mut __key_default: Option<()> = None;
    let mut key: *const () =
        key.unwrap_or_else(|| unsafe { __key_default.insert(unimplemented!()) as *const () });
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
pub unsafe fn test_hash_disabled_193() {
    const _: () = assert!(true, "!std::is_default_constructible<Hash>::value");;
    const _: () = assert!(true, "!std::is_copy_constructible<Hash>::value");;
    const _: () = assert!(true, "!std::is_move_constructible<Hash>::value");;
    const _: () = assert!(true, "!std::is_copy_assignable<Hash>::value");;
    const _: () = assert!(true, "!std::is_move_assignable<Hash>::value");;
    const _: () = assert!(
        true,
        "!std::is_function<typename std::remove_pointer<typename std::remove_reference<Hash>::type>::type>::value"
    );;
    const _: () = assert!(true, "!can_hash<Hash, Key&>()");;
    const _: () = assert!(true, "!can_hash<Hash, Key const&>()");;
    const _: () = assert!(true, "!can_hash<Hash, Key&&>()");;
    const _: () = assert!(true, "!can_hash<Hash const, Key&>()");;
    const _: () = assert!(true, "!can_hash<Hash const, Key const&>()");;
    const _: () = assert!(true, "!can_hash<Hash const, Key&&>()");;
    const _: () = assert!(true, "!can_hash<Hash, ConvertibleToSimple<Key>&>()");;
    const _: () = assert!(true, "!can_hash<Hash, ConvertibleToSimple<Key> const&>()");;
    const _: () = assert!(true, "!can_hash<Hash, ConvertibleToSimple<Key>&&>()");;
    const _: () = assert!(true, "!can_hash<Hash, ConvertibleTo<Key>&>()");;
    const _: () = assert!(true, "!can_hash<Hash, ConvertibleTo<Key> const&>()");;
    const _: () = assert!(true, "!can_hash<Hash, ConvertibleTo<Key>&&>()");;
    const _: () = assert!(true, "!can_hash<Hash, ConvertibleTo<Key> const&&>()");;
}
pub unsafe fn test_hash_disabled_194() {
    const _: () = assert!(true, "!std::is_default_constructible<Hash>::value");;
    const _: () = assert!(true, "!std::is_copy_constructible<Hash>::value");;
    const _: () = assert!(true, "!std::is_move_constructible<Hash>::value");;
    const _: () = assert!(true, "!std::is_copy_assignable<Hash>::value");;
    const _: () = assert!(true, "!std::is_move_assignable<Hash>::value");;
    const _: () = assert!(
        true,
        "!std::is_function<typename std::remove_pointer<typename std::remove_reference<Hash>::type>::type>::value"
    );;
    const _: () = assert!(true, "!can_hash<Hash, Key&>()");;
    const _: () = assert!(true, "!can_hash<Hash, Key const&>()");;
    const _: () = assert!(true, "!can_hash<Hash, Key&&>()");;
    const _: () = assert!(true, "!can_hash<Hash const, Key&>()");;
    const _: () = assert!(true, "!can_hash<Hash const, Key const&>()");;
    const _: () = assert!(true, "!can_hash<Hash const, Key&&>()");;
    const _: () = assert!(true, "!can_hash<Hash, ConvertibleToSimple<Key>&>()");;
    const _: () = assert!(true, "!can_hash<Hash, ConvertibleToSimple<Key> const&>()");;
    const _: () = assert!(true, "!can_hash<Hash, ConvertibleToSimple<Key>&&>()");;
    const _: () = assert!(true, "!can_hash<Hash, ConvertibleTo<Key>&>()");;
    const _: () = assert!(true, "!can_hash<Hash, ConvertibleTo<Key> const&>()");;
    const _: () = assert!(true, "!can_hash<Hash, ConvertibleTo<Key>&&>()");;
    const _: () = assert!(true, "!can_hash<Hash, ConvertibleTo<Key> const&&>()");;
}
pub type Enum = ();
pub type EnumClass = ();
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Class {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestHashEnabled {}
pub unsafe fn test_hash_variant_195() {
    {
        let v: () = {
            let mut __tmp_0: () = 42_();
            std_variant_int__long__int_::new(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_196)).clone() },
                { &mut __tmp_0 },
            )
        };
        let v_copy: () = unimplemented!();
        let mut v2: () = {
            let mut __tmp_1: () = 100_();
            std_variant_int__long__int_::new(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_196)).clone() },
                { &mut __tmp_1 },
            )
        };
        let h: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "std::is_copy_constructible<H>::value");;
        }
    }
    {
        let mut str: *const () = c"hello".as_ptr();
        let v0: () = unimplemented!();
        let v0_other: () = unimplemented!();
        let v1: () = 42_();
        let v1_other: () = 100_();
        let mut v2: () = 100_();
        let mut v2_other: () = 999_();
        let mut v3: () = std_variant_std_monostate__int__long__const_char_ptr_::new_1({ &mut str });
        let mut v3_other: () = std_variant_std_monostate__int__long__const_char_ptr_::new_2({
            &std::mem::transmute(*b"not hello\0")
        });
        let h: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_hash_monostate_197() {
    let h: () = unimplemented!();
    let mut m1: () = unimplemented!();
    let m2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_copy_constructible<H>::value");;
    }
    {
        (unsafe { test_hash_enabled_188(None) });
    }
}
pub unsafe fn test_hash_variant_duplicate_elements_198() {
    let mut h: () = unimplemented!();
    let v1: () = std_variant_std_monostate__std_monostate_::new_1({
        (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_196)).clone()
    });
    let v2: () = std_variant_std_monostate__std_monostate_::new_2({
        (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_199)).clone()
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct B {}
pub unsafe fn test_hash_variant_enabled_200() {
    {
        (unsafe { test_hash_enabled_189(None) });
        (unsafe { test_hash_enabled_190(None) });
    }
    {
        (unsafe { test_hash_disabled_193() });
        (unsafe { test_hash_disabled_194() });
    }
    {
        (unsafe { test_hash_enabled_191(None) });
        (unsafe { test_hash_enabled_192(None) });
    }
}
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
    (unsafe { test_hash_variant_195() });
    (unsafe { test_hash_variant_duplicate_elements_198() });
    (unsafe { test_hash_monostate_197() });
    (unsafe { test_hash_variant_enabled_200() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const last_call_type_4);
    std::cell::LazyCell::force(&*&raw const last_call_args_5);
}
