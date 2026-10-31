extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
pub fn operator_eq_1(LHS: Ptr<TypeID>, RHS: Ptr<TypeID>) -> bool {
    return ({ LHS.with(|__s| __s.m_id.clone()) } == { RHS.with(|__s| __s.m_id.clone()) });
}
pub fn operator_ne_2(LHS: Ptr<TypeID>, RHS: Ptr<TypeID>) -> bool {
    return ({ LHS.with(|__s| __s.m_id.clone()) } != { RHS.with(|__s| __s.m_id.clone()) });
}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(8)]
pub struct TypeID {
    #[offset(0)]
    #[byte_size(8)]
    m_id: Ptr<i8>,
}
impl TypeID {
    fn new(mut xid: Ptr<i8>) -> Self {
        Self {
            m_id: (xid).clone(),
        }
    }
}
impl CppEq for TypeID {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_1(__lhs.clone(), __rhs.clone())
    }
}
pub type CallType = u32;
pub const CallType_CT_None: CallType = 0;
pub const CallType_CT_NonConst: CallType = 1;
pub const CallType_CT_Const: CallType = 2;
pub const CallType_CT_LValue: CallType = 4;
pub const CallType_CT_RValue: CallType = 8;
pub fn operator_bitor_3(mut LHS: CallType, mut RHS: CallType) -> CallType {
    return (((LHS as u32) | (RHS as u32)) as CallType);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(i32, to_i32)]
#[byte_size(1)]
pub struct ForwardingCallObject {}
thread_local!(
    pub static last_call_type_4: Value<CallType> = Rc::new(RefCell::new(CallType_CT_None));
);
thread_local!(
    pub static last_call_args_5: Value<Ptr<TypeID>> = Rc::new(RefCell::new(Ptr::<TypeID>::null()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnFirst {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnArity {}
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
pub fn can_hash_impl_26(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_27(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_28(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_29(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_30(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_31(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_32(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_33(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_34(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_35(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_36(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_37(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_38(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_39(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_40(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_41(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_42(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_43(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_44(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_45(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_46(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_47(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_48(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_49(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_50(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_51(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_52(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_53(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_54(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_55(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_56(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_57(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_58(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_59(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_60(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_61(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_62(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_63(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_64(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_65(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_66(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_67(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_68(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_69(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_70(mut _a0: i32) -> bool {
    return unimplemented!();
}
pub fn can_hash_impl_71(mut _a0: i64) -> bool {
    return false;
}
pub fn can_hash_impl_72(mut _a0: i64) -> bool {
    return false;
}
pub fn can_hash_impl_73(mut _a0: i64) -> bool {
    return false;
}
pub fn can_hash_impl_74(mut _a0: i64) -> bool {
    return false;
}
pub fn can_hash_impl_75(mut _a0: i64) -> bool {
    return false;
}
pub fn can_hash_impl_76(mut _a0: i64) -> bool {
    return false;
}
pub fn can_hash_impl_77(mut _a0: i64) -> bool {
    return false;
}
pub fn can_hash_impl_78(mut _a0: i64) -> bool {
    return false;
}
pub fn can_hash_impl_79(mut _a0: i64) -> bool {
    return false;
}
pub fn can_hash_impl_80(mut _a0: i64) -> bool {
    return false;
}
pub fn can_hash_impl_81(mut _a0: i64) -> bool {
    return false;
}
pub fn can_hash_impl_82(mut _a0: i64) -> bool {
    return false;
}
pub fn can_hash_impl_83(mut _a0: i64) -> bool {
    return false;
}
pub fn can_hash_impl_84(mut _a0: i64) -> bool {
    return false;
}
pub fn can_hash_impl_85(mut _a0: i64) -> bool {
    return false;
}
pub fn can_hash_impl_86(mut _a0: i64) -> bool {
    return false;
}
pub fn can_hash_impl_87(mut _a0: i64) -> bool {
    return false;
}
pub fn can_hash_impl_88(mut _a0: i64) -> bool {
    return false;
}
pub fn can_hash_impl_89(mut _a0: i64) -> bool {
    return false;
}
pub fn can_hash_impl_90(mut _a0: i64) -> bool {
    return false;
}
pub fn can_hash_impl_91(mut _a0: i64) -> bool {
    return false;
}
pub fn can_hash_impl_92(mut _a0: i64) -> bool {
    return false;
}
pub fn can_hash_impl_93(mut _a0: i64) -> bool {
    return false;
}
pub fn can_hash_impl_94(mut _a0: i64) -> bool {
    return false;
}
pub fn can_hash_impl_95(mut _a0: i64) -> bool {
    return false;
}
pub fn can_hash_impl_96(mut _a0: i64) -> bool {
    return false;
}
pub fn can_hash_97() -> bool {
    return ({ can_hash_impl_6(0) });
}
pub fn can_hash_98() -> bool {
    return ({ can_hash_impl_7(0) });
}
pub fn can_hash_99() -> bool {
    return ({ can_hash_impl_8(0) });
}
pub fn can_hash_100() -> bool {
    return ({ can_hash_impl_9(0) });
}
pub fn can_hash_101() -> bool {
    return ({ can_hash_impl_10(0) });
}
pub fn can_hash_102() -> bool {
    return ({ can_hash_impl_11(0) });
}
pub fn can_hash_103() -> bool {
    return ({ can_hash_impl_12(0) });
}
pub fn can_hash_104() -> bool {
    return ({ can_hash_impl_13(0) });
}
pub fn can_hash_105() -> bool {
    return ({ can_hash_impl_14(0) });
}
pub fn can_hash_106() -> bool {
    return ({ can_hash_impl_15(0) });
}
pub fn can_hash_107() -> bool {
    return ({ can_hash_impl_16(0) });
}
pub fn can_hash_108() -> bool {
    return ({ can_hash_impl_17(0) });
}
pub fn can_hash_109() -> bool {
    return ({ can_hash_impl_18(0) });
}
pub fn can_hash_110() -> bool {
    return ({ can_hash_impl_19(0) });
}
pub fn can_hash_111() -> bool {
    return ({ can_hash_impl_20(0) });
}
pub fn can_hash_112() -> bool {
    return ({ can_hash_impl_21(0) });
}
pub fn can_hash_113() -> bool {
    return ({ can_hash_impl_22(0) });
}
pub fn can_hash_114() -> bool {
    return ({ can_hash_impl_23(0) });
}
pub fn can_hash_115() -> bool {
    return ({ can_hash_impl_24(0) });
}
pub fn can_hash_116() -> bool {
    return ({ can_hash_impl_25(0) });
}
pub fn can_hash_117() -> bool {
    return ({ can_hash_impl_26(0) });
}
pub fn can_hash_118() -> bool {
    return ({ can_hash_impl_27(0) });
}
pub fn can_hash_119() -> bool {
    return ({ can_hash_impl_28(0) });
}
pub fn can_hash_120() -> bool {
    return ({ can_hash_impl_29(0) });
}
pub fn can_hash_121() -> bool {
    return ({ can_hash_impl_30(0) });
}
pub fn can_hash_122() -> bool {
    return ({ can_hash_impl_31(0) });
}
pub fn can_hash_123() -> bool {
    return ({ can_hash_impl_32(0) });
}
pub fn can_hash_124() -> bool {
    return ({ can_hash_impl_33(0) });
}
pub fn can_hash_125() -> bool {
    return ({ can_hash_impl_34(0) });
}
pub fn can_hash_126() -> bool {
    return ({ can_hash_impl_35(0) });
}
pub fn can_hash_127() -> bool {
    return ({ can_hash_impl_36(0) });
}
pub fn can_hash_128() -> bool {
    return ({ can_hash_impl_37(0) });
}
pub fn can_hash_129() -> bool {
    return ({ can_hash_impl_38(0) });
}
pub fn can_hash_130() -> bool {
    return ({ can_hash_impl_39(0) });
}
pub fn can_hash_131() -> bool {
    return ({ can_hash_impl_40(0) });
}
pub fn can_hash_132() -> bool {
    return ({ can_hash_impl_41(0) });
}
pub fn can_hash_133() -> bool {
    return ({ can_hash_impl_42(0) });
}
pub fn can_hash_134() -> bool {
    return ({ can_hash_impl_43(0) });
}
pub fn can_hash_135() -> bool {
    return ({ can_hash_impl_44(0) });
}
pub fn can_hash_136() -> bool {
    return ({ can_hash_impl_45(0) });
}
pub fn can_hash_137() -> bool {
    return ({ can_hash_impl_46(0) });
}
pub fn can_hash_138() -> bool {
    return ({ can_hash_impl_47(0) });
}
pub fn can_hash_139() -> bool {
    return ({ can_hash_impl_48(0) });
}
pub fn can_hash_140() -> bool {
    return ({ can_hash_impl_49(0) });
}
pub fn can_hash_141() -> bool {
    return ({ can_hash_impl_50(0) });
}
pub fn can_hash_142() -> bool {
    return ({ can_hash_impl_51(0) });
}
pub fn can_hash_143() -> bool {
    return ({ can_hash_impl_52(0) });
}
pub fn can_hash_144() -> bool {
    return ({ can_hash_impl_53(0) });
}
pub fn can_hash_145() -> bool {
    return ({ can_hash_impl_54(0) });
}
pub fn can_hash_146() -> bool {
    return ({ can_hash_impl_55(0) });
}
pub fn can_hash_147() -> bool {
    return ({ can_hash_impl_56(0) });
}
pub fn can_hash_148() -> bool {
    return ({ can_hash_impl_57(0) });
}
pub fn can_hash_149() -> bool {
    return ({ can_hash_impl_58(0) });
}
pub fn can_hash_150() -> bool {
    return ({ can_hash_impl_59(0) });
}
pub fn can_hash_151() -> bool {
    return ({ can_hash_impl_60(0) });
}
pub fn can_hash_152() -> bool {
    return ({ can_hash_impl_61(0) });
}
pub fn can_hash_153() -> bool {
    return ({ can_hash_impl_62(0) });
}
pub fn can_hash_154() -> bool {
    return ({ can_hash_impl_63(0) });
}
pub fn can_hash_155() -> bool {
    return ({ can_hash_impl_64(0) });
}
pub fn can_hash_156() -> bool {
    return ({ can_hash_impl_65(0) });
}
pub fn can_hash_157() -> bool {
    return ({ can_hash_impl_66(0) });
}
pub fn can_hash_158() -> bool {
    return ({ can_hash_impl_67(0) });
}
pub fn can_hash_159() -> bool {
    return ({ can_hash_impl_68(0) });
}
pub fn can_hash_160() -> bool {
    return ({ can_hash_impl_69(0) });
}
pub fn can_hash_161() -> bool {
    return ({ can_hash_impl_70(0) });
}
pub fn can_hash_162() -> bool {
    return ({ can_hash_impl_71(0_i64) });
}
pub fn can_hash_163() -> bool {
    return ({ can_hash_impl_72(0_i64) });
}
pub fn can_hash_164() -> bool {
    return ({ can_hash_impl_73(0_i64) });
}
pub fn can_hash_165() -> bool {
    return ({ can_hash_impl_74(0_i64) });
}
pub fn can_hash_166() -> bool {
    return ({ can_hash_impl_75(0_i64) });
}
pub fn can_hash_167() -> bool {
    return ({ can_hash_impl_76(0_i64) });
}
pub fn can_hash_168() -> bool {
    return ({ can_hash_impl_77(0_i64) });
}
pub fn can_hash_169() -> bool {
    return ({ can_hash_impl_78(0_i64) });
}
pub fn can_hash_170() -> bool {
    return ({ can_hash_impl_79(0_i64) });
}
pub fn can_hash_171() -> bool {
    return ({ can_hash_impl_80(0_i64) });
}
pub fn can_hash_172() -> bool {
    return ({ can_hash_impl_81(0_i64) });
}
pub fn can_hash_173() -> bool {
    return ({ can_hash_impl_82(0_i64) });
}
pub fn can_hash_174() -> bool {
    return ({ can_hash_impl_83(0_i64) });
}
pub fn can_hash_175() -> bool {
    return ({ can_hash_impl_84(0_i64) });
}
pub fn can_hash_176() -> bool {
    return ({ can_hash_impl_85(0_i64) });
}
pub fn can_hash_177() -> bool {
    return ({ can_hash_impl_86(0_i64) });
}
pub fn can_hash_178() -> bool {
    return ({ can_hash_impl_87(0_i64) });
}
pub fn can_hash_179() -> bool {
    return ({ can_hash_impl_88(0_i64) });
}
pub fn can_hash_180() -> bool {
    return ({ can_hash_impl_89(0_i64) });
}
pub fn can_hash_181() -> bool {
    return ({ can_hash_impl_90(0_i64) });
}
pub fn can_hash_182() -> bool {
    return ({ can_hash_impl_91(0_i64) });
}
pub fn can_hash_183() -> bool {
    return ({ can_hash_impl_92(0_i64) });
}
pub fn can_hash_184() -> bool {
    return ({ can_hash_impl_93(0_i64) });
}
pub fn can_hash_185() -> bool {
    return ({ can_hash_impl_94(0_i64) });
}
pub fn can_hash_186() -> bool {
    return ({ can_hash_impl_95(0_i64) });
}
pub fn can_hash_187() -> bool {
    return ({ can_hash_impl_96(0_i64) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_std_monostate_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_std_variant_int__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_std_variant_int_ptr__long__double__const_int__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_std_variant_int__B__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_std_variant_const_B__int__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct ConvertibleTo_std_monostate_ {
    #[offset(0)]
    #[byte_size(1)]
    pub to: (),
}
impl Default for ConvertibleTo_std_monostate_ {
    fn default() -> Self {
        {
            ConvertibleTo_std_monostate_ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct ConvertibleTo_std_variant_int__ {
    #[offset(0)]
    #[byte_size(8)]
    pub to: (),
}
impl Default for ConvertibleTo_std_variant_int__ {
    fn default() -> Self {
        {
            ConvertibleTo_std_variant_int__ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct ConvertibleTo_std_variant_int_ptr__long__double__const_int__ {
    #[offset(0)]
    #[byte_size(16)]
    pub to: (),
}
impl Default for ConvertibleTo_std_variant_int_ptr__long__double__const_int__ {
    fn default() -> Self {
        {
            ConvertibleTo_std_variant_int_ptr__long__double__const_int__ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct ConvertibleTo_std_variant_int__B__ {
    #[offset(0)]
    #[byte_size(8)]
    pub to: (),
}
impl Default for ConvertibleTo_std_variant_int__B__ {
    fn default() -> Self {
        {
            ConvertibleTo_std_variant_int__B__ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct ConvertibleTo_std_variant_const_B__int__ {
    #[offset(0)]
    #[byte_size(8)]
    pub to: (),
}
impl Default for ConvertibleTo_std_variant_const_B__int__ {
    fn default() -> Self {
        {
            ConvertibleTo_std_variant_const_B__int__ {
                to: unimplemented!(),
            }
        }
    }
}
pub fn test_hash_enabled_188(key: Option<Ptr<()>>) {
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
pub fn test_hash_enabled_189(key: Option<Ptr<()>>) {
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
pub fn test_hash_enabled_190(key: Option<Ptr<()>>) {
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
pub fn test_hash_enabled_191(key: Option<Ptr<()>>) {
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
pub fn test_hash_enabled_192(key: Option<Ptr<()>>) {
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
pub fn test_hash_disabled_193() {
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
pub fn test_hash_disabled_194() {
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
pub type Enum = u32;
pub type EnumClass = bool;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Class {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestHashEnabled {}
pub fn test_hash_variant_195() {
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let h: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == unimplemented!()));
        assert!((unimplemented!() != unimplemented!()));
        assert!((unimplemented!() == unimplemented!()));
        {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "std::is_copy_constructible<H>::value");;
        }
    }
    {
        let mut str: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hello");
        let v0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v0_other: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v1_other: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2_other: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v3_other: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let h: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == unimplemented!()));
        assert!((unimplemented!() == unimplemented!()));
        assert!((unimplemented!() == unimplemented!()));
        assert!((unimplemented!() != unimplemented!()));
        assert!((unimplemented!() == unimplemented!()));
        assert!((unimplemented!() != unimplemented!()));
        assert!((unimplemented!() == unimplemented!()));
        assert!((unimplemented!() != unimplemented!()));
        assert!((unimplemented!() != unimplemented!()));
        assert!((unimplemented!() != unimplemented!()));
        assert!((unimplemented!() != unimplemented!()));
        assert!((unimplemented!() != unimplemented!()));
        assert!((unimplemented!() != unimplemented!()));
        assert!((unimplemented!() != unimplemented!()));
    }
}
pub fn test_hash_monostate_196() {
    let h: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!((unimplemented!() == unimplemented!()));
    assert!((unimplemented!() == unimplemented!()));
    assert!((unimplemented!() == unimplemented!()));
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_copy_constructible<H>::value");;
    }
    {
        ({ test_hash_enabled_188(None) });
    }
}
pub fn test_hash_variant_duplicate_elements_197() {
    let h: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!((unimplemented!() == unimplemented!()));
    assert!((unimplemented!() == unimplemented!()));
    const _: () = assert!(true, "");;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct B {}
pub fn test_hash_variant_enabled_198() {
    {
        ({ test_hash_enabled_189(None) });
        ({ test_hash_enabled_190(None) });
    }
    {
        ({ test_hash_disabled_193() });
        ({ test_hash_disabled_194() });
    }
    {
        ({ test_hash_enabled_191(None) });
        ({ test_hash_enabled_192(None) });
    }
}
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
    ({ test_hash_variant_195() });
    ({ test_hash_variant_duplicate_elements_197() });
    ({ test_hash_monostate_196() });
    ({ test_hash_variant_enabled_198() });
    return 0;
}
pub trait ConvertibleToSimple_std_monostate_Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_std_monostate_Impl for Ptr<ConvertibleToSimple_std_monostate_> {}
pub trait ConvertibleToSimple_std_variant_const_B__int__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_std_variant_const_B__int__Impl
    for Ptr<ConvertibleToSimple_std_variant_const_B__int__>
{
}
pub trait ConvertibleToSimple_std_variant_int__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_std_variant_int__Impl for Ptr<ConvertibleToSimple_std_variant_int__> {}
pub trait ConvertibleToSimple_std_variant_int__B__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_std_variant_int__B__Impl
    for Ptr<ConvertibleToSimple_std_variant_int__B__>
{
}
pub trait ConvertibleToSimple_std_variant_int_ptr__long__double__const_int__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_std_variant_int_ptr__long__double__const_int__Impl
    for Ptr<ConvertibleToSimple_std_variant_int_ptr__long__double__const_int__>
{
}
pub trait ConvertibleTo_std_monostate_Impl {
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
impl ConvertibleTo_std_monostate_Impl for Ptr<ConvertibleTo_std_monostate_> {}
pub trait ConvertibleTo_std_variant_const_B__int__Impl {
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
impl ConvertibleTo_std_variant_const_B__int__Impl
    for Ptr<ConvertibleTo_std_variant_const_B__int__>
{
}
pub trait ConvertibleTo_std_variant_int__Impl {
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
impl ConvertibleTo_std_variant_int__Impl for Ptr<ConvertibleTo_std_variant_int__> {}
pub trait ConvertibleTo_std_variant_int__B__Impl {
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
impl ConvertibleTo_std_variant_int__B__Impl for Ptr<ConvertibleTo_std_variant_int__B__> {}
pub trait ConvertibleTo_std_variant_int_ptr__long__double__const_int__Impl {
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
impl ConvertibleTo_std_variant_int_ptr__long__double__const_int__Impl
    for Ptr<ConvertibleTo_std_variant_int_ptr__long__double__const_int__>
{
}
pub trait ForwardingCallObjectImpl {
    fn to_i32(&self) -> i32;
}
impl ForwardingCallObjectImpl for Ptr<ForwardingCallObject> {
    fn to_i32(&self) -> i32 {
        return 0;
    }
}
pub trait TypeIDImpl {
    fn name(&self) -> Vec<i8>;
}
impl TypeIDImpl for Ptr<TypeID> {
    fn name(&self) -> Vec<i8> {
        return {
            let mut __bytes = (*self).with(|__s| __s.m_id.clone()).to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        };
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = last_call_type_4.with(|_| ());
    let _ = last_call_args_5.with(|_| ());
}
