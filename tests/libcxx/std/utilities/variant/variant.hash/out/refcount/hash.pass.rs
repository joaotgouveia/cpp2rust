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
impl CppMove for test_macros_detail_is_same_unsigned_long__unsigned_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
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
impl CppMove for ForwardingCallObject {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static last_call_type_4: Value<CallType> = Rc::new(RefCell::new(CallType_CT_None));
);
thread_local!(
    pub static last_call_args_5: Value<Ptr<TypeID>> = Rc::new(RefCell::new(Ptr::<TypeID>::null()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnFirst {}
impl CppMove for ReturnFirst {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnArity {}
impl CppMove for ReturnArity {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool__float__double__long_double___types_type_list_Enum__EnumClass__void_ptr__const_void_ptr__Class_ptr__std_nullptr_t__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool__float__double__long_double___types_type_list_Enum__EnumClass__void_ptr__const_void_ptr__Class_ptr__std_nullptr_t__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
pub type Enum = u32;
pub type EnumClass = bool;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Class {}
impl CppMove for Class {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestHashEnabled {}
impl CppMove for TestHashEnabled {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_hash_variant_6() {
    {
        let v: Value<std_variant_int__long__int_> = Rc::new(RefCell::new({
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(42));
            std_variant_int__long__int_::new_1(
                { (*in_place_index_7.with(Value::clone).borrow()).clone() },
                { __tmp_0.as_pointer() },
            )
        }));
        let v_copy: Value<std_variant_int__long__int_> =
            Rc::new(RefCell::new((*v.borrow()).clone()));
        let v2: Value<std_variant_int__long__int_> = Rc::new(RefCell::new({
            let __tmp_1: Value<i32> = Rc::new(RefCell::new(100));
            std_variant_int__long__int_::new_1(
                { (*in_place_index_7.with(Value::clone).borrow()).clone() },
                { __tmp_1.as_pointer() },
            )
        }));
        let h: Value<std_hash_std_variant_int__long__int__> = Rc::new(RefCell::new(
            <std_hash_std_variant_int__long__int__>::default(),
        ));
        assert!(
            ({
                (({ ((*h.borrow_mut()) as std___variant_hash_int__long__int_)(v.as_pointer()) })
                    == ({
                        ((*h.borrow_mut()) as std___variant_hash_int__long__int_)(v.as_pointer())
                    }))
            })
        );
        assert!(
            ({
                (({ ((*h.borrow_mut()) as std___variant_hash_int__long__int_)(v.as_pointer()) })
                    != ({
                        ((*h.borrow_mut()) as std___variant_hash_int__long__int_)(v2.as_pointer())
                    }))
            })
        );
        assert!(
            ({
                (({ ((*h.borrow_mut()) as std___variant_hash_int__long__int_)(v.as_pointer()) })
                    == ({
                        ((*h.borrow_mut()) as std___variant_hash_int__long__int_)(
                            v_copy.as_pointer(),
                        )
                    }))
            })
        );
        {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "std::is_copy_constructible<H>::value");;
        }
    }
    {
        let str: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::from_string_literal(b"hello")));
        let v0: Value<std_variant_std_monostate__int__long__const_char_ptr_> = Rc::new(
            RefCell::new(<std_variant_std_monostate__int__long__const_char_ptr_>::default()),
        );
        let v0_other: Value<std_variant_std_monostate__int__long__const_char_ptr_> = Rc::new(
            RefCell::new(<std_variant_std_monostate__int__long__const_char_ptr_>::default()),
        );
        let v1: Value<std_variant_std_monostate__int__long__const_char_ptr_> =
            Rc::new(RefCell::new(42));
        let v1_other: Value<std_variant_std_monostate__int__long__const_char_ptr_> =
            Rc::new(RefCell::new(100));
        let v2: Value<std_variant_std_monostate__int__long__const_char_ptr_> =
            Rc::new(RefCell::new(100_i64));
        let v2_other: Value<std_variant_std_monostate__int__long__const_char_ptr_> =
            Rc::new(RefCell::new(999_i64));
        let v3: Value<std_variant_std_monostate__int__long__const_char_ptr_> =
            Rc::new(RefCell::new(
                std_variant_std_monostate__int__long__const_char_ptr_::new_1({ str.as_pointer() }),
            ));
        let v3_other: Value<std_variant_std_monostate__int__long__const_char_ptr_> =
            Rc::new(RefCell::new(
                std_variant_std_monostate__int__long__const_char_ptr_::new_2({
                    Ptr::<i8>::from_string_literal(b"not hello")
                }),
            ));
        let h: Value<std_hash_std_variant_std_monostate__int__long__const_char_ptr__> =
            Rc::new(RefCell::new(
                <std_hash_std_variant_std_monostate__int__long__const_char_ptr__>::default(),
            ));
        assert!(
            ({
                (({
                    ((*h.borrow_mut())
                        as std___variant_hash_std_monostate__int__long__const_char_ptr_)(
                        v0.as_pointer(),
                    )
                }) == ({
                    ((*h.borrow_mut())
                        as std___variant_hash_std_monostate__int__long__const_char_ptr_)(
                        v0.as_pointer(),
                    )
                }))
            })
        );
        assert!(
            ({
                (({
                    ((*h.borrow_mut())
                        as std___variant_hash_std_monostate__int__long__const_char_ptr_)(
                        v0.as_pointer(),
                    )
                }) == ({
                    ((*h.borrow_mut())
                        as std___variant_hash_std_monostate__int__long__const_char_ptr_)(
                        v0_other.as_pointer(),
                    )
                }))
            })
        );
        assert!(
            ({
                (({
                    ((*h.borrow_mut())
                        as std___variant_hash_std_monostate__int__long__const_char_ptr_)(
                        v1.as_pointer(),
                    )
                }) == ({
                    ((*h.borrow_mut())
                        as std___variant_hash_std_monostate__int__long__const_char_ptr_)(
                        v1.as_pointer(),
                    )
                }))
            })
        );
        assert!(
            ({
                (({
                    ((*h.borrow_mut())
                        as std___variant_hash_std_monostate__int__long__const_char_ptr_)(
                        v1.as_pointer(),
                    )
                }) != ({
                    ((*h.borrow_mut())
                        as std___variant_hash_std_monostate__int__long__const_char_ptr_)(
                        v1_other.as_pointer(),
                    )
                }))
            })
        );
        assert!(
            ({
                (({
                    ((*h.borrow_mut())
                        as std___variant_hash_std_monostate__int__long__const_char_ptr_)(
                        v2.as_pointer(),
                    )
                }) == ({
                    ((*h.borrow_mut())
                        as std___variant_hash_std_monostate__int__long__const_char_ptr_)(
                        v2.as_pointer(),
                    )
                }))
            })
        );
        assert!(
            ({
                (({
                    ((*h.borrow_mut())
                        as std___variant_hash_std_monostate__int__long__const_char_ptr_)(
                        v2.as_pointer(),
                    )
                }) != ({
                    ((*h.borrow_mut())
                        as std___variant_hash_std_monostate__int__long__const_char_ptr_)(
                        v2_other.as_pointer(),
                    )
                }))
            })
        );
        assert!(
            ({
                (({
                    ((*h.borrow_mut())
                        as std___variant_hash_std_monostate__int__long__const_char_ptr_)(
                        v3.as_pointer(),
                    )
                }) == ({
                    ((*h.borrow_mut())
                        as std___variant_hash_std_monostate__int__long__const_char_ptr_)(
                        v3.as_pointer(),
                    )
                }))
            })
        );
        assert!(
            ({
                (({
                    ((*h.borrow_mut())
                        as std___variant_hash_std_monostate__int__long__const_char_ptr_)(
                        v3.as_pointer(),
                    )
                }) != ({
                    ((*h.borrow_mut())
                        as std___variant_hash_std_monostate__int__long__const_char_ptr_)(
                        v3_other.as_pointer(),
                    )
                }))
            })
        );
        assert!(
            ({
                (({
                    ((*h.borrow_mut())
                        as std___variant_hash_std_monostate__int__long__const_char_ptr_)(
                        v0.as_pointer(),
                    )
                }) != ({
                    ((*h.borrow_mut())
                        as std___variant_hash_std_monostate__int__long__const_char_ptr_)(
                        v1.as_pointer(),
                    )
                }))
            })
        );
        assert!(
            ({
                (({
                    ((*h.borrow_mut())
                        as std___variant_hash_std_monostate__int__long__const_char_ptr_)(
                        v0.as_pointer(),
                    )
                }) != ({
                    ((*h.borrow_mut())
                        as std___variant_hash_std_monostate__int__long__const_char_ptr_)(
                        v2.as_pointer(),
                    )
                }))
            })
        );
        assert!(
            ({
                (({
                    ((*h.borrow_mut())
                        as std___variant_hash_std_monostate__int__long__const_char_ptr_)(
                        v0.as_pointer(),
                    )
                }) != ({
                    ((*h.borrow_mut())
                        as std___variant_hash_std_monostate__int__long__const_char_ptr_)(
                        v3.as_pointer(),
                    )
                }))
            })
        );
        assert!(
            ({
                (({
                    ((*h.borrow_mut())
                        as std___variant_hash_std_monostate__int__long__const_char_ptr_)(
                        v1.as_pointer(),
                    )
                }) != ({
                    ((*h.borrow_mut())
                        as std___variant_hash_std_monostate__int__long__const_char_ptr_)(
                        v2.as_pointer(),
                    )
                }))
            })
        );
        assert!(
            ({
                (({
                    ((*h.borrow_mut())
                        as std___variant_hash_std_monostate__int__long__const_char_ptr_)(
                        v1.as_pointer(),
                    )
                }) != ({
                    ((*h.borrow_mut())
                        as std___variant_hash_std_monostate__int__long__const_char_ptr_)(
                        v3.as_pointer(),
                    )
                }))
            })
        );
        assert!(
            ({
                (({
                    ((*h.borrow_mut())
                        as std___variant_hash_std_monostate__int__long__const_char_ptr_)(
                        v2.as_pointer(),
                    )
                }) != ({
                    ((*h.borrow_mut())
                        as std___variant_hash_std_monostate__int__long__const_char_ptr_)(
                        v3.as_pointer(),
                    )
                }))
            })
        );
    }
}
pub fn test_hash_monostate_8() {
    let h: Value<std_hash_std_monostate_> =
        Rc::new(RefCell::new(<std_hash_std_monostate_>::default()));
    let m1: Value<std_monostate> = Rc::new(RefCell::new(<std_monostate>::default()));
    let m2: Value<std_monostate> = Rc::new(RefCell::new(<std_monostate>::default()));
    assert!(
        ({
            (({ (*h.borrow_mut())(m1.as_pointer()) }) == ({ (*h.borrow_mut())(m1.as_pointer()) }))
        })
    );
    assert!(
        ({
            (({ (*h.borrow_mut())(m2.as_pointer()) }) == ({ (*h.borrow_mut())(m2.as_pointer()) }))
        })
    );
    assert!(
        ({
            (({ (*h.borrow_mut())(m1.as_pointer()) }) == ({ (*h.borrow_mut())(m2.as_pointer()) }))
        })
    );
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_copy_constructible<H>::value");;
    }
    {
        ({ test_hash_enabled_9(None) });
    }
}
pub fn test_hash_variant_duplicate_elements_10() {
    let h: Value<std_hash_std_variant_std_monostate__std_monostate__> = Rc::new(RefCell::new(
        <std_hash_std_variant_std_monostate__std_monostate__>::default(),
    ));
    let v1: Value<std_variant_std_monostate__std_monostate_> = Rc::new(RefCell::new(
        std_variant_std_monostate__std_monostate_::new({
            (*in_place_index_7.with(Value::clone).borrow()).clone()
        }),
    ));
    let v2: Value<std_variant_std_monostate__std_monostate_> = Rc::new(RefCell::new(
        std_variant_std_monostate__std_monostate_::new({
            (*in_place_index_11.with(Value::clone).borrow()).clone()
        }),
    ));
    assert!(
        ({
            (({
                ((*h.borrow_mut()) as std___variant_hash_std_monostate__std_monostate_)(
                    v1.as_pointer(),
                )
            }) == ({
                ((*h.borrow_mut()) as std___variant_hash_std_monostate__std_monostate_)(
                    v1.as_pointer(),
                )
            }))
        })
    );
    assert!(
        ({
            (({
                ((*h.borrow_mut()) as std___variant_hash_std_monostate__std_monostate_)(
                    v2.as_pointer(),
                )
            }) == ({
                ((*h.borrow_mut()) as std___variant_hash_std_monostate__std_monostate_)(
                    v2.as_pointer(),
                )
            }))
        })
    );
    const _: () = assert!(true, "");;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A {}
impl CppMove for A {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct B {}
impl CppMove for B {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_hash_variant_enabled_12() {
    {
        ({ test_hash_enabled_13(None) });
        ({ test_hash_enabled_14(None) });
    }
    {
        ({ test_hash_disabled_15() });
        ({ test_hash_disabled_16() });
    }
    {
        ({ test_hash_enabled_17(None) });
        ({ test_hash_enabled_18(None) });
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
    ({ test_hash_variant_6() });
    ({ test_hash_variant_duplicate_elements_10() });
    ({ test_hash_monostate_8() });
    ({ test_hash_variant_enabled_12() });
    return 0;
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
