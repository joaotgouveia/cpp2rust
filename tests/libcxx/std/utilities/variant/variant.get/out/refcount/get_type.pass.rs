extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_int_ref__const_int_ref_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_long_ref__const_long_ref_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int_ref__int_ref_ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int_refref__int_refref_ {}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_long_refref__const_long_refref_ {}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_int_refref__const_int_refref_ {}
pub fn operator_eq_6(LHS: Ptr<TypeID>, RHS: Ptr<TypeID>) -> () {
    return ({ LHS.with(|__s| __s.m_id.clone()) } == { RHS.with(|__s| __s.m_id.clone()) });
}
pub fn operator_ne_7(LHS: Ptr<TypeID>, RHS: Ptr<TypeID>) -> () {
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
        operator_eq_6(__lhs.clone(), __rhs.clone())
    }
}
pub type CallType = ();
pub const CallType_CT_None: CallType = 0;
pub const CallType_CT_NonConst: CallType = 1;
pub const CallType_CT_Const: CallType = 2;
pub const CallType_CT_LValue: CallType = 4;
pub const CallType_CT_RValue: CallType = 8;
pub fn operator_bitor_8(mut LHS: CallType, mut RHS: CallType) -> CallType {
    return (((LHS as ()) | (RHS as ())) as CallType);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ForwardingCallObject {}
thread_local!(
    pub static last_call_type_9: Value<CallType> = Rc::new(RefCell::new(CallType_CT_None));
);
thread_local!(
    pub static last_call_args_10: Value<Ptr<TypeID>> = Rc::new(RefCell::new(Ptr::<TypeID>::null()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnFirst {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnArity {}
pub fn test_const_lvalue_get_11() {
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::get<int>(v) == 42");;
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::get<const long>(v) == 42");;
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
    }
}
pub fn test_lvalue_get_12() {
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        unimplemented!();
    }
}
pub fn test_rvalue_get_13() {
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        unimplemented!();
    }
}
pub fn test_const_rvalue_get_14() {
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        unimplemented!();
    }
}
pub fn test_throws_for_all_value_categories_15() {}
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
    ({ test_const_lvalue_get_11() });
    ({ test_lvalue_get_12() });
    ({ test_rvalue_get_13() });
    ({ test_const_rvalue_get_14() });
    ({ test_throws_for_all_value_categories_15() });
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
    let _ = last_call_type_9.with(|_| ());
    let _ = last_call_args_10.with(|_| ());
}
