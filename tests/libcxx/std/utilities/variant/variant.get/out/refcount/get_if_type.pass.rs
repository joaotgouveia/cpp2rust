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
pub struct test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {}
impl CppMove for test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_long_ptr__const_long_ptr_ {}
impl CppMove for test_macros_detail_is_same_const_long_ptr__const_long_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
impl CppMove for test_macros_detail_is_same_int_ptr__int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_3(LHS: Ptr<TypeID>, RHS: Ptr<TypeID>) -> bool {
    return ({ LHS.with(|__s| __s.m_id.clone()) } == { RHS.with(|__s| __s.m_id.clone()) });
}
pub fn operator_ne_4(LHS: Ptr<TypeID>, RHS: Ptr<TypeID>) -> bool {
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
        operator_eq_3(__lhs.clone(), __rhs.clone())
    }
}
pub type CallType = u32;
pub const CallType_CT_None: CallType = 0;
pub const CallType_CT_NonConst: CallType = 1;
pub const CallType_CT_Const: CallType = 2;
pub const CallType_CT_LValue: CallType = 4;
pub const CallType_CT_RValue: CallType = 8;
pub fn operator_bitor_5(mut LHS: CallType, mut RHS: CallType) -> CallType {
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
    pub static last_call_type_6: Value<CallType> = Rc::new(RefCell::new(CallType_CT_None));
);
thread_local!(
    pub static last_call_args_7: Value<Ptr<TypeID>> = Rc::new(RefCell::new(Ptr::<TypeID>::null()));
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
pub fn test_const_get_if_8() {
    {
        let mut v: Ptr<std_variant_int_> = Ptr::<std_variant_int_>::null();
        const _: () = assert!(true, "std::get_if<int>(v) == nullptr");;
    }
    {
        let v: Value<std_variant_int__const_long_> = Rc::new(RefCell::new(42));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "*std::get_if<int>(&v) == 42");;
        const _: () = assert!(true, "std::get_if<const long>(&v) == nullptr");;
    }
    {
        let v: Value<std_variant_int__const_long_> = Rc::new(RefCell::new(42_i64));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "*std::get_if<const long>(&v) == 42");;
        const _: () = assert!(true, "std::get_if<int>(&v) == nullptr");;
    }
}
pub fn test_get_if_9() {
    {
        let mut v: Ptr<std_variant_int_> = Ptr::<std_variant_int_>::null();
        assert!(({ get_if_10((v).clone(),) }).is_null());
    }
    {
        let v: Value<std_variant_int__const_long_> = Rc::new(RefCell::new(42));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(((({ get_if_11((v.as_pointer()),) }).read()) == 42));
        assert!(({ get_if_12((v.as_pointer()),) }).is_null());
    }
    {
        let v: Value<std_variant_int__const_long_> = Rc::new(RefCell::new(42_i64));
        const _: () = assert!(true, "");;
        assert!(((({ get_if_12((v.as_pointer()),) }).read()) == 42_i64));
        assert!(({ get_if_11((v.as_pointer()),) }).is_null());
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
    ({ test_const_get_if_8() });
    ({ test_get_if_9() });
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
    let _ = last_call_type_6.with(|_| ());
    let _ = last_call_args_7.with(|_| ());
}
