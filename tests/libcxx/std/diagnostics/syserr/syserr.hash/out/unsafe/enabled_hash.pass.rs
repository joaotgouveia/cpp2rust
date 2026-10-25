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
pub struct types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool__float__double__long_double__Enum__EnumClass__void_ptr__const_void_ptr__Class_ptr__std_nullptr_t_
{}
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
pub unsafe fn for_each_0(
    mut _a0 : types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool__float__double__long_double__Enum__EnumClass__void_ptr__const_void_ptr__Class_ptr__std_nullptr_t_,
    mut f: TestHashEnabled,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { TestHashEnabled::operator_call_1_char(&f) });
            0_()
        };
        let _arg1: () = {
            (unsafe { TestHashEnabled::operator_call_1_wchar_t(&f) });
            0_()
        };
        let _arg2: () = {
            (unsafe { TestHashEnabled::operator_call_1_char8_t(&f) });
            0_()
        };
        let _arg3: () = {
            (unsafe { TestHashEnabled::operator_call_1_char16_t(&f) });
            0_()
        };
        let _arg4: () = {
            (unsafe { TestHashEnabled::operator_call_1_char32_t(&f) });
            0_()
        };
        let _arg5: () = {
            (unsafe { TestHashEnabled::operator_call_1_signed_char(&f) });
            0_()
        };
        let _arg6: () = {
            (unsafe { TestHashEnabled::operator_call_1_short(&f) });
            0_()
        };
        let _arg7: () = {
            (unsafe { TestHashEnabled::operator_call_1_int(&f) });
            0_()
        };
        let _arg8: () = {
            (unsafe { TestHashEnabled::operator_call_1_long(&f) });
            0_()
        };
        let _arg9: () = {
            (unsafe { TestHashEnabled::operator_call_1_long_long(&f) });
            0_()
        };
        let _arg10: () = {
            (unsafe { TestHashEnabled::operator_call_1___int128(&f) });
            0_()
        };
        let _arg11: () = {
            (unsafe { TestHashEnabled::operator_call_1_unsigned_char(&f) });
            0_()
        };
        let _arg12: () = {
            (unsafe { TestHashEnabled::operator_call_1_unsigned_short(&f) });
            0_()
        };
        let _arg13: () = {
            (unsafe { TestHashEnabled::operator_call_1_unsigned_int(&f) });
            0_()
        };
        let _arg14: () = {
            (unsafe { TestHashEnabled::operator_call_1_unsigned_long(&f) });
            0_()
        };
        let _arg15: () = {
            (unsafe { TestHashEnabled::operator_call_1_unsigned_long_long(&f) });
            0_()
        };
        let _arg16: () = {
            (unsafe { TestHashEnabled::operator_call_1_unsigned___int128(&f) });
            0_()
        };
        let _arg17: () = {
            (unsafe { TestHashEnabled::operator_call_1__Bool(&f) });
            0_()
        };
        let _arg18: () = {
            (unsafe { TestHashEnabled::operator_call_1_float(&f) });
            0_()
        };
        let _arg19: () = {
            (unsafe { TestHashEnabled::operator_call_1_double(&f) });
            0_()
        };
        let _arg20: () = {
            (unsafe { TestHashEnabled::operator_call_1_long_double(&f) });
            0_()
        };
        let _arg21: () = {
            (unsafe { TestHashEnabled::operator_call_1_enum_Enum(&f) });
            0_()
        };
        let _arg22: () = {
            (unsafe { TestHashEnabled::operator_call_1_enum_EnumClass(&f) });
            0_()
        };
        let _arg23: () = {
            (unsafe { TestHashEnabled::operator_call_1_void_ptr(&f) });
            0_()
        };
        let _arg24: () = {
            (unsafe { TestHashEnabled::operator_call_1_const_void_ptr(&f) });
            0_()
        };
        let _arg25: () = {
            (unsafe { TestHashEnabled::operator_call_1_struct_Class_ptr(&f) });
            0_()
        };
        let _arg26: () = {
            (unsafe { TestHashEnabled::operator_call_1_nullptr_t(&f) });
            0_()
        };
        swallow_1(
            _arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7, _arg8, _arg9, _arg10, _arg11,
            _arg12, _arg13, _arg14, _arg15, _arg16, _arg17, _arg18, _arg19, _arg20, _arg21, _arg22,
            _arg23, _arg24, _arg25, _arg26,
        )
    });
}
pub unsafe fn swallow_1(
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
    return unimplemented!();
}
pub unsafe fn can_hash_impl_72(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_73(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_74(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_75(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_76(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_77(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_78(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_79(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_80(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_81(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_82(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_83(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_84(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_85(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_86(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_87(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_88(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_89(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_90(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_91(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_92(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_93(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_94(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_95(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_96(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_97(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_98(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_99(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_100(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_101(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_102(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_103(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_104(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_105(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_106(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_107(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_108(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_109(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_110(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_111(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_112(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_113(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_114(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_115(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_116(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_117(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_118(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_119(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_120(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_121(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_122(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_123(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_124(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_125(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_126(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_127(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_128(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_129(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_130(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_131(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_132(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_133(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_134(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_135(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_136(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_137(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_138(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_139(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_140(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_141(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_142(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_143(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_144(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_145(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_146(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_147(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_148(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_149(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_150(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_151(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_152(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_153(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_154(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_155(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_156(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_157(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_158(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_159(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_160(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_161(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_162(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_163(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_164(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_165(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_166(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_167(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_168(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_169(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_170(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_171(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_172(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_173(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_174(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_175(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_176(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_177(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_178(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_179(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_180(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_181(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_182(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_183(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_184(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_185(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_186(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_187(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_188(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_189(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_190(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_191(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_192(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_193(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_194(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_195(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_196(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_197(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_198(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_199(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_200(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_201(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_202(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_203(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_204(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_205(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_206(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_207(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_208(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_209(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_210(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_211(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_212(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_213(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_214(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_215(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_216(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_217(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_218(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_219(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_220(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_221(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_222(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_223(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_224(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_225(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_226(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_227(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_228(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_229(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_230(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_231(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_232(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_233(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_234(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_235(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_236(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_237(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_238(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_239(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_240(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_241(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_242(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_243(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_244(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_245(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_246(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_247(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_248(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_249(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_250(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_251(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_252(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_253(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_254(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_255(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_256(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_257(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_258(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_259(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_260(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_261(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_262(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_263(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_264(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_265(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_266(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_267(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_268(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_269(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_270(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_271(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_272(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_273(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_274(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_275(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_276(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_277(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_278(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_279(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_280(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_281(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_282(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_283(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_284(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_285(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_286(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_287(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_288(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_289(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_290(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_291(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_292(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_293(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_294(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_295(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_296(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_297(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_298(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_299(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_300(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_301(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_302(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_303(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_304(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_305(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_306(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_307(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_308(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_309(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_310(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_311(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_312(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_313(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_314(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_315(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_316(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_317(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_318(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_319(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_320(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_321(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_322(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_323(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_324(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_325(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_326(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_327(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_328(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_329(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_330(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_331(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_332(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_333(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_334(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_335(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_336(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_337(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_338(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_339(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_340(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_341(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_342(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_343(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_344(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_345(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_346(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_347(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_348(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_349(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_350(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_351(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_352(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_353(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_354(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_355(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_356(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_357(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_358(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_359(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_360(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_361(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_362(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_363(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_364(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_365(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_366(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_367(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_368(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_369(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_370(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_371(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_372(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_373(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_374(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_375(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_376(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_377(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_impl_378(mut _a0: ()) -> () {
    return unimplemented!();
}
pub unsafe fn can_hash_379() -> () {
    return (unsafe { can_hash_impl_2(0_()) });
}
pub unsafe fn can_hash_380() -> () {
    return (unsafe { can_hash_impl_3(0_()) });
}
pub unsafe fn can_hash_381() -> () {
    return (unsafe { can_hash_impl_4(0_()) });
}
pub unsafe fn can_hash_382() -> () {
    return (unsafe { can_hash_impl_5(0_()) });
}
pub unsafe fn can_hash_383() -> () {
    return (unsafe { can_hash_impl_6(0_()) });
}
pub unsafe fn can_hash_384() -> () {
    return (unsafe { can_hash_impl_7(0_()) });
}
pub unsafe fn can_hash_385() -> () {
    return (unsafe { can_hash_impl_8(0_()) });
}
pub unsafe fn can_hash_386() -> () {
    return (unsafe { can_hash_impl_9(0_()) });
}
pub unsafe fn can_hash_387() -> () {
    return (unsafe { can_hash_impl_10(0_()) });
}
pub unsafe fn can_hash_388() -> () {
    return (unsafe { can_hash_impl_11(0_()) });
}
pub unsafe fn can_hash_389() -> () {
    return (unsafe { can_hash_impl_12(0_()) });
}
pub unsafe fn can_hash_390() -> () {
    return (unsafe { can_hash_impl_13(0_()) });
}
pub unsafe fn can_hash_391() -> () {
    return (unsafe { can_hash_impl_14(0_()) });
}
pub unsafe fn can_hash_392() -> () {
    return (unsafe { can_hash_impl_15(0_()) });
}
pub unsafe fn can_hash_393() -> () {
    return (unsafe { can_hash_impl_16(0_()) });
}
pub unsafe fn can_hash_394() -> () {
    return (unsafe { can_hash_impl_17(0_()) });
}
pub unsafe fn can_hash_395() -> () {
    return (unsafe { can_hash_impl_18(0_()) });
}
pub unsafe fn can_hash_396() -> () {
    return (unsafe { can_hash_impl_19(0_()) });
}
pub unsafe fn can_hash_397() -> () {
    return (unsafe { can_hash_impl_20(0_()) });
}
pub unsafe fn can_hash_398() -> () {
    return (unsafe { can_hash_impl_21(0_()) });
}
pub unsafe fn can_hash_399() -> () {
    return (unsafe { can_hash_impl_22(0_()) });
}
pub unsafe fn can_hash_400() -> () {
    return (unsafe { can_hash_impl_23(0_()) });
}
pub unsafe fn can_hash_401() -> () {
    return (unsafe { can_hash_impl_24(0_()) });
}
pub unsafe fn can_hash_402() -> () {
    return (unsafe { can_hash_impl_25(0_()) });
}
pub unsafe fn can_hash_403() -> () {
    return (unsafe { can_hash_impl_26(0_()) });
}
pub unsafe fn can_hash_404() -> () {
    return (unsafe { can_hash_impl_27(0_()) });
}
pub unsafe fn can_hash_405() -> () {
    return (unsafe { can_hash_impl_28(0_()) });
}
pub unsafe fn can_hash_406() -> () {
    return (unsafe { can_hash_impl_29(0_()) });
}
pub unsafe fn can_hash_407() -> () {
    return (unsafe { can_hash_impl_30(0_()) });
}
pub unsafe fn can_hash_408() -> () {
    return (unsafe { can_hash_impl_31(0_()) });
}
pub unsafe fn can_hash_409() -> () {
    return (unsafe { can_hash_impl_32(0_()) });
}
pub unsafe fn can_hash_410() -> () {
    return (unsafe { can_hash_impl_33(0_()) });
}
pub unsafe fn can_hash_411() -> () {
    return (unsafe { can_hash_impl_34(0_()) });
}
pub unsafe fn can_hash_412() -> () {
    return (unsafe { can_hash_impl_35(0_()) });
}
pub unsafe fn can_hash_413() -> () {
    return (unsafe { can_hash_impl_36(0_()) });
}
pub unsafe fn can_hash_414() -> () {
    return (unsafe { can_hash_impl_37(0_()) });
}
pub unsafe fn can_hash_415() -> () {
    return (unsafe { can_hash_impl_38(0_()) });
}
pub unsafe fn can_hash_416() -> () {
    return (unsafe { can_hash_impl_39(0_()) });
}
pub unsafe fn can_hash_417() -> () {
    return (unsafe { can_hash_impl_40(0_()) });
}
pub unsafe fn can_hash_418() -> () {
    return (unsafe { can_hash_impl_41(0_()) });
}
pub unsafe fn can_hash_419() -> () {
    return (unsafe { can_hash_impl_42(0_()) });
}
pub unsafe fn can_hash_420() -> () {
    return (unsafe { can_hash_impl_43(0_()) });
}
pub unsafe fn can_hash_421() -> () {
    return (unsafe { can_hash_impl_44(0_()) });
}
pub unsafe fn can_hash_422() -> () {
    return (unsafe { can_hash_impl_45(0_()) });
}
pub unsafe fn can_hash_423() -> () {
    return (unsafe { can_hash_impl_46(0_()) });
}
pub unsafe fn can_hash_424() -> () {
    return (unsafe { can_hash_impl_47(0_()) });
}
pub unsafe fn can_hash_425() -> () {
    return (unsafe { can_hash_impl_48(0_()) });
}
pub unsafe fn can_hash_426() -> () {
    return (unsafe { can_hash_impl_49(0_()) });
}
pub unsafe fn can_hash_427() -> () {
    return (unsafe { can_hash_impl_50(0_()) });
}
pub unsafe fn can_hash_428() -> () {
    return (unsafe { can_hash_impl_51(0_()) });
}
pub unsafe fn can_hash_429() -> () {
    return (unsafe { can_hash_impl_52(0_()) });
}
pub unsafe fn can_hash_430() -> () {
    return (unsafe { can_hash_impl_53(0_()) });
}
pub unsafe fn can_hash_431() -> () {
    return (unsafe { can_hash_impl_54(0_()) });
}
pub unsafe fn can_hash_432() -> () {
    return (unsafe { can_hash_impl_55(0_()) });
}
pub unsafe fn can_hash_433() -> () {
    return (unsafe { can_hash_impl_56(0_()) });
}
pub unsafe fn can_hash_434() -> () {
    return (unsafe { can_hash_impl_57(0_()) });
}
pub unsafe fn can_hash_435() -> () {
    return (unsafe { can_hash_impl_58(0_()) });
}
pub unsafe fn can_hash_436() -> () {
    return (unsafe { can_hash_impl_59(0_()) });
}
pub unsafe fn can_hash_437() -> () {
    return (unsafe { can_hash_impl_60(0_()) });
}
pub unsafe fn can_hash_438() -> () {
    return (unsafe { can_hash_impl_61(0_()) });
}
pub unsafe fn can_hash_439() -> () {
    return (unsafe { can_hash_impl_62(0_()) });
}
pub unsafe fn can_hash_440() -> () {
    return (unsafe { can_hash_impl_63(0_()) });
}
pub unsafe fn can_hash_441() -> () {
    return (unsafe { can_hash_impl_64(0_()) });
}
pub unsafe fn can_hash_442() -> () {
    return (unsafe { can_hash_impl_65(0_()) });
}
pub unsafe fn can_hash_443() -> () {
    return (unsafe { can_hash_impl_66(0_()) });
}
pub unsafe fn can_hash_444() -> () {
    return (unsafe { can_hash_impl_67(0_()) });
}
pub unsafe fn can_hash_445() -> () {
    return (unsafe { can_hash_impl_68(0_()) });
}
pub unsafe fn can_hash_446() -> () {
    return (unsafe { can_hash_impl_69(0_()) });
}
pub unsafe fn can_hash_447() -> () {
    return (unsafe { can_hash_impl_70(0_()) });
}
pub unsafe fn can_hash_448() -> () {
    return (unsafe { can_hash_impl_71(0_()) });
}
pub unsafe fn can_hash_449() -> () {
    return (unsafe { can_hash_impl_72(0_()) });
}
pub unsafe fn can_hash_450() -> () {
    return (unsafe { can_hash_impl_73(0_()) });
}
pub unsafe fn can_hash_451() -> () {
    return (unsafe { can_hash_impl_74(0_()) });
}
pub unsafe fn can_hash_452() -> () {
    return (unsafe { can_hash_impl_75(0_()) });
}
pub unsafe fn can_hash_453() -> () {
    return (unsafe { can_hash_impl_76(0_()) });
}
pub unsafe fn can_hash_454() -> () {
    return (unsafe { can_hash_impl_77(0_()) });
}
pub unsafe fn can_hash_455() -> () {
    return (unsafe { can_hash_impl_78(0_()) });
}
pub unsafe fn can_hash_456() -> () {
    return (unsafe { can_hash_impl_79(0_()) });
}
pub unsafe fn can_hash_457() -> () {
    return (unsafe { can_hash_impl_80(0_()) });
}
pub unsafe fn can_hash_458() -> () {
    return (unsafe { can_hash_impl_81(0_()) });
}
pub unsafe fn can_hash_459() -> () {
    return (unsafe { can_hash_impl_82(0_()) });
}
pub unsafe fn can_hash_460() -> () {
    return (unsafe { can_hash_impl_83(0_()) });
}
pub unsafe fn can_hash_461() -> () {
    return (unsafe { can_hash_impl_84(0_()) });
}
pub unsafe fn can_hash_462() -> () {
    return (unsafe { can_hash_impl_85(0_()) });
}
pub unsafe fn can_hash_463() -> () {
    return (unsafe { can_hash_impl_86(0_()) });
}
pub unsafe fn can_hash_464() -> () {
    return (unsafe { can_hash_impl_87(0_()) });
}
pub unsafe fn can_hash_465() -> () {
    return (unsafe { can_hash_impl_88(0_()) });
}
pub unsafe fn can_hash_466() -> () {
    return (unsafe { can_hash_impl_89(0_()) });
}
pub unsafe fn can_hash_467() -> () {
    return (unsafe { can_hash_impl_90(0_()) });
}
pub unsafe fn can_hash_468() -> () {
    return (unsafe { can_hash_impl_91(0_()) });
}
pub unsafe fn can_hash_469() -> () {
    return (unsafe { can_hash_impl_92(0_()) });
}
pub unsafe fn can_hash_470() -> () {
    return (unsafe { can_hash_impl_93(0_()) });
}
pub unsafe fn can_hash_471() -> () {
    return (unsafe { can_hash_impl_94(0_()) });
}
pub unsafe fn can_hash_472() -> () {
    return (unsafe { can_hash_impl_95(0_()) });
}
pub unsafe fn can_hash_473() -> () {
    return (unsafe { can_hash_impl_96(0_()) });
}
pub unsafe fn can_hash_474() -> () {
    return (unsafe { can_hash_impl_97(0_()) });
}
pub unsafe fn can_hash_475() -> () {
    return (unsafe { can_hash_impl_98(0_()) });
}
pub unsafe fn can_hash_476() -> () {
    return (unsafe { can_hash_impl_99(0_()) });
}
pub unsafe fn can_hash_477() -> () {
    return (unsafe { can_hash_impl_100(0_()) });
}
pub unsafe fn can_hash_478() -> () {
    return (unsafe { can_hash_impl_101(0_()) });
}
pub unsafe fn can_hash_479() -> () {
    return (unsafe { can_hash_impl_102(0_()) });
}
pub unsafe fn can_hash_480() -> () {
    return (unsafe { can_hash_impl_103(0_()) });
}
pub unsafe fn can_hash_481() -> () {
    return (unsafe { can_hash_impl_104(0_()) });
}
pub unsafe fn can_hash_482() -> () {
    return (unsafe { can_hash_impl_105(0_()) });
}
pub unsafe fn can_hash_483() -> () {
    return (unsafe { can_hash_impl_106(0_()) });
}
pub unsafe fn can_hash_484() -> () {
    return (unsafe { can_hash_impl_107(0_()) });
}
pub unsafe fn can_hash_485() -> () {
    return (unsafe { can_hash_impl_108(0_()) });
}
pub unsafe fn can_hash_486() -> () {
    return (unsafe { can_hash_impl_109(0_()) });
}
pub unsafe fn can_hash_487() -> () {
    return (unsafe { can_hash_impl_110(0_()) });
}
pub unsafe fn can_hash_488() -> () {
    return (unsafe { can_hash_impl_111(0_()) });
}
pub unsafe fn can_hash_489() -> () {
    return (unsafe { can_hash_impl_112(0_()) });
}
pub unsafe fn can_hash_490() -> () {
    return (unsafe { can_hash_impl_113(0_()) });
}
pub unsafe fn can_hash_491() -> () {
    return (unsafe { can_hash_impl_114(0_()) });
}
pub unsafe fn can_hash_492() -> () {
    return (unsafe { can_hash_impl_115(0_()) });
}
pub unsafe fn can_hash_493() -> () {
    return (unsafe { can_hash_impl_116(0_()) });
}
pub unsafe fn can_hash_494() -> () {
    return (unsafe { can_hash_impl_117(0_()) });
}
pub unsafe fn can_hash_495() -> () {
    return (unsafe { can_hash_impl_118(0_()) });
}
pub unsafe fn can_hash_496() -> () {
    return (unsafe { can_hash_impl_119(0_()) });
}
pub unsafe fn can_hash_497() -> () {
    return (unsafe { can_hash_impl_120(0_()) });
}
pub unsafe fn can_hash_498() -> () {
    return (unsafe { can_hash_impl_121(0_()) });
}
pub unsafe fn can_hash_499() -> () {
    return (unsafe { can_hash_impl_122(0_()) });
}
pub unsafe fn can_hash_500() -> () {
    return (unsafe { can_hash_impl_123(0_()) });
}
pub unsafe fn can_hash_501() -> () {
    return (unsafe { can_hash_impl_124(0_()) });
}
pub unsafe fn can_hash_502() -> () {
    return (unsafe { can_hash_impl_125(0_()) });
}
pub unsafe fn can_hash_503() -> () {
    return (unsafe { can_hash_impl_126(0_()) });
}
pub unsafe fn can_hash_504() -> () {
    return (unsafe { can_hash_impl_127(0_()) });
}
pub unsafe fn can_hash_505() -> () {
    return (unsafe { can_hash_impl_128(0_()) });
}
pub unsafe fn can_hash_506() -> () {
    return (unsafe { can_hash_impl_129(0_()) });
}
pub unsafe fn can_hash_507() -> () {
    return (unsafe { can_hash_impl_130(0_()) });
}
pub unsafe fn can_hash_508() -> () {
    return (unsafe { can_hash_impl_131(0_()) });
}
pub unsafe fn can_hash_509() -> () {
    return (unsafe { can_hash_impl_132(0_()) });
}
pub unsafe fn can_hash_510() -> () {
    return (unsafe { can_hash_impl_133(0_()) });
}
pub unsafe fn can_hash_511() -> () {
    return (unsafe { can_hash_impl_134(0_()) });
}
pub unsafe fn can_hash_512() -> () {
    return (unsafe { can_hash_impl_135(0_()) });
}
pub unsafe fn can_hash_513() -> () {
    return (unsafe { can_hash_impl_136(0_()) });
}
pub unsafe fn can_hash_514() -> () {
    return (unsafe { can_hash_impl_137(0_()) });
}
pub unsafe fn can_hash_515() -> () {
    return (unsafe { can_hash_impl_138(0_()) });
}
pub unsafe fn can_hash_516() -> () {
    return (unsafe { can_hash_impl_139(0_()) });
}
pub unsafe fn can_hash_517() -> () {
    return (unsafe { can_hash_impl_140(0_()) });
}
pub unsafe fn can_hash_518() -> () {
    return (unsafe { can_hash_impl_141(0_()) });
}
pub unsafe fn can_hash_519() -> () {
    return (unsafe { can_hash_impl_142(0_()) });
}
pub unsafe fn can_hash_520() -> () {
    return (unsafe { can_hash_impl_143(0_()) });
}
pub unsafe fn can_hash_521() -> () {
    return (unsafe { can_hash_impl_144(0_()) });
}
pub unsafe fn can_hash_522() -> () {
    return (unsafe { can_hash_impl_145(0_()) });
}
pub unsafe fn can_hash_523() -> () {
    return (unsafe { can_hash_impl_146(0_()) });
}
pub unsafe fn can_hash_524() -> () {
    return (unsafe { can_hash_impl_147(0_()) });
}
pub unsafe fn can_hash_525() -> () {
    return (unsafe { can_hash_impl_148(0_()) });
}
pub unsafe fn can_hash_526() -> () {
    return (unsafe { can_hash_impl_149(0_()) });
}
pub unsafe fn can_hash_527() -> () {
    return (unsafe { can_hash_impl_150(0_()) });
}
pub unsafe fn can_hash_528() -> () {
    return (unsafe { can_hash_impl_151(0_()) });
}
pub unsafe fn can_hash_529() -> () {
    return (unsafe { can_hash_impl_152(0_()) });
}
pub unsafe fn can_hash_530() -> () {
    return (unsafe { can_hash_impl_153(0_()) });
}
pub unsafe fn can_hash_531() -> () {
    return (unsafe { can_hash_impl_154(0_()) });
}
pub unsafe fn can_hash_532() -> () {
    return (unsafe { can_hash_impl_155(0_()) });
}
pub unsafe fn can_hash_533() -> () {
    return (unsafe { can_hash_impl_156(0_()) });
}
pub unsafe fn can_hash_534() -> () {
    return (unsafe { can_hash_impl_157(0_()) });
}
pub unsafe fn can_hash_535() -> () {
    return (unsafe { can_hash_impl_158(0_()) });
}
pub unsafe fn can_hash_536() -> () {
    return (unsafe { can_hash_impl_159(0_()) });
}
pub unsafe fn can_hash_537() -> () {
    return (unsafe { can_hash_impl_160(0_()) });
}
pub unsafe fn can_hash_538() -> () {
    return (unsafe { can_hash_impl_161(0_()) });
}
pub unsafe fn can_hash_539() -> () {
    return (unsafe { can_hash_impl_162(0_()) });
}
pub unsafe fn can_hash_540() -> () {
    return (unsafe { can_hash_impl_163(0_()) });
}
pub unsafe fn can_hash_541() -> () {
    return (unsafe { can_hash_impl_164(0_()) });
}
pub unsafe fn can_hash_542() -> () {
    return (unsafe { can_hash_impl_165(0_()) });
}
pub unsafe fn can_hash_543() -> () {
    return (unsafe { can_hash_impl_166(0_()) });
}
pub unsafe fn can_hash_544() -> () {
    return (unsafe { can_hash_impl_167(0_()) });
}
pub unsafe fn can_hash_545() -> () {
    return (unsafe { can_hash_impl_168(0_()) });
}
pub unsafe fn can_hash_546() -> () {
    return (unsafe { can_hash_impl_169(0_()) });
}
pub unsafe fn can_hash_547() -> () {
    return (unsafe { can_hash_impl_170(0_()) });
}
pub unsafe fn can_hash_548() -> () {
    return (unsafe { can_hash_impl_171(0_()) });
}
pub unsafe fn can_hash_549() -> () {
    return (unsafe { can_hash_impl_172(0_()) });
}
pub unsafe fn can_hash_550() -> () {
    return (unsafe { can_hash_impl_173(0_()) });
}
pub unsafe fn can_hash_551() -> () {
    return (unsafe { can_hash_impl_174(0_()) });
}
pub unsafe fn can_hash_552() -> () {
    return (unsafe { can_hash_impl_175(0_()) });
}
pub unsafe fn can_hash_553() -> () {
    return (unsafe { can_hash_impl_176(0_()) });
}
pub unsafe fn can_hash_554() -> () {
    return (unsafe { can_hash_impl_177(0_()) });
}
pub unsafe fn can_hash_555() -> () {
    return (unsafe { can_hash_impl_178(0_()) });
}
pub unsafe fn can_hash_556() -> () {
    return (unsafe { can_hash_impl_179(0_()) });
}
pub unsafe fn can_hash_557() -> () {
    return (unsafe { can_hash_impl_180(0_()) });
}
pub unsafe fn can_hash_558() -> () {
    return (unsafe { can_hash_impl_181(0_()) });
}
pub unsafe fn can_hash_559() -> () {
    return (unsafe { can_hash_impl_182(0_()) });
}
pub unsafe fn can_hash_560() -> () {
    return (unsafe { can_hash_impl_183(0_()) });
}
pub unsafe fn can_hash_561() -> () {
    return (unsafe { can_hash_impl_184(0_()) });
}
pub unsafe fn can_hash_562() -> () {
    return (unsafe { can_hash_impl_185(0_()) });
}
pub unsafe fn can_hash_563() -> () {
    return (unsafe { can_hash_impl_186(0_()) });
}
pub unsafe fn can_hash_564() -> () {
    return (unsafe { can_hash_impl_187(0_()) });
}
pub unsafe fn can_hash_565() -> () {
    return (unsafe { can_hash_impl_188(0_()) });
}
pub unsafe fn can_hash_566() -> () {
    return (unsafe { can_hash_impl_189(0_()) });
}
pub unsafe fn can_hash_567() -> () {
    return (unsafe { can_hash_impl_190(0_()) });
}
pub unsafe fn can_hash_568() -> () {
    return (unsafe { can_hash_impl_191(0_()) });
}
pub unsafe fn can_hash_569() -> () {
    return (unsafe { can_hash_impl_192(0_()) });
}
pub unsafe fn can_hash_570() -> () {
    return (unsafe { can_hash_impl_193(0_()) });
}
pub unsafe fn can_hash_571() -> () {
    return (unsafe { can_hash_impl_194(0_()) });
}
pub unsafe fn can_hash_572() -> () {
    return (unsafe { can_hash_impl_195(0_()) });
}
pub unsafe fn can_hash_573() -> () {
    return (unsafe { can_hash_impl_196(0_()) });
}
pub unsafe fn can_hash_574() -> () {
    return (unsafe { can_hash_impl_197(0_()) });
}
pub unsafe fn can_hash_575() -> () {
    return (unsafe { can_hash_impl_198(0_()) });
}
pub unsafe fn can_hash_576() -> () {
    return (unsafe { can_hash_impl_199(0_()) });
}
pub unsafe fn can_hash_577() -> () {
    return (unsafe { can_hash_impl_200(0_()) });
}
pub unsafe fn can_hash_578() -> () {
    return (unsafe { can_hash_impl_201(0_()) });
}
pub unsafe fn can_hash_579() -> () {
    return (unsafe { can_hash_impl_202(0_()) });
}
pub unsafe fn can_hash_580() -> () {
    return (unsafe { can_hash_impl_203(0_()) });
}
pub unsafe fn can_hash_581() -> () {
    return (unsafe { can_hash_impl_204(0_()) });
}
pub unsafe fn can_hash_582() -> () {
    return (unsafe { can_hash_impl_205(0_()) });
}
pub unsafe fn can_hash_583() -> () {
    return (unsafe { can_hash_impl_206(0_()) });
}
pub unsafe fn can_hash_584() -> () {
    return (unsafe { can_hash_impl_207(0_()) });
}
pub unsafe fn can_hash_585() -> () {
    return (unsafe { can_hash_impl_208(0_()) });
}
pub unsafe fn can_hash_586() -> () {
    return (unsafe { can_hash_impl_209(0_()) });
}
pub unsafe fn can_hash_587() -> () {
    return (unsafe { can_hash_impl_210(0_()) });
}
pub unsafe fn can_hash_588() -> () {
    return (unsafe { can_hash_impl_211(0_()) });
}
pub unsafe fn can_hash_589() -> () {
    return (unsafe { can_hash_impl_212(0_()) });
}
pub unsafe fn can_hash_590() -> () {
    return (unsafe { can_hash_impl_213(0_()) });
}
pub unsafe fn can_hash_591() -> () {
    return (unsafe { can_hash_impl_214(0_()) });
}
pub unsafe fn can_hash_592() -> () {
    return (unsafe { can_hash_impl_215(0_()) });
}
pub unsafe fn can_hash_593() -> () {
    return (unsafe { can_hash_impl_216(0_()) });
}
pub unsafe fn can_hash_594() -> () {
    return (unsafe { can_hash_impl_217(0_()) });
}
pub unsafe fn can_hash_595() -> () {
    return (unsafe { can_hash_impl_218(0_()) });
}
pub unsafe fn can_hash_596() -> () {
    return (unsafe { can_hash_impl_219(0_()) });
}
pub unsafe fn can_hash_597() -> () {
    return (unsafe { can_hash_impl_220(0_()) });
}
pub unsafe fn can_hash_598() -> () {
    return (unsafe { can_hash_impl_221(0_()) });
}
pub unsafe fn can_hash_599() -> () {
    return (unsafe { can_hash_impl_222(0_()) });
}
pub unsafe fn can_hash_600() -> () {
    return (unsafe { can_hash_impl_223(0_()) });
}
pub unsafe fn can_hash_601() -> () {
    return (unsafe { can_hash_impl_224(0_()) });
}
pub unsafe fn can_hash_602() -> () {
    return (unsafe { can_hash_impl_225(0_()) });
}
pub unsafe fn can_hash_603() -> () {
    return (unsafe { can_hash_impl_226(0_()) });
}
pub unsafe fn can_hash_604() -> () {
    return (unsafe { can_hash_impl_227(0_()) });
}
pub unsafe fn can_hash_605() -> () {
    return (unsafe { can_hash_impl_228(0_()) });
}
pub unsafe fn can_hash_606() -> () {
    return (unsafe { can_hash_impl_229(0_()) });
}
pub unsafe fn can_hash_607() -> () {
    return (unsafe { can_hash_impl_230(0_()) });
}
pub unsafe fn can_hash_608() -> () {
    return (unsafe { can_hash_impl_231(0_()) });
}
pub unsafe fn can_hash_609() -> () {
    return (unsafe { can_hash_impl_232(0_()) });
}
pub unsafe fn can_hash_610() -> () {
    return (unsafe { can_hash_impl_233(0_()) });
}
pub unsafe fn can_hash_611() -> () {
    return (unsafe { can_hash_impl_234(0_()) });
}
pub unsafe fn can_hash_612() -> () {
    return (unsafe { can_hash_impl_235(0_()) });
}
pub unsafe fn can_hash_613() -> () {
    return (unsafe { can_hash_impl_236(0_()) });
}
pub unsafe fn can_hash_614() -> () {
    return (unsafe { can_hash_impl_237(0_()) });
}
pub unsafe fn can_hash_615() -> () {
    return (unsafe { can_hash_impl_238(0_()) });
}
pub unsafe fn can_hash_616() -> () {
    return (unsafe { can_hash_impl_239(0_()) });
}
pub unsafe fn can_hash_617() -> () {
    return (unsafe { can_hash_impl_240(0_()) });
}
pub unsafe fn can_hash_618() -> () {
    return (unsafe { can_hash_impl_241(0_()) });
}
pub unsafe fn can_hash_619() -> () {
    return (unsafe { can_hash_impl_242(0_()) });
}
pub unsafe fn can_hash_620() -> () {
    return (unsafe { can_hash_impl_243(0_()) });
}
pub unsafe fn can_hash_621() -> () {
    return (unsafe { can_hash_impl_244(0_()) });
}
pub unsafe fn can_hash_622() -> () {
    return (unsafe { can_hash_impl_245(0_()) });
}
pub unsafe fn can_hash_623() -> () {
    return (unsafe { can_hash_impl_246(0_()) });
}
pub unsafe fn can_hash_624() -> () {
    return (unsafe { can_hash_impl_247(0_()) });
}
pub unsafe fn can_hash_625() -> () {
    return (unsafe { can_hash_impl_248(0_()) });
}
pub unsafe fn can_hash_626() -> () {
    return (unsafe { can_hash_impl_249(0_()) });
}
pub unsafe fn can_hash_627() -> () {
    return (unsafe { can_hash_impl_250(0_()) });
}
pub unsafe fn can_hash_628() -> () {
    return (unsafe { can_hash_impl_251(0_()) });
}
pub unsafe fn can_hash_629() -> () {
    return (unsafe { can_hash_impl_252(0_()) });
}
pub unsafe fn can_hash_630() -> () {
    return (unsafe { can_hash_impl_253(0_()) });
}
pub unsafe fn can_hash_631() -> () {
    return (unsafe { can_hash_impl_254(0_()) });
}
pub unsafe fn can_hash_632() -> () {
    return (unsafe { can_hash_impl_255(0_()) });
}
pub unsafe fn can_hash_633() -> () {
    return (unsafe { can_hash_impl_256(0_()) });
}
pub unsafe fn can_hash_634() -> () {
    return (unsafe { can_hash_impl_257(0_()) });
}
pub unsafe fn can_hash_635() -> () {
    return (unsafe { can_hash_impl_258(0_()) });
}
pub unsafe fn can_hash_636() -> () {
    return (unsafe { can_hash_impl_259(0_()) });
}
pub unsafe fn can_hash_637() -> () {
    return (unsafe { can_hash_impl_260(0_()) });
}
pub unsafe fn can_hash_638() -> () {
    return (unsafe { can_hash_impl_261(0_()) });
}
pub unsafe fn can_hash_639() -> () {
    return (unsafe { can_hash_impl_262(0_()) });
}
pub unsafe fn can_hash_640() -> () {
    return (unsafe { can_hash_impl_263(0_()) });
}
pub unsafe fn can_hash_641() -> () {
    return (unsafe { can_hash_impl_264(0_()) });
}
pub unsafe fn can_hash_642() -> () {
    return (unsafe { can_hash_impl_265(0_()) });
}
pub unsafe fn can_hash_643() -> () {
    return (unsafe { can_hash_impl_266(0_()) });
}
pub unsafe fn can_hash_644() -> () {
    return (unsafe { can_hash_impl_267(0_()) });
}
pub unsafe fn can_hash_645() -> () {
    return (unsafe { can_hash_impl_268(0_()) });
}
pub unsafe fn can_hash_646() -> () {
    return (unsafe { can_hash_impl_269(0_()) });
}
pub unsafe fn can_hash_647() -> () {
    return (unsafe { can_hash_impl_270(0_()) });
}
pub unsafe fn can_hash_648() -> () {
    return (unsafe { can_hash_impl_271(0_()) });
}
pub unsafe fn can_hash_649() -> () {
    return (unsafe { can_hash_impl_272(0_()) });
}
pub unsafe fn can_hash_650() -> () {
    return (unsafe { can_hash_impl_273(0_()) });
}
pub unsafe fn can_hash_651() -> () {
    return (unsafe { can_hash_impl_274(0_()) });
}
pub unsafe fn can_hash_652() -> () {
    return (unsafe { can_hash_impl_275(0_()) });
}
pub unsafe fn can_hash_653() -> () {
    return (unsafe { can_hash_impl_276(0_()) });
}
pub unsafe fn can_hash_654() -> () {
    return (unsafe { can_hash_impl_277(0_()) });
}
pub unsafe fn can_hash_655() -> () {
    return (unsafe { can_hash_impl_278(0_()) });
}
pub unsafe fn can_hash_656() -> () {
    return (unsafe { can_hash_impl_279(0_()) });
}
pub unsafe fn can_hash_657() -> () {
    return (unsafe { can_hash_impl_280(0_()) });
}
pub unsafe fn can_hash_658() -> () {
    return (unsafe { can_hash_impl_281(0_()) });
}
pub unsafe fn can_hash_659() -> () {
    return (unsafe { can_hash_impl_282(0_()) });
}
pub unsafe fn can_hash_660() -> () {
    return (unsafe { can_hash_impl_283(0_()) });
}
pub unsafe fn can_hash_661() -> () {
    return (unsafe { can_hash_impl_284(0_()) });
}
pub unsafe fn can_hash_662() -> () {
    return (unsafe { can_hash_impl_285(0_()) });
}
pub unsafe fn can_hash_663() -> () {
    return (unsafe { can_hash_impl_286(0_()) });
}
pub unsafe fn can_hash_664() -> () {
    return (unsafe { can_hash_impl_287(0_()) });
}
pub unsafe fn can_hash_665() -> () {
    return (unsafe { can_hash_impl_288(0_()) });
}
pub unsafe fn can_hash_666() -> () {
    return (unsafe { can_hash_impl_289(0_()) });
}
pub unsafe fn can_hash_667() -> () {
    return (unsafe { can_hash_impl_290(0_()) });
}
pub unsafe fn can_hash_668() -> () {
    return (unsafe { can_hash_impl_291(0_()) });
}
pub unsafe fn can_hash_669() -> () {
    return (unsafe { can_hash_impl_292(0_()) });
}
pub unsafe fn can_hash_670() -> () {
    return (unsafe { can_hash_impl_293(0_()) });
}
pub unsafe fn can_hash_671() -> () {
    return (unsafe { can_hash_impl_294(0_()) });
}
pub unsafe fn can_hash_672() -> () {
    return (unsafe { can_hash_impl_295(0_()) });
}
pub unsafe fn can_hash_673() -> () {
    return (unsafe { can_hash_impl_296(0_()) });
}
pub unsafe fn can_hash_674() -> () {
    return (unsafe { can_hash_impl_297(0_()) });
}
pub unsafe fn can_hash_675() -> () {
    return (unsafe { can_hash_impl_298(0_()) });
}
pub unsafe fn can_hash_676() -> () {
    return (unsafe { can_hash_impl_299(0_()) });
}
pub unsafe fn can_hash_677() -> () {
    return (unsafe { can_hash_impl_300(0_()) });
}
pub unsafe fn can_hash_678() -> () {
    return (unsafe { can_hash_impl_301(0_()) });
}
pub unsafe fn can_hash_679() -> () {
    return (unsafe { can_hash_impl_302(0_()) });
}
pub unsafe fn can_hash_680() -> () {
    return (unsafe { can_hash_impl_303(0_()) });
}
pub unsafe fn can_hash_681() -> () {
    return (unsafe { can_hash_impl_304(0_()) });
}
pub unsafe fn can_hash_682() -> () {
    return (unsafe { can_hash_impl_305(0_()) });
}
pub unsafe fn can_hash_683() -> () {
    return (unsafe { can_hash_impl_306(0_()) });
}
pub unsafe fn can_hash_684() -> () {
    return (unsafe { can_hash_impl_307(0_()) });
}
pub unsafe fn can_hash_685() -> () {
    return (unsafe { can_hash_impl_308(0_()) });
}
pub unsafe fn can_hash_686() -> () {
    return (unsafe { can_hash_impl_309(0_()) });
}
pub unsafe fn can_hash_687() -> () {
    return (unsafe { can_hash_impl_310(0_()) });
}
pub unsafe fn can_hash_688() -> () {
    return (unsafe { can_hash_impl_311(0_()) });
}
pub unsafe fn can_hash_689() -> () {
    return (unsafe { can_hash_impl_312(0_()) });
}
pub unsafe fn can_hash_690() -> () {
    return (unsafe { can_hash_impl_313(0_()) });
}
pub unsafe fn can_hash_691() -> () {
    return (unsafe { can_hash_impl_314(0_()) });
}
pub unsafe fn can_hash_692() -> () {
    return (unsafe { can_hash_impl_315(0_()) });
}
pub unsafe fn can_hash_693() -> () {
    return (unsafe { can_hash_impl_316(0_()) });
}
pub unsafe fn can_hash_694() -> () {
    return (unsafe { can_hash_impl_317(0_()) });
}
pub unsafe fn can_hash_695() -> () {
    return (unsafe { can_hash_impl_318(0_()) });
}
pub unsafe fn can_hash_696() -> () {
    return (unsafe { can_hash_impl_319(0_()) });
}
pub unsafe fn can_hash_697() -> () {
    return (unsafe { can_hash_impl_320(0_()) });
}
pub unsafe fn can_hash_698() -> () {
    return (unsafe { can_hash_impl_321(0_()) });
}
pub unsafe fn can_hash_699() -> () {
    return (unsafe { can_hash_impl_322(0_()) });
}
pub unsafe fn can_hash_700() -> () {
    return (unsafe { can_hash_impl_323(0_()) });
}
pub unsafe fn can_hash_701() -> () {
    return (unsafe { can_hash_impl_324(0_()) });
}
pub unsafe fn can_hash_702() -> () {
    return (unsafe { can_hash_impl_325(0_()) });
}
pub unsafe fn can_hash_703() -> () {
    return (unsafe { can_hash_impl_326(0_()) });
}
pub unsafe fn can_hash_704() -> () {
    return (unsafe { can_hash_impl_327(0_()) });
}
pub unsafe fn can_hash_705() -> () {
    return (unsafe { can_hash_impl_328(0_()) });
}
pub unsafe fn can_hash_706() -> () {
    return (unsafe { can_hash_impl_329(0_()) });
}
pub unsafe fn can_hash_707() -> () {
    return (unsafe { can_hash_impl_330(0_()) });
}
pub unsafe fn can_hash_708() -> () {
    return (unsafe { can_hash_impl_331(0_()) });
}
pub unsafe fn can_hash_709() -> () {
    return (unsafe { can_hash_impl_332(0_()) });
}
pub unsafe fn can_hash_710() -> () {
    return (unsafe { can_hash_impl_333(0_()) });
}
pub unsafe fn can_hash_711() -> () {
    return (unsafe { can_hash_impl_334(0_()) });
}
pub unsafe fn can_hash_712() -> () {
    return (unsafe { can_hash_impl_335(0_()) });
}
pub unsafe fn can_hash_713() -> () {
    return (unsafe { can_hash_impl_336(0_()) });
}
pub unsafe fn can_hash_714() -> () {
    return (unsafe { can_hash_impl_337(0_()) });
}
pub unsafe fn can_hash_715() -> () {
    return (unsafe { can_hash_impl_338(0_()) });
}
pub unsafe fn can_hash_716() -> () {
    return (unsafe { can_hash_impl_339(0_()) });
}
pub unsafe fn can_hash_717() -> () {
    return (unsafe { can_hash_impl_340(0_()) });
}
pub unsafe fn can_hash_718() -> () {
    return (unsafe { can_hash_impl_341(0_()) });
}
pub unsafe fn can_hash_719() -> () {
    return (unsafe { can_hash_impl_342(0_()) });
}
pub unsafe fn can_hash_720() -> () {
    return (unsafe { can_hash_impl_343(0_()) });
}
pub unsafe fn can_hash_721() -> () {
    return (unsafe { can_hash_impl_344(0_()) });
}
pub unsafe fn can_hash_722() -> () {
    return (unsafe { can_hash_impl_345(0_()) });
}
pub unsafe fn can_hash_723() -> () {
    return (unsafe { can_hash_impl_346(0_()) });
}
pub unsafe fn can_hash_724() -> () {
    return (unsafe { can_hash_impl_347(0_()) });
}
pub unsafe fn can_hash_725() -> () {
    return (unsafe { can_hash_impl_348(0_()) });
}
pub unsafe fn can_hash_726() -> () {
    return (unsafe { can_hash_impl_349(0_()) });
}
pub unsafe fn can_hash_727() -> () {
    return (unsafe { can_hash_impl_350(0_()) });
}
pub unsafe fn can_hash_728() -> () {
    return (unsafe { can_hash_impl_351(0_()) });
}
pub unsafe fn can_hash_729() -> () {
    return (unsafe { can_hash_impl_352(0_()) });
}
pub unsafe fn can_hash_730() -> () {
    return (unsafe { can_hash_impl_353(0_()) });
}
pub unsafe fn can_hash_731() -> () {
    return (unsafe { can_hash_impl_354(0_()) });
}
pub unsafe fn can_hash_732() -> () {
    return (unsafe { can_hash_impl_355(0_()) });
}
pub unsafe fn can_hash_733() -> () {
    return (unsafe { can_hash_impl_356(0_()) });
}
pub unsafe fn can_hash_734() -> () {
    return (unsafe { can_hash_impl_357(0_()) });
}
pub unsafe fn can_hash_735() -> () {
    return (unsafe { can_hash_impl_358(0_()) });
}
pub unsafe fn can_hash_736() -> () {
    return (unsafe { can_hash_impl_359(0_()) });
}
pub unsafe fn can_hash_737() -> () {
    return (unsafe { can_hash_impl_360(0_()) });
}
pub unsafe fn can_hash_738() -> () {
    return (unsafe { can_hash_impl_361(0_()) });
}
pub unsafe fn can_hash_739() -> () {
    return (unsafe { can_hash_impl_362(0_()) });
}
pub unsafe fn can_hash_740() -> () {
    return (unsafe { can_hash_impl_363(0_()) });
}
pub unsafe fn can_hash_741() -> () {
    return (unsafe { can_hash_impl_364(0_()) });
}
pub unsafe fn can_hash_742() -> () {
    return (unsafe { can_hash_impl_365(0_()) });
}
pub unsafe fn can_hash_743() -> () {
    return (unsafe { can_hash_impl_366(0_()) });
}
pub unsafe fn can_hash_744() -> () {
    return (unsafe { can_hash_impl_367(0_()) });
}
pub unsafe fn can_hash_745() -> () {
    return (unsafe { can_hash_impl_368(0_()) });
}
pub unsafe fn can_hash_746() -> () {
    return (unsafe { can_hash_impl_369(0_()) });
}
pub unsafe fn can_hash_747() -> () {
    return (unsafe { can_hash_impl_370(0_()) });
}
pub unsafe fn can_hash_748() -> () {
    return (unsafe { can_hash_impl_371(0_()) });
}
pub unsafe fn can_hash_749() -> () {
    return (unsafe { can_hash_impl_372(0_()) });
}
pub unsafe fn can_hash_750() -> () {
    return (unsafe { can_hash_impl_373(0_()) });
}
pub unsafe fn can_hash_751() -> () {
    return (unsafe { can_hash_impl_374(0_()) });
}
pub unsafe fn can_hash_752() -> () {
    return (unsafe { can_hash_impl_375(0_()) });
}
pub unsafe fn can_hash_753() -> () {
    return (unsafe { can_hash_impl_376(0_()) });
}
pub unsafe fn can_hash_754() -> () {
    return (unsafe { can_hash_impl_377(0_()) });
}
pub unsafe fn can_hash_755() -> () {
    return (unsafe { can_hash_impl_378(0_()) });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_std_error_code_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_std_error_condition_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_char_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_wchar_t_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_char8_t_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_char16_t_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_char32_t_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_signed_char_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_short_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_long_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_long_long_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple___int128_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_unsigned_char_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_unsigned_short_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_unsigned_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_unsigned_long_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_unsigned_long_long_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_unsigned___int128_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_bool_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_float_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_double_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_long_double_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_Enum_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_EnumClass_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_void_ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_const_void_ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_Class_ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleToSimple_std_nullptr_t_ {}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_std_error_code_ {
    pub to: (),
}
impl Default for ConvertibleTo_std_error_code_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_std_error_code_ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_std_error_condition_ {
    pub to: (),
}
impl Default for ConvertibleTo_std_error_condition_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_std_error_condition_ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_char_ {
    pub to: (),
}
impl Default for ConvertibleTo_char_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_char_ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_wchar_t_ {
    pub to: (),
}
impl Default for ConvertibleTo_wchar_t_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_wchar_t_ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_char8_t_ {
    pub to: (),
}
impl Default for ConvertibleTo_char8_t_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_char8_t_ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_char16_t_ {
    pub to: (),
}
impl Default for ConvertibleTo_char16_t_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_char16_t_ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_char32_t_ {
    pub to: (),
}
impl Default for ConvertibleTo_char32_t_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_char32_t_ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_signed_char_ {
    pub to: (),
}
impl Default for ConvertibleTo_signed_char_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_signed_char_ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_short_ {
    pub to: (),
}
impl Default for ConvertibleTo_short_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_short_ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_int_ {
    pub to: (),
}
impl Default for ConvertibleTo_int_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_int_ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_long_ {
    pub to: (),
}
impl Default for ConvertibleTo_long_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_long_ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_long_long_ {
    pub to: (),
}
impl Default for ConvertibleTo_long_long_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_long_long_ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo___int128_ {
    pub to: (),
}
impl Default for ConvertibleTo___int128_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo___int128_ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_unsigned_char_ {
    pub to: (),
}
impl Default for ConvertibleTo_unsigned_char_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_unsigned_char_ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_unsigned_short_ {
    pub to: (),
}
impl Default for ConvertibleTo_unsigned_short_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_unsigned_short_ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_unsigned_int_ {
    pub to: (),
}
impl Default for ConvertibleTo_unsigned_int_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_unsigned_int_ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_unsigned_long_ {
    pub to: (),
}
impl Default for ConvertibleTo_unsigned_long_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_unsigned_long_ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_unsigned_long_long_ {
    pub to: (),
}
impl Default for ConvertibleTo_unsigned_long_long_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_unsigned_long_long_ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_unsigned___int128_ {
    pub to: (),
}
impl Default for ConvertibleTo_unsigned___int128_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_unsigned___int128_ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_bool_ {
    pub to: (),
}
impl Default for ConvertibleTo_bool_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_bool_ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_float_ {
    pub to: (),
}
impl Default for ConvertibleTo_float_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_float_ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_double_ {
    pub to: (),
}
impl Default for ConvertibleTo_double_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_double_ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_long_double_ {
    pub to: (),
}
impl Default for ConvertibleTo_long_double_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_long_double_ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_Enum_ {
    pub to: Enum,
}
impl Default for ConvertibleTo_Enum_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_Enum_ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_EnumClass_ {
    pub to: EnumClass,
}
impl Default for ConvertibleTo_EnumClass_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_EnumClass_ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_void_ptr_ {
    pub to: (),
}
impl Default for ConvertibleTo_void_ptr_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_void_ptr_ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_const_void_ptr_ {
    pub to: (),
}
impl Default for ConvertibleTo_const_void_ptr_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_const_void_ptr_ {
                to: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_Class_ptr_ {
    pub to: *mut Class,
}
impl Default for ConvertibleTo_Class_ptr_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_Class_ptr_ {
                to: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleTo_std_nullptr_t_ {
    pub to: (),
}
impl Default for ConvertibleTo_std_nullptr_t_ {
    fn default() -> Self {
        unsafe {
            ConvertibleTo_std_nullptr_t_ {
                to: unimplemented!(),
            }
        }
    }
}
pub unsafe fn test_hash_enabled_756(key: Option<*const ()>) {
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
pub unsafe fn test_hash_enabled_757(key: Option<*const ()>) {
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
pub unsafe fn test_hash_enabled_758(key: Option<*const ()>) {
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
pub unsafe fn test_hash_enabled_759(key: Option<*const ()>) {
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
pub unsafe fn test_hash_enabled_760(key: Option<*const ()>) {
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
pub unsafe fn test_hash_enabled_761(key: Option<*const ()>) {
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
pub unsafe fn test_hash_enabled_762(key: Option<*const ()>) {
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
pub unsafe fn test_hash_enabled_763(key: Option<*const ()>) {
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
pub unsafe fn test_hash_enabled_764(key: Option<*const ()>) {
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
pub unsafe fn test_hash_enabled_765(key: Option<*const ()>) {
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
pub unsafe fn test_hash_enabled_766(key: Option<*const ()>) {
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
pub unsafe fn test_hash_enabled_767(key: Option<*const ()>) {
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
pub unsafe fn test_hash_enabled_768(key: Option<*const ()>) {
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
pub unsafe fn test_hash_enabled_769(key: Option<*const ()>) {
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
pub unsafe fn test_hash_enabled_770(key: Option<*const ()>) {
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
pub unsafe fn test_hash_enabled_771(key: Option<*const ()>) {
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
pub unsafe fn test_hash_enabled_772(key: Option<*const ()>) {
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
pub unsafe fn test_hash_enabled_773(key: Option<*const ()>) {
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
pub unsafe fn test_hash_enabled_774(key: Option<*const ()>) {
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
pub unsafe fn test_hash_enabled_775(key: Option<*const ()>) {
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
pub unsafe fn test_hash_enabled_776(key: Option<*const ()>) {
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
pub unsafe fn test_hash_enabled_777(key: Option<*const ()>) {
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
pub unsafe fn test_hash_enabled_778(key: Option<*const ()>) {
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
pub unsafe fn test_hash_enabled_779(key: Option<*const Enum>) {
    let mut __key_default: Option<Enum> = None;
    let mut key: *const Enum =
        key.unwrap_or_else(|| unsafe { __key_default.insert(unimplemented!()) as *const Enum });
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
pub unsafe fn test_hash_enabled_780(key: Option<*const EnumClass>) {
    let mut __key_default: Option<EnumClass> = None;
    let mut key: *const EnumClass = key
        .unwrap_or_else(|| unsafe { __key_default.insert(unimplemented!()) as *const EnumClass });
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
pub unsafe fn test_hash_enabled_781(key: Option<*const ()>) {
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
pub unsafe fn test_hash_enabled_782(key: Option<*const ()>) {
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
pub unsafe fn test_hash_enabled_783(key: Option<*const *mut Class>) {
    let mut __key_default: Option<*mut Class> = None;
    let mut key: *const *mut Class = key.unwrap_or_else(|| unsafe {
        __key_default.insert(std::ptr::null_mut()) as *const *mut Class
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
    let h: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_hash_enabled_784(key: Option<*const ()>) {
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
pub type Enum = ();
pub type EnumClass = ();
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Class {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestHashEnabled {}
impl TestHashEnabled {
    pub unsafe fn operator_call_1_char(&self) {
        (unsafe { test_hash_enabled_758(None) });
    }
    pub unsafe fn operator_call_1_wchar_t(&self) {
        (unsafe { test_hash_enabled_759(None) });
    }
    pub unsafe fn operator_call_1_char8_t(&self) {
        (unsafe { test_hash_enabled_760(None) });
    }
    pub unsafe fn operator_call_1_char16_t(&self) {
        (unsafe { test_hash_enabled_761(None) });
    }
    pub unsafe fn operator_call_1_char32_t(&self) {
        (unsafe { test_hash_enabled_762(None) });
    }
    pub unsafe fn operator_call_1_signed_char(&self) {
        (unsafe { test_hash_enabled_763(None) });
    }
    pub unsafe fn operator_call_1_short(&self) {
        (unsafe { test_hash_enabled_764(None) });
    }
    pub unsafe fn operator_call_1_int(&self) {
        (unsafe { test_hash_enabled_765(None) });
    }
    pub unsafe fn operator_call_1_long(&self) {
        (unsafe { test_hash_enabled_766(None) });
    }
    pub unsafe fn operator_call_1_long_long(&self) {
        (unsafe { test_hash_enabled_767(None) });
    }
    pub unsafe fn operator_call_1___int128(&self) {
        (unsafe { test_hash_enabled_768(None) });
    }
    pub unsafe fn operator_call_1_unsigned_char(&self) {
        (unsafe { test_hash_enabled_769(None) });
    }
    pub unsafe fn operator_call_1_unsigned_short(&self) {
        (unsafe { test_hash_enabled_770(None) });
    }
    pub unsafe fn operator_call_1_unsigned_int(&self) {
        (unsafe { test_hash_enabled_771(None) });
    }
    pub unsafe fn operator_call_1_unsigned_long(&self) {
        (unsafe { test_hash_enabled_772(None) });
    }
    pub unsafe fn operator_call_1_unsigned_long_long(&self) {
        (unsafe { test_hash_enabled_773(None) });
    }
    pub unsafe fn operator_call_1_unsigned___int128(&self) {
        (unsafe { test_hash_enabled_774(None) });
    }
    pub unsafe fn operator_call_1__Bool(&self) {
        (unsafe { test_hash_enabled_775(None) });
    }
    pub unsafe fn operator_call_1_float(&self) {
        (unsafe { test_hash_enabled_776(None) });
    }
    pub unsafe fn operator_call_1_double(&self) {
        (unsafe { test_hash_enabled_777(None) });
    }
    pub unsafe fn operator_call_1_long_double(&self) {
        (unsafe { test_hash_enabled_778(None) });
    }
    pub unsafe fn operator_call_1_enum_Enum(&self) {
        (unsafe { test_hash_enabled_779(None) });
    }
    pub unsafe fn operator_call_1_enum_EnumClass(&self) {
        (unsafe { test_hash_enabled_780(None) });
    }
    pub unsafe fn operator_call_1_void_ptr(&self) {
        (unsafe { test_hash_enabled_781(None) });
    }
    pub unsafe fn operator_call_1_const_void_ptr(&self) {
        (unsafe { test_hash_enabled_782(None) });
    }
    pub unsafe fn operator_call_1_struct_Class_ptr(&self) {
        (unsafe { test_hash_enabled_783(None) });
    }
    pub unsafe fn operator_call_1_nullptr_t(&self) {
        (unsafe { test_hash_enabled_784(None) });
    }
}
pub unsafe fn test_library_hash_specializations_available_785() {
    (unsafe {
        for_each_0 ( <types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool__float__double__long_double__Enum__EnumClass__void_ptr__const_void_ptr__Class_ptr__std_nullptr_t_ >::default() , <TestHashEnabled >::default() , )
    });
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
    (unsafe { test_library_hash_specializations_available_785() });
    {
        (unsafe { test_hash_enabled_756(None) });
        (unsafe { test_hash_enabled_757(None) });
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
