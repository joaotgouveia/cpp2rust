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
impl std::cmp::PartialEq for TypeID {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_0(
                Rc::new(RefCell::new(TypeID {
                    m_id: self.m_id.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(TypeID {
                    m_id: other.m_id.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for TypeID {}
pub type UsesAllocatorType = ();
pub const UsesAllocatorType_UA_None: UsesAllocatorType = 0;
pub const UsesAllocatorType_UA_AllocArg: UsesAllocatorType = 2;
pub const UsesAllocatorType_UA_AllocLast: UsesAllocatorType = 4;
thread_local!(
    pub static UA_None_2: Value<UsesAllocatorType> =
        Rc::new(RefCell::new(UsesAllocatorType_UA_None));
);
thread_local!(
    pub static UA_AllocArg_3: Value<UsesAllocatorType> =
        Rc::new(RefCell::new(UsesAllocatorType_UA_AllocArg));
);
thread_local!(
    pub static UA_AllocLast_4: Value<UsesAllocatorType> =
        Rc::new(RefCell::new(UsesAllocatorType_UA_AllocLast));
);
pub fn toString_5(mut UA: UsesAllocatorType) -> Ptr<()> {
    switch!(match UA {
        __v if __v == UA_None_2.with(|rc| *rc.borrow()) => {
            return Ptr::<()>::from_string_literal(b"UA_None");
        }
        __v if __v == UA_AllocArg_3.with(|rc| *rc.borrow()) => {
            return Ptr::<()>::from_string_literal(b"UA_AllocArg");
        }
        __v if __v == UA_AllocLast_4.with(|rc| *rc.borrow()) => {
            return Ptr::<()>::from_string_literal(b"UA_AllocLast");
        }
        _ => {
            unimplemented!();
        }
    });
    panic!("ub: non-void function does not return a value")
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct AllocLastTag {}
thread_local!(
    pub static constructed_6: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct default_constructible_0_ {
    #[offset(0)]
    pub x: (),
}
impl default_constructible_0_ {
    pub fn new() -> Self {
        let __this: default_constructible_0_ = Self { x: 42_() };
        (*constructed_6.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Default for default_constructible_0_ {
    fn default() -> Self {
        { default_constructible_0_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct default_constructible_1_ {
    #[offset(0)]
    pub x: (),
}
impl default_constructible_1_ {
    pub fn new() -> Self {
        let __this: default_constructible_1_ = Self { x: 42_() };
        (*constructed_6.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Default for default_constructible_1_ {
    fn default() -> Self {
        { default_constructible_1_::new() }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    {
        let buffer: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..8).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let mut ptr: Ptr<()> = (buffer.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        constructed_6.with(|rc| *rc.borrow_mut() = 0_());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let buffer: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..8).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let mut ptr: Ptr<()> = (buffer.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        constructed_6.with(|rc| *rc.borrow_mut() = 0_());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
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
    let _ = UA_None_2.with(|_| ());
    let _ = UA_AllocArg_3.with(|_| ());
    let _ = UA_AllocLast_4.with(|_| ());
    let _ = constructed_6.with(|_| ());
}
