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
impl Callable2<Ptr<C2Int>, Ptr<i32>, bool> for transparent_less {
    fn call(&self, a1: Ptr<C2Int>, a2: Ptr<i32>) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            transparent_lessImpl::operator_call_1_const_struct_C2Int_ref_const_int_ref(__it, a1, a2)
        })
    }
}
impl Callable2<Ptr<i32>, Ptr<C2Int>, bool> for transparent_less {
    fn call(&self, a1: Ptr<i32>, a2: Ptr<C2Int>) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            transparent_lessImpl::operator_call_1_const_int_ref_const_struct_C2Int_ref(__it, a1, a2)
        })
    }
}
impl CppMove for transparent_less {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_not_referenceable {}
impl Callable2<Ptr<C2Int>, Ptr<i32>, bool> for transparent_less_not_referenceable {
    fn call(&self, a1: Ptr<C2Int>, a2: Ptr<i32>) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            transparent_less_not_referenceableImpl::operator_call_1_const_struct_C2Int_ref_const_int_ref(__it, a1, a2)
        })
    }
}
impl Callable2<Ptr<i32>, Ptr<C2Int>, bool> for transparent_less_not_referenceable {
    fn call(&self, a1: Ptr<i32>, a2: Ptr<C2Int>) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            transparent_less_not_referenceableImpl::operator_call_1_const_int_ref_const_struct_C2Int_ref(__it, a1, a2)
        })
    }
}
impl CppMove for transparent_less_not_referenceable {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_nonempty_is_transparent {}
impl CppMove for transparent_less_nonempty_is_transparent {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_nonempty {
    #[offset(0)]
    #[byte_size(1)]
    pub pad_: transparent_less_nonempty_is_transparent,
}
impl Callable2<Ptr<C2Int>, Ptr<i32>, bool> for transparent_less_nonempty {
    fn call(&self, a1: Ptr<C2Int>, a2: Ptr<i32>) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            transparent_less_nonemptyImpl::operator_call_1_const_struct_C2Int_ref_const_int_ref(
                __it, a1, a2,
            )
        })
    }
}
impl Callable2<Ptr<i32>, Ptr<C2Int>, bool> for transparent_less_nonempty {
    fn call(&self, a1: Ptr<i32>, a2: Ptr<C2Int>) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            transparent_less_nonemptyImpl::operator_call_1_const_int_ref_const_struct_C2Int_ref(
                __it, a1, a2,
            )
        })
    }
}
impl CppMove for transparent_less_nonempty {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_no_type {}
impl CppMove for transparent_less_no_type {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_private {}
impl CppMove for transparent_less_private {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct transparent_less_not_a_type {
    #[offset(0)]
    pub is_transparent: i32,
}
impl CppMove for transparent_less_not_a_type {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(i32, new_1, value)]
#[byte_size(4)]
pub struct C2Int {
    #[offset(0)]
    i_: i32,
}
impl C2Int {
    pub fn new_2() -> Self {
        Self { i_: 0 }
    }
    pub fn new_1(mut i: i32) -> Self {
        Self { i_: i }
    }
}
impl CppLess for C2Int {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_2(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for C2Int {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(operator_lt_2(__lhs.clone(), __rhs.clone()))
            && !(operator_lt_2(__rhs.clone(), __lhs.clone()))
    }
}
impl CppMove for C2Int {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for C2Int {
    fn default() -> Self {
        { C2Int::new_2() }
    }
}
pub fn operator_lt_1(mut rhs: i32, lhs: Ptr<C2Int>) -> bool {
    return ({ rhs } < { ({ C2IntImpl::get(&lhs) }) });
}
pub fn operator_lt_2(rhs: Ptr<C2Int>, lhs: Ptr<C2Int>) -> bool {
    return ({ ({ C2IntImpl::get(&rhs) }) } < { ({ C2IntImpl::get(&lhs) }) });
}
pub fn operator_lt_0(rhs: Ptr<C2Int>, mut lhs: i32) -> bool {
    return ({ ({ C2IntImpl::get(&rhs) }) } < { lhs });
}
pub fn test_3() -> bool {
    {
        let example: Value<Map<i32, (Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(Map::new()));
        let result: Value<(
            Value<BidirectionalIterator<(Value<i32>, Value<f64>)>>,
            Value<BidirectionalIterator<(Value<i32>, Value<f64>)>>,
        )> = Rc::new(RefCell::new(
            ({
                let ___x: Value<C2Int> = Rc::new(RefCell::new(C2Int::new_1({ 5 })));
                (*example.borrow()).equal_range_91_struct_C2Int(___x.as_pointer())
            }),
        ));
        assert!(({ (*(*result.borrow()).0.borrow()) == (*(*result.borrow()).1.borrow()) }));
    }
    {
        let example: Value<Map<i32, (Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(Map::new()));
        let result: Value<(
            Value<BidirectionalIterator<(Value<i32>, Value<f64>)>>,
            Value<BidirectionalIterator<(Value<i32>, Value<f64>)>>,
        )> = Rc::new(RefCell::new(
            ({
                let ___x: Value<C2Int> = Rc::new(RefCell::new(C2Int::new_1({ 5 })));
                (*example.borrow()).equal_range_91_struct_C2Int(___x.as_pointer())
            }),
        ));
        assert!(({ (*(*result.borrow()).0.borrow()) == (*(*result.borrow()).1.borrow()) }));
    }
    {
        let example: Value<Map<i32, (Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(Map::new()));
        let result: Value<(
            Value<BidirectionalIterator<(Value<i32>, Value<f64>)>>,
            Value<BidirectionalIterator<(Value<i32>, Value<f64>)>>,
        )> = Rc::new(RefCell::new(
            ({
                let ___x: Value<C2Int> = Rc::new(RefCell::new(C2Int::new_1({ 5 })));
                (*example.borrow()).equal_range_91_struct_C2Int(___x.as_pointer())
            }),
        ));
        assert!(({ (*(*result.borrow()).0.borrow()) == (*(*result.borrow()).1.borrow()) }));
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_3() });
    return 0;
}
pub trait C2IntImpl {
    fn get(&self) -> i32;
}
impl C2IntImpl for Ptr<C2Int> {
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.i_);
    }
}
pub trait transparent_lessImpl {
    fn operator_call_1_const_struct_C2Int_ref_const_int_ref(
        &self,
        t: Ptr<C2Int>,
        u: Ptr<i32>,
    ) -> bool;
    fn operator_call_1_const_int_ref_const_struct_C2Int_ref(
        &self,
        t: Ptr<i32>,
        u: Ptr<C2Int>,
    ) -> bool;
}
impl transparent_lessImpl for Ptr<transparent_less> {
    fn operator_call_1_const_struct_C2Int_ref_const_int_ref(
        &self,
        t: Ptr<C2Int>,
        u: Ptr<i32>,
    ) -> bool {
        return ({
            let _rhs: Ptr<C2Int> = (t).clone();
            let _lhs: i32 = (u.read());
            operator_lt_0(_rhs, _lhs)
        });
    }
    fn operator_call_1_const_int_ref_const_struct_C2Int_ref(
        &self,
        t: Ptr<i32>,
        u: Ptr<C2Int>,
    ) -> bool {
        return ({
            let _rhs: i32 = (t.read());
            let _lhs: Ptr<C2Int> = (u).clone();
            operator_lt_1(_rhs, _lhs)
        });
    }
}
pub trait transparent_less_nonemptyImpl {
    fn operator_call_1_const_struct_C2Int_ref_const_int_ref(
        &self,
        t: Ptr<C2Int>,
        u: Ptr<i32>,
    ) -> bool;
    fn operator_call_1_const_int_ref_const_struct_C2Int_ref(
        &self,
        t: Ptr<i32>,
        u: Ptr<C2Int>,
    ) -> bool;
}
impl transparent_less_nonemptyImpl for Ptr<transparent_less_nonempty> {
    fn operator_call_1_const_struct_C2Int_ref_const_int_ref(
        &self,
        t: Ptr<C2Int>,
        u: Ptr<i32>,
    ) -> bool {
        return ({
            let _rhs: Ptr<C2Int> = (t).clone();
            let _lhs: i32 = (u.read());
            operator_lt_0(_rhs, _lhs)
        });
    }
    fn operator_call_1_const_int_ref_const_struct_C2Int_ref(
        &self,
        t: Ptr<i32>,
        u: Ptr<C2Int>,
    ) -> bool {
        return ({
            let _rhs: i32 = (t.read());
            let _lhs: Ptr<C2Int> = (u).clone();
            operator_lt_1(_rhs, _lhs)
        });
    }
}
pub trait transparent_less_not_referenceableImpl {
    fn operator_call_1_const_struct_C2Int_ref_const_int_ref(
        &self,
        t: Ptr<C2Int>,
        u: Ptr<i32>,
    ) -> bool;
    fn operator_call_1_const_int_ref_const_struct_C2Int_ref(
        &self,
        t: Ptr<i32>,
        u: Ptr<C2Int>,
    ) -> bool;
}
impl transparent_less_not_referenceableImpl for Ptr<transparent_less_not_referenceable> {
    fn operator_call_1_const_struct_C2Int_ref_const_int_ref(
        &self,
        t: Ptr<C2Int>,
        u: Ptr<i32>,
    ) -> bool {
        return ({
            let _rhs: Ptr<C2Int> = (t).clone();
            let _lhs: i32 = (u.read());
            operator_lt_0(_rhs, _lhs)
        });
    }
    fn operator_call_1_const_int_ref_const_struct_C2Int_ref(
        &self,
        t: Ptr<i32>,
        u: Ptr<C2Int>,
    ) -> bool {
        return ({
            let _rhs: i32 = (t.read());
            let _lhs: Ptr<C2Int> = (u).clone();
            operator_lt_1(_rhs, _lhs)
        });
    }
}
pub fn __cpp2rust_init_globals() {}
