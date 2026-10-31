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
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub fn operator_ne_0(
    x: Ptr<forward_iterator_const_char_ptr_>,
    y: Ptr<forward_iterator_const_char_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct forward_iterator_const_char_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i8>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl forward_iterator_const_char_ptr_ {
    pub fn new_1(mut it: Ptr<i8>) -> Self {
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
impl CppAdvance for forward_iterator_const_char_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_6();
    }
}
pub fn operator_ne_1(
    x: Ptr<forward_iterator_const_wchar_t_ptr_>,
    y: Ptr<forward_iterator_const_wchar_t_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct forward_iterator_const_wchar_t_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl forward_iterator_const_wchar_t_ptr_ {
    pub fn new_1(mut it: Ptr<i32>) -> Self {
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
impl CppAdvance for forward_iterator_const_wchar_t_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_6();
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
pub fn operator_eq_2(
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
        operator_eq_2(__lhs.clone(), __rhs.clone())
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
pub fn operator_add_3(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_4(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_3(_i, n)
    });
}
pub fn iter_swap_5(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap!((a.ptr_).clone(), (b.ptr_).clone());
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_6(iter: adl_Iterator) -> Ptr<i32> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) });
}
pub fn operator_eq_7(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> bool {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_8(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> std::cmp::Ordering {
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
impl CppLess for adl_Iterator {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_8(__lhs.clone(), __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for adl_Iterator {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_7(__lhs.clone(), __rhs.clone())
    }
}
impl CppAdvance for adl_Iterator {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_14();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_16();
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
pub fn test_9(mut A: Ptr<i8>, expected: Ptr<Vec<i8>>) {
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(unimplemented!());
}
pub fn test_10(mut A: Ptr<i32>, expected: Ptr<Vec<i32>>) {
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(unimplemented!());
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
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v = Vec::with_capacity(1_usize as usize + 1);
            Ptr::<i8>::from_string_literal(b"\0")
                .with_slice(1_usize as usize, |__s| __v.extend_from_slice(__s));
            __v.push(0_i8);
            __v
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"NUL"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"\x07").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"alert"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"\x08").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"backspace"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"\t").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"tab"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"\r").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"carriage-return"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"\n").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"newline"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"\x0b").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"vertical-tab"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"\x0c").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"form-feed"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b" ").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"space"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"!").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"exclamation-mark"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"\"").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"quotation-mark"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"#").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"number-sign"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"$").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"dollar-sign"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"%").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"percent-sign"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"&").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"ampersand"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"\'").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"apostrophe"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"(").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"left-parenthesis"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b")").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"right-parenthesis"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"*").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"asterisk"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"+").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"plus-sign"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b",").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"comma"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"-").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"hyphen-minus"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"-").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"hyphen"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b".").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"full-stop"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b".").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"period"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"/").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"slash"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"/").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"solidus"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"0").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"zero"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"1").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"one"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"2").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"two"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"3").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"three"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"4").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"four"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"5").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"five"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"6").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"six"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"7").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"seven"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"8").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"eight"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"9").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"nine"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b":").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"colon"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b";").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"semicolon"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"<").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"less-than-sign"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"=").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"equals-sign"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b">").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"greater-than-sign"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"?").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"question-mark"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"@").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"commercial-at"),
            _expected.as_pointer(),
        )
    });
    let mut c: i8 = ('A' as i8);
    'loop_: while ((c as i32) <= (('Z' as i8) as i32)) {
        let a: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([c, 0_i8])));
        ({
            let _A: Ptr<i8> = (a.as_pointer() as Ptr<i8>);
            let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = (a.as_pointer() as Ptr<i8>).to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            test_9(_A, _expected.as_pointer())
        });
        c.prefix_inc();
    }
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"[").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"left-square-bracket"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"\\").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"backslash"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"\\").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"reverse-solidus"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"]").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"right-square-bracket"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"^").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"circumflex-accent"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"^").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"circumflex"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"_").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"low-line"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"_").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"underscore"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"`").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"grave-accent"),
            _expected.as_pointer(),
        )
    });
    let mut c: i8 = ('a' as i8);
    'loop_: while ((c as i32) <= (('z' as i8) as i32)) {
        let a: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([c, 0_i8])));
        ({
            let _A: Ptr<i8> = (a.as_pointer() as Ptr<i8>);
            let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = (a.as_pointer() as Ptr<i8>).to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            test_9(_A, _expected.as_pointer())
        });
        c.prefix_inc();
    }
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"{").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"left-brace"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"{").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"left-curly-bracket"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"|").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"vertical-line"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"}").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"right-brace"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"}").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"right-curly-bracket"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"~").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"tilde"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"tild"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_9(
            Ptr::<i8>::from_string_literal(b"ch"),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __v = Vec::with_capacity(1_usize as usize + 1);
            Ptr::<i32>::from_string_literal(&[0 as i32, 0 as i32])
                .with_slice(1_usize as usize, |__s| __v.extend_from_slice(__s));
            __v.push(0_i32);
            __v
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[78 as i32, 85 as i32, 76 as i32, 0 as i32]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[7 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 108 as i32, 101 as i32, 114 as i32, 116 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[8 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                98 as i32, 97 as i32, 99 as i32, 107 as i32, 115 as i32, 112 as i32, 97 as i32,
                99 as i32, 101 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[9 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[116 as i32, 97 as i32, 98 as i32, 0 as i32]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[13 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                99 as i32, 97 as i32, 114 as i32, 114 as i32, 105 as i32, 97 as i32, 103 as i32,
                101 as i32, 45 as i32, 114 as i32, 101 as i32, 116 as i32, 117 as i32, 114 as i32,
                110 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[10 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                110 as i32, 101 as i32, 119 as i32, 108 as i32, 105 as i32, 110 as i32, 101 as i32,
                0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[11 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                118 as i32, 101 as i32, 114 as i32, 116 as i32, 105 as i32, 99 as i32, 97 as i32,
                108 as i32, 45 as i32, 116 as i32, 97 as i32, 98 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[12 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                102 as i32, 111 as i32, 114 as i32, 109 as i32, 45 as i32, 102 as i32, 101 as i32,
                101 as i32, 100 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[32 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                115 as i32, 112 as i32, 97 as i32, 99 as i32, 101 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[33 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                101 as i32, 120 as i32, 99 as i32, 108 as i32, 97 as i32, 109 as i32, 97 as i32,
                116 as i32, 105 as i32, 111 as i32, 110 as i32, 45 as i32, 109 as i32, 97 as i32,
                114 as i32, 107 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[34 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                113 as i32, 117 as i32, 111 as i32, 116 as i32, 97 as i32, 116 as i32, 105 as i32,
                111 as i32, 110 as i32, 45 as i32, 109 as i32, 97 as i32, 114 as i32, 107 as i32,
                0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[35 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                110 as i32, 117 as i32, 109 as i32, 98 as i32, 101 as i32, 114 as i32, 45 as i32,
                115 as i32, 105 as i32, 103 as i32, 110 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[36 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                100 as i32, 111 as i32, 108 as i32, 108 as i32, 97 as i32, 114 as i32, 45 as i32,
                115 as i32, 105 as i32, 103 as i32, 110 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[37 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                112 as i32, 101 as i32, 114 as i32, 99 as i32, 101 as i32, 110 as i32, 116 as i32,
                45 as i32, 115 as i32, 105 as i32, 103 as i32, 110 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[38 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 109 as i32, 112 as i32, 101 as i32, 114 as i32, 115 as i32, 97 as i32,
                110 as i32, 100 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[39 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 112 as i32, 111 as i32, 115 as i32, 116 as i32, 114 as i32, 111 as i32,
                112 as i32, 104 as i32, 101 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[40 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                108 as i32, 101 as i32, 102 as i32, 116 as i32, 45 as i32, 112 as i32, 97 as i32,
                114 as i32, 101 as i32, 110 as i32, 116 as i32, 104 as i32, 101 as i32, 115 as i32,
                105 as i32, 115 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[41 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                114 as i32, 105 as i32, 103 as i32, 104 as i32, 116 as i32, 45 as i32, 112 as i32,
                97 as i32, 114 as i32, 101 as i32, 110 as i32, 116 as i32, 104 as i32, 101 as i32,
                115 as i32, 105 as i32, 115 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[42 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 115 as i32, 116 as i32, 101 as i32, 114 as i32, 105 as i32, 115 as i32,
                107 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[43 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                112 as i32, 108 as i32, 117 as i32, 115 as i32, 45 as i32, 115 as i32, 105 as i32,
                103 as i32, 110 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[44 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                99 as i32, 111 as i32, 109 as i32, 109 as i32, 97 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[45 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                104 as i32, 121 as i32, 112 as i32, 104 as i32, 101 as i32, 110 as i32, 45 as i32,
                109 as i32, 105 as i32, 110 as i32, 117 as i32, 115 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[45 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                104 as i32, 121 as i32, 112 as i32, 104 as i32, 101 as i32, 110 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[46 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                102 as i32, 117 as i32, 108 as i32, 108 as i32, 45 as i32, 115 as i32, 116 as i32,
                111 as i32, 112 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[46 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                112 as i32, 101 as i32, 114 as i32, 105 as i32, 111 as i32, 100 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[47 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                115 as i32, 108 as i32, 97 as i32, 115 as i32, 104 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[47 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                115 as i32, 111 as i32, 108 as i32, 105 as i32, 100 as i32, 117 as i32, 115 as i32,
                0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[48 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                122 as i32, 101 as i32, 114 as i32, 111 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[49 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[111 as i32, 110 as i32, 101 as i32, 0 as i32]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[50 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[116 as i32, 119 as i32, 111 as i32, 0 as i32]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[51 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                116 as i32, 104 as i32, 114 as i32, 101 as i32, 101 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[52 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                102 as i32, 111 as i32, 117 as i32, 114 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[53 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                102 as i32, 105 as i32, 118 as i32, 101 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[54 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[115 as i32, 105 as i32, 120 as i32, 0 as i32]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[55 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                115 as i32, 101 as i32, 118 as i32, 101 as i32, 110 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[56 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                101 as i32, 105 as i32, 103 as i32, 104 as i32, 116 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[57 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                110 as i32, 105 as i32, 110 as i32, 101 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[58 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                99 as i32, 111 as i32, 108 as i32, 111 as i32, 110 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[59 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                115 as i32, 101 as i32, 109 as i32, 105 as i32, 99 as i32, 111 as i32, 108 as i32,
                111 as i32, 110 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[60 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                108 as i32, 101 as i32, 115 as i32, 115 as i32, 45 as i32, 116 as i32, 104 as i32,
                97 as i32, 110 as i32, 45 as i32, 115 as i32, 105 as i32, 103 as i32, 110 as i32,
                0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[61 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                101 as i32, 113 as i32, 117 as i32, 97 as i32, 108 as i32, 115 as i32, 45 as i32,
                115 as i32, 105 as i32, 103 as i32, 110 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[62 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                103 as i32, 114 as i32, 101 as i32, 97 as i32, 116 as i32, 101 as i32, 114 as i32,
                45 as i32, 116 as i32, 104 as i32, 97 as i32, 110 as i32, 45 as i32, 115 as i32,
                105 as i32, 103 as i32, 110 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[63 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                113 as i32, 117 as i32, 101 as i32, 115 as i32, 116 as i32, 105 as i32, 111 as i32,
                110 as i32, 45 as i32, 109 as i32, 97 as i32, 114 as i32, 107 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[64 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                99 as i32, 111 as i32, 109 as i32, 109 as i32, 101 as i32, 114 as i32, 99 as i32,
                105 as i32, 97 as i32, 108 as i32, 45 as i32, 97 as i32, 116 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    let mut c: i32 = (65 as i32);
    'loop_: while (c <= (90 as i32)) {
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([c, 0_i32])));
        ({
            let _A: Ptr<i32> = (a.as_pointer() as Ptr<i32>);
            let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
                let mut __bytes = (a.as_pointer() as Ptr<i32>).to_c_bytes();
                __bytes.push(0_i32);
                __bytes
            }));
            test_10(_A, _expected.as_pointer())
        });
        c.prefix_inc();
    }
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[91 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                108 as i32, 101 as i32, 102 as i32, 116 as i32, 45 as i32, 115 as i32, 113 as i32,
                117 as i32, 97 as i32, 114 as i32, 101 as i32, 45 as i32, 98 as i32, 114 as i32,
                97 as i32, 99 as i32, 107 as i32, 101 as i32, 116 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[92 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                98 as i32, 97 as i32, 99 as i32, 107 as i32, 115 as i32, 108 as i32, 97 as i32,
                115 as i32, 104 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[92 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                114 as i32, 101 as i32, 118 as i32, 101 as i32, 114 as i32, 115 as i32, 101 as i32,
                45 as i32, 115 as i32, 111 as i32, 108 as i32, 105 as i32, 100 as i32, 117 as i32,
                115 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[93 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                114 as i32, 105 as i32, 103 as i32, 104 as i32, 116 as i32, 45 as i32, 115 as i32,
                113 as i32, 117 as i32, 97 as i32, 114 as i32, 101 as i32, 45 as i32, 98 as i32,
                114 as i32, 97 as i32, 99 as i32, 107 as i32, 101 as i32, 116 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[94 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                99 as i32, 105 as i32, 114 as i32, 99 as i32, 117 as i32, 109 as i32, 102 as i32,
                108 as i32, 101 as i32, 120 as i32, 45 as i32, 97 as i32, 99 as i32, 99 as i32,
                101 as i32, 110 as i32, 116 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[94 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                99 as i32, 105 as i32, 114 as i32, 99 as i32, 117 as i32, 109 as i32, 102 as i32,
                108 as i32, 101 as i32, 120 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[95 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                108 as i32, 111 as i32, 119 as i32, 45 as i32, 108 as i32, 105 as i32, 110 as i32,
                101 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[95 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                117 as i32, 110 as i32, 100 as i32, 101 as i32, 114 as i32, 115 as i32, 99 as i32,
                111 as i32, 114 as i32, 101 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[96 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                103 as i32, 114 as i32, 97 as i32, 118 as i32, 101 as i32, 45 as i32, 97 as i32,
                99 as i32, 99 as i32, 101 as i32, 110 as i32, 116 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    let mut c: i32 = (97 as i32);
    'loop_: while (c <= (122 as i32)) {
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([c, 0_i32])));
        ({
            let _A: Ptr<i32> = (a.as_pointer() as Ptr<i32>);
            let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
                let mut __bytes = (a.as_pointer() as Ptr<i32>).to_c_bytes();
                __bytes.push(0_i32);
                __bytes
            }));
            test_10(_A, _expected.as_pointer())
        });
        c.prefix_inc();
    }
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[123 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                108 as i32, 101 as i32, 102 as i32, 116 as i32, 45 as i32, 98 as i32, 114 as i32,
                97 as i32, 99 as i32, 101 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[123 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                108 as i32, 101 as i32, 102 as i32, 116 as i32, 45 as i32, 99 as i32, 117 as i32,
                114 as i32, 108 as i32, 121 as i32, 45 as i32, 98 as i32, 114 as i32, 97 as i32,
                99 as i32, 107 as i32, 101 as i32, 116 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[124 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                118 as i32, 101 as i32, 114 as i32, 116 as i32, 105 as i32, 99 as i32, 97 as i32,
                108 as i32, 45 as i32, 108 as i32, 105 as i32, 110 as i32, 101 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[125 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                114 as i32, 105 as i32, 103 as i32, 104 as i32, 116 as i32, 45 as i32, 98 as i32,
                114 as i32, 97 as i32, 99 as i32, 101 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[125 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                114 as i32, 105 as i32, 103 as i32, 104 as i32, 116 as i32, 45 as i32, 99 as i32,
                117 as i32, 114 as i32, 108 as i32, 121 as i32, 45 as i32, 98 as i32, 114 as i32,
                97 as i32, 99 as i32, 107 as i32, 101 as i32, 116 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[126 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                116 as i32, 105 as i32, 108 as i32, 100 as i32, 101 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[
                116 as i32, 105 as i32, 108 as i32, 100 as i32, 0 as i32,
            ]),
            _expected.as_pointer(),
        )
    });
    ({
        let _expected: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_10(
            Ptr::<i32>::from_string_literal(&[99 as i32, 104 as i32, 0 as i32]),
            _expected.as_pointer(),
        )
    });
    return 0;
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
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_post_inc_15(&self, mut _a0: i32) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(*self).upgrade().deref()).clone();
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return prev;
    }
    fn operator_dec_16(&self) -> Ptr<adl_Iterator> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_post_dec_17(&self, mut _a0: i32) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(*self).upgrade().deref()).clone();
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
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
pub trait forward_iterator_const_char_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i8>;
    fn operator_inc_6(&self) -> Ptr<forward_iterator_const_char_ptr_>;
    fn operator_post_inc_7(&self, mut _a0: i32) -> forward_iterator_const_char_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<forward_iterator_const_char_ptr_>,
    ) -> Ptr<forward_iterator_const_char_ptr_>;
}
impl forward_iterator_const_char_ptr_Impl for Ptr<forward_iterator_const_char_ptr_> {
    fn operator_deref(&self) -> Ptr<i8> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<forward_iterator_const_char_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<forward_iterator_const_char_ptr_>,
    ) -> Ptr<forward_iterator_const_char_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait forward_iterator_const_wchar_t_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_6(&self) -> Ptr<forward_iterator_const_wchar_t_ptr_>;
    fn operator_post_inc_7(&self, mut _a0: i32) -> forward_iterator_const_wchar_t_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<forward_iterator_const_wchar_t_ptr_>,
    ) -> Ptr<forward_iterator_const_wchar_t_ptr_>;
}
impl forward_iterator_const_wchar_t_ptr_Impl for Ptr<forward_iterator_const_wchar_t_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<forward_iterator_const_wchar_t_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<forward_iterator_const_wchar_t_ptr_>,
    ) -> Ptr<forward_iterator_const_wchar_t_ptr_> {
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
            if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow());
            }
        }
        return std::cmp::Ordering::Equal;
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
