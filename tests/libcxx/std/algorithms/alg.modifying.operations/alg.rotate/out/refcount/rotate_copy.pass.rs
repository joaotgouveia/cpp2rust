extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
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
pub struct types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp17_output_iterator_int_ptr__
{}
impl CppMove for types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp17_output_iterator_int_ptr__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr__
{}
impl CppMove for types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr____types_type_list_bidirectional_iterator_int_ptr___
{}
impl CppMove for types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr____types_type_list_bidirectional_iterator_int_ptr___ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr____types_type_list_forward_iterator_int_ptr___
{}
impl CppMove for types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr____types_type_list_forward_iterator_int_ptr___ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr____types_type_list_cpp17_output_iterator_int_ptr___
{}
impl CppMove for types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr____types_type_list_cpp17_output_iterator_int_ptr___ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr____types_type_list_bidirectional_iterator_const_int_ptr___
{}
impl CppMove for types_concatenate_types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr____types_type_list_bidirectional_iterator_const_int_ptr___ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
pub fn for_each_0(
    mut _a0 : types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr__,
    f: TestIter,
) {
    let f: Value<TestIter> = Rc::new(RefCell::new(f));
    ({
        let _arg0: i32 = {
            ({ TestIterImpl::operator_call_1_const_int_ptr(&f.as_pointer()) });
            0
        };
        let _arg1: i32 = {
            ({
                TestIterImpl::operator_call_1_class_contiguous_iterator_const_int_ptr_(
                    &f.as_pointer(),
                )
            });
            0
        };
        let _arg2: i32 = {
            ({
                TestIterImpl::operator_call_1_class_random_access_iterator_const_int_ptr_(
                    &f.as_pointer(),
                )
            });
            0
        };
        let _arg3: i32 = {
            ({
                TestIterImpl::operator_call_1_class_bidirectional_iterator_const_int_ptr_(
                    &f.as_pointer(),
                )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3)
    });
}
pub fn for_each_2(
    mut _a0 : types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp17_output_iterator_int_ptr__,
    f: TestIter_TestImpl_const_int_ptr_,
) {
    let f: Value<TestIter_TestImpl_const_int_ptr_> = Rc::new(RefCell::new(f));
    ({
        let _arg0: i32 = {
            ({ TestIter_TestImpl_const_int_ptr_Impl::operator_call_1_int_ptr(&f.as_pointer()) });
            0
        };
        let _arg1: i32 = {
            ({
                TestIter_TestImpl_const_int_ptr_Impl :: operator_call_1_class_contiguous_iterator_int_ptr_ ( &f .as_pointer()  , )
            });
            0
        };
        let _arg2: i32 = {
            ({
                TestIter_TestImpl_const_int_ptr_Impl :: operator_call_1_class_random_access_iterator_int_ptr_ ( &f .as_pointer()  , )
            });
            0
        };
        let _arg3: i32 = {
            ({
                TestIter_TestImpl_const_int_ptr_Impl :: operator_call_1_class_bidirectional_iterator_int_ptr_ ( &f .as_pointer()  , )
            });
            0
        };
        let _arg4: i32 = {
            ({
                TestIter_TestImpl_const_int_ptr_Impl :: operator_call_1_class_forward_iterator_int_ptr_ ( &f .as_pointer()  , )
            });
            0
        };
        let _arg5: i32 = {
            ({
                TestIter_TestImpl_const_int_ptr_Impl :: operator_call_1_class_cpp17_output_iterator_int_ptr_ ( &f .as_pointer()  , )
            });
            0
        };
        swallow_3(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub fn for_each_4(
    mut _a0 : types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp17_output_iterator_int_ptr__,
    f: TestIter_TestImpl_contiguous_iterator_const_int_ptr__,
) {
    let f: Value<TestIter_TestImpl_contiguous_iterator_const_int_ptr__> = Rc::new(RefCell::new(f));
    ({
        let _arg0: i32 = {
            ({
                TestIter_TestImpl_contiguous_iterator_const_int_ptr__Impl::operator_call_1_int_ptr(
                    &f.as_pointer(),
                )
            });
            0
        };
        let _arg1: i32 = {
            ({
                TestIter_TestImpl_contiguous_iterator_const_int_ptr__Impl :: operator_call_1_class_contiguous_iterator_int_ptr_ ( &f .as_pointer()  , )
            });
            0
        };
        let _arg2: i32 = {
            ({
                TestIter_TestImpl_contiguous_iterator_const_int_ptr__Impl :: operator_call_1_class_random_access_iterator_int_ptr_ ( &f .as_pointer()  , )
            });
            0
        };
        let _arg3: i32 = {
            ({
                TestIter_TestImpl_contiguous_iterator_const_int_ptr__Impl :: operator_call_1_class_bidirectional_iterator_int_ptr_ ( &f .as_pointer()  , )
            });
            0
        };
        let _arg4: i32 = {
            ({
                TestIter_TestImpl_contiguous_iterator_const_int_ptr__Impl :: operator_call_1_class_forward_iterator_int_ptr_ ( &f .as_pointer()  , )
            });
            0
        };
        let _arg5: i32 = {
            ({
                TestIter_TestImpl_contiguous_iterator_const_int_ptr__Impl :: operator_call_1_class_cpp17_output_iterator_int_ptr_ ( &f .as_pointer()  , )
            });
            0
        };
        swallow_3(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub fn for_each_5(
    mut _a0 : types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp17_output_iterator_int_ptr__,
    f: TestIter_TestImpl_random_access_iterator_const_int_ptr__,
) {
    let f: Value<TestIter_TestImpl_random_access_iterator_const_int_ptr__> =
        Rc::new(RefCell::new(f));
    ({
        let _arg0: i32 = {
            ({
                TestIter_TestImpl_random_access_iterator_const_int_ptr__Impl :: operator_call_1_int_ptr ( &f .as_pointer()  , )
            });
            0
        };
        let _arg1: i32 = {
            ({
                TestIter_TestImpl_random_access_iterator_const_int_ptr__Impl :: operator_call_1_class_contiguous_iterator_int_ptr_ ( &f .as_pointer()  , )
            });
            0
        };
        let _arg2: i32 = {
            ({
                TestIter_TestImpl_random_access_iterator_const_int_ptr__Impl :: operator_call_1_class_random_access_iterator_int_ptr_ ( &f .as_pointer()  , )
            });
            0
        };
        let _arg3: i32 = {
            ({
                TestIter_TestImpl_random_access_iterator_const_int_ptr__Impl :: operator_call_1_class_bidirectional_iterator_int_ptr_ ( &f .as_pointer()  , )
            });
            0
        };
        let _arg4: i32 = {
            ({
                TestIter_TestImpl_random_access_iterator_const_int_ptr__Impl :: operator_call_1_class_forward_iterator_int_ptr_ ( &f .as_pointer()  , )
            });
            0
        };
        let _arg5: i32 = {
            ({
                TestIter_TestImpl_random_access_iterator_const_int_ptr__Impl :: operator_call_1_class_cpp17_output_iterator_int_ptr_ ( &f .as_pointer()  , )
            });
            0
        };
        swallow_3(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub fn for_each_6(
    mut _a0 : types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp17_output_iterator_int_ptr__,
    f: TestIter_TestImpl_bidirectional_iterator_const_int_ptr__,
) {
    let f: Value<TestIter_TestImpl_bidirectional_iterator_const_int_ptr__> =
        Rc::new(RefCell::new(f));
    ({
        let _arg0: i32 = {
            ({
                TestIter_TestImpl_bidirectional_iterator_const_int_ptr__Impl :: operator_call_1_int_ptr ( &f .as_pointer()  , )
            });
            0
        };
        let _arg1: i32 = {
            ({
                TestIter_TestImpl_bidirectional_iterator_const_int_ptr__Impl :: operator_call_1_class_contiguous_iterator_int_ptr_ ( &f .as_pointer()  , )
            });
            0
        };
        let _arg2: i32 = {
            ({
                TestIter_TestImpl_bidirectional_iterator_const_int_ptr__Impl :: operator_call_1_class_random_access_iterator_int_ptr_ ( &f .as_pointer()  , )
            });
            0
        };
        let _arg3: i32 = {
            ({
                TestIter_TestImpl_bidirectional_iterator_const_int_ptr__Impl :: operator_call_1_class_bidirectional_iterator_int_ptr_ ( &f .as_pointer()  , )
            });
            0
        };
        let _arg4: i32 = {
            ({
                TestIter_TestImpl_bidirectional_iterator_const_int_ptr__Impl :: operator_call_1_class_forward_iterator_int_ptr_ ( &f .as_pointer()  , )
            });
            0
        };
        let _arg5: i32 = {
            ({
                TestIter_TestImpl_bidirectional_iterator_const_int_ptr__Impl :: operator_call_1_class_cpp17_output_iterator_int_ptr_ ( &f .as_pointer()  , )
            });
            0
        };
        swallow_3(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub fn swallow_3(
    mut _a0: i32,
    mut _a1: i32,
    mut _a2: i32,
    mut _a3: i32,
    mut _a4: i32,
    mut _a5: i32,
) {
}
pub fn swallow_1(mut _a0: i32, mut _a1: i32, mut _a2: i32, mut _a3: i32) {}
pub fn base_7(i: Ptr<cpp17_output_iterator_int_ptr_>) -> Ptr<i32> {
    return i.with(|__s| __s.it_.clone());
}
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
    pub fn new(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<cpp17_output_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppAdvance for cpp17_output_iterator_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_5();
    }
}
impl CppIterator<i32> for cpp17_output_iterator_int_ptr_ {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            cpp17_output_iterator_int_ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            cpp17_output_iterator_int_ptr_Impl::operator_inc_5(__it);
        });
        self
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
pub fn base_8(i: Ptr<forward_iterator_int_ptr_>) -> Ptr<i32> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct forward_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl forward_iterator_int_ptr_ {
    pub fn new_1(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<forward_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppAdvance for forward_iterator_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_6();
    }
}
impl CppIterator<i32> for forward_iterator_int_ptr_ {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            forward_iterator_int_ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            forward_iterator_int_ptr_Impl::operator_inc_6(__it);
        });
        self
    }
}
impl CppMove for forward_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn base_9(i: Ptr<bidirectional_iterator_int_ptr_>) -> Ptr<i32> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct bidirectional_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_int_ptr_ {
    pub fn new_1(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<bidirectional_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppAdvance for bidirectional_iterator_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_6();
    }
}
impl CppIterator<i32> for bidirectional_iterator_int_ptr_ {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            bidirectional_iterator_int_ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            bidirectional_iterator_int_ptr_Impl::operator_inc_6(__it);
        });
        self
    }
}
impl CppMove for bidirectional_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_ne_10(
    x: Ptr<bidirectional_iterator_const_int_ptr_>,
    y: Ptr<bidirectional_iterator_const_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct bidirectional_iterator_const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_const_int_ptr_ {
    pub fn new_1(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<bidirectional_iterator_const_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl std::cmp::PartialEq for bidirectional_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                !(operator_ne_10(__lhs.clone(), __rhs.clone()))
            })
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_const_int_ptr_ {}
impl CppAdvance for bidirectional_iterator_const_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_6();
    }
}
impl CppIterator<i32> for bidirectional_iterator_const_int_ptr_ {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            bidirectional_iterator_const_int_ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            bidirectional_iterator_const_int_ptr_Impl::operator_inc_6(__it);
        });
        self
    }
}
impl CppInputIterator<i32> for bidirectional_iterator_const_int_ptr_ {}
impl CppMove for bidirectional_iterator_const_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn base_11(i: Ptr<random_access_iterator_int_ptr_>) -> Ptr<i32> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct random_access_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_int_ptr_ {
    pub fn new_1(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<random_access_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppAdvance for random_access_iterator_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_7();
    }
}
impl CppIterator<i32> for random_access_iterator_int_ptr_ {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            random_access_iterator_int_ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            random_access_iterator_int_ptr_Impl::operator_inc_7(__it);
        });
        self
    }
}
impl CppMove for random_access_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_ne_12(
    x: Ptr<random_access_iterator_const_int_ptr_>,
    y: Ptr<random_access_iterator_const_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct random_access_iterator_const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_const_int_ptr_ {
    pub fn new_1(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<random_access_iterator_const_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl std::cmp::PartialEq for random_access_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                !(operator_ne_12(__lhs.clone(), __rhs.clone()))
            })
        }
    }
}
impl std::cmp::Eq for random_access_iterator_const_int_ptr_ {}
impl CppAdvance for random_access_iterator_const_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_7();
    }
}
impl CppIterator<i32> for random_access_iterator_const_int_ptr_ {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            random_access_iterator_const_int_ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            random_access_iterator_const_int_ptr_Impl::operator_inc_7(__it);
        });
        self
    }
}
impl CppInputIterator<i32> for random_access_iterator_const_int_ptr_ {}
impl CppMove for random_access_iterator_const_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
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
pub fn base_13(i: Ptr<contiguous_iterator_int_ptr_>) -> Ptr<i32> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct contiguous_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_int_ptr_ {
    pub fn new_1(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<contiguous_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppAdvance for contiguous_iterator_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_9();
    }
}
impl CppIterator<i32> for contiguous_iterator_int_ptr_ {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            contiguous_iterator_int_ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            contiguous_iterator_int_ptr_Impl::operator_inc_9(__it);
        });
        self
    }
}
impl CppMove for contiguous_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_sub_14(
    mut x: contiguous_iterator_const_int_ptr_,
    mut y: contiguous_iterator_const_int_ptr_,
) -> isize {
    return (((x.it_).clone() - (y.it_).clone()) as i64 as isize);
}
pub fn operator_ne_15(
    x: Ptr<contiguous_iterator_const_int_ptr_>,
    y: Ptr<contiguous_iterator_const_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct contiguous_iterator_const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_int_ptr_ {
    pub fn new_1(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<contiguous_iterator_const_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl std::cmp::PartialEq for contiguous_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                !(operator_ne_15(__lhs.clone(), __rhs.clone()))
            })
        }
    }
}
impl std::cmp::Eq for contiguous_iterator_const_int_ptr_ {}
impl CppAdvance for contiguous_iterator_const_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_9();
    }
}
impl CppIterator<i32> for contiguous_iterator_const_int_ptr_ {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            contiguous_iterator_const_int_ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            contiguous_iterator_const_int_ptr_Impl::operator_inc_9(__it);
        });
        self
    }
}
impl CppInputIterator<i32> for contiguous_iterator_const_int_ptr_ {}
impl CppMove for contiguous_iterator_const_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn base_16(i: Ptr<i32>) -> Ptr<i32> {
    let i: Value<Ptr<i32>> = Rc::new(RefCell::new(i));
    return (*i.borrow()).clone();
}
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
pub fn operator_eq_17(
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
        operator_eq_17(__lhs.clone(), __rhs.clone())
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
pub fn operator_add_18(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_19(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_18(_i, n)
    });
}
pub fn iter_swap_20(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap!((a.ptr_).clone(), (b.ptr_).clone());
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| {
            __v.prefix_inc();
        });
    }
}
pub fn iter_move_21(iter: adl_Iterator) -> Ptr<i32> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| {
            __v.prefix_inc();
        });
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) });
}
pub fn operator_eq_22(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> bool {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_23(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> std::cmp::Ordering {
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
                operator_cmp_23(__lhs.clone(), __rhs.clone())
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
                operator_eq_22(__lhs.clone(), __rhs.clone())
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestIter_TestImpl_const_int_ptr_ {}
impl Callable0<()> for TestIter_TestImpl_const_int_ptr_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestIter_TestImpl_const_int_ptr_Impl::operator_call_1_int_ptr(__it)
        })
    }
}
impl CppMove for TestIter_TestImpl_const_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestIter_TestImpl_contiguous_iterator_const_int_ptr__ {}
impl Callable0<()> for TestIter_TestImpl_contiguous_iterator_const_int_ptr__ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestIter_TestImpl_contiguous_iterator_const_int_ptr__Impl::operator_call_1_int_ptr(__it)
        })
    }
}
impl CppMove for TestIter_TestImpl_contiguous_iterator_const_int_ptr__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestIter_TestImpl_random_access_iterator_const_int_ptr__ {}
impl Callable0<()> for TestIter_TestImpl_random_access_iterator_const_int_ptr__ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestIter_TestImpl_random_access_iterator_const_int_ptr__Impl::operator_call_1_int_ptr(
                __it,
            )
        })
    }
}
impl CppMove for TestIter_TestImpl_random_access_iterator_const_int_ptr__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestIter_TestImpl_bidirectional_iterator_const_int_ptr__ {}
impl Callable0<()> for TestIter_TestImpl_bidirectional_iterator_const_int_ptr__ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestIter_TestImpl_bidirectional_iterator_const_int_ptr__Impl::operator_call_1_int_ptr(
                __it,
            )
        })
    }
}
impl CppMove for TestIter_TestImpl_bidirectional_iterator_const_int_ptr__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestIter {}
impl Callable0<()> for TestIter {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestIterImpl::operator_call_1_const_int_ptr(__it)
        })
    }
}
impl CppMove for TestIter {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_52() -> bool {
    ({
        for_each_0 ( <types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr__  >::default()  , <TestIter  >::default()  , )
    });
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
    ({ test_52() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub trait TestIterImpl {
    fn operator_call_1_const_int_ptr(&self);
    fn operator_call_1_class_contiguous_iterator_const_int_ptr_(&self);
    fn operator_call_1_class_random_access_iterator_const_int_ptr_(&self);
    fn operator_call_1_class_bidirectional_iterator_const_int_ptr_(&self);
}
impl TestIterImpl for Ptr<TestIter> {
    fn operator_call_1_const_int_ptr(&self) {
        ({
            for_each_2 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp17_output_iterator_int_ptr__  >::default()  , <TestIter_TestImpl_const_int_ptr_  >::default()  , )
        });
    }
    fn operator_call_1_class_contiguous_iterator_const_int_ptr_(&self) {
        ({
            for_each_4 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp17_output_iterator_int_ptr__  >::default()  , <TestIter_TestImpl_contiguous_iterator_const_int_ptr__  >::default()  , )
        });
    }
    fn operator_call_1_class_random_access_iterator_const_int_ptr_(&self) {
        ({
            for_each_5 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp17_output_iterator_int_ptr__  >::default()  , <TestIter_TestImpl_random_access_iterator_const_int_ptr__  >::default()  , )
        });
    }
    fn operator_call_1_class_bidirectional_iterator_const_int_ptr_(&self) {
        ({
            for_each_6 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp17_output_iterator_int_ptr__  >::default()  , <TestIter_TestImpl_bidirectional_iterator_const_int_ptr__  >::default()  , )
        });
    }
}
pub trait TestIter_TestImpl_bidirectional_iterator_const_int_ptr__Impl {
    fn operator_call_1_int_ptr(&self);
    fn operator_call_1_class_contiguous_iterator_int_ptr_(&self);
    fn operator_call_1_class_random_access_iterator_int_ptr_(&self);
    fn operator_call_1_class_bidirectional_iterator_int_ptr_(&self);
    fn operator_call_1_class_forward_iterator_int_ptr_(&self);
    fn operator_call_1_class_cpp17_output_iterator_int_ptr_(&self);
}
impl TestIter_TestImpl_bidirectional_iterator_const_int_ptr__Impl
    for Ptr<TestIter_TestImpl_bidirectional_iterator_const_int_ptr__>
{
    fn operator_call_1_int_ptr(&self) {
        let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3])));
        let mut sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as u32);
        let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0_i32, 0_i32, 0_i32])));
        let mut r: Ptr<i32> = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            rotate_copy_46(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(({ ({ base_16((r).clone(),) }) } == { (ib.as_pointer() as Ptr::<i32>) }));
        r = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_46(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((1) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        r = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_46(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((1) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        r = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_46(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((2) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        r = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_46(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((2) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        r = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_46(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((2) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        r = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_46(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((3) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        r = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_46(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((3) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        r = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_46(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((3) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        r = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_46(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((3) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        r = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_46(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((4) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        r = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_46(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((4) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 0) }));
        r = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_46(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((4) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 1) }));
        r = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_46(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((4) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 2) }));
        r = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_46(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((4) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        {
            let ints: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 2, 5, 6])));
            let mut n_ints: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                as i32);
            let zeros: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
                0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
            ])));
            let mut N: usize = 2_usize;
            let mut middle: Ptr<i32> = ({ begin_25(ints.as_pointer()) }).offset((N) as isize);
            let mut it: Ptr<i32> = ({
                let ___first: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                let ___last: Ptr<i32> = (ints.as_pointer() as Ptr<i32>).to_end();
                let ___result: Ptr<i32> = ({ begin_25(zeros.as_pointer()) });
                rotate_copy_26(___first, (middle).clone(), ___last, ___result)
            });
            assert!(
                ({ ({ distance_27(({ begin_25(zeros.as_pointer(),) }), (it).clone(),) }) } == {
                    (n_ints as isize)
                })
            );
            assert!(
                ({
                    let ___first1: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                    let ___first2: Ptr<i32> = ({ begin_25(zeros.as_pointer()) })
                        .offset((n_ints) as isize)
                        .offset(-((N) as isize));
                    equal_28(___first1, (middle).clone(), ___first2)
                })
            );
            assert!(
                ({
                    equal_28(
                        (middle).clone(),
                        (ints.as_pointer() as Ptr<i32>).to_end(),
                        ({ begin_25(zeros.as_pointer()) }),
                    )
                })
            );
        }
    }
    fn operator_call_1_class_contiguous_iterator_int_ptr_(&self) {
        let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3])));
        let mut sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as u32);
        let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0_i32, 0_i32, 0_i32])));
        let r: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                rotate_copy_47(
                    ___first,
                    ___middle,
                    ___last,
                    contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
                )
            }),
        ));
        assert!((({ base_13(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>)));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_47(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_47(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_47(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_47(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_47(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_47(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_47(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_47(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_47(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_47(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_47(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_47(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_47(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_47(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        {
            let ints: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 2, 5, 6])));
            let mut n_ints: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                as i32);
            let zeros: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
                0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
            ])));
            let mut N: usize = 2_usize;
            let mut middle: Ptr<i32> = ({ begin_25(ints.as_pointer()) }).offset((N) as isize);
            let mut it: Ptr<i32> = ({
                let ___first: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                let ___last: Ptr<i32> = (ints.as_pointer() as Ptr<i32>).to_end();
                let ___result: Ptr<i32> = ({ begin_25(zeros.as_pointer()) });
                rotate_copy_26(___first, (middle).clone(), ___last, ___result)
            });
            assert!(
                ({ ({ distance_27(({ begin_25(zeros.as_pointer(),) }), (it).clone(),) }) } == {
                    (n_ints as isize)
                })
            );
            assert!(
                ({
                    let ___first1: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                    let ___first2: Ptr<i32> = ({ begin_25(zeros.as_pointer()) })
                        .offset((n_ints) as isize)
                        .offset(-((N) as isize));
                    equal_28(___first1, (middle).clone(), ___first2)
                })
            );
            assert!(
                ({
                    equal_28(
                        (middle).clone(),
                        (ints.as_pointer() as Ptr<i32>).to_end(),
                        ({ begin_25(zeros.as_pointer()) }),
                    )
                })
            );
        }
    }
    fn operator_call_1_class_random_access_iterator_int_ptr_(&self) {
        let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3])));
        let mut sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as u32);
        let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0_i32, 0_i32, 0_i32])));
        let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                rotate_copy_48(
                    ___first,
                    ___middle,
                    ___last,
                    random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
                )
            }),
        ));
        assert!((({ base_11(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>)));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_48(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_48(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_48(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_48(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_48(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_48(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_48(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_48(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_48(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_48(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_48(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_48(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_48(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_48(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        {
            let ints: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 2, 5, 6])));
            let mut n_ints: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                as i32);
            let zeros: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
                0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
            ])));
            let mut N: usize = 2_usize;
            let mut middle: Ptr<i32> = ({ begin_25(ints.as_pointer()) }).offset((N) as isize);
            let mut it: Ptr<i32> = ({
                let ___first: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                let ___last: Ptr<i32> = (ints.as_pointer() as Ptr<i32>).to_end();
                let ___result: Ptr<i32> = ({ begin_25(zeros.as_pointer()) });
                rotate_copy_26(___first, (middle).clone(), ___last, ___result)
            });
            assert!(
                ({ ({ distance_27(({ begin_25(zeros.as_pointer(),) }), (it).clone(),) }) } == {
                    (n_ints as isize)
                })
            );
            assert!(
                ({
                    let ___first1: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                    let ___first2: Ptr<i32> = ({ begin_25(zeros.as_pointer()) })
                        .offset((n_ints) as isize)
                        .offset(-((N) as isize));
                    equal_28(___first1, (middle).clone(), ___first2)
                })
            );
            assert!(
                ({
                    equal_28(
                        (middle).clone(),
                        (ints.as_pointer() as Ptr<i32>).to_end(),
                        ({ begin_25(zeros.as_pointer()) }),
                    )
                })
            );
        }
    }
    fn operator_call_1_class_bidirectional_iterator_int_ptr_(&self) {
        let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3])));
        let mut sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as u32);
        let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0_i32, 0_i32, 0_i32])));
        let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                rotate_copy_49(
                    ___first,
                    ___middle,
                    ___last,
                    bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
                )
            }),
        ));
        assert!((({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>)));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_49(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_49(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_49(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_49(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_49(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_49(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_49(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_49(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_49(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_49(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_49(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_49(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_49(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_49(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        {
            let ints: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 2, 5, 6])));
            let mut n_ints: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                as i32);
            let zeros: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
                0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
            ])));
            let mut N: usize = 2_usize;
            let mut middle: Ptr<i32> = ({ begin_25(ints.as_pointer()) }).offset((N) as isize);
            let mut it: Ptr<i32> = ({
                let ___first: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                let ___last: Ptr<i32> = (ints.as_pointer() as Ptr<i32>).to_end();
                let ___result: Ptr<i32> = ({ begin_25(zeros.as_pointer()) });
                rotate_copy_26(___first, (middle).clone(), ___last, ___result)
            });
            assert!(
                ({ ({ distance_27(({ begin_25(zeros.as_pointer(),) }), (it).clone(),) }) } == {
                    (n_ints as isize)
                })
            );
            assert!(
                ({
                    let ___first1: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                    let ___first2: Ptr<i32> = ({ begin_25(zeros.as_pointer()) })
                        .offset((n_ints) as isize)
                        .offset(-((N) as isize));
                    equal_28(___first1, (middle).clone(), ___first2)
                })
            );
            assert!(
                ({
                    equal_28(
                        (middle).clone(),
                        (ints.as_pointer() as Ptr<i32>).to_end(),
                        ({ begin_25(zeros.as_pointer()) }),
                    )
                })
            );
        }
    }
    fn operator_call_1_class_forward_iterator_int_ptr_(&self) {
        let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3])));
        let mut sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as u32);
        let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0_i32, 0_i32, 0_i32])));
        let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                rotate_copy_50(
                    ___first,
                    ___middle,
                    ___last,
                    forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
                )
            }),
        ));
        assert!((({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>)));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_50(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_50(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_50(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_50(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_50(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_50(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_50(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_50(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_50(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_50(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_50(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_50(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_50(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_50(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        {
            let ints: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 2, 5, 6])));
            let mut n_ints: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                as i32);
            let zeros: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
                0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
            ])));
            let mut N: usize = 2_usize;
            let mut middle: Ptr<i32> = ({ begin_25(ints.as_pointer()) }).offset((N) as isize);
            let mut it: Ptr<i32> = ({
                let ___first: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                let ___last: Ptr<i32> = (ints.as_pointer() as Ptr<i32>).to_end();
                let ___result: Ptr<i32> = ({ begin_25(zeros.as_pointer()) });
                rotate_copy_26(___first, (middle).clone(), ___last, ___result)
            });
            assert!(
                ({ ({ distance_27(({ begin_25(zeros.as_pointer(),) }), (it).clone(),) }) } == {
                    (n_ints as isize)
                })
            );
            assert!(
                ({
                    let ___first1: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                    let ___first2: Ptr<i32> = ({ begin_25(zeros.as_pointer()) })
                        .offset((n_ints) as isize)
                        .offset(-((N) as isize));
                    equal_28(___first1, (middle).clone(), ___first2)
                })
            );
            assert!(
                ({
                    equal_28(
                        (middle).clone(),
                        (ints.as_pointer() as Ptr<i32>).to_end(),
                        ({ begin_25(zeros.as_pointer()) }),
                    )
                })
            );
        }
    }
    fn operator_call_1_class_cpp17_output_iterator_int_ptr_(&self) {
        let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3])));
        let mut sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as u32);
        let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0_i32, 0_i32, 0_i32])));
        let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                let ___first: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___middle: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___last: bidirectional_iterator_const_int_ptr_ =
                    bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                rotate_copy_51(
                    ___first,
                    ___middle,
                    ___last,
                    cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
                )
            }),
        ));
        assert!((({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>)));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_51(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_51(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_51(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_51(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_51(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_51(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_51(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_51(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_51(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_51(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_51(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_51(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_51(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            let ___last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_51(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        {
            let ints: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 2, 5, 6])));
            let mut n_ints: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                as i32);
            let zeros: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
                0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
            ])));
            let mut N: usize = 2_usize;
            let mut middle: Ptr<i32> = ({ begin_25(ints.as_pointer()) }).offset((N) as isize);
            let mut it: Ptr<i32> = ({
                let ___first: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                let ___last: Ptr<i32> = (ints.as_pointer() as Ptr<i32>).to_end();
                let ___result: Ptr<i32> = ({ begin_25(zeros.as_pointer()) });
                rotate_copy_26(___first, (middle).clone(), ___last, ___result)
            });
            assert!(
                ({ ({ distance_27(({ begin_25(zeros.as_pointer(),) }), (it).clone(),) }) } == {
                    (n_ints as isize)
                })
            );
            assert!(
                ({
                    let ___first1: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                    let ___first2: Ptr<i32> = ({ begin_25(zeros.as_pointer()) })
                        .offset((n_ints) as isize)
                        .offset(-((N) as isize));
                    equal_28(___first1, (middle).clone(), ___first2)
                })
            );
            assert!(
                ({
                    equal_28(
                        (middle).clone(),
                        (ints.as_pointer() as Ptr<i32>).to_end(),
                        ({ begin_25(zeros.as_pointer()) }),
                    )
                })
            );
        }
    }
}
pub trait TestIter_TestImpl_const_int_ptr_Impl {
    fn operator_call_1_int_ptr(&self);
    fn operator_call_1_class_contiguous_iterator_int_ptr_(&self);
    fn operator_call_1_class_random_access_iterator_int_ptr_(&self);
    fn operator_call_1_class_bidirectional_iterator_int_ptr_(&self);
    fn operator_call_1_class_forward_iterator_int_ptr_(&self);
    fn operator_call_1_class_cpp17_output_iterator_int_ptr_(&self);
}
impl TestIter_TestImpl_const_int_ptr_Impl for Ptr<TestIter_TestImpl_const_int_ptr_> {
    fn operator_call_1_int_ptr(&self) {
        let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3])));
        let mut sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as u32);
        let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0_i32, 0_i32, 0_i32])));
        let mut r: Ptr<i32> = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            rotate_copy_24(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(({ ({ base_16((r).clone(),) }) } == { (ib.as_pointer() as Ptr::<i32>) }));
        r = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            rotate_copy_24(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((1) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        r = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            rotate_copy_24(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((1) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        r = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            rotate_copy_24(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((2) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        r = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            rotate_copy_24(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((2) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        r = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            rotate_copy_24(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((2) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        r = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            rotate_copy_24(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((3) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        r = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            rotate_copy_24(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((3) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        r = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            rotate_copy_24(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((3) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        r = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            rotate_copy_24(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((3) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        r = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_24(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((4) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        r = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_24(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((4) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 0) }));
        r = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_24(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((4) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 1) }));
        r = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_24(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((4) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 2) }));
        r = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_24(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((4) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        {
            let ints: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 2, 5, 6])));
            let mut n_ints: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                as i32);
            let zeros: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
                0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
            ])));
            let mut N: usize = 2_usize;
            let mut middle: Ptr<i32> = ({ begin_25(ints.as_pointer()) }).offset((N) as isize);
            let mut it: Ptr<i32> = ({
                let ___first: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                let ___last: Ptr<i32> = (ints.as_pointer() as Ptr<i32>).to_end();
                let ___result: Ptr<i32> = ({ begin_25(zeros.as_pointer()) });
                rotate_copy_26(___first, (middle).clone(), ___last, ___result)
            });
            assert!(
                ({ ({ distance_27(({ begin_25(zeros.as_pointer(),) }), (it).clone(),) }) } == {
                    (n_ints as isize)
                })
            );
            assert!(
                ({
                    let ___first1: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                    let ___first2: Ptr<i32> = ({ begin_25(zeros.as_pointer()) })
                        .offset((n_ints) as isize)
                        .offset(-((N) as isize));
                    equal_28(___first1, (middle).clone(), ___first2)
                })
            );
            assert!(
                ({
                    equal_28(
                        (middle).clone(),
                        (ints.as_pointer() as Ptr<i32>).to_end(),
                        ({ begin_25(zeros.as_pointer()) }),
                    )
                })
            );
        }
    }
    fn operator_call_1_class_contiguous_iterator_int_ptr_(&self) {
        let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3])));
        let mut sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as u32);
        let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0_i32, 0_i32, 0_i32])));
        let r: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
                let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
                let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
                rotate_copy_29(
                    ___first,
                    ___middle,
                    ___last,
                    contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
                )
            }),
        ));
        assert!((({ base_13(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>)));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            rotate_copy_29(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            rotate_copy_29(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            rotate_copy_29(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            rotate_copy_29(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            rotate_copy_29(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            rotate_copy_29(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            rotate_copy_29(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            rotate_copy_29(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            rotate_copy_29(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_29(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_29(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_29(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_29(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_29(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        {
            let ints: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 2, 5, 6])));
            let mut n_ints: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                as i32);
            let zeros: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
                0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
            ])));
            let mut N: usize = 2_usize;
            let mut middle: Ptr<i32> = ({ begin_25(ints.as_pointer()) }).offset((N) as isize);
            let mut it: Ptr<i32> = ({
                let ___first: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                let ___last: Ptr<i32> = (ints.as_pointer() as Ptr<i32>).to_end();
                let ___result: Ptr<i32> = ({ begin_25(zeros.as_pointer()) });
                rotate_copy_26(___first, (middle).clone(), ___last, ___result)
            });
            assert!(
                ({ ({ distance_27(({ begin_25(zeros.as_pointer(),) }), (it).clone(),) }) } == {
                    (n_ints as isize)
                })
            );
            assert!(
                ({
                    let ___first1: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                    let ___first2: Ptr<i32> = ({ begin_25(zeros.as_pointer()) })
                        .offset((n_ints) as isize)
                        .offset(-((N) as isize));
                    equal_28(___first1, (middle).clone(), ___first2)
                })
            );
            assert!(
                ({
                    equal_28(
                        (middle).clone(),
                        (ints.as_pointer() as Ptr<i32>).to_end(),
                        ({ begin_25(zeros.as_pointer()) }),
                    )
                })
            );
        }
    }
    fn operator_call_1_class_random_access_iterator_int_ptr_(&self) {
        let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3])));
        let mut sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as u32);
        let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0_i32, 0_i32, 0_i32])));
        let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
                let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
                let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
                rotate_copy_30(
                    ___first,
                    ___middle,
                    ___last,
                    random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
                )
            }),
        ));
        assert!((({ base_11(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>)));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            rotate_copy_30(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            rotate_copy_30(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            rotate_copy_30(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            rotate_copy_30(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            rotate_copy_30(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            rotate_copy_30(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            rotate_copy_30(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            rotate_copy_30(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            rotate_copy_30(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_30(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_30(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_30(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_30(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_30(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        {
            let ints: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 2, 5, 6])));
            let mut n_ints: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                as i32);
            let zeros: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
                0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
            ])));
            let mut N: usize = 2_usize;
            let mut middle: Ptr<i32> = ({ begin_25(ints.as_pointer()) }).offset((N) as isize);
            let mut it: Ptr<i32> = ({
                let ___first: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                let ___last: Ptr<i32> = (ints.as_pointer() as Ptr<i32>).to_end();
                let ___result: Ptr<i32> = ({ begin_25(zeros.as_pointer()) });
                rotate_copy_26(___first, (middle).clone(), ___last, ___result)
            });
            assert!(
                ({ ({ distance_27(({ begin_25(zeros.as_pointer(),) }), (it).clone(),) }) } == {
                    (n_ints as isize)
                })
            );
            assert!(
                ({
                    let ___first1: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                    let ___first2: Ptr<i32> = ({ begin_25(zeros.as_pointer()) })
                        .offset((n_ints) as isize)
                        .offset(-((N) as isize));
                    equal_28(___first1, (middle).clone(), ___first2)
                })
            );
            assert!(
                ({
                    equal_28(
                        (middle).clone(),
                        (ints.as_pointer() as Ptr<i32>).to_end(),
                        ({ begin_25(zeros.as_pointer()) }),
                    )
                })
            );
        }
    }
    fn operator_call_1_class_bidirectional_iterator_int_ptr_(&self) {
        let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3])));
        let mut sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as u32);
        let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0_i32, 0_i32, 0_i32])));
        let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
                let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
                let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
                rotate_copy_31(
                    ___first,
                    ___middle,
                    ___last,
                    bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
                )
            }),
        ));
        assert!((({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>)));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            rotate_copy_31(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            rotate_copy_31(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            rotate_copy_31(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            rotate_copy_31(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            rotate_copy_31(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            rotate_copy_31(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            rotate_copy_31(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            rotate_copy_31(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            rotate_copy_31(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_31(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_31(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_31(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_31(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_31(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        {
            let ints: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 2, 5, 6])));
            let mut n_ints: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                as i32);
            let zeros: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
                0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
            ])));
            let mut N: usize = 2_usize;
            let mut middle: Ptr<i32> = ({ begin_25(ints.as_pointer()) }).offset((N) as isize);
            let mut it: Ptr<i32> = ({
                let ___first: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                let ___last: Ptr<i32> = (ints.as_pointer() as Ptr<i32>).to_end();
                let ___result: Ptr<i32> = ({ begin_25(zeros.as_pointer()) });
                rotate_copy_26(___first, (middle).clone(), ___last, ___result)
            });
            assert!(
                ({ ({ distance_27(({ begin_25(zeros.as_pointer(),) }), (it).clone(),) }) } == {
                    (n_ints as isize)
                })
            );
            assert!(
                ({
                    let ___first1: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                    let ___first2: Ptr<i32> = ({ begin_25(zeros.as_pointer()) })
                        .offset((n_ints) as isize)
                        .offset(-((N) as isize));
                    equal_28(___first1, (middle).clone(), ___first2)
                })
            );
            assert!(
                ({
                    equal_28(
                        (middle).clone(),
                        (ints.as_pointer() as Ptr<i32>).to_end(),
                        ({ begin_25(zeros.as_pointer()) }),
                    )
                })
            );
        }
    }
    fn operator_call_1_class_forward_iterator_int_ptr_(&self) {
        let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3])));
        let mut sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as u32);
        let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0_i32, 0_i32, 0_i32])));
        let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
                let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
                let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
                rotate_copy_32(
                    ___first,
                    ___middle,
                    ___last,
                    forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
                )
            }),
        ));
        assert!((({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>)));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            rotate_copy_32(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            rotate_copy_32(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            rotate_copy_32(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            rotate_copy_32(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            rotate_copy_32(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            rotate_copy_32(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            rotate_copy_32(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            rotate_copy_32(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            rotate_copy_32(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_32(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_32(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_32(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_32(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_32(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        {
            let ints: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 2, 5, 6])));
            let mut n_ints: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                as i32);
            let zeros: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
                0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
            ])));
            let mut N: usize = 2_usize;
            let mut middle: Ptr<i32> = ({ begin_25(ints.as_pointer()) }).offset((N) as isize);
            let mut it: Ptr<i32> = ({
                let ___first: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                let ___last: Ptr<i32> = (ints.as_pointer() as Ptr<i32>).to_end();
                let ___result: Ptr<i32> = ({ begin_25(zeros.as_pointer()) });
                rotate_copy_26(___first, (middle).clone(), ___last, ___result)
            });
            assert!(
                ({ ({ distance_27(({ begin_25(zeros.as_pointer(),) }), (it).clone(),) }) } == {
                    (n_ints as isize)
                })
            );
            assert!(
                ({
                    let ___first1: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                    let ___first2: Ptr<i32> = ({ begin_25(zeros.as_pointer()) })
                        .offset((n_ints) as isize)
                        .offset(-((N) as isize));
                    equal_28(___first1, (middle).clone(), ___first2)
                })
            );
            assert!(
                ({
                    equal_28(
                        (middle).clone(),
                        (ints.as_pointer() as Ptr<i32>).to_end(),
                        ({ begin_25(zeros.as_pointer()) }),
                    )
                })
            );
        }
    }
    fn operator_call_1_class_cpp17_output_iterator_int_ptr_(&self) {
        let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3])));
        let mut sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as u32);
        let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0_i32, 0_i32, 0_i32])));
        let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
                let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
                let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
                rotate_copy_33(
                    ___first,
                    ___middle,
                    ___last,
                    cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
                )
            }),
        ));
        assert!((({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>)));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            rotate_copy_33(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            rotate_copy_33(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            rotate_copy_33(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            rotate_copy_33(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            rotate_copy_33(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            rotate_copy_33(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            rotate_copy_33(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            rotate_copy_33(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            rotate_copy_33(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_33(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((1) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_33(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((2) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_33(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((3) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_33(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___middle: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((4) as isize);
            rotate_copy_33(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        {
            let ints: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 2, 5, 6])));
            let mut n_ints: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                as i32);
            let zeros: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
                0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
            ])));
            let mut N: usize = 2_usize;
            let mut middle: Ptr<i32> = ({ begin_25(ints.as_pointer()) }).offset((N) as isize);
            let mut it: Ptr<i32> = ({
                let ___first: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                let ___last: Ptr<i32> = (ints.as_pointer() as Ptr<i32>).to_end();
                let ___result: Ptr<i32> = ({ begin_25(zeros.as_pointer()) });
                rotate_copy_26(___first, (middle).clone(), ___last, ___result)
            });
            assert!(
                ({ ({ distance_27(({ begin_25(zeros.as_pointer(),) }), (it).clone(),) }) } == {
                    (n_ints as isize)
                })
            );
            assert!(
                ({
                    let ___first1: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                    let ___first2: Ptr<i32> = ({ begin_25(zeros.as_pointer()) })
                        .offset((n_ints) as isize)
                        .offset(-((N) as isize));
                    equal_28(___first1, (middle).clone(), ___first2)
                })
            );
            assert!(
                ({
                    equal_28(
                        (middle).clone(),
                        (ints.as_pointer() as Ptr<i32>).to_end(),
                        ({ begin_25(zeros.as_pointer()) }),
                    )
                })
            );
        }
    }
}
pub trait TestIter_TestImpl_contiguous_iterator_const_int_ptr__Impl {
    fn operator_call_1_int_ptr(&self);
    fn operator_call_1_class_contiguous_iterator_int_ptr_(&self);
    fn operator_call_1_class_random_access_iterator_int_ptr_(&self);
    fn operator_call_1_class_bidirectional_iterator_int_ptr_(&self);
    fn operator_call_1_class_forward_iterator_int_ptr_(&self);
    fn operator_call_1_class_cpp17_output_iterator_int_ptr_(&self);
}
impl TestIter_TestImpl_contiguous_iterator_const_int_ptr__Impl
    for Ptr<TestIter_TestImpl_contiguous_iterator_const_int_ptr__>
{
    fn operator_call_1_int_ptr(&self) {
        let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3])));
        let mut sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as u32);
        let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0_i32, 0_i32, 0_i32])));
        let mut r: Ptr<i32> = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            rotate_copy_34(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(({ ({ base_16((r).clone(),) }) } == { (ib.as_pointer() as Ptr::<i32>) }));
        r = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_34(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((1) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        r = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_34(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((1) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        r = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_34(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((2) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        r = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_34(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((2) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        r = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_34(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((2) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        r = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_34(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((3) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        r = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_34(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((3) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        r = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_34(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((3) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        r = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_34(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((3) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        r = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_34(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((4) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        r = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_34(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((4) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 0) }));
        r = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_34(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((4) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 1) }));
        r = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_34(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((4) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 2) }));
        r = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_34(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((4) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        {
            let ints: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 2, 5, 6])));
            let mut n_ints: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                as i32);
            let zeros: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
                0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
            ])));
            let mut N: usize = 2_usize;
            let mut middle: Ptr<i32> = ({ begin_25(ints.as_pointer()) }).offset((N) as isize);
            let mut it: Ptr<i32> = ({
                let ___first: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                let ___last: Ptr<i32> = (ints.as_pointer() as Ptr<i32>).to_end();
                let ___result: Ptr<i32> = ({ begin_25(zeros.as_pointer()) });
                rotate_copy_26(___first, (middle).clone(), ___last, ___result)
            });
            assert!(
                ({ ({ distance_27(({ begin_25(zeros.as_pointer(),) }), (it).clone(),) }) } == {
                    (n_ints as isize)
                })
            );
            assert!(
                ({
                    let ___first1: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                    let ___first2: Ptr<i32> = ({ begin_25(zeros.as_pointer()) })
                        .offset((n_ints) as isize)
                        .offset(-((N) as isize));
                    equal_28(___first1, (middle).clone(), ___first2)
                })
            );
            assert!(
                ({
                    equal_28(
                        (middle).clone(),
                        (ints.as_pointer() as Ptr<i32>).to_end(),
                        ({ begin_25(zeros.as_pointer()) }),
                    )
                })
            );
        }
    }
    fn operator_call_1_class_contiguous_iterator_int_ptr_(&self) {
        let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3])));
        let mut sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as u32);
        let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0_i32, 0_i32, 0_i32])));
        let r: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                rotate_copy_35(
                    ___first,
                    ___middle,
                    ___last,
                    contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
                )
            }),
        ));
        assert!((({ base_13(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>)));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_35(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_35(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_35(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_35(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_35(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_35(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_35(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_35(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_35(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_35(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_35(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_35(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_35(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_35(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        {
            let ints: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 2, 5, 6])));
            let mut n_ints: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                as i32);
            let zeros: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
                0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
            ])));
            let mut N: usize = 2_usize;
            let mut middle: Ptr<i32> = ({ begin_25(ints.as_pointer()) }).offset((N) as isize);
            let mut it: Ptr<i32> = ({
                let ___first: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                let ___last: Ptr<i32> = (ints.as_pointer() as Ptr<i32>).to_end();
                let ___result: Ptr<i32> = ({ begin_25(zeros.as_pointer()) });
                rotate_copy_26(___first, (middle).clone(), ___last, ___result)
            });
            assert!(
                ({ ({ distance_27(({ begin_25(zeros.as_pointer(),) }), (it).clone(),) }) } == {
                    (n_ints as isize)
                })
            );
            assert!(
                ({
                    let ___first1: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                    let ___first2: Ptr<i32> = ({ begin_25(zeros.as_pointer()) })
                        .offset((n_ints) as isize)
                        .offset(-((N) as isize));
                    equal_28(___first1, (middle).clone(), ___first2)
                })
            );
            assert!(
                ({
                    equal_28(
                        (middle).clone(),
                        (ints.as_pointer() as Ptr<i32>).to_end(),
                        ({ begin_25(zeros.as_pointer()) }),
                    )
                })
            );
        }
    }
    fn operator_call_1_class_random_access_iterator_int_ptr_(&self) {
        let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3])));
        let mut sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as u32);
        let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0_i32, 0_i32, 0_i32])));
        let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                rotate_copy_36(
                    ___first,
                    ___middle,
                    ___last,
                    random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
                )
            }),
        ));
        assert!((({ base_11(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>)));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_36(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_36(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_36(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_36(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_36(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_36(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_36(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_36(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_36(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_36(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_36(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_36(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_36(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_36(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        {
            let ints: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 2, 5, 6])));
            let mut n_ints: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                as i32);
            let zeros: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
                0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
            ])));
            let mut N: usize = 2_usize;
            let mut middle: Ptr<i32> = ({ begin_25(ints.as_pointer()) }).offset((N) as isize);
            let mut it: Ptr<i32> = ({
                let ___first: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                let ___last: Ptr<i32> = (ints.as_pointer() as Ptr<i32>).to_end();
                let ___result: Ptr<i32> = ({ begin_25(zeros.as_pointer()) });
                rotate_copy_26(___first, (middle).clone(), ___last, ___result)
            });
            assert!(
                ({ ({ distance_27(({ begin_25(zeros.as_pointer(),) }), (it).clone(),) }) } == {
                    (n_ints as isize)
                })
            );
            assert!(
                ({
                    let ___first1: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                    let ___first2: Ptr<i32> = ({ begin_25(zeros.as_pointer()) })
                        .offset((n_ints) as isize)
                        .offset(-((N) as isize));
                    equal_28(___first1, (middle).clone(), ___first2)
                })
            );
            assert!(
                ({
                    equal_28(
                        (middle).clone(),
                        (ints.as_pointer() as Ptr<i32>).to_end(),
                        ({ begin_25(zeros.as_pointer()) }),
                    )
                })
            );
        }
    }
    fn operator_call_1_class_bidirectional_iterator_int_ptr_(&self) {
        let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3])));
        let mut sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as u32);
        let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0_i32, 0_i32, 0_i32])));
        let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                rotate_copy_37(
                    ___first,
                    ___middle,
                    ___last,
                    bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
                )
            }),
        ));
        assert!((({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>)));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_37(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_37(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_37(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_37(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_37(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_37(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_37(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_37(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_37(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_37(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_37(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_37(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_37(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_37(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        {
            let ints: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 2, 5, 6])));
            let mut n_ints: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                as i32);
            let zeros: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
                0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
            ])));
            let mut N: usize = 2_usize;
            let mut middle: Ptr<i32> = ({ begin_25(ints.as_pointer()) }).offset((N) as isize);
            let mut it: Ptr<i32> = ({
                let ___first: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                let ___last: Ptr<i32> = (ints.as_pointer() as Ptr<i32>).to_end();
                let ___result: Ptr<i32> = ({ begin_25(zeros.as_pointer()) });
                rotate_copy_26(___first, (middle).clone(), ___last, ___result)
            });
            assert!(
                ({ ({ distance_27(({ begin_25(zeros.as_pointer(),) }), (it).clone(),) }) } == {
                    (n_ints as isize)
                })
            );
            assert!(
                ({
                    let ___first1: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                    let ___first2: Ptr<i32> = ({ begin_25(zeros.as_pointer()) })
                        .offset((n_ints) as isize)
                        .offset(-((N) as isize));
                    equal_28(___first1, (middle).clone(), ___first2)
                })
            );
            assert!(
                ({
                    equal_28(
                        (middle).clone(),
                        (ints.as_pointer() as Ptr<i32>).to_end(),
                        ({ begin_25(zeros.as_pointer()) }),
                    )
                })
            );
        }
    }
    fn operator_call_1_class_forward_iterator_int_ptr_(&self) {
        let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3])));
        let mut sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as u32);
        let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0_i32, 0_i32, 0_i32])));
        let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                rotate_copy_38(
                    ___first,
                    ___middle,
                    ___last,
                    forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
                )
            }),
        ));
        assert!((({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>)));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_38(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_38(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_38(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_38(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_38(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_38(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_38(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_38(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_38(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_38(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_38(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_38(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_38(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_38(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        {
            let ints: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 2, 5, 6])));
            let mut n_ints: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                as i32);
            let zeros: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
                0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
            ])));
            let mut N: usize = 2_usize;
            let mut middle: Ptr<i32> = ({ begin_25(ints.as_pointer()) }).offset((N) as isize);
            let mut it: Ptr<i32> = ({
                let ___first: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                let ___last: Ptr<i32> = (ints.as_pointer() as Ptr<i32>).to_end();
                let ___result: Ptr<i32> = ({ begin_25(zeros.as_pointer()) });
                rotate_copy_26(___first, (middle).clone(), ___last, ___result)
            });
            assert!(
                ({ ({ distance_27(({ begin_25(zeros.as_pointer(),) }), (it).clone(),) }) } == {
                    (n_ints as isize)
                })
            );
            assert!(
                ({
                    let ___first1: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                    let ___first2: Ptr<i32> = ({ begin_25(zeros.as_pointer()) })
                        .offset((n_ints) as isize)
                        .offset(-((N) as isize));
                    equal_28(___first1, (middle).clone(), ___first2)
                })
            );
            assert!(
                ({
                    equal_28(
                        (middle).clone(),
                        (ints.as_pointer() as Ptr<i32>).to_end(),
                        ({ begin_25(zeros.as_pointer()) }),
                    )
                })
            );
        }
    }
    fn operator_call_1_class_cpp17_output_iterator_int_ptr_(&self) {
        let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3])));
        let mut sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as u32);
        let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0_i32, 0_i32, 0_i32])));
        let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                let ___first: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___middle: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___last: contiguous_iterator_const_int_ptr_ =
                    contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                rotate_copy_39(
                    ___first,
                    ___middle,
                    ___last,
                    cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
                )
            }),
        ));
        assert!((({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>)));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_39(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_39(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_39(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_39(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_39(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_39(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_39(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_39(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_39(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_39(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_39(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_39(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_39(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            let ___last: contiguous_iterator_const_int_ptr_ =
                contiguous_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_39(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        {
            let ints: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 2, 5, 6])));
            let mut n_ints: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                as i32);
            let zeros: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
                0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
            ])));
            let mut N: usize = 2_usize;
            let mut middle: Ptr<i32> = ({ begin_25(ints.as_pointer()) }).offset((N) as isize);
            let mut it: Ptr<i32> = ({
                let ___first: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                let ___last: Ptr<i32> = (ints.as_pointer() as Ptr<i32>).to_end();
                let ___result: Ptr<i32> = ({ begin_25(zeros.as_pointer()) });
                rotate_copy_26(___first, (middle).clone(), ___last, ___result)
            });
            assert!(
                ({ ({ distance_27(({ begin_25(zeros.as_pointer(),) }), (it).clone(),) }) } == {
                    (n_ints as isize)
                })
            );
            assert!(
                ({
                    let ___first1: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                    let ___first2: Ptr<i32> = ({ begin_25(zeros.as_pointer()) })
                        .offset((n_ints) as isize)
                        .offset(-((N) as isize));
                    equal_28(___first1, (middle).clone(), ___first2)
                })
            );
            assert!(
                ({
                    equal_28(
                        (middle).clone(),
                        (ints.as_pointer() as Ptr<i32>).to_end(),
                        ({ begin_25(zeros.as_pointer()) }),
                    )
                })
            );
        }
    }
}
pub trait TestIter_TestImpl_random_access_iterator_const_int_ptr__Impl {
    fn operator_call_1_int_ptr(&self);
    fn operator_call_1_class_contiguous_iterator_int_ptr_(&self);
    fn operator_call_1_class_random_access_iterator_int_ptr_(&self);
    fn operator_call_1_class_bidirectional_iterator_int_ptr_(&self);
    fn operator_call_1_class_forward_iterator_int_ptr_(&self);
    fn operator_call_1_class_cpp17_output_iterator_int_ptr_(&self);
}
impl TestIter_TestImpl_random_access_iterator_const_int_ptr__Impl
    for Ptr<TestIter_TestImpl_random_access_iterator_const_int_ptr__>
{
    fn operator_call_1_int_ptr(&self) {
        let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3])));
        let mut sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as u32);
        let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0_i32, 0_i32, 0_i32])));
        let mut r: Ptr<i32> = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            rotate_copy_40(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(({ ({ base_16((r).clone(),) }) } == { (ib.as_pointer() as Ptr::<i32>) }));
        r = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_40(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((1) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        r = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_40(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((1) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        r = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_40(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((2) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        r = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_40(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((2) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        r = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_40(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((2) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        r = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_40(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((3) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        r = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_40(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((3) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        r = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_40(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((3) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        r = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_40(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((3) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        r = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_40(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((4) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        r = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_40(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((4) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 0) }));
        r = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_40(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((4) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 1) }));
        r = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_40(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((4) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 2) }));
        r = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_40(___first, ___middle, ___last, (ib.as_pointer() as Ptr<i32>))
        });
        assert!(
            ({ ({ base_16((r).clone(),) }) } == {
                (ib.as_pointer() as Ptr<i32>).offset((4) as isize)
            })
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        {
            let ints: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 2, 5, 6])));
            let mut n_ints: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                as i32);
            let zeros: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
                0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
            ])));
            let mut N: usize = 2_usize;
            let mut middle: Ptr<i32> = ({ begin_25(ints.as_pointer()) }).offset((N) as isize);
            let mut it: Ptr<i32> = ({
                let ___first: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                let ___last: Ptr<i32> = (ints.as_pointer() as Ptr<i32>).to_end();
                let ___result: Ptr<i32> = ({ begin_25(zeros.as_pointer()) });
                rotate_copy_26(___first, (middle).clone(), ___last, ___result)
            });
            assert!(
                ({ ({ distance_27(({ begin_25(zeros.as_pointer(),) }), (it).clone(),) }) } == {
                    (n_ints as isize)
                })
            );
            assert!(
                ({
                    let ___first1: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                    let ___first2: Ptr<i32> = ({ begin_25(zeros.as_pointer()) })
                        .offset((n_ints) as isize)
                        .offset(-((N) as isize));
                    equal_28(___first1, (middle).clone(), ___first2)
                })
            );
            assert!(
                ({
                    equal_28(
                        (middle).clone(),
                        (ints.as_pointer() as Ptr<i32>).to_end(),
                        ({ begin_25(zeros.as_pointer()) }),
                    )
                })
            );
        }
    }
    fn operator_call_1_class_contiguous_iterator_int_ptr_(&self) {
        let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3])));
        let mut sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as u32);
        let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0_i32, 0_i32, 0_i32])));
        let r: Value<contiguous_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                rotate_copy_41(
                    ___first,
                    ___middle,
                    ___last,
                    contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
                )
            }),
        ));
        assert!((({ base_13(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>)));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_41(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_41(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_41(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_41(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_41(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_41(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_41(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_41(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_41(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_41(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_41(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_41(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_41(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_41(
                ___first,
                ___middle,
                ___last,
                contiguous_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_13(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        {
            let ints: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 2, 5, 6])));
            let mut n_ints: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                as i32);
            let zeros: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
                0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
            ])));
            let mut N: usize = 2_usize;
            let mut middle: Ptr<i32> = ({ begin_25(ints.as_pointer()) }).offset((N) as isize);
            let mut it: Ptr<i32> = ({
                let ___first: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                let ___last: Ptr<i32> = (ints.as_pointer() as Ptr<i32>).to_end();
                let ___result: Ptr<i32> = ({ begin_25(zeros.as_pointer()) });
                rotate_copy_26(___first, (middle).clone(), ___last, ___result)
            });
            assert!(
                ({ ({ distance_27(({ begin_25(zeros.as_pointer(),) }), (it).clone(),) }) } == {
                    (n_ints as isize)
                })
            );
            assert!(
                ({
                    let ___first1: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                    let ___first2: Ptr<i32> = ({ begin_25(zeros.as_pointer()) })
                        .offset((n_ints) as isize)
                        .offset(-((N) as isize));
                    equal_28(___first1, (middle).clone(), ___first2)
                })
            );
            assert!(
                ({
                    equal_28(
                        (middle).clone(),
                        (ints.as_pointer() as Ptr<i32>).to_end(),
                        ({ begin_25(zeros.as_pointer()) }),
                    )
                })
            );
        }
    }
    fn operator_call_1_class_random_access_iterator_int_ptr_(&self) {
        let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3])));
        let mut sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as u32);
        let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0_i32, 0_i32, 0_i32])));
        let r: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                rotate_copy_42(
                    ___first,
                    ___middle,
                    ___last,
                    random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
                )
            }),
        ));
        assert!((({ base_11(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>)));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_42(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_42(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_42(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_42(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_42(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_42(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_42(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_42(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_42(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_42(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_42(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_42(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_42(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_42(
                ___first,
                ___middle,
                ___last,
                random_access_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_11(r.as_pointer(),) })
                == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        {
            let ints: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 2, 5, 6])));
            let mut n_ints: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                as i32);
            let zeros: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
                0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
            ])));
            let mut N: usize = 2_usize;
            let mut middle: Ptr<i32> = ({ begin_25(ints.as_pointer()) }).offset((N) as isize);
            let mut it: Ptr<i32> = ({
                let ___first: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                let ___last: Ptr<i32> = (ints.as_pointer() as Ptr<i32>).to_end();
                let ___result: Ptr<i32> = ({ begin_25(zeros.as_pointer()) });
                rotate_copy_26(___first, (middle).clone(), ___last, ___result)
            });
            assert!(
                ({ ({ distance_27(({ begin_25(zeros.as_pointer(),) }), (it).clone(),) }) } == {
                    (n_ints as isize)
                })
            );
            assert!(
                ({
                    let ___first1: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                    let ___first2: Ptr<i32> = ({ begin_25(zeros.as_pointer()) })
                        .offset((n_ints) as isize)
                        .offset(-((N) as isize));
                    equal_28(___first1, (middle).clone(), ___first2)
                })
            );
            assert!(
                ({
                    equal_28(
                        (middle).clone(),
                        (ints.as_pointer() as Ptr<i32>).to_end(),
                        ({ begin_25(zeros.as_pointer()) }),
                    )
                })
            );
        }
    }
    fn operator_call_1_class_bidirectional_iterator_int_ptr_(&self) {
        let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3])));
        let mut sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as u32);
        let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0_i32, 0_i32, 0_i32])));
        let r: Value<bidirectional_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                rotate_copy_43(
                    ___first,
                    ___middle,
                    ___last,
                    bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
                )
            }),
        ));
        assert!((({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>)));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_43(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_43(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_43(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_43(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_43(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_43(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_43(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_43(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_43(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_43(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_43(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_43(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_43(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_43(
                ___first,
                ___middle,
                ___last,
                bidirectional_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_9(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        {
            let ints: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 2, 5, 6])));
            let mut n_ints: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                as i32);
            let zeros: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
                0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
            ])));
            let mut N: usize = 2_usize;
            let mut middle: Ptr<i32> = ({ begin_25(ints.as_pointer()) }).offset((N) as isize);
            let mut it: Ptr<i32> = ({
                let ___first: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                let ___last: Ptr<i32> = (ints.as_pointer() as Ptr<i32>).to_end();
                let ___result: Ptr<i32> = ({ begin_25(zeros.as_pointer()) });
                rotate_copy_26(___first, (middle).clone(), ___last, ___result)
            });
            assert!(
                ({ ({ distance_27(({ begin_25(zeros.as_pointer(),) }), (it).clone(),) }) } == {
                    (n_ints as isize)
                })
            );
            assert!(
                ({
                    let ___first1: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                    let ___first2: Ptr<i32> = ({ begin_25(zeros.as_pointer()) })
                        .offset((n_ints) as isize)
                        .offset(-((N) as isize));
                    equal_28(___first1, (middle).clone(), ___first2)
                })
            );
            assert!(
                ({
                    equal_28(
                        (middle).clone(),
                        (ints.as_pointer() as Ptr<i32>).to_end(),
                        ({ begin_25(zeros.as_pointer()) }),
                    )
                })
            );
        }
    }
    fn operator_call_1_class_forward_iterator_int_ptr_(&self) {
        let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3])));
        let mut sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as u32);
        let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0_i32, 0_i32, 0_i32])));
        let r: Value<forward_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                rotate_copy_44(
                    ___first,
                    ___middle,
                    ___last,
                    forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
                )
            }),
        ));
        assert!((({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>)));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_44(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_44(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_44(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_44(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_44(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_44(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_44(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_44(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_44(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_44(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_44(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_44(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_44(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_44(
                ___first,
                ___middle,
                ___last,
                forward_iterator_int_ptr_::new_1({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_8(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        {
            let ints: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 2, 5, 6])));
            let mut n_ints: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                as i32);
            let zeros: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
                0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
            ])));
            let mut N: usize = 2_usize;
            let mut middle: Ptr<i32> = ({ begin_25(ints.as_pointer()) }).offset((N) as isize);
            let mut it: Ptr<i32> = ({
                let ___first: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                let ___last: Ptr<i32> = (ints.as_pointer() as Ptr<i32>).to_end();
                let ___result: Ptr<i32> = ({ begin_25(zeros.as_pointer()) });
                rotate_copy_26(___first, (middle).clone(), ___last, ___result)
            });
            assert!(
                ({ ({ distance_27(({ begin_25(zeros.as_pointer(),) }), (it).clone(),) }) } == {
                    (n_ints as isize)
                })
            );
            assert!(
                ({
                    let ___first1: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                    let ___first2: Ptr<i32> = ({ begin_25(zeros.as_pointer()) })
                        .offset((n_ints) as isize)
                        .offset(-((N) as isize));
                    equal_28(___first1, (middle).clone(), ___first2)
                })
            );
            assert!(
                ({
                    equal_28(
                        (middle).clone(),
                        (ints.as_pointer() as Ptr<i32>).to_end(),
                        ({ begin_25(zeros.as_pointer()) }),
                    )
                })
            );
        }
    }
    fn operator_call_1_class_cpp17_output_iterator_int_ptr_(&self) {
        let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3])));
        let mut sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as u32);
        let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0_i32, 0_i32, 0_i32])));
        let r: Value<cpp17_output_iterator_int_ptr_> = Rc::new(RefCell::new(
            ({
                let ___first: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___middle: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                let ___last: random_access_iterator_const_int_ptr_ =
                    random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
                rotate_copy_45(
                    ___first,
                    ___middle,
                    ___last,
                    cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
                )
            }),
        ));
        assert!((({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>)));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_45(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            rotate_copy_45(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((1) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_45(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_45(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            rotate_copy_45(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((2) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_45(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_45(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_45(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            rotate_copy_45(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_45(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((1) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_45(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 0) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((2) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_45(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 1) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((3) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_45(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 3) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 2) }));
        (*r.borrow_mut()) = ({
            let ___first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_pointer() as Ptr<i32>) });
            let ___middle: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            let ___last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_pointer() as Ptr<i32>).offset((4) as isize)
                });
            rotate_copy_45(
                ___first,
                ___middle,
                ___last,
                cpp17_output_iterator_int_ptr_::new({ (ib.as_pointer() as Ptr<i32>) }),
            )
        });
        assert!(
            (({ base_7(r.as_pointer(),) }) == (ib.as_pointer() as Ptr::<i32>).offset((4) as isize))
        );
        assert!(({ ((*ib.borrow())[(0) as usize] == 0) }));
        assert!(({ ((*ib.borrow())[(1) as usize] == 1) }));
        assert!(({ ((*ib.borrow())[(2) as usize] == 2) }));
        assert!(({ ((*ib.borrow())[(3) as usize] == 3) }));
        {
            let ints: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5, 2, 5, 6])));
            let mut n_ints: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                as i32);
            let zeros: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
                0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
            ])));
            let mut N: usize = 2_usize;
            let mut middle: Ptr<i32> = ({ begin_25(ints.as_pointer()) }).offset((N) as isize);
            let mut it: Ptr<i32> = ({
                let ___first: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                let ___last: Ptr<i32> = (ints.as_pointer() as Ptr<i32>).to_end();
                let ___result: Ptr<i32> = ({ begin_25(zeros.as_pointer()) });
                rotate_copy_26(___first, (middle).clone(), ___last, ___result)
            });
            assert!(
                ({ ({ distance_27(({ begin_25(zeros.as_pointer(),) }), (it).clone(),) }) } == {
                    (n_ints as isize)
                })
            );
            assert!(
                ({
                    let ___first1: Ptr<i32> = ({ begin_25(ints.as_pointer()) });
                    let ___first2: Ptr<i32> = ({ begin_25(zeros.as_pointer()) })
                        .offset((n_ints) as isize)
                        .offset(-((N) as isize));
                    equal_28(___first1, (middle).clone(), ___first2)
                })
            );
            assert!(
                ({
                    equal_28(
                        (middle).clone(),
                        (ints.as_pointer() as Ptr<i32>).to_end(),
                        ({ begin_25(zeros.as_pointer()) }),
                    )
                })
            );
        }
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
pub trait bidirectional_iterator_const_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_const_int_ptr_>;
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_8(&self, mut _a0: i32) -> bidirectional_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_9(&self, mut _a0: i32) -> bidirectional_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_const_int_ptr_>,
    ) -> Ptr<bidirectional_iterator_const_int_ptr_>;
}
impl bidirectional_iterator_const_int_ptr_Impl for Ptr<bidirectional_iterator_const_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_const_int_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_const_int_ptr_>,
    ) -> Ptr<bidirectional_iterator_const_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait bidirectional_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_int_ptr_>;
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_8(&self, mut _a0: i32) -> bidirectional_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_9(&self, mut _a0: i32) -> bidirectional_iterator_int_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_int_ptr_>,
    ) -> Ptr<bidirectional_iterator_int_ptr_>;
}
impl bidirectional_iterator_int_ptr_Impl for Ptr<bidirectional_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_int_ptr_>,
    ) -> Ptr<bidirectional_iterator_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait contiguous_iterator_const_int_ptr_Impl {
    fn base(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_arrow(&self) -> Ptr<i32>;
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_const_int_ptr_>;
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_11(&self, mut _a0: i32) -> contiguous_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_12(&self, mut _a0: i32) -> contiguous_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: isize) -> Ptr<contiguous_iterator_const_int_ptr_>;
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<contiguous_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_const_int_ptr_>,
    ) -> Ptr<contiguous_iterator_const_int_ptr_>;
}
impl contiguous_iterator_const_int_ptr_Impl for Ptr<contiguous_iterator_const_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_arrow(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_const_int_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<contiguous_iterator_const_int_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_const_int_ptr_>,
    ) -> Ptr<contiguous_iterator_const_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait contiguous_iterator_int_ptr_Impl {
    fn base(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_arrow(&self) -> Ptr<i32>;
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_int_ptr_>;
    fn operator_dec_10(&self) -> Ptr<contiguous_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_11(&self, mut _a0: i32) -> contiguous_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_12(&self, mut _a0: i32) -> contiguous_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: isize) -> Ptr<contiguous_iterator_int_ptr_>;
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<contiguous_iterator_int_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_int_ptr_>,
    ) -> Ptr<contiguous_iterator_int_ptr_>;
}
impl contiguous_iterator_int_ptr_Impl for Ptr<contiguous_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_arrow(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_9(&self) -> Ptr<contiguous_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<contiguous_iterator_int_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<contiguous_iterator_int_ptr_>,
    ) -> Ptr<contiguous_iterator_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
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
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_5(&self) -> Ptr<cpp17_output_iterator_int_ptr_>;
    fn operator_post_inc_6(&self, mut _a0: i32) -> cpp17_output_iterator_int_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<cpp17_output_iterator_int_ptr_>,
    ) -> Ptr<cpp17_output_iterator_int_ptr_>;
}
impl cpp17_output_iterator_int_ptr_Impl for Ptr<cpp17_output_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_output_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
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
pub trait forward_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_6(&self) -> Ptr<forward_iterator_int_ptr_>;
    fn operator_post_inc_7(&self, mut _a0: i32) -> forward_iterator_int_ptr_ {
        unimplemented!()
    }
    fn move_assign(&self, _a0: Ptr<forward_iterator_int_ptr_>) -> Ptr<forward_iterator_int_ptr_>;
}
impl forward_iterator_int_ptr_Impl for Ptr<forward_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<forward_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<forward_iterator_int_ptr_>) -> Ptr<forward_iterator_int_ptr_> {
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
pub trait random_access_iterator_const_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_const_int_ptr_>;
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: i32) -> random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: i32) -> random_access_iterator_const_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<random_access_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<random_access_iterator_const_int_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_const_int_ptr_>,
    ) -> Ptr<random_access_iterator_const_int_ptr_>;
}
impl random_access_iterator_const_int_ptr_Impl for Ptr<random_access_iterator_const_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_const_int_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_const_int_ptr_>,
    ) -> Ptr<random_access_iterator_const_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait random_access_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_int_ptr_>;
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: i32) -> random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: i32) -> random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_int_ptr_>,
    ) -> Ptr<random_access_iterator_int_ptr_>;
}
impl random_access_iterator_int_ptr_Impl for Ptr<random_access_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_int_ptr_>,
    ) -> Ptr<random_access_iterator_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
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
pub fn __cpp2rust_init_globals() {}
