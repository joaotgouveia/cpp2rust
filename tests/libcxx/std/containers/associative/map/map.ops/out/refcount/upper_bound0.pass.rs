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
pub struct transparent_less {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_not_referenceable {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_nonempty_is_transparent {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_nonempty {
    #[offset(0)]
    #[byte_size(1)]
    pub pad_: transparent_less_nonempty_is_transparent,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_no_type {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_private {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct transparent_less_not_a_type {
    #[offset(0)]
    pub is_transparent: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct C2Int {
    #[offset(0)]
    i_: (),
}
impl C2Int {
    pub fn new_1() -> Self {
        Self { i_: 0_() }
    }
    pub fn new_2(mut i: ()) -> Self {
        Self { i_: i }
    }
}
impl std::cmp::Ord for C2Int {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_1(
                Rc::new(RefCell::new(C2Int {
                    i_: self.i_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(C2Int {
                    i_: other.i_.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_1(
                Rc::new(RefCell::new(C2Int {
                    i_: other.i_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(C2Int {
                    i_: self.i_.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for C2Int {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for C2Int {
    fn eq(&self, other: &Self) -> bool {
        {
            !(operator_lt_1(
                Rc::new(RefCell::new(C2Int {
                    i_: self.i_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(C2Int {
                    i_: other.i_.clone(),
                }))
                .as_pointer(),
            )) && !(operator_lt_1(
                Rc::new(RefCell::new(C2Int {
                    i_: other.i_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(C2Int {
                    i_: self.i_.clone(),
                }))
                .as_pointer(),
            ))
        }
    }
}
impl std::cmp::Eq for C2Int {}
impl Default for C2Int {
    fn default() -> Self {
        { C2Int::new_1() }
    }
}
pub fn operator_lt_2(mut rhs: (), lhs: Ptr<C2Int>) -> () {
    return ({ rhs } < { ({ C2IntImpl::get(&lhs) }) });
}
pub fn operator_lt_1(rhs: Ptr<C2Int>, lhs: Ptr<C2Int>) -> () {
    return ({ ({ C2IntImpl::get(&rhs) }) } < { ({ C2IntImpl::get(&lhs) }) });
}
pub fn operator_lt_0(rhs: Ptr<C2Int>, mut lhs: ()) -> () {
    return ({ ({ C2IntImpl::get(&rhs) }) } < { lhs });
}
pub fn test_3() -> () {
    {
        let example: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let example: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let example: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    return true;
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
    ({ test_3() });
    return 0_();
}
pub trait C2IntImpl {
    fn get(&self) -> ();
}
impl C2IntImpl for Ptr<C2Int> {
    fn get(&self) -> () {
        return (*self).with(|__s| __s.i_);
    }
}
pub trait transparent_lessImpl {
    fn operator_call_1_const_struct_C2Int_ref_const_int_ref(&self, t: Ptr<C2Int>, u: Ptr<()>)
    -> ();
}
impl transparent_lessImpl for Ptr<transparent_less> {
    fn operator_call_1_const_struct_C2Int_ref_const_int_ref(
        &self,
        t: Ptr<C2Int>,
        u: Ptr<()>,
    ) -> () {
        return ({
            let _rhs: Ptr<C2Int> = (t).clone();
            let _lhs: () = (u.read());
            operator_lt_0(_rhs, _lhs)
        });
    }
}
pub trait transparent_less_nonemptyImpl {
    fn operator_call_1_const_struct_C2Int_ref_const_int_ref(&self, t: Ptr<C2Int>, u: Ptr<()>)
    -> ();
}
impl transparent_less_nonemptyImpl for Ptr<transparent_less_nonempty> {
    fn operator_call_1_const_struct_C2Int_ref_const_int_ref(
        &self,
        t: Ptr<C2Int>,
        u: Ptr<()>,
    ) -> () {
        return ({
            let _rhs: Ptr<C2Int> = (t).clone();
            let _lhs: () = (u.read());
            operator_lt_0(_rhs, _lhs)
        });
    }
}
pub trait transparent_less_not_referenceableImpl {
    fn operator_call_1_const_struct_C2Int_ref_const_int_ref(&self, t: Ptr<C2Int>, u: Ptr<()>)
    -> ();
}
impl transparent_less_not_referenceableImpl for Ptr<transparent_less_not_referenceable> {
    fn operator_call_1_const_struct_C2Int_ref_const_int_ref(
        &self,
        t: Ptr<C2Int>,
        u: Ptr<()>,
    ) -> () {
        return ({
            let _rhs: Ptr<C2Int> = (t).clone();
            let _lhs: () = (u.read());
            operator_lt_0(_rhs, _lhs)
        });
    }
}
pub fn __cpp2rust_init_globals() {}
