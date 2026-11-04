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
pub struct N_X {}
impl CppMove for N_X {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<std_vector_N_X____gnu_cxx__ExtPtr_allocator_N_X__> = Rc::new(RefCell::new(
        std_vector_N_X____gnu_cxx__ExtPtr_allocator_N_X__::new_1({ 5_u64 }, None),
    ));
    let w: Value<std_vector_N_X____gnu_cxx__ExtPtr_allocator_N_X__> = Rc::new(RefCell::new(
        std_vector_N_X____gnu_cxx__ExtPtr_allocator_N_X__::new_1({ 1_u64 }, None),
    ));
    &(*elem!(v.as_pointer(), 0_u64).upgrade().deref());
    &(*elem!(w.as_pointer(), 0_u64).upgrade().deref());
    &({ (*v.borrow()).size() });
    &({ (*v.borrow()).capacity() });
    ({ (*v.borrow()).resize_42(1_u64) });
    ({
        let ___position: __gnu_cxx___normal_iterator___gnu_cxx__Pointer_adapter___gnu_cxx__Relative_pointer_impl_const_N_X____std_vector_N_X____gnu_cxx__ExtPtr_allocator_N_X___   = { let __tmp_0 : Value<__gnu_cxx___normal_iterator___gnu_cxx__Pointer_adapter___gnu_cxx__Relative_pointer_impl_N_X____std_vector_N_X____gnu_cxx__ExtPtr_allocator_N_X___ > = Rc::new(RefCell::new(( { (*v.borrow()) . begin_28 ( ) } )  )); __gnu_cxx___normal_iterator___gnu_cxx__Pointer_adapter___gnu_cxx__Relative_pointer_impl_const_N_X____std_vector_N_X____gnu_cxx__ExtPtr_allocator_N_X___ :: new_1 ( { __tmp_0.as_pointer() } , )  }  ;
        let ___x: Value<N_X> = Rc::new(RefCell::new(<N_X>::default()));
        (*v.borrow()).insert_65(___position, ___x.as_pointer())
    });
    ({
        let ___position: __gnu_cxx___normal_iterator___gnu_cxx__Pointer_adapter___gnu_cxx__Relative_pointer_impl_const_N_X____std_vector_N_X____gnu_cxx__ExtPtr_allocator_N_X___   = { let __tmp_1 : Value<__gnu_cxx___normal_iterator___gnu_cxx__Pointer_adapter___gnu_cxx__Relative_pointer_impl_N_X____std_vector_N_X____gnu_cxx__ExtPtr_allocator_N_X___ > = Rc::new(RefCell::new(( { (*v.borrow()) . begin_28 ( ) } )  )); __gnu_cxx___normal_iterator___gnu_cxx__Pointer_adapter___gnu_cxx__Relative_pointer_impl_const_N_X____std_vector_N_X____gnu_cxx__ExtPtr_allocator_N_X___ :: new_1 ( { __tmp_1.as_pointer() } , )  }  ;
        let ___x: Value<N_X> = Rc::new(RefCell::new(<N_X>::default()));
        (*v.borrow()).insert_67(___position, 1_u64, ___x.as_pointer())
    });
    ({
        let ___position: __gnu_cxx___normal_iterator___gnu_cxx__Pointer_adapter___gnu_cxx__Relative_pointer_impl_const_N_X____std_vector_N_X____gnu_cxx__ExtPtr_allocator_N_X___   = { let __tmp_2 : Value<__gnu_cxx___normal_iterator___gnu_cxx__Pointer_adapter___gnu_cxx__Relative_pointer_impl_N_X____std_vector_N_X____gnu_cxx__ExtPtr_allocator_N_X___ > = Rc::new(RefCell::new(( { (*v.borrow()) . begin_28 ( ) } )  )); __gnu_cxx___normal_iterator___gnu_cxx__Pointer_adapter___gnu_cxx__Relative_pointer_impl_const_N_X____std_vector_N_X____gnu_cxx__ExtPtr_allocator_N_X___ :: new_1 ( { __tmp_2.as_pointer() } , )  }  ;
        let ___first: __gnu_cxx___normal_iterator___gnu_cxx__Pointer_adapter___gnu_cxx__Relative_pointer_impl_const_N_X____std_vector_N_X____gnu_cxx__ExtPtr_allocator_N_X___   = ( { (*w.borrow()) . begin_29 ( ) } )   ;
        let ___last: __gnu_cxx___normal_iterator___gnu_cxx__Pointer_adapter___gnu_cxx__Relative_pointer_impl_const_N_X____std_vector_N_X____gnu_cxx__ExtPtr_allocator_N_X___   = ( { (*w.borrow()) . end_31 ( ) } )   ;
        (*v.borrow()) . insert_68_class___gnu_cxx___normal_iterator_class___gnu_cxx__Pointer_adapter_class___gnu_cxx__Relative_pointer_impl_const_struct_N_X_____class_std_vector_struct_N_X__class___gnu_cxx__ExtPtr_allocator_struct_N_X______void ( ___position , ___first , ___last , )
    });
    (*v.borrow_mut()) = (*w.borrow()).clone();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
