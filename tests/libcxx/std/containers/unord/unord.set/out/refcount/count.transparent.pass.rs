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
impl CppMove for transparent_less {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_not_referenceable {}
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
        operator_lt_0(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for C2Int {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(operator_lt_0(__lhs.clone(), __rhs.clone()))
            && !(operator_lt_0(__rhs.clone(), __lhs.clone()))
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
pub fn operator_lt_0(rhs: Ptr<C2Int>, lhs: Ptr<C2Int>) -> bool {
    return ({ ({ C2IntImpl::get(&rhs) }) } < { ({ C2IntImpl::get(&lhs) }) });
}
pub fn operator_lt_2(rhs: Ptr<C2Int>, mut lhs: i32) -> bool {
    return ({ ({ C2IntImpl::get(&rhs) }) } < { lhs });
}
pub fn operator_eq_3(lhs: Ptr<StoredType_int_>, rhs: Ptr<StoredType_int_>) -> bool {
    return ({ lhs.with(|__s| __s.value_) } == { rhs.with(|__s| __s.value_) });
}
pub fn operator_eq_4(lhs: Ptr<StoredType_int_>, rhs: Ptr<SearchedType_int_>) -> bool {
    return ({ lhs.with(|__s| __s.value_) } == { ({ SearchedType_int_Impl::get_value(&rhs) }) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new_1, value)]
#[byte_size(4)]
pub struct StoredType_int_ {
    #[offset(0)]
    value_: i32,
}
impl StoredType_int_ {
    pub fn new_1(mut value: i32) -> Self {
        Self { value_: value }
    }
}
impl CppEq for StoredType_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_3(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for StoredType_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(StoredType_int_, to_StoredType_int_)]
#[byte_size(16)]
pub struct SearchedType_int_ {
    #[offset(0)]
    value_: i32,
    #[offset(8)]
    #[byte_size(8)]
    conversions_: Ptr<i32>,
}
impl SearchedType_int_ {
    pub fn new(mut value: i32, mut counter: Ptr<i32>) -> Self {
        Self {
            value_: value,
            conversions_: (counter).clone(),
        }
    }
}
impl CppMove for SearchedType_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct hash_impl {}
impl Callable1<Ptr<SearchedType_int_>, usize> for hash_impl {
    fn call(&self, a1: Ptr<SearchedType_int_>) -> usize {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            hash_implImpl::operator_call_1_int(__it, a1)
        })
    }
}
impl Callable1<Ptr<StoredType_int_>, usize> for hash_impl {
    fn call(&self, a1: Ptr<StoredType_int_>) -> usize {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            hash_implImpl::operator_call_2_int(__it, a1)
        })
    }
}
impl CppMove for hash_impl {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct non_transparent_hash {}
impl CppMove for non_transparent_hash {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_hash {}
impl CppMove for transparent_hash {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_hash_final {}
impl CppMove for transparent_hash_final {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_equal_final {}
impl CppMove for transparent_equal_final {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
    {
        ({
            test_transparent_count_5 ( std_unordered_set_StoredType_int___transparent_hash__std_equal_to_void___std_allocator_StoredType_int___ :: new_1 ( { Vec::from([ StoredType_int_ :: new_1 ( { 1  } , )   , StoredType_int_ :: new_1 ( { 2  } , )   , ] )  } , None , None , None , None , )   , )
        });
        ({
            test_transparent_count_6 ( std_unordered_set_StoredType_int___transparent_hash__std_equal_to_void___std_allocator_StoredType_int___ :: new_1 ( { Vec::from([ StoredType_int_ :: new_1 ( { 1  } , )   , StoredType_int_ :: new_1 ( { 2  } , )   , ] )  } , None , None , None , None , )   , )
        });
    }
    {
        ({
            test_transparent_count_7 ( std_unordered_set_StoredType_int___transparent_hash_final__transparent_equal_final__std_allocator_StoredType_int___ :: new_1 ( { Vec::from([ StoredType_int_ :: new_1 ( { 1  } , )   , StoredType_int_ :: new_1 ( { 2  } , )   , ] )  } , None , None , None , None , )   , )
        });
        ({
            test_transparent_count_8 ( std_unordered_set_StoredType_int___transparent_hash_final__transparent_equal_final__std_allocator_StoredType_int___ :: new_1 ( { Vec::from([ StoredType_int_ :: new_1 ( { 1  } , )   , StoredType_int_ :: new_1 ( { 2  } , )   , ] )  } , None , None , None , None , )   , )
        });
    }
    {
        ({
            test_non_transparent_count_9 ( std_unordered_set_StoredType_int___non_transparent_hash__std_equal_to_void___std_allocator_StoredType_int___ :: new_1 ( { Vec::from([ StoredType_int_ :: new_1 ( { 1  } , )   , StoredType_int_ :: new_1 ( { 2  } , )   , ] )  } , None , None , None , None , )   , )
        });
        ({
            test_non_transparent_count_10 ( std_unordered_set_StoredType_int___non_transparent_hash__std_equal_to_void___std_allocator_StoredType_int___ :: new_1 ( { Vec::from([ StoredType_int_ :: new_1 ( { 1  } , )   , StoredType_int_ :: new_1 ( { 2  } , )   , ] )  } , None , None , None , None , )   , )
        });
    }
    {
        ({
            test_non_transparent_count_11 ( std_unordered_set_StoredType_int___transparent_hash__std_equal_to_StoredType_int____std_allocator_StoredType_int___ :: new_1 ( { Vec::from([ StoredType_int_ :: new_1 ( { 1  } , )   , StoredType_int_ :: new_1 ( { 2  } , )   , ] )  } , None , None , None , None , )   , )
        });
        ({
            test_non_transparent_count_12 ( std_unordered_set_StoredType_int___transparent_hash__std_equal_to_StoredType_int____std_allocator_StoredType_int___ :: new_1 ( { Vec::from([ StoredType_int_ :: new_1 ( { 1  } , )   , StoredType_int_ :: new_1 ( { 2  } , )   , ] )  } , None , None , None , None , )   , )
        });
    }
    {
        ({
            test_non_transparent_count_13 ( std_unordered_set_StoredType_int___non_transparent_hash__std_equal_to_StoredType_int____std_allocator_StoredType_int___ :: new_1 ( { Vec::from([ StoredType_int_ :: new_1 ( { 1  } , )   , StoredType_int_ :: new_1 ( { 2  } , )   , ] )  } , None , None , None , None , )   , )
        });
        ({
            test_non_transparent_count_14 ( std_unordered_set_StoredType_int___non_transparent_hash__std_equal_to_StoredType_int____std_allocator_StoredType_int___ :: new_1 ( { Vec::from([ StoredType_int_ :: new_1 ( { 1  } , )   , StoredType_int_ :: new_1 ( { 2  } , )   , ] )  } , None , None , None , None , )   , )
        });
    }
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
pub trait SearchedType_int_Impl {
    fn to_StoredType_int_(&self) -> StoredType_int_;
    fn get_value(&self) -> i32;
}
impl SearchedType_int_Impl for Ptr<SearchedType_int_> {
    fn to_StoredType_int_(&self) -> StoredType_int_ {
        (*self)
            .with(|__s| __s.conversions_.clone())
            .with_mut(|__v| {
                __v.prefix_inc();
            });
        return StoredType_int_::new_1({ (*self).with(|__s| __s.value_) });
    }
    fn get_value(&self) -> i32 {
        return (*self).with(|__s| __s.value_);
    }
}
pub trait StoredType_int_Impl {
    fn get_value(&self) -> i32;
}
impl StoredType_int_Impl for Ptr<StoredType_int_> {
    fn get_value(&self) -> i32 {
        return (*self).with(|__s| __s.value_);
    }
}
pub trait hash_implImpl {
    fn operator_call_1_int(&self, t: Ptr<SearchedType_int_>) -> usize;
    fn operator_call_2_int(&self, t: Ptr<StoredType_int_>) -> usize;
}
impl hash_implImpl for Ptr<hash_impl> {
    fn operator_call_1_int(&self, t: Ptr<SearchedType_int_>) -> usize {
        return (({ SearchedType_int_Impl::get_value(&t) }) as usize);
    }
    fn operator_call_2_int(&self, t: Ptr<StoredType_int_>) -> usize {
        return (({ StoredType_int_Impl::get_value(&t) }) as usize);
    }
}
pub fn __cpp2rust_init_globals() {}
