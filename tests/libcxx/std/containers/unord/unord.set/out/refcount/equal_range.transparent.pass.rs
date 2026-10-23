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
            if operator_lt_0(
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
            } else if operator_lt_0(
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
            !(operator_lt_0(
                Rc::new(RefCell::new(C2Int {
                    i_: self.i_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(C2Int {
                    i_: other.i_.clone(),
                }))
                .as_pointer(),
            )) && !(operator_lt_0(
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
pub fn operator_lt_1(mut rhs: (), lhs: Ptr<C2Int>) -> () {
    return ({ rhs } < { ({ C2IntImpl::get(&lhs) }) });
}
pub fn operator_lt_0(rhs: Ptr<C2Int>, lhs: Ptr<C2Int>) -> () {
    return ({ ({ C2IntImpl::get(&rhs) }) } < { ({ C2IntImpl::get(&lhs) }) });
}
pub fn operator_lt_2(rhs: Ptr<C2Int>, mut lhs: ()) -> () {
    return ({ ({ C2IntImpl::get(&rhs) }) } < { lhs });
}
pub fn operator_eq_3(lhs: Ptr<StoredType_int_>, rhs: Ptr<StoredType_int_>) -> () {
    return ({ lhs.with(|__s| __s.value_) } == { rhs.with(|__s| __s.value_) });
}
pub fn operator_eq_4(lhs: Ptr<StoredType_int_>, rhs: Ptr<SearchedType_int_>) -> () {
    return ({ lhs.with(|__s| __s.value_) } == { ({ SearchedType_int_Impl::get_value(&rhs) }) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct StoredType_int_ {
    #[offset(0)]
    value_: (),
}
impl StoredType_int_ {
    pub fn new_1(mut value: ()) -> Self {
        Self { value_: value }
    }
}
impl std::cmp::PartialEq for StoredType_int_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_3(
                Rc::new(RefCell::new(StoredType_int_ {
                    value_: self.value_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(StoredType_int_ {
                    value_: other.value_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for StoredType_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct SearchedType_int_ {
    #[offset(0)]
    value_: (),
    #[offset(8)]
    #[byte_size(8)]
    conversions_: Ptr<()>,
}
impl SearchedType_int_ {
    pub fn new(mut value: (), mut counter: Ptr<()>) -> Self {
        Self {
            value_: value,
            conversions_: (counter).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct hash_impl {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct non_transparent_hash {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_hash {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_hash_final {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_equal_final {}
pub fn test_transparent_equal_range_5(c: ()) {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let conversions: Value<()> = Rc::new(RefCell::new(0_()));
    let iters: Value<()> = Rc::new(RefCell::new(
        ({
            let ___k: Value<SearchedType_int_> =
                Rc::new(RefCell::new(SearchedType_int_::new({ 1_() }, {
                    (conversions.as_pointer())
                })));
            (*c.borrow()) . equal_range_72_struct_SearchedType_int__struct_transparent_hash_struct_std_equal_to_void__targ ( ___k.as_pointer() , )
        }),
    ));
    unimplemented!();
    (*iters.borrow_mut()) = ({
        let ___k: Value<SearchedType_int_> =
            Rc::new(RefCell::new(SearchedType_int_::new({ 2_() }, {
                (conversions.as_pointer())
            })));
        (*c.borrow()) . equal_range_72_struct_SearchedType_int__struct_transparent_hash_struct_std_equal_to_void__targ ( ___k.as_pointer() , )
    });
    unimplemented!();
    (*iters.borrow_mut()) = ({
        let ___k: Value<SearchedType_int_> =
            Rc::new(RefCell::new(SearchedType_int_::new({ 3_() }, {
                (conversions.as_pointer())
            })));
        (*c.borrow()) . equal_range_72_struct_SearchedType_int__struct_transparent_hash_struct_std_equal_to_void__targ ( ___k.as_pointer() , )
    });
    unimplemented!();
    unimplemented!();
}
pub fn test_transparent_equal_range_6(c: ()) {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let conversions: Value<()> = Rc::new(RefCell::new(0_()));
    let iters: Value<()> = Rc::new(RefCell::new(
        ({
            let ___k: Value<SearchedType_int_> =
                Rc::new(RefCell::new(SearchedType_int_::new({ 1_() }, {
                    (conversions.as_pointer())
                })));
            (*c.borrow()) . equal_range_73_struct_SearchedType_int__struct_transparent_hash_struct_std_equal_to_void__targ ( ___k.as_pointer() , )
        }),
    ));
    unimplemented!();
    (*iters.borrow_mut()) = ({
        let ___k: Value<SearchedType_int_> =
            Rc::new(RefCell::new(SearchedType_int_::new({ 2_() }, {
                (conversions.as_pointer())
            })));
        (*c.borrow()) . equal_range_73_struct_SearchedType_int__struct_transparent_hash_struct_std_equal_to_void__targ ( ___k.as_pointer() , )
    });
    unimplemented!();
    (*iters.borrow_mut()) = ({
        let ___k: Value<SearchedType_int_> =
            Rc::new(RefCell::new(SearchedType_int_::new({ 3_() }, {
                (conversions.as_pointer())
            })));
        (*c.borrow()) . equal_range_73_struct_SearchedType_int__struct_transparent_hash_struct_std_equal_to_void__targ ( ___k.as_pointer() , )
    });
    unimplemented!();
    unimplemented!();
}
pub fn test_transparent_equal_range_7(c: ()) {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let conversions: Value<()> = Rc::new(RefCell::new(0_()));
    let iters: Value<()> = Rc::new(RefCell::new(
        ({
            let ___k: Value<SearchedType_int_> =
                Rc::new(RefCell::new(SearchedType_int_::new({ 1_() }, {
                    (conversions.as_pointer())
                })));
            (*c.borrow()) . equal_range_72_struct_SearchedType_int__struct_transparent_hash_final_struct_transparent_equal_final_targ ( ___k.as_pointer() , )
        }),
    ));
    unimplemented!();
    (*iters.borrow_mut()) = ({
        let ___k: Value<SearchedType_int_> =
            Rc::new(RefCell::new(SearchedType_int_::new({ 2_() }, {
                (conversions.as_pointer())
            })));
        (*c.borrow()) . equal_range_72_struct_SearchedType_int__struct_transparent_hash_final_struct_transparent_equal_final_targ ( ___k.as_pointer() , )
    });
    unimplemented!();
    (*iters.borrow_mut()) = ({
        let ___k: Value<SearchedType_int_> =
            Rc::new(RefCell::new(SearchedType_int_::new({ 3_() }, {
                (conversions.as_pointer())
            })));
        (*c.borrow()) . equal_range_72_struct_SearchedType_int__struct_transparent_hash_final_struct_transparent_equal_final_targ ( ___k.as_pointer() , )
    });
    unimplemented!();
    unimplemented!();
}
pub fn test_transparent_equal_range_8(c: ()) {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let conversions: Value<()> = Rc::new(RefCell::new(0_()));
    let iters: Value<()> = Rc::new(RefCell::new(
        ({
            let ___k: Value<SearchedType_int_> =
                Rc::new(RefCell::new(SearchedType_int_::new({ 1_() }, {
                    (conversions.as_pointer())
                })));
            (*c.borrow()) . equal_range_73_struct_SearchedType_int__struct_transparent_hash_final_struct_transparent_equal_final_targ ( ___k.as_pointer() , )
        }),
    ));
    unimplemented!();
    (*iters.borrow_mut()) = ({
        let ___k: Value<SearchedType_int_> =
            Rc::new(RefCell::new(SearchedType_int_::new({ 2_() }, {
                (conversions.as_pointer())
            })));
        (*c.borrow()) . equal_range_73_struct_SearchedType_int__struct_transparent_hash_final_struct_transparent_equal_final_targ ( ___k.as_pointer() , )
    });
    unimplemented!();
    (*iters.borrow_mut()) = ({
        let ___k: Value<SearchedType_int_> =
            Rc::new(RefCell::new(SearchedType_int_::new({ 3_() }, {
                (conversions.as_pointer())
            })));
        (*c.borrow()) . equal_range_73_struct_SearchedType_int__struct_transparent_hash_final_struct_transparent_equal_final_targ ( ___k.as_pointer() , )
    });
    unimplemented!();
    unimplemented!();
}
pub fn test_non_transparent_equal_range_9(c: ()) {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let conversions: Value<()> = Rc::new(RefCell::new(0_()));
    let iters: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    (*iters.borrow_mut()) = unimplemented!();
    unimplemented!();
    unimplemented!();
    (*iters.borrow_mut()) = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_non_transparent_equal_range_10(c: ()) {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let conversions: Value<()> = Rc::new(RefCell::new(0_()));
    let iters: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    (*iters.borrow_mut()) = unimplemented!();
    unimplemented!();
    unimplemented!();
    (*iters.borrow_mut()) = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_non_transparent_equal_range_11(c: ()) {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let conversions: Value<()> = Rc::new(RefCell::new(0_()));
    let iters: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    (*iters.borrow_mut()) = unimplemented!();
    unimplemented!();
    unimplemented!();
    (*iters.borrow_mut()) = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_non_transparent_equal_range_12(c: ()) {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let conversions: Value<()> = Rc::new(RefCell::new(0_()));
    let iters: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    (*iters.borrow_mut()) = unimplemented!();
    unimplemented!();
    unimplemented!();
    (*iters.borrow_mut()) = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_non_transparent_equal_range_13(c: ()) {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let conversions: Value<()> = Rc::new(RefCell::new(0_()));
    let iters: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    (*iters.borrow_mut()) = unimplemented!();
    unimplemented!();
    unimplemented!();
    (*iters.borrow_mut()) = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_non_transparent_equal_range_14(c: ()) {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let conversions: Value<()> = Rc::new(RefCell::new(0_()));
    let iters: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    (*iters.borrow_mut()) = unimplemented!();
    unimplemented!();
    unimplemented!();
    (*iters.borrow_mut()) = unimplemented!();
    unimplemented!();
    unimplemented!();
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
        ({ test_transparent_equal_range_5(unimplemented!()) });
        ({ test_transparent_equal_range_6(unimplemented!()) });
    }
    {
        ({ test_transparent_equal_range_7(unimplemented!()) });
        ({ test_transparent_equal_range_8(unimplemented!()) });
    }
    {
        ({ test_non_transparent_equal_range_9(unimplemented!()) });
        ({ test_non_transparent_equal_range_10(unimplemented!()) });
    }
    {
        ({ test_non_transparent_equal_range_11(unimplemented!()) });
        ({ test_non_transparent_equal_range_12(unimplemented!()) });
    }
    {
        ({ test_non_transparent_equal_range_13(unimplemented!()) });
        ({ test_non_transparent_equal_range_14(unimplemented!()) });
    }
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
pub trait SearchedType_int_Impl {
    fn to_StoredType_int_(&self) -> StoredType_int_;
    fn get_value(&self) -> ();
}
impl SearchedType_int_Impl for Ptr<SearchedType_int_> {
    fn to_StoredType_int_(&self) -> StoredType_int_ {
        (*self)
            .with(|__s| __s.conversions_.clone())
            .with_mut(|__v| __v.prefix_inc());
        return StoredType_int_::new_1({ (*self).with(|__s| __s.value_) });
    }
    fn get_value(&self) -> () {
        return (*self).with(|__s| __s.value_);
    }
}
pub trait StoredType_int_Impl {
    fn get_value(&self) -> ();
}
impl StoredType_int_Impl for Ptr<StoredType_int_> {
    fn get_value(&self) -> () {
        return (*self).with(|__s| __s.value_);
    }
}
pub trait hash_implImpl {
    fn operator_call_1_int(&self, t: Ptr<SearchedType_int_>) -> ();
    fn operator_call_2_int(&self, t: Ptr<StoredType_int_>) -> ();
}
impl hash_implImpl for Ptr<hash_impl> {
    fn operator_call_1_int(&self, t: Ptr<SearchedType_int_>) -> () {
        return ({ SearchedType_int_Impl::get_value(&t) });
    }
    fn operator_call_2_int(&self, t: Ptr<StoredType_int_>) -> () {
        return ({ StoredType_int_Impl::get_value(&t) });
    }
}
pub fn __cpp2rust_init_globals() {}
