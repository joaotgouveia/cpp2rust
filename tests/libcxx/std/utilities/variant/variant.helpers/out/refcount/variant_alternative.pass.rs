extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(LHS: Ptr<TypeID>, RHS: Ptr<TypeID>) -> () {
    return ({ LHS.with(|__s| __s.m_id.clone()) } == { RHS.with(|__s| __s.m_id.clone()) });
}
pub fn operator_ne_1(LHS: Ptr<TypeID>, RHS: Ptr<TypeID>) -> () {
    return ({ LHS.with(|__s| __s.m_id.clone()) } != { RHS.with(|__s| __s.m_id.clone()) });
}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(8)]
pub struct TypeID {
    #[offset(0)]
    #[byte_size(8)]
    m_id: Ptr<()>,
}
impl TypeID {
    fn new(mut xid: Ptr<()>) -> Self {
        Self {
            m_id: (xid).clone(),
        }
    }
}
impl CppEq for TypeID {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_0(__lhs.clone(), __rhs.clone())
    }
}
pub type CallType = ();
pub const CallType_CT_None: CallType = 0;
pub const CallType_CT_NonConst: CallType = 1;
pub const CallType_CT_Const: CallType = 2;
pub const CallType_CT_LValue: CallType = 4;
pub const CallType_CT_RValue: CallType = 8;
pub fn operator_bitor_2(mut LHS: CallType, mut RHS: CallType) -> CallType {
    return (((LHS as ()) | (RHS as ())) as CallType);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ForwardingCallObject {}
thread_local!(
    pub static last_call_type_3: Value<CallType> = Rc::new(RefCell::new(CallType_CT_None));
);
thread_local!(
    pub static last_call_args_4: Value<Ptr<TypeID>> = Rc::new(RefCell::new(Ptr::<TypeID>::null()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnFirst {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnArity {}
pub fn test_5() {
    const _: () = assert!(
        true,
        "std::is_same_v<typename std::variant_alternative<I, V>::type, E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<typename std::variant_alternative<I, const V>::type,\n                     const E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<typename std::variant_alternative<I, volatile V>::type,\n                     volatile E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<\n          typename std::variant_alternative<I, const volatile V>::type,\n          const volatile E>"
    );;
    const _: () = assert!(true, "std::is_same_v<std::variant_alternative_t<I, V>, E>");;
    const _: () = assert!(
        true,
        "std::is_same_v<std::variant_alternative_t<I, const V>, const E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<std::variant_alternative_t<I, volatile V>, volatile E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<std::variant_alternative_t<I, const volatile V>,\n                               const volatile E>"
    );;
}
pub fn test_6() {
    const _: () = assert!(
        true,
        "std::is_same_v<typename std::variant_alternative<I, V>::type, E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<typename std::variant_alternative<I, const V>::type,\n                     const E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<typename std::variant_alternative<I, volatile V>::type,\n                     volatile E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<\n          typename std::variant_alternative<I, const volatile V>::type,\n          const volatile E>"
    );;
    const _: () = assert!(true, "std::is_same_v<std::variant_alternative_t<I, V>, E>");;
    const _: () = assert!(
        true,
        "std::is_same_v<std::variant_alternative_t<I, const V>, const E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<std::variant_alternative_t<I, volatile V>, volatile E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<std::variant_alternative_t<I, const volatile V>,\n                               const volatile E>"
    );;
}
pub fn test_7() {
    const _: () = assert!(
        true,
        "std::is_same_v<typename std::variant_alternative<I, V>::type, E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<typename std::variant_alternative<I, const V>::type,\n                     const E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<typename std::variant_alternative<I, volatile V>::type,\n                     volatile E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<\n          typename std::variant_alternative<I, const volatile V>::type,\n          const volatile E>"
    );;
    const _: () = assert!(true, "std::is_same_v<std::variant_alternative_t<I, V>, E>");;
    const _: () = assert!(
        true,
        "std::is_same_v<std::variant_alternative_t<I, const V>, const E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<std::variant_alternative_t<I, volatile V>, volatile E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<std::variant_alternative_t<I, const volatile V>,\n                               const volatile E>"
    );;
}
pub fn test_8() {
    const _: () = assert!(
        true,
        "std::is_same_v<typename std::variant_alternative<I, V>::type, E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<typename std::variant_alternative<I, const V>::type,\n                     const E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<typename std::variant_alternative<I, volatile V>::type,\n                     volatile E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<\n          typename std::variant_alternative<I, const volatile V>::type,\n          const volatile E>"
    );;
    const _: () = assert!(true, "std::is_same_v<std::variant_alternative_t<I, V>, E>");;
    const _: () = assert!(
        true,
        "std::is_same_v<std::variant_alternative_t<I, const V>, const E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<std::variant_alternative_t<I, volatile V>, volatile E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<std::variant_alternative_t<I, const volatile V>,\n                               const volatile E>"
    );;
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
    {
        ({ test_5() });
        ({ test_6() });
        ({ test_7() });
        ({ test_8() });
    }
    return 0_();
}
pub trait ForwardingCallObjectImpl {
    fn to_(&self) -> ();
}
impl ForwardingCallObjectImpl for Ptr<ForwardingCallObject> {
    fn to_(&self) -> () {
        return 0_();
    }
}
pub trait TypeIDImpl {
    fn name(&self) -> ();
}
impl TypeIDImpl for Ptr<TypeID> {
    fn name(&self) -> () {
        return unimplemented!();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = last_call_type_3.with(|_| ());
    let _ = last_call_args_4.with(|_| ());
}
