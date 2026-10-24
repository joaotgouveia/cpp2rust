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
pub fn operator_ne_0(
    x: Ptr<forward_iterator_const_char_ptr_>,
    y: Ptr<forward_iterator_const_char_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(16)]
pub struct forward_iterator_const_char_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl forward_iterator_const_char_ptr_ {
    pub fn new_1(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<forward_iterator_const_char_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
pub fn operator_ne_1(
    x: Ptr<forward_iterator_const_wchar_t_ptr_>,
    y: Ptr<forward_iterator_const_wchar_t_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(16)]
pub struct forward_iterator_const_wchar_t_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl forward_iterator_const_wchar_t_ptr_ {
    pub fn new_1(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<forward_iterator_const_wchar_t_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
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
pub fn operator_eq_2(
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
            operator_eq_2(
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
pub fn operator_add_3(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_4(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_3(_i, n)
    });
}
pub fn iter_swap_5(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_6(iter: adl_Iterator) -> Ptr<()> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return unimplemented!();
}
pub fn operator_eq_7(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_8(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
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
            operator_cmp_8(
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
            operator_eq_7(
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
pub fn test_9(mut A: Ptr<()>, expected: Ptr<()>) {
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_10(mut A: Ptr<()>, expected: Ptr<()>) {
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"NUL"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"alert"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"backspace"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"tab"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"carriage-return"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"newline"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"vertical-tab"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"form-feed"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"space"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"exclamation-mark"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"quotation-mark"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"number-sign"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"dollar-sign"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"percent-sign"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"ampersand"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"apostrophe"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"left-parenthesis"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"right-parenthesis"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"asterisk"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"plus-sign"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"comma"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"hyphen-minus"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"hyphen"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"full-stop"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"period"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"slash"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"solidus"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"zero"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"one"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"two"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"three"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"four"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"five"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"six"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"seven"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"eight"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"nine"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"colon"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"semicolon"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"less-than-sign"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"equals-sign"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"greater-than-sign"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"question-mark"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"commercial-at"),
            _expected.as_pointer(),
        )
    });
    let mut c: () = ('A' as ());
    'loop_: while (c <= ('Z' as ())) {
        let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([c, unimplemented!()])));
        ({
            let _A: Ptr<()> = (a.as_pointer() as Ptr<()>);
            let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_9(_A, _expected.as_pointer())
        });
        c.prefix_inc();
    }
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"left-square-bracket"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"backslash"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"reverse-solidus"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"right-square-bracket"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"circumflex-accent"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"circumflex"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"low-line"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"underscore"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"grave-accent"),
            _expected.as_pointer(),
        )
    });
    let mut c: () = ('a' as ());
    'loop_: while (c <= ('z' as ())) {
        let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([c, unimplemented!()])));
        ({
            let _A: Ptr<()> = (a.as_pointer() as Ptr<()>);
            let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_9(_A, _expected.as_pointer())
        });
        c.prefix_inc();
    }
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"left-brace"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"left-curly-bracket"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"vertical-line"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"right-brace"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"right-curly-bracket"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"tilde"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"tild"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(
            Ptr::<()>::from_string_literal(b"ch"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[78 as (), 85 as (), 76 as (), 0 as ()]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                97 as (), 108 as (), 101 as (), 114 as (), 116 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                98 as (), 97 as (), 99 as (), 107 as (), 115 as (), 112 as (), 97 as (), 99 as (),
                101 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[116 as (), 97 as (), 98 as (), 0 as ()]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                99 as (), 97 as (), 114 as (), 114 as (), 105 as (), 97 as (), 103 as (),
                101 as (), 45 as (), 114 as (), 101 as (), 116 as (), 117 as (), 114 as (),
                110 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                110 as (), 101 as (), 119 as (), 108 as (), 105 as (), 110 as (), 101 as (),
                0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                118 as (), 101 as (), 114 as (), 116 as (), 105 as (), 99 as (), 97 as (),
                108 as (), 45 as (), 116 as (), 97 as (), 98 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                102 as (), 111 as (), 114 as (), 109 as (), 45 as (), 102 as (), 101 as (),
                101 as (), 100 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                115 as (), 112 as (), 97 as (), 99 as (), 101 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                101 as (), 120 as (), 99 as (), 108 as (), 97 as (), 109 as (), 97 as (),
                116 as (), 105 as (), 111 as (), 110 as (), 45 as (), 109 as (), 97 as (),
                114 as (), 107 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                113 as (), 117 as (), 111 as (), 116 as (), 97 as (), 116 as (), 105 as (),
                111 as (), 110 as (), 45 as (), 109 as (), 97 as (), 114 as (), 107 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                110 as (), 117 as (), 109 as (), 98 as (), 101 as (), 114 as (), 45 as (),
                115 as (), 105 as (), 103 as (), 110 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                100 as (), 111 as (), 108 as (), 108 as (), 97 as (), 114 as (), 45 as (),
                115 as (), 105 as (), 103 as (), 110 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                112 as (), 101 as (), 114 as (), 99 as (), 101 as (), 110 as (), 116 as (),
                45 as (), 115 as (), 105 as (), 103 as (), 110 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                97 as (), 109 as (), 112 as (), 101 as (), 114 as (), 115 as (), 97 as (),
                110 as (), 100 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                97 as (), 112 as (), 111 as (), 115 as (), 116 as (), 114 as (), 111 as (),
                112 as (), 104 as (), 101 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                108 as (), 101 as (), 102 as (), 116 as (), 45 as (), 112 as (), 97 as (),
                114 as (), 101 as (), 110 as (), 116 as (), 104 as (), 101 as (), 115 as (),
                105 as (), 115 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                114 as (), 105 as (), 103 as (), 104 as (), 116 as (), 45 as (), 112 as (),
                97 as (), 114 as (), 101 as (), 110 as (), 116 as (), 104 as (), 101 as (),
                115 as (), 105 as (), 115 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                97 as (), 115 as (), 116 as (), 101 as (), 114 as (), 105 as (), 115 as (),
                107 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                112 as (), 108 as (), 117 as (), 115 as (), 45 as (), 115 as (), 105 as (),
                103 as (), 110 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                99 as (), 111 as (), 109 as (), 109 as (), 97 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                104 as (), 121 as (), 112 as (), 104 as (), 101 as (), 110 as (), 45 as (),
                109 as (), 105 as (), 110 as (), 117 as (), 115 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                104 as (), 121 as (), 112 as (), 104 as (), 101 as (), 110 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                102 as (), 117 as (), 108 as (), 108 as (), 45 as (), 115 as (), 116 as (),
                111 as (), 112 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                112 as (), 101 as (), 114 as (), 105 as (), 111 as (), 100 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                115 as (), 108 as (), 97 as (), 115 as (), 104 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                115 as (), 111 as (), 108 as (), 105 as (), 100 as (), 117 as (), 115 as (),
                0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[122 as (), 101 as (), 114 as (), 111 as (), 0 as ()]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[111 as (), 110 as (), 101 as (), 0 as ()]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[116 as (), 119 as (), 111 as (), 0 as ()]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                116 as (), 104 as (), 114 as (), 101 as (), 101 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[102 as (), 111 as (), 117 as (), 114 as (), 0 as ()]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[102 as (), 105 as (), 118 as (), 101 as (), 0 as ()]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[115 as (), 105 as (), 120 as (), 0 as ()]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                115 as (), 101 as (), 118 as (), 101 as (), 110 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                101 as (), 105 as (), 103 as (), 104 as (), 116 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[110 as (), 105 as (), 110 as (), 101 as (), 0 as ()]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                99 as (), 111 as (), 108 as (), 111 as (), 110 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                115 as (), 101 as (), 109 as (), 105 as (), 99 as (), 111 as (), 108 as (),
                111 as (), 110 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                108 as (), 101 as (), 115 as (), 115 as (), 45 as (), 116 as (), 104 as (),
                97 as (), 110 as (), 45 as (), 115 as (), 105 as (), 103 as (), 110 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                101 as (), 113 as (), 117 as (), 97 as (), 108 as (), 115 as (), 45 as (),
                115 as (), 105 as (), 103 as (), 110 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                103 as (), 114 as (), 101 as (), 97 as (), 116 as (), 101 as (), 114 as (),
                45 as (), 116 as (), 104 as (), 97 as (), 110 as (), 45 as (), 115 as (),
                105 as (), 103 as (), 110 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                113 as (), 117 as (), 101 as (), 115 as (), 116 as (), 105 as (), 111 as (),
                110 as (), 45 as (), 109 as (), 97 as (), 114 as (), 107 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                99 as (), 111 as (), 109 as (), 109 as (), 101 as (), 114 as (), 99 as (),
                105 as (), 97 as (), 108 as (), 45 as (), 97 as (), 116 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    let mut c: () = (65 as ());
    'loop_: while (c <= (90 as ())) {
        let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([c, unimplemented!()])));
        ({
            let _A: Ptr<()> = (a.as_pointer() as Ptr<()>);
            let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_10(_A, _expected.as_pointer())
        });
        c.prefix_inc();
    }
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                108 as (), 101 as (), 102 as (), 116 as (), 45 as (), 115 as (), 113 as (),
                117 as (), 97 as (), 114 as (), 101 as (), 45 as (), 98 as (), 114 as (), 97 as (),
                99 as (), 107 as (), 101 as (), 116 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                98 as (), 97 as (), 99 as (), 107 as (), 115 as (), 108 as (), 97 as (), 115 as (),
                104 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                114 as (), 101 as (), 118 as (), 101 as (), 114 as (), 115 as (), 101 as (),
                45 as (), 115 as (), 111 as (), 108 as (), 105 as (), 100 as (), 117 as (),
                115 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                114 as (), 105 as (), 103 as (), 104 as (), 116 as (), 45 as (), 115 as (),
                113 as (), 117 as (), 97 as (), 114 as (), 101 as (), 45 as (), 98 as (),
                114 as (), 97 as (), 99 as (), 107 as (), 101 as (), 116 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                99 as (), 105 as (), 114 as (), 99 as (), 117 as (), 109 as (), 102 as (),
                108 as (), 101 as (), 120 as (), 45 as (), 97 as (), 99 as (), 99 as (), 101 as (),
                110 as (), 116 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                99 as (), 105 as (), 114 as (), 99 as (), 117 as (), 109 as (), 102 as (),
                108 as (), 101 as (), 120 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                108 as (), 111 as (), 119 as (), 45 as (), 108 as (), 105 as (), 110 as (),
                101 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                117 as (), 110 as (), 100 as (), 101 as (), 114 as (), 115 as (), 99 as (),
                111 as (), 114 as (), 101 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                103 as (), 114 as (), 97 as (), 118 as (), 101 as (), 45 as (), 97 as (), 99 as (),
                99 as (), 101 as (), 110 as (), 116 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    let mut c: () = (97 as ());
    'loop_: while (c <= (122 as ())) {
        let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([c, unimplemented!()])));
        ({
            let _A: Ptr<()> = (a.as_pointer() as Ptr<()>);
            let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_10(_A, _expected.as_pointer())
        });
        c.prefix_inc();
    }
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                108 as (), 101 as (), 102 as (), 116 as (), 45 as (), 98 as (), 114 as (),
                97 as (), 99 as (), 101 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                108 as (), 101 as (), 102 as (), 116 as (), 45 as (), 99 as (), 117 as (),
                114 as (), 108 as (), 121 as (), 45 as (), 98 as (), 114 as (), 97 as (), 99 as (),
                107 as (), 101 as (), 116 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                118 as (), 101 as (), 114 as (), 116 as (), 105 as (), 99 as (), 97 as (),
                108 as (), 45 as (), 108 as (), 105 as (), 110 as (), 101 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                114 as (), 105 as (), 103 as (), 104 as (), 116 as (), 45 as (), 98 as (),
                114 as (), 97 as (), 99 as (), 101 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                114 as (), 105 as (), 103 as (), 104 as (), 116 as (), 45 as (), 99 as (),
                117 as (), 114 as (), 108 as (), 121 as (), 45 as (), 98 as (), 114 as (),
                97 as (), 99 as (), 107 as (), 101 as (), 116 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[
                116 as (), 105 as (), 108 as (), 100 as (), 101 as (), 0 as (),
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[116 as (), 105 as (), 108 as (), 100 as (), 0 as ()]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(
            Ptr::<()>::from_string_literal(&[99 as (), 104 as (), 0 as ()]),
            _expected.as_pointer(),
        )
    });
    return 0_();
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
pub trait forward_iterator_const_char_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_inc_6(&self) -> Ptr<forward_iterator_const_char_ptr_>;
    fn operator_post_inc_7(&self, mut _a0: ()) -> forward_iterator_const_char_ptr_ {
        unimplemented!()
    }
}
impl forward_iterator_const_char_ptr_Impl for Ptr<forward_iterator_const_char_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<forward_iterator_const_char_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
}
pub trait forward_iterator_const_wchar_t_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_inc_6(&self) -> Ptr<forward_iterator_const_wchar_t_ptr_>;
    fn operator_post_inc_7(&self, mut _a0: ()) -> forward_iterator_const_wchar_t_ptr_ {
        unimplemented!()
    }
}
impl forward_iterator_const_wchar_t_ptr_Impl for Ptr<forward_iterator_const_wchar_t_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<forward_iterator_const_wchar_t_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
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
