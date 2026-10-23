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
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_1: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_2: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_3: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_std_pair_const_Static__int___std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_std_pair_const_Static__int___std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_Static__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<Static>,
}
impl min_pointer_Static__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { #[offset(0)] #[byte_size(8)] ptr_ : Ptr< min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  >  , }
impl min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { #[offset(0)] #[byte_size(8)] ptr_ : Ptr< min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  >  , }
impl min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_Static_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_pair_const_Static__int__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___
{}
thread_local!(
    pub static value_4: Value<()> = Rc::new(RefCell::new(
        (((((((is_same_v_5.with(|rc| *rc.borrow())) && (is_same_v_5.with(|rc| *rc.borrow())))
            && (is_same_v_6.with(|rc| *rc.borrow())))
            && (is_same_v_6.with(|rc| *rc.borrow())))
            && (is_same_v_7.with(|rc| *rc.borrow())))
            && (is_same_v_7.with(|rc| *rc.borrow())))
            && (is_same_v_8.with(|rc| *rc.borrow())))
            && (is_same_v_8.with(|rc| *rc.borrow())),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct node_compatibility_table_int__int__std_less_int___std_less_int___std_hash_int___std_hash_int___std_equal_to_int___std_equal_to_int___std_allocator_int___std_allocator_std_pair_const_int__int___
{}
thread_local!(
    pub static value_9: Value<()> = Rc::new(RefCell::new(
        (((((((is_same_v_5.with(|rc| *rc.borrow())) && (is_same_v_5.with(|rc| *rc.borrow())))
            && (is_same_v_6.with(|rc| *rc.borrow())))
            && (is_same_v_6.with(|rc| *rc.borrow())))
            && (is_same_v_7.with(|rc| *rc.borrow())))
            && (is_same_v_7.with(|rc| *rc.borrow())))
            && (is_same_v_8.with(|rc| *rc.borrow())))
            && (is_same_v_8.with(|rc| *rc.borrow())),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct node_compatibility_table_int__int__std_less_int___my_compare_int___std_hash_int___my_hash_int___std_equal_to_int___my_equal_int___std_allocator_int___std_allocator_std_pair_const_int__int___
{}
thread_local!(
    pub static value_10: Value<()> = Rc::new(RefCell::new(
        (((((((is_same_v_11.with(|rc| *rc.borrow()))
            && (is_same_v_11.with(|rc| *rc.borrow())))
            && (is_same_v_12.with(|rc| *rc.borrow())))
            && (is_same_v_12.with(|rc| *rc.borrow())))
            && (is_same_v_13.with(|rc| *rc.borrow())))
            && (is_same_v_13.with(|rc| *rc.borrow())))
            && (is_same_v_14.with(|rc| *rc.borrow())))
            && (is_same_v_14.with(|rc| *rc.borrow())),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct node_compatibility_table_Static__int__my_compare_Static___std_less_Static___my_hash_Static___std_hash_Static___my_equal_Static___std_equal_to_Static___min_allocator_Static___min_allocator_std_pair_const_Static__int___
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct my_hash_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct my_hash_Static_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct my_compare_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct my_compare_Static_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct my_equal_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct my_equal_Static_ {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Static {}
const _: () = assert!(
    true,
    "node_compatibility_table< int,\n                              int,\n                              std::less<int>,\n                              std::less<int>,\n                              std::hash<int>,\n                              std::hash<int>,\n                              std::equal_to<int>,\n                              std::equal_to<int>,\n                              std::allocator<int>,\n                              std::allocator<std::pair<const int, int>>>::value"
);
const _: () = assert!(
    true,
    "node_compatibility_table<int,\n                             int,\n                             std::less<int>,\n                             my_compare<int>,\n                             std::hash<int>,\n                             my_hash<int>,\n                             std::equal_to<int>,\n                             my_equal<int>,\n                             std::allocator<int>,\n                             std::allocator<std::pair<const int, int>>>::value"
);
const _: () = assert!(
    true,
    "node_compatibility_table< Static,\n                              int,\n                              my_compare<Static>,\n                              std::less<Static>,\n                              my_hash<Static>,\n                              std::hash<Static>,\n                              my_equal<Static>,\n                              std::equal_to<Static>,\n                              min_allocator<Static>,\n                              min_allocator<std::pair<const Static, int>>>::value"
);
pub fn test_node_handle_operations_15() -> () {
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let nt1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let nt2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    ({ swap_16(nt1.as_pointer(), nt2.as_pointer()) });
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_node_handle_operations_17() -> () {
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let nt1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let nt2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    ({ swap_18(nt1.as_pointer(), nt2.as_pointer()) });
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_node_handle_operations_19() -> () {
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let nt1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let nt2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    ({ swap_20(nt1.as_pointer(), nt2.as_pointer()) });
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_node_handle_operations_21() -> () {
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let nt1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let nt2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    ({ swap_22(nt1.as_pointer(), nt2.as_pointer()) });
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_node_handle_operations_multi_23() -> () {
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let nt1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let nt2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    ({ swap_16(nt1.as_pointer(), nt2.as_pointer()) });
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_node_handle_operations_multi_24() -> () {
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let nt1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let nt2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    ({ swap_18(nt1.as_pointer(), nt2.as_pointer()) });
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_node_handle_operations_multi_25() -> () {
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let nt1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let nt2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    ({ swap_20(nt1.as_pointer(), nt2.as_pointer()) });
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_node_handle_operations_multi_26() -> () {
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let nt1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let nt2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    ({ swap_22(nt1.as_pointer(), nt2.as_pointer()) });
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_typedef_27() -> () {
    return true;
}
pub fn test_typedef_28() -> () {
    return true;
}
pub fn test_typedef_29() -> () {
    return true;
}
pub fn test_typedef_30() -> () {
    return true;
}
pub fn test_insert_return_type_31() -> () {
    ({ test_typedef_27() });
    return true;
}
pub fn test_insert_return_type_32() -> () {
    ({ test_typedef_28() });
    return true;
}
pub fn test_insert_return_type_33() -> () {
    ({ test_typedef_29() });
    return true;
}
pub fn test_insert_return_type_34() -> () {
    ({ test_typedef_30() });
    return true;
}
pub fn test_35() -> () {
    ({ test_node_handle_operations_15() });
    ({ test_node_handle_operations_multi_23() });
    ({ test_node_handle_operations_17() });
    {
        ({ test_node_handle_operations_multi_24() });
        ({ test_node_handle_operations_19() });
        ({ test_node_handle_operations_multi_25() });
        ({ test_node_handle_operations_21() });
        ({ test_node_handle_operations_multi_26() });
    }
    ({ test_insert_return_type_31() });
    ({ test_insert_return_type_32() });
    {
        ({ test_insert_return_type_33() });
        ({ test_insert_return_type_34() });
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
    ({ test_35() });
    return 0_();
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__;
pub trait min_allocator_Static_Impl {
    fn allocate(&self, mut n: ()) -> min_pointer_Static__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_Static__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_Static_Impl for Ptr<min_allocator_Static_> {}
pub trait min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___Impl
{
    fn allocate ( &self , mut n : ()  , ) -> min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p : min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___Impl for Ptr<min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___> {
}
pub trait min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___Impl
{
    fn allocate ( &self , mut n : ()  , ) -> min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p : min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___Impl for Ptr<min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___> {
}
pub trait min_allocator_std_pair_const_Static__int__Impl {
    fn allocate(
        &self,
        mut n: (),
    ) -> min_pointer_std_pair_const_Static__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_std_pair_const_Static__int___std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_std_pair_const_Static__int__Impl
    for Ptr<min_allocator_std_pair_const_Static__int__>
{
}
pub trait min_pointer_Static__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<Static> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<Static> {
        unimplemented!()
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_Static__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_Static__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_Static__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_Static__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_Static__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_Static__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_Static__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_Static__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<Static> {
        unimplemented!()
    }
}
impl min_pointer_Static__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_Static__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl
{
    fn to_(&self) -> () {
        unimplemented!()
    }    fn operator_deref ( &self , ) -> Ptr< min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_arrow ( &self , ) -> Ptr< min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_inc_9 ( &self , mut _a0 : ()  , ) -> min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_dec_11 ( &self , mut _a0 : ()  , ) -> min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_add_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_sub_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_add ( &self , mut n : ()  , ) -> min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_sub ( &self , mut n : ()  , ) -> min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_index ( &self , mut n : ()  , ) -> Ptr< min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }
}
impl min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__> {
}
pub trait min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl
{
    fn to_(&self) -> () {
        unimplemented!()
    }    fn operator_deref ( &self , ) -> Ptr< min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_arrow ( &self , ) -> Ptr< min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_inc_9 ( &self , mut _a0 : ()  , ) -> min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_dec_11 ( &self , mut _a0 : ()  , ) -> min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_add_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_sub_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_add ( &self , mut n : ()  , ) -> min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_sub ( &self , mut n : ()  , ) -> min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_index ( &self , mut n : ()  , ) -> Ptr< min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }
}
impl min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__> {
}
pub trait min_pointer_std_pair_const_Static__int___std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_std_pair_const_Static__int___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_std_pair_const_Static__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_std_pair_const_Static__int___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_std_pair_const_Static__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_std_pair_const_Static__int___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_std_pair_const_Static__int___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_std_pair_const_Static__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_std_pair_const_Static__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_std_pair_const_Static__int___std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_std_pair_const_Static__int___std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait my_compare_Static_Impl {
    fn operator_call(&self, _a0: Ptr<Static>, _a1: Ptr<Static>) -> () {
        unimplemented!()
    }
}
impl my_compare_Static_Impl for Ptr<my_compare_Static_> {}
pub trait my_compare_int_Impl {
    fn operator_call(&self, _a0: Ptr<()>, _a1: Ptr<()>) -> () {
        unimplemented!()
    }
}
impl my_compare_int_Impl for Ptr<my_compare_int_> {}
pub trait my_equal_Static_Impl {
    fn operator_call(&self, _a0: Ptr<Static>, _a1: Ptr<Static>) -> () {
        unimplemented!()
    }
}
impl my_equal_Static_Impl for Ptr<my_equal_Static_> {}
pub trait my_equal_int_Impl {
    fn operator_call(&self, _a0: Ptr<()>, _a1: Ptr<()>) -> () {
        unimplemented!()
    }
}
impl my_equal_int_Impl for Ptr<my_equal_int_> {}
pub trait my_hash_Static_Impl {
    fn operator_call(&self, _a0: Ptr<Static>) -> () {
        unimplemented!()
    }
}
impl my_hash_Static_Impl for Ptr<my_hash_Static_> {}
pub trait my_hash_int_Impl {
    fn operator_call(&self, _a0: Ptr<()>) -> () {
        unimplemented!()
    }
}
impl my_hash_int_Impl for Ptr<my_hash_int_> {}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
    let _ = value_4.with(|_| ());
    let _ = value_9.with(|_| ());
    let _ = value_10.with(|_| ());
}
