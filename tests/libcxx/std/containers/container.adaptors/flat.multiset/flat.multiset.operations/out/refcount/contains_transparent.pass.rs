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
pub struct test_macros_detail_is_same_bool__bool_ {}
impl CppMove for test_macros_detail_is_same_bool__bool_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(1)]
pub struct support_double_move_tracker {
    #[offset(0)]
    moved_from_: bool,
}
impl support_double_move_tracker {
    pub fn new() -> Self {
        Self { moved_from_: false }
    }
    pub fn move_from(other: Ptr<support_double_move_tracker>) -> Self {
        let __this: support_double_move_tracker = Self { moved_from_: false };
        assert!(!(other.with(|__s| __s.moved_from_)));
        field!(other, moved_from_).write(true);
        __this
    }
}
impl CppMove for support_double_move_tracker {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for support_double_move_tracker {
    fn default() -> Self {
        { support_double_move_tracker::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp17_output_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_output_iterator_int_ptr_ {
    pub fn move_from(_a0: Ptr<cpp17_output_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppMove for cpp17_output_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
const _: () = assert!(
    true,
    "std::output_iterator<cpp17_output_iterator<int*>, int>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {
    pub fn move_from(_a0: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppMove for cpp17_input_iterator_int_ptr__int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct three_way_random_access_iterator_const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl three_way_random_access_iterator_const_int_ptr_ {
    pub fn move_from(_a0: Ptr<three_way_random_access_iterator_const_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppMove for three_way_random_access_iterator_const_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}pub fn operator_add_1 ( x : three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  , mut n : isize  , ) -> three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_{
    let x : Value<three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ >  = Rc::new(RefCell::new(x)) ;
    ({
        three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_Impl :: operator_add_assign ( &x .as_pointer()  , n  , )
    });
    return three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ :: move_from ( { x .as_pointer()  } , )    ;
}pub fn operator_add_2 ( mut n : isize  , x : three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  , ) -> three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_{
    let x : Value<three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ >  = Rc::new(RefCell::new(x)) ;
    ({
        three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_Impl :: operator_add_assign ( &x .as_pointer()  , n  , )
    });
    return three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ :: move_from ( { x .as_pointer()  } , )    ;
}pub fn operator_sub_3 ( x : three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  , mut n : isize  , ) -> three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_{
    let x : Value<three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ >  = Rc::new(RefCell::new(x)) ;
    ({
        three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_Impl :: operator_sub_assign ( &x .as_pointer()  , n  , )
    });
    return three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ :: move_from ( { x .as_pointer()  } , )    ;
}
pub fn operator_sub_4(
    mut x : three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_,
    mut y : three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_,
) -> isize {
    return (((x.it_).clone() - (y.it_).clone()) as i64 as isize);
}
pub fn operator_eq_5(
    x : Ptr< three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  >,
    y : Ptr< three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  >,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_lt_6(
    x : Ptr< three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  >,
    y : Ptr< three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  >,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } < { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_
{
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<Vec<i8>>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { pub fn new_1 ( mut it : Ptr< Vec<i8  >  >  , ) -> Self { Self { it_ : (it ).clone() , tracker_ : support_double_move_tracker :: new ( )   , }  } pub fn move_from ( _a0 : Ptr< three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  > , ) -> Self { Self { it_ : { (*_a0.upgrade().deref()) . it_ .clone() }  , tracker_ : support_double_move_tracker :: move_from ( { field_ptr!(_a0 , tracker_)  } , )   , }  } }
impl std::cmp::Ord for  three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { fn cmp(&self, other: &Self) -> std::cmp::Ordering {  { libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| if operator_lt_6(__lhs.clone(), __rhs.clone()) { std::cmp::Ordering::Less } else if operator_lt_6(__rhs.clone(), __lhs.clone()) { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Equal }) }}}
impl std::cmp::PartialOrd for three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }}
impl std::cmp::PartialEq for three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { fn eq(&self, other: &Self) -> bool {  { libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| operator_eq_5(__lhs.clone(), __rhs.clone())) }}}
impl std::cmp::Eq for three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ {}
impl CppAdvance for three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { fn cpp_inc(__it: &Ptr<Self>) { __it.operator_inc_7(); } fn cpp_dec(__it: &Ptr<Self>) { __it.operator_dec_8(); } }
impl CppIterator<Vec<i8  >  > for three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { fn iter_deref(&self) -> Ptr<Vec<i8  >  > { libcc2rs::with_record_ptr(&mut self.clone(), |__it| three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_Impl::operator_deref(__it)) } fn iter_inc(&mut self) -> &mut Self { libcc2rs::with_record_ptr(self, |__it| { three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_Impl::operator_inc_7(__it); }); self } }
impl CppInputIterator<Vec<i8  >  > for three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ {}
impl CppBidirectionalIterator<Vec<i8  >  > for three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { fn iter_dec(&mut self) -> &mut Self { libcc2rs::with_record_ptr(self, |__it| { three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_Impl::operator_dec_8(__it); }); self } }
impl CppRandomAccessIterator<Vec<i8  >  > for three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { fn iter_add_assign(&mut self, n: isize) -> Self { libcc2rs::with_record_ptr(self, |__it| { three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_Impl::operator_add_assign(__it, n as _); }); self.clone() } fn iter_distance(&self, from: &Self) -> isize { libcc2rs::with_record_ptrs(self, from, |__lhs, __rhs| (operator_sub_4(__lhs.read(), __rhs.read())) as isize) } }
impl CppMove for three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { fn cpp_move(src: Ptr<Self>) -> Self { Self::move_from(src) } }
impl three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { pub fn new_1 ( u : Ptr< three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  > , ) -> Self { let __this : three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ = Self { it_ : u.with(|__s| __s . it_ .clone())  , tracker_ : support_double_move_tracker :: move_from ( { field_ptr!(u , tracker_)  } , )   , }  ;
 field!(u, it_) .write( Ptr::<Vec<i8  >>::null()  ) ;
  __this } }pub fn operator_add_7 ( x : three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  , mut n : isize  , ) -> three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_{
    let x : Value<three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ >  = Rc::new(RefCell::new(x)) ;
    ({
        three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_Impl :: operator_add_assign ( &x .as_pointer()  , n  , )
    });
    return three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ :: move_from ( { x .as_pointer()  } , )    ;
}
pub fn operator_sub_8(
    mut x : three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_,
    mut y : three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_,
) -> isize {
    return (((x.it_).clone() - (y.it_).clone()) as i64 as isize);
}
pub fn operator_eq_9(
    x : Ptr< three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  >,
    y : Ptr< three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  >,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CppFrom, MoveAssign, Default)]
#[cpp_from_move(three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  , new_1, ptr)]
#[byte_size(16)]
pub struct three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_
{
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<Vec<i8>>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { pub fn new_2 ( mut it : Ptr< Vec<i8  >  >  , ) -> Self { Self { it_ : (it ).clone() , tracker_ : support_double_move_tracker :: new ( )   , }  } pub fn move_from ( _a0 : Ptr< three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  > , ) -> Self { Self { it_ : { (*_a0.upgrade().deref()) . it_ .clone() }  , tracker_ : support_double_move_tracker :: move_from ( { field_ptr!(_a0 , tracker_)  } , )   , }  } }
impl std::cmp::PartialEq for three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { fn eq(&self, other: &Self) -> bool {  { libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| operator_eq_9(__lhs.clone(), __rhs.clone())) }}}
impl std::cmp::Eq for three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ {}
impl CppAdvance for three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { fn cpp_inc(__it: &Ptr<Self>) { __it.operator_inc_7(); } fn cpp_dec(__it: &Ptr<Self>) { __it.operator_dec_8(); } }
impl CppIterator<Vec<i8  >  > for three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { fn iter_deref(&self) -> Ptr<Vec<i8  >  > { libcc2rs::with_record_ptr(&mut self.clone(), |__it| three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_Impl::operator_deref(__it)) } fn iter_inc(&mut self) -> &mut Self { libcc2rs::with_record_ptr(self, |__it| { three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_Impl::operator_inc_7(__it); }); self } }
impl CppInputIterator<Vec<i8  >  > for three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ {}
impl CppBidirectionalIterator<Vec<i8  >  > for three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { fn iter_dec(&mut self) -> &mut Self { libcc2rs::with_record_ptr(self, |__it| { three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_Impl::operator_dec_8(__it); }); self } }
impl CppMove for three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { fn cpp_move(src: Ptr<Self>) -> Self { Self::move_from(src) } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp20_random_access_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp20_random_access_iterator_int_ptr_ {
    pub fn move_from(_a0: Ptr<cpp20_random_access_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppMove for cpp20_random_access_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
const _: () = assert!(
    true,
    "std::random_access_iterator<cpp20_random_access_iterator<int*>>"
);
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp20_input_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_int_ptr_ {
    pub fn move_from(_a0: Ptr<cpp20_input_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppMove for cpp20_input_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
const _: () = assert!(true, "std::input_iterator<cpp20_input_iterator<int*>>");
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp20_output_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp20_output_iterator_int_ptr_ {
    pub fn move_from(_a0: Ptr<cpp20_output_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppMove for cpp20_output_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
const _: () = assert!(
    true,
    "std::output_iterator<cpp20_output_iterator<int*>, int>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct IteratorOpCounts {
    #[offset(0)]
    pub increments: usize,
    #[offset(8)]
    pub decrements: usize,
    #[offset(16)]
    pub zero_moves: usize,
    #[offset(24)]
    pub equal_cmps: usize,
}
impl CppMove for IteratorOpCounts {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for IteratorOpCounts {
    fn default() -> Self {
        {
            IteratorOpCounts {
                increments: 0_usize,
                decrements: 0_usize,
                zero_moves: 0_usize,
                equal_cmps: 0_usize,
            }
        }
    }
}
pub fn operator_eq_10(
    _a0: Ptr<forward_sized_iterator_int_ptr_>,
    _a1: Ptr<forward_sized_iterator_int_ptr_>,
) -> bool {
    return ({ _a0.with(|__s| __s.it_.clone()) } == { _a1.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct forward_sized_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    pub it_: Ptr<i32>,
}
impl forward_sized_iterator_int_ptr_ {}
impl CppEq for forward_sized_iterator_int_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_10(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for forward_sized_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for forward_sized_iterator_int_ptr_ {
    fn default() -> Self {
        {
            forward_sized_iterator_int_ptr_ {
                it_: Ptr::<i32>::null(),
            }
        }
    }
}
const _: () = assert!(true, "std::forward_iterator<forward_sized_iterator<>>");
const _: () = assert!(
    true,
    "std::sized_sentinel_for<forward_sized_iterator<>, forward_sized_iterator<>>"
);
pub fn operator_add_11(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_12(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_11(_i, n)
    });
}
pub fn iter_swap_13(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap!((a.ptr_).clone(), (b.ptr_).clone());
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| {
            __v.prefix_inc();
        });
    }
}
pub fn iter_move_14(iter: adl_Iterator) -> Ptr<i32> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| {
            __v.prefix_inc();
        });
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) });
}
pub fn operator_eq_15(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> bool {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_16(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> std::cmp::Ordering {
    return std::cmp::Ord::cmp(
        &(lhs.with(|__s| __s.ptr_.clone())),
        &(rhs.with(|__s| __s.ptr_.clone())),
    );
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct adl_Iterator {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(8)]
    iter_moves_: Ptr<i32>,
    #[offset(16)]
    #[byte_size(8)]
    iter_swaps_: Ptr<i32>,
}
impl adl_Iterator {
    fn new_1(mut p: Ptr<i32>, mut iter_moves: Ptr<i32>, mut iter_swaps: Ptr<i32>) -> Self {
        Self {
            ptr_: (p).clone(),
            iter_moves_: (iter_moves).clone(),
            iter_swaps_: (iter_swaps).clone(),
        }
    }
    pub fn TrackMoves(mut p: Ptr<i32>, iter_moves: Ptr<i32>) -> adl_Iterator {
        return adl_Iterator::new_1({ (p).clone() }, { (iter_moves).clone() }, {
            Ptr::<i32>::null()
        });
    }
    pub fn TrackSwaps_4(iter_swaps: Ptr<i32>) -> adl_Iterator {
        return adl_Iterator::new_1({ Ptr::<i32>::null() }, { Ptr::<i32>::null() }, {
            (iter_swaps).clone()
        });
    }
    pub fn TrackSwaps_5(mut p: Ptr<i32>, iter_swaps: Ptr<i32>) -> adl_Iterator {
        return adl_Iterator::new_1({ (p).clone() }, { Ptr::<i32>::null() }, {
            (iter_swaps).clone()
        });
    }
}
impl std::cmp::Ord for adl_Iterator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_cmp_16(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_15(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for adl_Iterator {}
impl CppAdvance for adl_Iterator {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_14();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_16();
    }
}
impl CppIterator<i32> for adl_Iterator {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            adl_IteratorImpl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            adl_IteratorImpl::operator_inc_14(__it);
        });
        self
    }
}
impl CppInputIterator<i32> for adl_Iterator {}
impl CppBidirectionalIterator<i32> for adl_Iterator {
    fn iter_dec(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            adl_IteratorImpl::operator_dec_16(__it);
        });
        self
    }
}
impl CppRandomAccessIterator<i32> for adl_Iterator {
    fn iter_add_assign(&mut self, n: isize) -> Self {
        libcc2rs::with_record_ptr(self, |__it| {
            adl_IteratorImpl::operator_add_assign(__it, n as _);
        });
        self.clone()
    }
    fn iter_distance(&self, from: &Self) -> isize {
        libcc2rs::with_record_ptrs(self, from, |__lhs, __rhs| {
            (adl_IteratorImpl::operator_sub_11(__lhs, __rhs.read())) as isize
        })
    }
}
impl CppMove for adl_Iterator {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for adl_Iterator {
    fn default() -> Self {
        {
            adl_Iterator {
                ptr_: Ptr::<i32>::null(),
                iter_moves_: Ptr::<i32>::null(),
                iter_swaps_: Ptr::<i32>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct rvalue_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<Ptr<i32>>,
}
impl rvalue_iterator_int_ptr_ {}
impl CppLess for rvalue_iterator_int_ptr_ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        rvalue_iterator_int_ptr_Impl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for rvalue_iterator_int_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        rvalue_iterator_int_ptr_Impl::operator_cmp(__lhs, __rhs.clone())
            == std::cmp::Ordering::Equal
    }
}
impl CppMove for rvalue_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
const _: () = assert!(true, "std::random_access_iterator<rvalue_iterator<int*>>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Ptr< i8  >  , new, value)]
#[byte_size(16)]
pub struct util_iterator_error {}
impl util_iterator_error {
    pub fn new(mut what: Ptr<i8>) -> Self {
        Self {}
    }
}
impl CppMove for util_iterator_error {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__
{
    #[offset(0)]
    #[byte_size(24)]
    data_: Vec<i32>,
}
impl MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__ { pub fn move_from ( _a0 : Ptr< MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__  > , ) -> Self { Self { data_ : field!(_a0, data_).with_mut(std::mem::take)  , }  } }
impl CppMove for MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__ { fn cpp_move(src: Ptr<Self>) -> Self { Self::move_from(src) } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__
{
    #[offset(0)]
    #[byte_size(24)]
    data_: Vec<Vec<i8>>,
}
impl MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__ { pub fn move_from ( _a0 : Ptr< MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__  > , ) -> Self { Self { data_ : field!(_a0, data_).with_mut(std::mem::take)  , }  } }
impl CppMove for MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__ { fn cpp_move(src: Ptr<Self>) -> Self { Self::move_from(src) } }
const _: () = assert!(!(false), "!HasAllocatorType<MinSequenceContainer<int>>");
const _: () = assert!(!(false), "!HasEmplace<MinSequenceContainer<int>>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Transparent_int__false_ {
    #[offset(0)]
    pub t: i32,
}
impl CppMove for Transparent_int__false_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(32)]
pub struct Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_ {
    #[offset(0)]
    #[byte_size(32)]
    pub t: Vec<i8>,
}
impl Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_ {
    pub fn move_from(
        _a0: Ptr<
            Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_,
        >,
    ) -> Self {
        Self {
            t: field!(_a0, t).with_mut(std::mem::take),
        }
    }
}
impl CppMove
    for Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_
{
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default
    for Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_
{
    fn default() -> Self {
        {
            Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_ {
                t: vec![i8::default()],
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(bool, new_1, mut_ptr)]
#[byte_size(8)]
pub struct TransparentComparator {
    #[offset(0)]
    #[byte_size(8)]
    pub transparent_used: Ptr<bool>,
}
impl TransparentComparator {
    pub fn new_1(used: Ptr<bool>) -> Self {
        Self {
            transparent_used: (used).clone(),
        }
    }
}
impl
    Callable2<
        Ptr<Vec<i8>>,
        Ptr<Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_>,
        bool,
    > for TransparentComparator
{
    fn call(
        &self,
        a1: Ptr<Vec<i8>>,
        a2: Ptr<
            Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_,
        >,
    ) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TransparentComparatorImpl::operator_call_3_class_std_basic_string_char__0(__it, a1, a2)
        })
    }
}
impl Callable2<Ptr<i32>, Ptr<Transparent_int__false_>, bool> for TransparentComparator {
    fn call(&self, a1: Ptr<i32>, a2: Ptr<Transparent_int__false_>) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TransparentComparatorImpl::operator_call_3_int_0(__it, a1, a2)
        })
    }
}
impl
    Callable2<
        Ptr<Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_>,
        Ptr<Vec<i8>>,
        bool,
    > for TransparentComparator
{
    fn call(
        &self,
        a1: Ptr<
            Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_,
        >,
        a2: Ptr<Vec<i8>>,
    ) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TransparentComparatorImpl::operator_call_4_class_std_basic_string_char__0(__it, a1, a2)
        })
    }
}
impl Callable2<Ptr<Transparent_int__false_>, Ptr<i32>, bool> for TransparentComparator {
    fn call(&self, a1: Ptr<Transparent_int__false_>, a2: Ptr<i32>) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TransparentComparatorImpl::operator_call_4_int_0(__it, a1, a2)
        })
    }
}
impl Callable2<Ptr<Vec<i8>>, Ptr<Vec<i8>>, bool> for TransparentComparator {
    fn call(&self, a1: Ptr<Vec<i8>>, a2: Ptr<Vec<i8>>) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TransparentComparatorImpl::operator_call_5_class_std_basic_string_char_(__it, a1, a2)
        })
    }
}
impl Callable2<Ptr<i32>, Ptr<i32>, bool> for TransparentComparator {
    fn call(&self, a1: Ptr<i32>, a2: Ptr<i32>) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TransparentComparatorImpl::operator_call_5_int(__it, a1, a2)
        })
    }
}
impl CppMove for TransparentComparator {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for TransparentComparator {
    fn default() -> Self {
        {
            TransparentComparator {
                transparent_used: Ptr::<bool>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NonTransparentComparator {}
impl CppMove for NonTransparentComparator {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoDefaultCtr {}
impl CppMove for NoDefaultCtr {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(16)]
pub struct Moveable {
    #[offset(0)]
    int_: i32,
    #[offset(8)]
    double_: f64,
}
impl Moveable {
    pub fn new_1() -> Self {
        Self {
            int_: 0,
            double_: 0_f64,
        }
    }
    pub fn new_2(mut i: i32, mut d: f64) -> Self {
        Self {
            int_: i,
            double_: d,
        }
    }
    pub fn move_from(x: Ptr<Moveable>) -> Self {
        let __this: Moveable = Self {
            int_: x.with(|__s| __s.int_),
            double_: x.with(|__s| __s.double_),
        };
        field!(x, int_).write(-1_i32);
        field!(x, double_).write((-1_i32 as f64));
        __this
    }
}
impl CppLess for Moveable {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        MoveableImpl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for Moveable {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        MoveableImpl::operator_eq(__lhs, __rhs.clone())
    }
}
impl CppMove for Moveable {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for Moveable {
    fn default() -> Self {
        { Moveable::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(40)]
pub struct test_allocator_statistics {
    #[offset(0)]
    pub time_to_throw: i32,
    #[offset(4)]
    pub throw_after: i32,
    #[offset(8)]
    pub count: i32,
    #[offset(12)]
    pub alloc_count: i32,
    #[offset(16)]
    pub allocated_size: i32,
    #[offset(20)]
    pub construct_count: i32,
    #[offset(24)]
    pub destroy_count: i32,
    #[offset(28)]
    pub copied: i32,
    #[offset(32)]
    pub moved: i32,
    #[offset(36)]
    pub converted: i32,
}
impl CppMove for test_allocator_statistics {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        {
            test_allocator_statistics {
                time_to_throw: 0,
                throw_after: 2147483647,
                count: 0,
                alloc_count: 0,
                allocated_size: 0,
                construct_count: 0,
                destroy_count: 0,
                copied: 0,
                moved: 0,
                converted: 0,
            }
        }
    }
}
thread_local!(
    pub static destructed_value_17: Value<i32> = Rc::new(RefCell::new(-1_i32));
);
thread_local!(
    pub static moved_value_18: Value<i32> = Rc::new(RefCell::new(2147483647));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
impl CppMove for test_alloc_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_19(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_20(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_19(_x, _y)
    });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(16)]
pub struct test_allocator_void_ {
    #[offset(0)]
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_void_ {
    pub fn new_1(mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: 0,
            id_: 0,
            stats_: (stats).clone(),
        }
    }
    pub fn new_2(mut data: i32) -> Self {
        Self {
            data_: data,
            id_: 0,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_3(mut data: i32, mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: data,
            id_: 0,
            stats_: (stats).clone(),
        }
    }
    pub fn new_4(mut data: i32, mut id: i32) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_5(mut data: i32, mut id: i32, mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: (stats).clone(),
        }
    }
    pub fn copy_from(a: Ptr<test_allocator_void_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }
    }
}
impl CppEq for test_allocator_void_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_19(__lhs.clone(), __rhs.clone())
    }
}
impl Clone for test_allocator_void_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_void_> = Rc::new(RefCell::new(test_allocator_void_ {
            data_: self.data_.clone(),
            id_: self.id_.clone(),
            stats_: self.stats_.clone(),
        }));
        test_allocator_void_::copy_from(__src.as_pointer())
    }
}
impl CppMove for test_allocator_void_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for test_allocator_void_ {
    fn default() -> Self {
        {
            test_allocator_void_ {
                data_: 0,
                id_: 0,
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Ctor_Tag {}
impl CppMove for Ctor_Tag {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, FnPtrArg, CppFrom, CopyAssign, MoveAssign, Default)]
#[cpp_from(Ctor_Tag, new_1, value)]
#[byte_size(1)]
pub struct Tag_X {}
impl Tag_X {
    pub fn new_1(mut _a0: Ctor_Tag) -> Self {
        Self {}
    }
    pub fn new_2(mut _a0: Ctor_Tag, _a1: Ptr<Tag_X>) -> Self {
        Self {}
    }
    pub fn new_3(mut _a0: Ctor_Tag, _a1: Ptr<Tag_X>) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!(
            (alloc_count_22.with(|rc| *rc.borrow()) >= dealloc_count_23.with(|rc| *rc.borrow()))
        );
        return ((alloc_count_22.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_23.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_24.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_21.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_22.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_23.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_21: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_22: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_23: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_24: Value<bool> = Rc::new(RefCell::new(false));
);
pub fn operator_sub_25(
    mut x : min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_lt_26(
    mut x : min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } < { (y.ptr_).clone() });
}
pub fn operator_eq_27(
    mut x : min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_28(
    mut x : min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__,
) -> bool {
    return !({
        let _x: min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__   = x   ;
        operator_eq_27(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__
{
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<Vec<i8>>,
}
impl min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { fn new_1 ( mut p : Ptr< Vec<i8  >  >  , ) -> Self { Self { ptr_ : (p ).clone() , }  } }
impl std::cmp::Ord for  min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { fn cmp(&self, other: &Self) -> std::cmp::Ordering {  { libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| if operator_lt_26(__lhs.read(), __rhs.read()) { std::cmp::Ordering::Less } else if operator_lt_26(__rhs.read(), __lhs.read()) { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Equal }) }}}
impl std::cmp::PartialOrd for min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }}
impl std::cmp::PartialEq for min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool {  { libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| operator_eq_27(__lhs.read(), __rhs.read())) }}}
impl std::cmp::Eq for min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ {}
impl CppAdvance for min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { fn cpp_inc(__it: &Ptr<Self>) { __it.operator_inc_8(); } fn cpp_dec(__it: &Ptr<Self>) { __it.operator_dec_10(); } }
impl CppIterator<Vec<i8  >  > for min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { fn iter_deref(&self) -> Ptr<Vec<i8  >  > { libcc2rs::with_record_ptr(&mut self.clone(), |__it| min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__Impl::operator_deref(__it)) } fn iter_inc(&mut self) -> &mut Self { libcc2rs::with_record_ptr(self, |__it| { min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__Impl::operator_inc_8(__it); }); self } }
impl CppInputIterator<Vec<i8  >  > for min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ {}
impl CppBidirectionalIterator<Vec<i8  >  > for min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { fn iter_dec(&mut self) -> &mut Self { libcc2rs::with_record_ptr(self, |__it| { min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__Impl::operator_dec_10(__it); }); self } }
impl CppRandomAccessIterator<Vec<i8  >  > for min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { fn iter_add_assign(&mut self, n: isize) -> Self { libcc2rs::with_record_ptr(self, |__it| { min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__Impl::operator_add_assign(__it, n as _); }); self.clone() } fn iter_distance(&self, from: &Self) -> isize { libcc2rs::with_record_ptrs(self, from, |__lhs, __rhs| (operator_sub_25(__lhs.read(), __rhs.read())) as isize) } }
impl CppMove for min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: AnyPtr,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
impl CppMove for min_pointer_void__std_integral_constant_unsigned_long__0__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_sub_29(
    mut x : min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_eq_30(
    mut x : min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  , new_1, value)]
#[byte_size(8)]
pub struct min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__
{
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<Vec<i8>>,
}
impl min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { pub fn new_1 ( mut p : min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  , ) -> Self { Self { ptr_ : (p . ptr_  ).clone() , }  } }
impl std::cmp::PartialEq for min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool {  { libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| operator_eq_30(__lhs.read(), __rhs.read())) }}}
impl std::cmp::Eq for min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ {}
impl CppAdvance for min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { fn cpp_inc(__it: &Ptr<Self>) { __it.operator_inc_9(); } fn cpp_dec(__it: &Ptr<Self>) { __it.operator_dec_11(); } }
impl CppIterator<Vec<i8  >  > for min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { fn iter_deref(&self) -> Ptr<Vec<i8  >  > { libcc2rs::with_record_ptr(&mut self.clone(), |__it| min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__Impl::operator_deref(__it)) } fn iter_inc(&mut self) -> &mut Self { libcc2rs::with_record_ptr(self, |__it| { min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__Impl::operator_inc_9(__it); }); self } }
impl CppInputIterator<Vec<i8  >  > for min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ {}
impl CppBidirectionalIterator<Vec<i8  >  > for min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { fn iter_dec(&mut self) -> &mut Self { libcc2rs::with_record_ptr(self, |__it| { min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__Impl::operator_dec_11(__it); }); self } }
impl CppMove for min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: AnyPtr,
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
impl CppMove for min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_basic_string_char__std_char_traits_char___std_allocator_char___ {}
impl CppMove for min_allocator_std_basic_string_char__std_char_traits_char___std_allocator_char___ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
const _: () = assert!(true, "CanContains<TransparentSet>");
const _: () = assert!(true, "CanContains<const TransparentSet>");
const _: () = assert!(!(false), "!CanContains<NonTransparentSet>");
const _: () = assert!(!(false), "!CanContains<const NonTransparentSet>");
pub fn test_one_31() {
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
    }
}
pub fn test_one_32() {
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
    }
}
pub fn test_one_33() {
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
    }
}
pub fn test_one_34() {
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
    }
}
pub fn test_35() -> bool {
    ({ test_one_31() });
    {
        ({ test_one_32() });
    }
    ({ test_one_33() });
    ({ test_one_34() });
    {
        let transparent_used: Value<bool> = Rc::new(RefCell::new(false));
        let mut c: TransparentComparator =
            TransparentComparator::new_1({ transparent_used.as_pointer() });
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ !(*transparent_used.borrow()) }));
        let mut b: bool = unimplemented!();
        assert!((b as bool));
        assert!(({ (*transparent_used.borrow()) }));
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
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
    ({ test_35() });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct three_way_random_access_iterator_int_ptr_;
pub trait MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__Impl
{
    fn operator_assign_6 ( &self , il : Vec<i32  >  , ) -> Ptr< MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__  >{
        unimplemented!()
    }
    fn assign_8(&self, il: Vec<i32>) {
        unimplemented!()
    }
    fn assign_9(&self, mut n: u32, mut t: i32) {
        unimplemented!()
    }
    fn begin_11(&self) -> three_way_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn begin_12(&self) -> three_way_random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn cbegin(&self) -> three_way_random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn end_14(&self) -> three_way_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn end_15(&self) -> three_way_random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn cend(&self) -> three_way_random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn size(&self) -> u32 {
        unimplemented!()
    }
    fn empty(&self) -> bool {
        unimplemented!()
    }
    fn clear(&self) {
        unimplemented!()
    }
    fn insert_21(
        &self,
        mut p: three_way_random_access_iterator_const_int_ptr_,
        mut value: i32,
    ) -> three_way_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn insert_22(
        &self,
        mut p: three_way_random_access_iterator_const_int_ptr_,
        mut n: u32,
        mut value: i32,
    ) -> three_way_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn insert_23(
        &self,
        mut p: three_way_random_access_iterator_const_int_ptr_,
        il: Vec<i32>,
    ) -> three_way_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn erase_25(
        &self,
        mut first: three_way_random_access_iterator_const_int_ptr_,
        mut last: three_way_random_access_iterator_const_int_ptr_,
    ) -> three_way_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn erase_26(
        &self,
        mut iter: three_way_random_access_iterator_const_int_ptr_,
    ) -> three_way_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn to_vector_iterator(
        &self,
        mut cit: three_way_random_access_iterator_const_int_ptr_,
    ) -> RandomAccessIterator<i32> {
        unimplemented!()
    }
    fn from_vector_iterator(
        &self,
        it: RandomAccessIterator<i32>,
    ) -> three_way_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn move_assign ( &self , _a0 : Ptr< MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__  > , ) -> Ptr< MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__  > ;
}
impl MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__Impl for Ptr<MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__> {
fn move_assign ( &self , _a0 : Ptr< MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__  > , ) -> Ptr< MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__  > { field_ref!((* self )  , data_).as_pointer() .write(field!(_a0, data_).with_mut(std::mem::take))  ;
 return ((* self )  ).clone()  ;
 } }
pub trait MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__Impl
{
    fn operator_assign_6 ( &self , il : Vec<Vec<i8  >  >  , ) -> Ptr< MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__  >{
        unimplemented!()
    }
    fn assign_8(&self, il: Vec<Vec<i8>>) {
        unimplemented!()
    }
    fn assign_9(&self, mut n: u32, mut t: Vec<i8>) {
        unimplemented!()
    }
    fn begin_11 ( &self , ) -> three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  ;
    fn begin_12 ( &self , ) -> three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  ;
    fn cbegin ( &self , ) -> three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  ;
    fn end_14 ( &self , ) -> three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  ;
    fn end_15 ( &self , ) -> three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  ;
    fn cend ( &self , ) -> three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_{
        unimplemented!()
    }
    fn size(&self) -> u32;
    fn empty(&self) -> bool {
        unimplemented!()
    }
    fn clear(&self);
    fn insert_21 ( &self , mut p : three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  , mut value : Vec<i8  >  , ) -> three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_{
        unimplemented!()
    }    fn insert_22 ( &self , mut p : three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  , mut n : u32  , mut value : Vec<i8  >  , ) -> three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_{
        unimplemented!()
    }    fn insert_23 ( &self , mut p : three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  , il : Vec<Vec<i8  >  >  , ) -> three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_{
        unimplemented!()
    }    fn erase_25 ( &self , mut first : three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  , mut last : three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  , ) -> three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_{
        unimplemented!()
    }    fn erase_26 ( &self , mut iter : three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  , ) -> three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_{
        unimplemented!()
    }
    fn to_vector_iterator(
        &self,
        cit : three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_,
    ) -> RandomAccessIterator<Vec<i8>>;
    fn from_vector_iterator ( &self , it : RandomAccessIterator<Vec<i8  >  >  , ) -> three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  ;
    fn move_assign ( &self , _a0 : Ptr< MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__  > , ) -> Ptr< MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__  > ;
    fn insert_20_const_class_std_basic_string_char__ptr ( &self , p : three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  , first : Ptr< Vec<i8  >  >  , last : Ptr< Vec<i8  >  >  , ) -> three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  ;
}
impl MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__Impl for Ptr<MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__> {
fn begin_11 ( &self , ) -> three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  { return three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ :: new_1 ( { field_ref!((* self )  , data_).decay()    } , )    ;
 } fn begin_12 ( &self , ) -> three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  { return three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ :: new_2 ( { field_ref!((* self )  , data_).decay()    } , )    ;
 } fn cbegin ( &self , ) -> three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  { return three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ :: new_2 ( { field_ref!((* self )  , data_).decay()    } , )    ;
 } fn end_14 ( &self , ) -> three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  { return ( { let _x: three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_   = ( { MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__Impl :: begin_11 ( self , ) } )   ; let _n: isize   = ( ( { MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__Impl :: size ( self , ) } )  as isize )  ; operator_add_1 ( _x , _n , ) } )    ;
 } fn end_15 ( &self , ) -> three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  { return ( { let _x: three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_   = ( { MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__Impl :: begin_12 ( self , ) } )   ; let _n: isize   = ( ( { MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__Impl :: size ( self , ) } )  as isize )  ; operator_add_7 ( _x , _n , ) } )    ;
 } fn size ( &self , ) -> u32  { return ( (*(* self ) .upgrade().deref()) . data_  .len()  as u32 )   ;
 } fn clear ( &self , ) { field!((* self ) , data_).with_mut(|__v: &mut Vec<Vec<i8  >  >| __v.clear())  ;
 } fn to_vector_iterator ( &self , mut cit : three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  , ) -> RandomAccessIterator<Vec<i8  >  >  { return match ({ ( { let _x: three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_   = (cit ).clone()  ; operator_sub_8 ( _x , ( { MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__Impl :: cbegin ( self , ) } )   , ) } )   },) { (mut __tmp_0,) => { field_ref!((* self )  , data_).decay() .begin()  .iter_add(__tmp_0 ) } }    ;
 } fn from_vector_iterator ( &self , it : RandomAccessIterator<Vec<i8  >  >  , ) -> three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  { let it : Value<RandomAccessIterator<Vec<i8  >  > >  = Rc::new(RefCell::new(it)) ;
 return ( { let _n: isize   = ((*it.borrow()) .iter_distance(&field_ref!((* self )  , data_).decay() .begin()  .into()) as i64 as isize)   ; let _x: three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_   = ( { MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__Impl :: begin_11 ( self , ) } )   ; operator_add_2 ( _n , _x , ) } )    ;
 } fn move_assign ( &self , _a0 : Ptr< MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__  > , ) -> Ptr< MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__  > { field_ref!((* self )  , data_).as_pointer() .write(field!(_a0, data_).with_mut(std::mem::take))  ;
 return ((* self )  ).clone()  ;
 } fn insert_20_const_class_std_basic_string_char__ptr ( &self , mut p : three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  , mut first : Ptr< Vec<i8  >  >  , mut last : Ptr< Vec<i8  >  >  , ) -> three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  { return ( { let _it: RandomAccessIterator<Vec<i8  >  >   = match ({ ( { MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__Impl :: to_vector_iterator ( self , (p ).clone()  , ) } )   },) { (mut __tmp_1,) => { {let __start = __tmp_1 .index();
    let mut __tmp: Vec<Vec<i8  >  > = Vec::new();
    let mut __it = (first ).clone();
    while __it != last  {
        __tmp.push(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from(__it.iter_deref()));
        __it.iter_inc();
    }
    field_ref!((* self )  , data_).as_pointer() .with_mut(|__v: &mut Vec<Vec<i8  >  >| {
        __v.splice(__start..__start, __tmp);
    });
    (__tmp_1 ).clone()} } }   ; MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__Impl :: from_vector_iterator ( self , _it , ) } )    ;
 } }
pub trait MoveableImpl {
    fn move_assign(&self, x: Ptr<Moveable>) -> Ptr<Moveable>;
    fn operator_eq(&self, x: Ptr<Moveable>) -> bool;
    fn operator_lt(&self, x: Ptr<Moveable>) -> bool;
    fn get(&self) -> i32;
    fn moved(&self) -> bool;
}
impl MoveableImpl for Ptr<Moveable> {
    fn move_assign(&self, x: Ptr<Moveable>) -> Ptr<Moveable> {
        field!((*self), int_).write({ x.with(|__s| __s.int_) });
        field!(x, int_).write(-1_i32);
        field!((*self), double_).write({ x.with(|__s| __s.double_) });
        field!(x, double_).write((-1_i32 as f64));
        return (*self).clone();
    }
    fn operator_eq(&self, x: Ptr<Moveable>) -> bool {
        return ({ (*self).with(|__s| __s.int_) } == { x.with(|__s| __s.int_) })
            && ({ (*self).with(|__s| __s.double_) } == { x.with(|__s| __s.double_) });
    }
    fn operator_lt(&self, x: Ptr<Moveable>) -> bool {
        return ({ (*self).with(|__s| __s.int_) } < { x.with(|__s| __s.int_) })
            || (({ (*self).with(|__s| __s.int_) } == { x.with(|__s| __s.int_) })
                && ({ (*self).with(|__s| __s.double_) } < { x.with(|__s| __s.double_) }));
    }
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.int_);
    }
    fn moved(&self) -> bool {
        return ((*self).with(|__s| __s.int_) == -1_i32);
    }
}
pub trait Tag_XImpl {
    fn copy_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X>;
    fn move_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X>;
}
impl Tag_XImpl for Ptr<Tag_X> {
    fn copy_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X> {
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X> {
        return (*self).clone();
    }
}
pub trait TransparentComparatorImpl {
    fn operator_call_3_class_std_basic_string_char__0(
        &self,
        t: Ptr<Vec<i8>>,
        transparent: Ptr<
            Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_,
        >,
    ) -> bool;
    fn operator_call_3_int_0(&self, t: Ptr<i32>, transparent: Ptr<Transparent_int__false_>)
    -> bool;
    fn operator_call_4_class_std_basic_string_char__0(
        &self,
        transparent: Ptr<
            Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_,
        >,
        t: Ptr<Vec<i8>>,
    ) -> bool;
    fn operator_call_4_int_0(&self, transparent: Ptr<Transparent_int__false_>, t: Ptr<i32>)
    -> bool;
    fn operator_call_5_class_std_basic_string_char_(
        &self,
        t1: Ptr<Vec<i8>>,
        t2: Ptr<Vec<i8>>,
    ) -> bool;
    fn operator_call_5_int(&self, t1: Ptr<i32>, t2: Ptr<i32>) -> bool;
}
impl TransparentComparatorImpl for Ptr<TransparentComparator> {
    fn operator_call_3_class_std_basic_string_char__0(
        &self,
        t: Ptr<Vec<i8>>,
        transparent: Ptr<
            Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_,
        >,
    ) -> bool {
        if !(((*self).with(|__s| __s.transparent_used.clone())).is_null()) {
            (*self).with(|__s| __s.transparent_used.clone()).write(true);
        }
        return unimplemented!() == std::cmp::Ordering::Less;
    }
    fn operator_call_3_int_0(
        &self,
        t: Ptr<i32>,
        transparent: Ptr<Transparent_int__false_>,
    ) -> bool {
        if !(((*self).with(|__s| __s.transparent_used.clone())).is_null()) {
            (*self).with(|__s| __s.transparent_used.clone()).write(true);
        }
        return ({ (t.read()) } < { transparent.with(|__s| __s.t) });
    }
    fn operator_call_4_class_std_basic_string_char__0(
        &self,
        transparent: Ptr<
            Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_,
        >,
        t: Ptr<Vec<i8>>,
    ) -> bool {
        if !(((*self).with(|__s| __s.transparent_used.clone())).is_null()) {
            (*self).with(|__s| __s.transparent_used.clone()).write(true);
        }
        return unimplemented!() == std::cmp::Ordering::Less;
    }
    fn operator_call_4_int_0(
        &self,
        transparent: Ptr<Transparent_int__false_>,
        t: Ptr<i32>,
    ) -> bool {
        if !(((*self).with(|__s| __s.transparent_used.clone())).is_null()) {
            (*self).with(|__s| __s.transparent_used.clone()).write(true);
        }
        return ({ transparent.with(|__s| __s.t) } < { (t.read()) });
    }
    fn operator_call_5_class_std_basic_string_char_(
        &self,
        t1: Ptr<Vec<i8>>,
        t2: Ptr<Vec<i8>>,
    ) -> bool {
        return unimplemented!() == std::cmp::Ordering::Less;
    }
    fn operator_call_5_int(&self, t1: Ptr<i32>, t2: Ptr<i32>) -> bool {
        return ({ (t1.read()) } < { (t2.read()) });
    }
}
pub trait Transparent_int__false_Impl {
    fn to_i32(&self) -> i32 {
        unimplemented!()
    }
}
impl Transparent_int__false_Impl for Ptr<Transparent_int__false_> {}
pub trait Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_Impl
{
    fn to_Veci8(&self) -> Vec<i8> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<
            Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_,
        >,
    ) -> Ptr<Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_>;
}
impl Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_Impl
    for Ptr<Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_>
{
    fn move_assign(
        &self,
        _a0: Ptr<
            Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_,
        >,
    ) -> Ptr<Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_>
    {
        field_ref!((*self), t)
            .as_pointer()
            .write(field!(_a0, t).with_mut(std::mem::take));
        return (*self).clone();
    }
}
pub trait adl_IteratorImpl {
    fn iter_moves(&self) -> i32;
    fn iter_swaps(&self) -> i32;
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_index(&self, n: isize) -> Ptr<i32>;
    fn operator_sub_10(&self, n: isize) -> adl_Iterator;
    fn operator_sub_11(&self, rhs: adl_Iterator) -> isize;
    fn operator_add_assign(&self, n: isize) -> Ptr<adl_Iterator>;
    fn operator_sub_assign(&self, n: isize) -> Ptr<adl_Iterator>;
    fn operator_inc_14(&self) -> Ptr<adl_Iterator>;
    fn operator_post_inc_15(&self, _a0: i32) -> adl_Iterator;
    fn operator_dec_16(&self) -> Ptr<adl_Iterator>;
    fn operator_post_dec_17(&self, _a0: i32) -> adl_Iterator;
}
impl adl_IteratorImpl for Ptr<adl_Iterator> {
    fn iter_moves(&self) -> i32 {
        assert!(!((*self).with(|__s| __s.iter_moves_.clone())).is_null());
        return ((*self).with(|__s| __s.iter_moves_.clone()).read());
    }
    fn iter_swaps(&self) -> i32 {
        assert!(!((*self).with(|__s| __s.iter_swaps_.clone())).is_null());
        return ((*self).with(|__s| __s.iter_swaps_.clone()).read());
    }
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        return (*self).with(|__s| __s.ptr_.clone()).offset((n) as isize);
    }
    fn operator_sub_10(&self, mut n: isize) -> adl_Iterator {
        return adl_Iterator::new_1(
            { (*self).with(|__s| __s.ptr_.clone()).offset(-((n) as isize)) },
            { (*self).with(|__s| __s.iter_moves_.clone()) },
            { (*self).with(|__s| __s.iter_swaps_.clone()) },
        );
    }
    fn operator_sub_11(&self, mut rhs: adl_Iterator) -> isize {
        return (((*self).with(|__s| __s.ptr_.clone()) - (rhs.ptr_).clone()) as i64 as isize);
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<adl_Iterator> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<adl_Iterator> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn operator_inc_14(&self) -> Ptr<adl_Iterator> {
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn operator_post_inc_15(&self, mut _a0: i32) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(*self).upgrade().deref()).clone();
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return prev;
    }
    fn operator_dec_16(&self) -> Ptr<adl_Iterator> {
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_dec();
        });
        return (*self).clone();
    }
    fn operator_post_dec_17(&self, mut _a0: i32) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(*self).upgrade().deref()).clone();
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_dec();
        });
        return prev;
    }
}
pub trait cpp17_input_iterator_int_ptr__int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: i32) -> cpp17_input_iterator_int_ptr__int_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>,
    ) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_>;
}
impl cpp17_input_iterator_int_ptr__int_ptr_Impl for Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {
    fn move_assign(
        &self,
        _a0: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>,
    ) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait cpp17_output_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_output_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: i32) -> cpp17_output_iterator_int_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<cpp17_output_iterator_int_ptr_>,
    ) -> Ptr<cpp17_output_iterator_int_ptr_>;
}
impl cpp17_output_iterator_int_ptr_Impl for Ptr<cpp17_output_iterator_int_ptr_> {
    fn move_assign(
        &self,
        _a0: Ptr<cpp17_output_iterator_int_ptr_>,
    ) -> Ptr<cpp17_output_iterator_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait cpp20_input_iterator_int_ptr_Impl {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_input_iterator_int_ptr_>,
    ) -> Ptr<cpp20_input_iterator_int_ptr_>;
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_5(&self) -> Ptr<cpp20_input_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: i32) {
        unimplemented!()
    }
}
impl cpp20_input_iterator_int_ptr_Impl for Ptr<cpp20_input_iterator_int_ptr_> {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_input_iterator_int_ptr_>,
    ) -> Ptr<cpp20_input_iterator_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
}
pub trait cpp20_output_iterator_int_ptr_Impl {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_output_iterator_int_ptr_>,
    ) -> Ptr<cpp20_output_iterator_int_ptr_>;
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_5(&self) -> Ptr<cpp20_output_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: i32) -> cpp20_output_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl cpp20_output_iterator_int_ptr_Impl for Ptr<cpp20_output_iterator_int_ptr_> {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_output_iterator_int_ptr_>,
    ) -> Ptr<cpp20_output_iterator_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
}
pub trait cpp20_random_access_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_index(&self, n: isize) -> Ptr<i32>;
    fn operator_inc_7(&self) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_dec_8(&self) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: i32) -> cpp20_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: i32) -> cpp20_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_random_access_iterator_int_ptr_>,
    ) -> Ptr<cpp20_random_access_iterator_int_ptr_>;
}
impl cpp20_random_access_iterator_int_ptr_Impl for Ptr<cpp20_random_access_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone()).offset((n) as isize);
    }
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_random_access_iterator_int_ptr_>,
    ) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait forward_sized_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_4(&self) -> Ptr<forward_sized_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_5(&self, mut _a0: i32) -> forward_sized_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl forward_sized_iterator_int_ptr_Impl for Ptr<forward_sized_iterator_int_ptr_> {}
pub trait min_allocator_std_basic_string_char__std_char_traits_char___std_allocator_char___Impl {
    fn allocate ( &self , n : usize  , ) -> min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  ;
    fn deallocate(
        &self,
        p : min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__,
        n: usize,
    );
}
impl min_allocator_std_basic_string_char__std_char_traits_char___std_allocator_char___Impl
    for Ptr<min_allocator_std_basic_string_char__std_char_traits_char___std_allocator_char___>
{
    fn allocate ( &self , mut n : usize  , ) -> min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__{
        return min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()   } , )    ;
    }
    fn deallocate(
        &self,
        mut p : min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
pub trait min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__Impl
{
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<Vec<i8>>;
    fn operator_arrow(&self) -> Ptr<Vec<i8>> {
        unimplemented!()
    }
    fn operator_inc_9 ( &self , ) -> Ptr< min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  > ;
    fn operator_post_inc_10 ( &self , mut _a0 : i32  , ) -> min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_dec_11 ( &self , ) -> Ptr< min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  > ;
    fn operator_post_dec_12 ( &self , mut _a0 : i32  , ) -> min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_add_assign ( &self , n : isize  , ) -> Ptr< min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  > ;
    fn operator_sub_assign ( &self , mut n : isize  , ) -> Ptr< min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_add ( &self , mut n : isize  , ) -> min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_sub ( &self , mut n : isize  , ) -> min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<Vec<i8>> {
        unimplemented!()
    }
}
impl min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__> {
fn operator_deref ( &self , ) -> Ptr< Vec<i8  >  > { return (* self ) .with(|__s| __s . ptr_ .clone())    ;
 } fn operator_inc_9 ( &self , ) -> Ptr< min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  > { field!((* self ) , ptr_) .with_mut(|__v| { __v. prefix_inc (); }) ;
 return ((* self )  ).clone()  ;
 } fn operator_dec_11 ( &self , ) -> Ptr< min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  > { field!((* self ) , ptr_) .with_mut(|__v| { __v. prefix_dec (); }) ;
 return ((* self )  ).clone()  ;
 } fn operator_add_assign ( &self , mut n : isize  , ) -> Ptr< min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  > { { let _ptr = field!((* self ) , ptr_); _ptr.write(_ptr.read() + n ) } ;
 return ((* self )  ).clone()  ;
 } }
pub trait min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__Impl
{
    fn to_bool(&self) -> bool;
    fn operator_deref(&self) -> Ptr<Vec<i8>>;
    fn operator_arrow(&self) -> Ptr<Vec<i8>> {
        unimplemented!()
    }
    fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  > ;
    fn operator_post_inc_9 ( &self , mut _a0 : i32  , ) -> min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  > ;
    fn operator_post_dec_11 ( &self , mut _a0 : i32  , ) -> min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_add_assign ( &self , n : isize  , ) -> Ptr< min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  > ;
    fn operator_sub_assign ( &self , n : isize  , ) -> Ptr< min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  > ;
    fn operator_add ( &self , n : isize  , ) -> min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  ;
    fn operator_sub ( &self , n : isize  , ) -> min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  ;
    fn operator_index(&self, mut n: isize) -> Ptr<Vec<i8>> {
        unimplemented!()
    }
}
impl min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__> {
fn to_bool ( &self , ) -> bool  { return ! ( ( (* self ) .with(|__s| __s . ptr_ .clone()) ).is_null() )   ;
 } fn operator_deref ( &self , ) -> Ptr< Vec<i8  >  > { return (* self ) .with(|__s| __s . ptr_ .clone())    ;
 } fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  > { field!((* self ) , ptr_) .with_mut(|__v| { __v. prefix_inc (); }) ;
 return ((* self )  ).clone()  ;
 } fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  > { field!((* self ) , ptr_) .with_mut(|__v| { __v. prefix_dec (); }) ;
 return ((* self )  ).clone()  ;
 } fn operator_add_assign ( &self , mut n : isize  , ) -> Ptr< min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  > { { let _ptr = field!((* self ) , ptr_); _ptr.write(_ptr.read() + n ) } ;
 return ((* self )  ).clone()  ;
 } fn operator_sub_assign ( &self , mut n : isize  , ) -> Ptr< min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  > { { let _ptr = field!((* self ) , ptr_); _ptr.write(_ptr.read() - n ) } ;
 return ((* self )  ).clone()  ;
 } fn operator_add ( &self , mut n : isize  , ) -> min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  { let tmp : Value<min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ > = Rc::new(RefCell::new((( * (* self ) .upgrade().deref() )  ).clone() )) ;
  ;
 ;
 ( { min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__Impl :: operator_add_assign ( &tmp .as_pointer()  , n  , ) } )  ;
 return ((*tmp.borrow()) ).clone()   ;
 } fn operator_sub ( &self , mut n : isize  , ) -> min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  { let tmp : Value<min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ > = Rc::new(RefCell::new((( * (* self ) .upgrade().deref() )  ).clone() )) ;
  ;
 ;
 ( { min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__Impl :: operator_sub_assign ( &tmp .as_pointer()  , n  , ) } )  ;
 return ((*tmp.borrow()) ).clone()   ;
 } }
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait rvalue_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn operator_inc_4(&self) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_5(&self, mut _a0: i32) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_dec_6(&self) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_dec_7(&self, mut _a0: i32) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add(&self, mut n: isize) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_sub_9(&self, mut n: isize) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_sub_10(&self, other: Ptr<rvalue_iterator_int_ptr_>) -> isize {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn operator_cmp(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> std::cmp::Ordering;
    fn operator_eq(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> bool;
}
impl rvalue_iterator_int_ptr_Impl for Ptr<rvalue_iterator_int_ptr_> {
    fn operator_cmp(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> std::cmp::Ordering {
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.it_.clone())),
                &(_a0.with(|__s| __s.it_.clone())),
            )));
            if !(({ (*cmp.borrow()) }) == std::cmp::Ordering::Equal) {
                return ({ (*cmp.borrow()) });
            }
        }
        return ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) });
    }
    fn operator_eq(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> bool {
        return ({ (*self).with(|__s| __s.it_.clone()) } == { _a0.with(|__s| __s.it_.clone()) });
    }
}
pub trait support_double_move_trackerImpl {
    fn move_assign(
        &self,
        other: Ptr<support_double_move_tracker>,
    ) -> Ptr<support_double_move_tracker>;
}
impl support_double_move_trackerImpl for Ptr<support_double_move_tracker> {
    fn move_assign(
        &self,
        other: Ptr<support_double_move_tracker>,
    ) -> Ptr<support_double_move_tracker> {
        assert!(!(other.with(|__s| __s.moved_from_)));
        field!(other, moved_from_).write(true);
        field!((*self), moved_from_).write(false);
        return (*self).clone();
    }
}
pub trait test_allocator_statisticsImpl {
    fn clear(&self);
    fn clear_ctor_counters(&self);
}
impl test_allocator_statisticsImpl for Ptr<test_allocator_statistics> {
    fn clear(&self) {
        assert!(
            ((*self).with(|__s| __s.count) == 0)
                && (!(Ptr::<i8>::from_string_literal(b"clearing leaking allocator data?"))
                    .is_null())
        );
        field!((*self), count).write(0);
        field!((*self), time_to_throw).write(0);
        field!((*self), alloc_count).write(0);
        field!((*self), allocated_size).write(0);
        field!((*self), construct_count).write(0);
        field!((*self), destroy_count).write(0);
        field!((*self), throw_after).write(2147483647);
        ({ test_allocator_statisticsImpl::clear_ctor_counters(self) });
    }
    fn clear_ctor_counters(&self) {
        field!((*self), copied).write(0);
        field!((*self), moved).write(0);
        field!((*self), converted).write(0);
    }
}
pub trait test_allocator_void_Impl {
    fn destructor(&self);
    fn get_id(&self) -> i32;
    fn get_data(&self) -> i32;
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_>;
}
impl test_allocator_void_Impl for Ptr<test_allocator_void_> {
    fn destructor(&self) {
        field!((*self), data_).write(destructed_value_17.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_17.with(|rc| *rc.borrow()));
    }
    fn get_id(&self) -> i32 {
        return (*self).with(|__s| __s.id_);
    }
    fn get_data(&self) -> i32 {
        return (*self).with(|__s| __s.data_);
    }
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_> {
        field!((*self), data_).write({ _a0.with(|__s| __s.data_) });
        field!((*self), id_).write({ _a0.with(|__s| __s.id_) });
        field!((*self), stats_).write({ _a0.with(|__s| __s.stats_.clone()) });
        return (*self).clone();
    }
}
pub trait three_way_random_access_iterator_const_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<three_way_random_access_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_dec_8(&self) -> Ptr<three_way_random_access_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: i32) -> three_way_random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(
        &self,
        mut _a0: i32,
    ) -> three_way_random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<three_way_random_access_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<three_way_random_access_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<three_way_random_access_iterator_const_int_ptr_>,
    ) -> Ptr<three_way_random_access_iterator_const_int_ptr_>;
}
impl three_way_random_access_iterator_const_int_ptr_Impl
    for Ptr<three_way_random_access_iterator_const_int_ptr_>
{
    fn move_assign(
        &self,
        _a0: Ptr<three_way_random_access_iterator_const_int_ptr_>,
    ) -> Ptr<three_way_random_access_iterator_const_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_Impl
{
    fn operator_deref(&self) -> Ptr<Vec<i8>>;
    fn operator_index(&self, mut n: isize) -> Ptr<Vec<i8>> {
        unimplemented!()
    }
    fn operator_inc_7 ( &self , ) -> Ptr< three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  > ;
    fn operator_dec_8 ( &self , ) -> Ptr< three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  > ;
    fn operator_post_inc_9 ( &self , mut _a0 : i32  , ) -> three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_{
        unimplemented!()
    }    fn operator_post_dec_10 ( &self , mut _a0 : i32  , ) -> three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_{
        unimplemented!()
    }
    fn operator_add_assign ( &self , n : isize  , ) -> Ptr< three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  > ;
    fn operator_sub_assign ( &self , mut n : isize  , ) -> Ptr< three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  >{
        unimplemented!()
    }
    fn move_assign ( &self , _a0 : Ptr< three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  > , ) -> Ptr< three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  > ;
}
impl three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_Impl for Ptr<three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_> {
fn operator_deref ( &self , ) -> Ptr<Vec<i8  >  > { return (* self ) .with(|__s| __s . it_ .clone())    ;
 } fn operator_inc_7 ( &self , ) -> Ptr< three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  > { field!((* self ) , it_) .with_mut(|__v| { __v. prefix_inc (); }) ;
 return ((* self )  ).clone()  ;
 } fn operator_dec_8 ( &self , ) -> Ptr< three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  > { field!((* self ) , it_) .with_mut(|__v| { __v. prefix_dec (); }) ;
 return ((* self )  ).clone()  ;
 } fn operator_add_assign ( &self , mut n : isize  , ) -> Ptr< three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  > { { let _ptr = field!((* self ) , it_); _ptr.write(_ptr.read() + n ) } ;
 return ((* self )  ).clone()  ;
 } fn move_assign ( &self , _a0 : Ptr< three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  > , ) -> Ptr< three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  > { field!((* self ) , it_) .write( { { (*_a0.upgrade().deref()) . it_ .clone() }  } ) ;
 ( { let _other: Ptr< support_double_move_tracker  >  = field_ptr!(_a0 , tracker_)  ; support_double_move_trackerImpl :: move_assign ( &field_ptr!((* self )  , tracker_)  , _other , ) } )  ;
 return ((* self )  ).clone()  ;
 } }
pub trait three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_Impl
{
    fn operator_deref(&self) -> Ptr<Vec<i8>>;
    fn operator_index(&self, mut n: isize) -> Ptr<Vec<i8>> {
        unimplemented!()
    }
    fn operator_inc_7 ( &self , ) -> Ptr< three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  > ;
    fn operator_dec_8 ( &self , ) -> Ptr< three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  > ;
    fn operator_post_inc_9 ( &self , mut _a0 : i32  , ) -> three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_{
        unimplemented!()
    }    fn operator_post_dec_10 ( &self , mut _a0 : i32  , ) -> three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_{
        unimplemented!()
    }
    fn operator_add_assign ( &self , n : isize  , ) -> Ptr< three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  > ;
    fn operator_sub_assign ( &self , n : isize  , ) -> Ptr< three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  > ;
    fn move_assign ( &self , _a0 : Ptr< three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  > , ) -> Ptr< three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  > ;
}
impl three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_Impl for Ptr<three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_> {
fn operator_deref ( &self , ) -> Ptr<Vec<i8  >  > { return (* self ) .with(|__s| __s . it_ .clone())    ;
 } fn operator_inc_7 ( &self , ) -> Ptr< three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  > { field!((* self ) , it_) .with_mut(|__v| { __v. prefix_inc (); }) ;
 return ((* self )  ).clone()  ;
 } fn operator_dec_8 ( &self , ) -> Ptr< three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  > { field!((* self ) , it_) .with_mut(|__v| { __v. prefix_dec (); }) ;
 return ((* self )  ).clone()  ;
 } fn operator_add_assign ( &self , mut n : isize  , ) -> Ptr< three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  > { { let _ptr = field!((* self ) , it_); _ptr.write(_ptr.read() + n ) } ;
 return ((* self )  ).clone()  ;
 } fn operator_sub_assign ( &self , mut n : isize  , ) -> Ptr< three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  > { { let _ptr = field!((* self ) , it_); _ptr.write(_ptr.read() - n ) } ;
 return ((* self )  ).clone()  ;
 } fn move_assign ( &self , _a0 : Ptr< three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  > , ) -> Ptr< three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  > { field!((* self ) , it_) .write( { { (*_a0.upgrade().deref()) . it_ .clone() }  } ) ;
 ( { let _other: Ptr< support_double_move_tracker  >  = field_ptr!(_a0 , tracker_)  ; support_double_move_trackerImpl :: move_assign ( &field_ptr!((* self )  , tracker_)  , _other , ) } )  ;
 return ((* self )  ).clone()  ;
 } }
pub fn __cpp2rust_init_globals() {
    let _ = destructed_value_17.with(|_| ());
    let _ = moved_value_18.with(|_| ());
    let _ = outstanding_bytes_21.with(|_| ());
    let _ = alloc_count_22.with(|_| ());
    let _ = dealloc_count_23.with(|_| ());
    let _ = disable_default_constructor_24.with(|_| ());
}
