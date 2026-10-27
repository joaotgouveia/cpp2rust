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
    moved_from_: (),
}
impl support_double_move_tracker {
    pub fn new() -> Self {
        Self { moved_from_: false }
    }
    pub fn move_from(other: Ptr<support_double_move_tracker>) -> Self {
        let __this: support_double_move_tracker = Self { moved_from_: false };
        unimplemented!();
        field!(other, moved_from_).write(true);
        __this
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct cpp17_output_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_output_iterator_int_ptr_ {}
const _: () = assert!(
    true,
    "std::output_iterator<cpp17_output_iterator<int*>, int>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct cpp20_random_access_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp20_random_access_iterator_int_ptr_ {}
const _: () = assert!(
    true,
    "std::random_access_iterator<cpp20_random_access_iterator<int*>>"
);
pub fn base_0(i: Ptr<()>) -> Ptr<()> {
    let i: Value<Ptr<()>> = Rc::new(RefCell::new(i));
    return (*i.borrow()).clone();
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp20_input_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
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
const _: () = assert!(true, "std::input_iterator<cpp20_input_iterator<int*>>");
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp20_output_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
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
const _: () = assert!(
    true,
    "std::output_iterator<cpp20_output_iterator<int*>, int>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct IteratorOpCounts {
    #[offset(0)]
    pub increments: (),
    #[offset(8)]
    pub decrements: (),
    #[offset(16)]
    pub zero_moves: (),
    #[offset(24)]
    pub equal_cmps: (),
}
impl Default for IteratorOpCounts {
    fn default() -> Self {
        {
            IteratorOpCounts {
                increments: 0_(),
                decrements: 0_(),
                zero_moves: 0_(),
                equal_cmps: 0_(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_int_ptr_ {
    pub fn new_1(it: Ptr<Ptr<()>>) -> Self {
        Self {
            base_: ({ base_0((it.read())) }),
        }
    }
}
pub fn operator_eq_1(
    _a0: Ptr<forward_sized_iterator_int_ptr_>,
    _a1: Ptr<forward_sized_iterator_int_ptr_>,
) -> () {
    return ({ _a0.with(|__s| __s.it_.clone()) } == { _a1.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct forward_sized_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    pub it_: Ptr<()>,
}
impl forward_sized_iterator_int_ptr_ {}
impl std::cmp::PartialEq for forward_sized_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_1(
                Rc::new(RefCell::new(forward_sized_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(forward_sized_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for forward_sized_iterator_int_ptr_ {}
impl Default for forward_sized_iterator_int_ptr_ {
    fn default() -> Self {
        {
            forward_sized_iterator_int_ptr_ {
                it_: Ptr::<()>::null(),
            }
        }
    }
}
const _: () = assert!(true, "std::forward_iterator<forward_sized_iterator<>>");
const _: () = assert!(
    true,
    "std::sized_sentinel_for<forward_sized_iterator<>, forward_sized_iterator<>>"
);
pub fn operator_add_2(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_3(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_2(_i, n)
    });
}
pub fn iter_swap_4(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_5(iter: adl_Iterator) -> Ptr<()> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) });
}
pub fn operator_eq_6(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_7(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
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
    ptr_: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    iter_moves_: Ptr<()>,
    #[offset(16)]
    #[byte_size(8)]
    iter_swaps_: Ptr<()>,
}
impl adl_Iterator {
    fn new_1(mut p: Ptr<()>, mut iter_moves: Ptr<()>, mut iter_swaps: Ptr<()>) -> Self {
        Self {
            ptr_: (p).clone(),
            iter_moves_: (iter_moves).clone(),
            iter_swaps_: (iter_swaps).clone(),
        }
    }
    pub fn TrackMoves(mut p: Ptr<()>, iter_moves: Ptr<()>) -> adl_Iterator {
        return adl_Iterator::new_1({ (p).clone() }, { (iter_moves).clone() }, {
            Ptr::<()>::null()
        });
    }
    pub fn TrackSwaps_4(iter_swaps: Ptr<()>) -> adl_Iterator {
        return adl_Iterator::new_1({ Ptr::<()>::null() }, { Ptr::<()>::null() }, {
            (iter_swaps).clone()
        });
    }
    pub fn TrackSwaps_5(mut p: Ptr<()>, iter_swaps: Ptr<()>) -> adl_Iterator {
        return adl_Iterator::new_1({ (p).clone() }, { Ptr::<()>::null() }, {
            (iter_swaps).clone()
        });
    }
}
impl std::cmp::Ord for adl_Iterator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_7(
                Rc::new(RefCell::new(adl_Iterator {
                    ptr_: self.ptr_.clone(),
                    iter_moves_: self.iter_moves_.clone(),
                    iter_swaps_: self.iter_swaps_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(adl_Iterator {
                    ptr_: other.ptr_.clone(),
                    iter_moves_: other.iter_moves_.clone(),
                    iter_swaps_: other.iter_swaps_.clone(),
                }))
                .as_pointer(),
            )
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
            operator_eq_6(
                Rc::new(RefCell::new(adl_Iterator {
                    ptr_: self.ptr_.clone(),
                    iter_moves_: self.iter_moves_.clone(),
                    iter_swaps_: self.iter_swaps_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(adl_Iterator {
                    ptr_: other.ptr_.clone(),
                    iter_moves_: other.iter_moves_.clone(),
                    iter_swaps_: other.iter_swaps_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for adl_Iterator {}
impl Default for adl_Iterator {
    fn default() -> Self {
        {
            adl_Iterator {
                ptr_: Ptr::<()>::null(),
                iter_moves_: Ptr::<()>::null(),
                iter_swaps_: Ptr::<()>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct rvalue_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<Ptr<()>>,
}
impl rvalue_iterator_int_ptr_ {}
impl std::cmp::Ord for rvalue_iterator_int_ptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            rvalue_iterator_int_ptr_Impl::operator_cmp(
                &Rc::new(RefCell::new(rvalue_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(rvalue_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for rvalue_iterator_int_ptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for rvalue_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            rvalue_iterator_int_ptr_Impl::operator_cmp(
                &Rc::new(RefCell::new(rvalue_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(rvalue_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                }))
                .as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for rvalue_iterator_int_ptr_ {}
const _: () = assert!(true, "std::random_access_iterator<rvalue_iterator<int*>>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct util_iterator_error {}
impl util_iterator_error {
    pub fn new(mut what: Ptr<()>) -> Self {
        Self {}
    }
}
impl NonBorrowedRange {
    pub fn new(arr: Ptr<()>) -> Self {
        Self {
            data_: unimplemented!(),
            size_: unimplemented!(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct NonBorrowedRange {
    #[offset(0)]
    #[byte_size(8)]
    pub data_: Ptr<()>,
    #[offset(8)]
    pub size_: (),
}
pub fn rand_gen_8() -> () {
    return unimplemented!();
}
pub fn test_all_9() -> () {
    let unary_pred: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
        |mut i: ()| -> () {
            {
                return (i > 0_());
            }
        },
    )));
    let binary_pred: Value<FnPtr<fn((), ()) -> ()>> =
        Rc::new(RefCell::new(FnPtr::<fn((), ()) -> ()>::new(
            |mut i: (), mut j: ()| -> () {
                {
                    return (i < j);
                }
            },
        )));
    let gen_: Value<FnPtr<fn() -> ()>> =
        Rc::new(RefCell::new(FnPtr::<fn() -> ()>::new(|| -> () {
            {
                return 42_();
            }
        })));
    let in_: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let in2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mid: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!().offset((1_()) as isize)));
    let output: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let out2: Value<()> = Rc::new(RefCell::new(unimplemented!().offset((1_()) as isize)));
    let x: Value<()> = Rc::new(RefCell::new(2_()));
    let count: Value<()> = Rc::new(RefCell::new(1_()));
    ({
        dangling_1st_10(
            find_11.with(|v| v.as_pointer()),
            in_.as_pointer(),
            x.as_pointer(),
        )
    });
    ({
        dangling_1st_12(
            find_if_13.with(|v| v.as_pointer()),
            in_.as_pointer(),
            unary_pred.as_pointer(),
        )
    });
    ({
        dangling_1st_14(
            find_if_not_15.with(|v| v.as_pointer()),
            in_.as_pointer(),
            unary_pred.as_pointer(),
        )
    });
    ({
        dangling_1st_16(
            find_last_17.with(|v| v.as_pointer()),
            in_.as_pointer(),
            x.as_pointer(),
        )
    });
    ({
        dangling_1st_18(
            find_last_if_19.with(|v| v.as_pointer()),
            in_.as_pointer(),
            unary_pred.as_pointer(),
        )
    });
    ({
        dangling_1st_20(
            find_last_if_not_21.with(|v| v.as_pointer()),
            in_.as_pointer(),
            unary_pred.as_pointer(),
        )
    });
    ({
        dangling_1st_22(
            find_first_of_23.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
        )
    });
    ({ dangling_1st_24(adjacent_find_25.with(|v| v.as_pointer()), in_.as_pointer()) });
    ({
        dangling_1st_26(
            mismatch_27.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
        )
    });
    ({
        dangling_2nd_28(
            mismatch_27.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
        )
    });
    ({
        dangling_both_29(
            mismatch_27.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
        )
    });
    ({
        dangling_1st_30(
            partition_point_31.with(|v| v.as_pointer()),
            in_.as_pointer(),
            unary_pred.as_pointer(),
        )
    });
    ({
        dangling_1st_32(
            lower_bound_33.with(|v| v.as_pointer()),
            in_.as_pointer(),
            x.as_pointer(),
        )
    });
    ({
        dangling_1st_34(
            upper_bound_35.with(|v| v.as_pointer()),
            in_.as_pointer(),
            x.as_pointer(),
        )
    });
    ({
        dangling_1st_36(
            equal_range_37.with(|v| v.as_pointer()),
            in_.as_pointer(),
            x.as_pointer(),
        )
    });
    ({ dangling_1st_38(min_element_39.with(|v| v.as_pointer()), in_.as_pointer()) });
    ({ dangling_1st_40(max_element_41.with(|v| v.as_pointer()), in_.as_pointer()) });
    ({ dangling_1st_42(minmax_element_43.with(|v| v.as_pointer()), in_.as_pointer()) });
    ({
        dangling_1st_44(
            search_45.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
        )
    });
    ({
        dangling_1st_46(
            search_n_47.with(|v| v.as_pointer()),
            in_.as_pointer(),
            count.as_pointer(),
            x.as_pointer(),
        )
    });
    ({
        dangling_1st_48(
            find_end_49.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
        )
    });
    ({
        dangling_1st_50(
            is_sorted_until_51.with(|v| v.as_pointer()),
            in_.as_pointer(),
        )
    });
    ({ dangling_1st_52(is_heap_until_53.with(|v| v.as_pointer()), in_.as_pointer()) });
    ({
        dangling_1st_54(
            for_each_55.with(|v| v.as_pointer()),
            in_.as_pointer(),
            unary_pred.as_pointer(),
        )
    });
    ({
        dangling_1st_56(
            copy_57.with(|v| v.as_pointer()),
            in_.as_pointer(),
            out.as_pointer(),
        )
    });
    ({
        let _args: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
        dangling_1st_58(
            copy_backward_59.with(|v| v.as_pointer()),
            in_.as_pointer(),
            _args.as_pointer(),
        )
    });
    ({
        dangling_1st_60(
            copy_if_61.with(|v| v.as_pointer()),
            in_.as_pointer(),
            out.as_pointer(),
            unary_pred.as_pointer(),
        )
    });
    ({
        dangling_1st_62(
            move_63.with(|v| v.as_pointer()),
            in_.as_pointer(),
            out.as_pointer(),
        )
    });
    ({
        let _args: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
        dangling_1st_64(
            move_backward_65.with(|v| v.as_pointer()),
            in_.as_pointer(),
            _args.as_pointer(),
        )
    });
    ({
        dangling_1st_66(
            fill_67.with(|v| v.as_pointer()),
            in_.as_pointer(),
            x.as_pointer(),
        )
    });
    {
        let out_transform: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let _args_2: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
            dangling_1st_68(
                transform_69.with(|v| v.as_pointer()),
                in_.as_pointer(),
                _args_2.as_pointer(),
                unary_pred.as_pointer(),
            )
        });
        ({
            let _args_3: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
            dangling_1st_70(
                transform_69.with(|v| v.as_pointer()),
                in_.as_pointer(),
                in2.as_pointer(),
                _args_3.as_pointer(),
                binary_pred.as_pointer(),
            )
        });
        ({
            let _args_3: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
            dangling_2nd_71(
                transform_69.with(|v| v.as_pointer()),
                in_.as_pointer(),
                in2.as_pointer(),
                _args_3.as_pointer(),
                binary_pred.as_pointer(),
            )
        });
        ({
            let _args_3: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
            dangling_both_72(
                transform_69.with(|v| v.as_pointer()),
                in_.as_pointer(),
                in2.as_pointer(),
                _args_3.as_pointer(),
                binary_pred.as_pointer(),
            )
        });
    }
    ({
        dangling_1st_73(
            generate_74.with(|v| v.as_pointer()),
            in_.as_pointer(),
            gen_.as_pointer(),
        )
    });
    ({
        let _args_2: Ptr<Ptr<()>> = out.as_pointer();
        let _args_3: Ptr<()> = x.as_pointer();
        dangling_1st_75(
            remove_copy_76.with(|v| v.as_pointer()),
            in_.as_pointer(),
            _args_2,
            _args_3,
        )
    });
    ({
        dangling_1st_77(
            remove_copy_if_78.with(|v| v.as_pointer()),
            in_.as_pointer(),
            out.as_pointer(),
            unary_pred.as_pointer(),
        )
    });
    ({
        let _args_2: Ptr<()> = x.as_pointer();
        let _args_3: Ptr<()> = x.as_pointer();
        dangling_1st_79(
            replace_80.with(|v| v.as_pointer()),
            in_.as_pointer(),
            _args_2,
            _args_3,
        )
    });
    ({
        let _args_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        dangling_1st_81(
            replace_if_82.with(|v| v.as_pointer()),
            in_.as_pointer(),
            _args_2.as_pointer(),
            x.as_pointer(),
        )
    });
    ({
        let _args_2: Ptr<Ptr<()>> = out.as_pointer();
        let _args_3: Ptr<()> = x.as_pointer();
        let _args_4: Ptr<()> = x.as_pointer();
        dangling_1st_83(
            replace_copy_84.with(|v| v.as_pointer()),
            in_.as_pointer(),
            _args_2,
            _args_3,
            _args_4,
        )
    });
    ({
        let _args_2: Ptr<Ptr<()>> = out.as_pointer();
        let _args_4: Ptr<()> = x.as_pointer();
        dangling_1st_85(
            replace_copy_if_86.with(|v| v.as_pointer()),
            in_.as_pointer(),
            _args_2,
            unary_pred.as_pointer(),
            _args_4,
        )
    });
    ({
        dangling_1st_87(
            swap_ranges_88.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
        )
    });
    ({
        dangling_2nd_89(
            swap_ranges_88.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
        )
    });
    ({
        dangling_both_90(
            swap_ranges_88.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
        )
    });
    ({
        dangling_1st_91(
            reverse_copy_92.with(|v| v.as_pointer()),
            in_.as_pointer(),
            out.as_pointer(),
        )
    });
    ({
        dangling_1st_93(
            rotate_copy_94.with(|v| v.as_pointer()),
            in_.as_pointer(),
            mid.as_pointer(),
            out.as_pointer(),
        )
    });
    ({
        dangling_1st_95(
            unique_copy_96.with(|v| v.as_pointer()),
            in_.as_pointer(),
            out.as_pointer(),
        )
    });
    ({
        dangling_1st_97(
            partition_copy_98.with(|v| v.as_pointer()),
            in_.as_pointer(),
            out.as_pointer(),
            out2.as_pointer(),
            unary_pred.as_pointer(),
        )
    });
    ({
        dangling_1st_99(
            partial_sort_copy_100.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
        )
    });
    ({
        dangling_2nd_101(
            partial_sort_copy_100.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
        )
    });
    ({
        dangling_both_102(
            partial_sort_copy_100.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
        )
    });
    ({
        dangling_1st_103(
            merge_104.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            out.as_pointer(),
        )
    });
    ({
        dangling_2nd_105(
            merge_104.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            out.as_pointer(),
        )
    });
    ({
        dangling_both_106(
            merge_104.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            out.as_pointer(),
        )
    });
    ({
        dangling_1st_107(
            set_difference_108.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            out.as_pointer(),
        )
    });
    ({
        dangling_1st_109(
            set_intersection_110.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            out.as_pointer(),
        )
    });
    ({
        dangling_2nd_111(
            set_intersection_110.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            out.as_pointer(),
        )
    });
    ({
        dangling_both_112(
            set_intersection_110.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            out.as_pointer(),
        )
    });
    ({
        dangling_1st_113(
            set_symmetric_difference_114.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            out.as_pointer(),
        )
    });
    ({
        dangling_2nd_115(
            set_symmetric_difference_114.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            out.as_pointer(),
        )
    });
    ({
        dangling_both_116(
            set_symmetric_difference_114.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            out.as_pointer(),
        )
    });
    ({
        dangling_1st_117(
            set_union_118.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            out.as_pointer(),
        )
    });
    ({
        dangling_2nd_119(
            set_union_118.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            out.as_pointer(),
        )
    });
    ({
        dangling_both_120(
            set_union_118.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            out.as_pointer(),
        )
    });
    ({
        dangling_1st_121(
            remove_122.with(|v| v.as_pointer()),
            in_.as_pointer(),
            x.as_pointer(),
        )
    });
    ({
        dangling_1st_123(
            remove_if_124.with(|v| v.as_pointer()),
            in_.as_pointer(),
            unary_pred.as_pointer(),
        )
    });
    ({ dangling_1st_125(reverse_126.with(|v| v.as_pointer()), in_.as_pointer()) });
    ({
        dangling_1st_127(
            rotate_128.with(|v| v.as_pointer()),
            in_.as_pointer(),
            mid.as_pointer(),
        )
    });
    {
        ({
            let _args: Value<()> = Rc::new(RefCell::new(({ rand_gen_8() })));
            dangling_1st_129(
                shuffle_130.with(|v| v.as_pointer()),
                in_.as_pointer(),
                _args.as_pointer(),
            )
        });
    }
    ({ dangling_1st_131(unique_132.with(|v| v.as_pointer()), in_.as_pointer()) });
    ({
        dangling_1st_133(
            partition_134.with(|v| v.as_pointer()),
            in_.as_pointer(),
            unary_pred.as_pointer(),
        )
    });
    {
        ({
            dangling_1st_135(
                stable_partition_136.with(|v| v.as_pointer()),
                in_.as_pointer(),
                unary_pred.as_pointer(),
            )
        });
    }
    ({ dangling_1st_137(sort_138.with(|v| v.as_pointer()), in_.as_pointer()) });
    {
        ({ dangling_1st_139(stable_sort_140.with(|v| v.as_pointer()), in_.as_pointer()) });
    }
    ({
        dangling_1st_141(
            partial_sort_142.with(|v| v.as_pointer()),
            in_.as_pointer(),
            mid.as_pointer(),
        )
    });
    ({
        dangling_1st_143(
            nth_element_144.with(|v| v.as_pointer()),
            in_.as_pointer(),
            mid.as_pointer(),
        )
    });
    {
        ({
            dangling_1st_145(
                inplace_merge_146.with(|v| v.as_pointer()),
                in_.as_pointer(),
                mid.as_pointer(),
            )
        });
    }
    ({ dangling_1st_147(make_heap_148.with(|v| v.as_pointer()), in_.as_pointer()) });
    ({ dangling_1st_149(push_heap_150.with(|v| v.as_pointer()), in_.as_pointer()) });
    ({ dangling_1st_151(pop_heap_152.with(|v| v.as_pointer()), in_.as_pointer()) });
    ({ dangling_1st_147(make_heap_148.with(|v| v.as_pointer()), in_.as_pointer()) });
    ({ dangling_1st_153(sort_heap_154.with(|v| v.as_pointer()), in_.as_pointer()) });
    ({
        dangling_1st_155(
            prev_permutation_156.with(|v| v.as_pointer()),
            in_.as_pointer(),
        )
    });
    ({
        dangling_1st_157(
            next_permutation_158.with(|v| v.as_pointer()),
            in_.as_pointer(),
        )
    });
    ({
        dangling_1st_159(
            iota_160.with(|v| v.as_pointer()),
            in_.as_pointer(),
            x.as_pointer(),
        )
    });
    ({
        dangling_1st_161(
            shift_left_162.with(|v| v.as_pointer()),
            in_.as_pointer(),
            x.as_pointer(),
        )
    });
    ({
        dangling_1st_163(
            shift_right_164.with(|v| v.as_pointer()),
            in_.as_pointer(),
            x.as_pointer(),
        )
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_all_9() });
    const _: () = assert!(true, "test_all()");;
    return 0_();
}
pub trait NonBorrowedRangeImpl {
    fn begin(&self) -> Ptr<()>;
    fn end(&self) -> sentinel_wrapper_int_ptr_;
}
impl NonBorrowedRangeImpl for Ptr<NonBorrowedRange> {
    fn begin(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.data_.clone());
    }
    fn end(&self) -> sentinel_wrapper_int_ptr_ {
        return {
            let __tmp_0: Value<Ptr<()>> = Rc::new(RefCell::new(
                (*self)
                    .with(|__s| __s.data_.clone())
                    .offset(((*self).with(|__s| __s.size_)) as isize),
            ));
            sentinel_wrapper_int_ptr_::new_1({ __tmp_0.as_pointer() })
        };
    }
}
pub trait adl_IteratorImpl {
    fn iter_moves(&self) -> ();
    fn iter_swaps(&self) -> ();
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_index(&self, n: ()) -> Ptr<()>;
    fn operator_sub_10(&self, n: ()) -> adl_Iterator;
    fn operator_sub_11(&self, rhs: adl_Iterator) -> ();
    fn operator_add_assign(&self, n: ()) -> Ptr<adl_Iterator>;
    fn operator_sub_assign(&self, n: ()) -> Ptr<adl_Iterator>;
    fn operator_inc_14(&self) -> Ptr<adl_Iterator>;
    fn operator_post_inc_15(&self, _a0: ()) -> adl_Iterator;
    fn operator_dec_16(&self) -> Ptr<adl_Iterator>;
    fn operator_post_dec_17(&self, _a0: ()) -> adl_Iterator;
}
impl adl_IteratorImpl for Ptr<adl_Iterator> {
    fn iter_moves(&self) -> () {
        unimplemented!();
        return ((*self).with(|__s| __s.iter_moves_.clone()).read());
    }
    fn iter_swaps(&self) -> () {
        unimplemented!();
        return ((*self).with(|__s| __s.iter_swaps_.clone()).read());
    }
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone()).offset((n) as isize);
    }
    fn operator_sub_10(&self, mut n: ()) -> adl_Iterator {
        return adl_Iterator::new_1(
            { (*self).with(|__s| __s.ptr_.clone()).offset(-((n) as isize)) },
            { (*self).with(|__s| __s.iter_moves_.clone()) },
            { (*self).with(|__s| __s.iter_swaps_.clone()) },
        );
    }
    fn operator_sub_11(&self, mut rhs: adl_Iterator) -> () {
        return (((*self).with(|__s| __s.ptr_.clone()) - (rhs.ptr_).clone()) as () as ());
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<adl_Iterator> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<adl_Iterator> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn operator_inc_14(&self) -> Ptr<adl_Iterator> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_post_inc_15(&self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(*self).upgrade().deref()).clone();
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return prev;
    }
    fn operator_dec_16(&self) -> Ptr<adl_Iterator> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_post_dec_17(&self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(*self).upgrade().deref()).clone();
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return prev;
    }
}
pub trait cpp17_input_iterator_int_ptr__int_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp17_input_iterator_int_ptr__int_ptr_ {
        unimplemented!()
    }
}
impl cpp17_input_iterator_int_ptr__int_ptr_Impl for Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {}
pub trait cpp17_output_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_output_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp17_output_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl cpp17_output_iterator_int_ptr_Impl for Ptr<cpp17_output_iterator_int_ptr_> {}
pub trait cpp20_input_iterator_int_ptr_Impl {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_input_iterator_int_ptr_>,
    ) -> Ptr<cpp20_input_iterator_int_ptr_>;
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_inc_5(&self) -> Ptr<cpp20_input_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) {
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
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone());
    }
}
pub trait cpp20_output_iterator_int_ptr_Impl {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_output_iterator_int_ptr_>,
    ) -> Ptr<cpp20_output_iterator_int_ptr_>;
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_inc_5(&self) -> Ptr<cpp20_output_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp20_output_iterator_int_ptr_ {
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
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone());
    }
}
pub trait cpp20_random_access_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_index(&self, n: ()) -> Ptr<()>;
    fn operator_inc_7(&self) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_dec_8(&self) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: ()) -> cpp20_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: ()) -> cpp20_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
}
impl cpp20_random_access_iterator_int_ptr_Impl for Ptr<cpp20_random_access_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone()).offset((n) as isize);
    }
}
pub trait forward_sized_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_4(&self) -> Ptr<forward_sized_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_5(&self, mut _a0: ()) -> forward_sized_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl forward_sized_iterator_int_ptr_Impl for Ptr<forward_sized_iterator_int_ptr_> {}
pub trait rvalue_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn operator_inc_4(&self) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_5(&self, mut _a0: ()) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_dec_6(&self) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_dec_7(&self, mut _a0: ()) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add(&self, mut n: ()) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_sub_9(&self, mut n: ()) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_sub_10(&self, other: Ptr<rvalue_iterator_int_ptr_>) -> () {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn operator_cmp(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> ();
    fn operator_eq(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> ();
}
impl rvalue_iterator_int_ptr_Impl for Ptr<rvalue_iterator_int_ptr_> {
    fn operator_cmp(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> () {
        {
            let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.it_.clone())),
                &(_a0.with(|__s| __s.it_.clone())),
            )));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    fn operator_eq(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> () {
        return ({ (*self).with(|__s| __s.it_.clone()) } == { _a0.with(|__s| __s.it_.clone()) });
    }
}
pub trait sentinel_wrapper_int_ptr_Impl {
    fn operator_eq(&self, other: Ptr<Ptr<()>>) -> ();
}
impl sentinel_wrapper_int_ptr_Impl for Ptr<sentinel_wrapper_int_ptr_> {
    fn operator_eq(&self, other: Ptr<Ptr<()>>) -> () {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_0((other.read())) }) });
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
        unimplemented!();
        field!(other, moved_from_).write(true);
        field!((*self), moved_from_).write(false);
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
