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
pub struct types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool__float__double__long_double__Enum__EnumClass__void_ptr__const_void_ptr__Class_ptr__std_nullptr_t_
{}
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
pub fn for_each_0(
    mut _a0 : types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool__float__double__long_double__Enum__EnumClass__void_ptr__const_void_ptr__Class_ptr__std_nullptr_t_,
    f: TestHashEnabled,
) {
    let f: Value<TestHashEnabled> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ TestHashEnabledImpl::operator_call_1_char(&f.as_pointer()) });
            0_()
        };
        let _arg1: () = {
            ({ TestHashEnabledImpl::operator_call_1_wchar_t(&f.as_pointer()) });
            0_()
        };
        let _arg2: () = {
            ({ TestHashEnabledImpl::operator_call_1_char8_t(&f.as_pointer()) });
            0_()
        };
        let _arg3: () = {
            ({ TestHashEnabledImpl::operator_call_1_char16_t(&f.as_pointer()) });
            0_()
        };
        let _arg4: () = {
            ({ TestHashEnabledImpl::operator_call_1_char32_t(&f.as_pointer()) });
            0_()
        };
        let _arg5: () = {
            ({ TestHashEnabledImpl::operator_call_1_signed_char(&f.as_pointer()) });
            0_()
        };
        let _arg6: () = {
            ({ TestHashEnabledImpl::operator_call_1_short(&f.as_pointer()) });
            0_()
        };
        let _arg7: () = {
            ({ TestHashEnabledImpl::operator_call_1_int(&f.as_pointer()) });
            0_()
        };
        let _arg8: () = {
            ({ TestHashEnabledImpl::operator_call_1_long(&f.as_pointer()) });
            0_()
        };
        let _arg9: () = {
            ({ TestHashEnabledImpl::operator_call_1_long_long(&f.as_pointer()) });
            0_()
        };
        let _arg10: () = {
            ({ TestHashEnabledImpl::operator_call_1___int128(&f.as_pointer()) });
            0_()
        };
        let _arg11: () = {
            ({ TestHashEnabledImpl::operator_call_1_unsigned_char(&f.as_pointer()) });
            0_()
        };
        let _arg12: () = {
            ({ TestHashEnabledImpl::operator_call_1_unsigned_short(&f.as_pointer()) });
            0_()
        };
        let _arg13: () = {
            ({ TestHashEnabledImpl::operator_call_1_unsigned_int(&f.as_pointer()) });
            0_()
        };
        let _arg14: () = {
            ({ TestHashEnabledImpl::operator_call_1_unsigned_long(&f.as_pointer()) });
            0_()
        };
        let _arg15: () = {
            ({ TestHashEnabledImpl::operator_call_1_unsigned_long_long(&f.as_pointer()) });
            0_()
        };
        let _arg16: () = {
            ({ TestHashEnabledImpl::operator_call_1_unsigned___int128(&f.as_pointer()) });
            0_()
        };
        let _arg17: () = {
            ({ TestHashEnabledImpl::operator_call_1__Bool(&f.as_pointer()) });
            0_()
        };
        let _arg18: () = {
            ({ TestHashEnabledImpl::operator_call_1_float(&f.as_pointer()) });
            0_()
        };
        let _arg19: () = {
            ({ TestHashEnabledImpl::operator_call_1_double(&f.as_pointer()) });
            0_()
        };
        let _arg20: () = {
            ({ TestHashEnabledImpl::operator_call_1_long_double(&f.as_pointer()) });
            0_()
        };
        let _arg21: () = {
            ({ TestHashEnabledImpl::operator_call_1_enum_Enum(&f.as_pointer()) });
            0_()
        };
        let _arg22: () = {
            ({ TestHashEnabledImpl::operator_call_1_enum_EnumClass(&f.as_pointer()) });
            0_()
        };
        let _arg23: () = {
            ({ TestHashEnabledImpl::operator_call_1_void_ptr(&f.as_pointer()) });
            0_()
        };
        let _arg24: () = {
            ({ TestHashEnabledImpl::operator_call_1_const_void_ptr(&f.as_pointer()) });
            0_()
        };
        let _arg25: () = {
            ({ TestHashEnabledImpl::operator_call_1_struct_Class_ptr(&f.as_pointer()) });
            0_()
        };
        let _arg26: () = {
            ({ TestHashEnabledImpl::operator_call_1_nullptr_t(&f.as_pointer()) });
            0_()
        };
        swallow_1(
            _arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7, _arg8, _arg9, _arg10, _arg11,
            _arg12, _arg13, _arg14, _arg15, _arg16, _arg17, _arg18, _arg19, _arg20, _arg21, _arg22,
            _arg23, _arg24, _arg25, _arg26,
        )
    });
}
pub fn swallow_1(
    mut _a0: (),
    mut _a1: (),
    mut _a2: (),
    mut _a3: (),
    mut _a4: (),
    mut _a5: (),
    mut _a6: (),
    mut _a7: (),
    mut _a8: (),
    mut _a9: (),
    mut _a10: (),
    mut _a11: (),
    mut _a12: (),
    mut _a13: (),
    mut _a14: (),
    mut _a15: (),
    mut _a16: (),
    mut _a17: (),
    mut _a18: (),
    mut _a19: (),
    mut _a20: (),
    mut _a21: (),
    mut _a22: (),
    mut _a23: (),
    mut _a24: (),
    mut _a25: (),
    mut _a26: (),
) {
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
pub fn can_hash_impl_13(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_14(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_15(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_16(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_17(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_18(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_19(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_20(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_21(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_22(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_23(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_24(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_25(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_26(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_27(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_28(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_29(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_30(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_31(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_32(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_33(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_34(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_35(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_36(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_37(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_38(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_39(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_40(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_41(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_42(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_43(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_44(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_45(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_46(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_47(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_48(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_49(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_50(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_51(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_52(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_53(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_54(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_55(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_56(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_57(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_58(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_59(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_60(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_61(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_62(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_63(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_64(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_65(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_66(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_67(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_68(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_69(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_70(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_71(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_72(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_73(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_74(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_75(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_76(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_77(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_78(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_79(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_80(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_81(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_82(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_83(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_84(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_85(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_86(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_87(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_88(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_89(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_90(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_91(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_92(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_93(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_94(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_95(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_96(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_97(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_98(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_99(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_100(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_101(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_102(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_103(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_104(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_105(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_106(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_107(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_108(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_109(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_110(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_111(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_112(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_113(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_114(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_115(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_116(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_117(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_118(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_119(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_120(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_121(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_122(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_123(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_124(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_125(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_126(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_127(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_128(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_129(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_130(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_131(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_132(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_133(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_134(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_135(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_136(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_137(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_138(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_139(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_140(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_141(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_142(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_143(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_144(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_145(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_146(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_147(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_148(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_149(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_150(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_151(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_152(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_153(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_154(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_155(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_156(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_157(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_158(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_159(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_160(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_161(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_162(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_163(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_164(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_165(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_166(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_167(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_168(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_169(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_170(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_171(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_172(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_173(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_174(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_175(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_176(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_177(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_178(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_179(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_180(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_181(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_182(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_183(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_184(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_185(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_186(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_187(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_188(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_189(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_190(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_191(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_192(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_193(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_194(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_195(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_196(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_197(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_198(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_199(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_200(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_201(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_202(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_203(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_204(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_205(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_206(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_207(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_208(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_209(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_210(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_211(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_212(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_213(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_214(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_215(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_216(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_217(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_218(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_219(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_220(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_221(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_222(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_223(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_224(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_225(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_226(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_227(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_228(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_229(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_230(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_231(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_232(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_233(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_234(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_235(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_236(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_237(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_238(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_239(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_240(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_241(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_242(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_243(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_244(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_245(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_246(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_247(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_248(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_249(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_250(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_251(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_252(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_253(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_254(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_255(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_256(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_257(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_258(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_259(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_260(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_261(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_262(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_263(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_264(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_265(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_266(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_267(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_268(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_269(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_270(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_271(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_272(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_273(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_274(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_275(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_276(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_277(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_278(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_279(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_280(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_281(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_282(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_283(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_284(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_285(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_286(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_287(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_288(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_289(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_290(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_291(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_292(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_293(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_294(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_295(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_296(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_297(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_298(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_299(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_300(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_301(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_302(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_303(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_304(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_305(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_306(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_307(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_308(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_309(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_310(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_311(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_312(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_313(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_314(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_315(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_316(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_317(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_318(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_319(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_320(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_321(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_322(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_323(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_324(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_325(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_326(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_327(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_328(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_329(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_330(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_331(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_332(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_333(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_334(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_335(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_336(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_337(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_338(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_339(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_340(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_341(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_342(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_343(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_344(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_345(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_346(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_347(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_348(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_349(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_350(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_351(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_352(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_353(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_354(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_355(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_356(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_357(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_358(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_359(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_360(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_361(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_362(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_363(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_364(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_365(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_366(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_367(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_368(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_369(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_370(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_371(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_372(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_373(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_374(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_375(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_376(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_377(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_impl_378(mut _a0: ()) -> () {
    return unimplemented!();
}
pub fn can_hash_379() -> () {
    return ({ can_hash_impl_2(0_()) });
}
pub fn can_hash_380() -> () {
    return ({ can_hash_impl_3(0_()) });
}
pub fn can_hash_381() -> () {
    return ({ can_hash_impl_4(0_()) });
}
pub fn can_hash_382() -> () {
    return ({ can_hash_impl_5(0_()) });
}
pub fn can_hash_383() -> () {
    return ({ can_hash_impl_6(0_()) });
}
pub fn can_hash_384() -> () {
    return ({ can_hash_impl_7(0_()) });
}
pub fn can_hash_385() -> () {
    return ({ can_hash_impl_8(0_()) });
}
pub fn can_hash_386() -> () {
    return ({ can_hash_impl_9(0_()) });
}
pub fn can_hash_387() -> () {
    return ({ can_hash_impl_10(0_()) });
}
pub fn can_hash_388() -> () {
    return ({ can_hash_impl_11(0_()) });
}
pub fn can_hash_389() -> () {
    return ({ can_hash_impl_12(0_()) });
}
pub fn can_hash_390() -> () {
    return ({ can_hash_impl_13(0_()) });
}
pub fn can_hash_391() -> () {
    return ({ can_hash_impl_14(0_()) });
}
pub fn can_hash_392() -> () {
    return ({ can_hash_impl_15(0_()) });
}
pub fn can_hash_393() -> () {
    return ({ can_hash_impl_16(0_()) });
}
pub fn can_hash_394() -> () {
    return ({ can_hash_impl_17(0_()) });
}
pub fn can_hash_395() -> () {
    return ({ can_hash_impl_18(0_()) });
}
pub fn can_hash_396() -> () {
    return ({ can_hash_impl_19(0_()) });
}
pub fn can_hash_397() -> () {
    return ({ can_hash_impl_20(0_()) });
}
pub fn can_hash_398() -> () {
    return ({ can_hash_impl_21(0_()) });
}
pub fn can_hash_399() -> () {
    return ({ can_hash_impl_22(0_()) });
}
pub fn can_hash_400() -> () {
    return ({ can_hash_impl_23(0_()) });
}
pub fn can_hash_401() -> () {
    return ({ can_hash_impl_24(0_()) });
}
pub fn can_hash_402() -> () {
    return ({ can_hash_impl_25(0_()) });
}
pub fn can_hash_403() -> () {
    return ({ can_hash_impl_26(0_()) });
}
pub fn can_hash_404() -> () {
    return ({ can_hash_impl_27(0_()) });
}
pub fn can_hash_405() -> () {
    return ({ can_hash_impl_28(0_()) });
}
pub fn can_hash_406() -> () {
    return ({ can_hash_impl_29(0_()) });
}
pub fn can_hash_407() -> () {
    return ({ can_hash_impl_30(0_()) });
}
pub fn can_hash_408() -> () {
    return ({ can_hash_impl_31(0_()) });
}
pub fn can_hash_409() -> () {
    return ({ can_hash_impl_32(0_()) });
}
pub fn can_hash_410() -> () {
    return ({ can_hash_impl_33(0_()) });
}
pub fn can_hash_411() -> () {
    return ({ can_hash_impl_34(0_()) });
}
pub fn can_hash_412() -> () {
    return ({ can_hash_impl_35(0_()) });
}
pub fn can_hash_413() -> () {
    return ({ can_hash_impl_36(0_()) });
}
pub fn can_hash_414() -> () {
    return ({ can_hash_impl_37(0_()) });
}
pub fn can_hash_415() -> () {
    return ({ can_hash_impl_38(0_()) });
}
pub fn can_hash_416() -> () {
    return ({ can_hash_impl_39(0_()) });
}
pub fn can_hash_417() -> () {
    return ({ can_hash_impl_40(0_()) });
}
pub fn can_hash_418() -> () {
    return ({ can_hash_impl_41(0_()) });
}
pub fn can_hash_419() -> () {
    return ({ can_hash_impl_42(0_()) });
}
pub fn can_hash_420() -> () {
    return ({ can_hash_impl_43(0_()) });
}
pub fn can_hash_421() -> () {
    return ({ can_hash_impl_44(0_()) });
}
pub fn can_hash_422() -> () {
    return ({ can_hash_impl_45(0_()) });
}
pub fn can_hash_423() -> () {
    return ({ can_hash_impl_46(0_()) });
}
pub fn can_hash_424() -> () {
    return ({ can_hash_impl_47(0_()) });
}
pub fn can_hash_425() -> () {
    return ({ can_hash_impl_48(0_()) });
}
pub fn can_hash_426() -> () {
    return ({ can_hash_impl_49(0_()) });
}
pub fn can_hash_427() -> () {
    return ({ can_hash_impl_50(0_()) });
}
pub fn can_hash_428() -> () {
    return ({ can_hash_impl_51(0_()) });
}
pub fn can_hash_429() -> () {
    return ({ can_hash_impl_52(0_()) });
}
pub fn can_hash_430() -> () {
    return ({ can_hash_impl_53(0_()) });
}
pub fn can_hash_431() -> () {
    return ({ can_hash_impl_54(0_()) });
}
pub fn can_hash_432() -> () {
    return ({ can_hash_impl_55(0_()) });
}
pub fn can_hash_433() -> () {
    return ({ can_hash_impl_56(0_()) });
}
pub fn can_hash_434() -> () {
    return ({ can_hash_impl_57(0_()) });
}
pub fn can_hash_435() -> () {
    return ({ can_hash_impl_58(0_()) });
}
pub fn can_hash_436() -> () {
    return ({ can_hash_impl_59(0_()) });
}
pub fn can_hash_437() -> () {
    return ({ can_hash_impl_60(0_()) });
}
pub fn can_hash_438() -> () {
    return ({ can_hash_impl_61(0_()) });
}
pub fn can_hash_439() -> () {
    return ({ can_hash_impl_62(0_()) });
}
pub fn can_hash_440() -> () {
    return ({ can_hash_impl_63(0_()) });
}
pub fn can_hash_441() -> () {
    return ({ can_hash_impl_64(0_()) });
}
pub fn can_hash_442() -> () {
    return ({ can_hash_impl_65(0_()) });
}
pub fn can_hash_443() -> () {
    return ({ can_hash_impl_66(0_()) });
}
pub fn can_hash_444() -> () {
    return ({ can_hash_impl_67(0_()) });
}
pub fn can_hash_445() -> () {
    return ({ can_hash_impl_68(0_()) });
}
pub fn can_hash_446() -> () {
    return ({ can_hash_impl_69(0_()) });
}
pub fn can_hash_447() -> () {
    return ({ can_hash_impl_70(0_()) });
}
pub fn can_hash_448() -> () {
    return ({ can_hash_impl_71(0_()) });
}
pub fn can_hash_449() -> () {
    return ({ can_hash_impl_72(0_()) });
}
pub fn can_hash_450() -> () {
    return ({ can_hash_impl_73(0_()) });
}
pub fn can_hash_451() -> () {
    return ({ can_hash_impl_74(0_()) });
}
pub fn can_hash_452() -> () {
    return ({ can_hash_impl_75(0_()) });
}
pub fn can_hash_453() -> () {
    return ({ can_hash_impl_76(0_()) });
}
pub fn can_hash_454() -> () {
    return ({ can_hash_impl_77(0_()) });
}
pub fn can_hash_455() -> () {
    return ({ can_hash_impl_78(0_()) });
}
pub fn can_hash_456() -> () {
    return ({ can_hash_impl_79(0_()) });
}
pub fn can_hash_457() -> () {
    return ({ can_hash_impl_80(0_()) });
}
pub fn can_hash_458() -> () {
    return ({ can_hash_impl_81(0_()) });
}
pub fn can_hash_459() -> () {
    return ({ can_hash_impl_82(0_()) });
}
pub fn can_hash_460() -> () {
    return ({ can_hash_impl_83(0_()) });
}
pub fn can_hash_461() -> () {
    return ({ can_hash_impl_84(0_()) });
}
pub fn can_hash_462() -> () {
    return ({ can_hash_impl_85(0_()) });
}
pub fn can_hash_463() -> () {
    return ({ can_hash_impl_86(0_()) });
}
pub fn can_hash_464() -> () {
    return ({ can_hash_impl_87(0_()) });
}
pub fn can_hash_465() -> () {
    return ({ can_hash_impl_88(0_()) });
}
pub fn can_hash_466() -> () {
    return ({ can_hash_impl_89(0_()) });
}
pub fn can_hash_467() -> () {
    return ({ can_hash_impl_90(0_()) });
}
pub fn can_hash_468() -> () {
    return ({ can_hash_impl_91(0_()) });
}
pub fn can_hash_469() -> () {
    return ({ can_hash_impl_92(0_()) });
}
pub fn can_hash_470() -> () {
    return ({ can_hash_impl_93(0_()) });
}
pub fn can_hash_471() -> () {
    return ({ can_hash_impl_94(0_()) });
}
pub fn can_hash_472() -> () {
    return ({ can_hash_impl_95(0_()) });
}
pub fn can_hash_473() -> () {
    return ({ can_hash_impl_96(0_()) });
}
pub fn can_hash_474() -> () {
    return ({ can_hash_impl_97(0_()) });
}
pub fn can_hash_475() -> () {
    return ({ can_hash_impl_98(0_()) });
}
pub fn can_hash_476() -> () {
    return ({ can_hash_impl_99(0_()) });
}
pub fn can_hash_477() -> () {
    return ({ can_hash_impl_100(0_()) });
}
pub fn can_hash_478() -> () {
    return ({ can_hash_impl_101(0_()) });
}
pub fn can_hash_479() -> () {
    return ({ can_hash_impl_102(0_()) });
}
pub fn can_hash_480() -> () {
    return ({ can_hash_impl_103(0_()) });
}
pub fn can_hash_481() -> () {
    return ({ can_hash_impl_104(0_()) });
}
pub fn can_hash_482() -> () {
    return ({ can_hash_impl_105(0_()) });
}
pub fn can_hash_483() -> () {
    return ({ can_hash_impl_106(0_()) });
}
pub fn can_hash_484() -> () {
    return ({ can_hash_impl_107(0_()) });
}
pub fn can_hash_485() -> () {
    return ({ can_hash_impl_108(0_()) });
}
pub fn can_hash_486() -> () {
    return ({ can_hash_impl_109(0_()) });
}
pub fn can_hash_487() -> () {
    return ({ can_hash_impl_110(0_()) });
}
pub fn can_hash_488() -> () {
    return ({ can_hash_impl_111(0_()) });
}
pub fn can_hash_489() -> () {
    return ({ can_hash_impl_112(0_()) });
}
pub fn can_hash_490() -> () {
    return ({ can_hash_impl_113(0_()) });
}
pub fn can_hash_491() -> () {
    return ({ can_hash_impl_114(0_()) });
}
pub fn can_hash_492() -> () {
    return ({ can_hash_impl_115(0_()) });
}
pub fn can_hash_493() -> () {
    return ({ can_hash_impl_116(0_()) });
}
pub fn can_hash_494() -> () {
    return ({ can_hash_impl_117(0_()) });
}
pub fn can_hash_495() -> () {
    return ({ can_hash_impl_118(0_()) });
}
pub fn can_hash_496() -> () {
    return ({ can_hash_impl_119(0_()) });
}
pub fn can_hash_497() -> () {
    return ({ can_hash_impl_120(0_()) });
}
pub fn can_hash_498() -> () {
    return ({ can_hash_impl_121(0_()) });
}
pub fn can_hash_499() -> () {
    return ({ can_hash_impl_122(0_()) });
}
pub fn can_hash_500() -> () {
    return ({ can_hash_impl_123(0_()) });
}
pub fn can_hash_501() -> () {
    return ({ can_hash_impl_124(0_()) });
}
pub fn can_hash_502() -> () {
    return ({ can_hash_impl_125(0_()) });
}
pub fn can_hash_503() -> () {
    return ({ can_hash_impl_126(0_()) });
}
pub fn can_hash_504() -> () {
    return ({ can_hash_impl_127(0_()) });
}
pub fn can_hash_505() -> () {
    return ({ can_hash_impl_128(0_()) });
}
pub fn can_hash_506() -> () {
    return ({ can_hash_impl_129(0_()) });
}
pub fn can_hash_507() -> () {
    return ({ can_hash_impl_130(0_()) });
}
pub fn can_hash_508() -> () {
    return ({ can_hash_impl_131(0_()) });
}
pub fn can_hash_509() -> () {
    return ({ can_hash_impl_132(0_()) });
}
pub fn can_hash_510() -> () {
    return ({ can_hash_impl_133(0_()) });
}
pub fn can_hash_511() -> () {
    return ({ can_hash_impl_134(0_()) });
}
pub fn can_hash_512() -> () {
    return ({ can_hash_impl_135(0_()) });
}
pub fn can_hash_513() -> () {
    return ({ can_hash_impl_136(0_()) });
}
pub fn can_hash_514() -> () {
    return ({ can_hash_impl_137(0_()) });
}
pub fn can_hash_515() -> () {
    return ({ can_hash_impl_138(0_()) });
}
pub fn can_hash_516() -> () {
    return ({ can_hash_impl_139(0_()) });
}
pub fn can_hash_517() -> () {
    return ({ can_hash_impl_140(0_()) });
}
pub fn can_hash_518() -> () {
    return ({ can_hash_impl_141(0_()) });
}
pub fn can_hash_519() -> () {
    return ({ can_hash_impl_142(0_()) });
}
pub fn can_hash_520() -> () {
    return ({ can_hash_impl_143(0_()) });
}
pub fn can_hash_521() -> () {
    return ({ can_hash_impl_144(0_()) });
}
pub fn can_hash_522() -> () {
    return ({ can_hash_impl_145(0_()) });
}
pub fn can_hash_523() -> () {
    return ({ can_hash_impl_146(0_()) });
}
pub fn can_hash_524() -> () {
    return ({ can_hash_impl_147(0_()) });
}
pub fn can_hash_525() -> () {
    return ({ can_hash_impl_148(0_()) });
}
pub fn can_hash_526() -> () {
    return ({ can_hash_impl_149(0_()) });
}
pub fn can_hash_527() -> () {
    return ({ can_hash_impl_150(0_()) });
}
pub fn can_hash_528() -> () {
    return ({ can_hash_impl_151(0_()) });
}
pub fn can_hash_529() -> () {
    return ({ can_hash_impl_152(0_()) });
}
pub fn can_hash_530() -> () {
    return ({ can_hash_impl_153(0_()) });
}
pub fn can_hash_531() -> () {
    return ({ can_hash_impl_154(0_()) });
}
pub fn can_hash_532() -> () {
    return ({ can_hash_impl_155(0_()) });
}
pub fn can_hash_533() -> () {
    return ({ can_hash_impl_156(0_()) });
}
pub fn can_hash_534() -> () {
    return ({ can_hash_impl_157(0_()) });
}
pub fn can_hash_535() -> () {
    return ({ can_hash_impl_158(0_()) });
}
pub fn can_hash_536() -> () {
    return ({ can_hash_impl_159(0_()) });
}
pub fn can_hash_537() -> () {
    return ({ can_hash_impl_160(0_()) });
}
pub fn can_hash_538() -> () {
    return ({ can_hash_impl_161(0_()) });
}
pub fn can_hash_539() -> () {
    return ({ can_hash_impl_162(0_()) });
}
pub fn can_hash_540() -> () {
    return ({ can_hash_impl_163(0_()) });
}
pub fn can_hash_541() -> () {
    return ({ can_hash_impl_164(0_()) });
}
pub fn can_hash_542() -> () {
    return ({ can_hash_impl_165(0_()) });
}
pub fn can_hash_543() -> () {
    return ({ can_hash_impl_166(0_()) });
}
pub fn can_hash_544() -> () {
    return ({ can_hash_impl_167(0_()) });
}
pub fn can_hash_545() -> () {
    return ({ can_hash_impl_168(0_()) });
}
pub fn can_hash_546() -> () {
    return ({ can_hash_impl_169(0_()) });
}
pub fn can_hash_547() -> () {
    return ({ can_hash_impl_170(0_()) });
}
pub fn can_hash_548() -> () {
    return ({ can_hash_impl_171(0_()) });
}
pub fn can_hash_549() -> () {
    return ({ can_hash_impl_172(0_()) });
}
pub fn can_hash_550() -> () {
    return ({ can_hash_impl_173(0_()) });
}
pub fn can_hash_551() -> () {
    return ({ can_hash_impl_174(0_()) });
}
pub fn can_hash_552() -> () {
    return ({ can_hash_impl_175(0_()) });
}
pub fn can_hash_553() -> () {
    return ({ can_hash_impl_176(0_()) });
}
pub fn can_hash_554() -> () {
    return ({ can_hash_impl_177(0_()) });
}
pub fn can_hash_555() -> () {
    return ({ can_hash_impl_178(0_()) });
}
pub fn can_hash_556() -> () {
    return ({ can_hash_impl_179(0_()) });
}
pub fn can_hash_557() -> () {
    return ({ can_hash_impl_180(0_()) });
}
pub fn can_hash_558() -> () {
    return ({ can_hash_impl_181(0_()) });
}
pub fn can_hash_559() -> () {
    return ({ can_hash_impl_182(0_()) });
}
pub fn can_hash_560() -> () {
    return ({ can_hash_impl_183(0_()) });
}
pub fn can_hash_561() -> () {
    return ({ can_hash_impl_184(0_()) });
}
pub fn can_hash_562() -> () {
    return ({ can_hash_impl_185(0_()) });
}
pub fn can_hash_563() -> () {
    return ({ can_hash_impl_186(0_()) });
}
pub fn can_hash_564() -> () {
    return ({ can_hash_impl_187(0_()) });
}
pub fn can_hash_565() -> () {
    return ({ can_hash_impl_188(0_()) });
}
pub fn can_hash_566() -> () {
    return ({ can_hash_impl_189(0_()) });
}
pub fn can_hash_567() -> () {
    return ({ can_hash_impl_190(0_()) });
}
pub fn can_hash_568() -> () {
    return ({ can_hash_impl_191(0_()) });
}
pub fn can_hash_569() -> () {
    return ({ can_hash_impl_192(0_()) });
}
pub fn can_hash_570() -> () {
    return ({ can_hash_impl_193(0_()) });
}
pub fn can_hash_571() -> () {
    return ({ can_hash_impl_194(0_()) });
}
pub fn can_hash_572() -> () {
    return ({ can_hash_impl_195(0_()) });
}
pub fn can_hash_573() -> () {
    return ({ can_hash_impl_196(0_()) });
}
pub fn can_hash_574() -> () {
    return ({ can_hash_impl_197(0_()) });
}
pub fn can_hash_575() -> () {
    return ({ can_hash_impl_198(0_()) });
}
pub fn can_hash_576() -> () {
    return ({ can_hash_impl_199(0_()) });
}
pub fn can_hash_577() -> () {
    return ({ can_hash_impl_200(0_()) });
}
pub fn can_hash_578() -> () {
    return ({ can_hash_impl_201(0_()) });
}
pub fn can_hash_579() -> () {
    return ({ can_hash_impl_202(0_()) });
}
pub fn can_hash_580() -> () {
    return ({ can_hash_impl_203(0_()) });
}
pub fn can_hash_581() -> () {
    return ({ can_hash_impl_204(0_()) });
}
pub fn can_hash_582() -> () {
    return ({ can_hash_impl_205(0_()) });
}
pub fn can_hash_583() -> () {
    return ({ can_hash_impl_206(0_()) });
}
pub fn can_hash_584() -> () {
    return ({ can_hash_impl_207(0_()) });
}
pub fn can_hash_585() -> () {
    return ({ can_hash_impl_208(0_()) });
}
pub fn can_hash_586() -> () {
    return ({ can_hash_impl_209(0_()) });
}
pub fn can_hash_587() -> () {
    return ({ can_hash_impl_210(0_()) });
}
pub fn can_hash_588() -> () {
    return ({ can_hash_impl_211(0_()) });
}
pub fn can_hash_589() -> () {
    return ({ can_hash_impl_212(0_()) });
}
pub fn can_hash_590() -> () {
    return ({ can_hash_impl_213(0_()) });
}
pub fn can_hash_591() -> () {
    return ({ can_hash_impl_214(0_()) });
}
pub fn can_hash_592() -> () {
    return ({ can_hash_impl_215(0_()) });
}
pub fn can_hash_593() -> () {
    return ({ can_hash_impl_216(0_()) });
}
pub fn can_hash_594() -> () {
    return ({ can_hash_impl_217(0_()) });
}
pub fn can_hash_595() -> () {
    return ({ can_hash_impl_218(0_()) });
}
pub fn can_hash_596() -> () {
    return ({ can_hash_impl_219(0_()) });
}
pub fn can_hash_597() -> () {
    return ({ can_hash_impl_220(0_()) });
}
pub fn can_hash_598() -> () {
    return ({ can_hash_impl_221(0_()) });
}
pub fn can_hash_599() -> () {
    return ({ can_hash_impl_222(0_()) });
}
pub fn can_hash_600() -> () {
    return ({ can_hash_impl_223(0_()) });
}
pub fn can_hash_601() -> () {
    return ({ can_hash_impl_224(0_()) });
}
pub fn can_hash_602() -> () {
    return ({ can_hash_impl_225(0_()) });
}
pub fn can_hash_603() -> () {
    return ({ can_hash_impl_226(0_()) });
}
pub fn can_hash_604() -> () {
    return ({ can_hash_impl_227(0_()) });
}
pub fn can_hash_605() -> () {
    return ({ can_hash_impl_228(0_()) });
}
pub fn can_hash_606() -> () {
    return ({ can_hash_impl_229(0_()) });
}
pub fn can_hash_607() -> () {
    return ({ can_hash_impl_230(0_()) });
}
pub fn can_hash_608() -> () {
    return ({ can_hash_impl_231(0_()) });
}
pub fn can_hash_609() -> () {
    return ({ can_hash_impl_232(0_()) });
}
pub fn can_hash_610() -> () {
    return ({ can_hash_impl_233(0_()) });
}
pub fn can_hash_611() -> () {
    return ({ can_hash_impl_234(0_()) });
}
pub fn can_hash_612() -> () {
    return ({ can_hash_impl_235(0_()) });
}
pub fn can_hash_613() -> () {
    return ({ can_hash_impl_236(0_()) });
}
pub fn can_hash_614() -> () {
    return ({ can_hash_impl_237(0_()) });
}
pub fn can_hash_615() -> () {
    return ({ can_hash_impl_238(0_()) });
}
pub fn can_hash_616() -> () {
    return ({ can_hash_impl_239(0_()) });
}
pub fn can_hash_617() -> () {
    return ({ can_hash_impl_240(0_()) });
}
pub fn can_hash_618() -> () {
    return ({ can_hash_impl_241(0_()) });
}
pub fn can_hash_619() -> () {
    return ({ can_hash_impl_242(0_()) });
}
pub fn can_hash_620() -> () {
    return ({ can_hash_impl_243(0_()) });
}
pub fn can_hash_621() -> () {
    return ({ can_hash_impl_244(0_()) });
}
pub fn can_hash_622() -> () {
    return ({ can_hash_impl_245(0_()) });
}
pub fn can_hash_623() -> () {
    return ({ can_hash_impl_246(0_()) });
}
pub fn can_hash_624() -> () {
    return ({ can_hash_impl_247(0_()) });
}
pub fn can_hash_625() -> () {
    return ({ can_hash_impl_248(0_()) });
}
pub fn can_hash_626() -> () {
    return ({ can_hash_impl_249(0_()) });
}
pub fn can_hash_627() -> () {
    return ({ can_hash_impl_250(0_()) });
}
pub fn can_hash_628() -> () {
    return ({ can_hash_impl_251(0_()) });
}
pub fn can_hash_629() -> () {
    return ({ can_hash_impl_252(0_()) });
}
pub fn can_hash_630() -> () {
    return ({ can_hash_impl_253(0_()) });
}
pub fn can_hash_631() -> () {
    return ({ can_hash_impl_254(0_()) });
}
pub fn can_hash_632() -> () {
    return ({ can_hash_impl_255(0_()) });
}
pub fn can_hash_633() -> () {
    return ({ can_hash_impl_256(0_()) });
}
pub fn can_hash_634() -> () {
    return ({ can_hash_impl_257(0_()) });
}
pub fn can_hash_635() -> () {
    return ({ can_hash_impl_258(0_()) });
}
pub fn can_hash_636() -> () {
    return ({ can_hash_impl_259(0_()) });
}
pub fn can_hash_637() -> () {
    return ({ can_hash_impl_260(0_()) });
}
pub fn can_hash_638() -> () {
    return ({ can_hash_impl_261(0_()) });
}
pub fn can_hash_639() -> () {
    return ({ can_hash_impl_262(0_()) });
}
pub fn can_hash_640() -> () {
    return ({ can_hash_impl_263(0_()) });
}
pub fn can_hash_641() -> () {
    return ({ can_hash_impl_264(0_()) });
}
pub fn can_hash_642() -> () {
    return ({ can_hash_impl_265(0_()) });
}
pub fn can_hash_643() -> () {
    return ({ can_hash_impl_266(0_()) });
}
pub fn can_hash_644() -> () {
    return ({ can_hash_impl_267(0_()) });
}
pub fn can_hash_645() -> () {
    return ({ can_hash_impl_268(0_()) });
}
pub fn can_hash_646() -> () {
    return ({ can_hash_impl_269(0_()) });
}
pub fn can_hash_647() -> () {
    return ({ can_hash_impl_270(0_()) });
}
pub fn can_hash_648() -> () {
    return ({ can_hash_impl_271(0_()) });
}
pub fn can_hash_649() -> () {
    return ({ can_hash_impl_272(0_()) });
}
pub fn can_hash_650() -> () {
    return ({ can_hash_impl_273(0_()) });
}
pub fn can_hash_651() -> () {
    return ({ can_hash_impl_274(0_()) });
}
pub fn can_hash_652() -> () {
    return ({ can_hash_impl_275(0_()) });
}
pub fn can_hash_653() -> () {
    return ({ can_hash_impl_276(0_()) });
}
pub fn can_hash_654() -> () {
    return ({ can_hash_impl_277(0_()) });
}
pub fn can_hash_655() -> () {
    return ({ can_hash_impl_278(0_()) });
}
pub fn can_hash_656() -> () {
    return ({ can_hash_impl_279(0_()) });
}
pub fn can_hash_657() -> () {
    return ({ can_hash_impl_280(0_()) });
}
pub fn can_hash_658() -> () {
    return ({ can_hash_impl_281(0_()) });
}
pub fn can_hash_659() -> () {
    return ({ can_hash_impl_282(0_()) });
}
pub fn can_hash_660() -> () {
    return ({ can_hash_impl_283(0_()) });
}
pub fn can_hash_661() -> () {
    return ({ can_hash_impl_284(0_()) });
}
pub fn can_hash_662() -> () {
    return ({ can_hash_impl_285(0_()) });
}
pub fn can_hash_663() -> () {
    return ({ can_hash_impl_286(0_()) });
}
pub fn can_hash_664() -> () {
    return ({ can_hash_impl_287(0_()) });
}
pub fn can_hash_665() -> () {
    return ({ can_hash_impl_288(0_()) });
}
pub fn can_hash_666() -> () {
    return ({ can_hash_impl_289(0_()) });
}
pub fn can_hash_667() -> () {
    return ({ can_hash_impl_290(0_()) });
}
pub fn can_hash_668() -> () {
    return ({ can_hash_impl_291(0_()) });
}
pub fn can_hash_669() -> () {
    return ({ can_hash_impl_292(0_()) });
}
pub fn can_hash_670() -> () {
    return ({ can_hash_impl_293(0_()) });
}
pub fn can_hash_671() -> () {
    return ({ can_hash_impl_294(0_()) });
}
pub fn can_hash_672() -> () {
    return ({ can_hash_impl_295(0_()) });
}
pub fn can_hash_673() -> () {
    return ({ can_hash_impl_296(0_()) });
}
pub fn can_hash_674() -> () {
    return ({ can_hash_impl_297(0_()) });
}
pub fn can_hash_675() -> () {
    return ({ can_hash_impl_298(0_()) });
}
pub fn can_hash_676() -> () {
    return ({ can_hash_impl_299(0_()) });
}
pub fn can_hash_677() -> () {
    return ({ can_hash_impl_300(0_()) });
}
pub fn can_hash_678() -> () {
    return ({ can_hash_impl_301(0_()) });
}
pub fn can_hash_679() -> () {
    return ({ can_hash_impl_302(0_()) });
}
pub fn can_hash_680() -> () {
    return ({ can_hash_impl_303(0_()) });
}
pub fn can_hash_681() -> () {
    return ({ can_hash_impl_304(0_()) });
}
pub fn can_hash_682() -> () {
    return ({ can_hash_impl_305(0_()) });
}
pub fn can_hash_683() -> () {
    return ({ can_hash_impl_306(0_()) });
}
pub fn can_hash_684() -> () {
    return ({ can_hash_impl_307(0_()) });
}
pub fn can_hash_685() -> () {
    return ({ can_hash_impl_308(0_()) });
}
pub fn can_hash_686() -> () {
    return ({ can_hash_impl_309(0_()) });
}
pub fn can_hash_687() -> () {
    return ({ can_hash_impl_310(0_()) });
}
pub fn can_hash_688() -> () {
    return ({ can_hash_impl_311(0_()) });
}
pub fn can_hash_689() -> () {
    return ({ can_hash_impl_312(0_()) });
}
pub fn can_hash_690() -> () {
    return ({ can_hash_impl_313(0_()) });
}
pub fn can_hash_691() -> () {
    return ({ can_hash_impl_314(0_()) });
}
pub fn can_hash_692() -> () {
    return ({ can_hash_impl_315(0_()) });
}
pub fn can_hash_693() -> () {
    return ({ can_hash_impl_316(0_()) });
}
pub fn can_hash_694() -> () {
    return ({ can_hash_impl_317(0_()) });
}
pub fn can_hash_695() -> () {
    return ({ can_hash_impl_318(0_()) });
}
pub fn can_hash_696() -> () {
    return ({ can_hash_impl_319(0_()) });
}
pub fn can_hash_697() -> () {
    return ({ can_hash_impl_320(0_()) });
}
pub fn can_hash_698() -> () {
    return ({ can_hash_impl_321(0_()) });
}
pub fn can_hash_699() -> () {
    return ({ can_hash_impl_322(0_()) });
}
pub fn can_hash_700() -> () {
    return ({ can_hash_impl_323(0_()) });
}
pub fn can_hash_701() -> () {
    return ({ can_hash_impl_324(0_()) });
}
pub fn can_hash_702() -> () {
    return ({ can_hash_impl_325(0_()) });
}
pub fn can_hash_703() -> () {
    return ({ can_hash_impl_326(0_()) });
}
pub fn can_hash_704() -> () {
    return ({ can_hash_impl_327(0_()) });
}
pub fn can_hash_705() -> () {
    return ({ can_hash_impl_328(0_()) });
}
pub fn can_hash_706() -> () {
    return ({ can_hash_impl_329(0_()) });
}
pub fn can_hash_707() -> () {
    return ({ can_hash_impl_330(0_()) });
}
pub fn can_hash_708() -> () {
    return ({ can_hash_impl_331(0_()) });
}
pub fn can_hash_709() -> () {
    return ({ can_hash_impl_332(0_()) });
}
pub fn can_hash_710() -> () {
    return ({ can_hash_impl_333(0_()) });
}
pub fn can_hash_711() -> () {
    return ({ can_hash_impl_334(0_()) });
}
pub fn can_hash_712() -> () {
    return ({ can_hash_impl_335(0_()) });
}
pub fn can_hash_713() -> () {
    return ({ can_hash_impl_336(0_()) });
}
pub fn can_hash_714() -> () {
    return ({ can_hash_impl_337(0_()) });
}
pub fn can_hash_715() -> () {
    return ({ can_hash_impl_338(0_()) });
}
pub fn can_hash_716() -> () {
    return ({ can_hash_impl_339(0_()) });
}
pub fn can_hash_717() -> () {
    return ({ can_hash_impl_340(0_()) });
}
pub fn can_hash_718() -> () {
    return ({ can_hash_impl_341(0_()) });
}
pub fn can_hash_719() -> () {
    return ({ can_hash_impl_342(0_()) });
}
pub fn can_hash_720() -> () {
    return ({ can_hash_impl_343(0_()) });
}
pub fn can_hash_721() -> () {
    return ({ can_hash_impl_344(0_()) });
}
pub fn can_hash_722() -> () {
    return ({ can_hash_impl_345(0_()) });
}
pub fn can_hash_723() -> () {
    return ({ can_hash_impl_346(0_()) });
}
pub fn can_hash_724() -> () {
    return ({ can_hash_impl_347(0_()) });
}
pub fn can_hash_725() -> () {
    return ({ can_hash_impl_348(0_()) });
}
pub fn can_hash_726() -> () {
    return ({ can_hash_impl_349(0_()) });
}
pub fn can_hash_727() -> () {
    return ({ can_hash_impl_350(0_()) });
}
pub fn can_hash_728() -> () {
    return ({ can_hash_impl_351(0_()) });
}
pub fn can_hash_729() -> () {
    return ({ can_hash_impl_352(0_()) });
}
pub fn can_hash_730() -> () {
    return ({ can_hash_impl_353(0_()) });
}
pub fn can_hash_731() -> () {
    return ({ can_hash_impl_354(0_()) });
}
pub fn can_hash_732() -> () {
    return ({ can_hash_impl_355(0_()) });
}
pub fn can_hash_733() -> () {
    return ({ can_hash_impl_356(0_()) });
}
pub fn can_hash_734() -> () {
    return ({ can_hash_impl_357(0_()) });
}
pub fn can_hash_735() -> () {
    return ({ can_hash_impl_358(0_()) });
}
pub fn can_hash_736() -> () {
    return ({ can_hash_impl_359(0_()) });
}
pub fn can_hash_737() -> () {
    return ({ can_hash_impl_360(0_()) });
}
pub fn can_hash_738() -> () {
    return ({ can_hash_impl_361(0_()) });
}
pub fn can_hash_739() -> () {
    return ({ can_hash_impl_362(0_()) });
}
pub fn can_hash_740() -> () {
    return ({ can_hash_impl_363(0_()) });
}
pub fn can_hash_741() -> () {
    return ({ can_hash_impl_364(0_()) });
}
pub fn can_hash_742() -> () {
    return ({ can_hash_impl_365(0_()) });
}
pub fn can_hash_743() -> () {
    return ({ can_hash_impl_366(0_()) });
}
pub fn can_hash_744() -> () {
    return ({ can_hash_impl_367(0_()) });
}
pub fn can_hash_745() -> () {
    return ({ can_hash_impl_368(0_()) });
}
pub fn can_hash_746() -> () {
    return ({ can_hash_impl_369(0_()) });
}
pub fn can_hash_747() -> () {
    return ({ can_hash_impl_370(0_()) });
}
pub fn can_hash_748() -> () {
    return ({ can_hash_impl_371(0_()) });
}
pub fn can_hash_749() -> () {
    return ({ can_hash_impl_372(0_()) });
}
pub fn can_hash_750() -> () {
    return ({ can_hash_impl_373(0_()) });
}
pub fn can_hash_751() -> () {
    return ({ can_hash_impl_374(0_()) });
}
pub fn can_hash_752() -> () {
    return ({ can_hash_impl_375(0_()) });
}
pub fn can_hash_753() -> () {
    return ({ can_hash_impl_376(0_()) });
}
pub fn can_hash_754() -> () {
    return ({ can_hash_impl_377(0_()) });
}
pub fn can_hash_755() -> () {
    return ({ can_hash_impl_378(0_()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_std_vector_bool__std_allocator_bool___ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_std_vector_bool__min_allocator_bool___ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_wchar_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_char8_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_char16_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_char32_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_signed_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_short_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_long_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple___int128_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_unsigned_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_unsigned_short_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_unsigned_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_unsigned_long_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_unsigned___int128_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_bool_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_float_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_double_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_long_double_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_Enum_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_EnumClass_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_void_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_const_void_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_Class_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToSimple_std_nullptr_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct ConvertibleTo_std_vector_bool__std_allocator_bool___ {
    #[offset(0)]
    #[byte_size(24)]
    pub to: (),
}
impl Default for ConvertibleTo_std_vector_bool__std_allocator_bool___ {
    fn default() -> Self {
        {
            ConvertibleTo_std_vector_bool__std_allocator_bool___ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct ConvertibleTo_std_vector_bool__min_allocator_bool___ {
    #[offset(0)]
    #[byte_size(24)]
    pub to: (),
}
impl Default for ConvertibleTo_std_vector_bool__min_allocator_bool___ {
    fn default() -> Self {
        {
            ConvertibleTo_std_vector_bool__min_allocator_bool___ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct ConvertibleTo_char_ {
    #[offset(0)]
    pub to: (),
}
impl Default for ConvertibleTo_char_ {
    fn default() -> Self {
        {
            ConvertibleTo_char_ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct ConvertibleTo_wchar_t_ {
    #[offset(0)]
    pub to: (),
}
impl Default for ConvertibleTo_wchar_t_ {
    fn default() -> Self {
        {
            ConvertibleTo_wchar_t_ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct ConvertibleTo_char8_t_ {
    #[offset(0)]
    pub to: (),
}
impl Default for ConvertibleTo_char8_t_ {
    fn default() -> Self {
        {
            ConvertibleTo_char8_t_ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(2)]
pub struct ConvertibleTo_char16_t_ {
    #[offset(0)]
    pub to: (),
}
impl Default for ConvertibleTo_char16_t_ {
    fn default() -> Self {
        {
            ConvertibleTo_char16_t_ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct ConvertibleTo_char32_t_ {
    #[offset(0)]
    pub to: (),
}
impl Default for ConvertibleTo_char32_t_ {
    fn default() -> Self {
        {
            ConvertibleTo_char32_t_ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct ConvertibleTo_signed_char_ {
    #[offset(0)]
    pub to: (),
}
impl Default for ConvertibleTo_signed_char_ {
    fn default() -> Self {
        {
            ConvertibleTo_signed_char_ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(2)]
pub struct ConvertibleTo_short_ {
    #[offset(0)]
    pub to: (),
}
impl Default for ConvertibleTo_short_ {
    fn default() -> Self {
        {
            ConvertibleTo_short_ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct ConvertibleTo_int_ {
    #[offset(0)]
    pub to: (),
}
impl Default for ConvertibleTo_int_ {
    fn default() -> Self {
        {
            ConvertibleTo_int_ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct ConvertibleTo_long_ {
    #[offset(0)]
    pub to: (),
}
impl Default for ConvertibleTo_long_ {
    fn default() -> Self {
        {
            ConvertibleTo_long_ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct ConvertibleTo_long_long_ {
    #[offset(0)]
    pub to: (),
}
impl Default for ConvertibleTo_long_long_ {
    fn default() -> Self {
        {
            ConvertibleTo_long_long_ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct ConvertibleTo___int128_ {
    #[offset(0)]
    pub to: (),
}
impl Default for ConvertibleTo___int128_ {
    fn default() -> Self {
        {
            ConvertibleTo___int128_ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct ConvertibleTo_unsigned_char_ {
    #[offset(0)]
    pub to: (),
}
impl Default for ConvertibleTo_unsigned_char_ {
    fn default() -> Self {
        {
            ConvertibleTo_unsigned_char_ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(2)]
pub struct ConvertibleTo_unsigned_short_ {
    #[offset(0)]
    pub to: (),
}
impl Default for ConvertibleTo_unsigned_short_ {
    fn default() -> Self {
        {
            ConvertibleTo_unsigned_short_ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct ConvertibleTo_unsigned_int_ {
    #[offset(0)]
    pub to: (),
}
impl Default for ConvertibleTo_unsigned_int_ {
    fn default() -> Self {
        {
            ConvertibleTo_unsigned_int_ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct ConvertibleTo_unsigned_long_ {
    #[offset(0)]
    pub to: (),
}
impl Default for ConvertibleTo_unsigned_long_ {
    fn default() -> Self {
        {
            ConvertibleTo_unsigned_long_ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct ConvertibleTo_unsigned_long_long_ {
    #[offset(0)]
    pub to: (),
}
impl Default for ConvertibleTo_unsigned_long_long_ {
    fn default() -> Self {
        {
            ConvertibleTo_unsigned_long_long_ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct ConvertibleTo_unsigned___int128_ {
    #[offset(0)]
    pub to: (),
}
impl Default for ConvertibleTo_unsigned___int128_ {
    fn default() -> Self {
        {
            ConvertibleTo_unsigned___int128_ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct ConvertibleTo_bool_ {
    #[offset(0)]
    pub to: (),
}
impl Default for ConvertibleTo_bool_ {
    fn default() -> Self {
        {
            ConvertibleTo_bool_ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct ConvertibleTo_float_ {
    #[offset(0)]
    pub to: (),
}
impl Default for ConvertibleTo_float_ {
    fn default() -> Self {
        {
            ConvertibleTo_float_ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct ConvertibleTo_double_ {
    #[offset(0)]
    pub to: (),
}
impl Default for ConvertibleTo_double_ {
    fn default() -> Self {
        {
            ConvertibleTo_double_ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct ConvertibleTo_long_double_ {
    #[offset(0)]
    #[byte_size(16)]
    pub to: (),
}
impl Default for ConvertibleTo_long_double_ {
    fn default() -> Self {
        {
            ConvertibleTo_long_double_ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct ConvertibleTo_Enum_ {
    #[offset(0)]
    pub to: Enum,
}
impl Default for ConvertibleTo_Enum_ {
    fn default() -> Self {
        {
            ConvertibleTo_Enum_ {
                to: <Enum>::default(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct ConvertibleTo_EnumClass_ {
    #[offset(0)]
    pub to: EnumClass,
}
impl Default for ConvertibleTo_EnumClass_ {
    fn default() -> Self {
        {
            ConvertibleTo_EnumClass_ {
                to: <EnumClass>::default(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct ConvertibleTo_void_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    pub to: (),
}
impl Default for ConvertibleTo_void_ptr_ {
    fn default() -> Self {
        {
            ConvertibleTo_void_ptr_ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct ConvertibleTo_const_void_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    pub to: (),
}
impl Default for ConvertibleTo_const_void_ptr_ {
    fn default() -> Self {
        {
            ConvertibleTo_const_void_ptr_ {
                to: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct ConvertibleTo_Class_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    pub to: Ptr<Class>,
}
impl Default for ConvertibleTo_Class_ptr_ {
    fn default() -> Self {
        {
            ConvertibleTo_Class_ptr_ {
                to: Ptr::<Class>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct ConvertibleTo_std_nullptr_t_ {
    #[offset(0)]
    #[byte_size(8)]
    pub to: (),
}
impl Default for ConvertibleTo_std_nullptr_t_ {
    fn default() -> Self {
        {
            ConvertibleTo_std_nullptr_t_ {
                to: unimplemented!(),
            }
        }
    }
}
pub fn test_hash_enabled_756(key: Option<Ptr<()>>) {
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
    unimplemented!();
}
pub fn test_hash_enabled_757(key: Option<Ptr<()>>) {
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
    unimplemented!();
}
pub fn test_hash_enabled_758(key: Option<Ptr<()>>) {
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
    unimplemented!();
}
pub fn test_hash_enabled_759(key: Option<Ptr<()>>) {
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
    unimplemented!();
}
pub fn test_hash_enabled_760(key: Option<Ptr<()>>) {
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
    unimplemented!();
}
pub fn test_hash_enabled_761(key: Option<Ptr<()>>) {
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
    unimplemented!();
}
pub fn test_hash_enabled_762(key: Option<Ptr<()>>) {
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
    unimplemented!();
}
pub fn test_hash_enabled_763(key: Option<Ptr<()>>) {
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
    unimplemented!();
}
pub fn test_hash_enabled_764(key: Option<Ptr<()>>) {
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
    unimplemented!();
}
pub fn test_hash_enabled_765(key: Option<Ptr<()>>) {
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
    unimplemented!();
}
pub fn test_hash_enabled_766(key: Option<Ptr<()>>) {
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
    unimplemented!();
}
pub fn test_hash_enabled_767(key: Option<Ptr<()>>) {
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
    unimplemented!();
}
pub fn test_hash_enabled_768(key: Option<Ptr<()>>) {
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
    unimplemented!();
}
pub fn test_hash_enabled_769(key: Option<Ptr<()>>) {
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
    unimplemented!();
}
pub fn test_hash_enabled_770(key: Option<Ptr<()>>) {
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
    unimplemented!();
}
pub fn test_hash_enabled_771(key: Option<Ptr<()>>) {
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
    unimplemented!();
}
pub fn test_hash_enabled_772(key: Option<Ptr<()>>) {
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
    unimplemented!();
}
pub fn test_hash_enabled_773(key: Option<Ptr<()>>) {
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
    unimplemented!();
}
pub fn test_hash_enabled_774(key: Option<Ptr<()>>) {
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
    unimplemented!();
}
pub fn test_hash_enabled_775(key: Option<Ptr<()>>) {
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
    unimplemented!();
}
pub fn test_hash_enabled_776(key: Option<Ptr<()>>) {
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
    unimplemented!();
}
pub fn test_hash_enabled_777(key: Option<Ptr<()>>) {
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
    unimplemented!();
}
pub fn test_hash_enabled_778(key: Option<Ptr<()>>) {
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
    unimplemented!();
}
pub fn test_hash_enabled_779(key: Option<Ptr<Enum>>) {
    let mut __key_default: Option<Value<Enum>> = None;
    let key: Ptr<Enum> = key.unwrap_or_else(|| {
        __key_default
            .insert(Rc::new(RefCell::new(<Enum>::default())))
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
    unimplemented!();
}
pub fn test_hash_enabled_780(key: Option<Ptr<EnumClass>>) {
    let mut __key_default: Option<Value<EnumClass>> = None;
    let key: Ptr<EnumClass> = key.unwrap_or_else(|| {
        __key_default
            .insert(Rc::new(RefCell::new(<EnumClass>::default())))
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
    unimplemented!();
}
pub fn test_hash_enabled_781(key: Option<Ptr<()>>) {
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
    unimplemented!();
}
pub fn test_hash_enabled_782(key: Option<Ptr<()>>) {
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
    unimplemented!();
}
pub fn test_hash_enabled_783(key: Option<Ptr<Ptr<Class>>>) {
    let mut __key_default: Option<Value<Ptr<Class>>> = None;
    let key: Ptr<Ptr<Class>> = key.unwrap_or_else(|| {
        __key_default
            .insert(Rc::new(RefCell::new(Ptr::<Class>::null())))
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
    unimplemented!();
}
pub fn test_hash_enabled_784(key: Option<Ptr<()>>) {
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
pub fn test_library_hash_specializations_available_785() {
    ({
        for_each_0 ( <types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool__float__double__long_double__Enum__EnumClass__void_ptr__const_void_ptr__Class_ptr__std_nullptr_t_  >::default()  , <TestHashEnabled  >::default()  , )
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_787.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_788.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_789.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_786.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_787.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_788.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_786: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_787: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_788: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_789: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_bool__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_bool__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub fn operator_eq_790(
    mut x: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_791(
    mut x: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
) -> () {
    return !({
        let _x: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_790(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn new_2(mut _a0: ()) -> Self {
        Self {
            ptr_: Ptr::<()>::null(),
        }
    }
}
impl std::cmp::PartialEq for min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_790(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_bool_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_unsigned_long_ {}
pub fn test_792() {
    ({ test_hash_enabled_756(None) });
    ({ test_hash_enabled_757(None) });
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_library_hash_specializations_available_785() });
    ({ test_792() });
    return 0_();
}
pub trait ConvertibleToSimple_Class_ptr_Impl {
    fn to_pmutClass(&self) -> Ptr<Class> {
        unimplemented!()
    }
}
impl ConvertibleToSimple_Class_ptr_Impl for Ptr<ConvertibleToSimple_Class_ptr_> {}
pub trait ConvertibleToSimple_EnumClass_Impl {
    fn to_EnumClass(&self) -> EnumClass {
        unimplemented!()
    }
}
impl ConvertibleToSimple_EnumClass_Impl for Ptr<ConvertibleToSimple_EnumClass_> {}
pub trait ConvertibleToSimple_Enum_Impl {
    fn to_Enum(&self) -> Enum {
        unimplemented!()
    }
}
impl ConvertibleToSimple_Enum_Impl for Ptr<ConvertibleToSimple_Enum_> {}
pub trait ConvertibleToSimple___int128_Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple___int128_Impl for Ptr<ConvertibleToSimple___int128_> {}
pub trait ConvertibleToSimple_bool_Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_bool_Impl for Ptr<ConvertibleToSimple_bool_> {}
pub trait ConvertibleToSimple_char16_t_Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_char16_t_Impl for Ptr<ConvertibleToSimple_char16_t_> {}
pub trait ConvertibleToSimple_char32_t_Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_char32_t_Impl for Ptr<ConvertibleToSimple_char32_t_> {}
pub trait ConvertibleToSimple_char8_t_Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_char8_t_Impl for Ptr<ConvertibleToSimple_char8_t_> {}
pub trait ConvertibleToSimple_char_Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_char_Impl for Ptr<ConvertibleToSimple_char_> {}
pub trait ConvertibleToSimple_const_void_ptr_Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_const_void_ptr_Impl for Ptr<ConvertibleToSimple_const_void_ptr_> {}
pub trait ConvertibleToSimple_double_Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_double_Impl for Ptr<ConvertibleToSimple_double_> {}
pub trait ConvertibleToSimple_float_Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_float_Impl for Ptr<ConvertibleToSimple_float_> {}
pub trait ConvertibleToSimple_int_Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_int_Impl for Ptr<ConvertibleToSimple_int_> {}
pub trait ConvertibleToSimple_long_Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_long_Impl for Ptr<ConvertibleToSimple_long_> {}
pub trait ConvertibleToSimple_long_double_Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_long_double_Impl for Ptr<ConvertibleToSimple_long_double_> {}
pub trait ConvertibleToSimple_long_long_Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_long_long_Impl for Ptr<ConvertibleToSimple_long_long_> {}
pub trait ConvertibleToSimple_short_Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_short_Impl for Ptr<ConvertibleToSimple_short_> {}
pub trait ConvertibleToSimple_signed_char_Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_signed_char_Impl for Ptr<ConvertibleToSimple_signed_char_> {}
pub trait ConvertibleToSimple_std_nullptr_t_Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_std_nullptr_t_Impl for Ptr<ConvertibleToSimple_std_nullptr_t_> {}
pub trait ConvertibleToSimple_std_vector_bool__min_allocator_bool___Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_std_vector_bool__min_allocator_bool___Impl
    for Ptr<ConvertibleToSimple_std_vector_bool__min_allocator_bool___>
{
}
pub trait ConvertibleToSimple_std_vector_bool__std_allocator_bool___Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_std_vector_bool__std_allocator_bool___Impl
    for Ptr<ConvertibleToSimple_std_vector_bool__std_allocator_bool___>
{
}
pub trait ConvertibleToSimple_unsigned___int128_Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_unsigned___int128_Impl for Ptr<ConvertibleToSimple_unsigned___int128_> {}
pub trait ConvertibleToSimple_unsigned_char_Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_unsigned_char_Impl for Ptr<ConvertibleToSimple_unsigned_char_> {}
pub trait ConvertibleToSimple_unsigned_int_Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_unsigned_int_Impl for Ptr<ConvertibleToSimple_unsigned_int_> {}
pub trait ConvertibleToSimple_unsigned_long_Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_unsigned_long_Impl for Ptr<ConvertibleToSimple_unsigned_long_> {}
pub trait ConvertibleToSimple_unsigned_long_long_Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_unsigned_long_long_Impl for Ptr<ConvertibleToSimple_unsigned_long_long_> {}
pub trait ConvertibleToSimple_unsigned_short_Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_unsigned_short_Impl for Ptr<ConvertibleToSimple_unsigned_short_> {}
pub trait ConvertibleToSimple_void_ptr_Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_void_ptr_Impl for Ptr<ConvertibleToSimple_void_ptr_> {}
pub trait ConvertibleToSimple_wchar_t_Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl ConvertibleToSimple_wchar_t_Impl for Ptr<ConvertibleToSimple_wchar_t_> {}
pub trait ConvertibleTo_Class_ptr_Impl {
    fn to_pmutpmutClass_1(&self) -> Ptr<Ptr<Class>> {
        unimplemented!()
    }
    fn to_pconstpmutClass_2(&self) -> Ptr<Ptr<Class>> {
        unimplemented!()
    }
    fn to_pmutpmutClass_3(&self) -> Ptr<Ptr<Class>> {
        unimplemented!()
    }
    fn to_pconstpmutClass_4(&self) -> Ptr<Ptr<Class>> {
        unimplemented!()
    }
}
impl ConvertibleTo_Class_ptr_Impl for Ptr<ConvertibleTo_Class_ptr_> {}
pub trait ConvertibleTo_EnumClass_Impl {
    fn to_pmutEnumClass_1(&self) -> Ptr<EnumClass> {
        unimplemented!()
    }
    fn to_pconstEnumClass_2(&self) -> Ptr<EnumClass> {
        unimplemented!()
    }
    fn to_pmutEnumClass_3(&self) -> Ptr<EnumClass> {
        unimplemented!()
    }
    fn to_pconstEnumClass_4(&self) -> Ptr<EnumClass> {
        unimplemented!()
    }
}
impl ConvertibleTo_EnumClass_Impl for Ptr<ConvertibleTo_EnumClass_> {}
pub trait ConvertibleTo_Enum_Impl {
    fn to_pmutEnum_1(&self) -> Ptr<Enum> {
        unimplemented!()
    }
    fn to_pconstEnum_2(&self) -> Ptr<Enum> {
        unimplemented!()
    }
    fn to_pmutEnum_3(&self) -> Ptr<Enum> {
        unimplemented!()
    }
    fn to_pconstEnum_4(&self) -> Ptr<Enum> {
        unimplemented!()
    }
}
impl ConvertibleTo_Enum_Impl for Ptr<ConvertibleTo_Enum_> {}
pub trait ConvertibleTo___int128_Impl {
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
impl ConvertibleTo___int128_Impl for Ptr<ConvertibleTo___int128_> {}
pub trait ConvertibleTo_bool_Impl {
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
impl ConvertibleTo_bool_Impl for Ptr<ConvertibleTo_bool_> {}
pub trait ConvertibleTo_char16_t_Impl {
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
impl ConvertibleTo_char16_t_Impl for Ptr<ConvertibleTo_char16_t_> {}
pub trait ConvertibleTo_char32_t_Impl {
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
impl ConvertibleTo_char32_t_Impl for Ptr<ConvertibleTo_char32_t_> {}
pub trait ConvertibleTo_char8_t_Impl {
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
impl ConvertibleTo_char8_t_Impl for Ptr<ConvertibleTo_char8_t_> {}
pub trait ConvertibleTo_char_Impl {
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
impl ConvertibleTo_char_Impl for Ptr<ConvertibleTo_char_> {}
pub trait ConvertibleTo_const_void_ptr_Impl {
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
impl ConvertibleTo_const_void_ptr_Impl for Ptr<ConvertibleTo_const_void_ptr_> {}
pub trait ConvertibleTo_double_Impl {
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
impl ConvertibleTo_double_Impl for Ptr<ConvertibleTo_double_> {}
pub trait ConvertibleTo_float_Impl {
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
impl ConvertibleTo_float_Impl for Ptr<ConvertibleTo_float_> {}
pub trait ConvertibleTo_int_Impl {
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
impl ConvertibleTo_int_Impl for Ptr<ConvertibleTo_int_> {}
pub trait ConvertibleTo_long_Impl {
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
impl ConvertibleTo_long_Impl for Ptr<ConvertibleTo_long_> {}
pub trait ConvertibleTo_long_double_Impl {
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
impl ConvertibleTo_long_double_Impl for Ptr<ConvertibleTo_long_double_> {}
pub trait ConvertibleTo_long_long_Impl {
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
impl ConvertibleTo_long_long_Impl for Ptr<ConvertibleTo_long_long_> {}
pub trait ConvertibleTo_short_Impl {
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
impl ConvertibleTo_short_Impl for Ptr<ConvertibleTo_short_> {}
pub trait ConvertibleTo_signed_char_Impl {
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
impl ConvertibleTo_signed_char_Impl for Ptr<ConvertibleTo_signed_char_> {}
pub trait ConvertibleTo_std_nullptr_t_Impl {
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
impl ConvertibleTo_std_nullptr_t_Impl for Ptr<ConvertibleTo_std_nullptr_t_> {}
pub trait ConvertibleTo_std_vector_bool__min_allocator_bool___Impl {
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
impl ConvertibleTo_std_vector_bool__min_allocator_bool___Impl
    for Ptr<ConvertibleTo_std_vector_bool__min_allocator_bool___>
{
}
pub trait ConvertibleTo_std_vector_bool__std_allocator_bool___Impl {
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
impl ConvertibleTo_std_vector_bool__std_allocator_bool___Impl
    for Ptr<ConvertibleTo_std_vector_bool__std_allocator_bool___>
{
}
pub trait ConvertibleTo_unsigned___int128_Impl {
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
impl ConvertibleTo_unsigned___int128_Impl for Ptr<ConvertibleTo_unsigned___int128_> {}
pub trait ConvertibleTo_unsigned_char_Impl {
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
impl ConvertibleTo_unsigned_char_Impl for Ptr<ConvertibleTo_unsigned_char_> {}
pub trait ConvertibleTo_unsigned_int_Impl {
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
impl ConvertibleTo_unsigned_int_Impl for Ptr<ConvertibleTo_unsigned_int_> {}
pub trait ConvertibleTo_unsigned_long_Impl {
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
impl ConvertibleTo_unsigned_long_Impl for Ptr<ConvertibleTo_unsigned_long_> {}
pub trait ConvertibleTo_unsigned_long_long_Impl {
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
impl ConvertibleTo_unsigned_long_long_Impl for Ptr<ConvertibleTo_unsigned_long_long_> {}
pub trait ConvertibleTo_unsigned_short_Impl {
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
impl ConvertibleTo_unsigned_short_Impl for Ptr<ConvertibleTo_unsigned_short_> {}
pub trait ConvertibleTo_void_ptr_Impl {
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
impl ConvertibleTo_void_ptr_Impl for Ptr<ConvertibleTo_void_ptr_> {}
pub trait ConvertibleTo_wchar_t_Impl {
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
impl ConvertibleTo_wchar_t_Impl for Ptr<ConvertibleTo_wchar_t_> {}
pub trait TestHashEnabledImpl {
    fn operator_call_1_char(&self);
    fn operator_call_1_wchar_t(&self);
    fn operator_call_1_char8_t(&self);
    fn operator_call_1_char16_t(&self);
    fn operator_call_1_char32_t(&self);
    fn operator_call_1_signed_char(&self);
    fn operator_call_1_short(&self);
    fn operator_call_1_int(&self);
    fn operator_call_1_long(&self);
    fn operator_call_1_long_long(&self);
    fn operator_call_1___int128(&self);
    fn operator_call_1_unsigned_char(&self);
    fn operator_call_1_unsigned_short(&self);
    fn operator_call_1_unsigned_int(&self);
    fn operator_call_1_unsigned_long(&self);
    fn operator_call_1_unsigned_long_long(&self);
    fn operator_call_1_unsigned___int128(&self);
    fn operator_call_1__Bool(&self);
    fn operator_call_1_float(&self);
    fn operator_call_1_double(&self);
    fn operator_call_1_long_double(&self);
    fn operator_call_1_enum_Enum(&self);
    fn operator_call_1_enum_EnumClass(&self);
    fn operator_call_1_void_ptr(&self);
    fn operator_call_1_const_void_ptr(&self);
    fn operator_call_1_struct_Class_ptr(&self);
    fn operator_call_1_nullptr_t(&self);
}
impl TestHashEnabledImpl for Ptr<TestHashEnabled> {
    fn operator_call_1_char(&self) {
        ({ test_hash_enabled_758(None) });
    }
    fn operator_call_1_wchar_t(&self) {
        ({ test_hash_enabled_759(None) });
    }
    fn operator_call_1_char8_t(&self) {
        ({ test_hash_enabled_760(None) });
    }
    fn operator_call_1_char16_t(&self) {
        ({ test_hash_enabled_761(None) });
    }
    fn operator_call_1_char32_t(&self) {
        ({ test_hash_enabled_762(None) });
    }
    fn operator_call_1_signed_char(&self) {
        ({ test_hash_enabled_763(None) });
    }
    fn operator_call_1_short(&self) {
        ({ test_hash_enabled_764(None) });
    }
    fn operator_call_1_int(&self) {
        ({ test_hash_enabled_765(None) });
    }
    fn operator_call_1_long(&self) {
        ({ test_hash_enabled_766(None) });
    }
    fn operator_call_1_long_long(&self) {
        ({ test_hash_enabled_767(None) });
    }
    fn operator_call_1___int128(&self) {
        ({ test_hash_enabled_768(None) });
    }
    fn operator_call_1_unsigned_char(&self) {
        ({ test_hash_enabled_769(None) });
    }
    fn operator_call_1_unsigned_short(&self) {
        ({ test_hash_enabled_770(None) });
    }
    fn operator_call_1_unsigned_int(&self) {
        ({ test_hash_enabled_771(None) });
    }
    fn operator_call_1_unsigned_long(&self) {
        ({ test_hash_enabled_772(None) });
    }
    fn operator_call_1_unsigned_long_long(&self) {
        ({ test_hash_enabled_773(None) });
    }
    fn operator_call_1_unsigned___int128(&self) {
        ({ test_hash_enabled_774(None) });
    }
    fn operator_call_1__Bool(&self) {
        ({ test_hash_enabled_775(None) });
    }
    fn operator_call_1_float(&self) {
        ({ test_hash_enabled_776(None) });
    }
    fn operator_call_1_double(&self) {
        ({ test_hash_enabled_777(None) });
    }
    fn operator_call_1_long_double(&self) {
        ({ test_hash_enabled_778(None) });
    }
    fn operator_call_1_enum_Enum(&self) {
        ({ test_hash_enabled_779(None) });
    }
    fn operator_call_1_enum_EnumClass(&self) {
        ({ test_hash_enabled_780(None) });
    }
    fn operator_call_1_void_ptr(&self) {
        ({ test_hash_enabled_781(None) });
    }
    fn operator_call_1_const_void_ptr(&self) {
        ({ test_hash_enabled_782(None) });
    }
    fn operator_call_1_struct_Class_ptr(&self) {
        ({ test_hash_enabled_783(None) });
    }
    fn operator_call_1_nullptr_t(&self) {
        ({ test_hash_enabled_784(None) });
    }
}
pub trait min_allocator_bool_Impl {
    fn allocate(&self, mut n: ()) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_bool__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_bool_Impl for Ptr<min_allocator_bool_> {}
pub trait min_allocator_unsigned_long_Impl {
    fn allocate(
        &self,
        n: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__;
    fn deallocate(
        &self,
        p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        n: (),
    );
}
impl min_allocator_unsigned_long_Impl for Ptr<min_allocator_unsigned_long_> {
    fn allocate(
        &self,
        mut n: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        return min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub trait min_pointer_bool__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_bool__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_bool__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_bool__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_bool__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_bool__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_bool__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__>;
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: (),
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        n: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__;
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__>
{
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__Impl :: operator_add_assign ( &tmp .as_pointer()  , n  , )
        });
        return (*tmp.borrow()).clone();
    }
}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_786.with(|_| ());
    let _ = alloc_count_787.with(|_| ());
    let _ = dealloc_count_788.with(|_| ());
    let _ = disable_default_constructor_789.with(|_| ());
}
