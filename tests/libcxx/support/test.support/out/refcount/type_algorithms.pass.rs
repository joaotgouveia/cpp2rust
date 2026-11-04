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
pub struct types_type_list_NumT_0___NumT_1___NumT_2__ {}
impl CppMove for types_type_list_NumT_0___NumT_1___NumT_2__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list___ {}
impl CppMove for types_concatenate_types_type_list___ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_int__ {}
impl CppMove for types_concatenate_types_type_list_int__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_int___types_type_list_long__ {}
impl CppMove for types_concatenate_types_type_list_int___types_type_list_long__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_int___types_type_list_long___types_type_list_long_long__
{}
impl CppMove
    for types_concatenate_types_type_list_int___types_type_list_long___types_type_list_long_long__
{
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_int__long___types_type_list_long_long__ {}
impl CppMove for types_concatenate_types_type_list_int__long___types_type_list_long_long__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn for_each_0(mut _a0: types_type_list_NumT_0___NumT_1___NumT_2__, f: ApplyAllTest) {
    let f: Value<ApplyAllTest> = Rc::new(RefCell::new(f));
    ({
        let _arg0: i32 = {
            ({ ApplyAllTestImpl::operator_call_4_class_NumT_0_(&f.as_pointer()) });
            0
        };
        let _arg1: i32 = {
            ({ ApplyAllTestImpl::operator_call_4_class_NumT_1_(&f.as_pointer()) });
            0
        };
        let _arg2: i32 = {
            ({ ApplyAllTestImpl::operator_call_4_class_NumT_2_(&f.as_pointer()) });
            0
        };
        swallow_1(_arg0, _arg1, _arg2)
    });
}
pub fn swallow_1(mut _a0: i32, mut _a1: i32, mut _a2: i32) {}
const _: () = assert!(
    true,
    "std::is_same<types::concatenate_t<types::type_list<>>,\n                           types::type_list<>>::value"
);
const _: () = assert!(
    true,
    "std::is_same<types::concatenate_t<types::type_list<int>>,\n                           types::type_list<int>>::value"
);
const _: () = assert!(
    true,
    "std::is_same<types::concatenate_t<types::type_list<int>,\n                                                types::type_list<long>>,\n                           types::type_list<int, long>>::value"
);
const _: () = assert!(
    true,
    "std::is_same<types::concatenate_t<types::type_list<int>,\n                                                types::type_list<long>,\n                                                types::type_list<long long>>,\n                           types::type_list<int, long, long long>>::value"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NumT_0_ {}
impl CppMove for NumT_0_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NumT_1_ {}
impl CppMove for NumT_1_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NumT_2_ {}
impl CppMove for NumT_2_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Ptr< bool  >  , new, value)]
#[byte_size(8)]
pub struct ApplyAllTest {
    #[offset(0)]
    #[byte_size(8)]
    pub is_called_array_: Ptr<bool>,
}
impl ApplyAllTest {
    pub fn new(mut is_called_array: Ptr<bool>) -> Self {
        Self {
            is_called_array_: (is_called_array).clone(),
        }
    }
}
impl Callable0<()> for ApplyAllTest {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            ApplyAllTestImpl::operator_call_4_class_NumT_0_(__it)
        })
    }
}
impl CppMove for ApplyAllTest {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Identity {}
impl Callable1<bool, bool> for Identity {
    fn call(&self, a1: bool) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            IdentityImpl::operator_call(__it, a1)
        })
    }
}
impl CppMove for Identity {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_for_each_2() {
    let is_called_array: Value<Box<[bool]>> =
        Rc::new(RefCell::new(Box::new([false, false, false])));
    ({
        for_each_0(
            <types_type_list_NumT_0___NumT_1___NumT_2__>::default(),
            ApplyAllTest::new({ (is_called_array.as_pointer() as Ptr<bool>) }),
        )
    });
    assert!(
        ({
            let ___first: Ptr<bool> = (is_called_array.as_pointer() as Ptr<bool>);
            let ___last: Ptr<bool> =
                (is_called_array.as_pointer() as Ptr<bool>).offset((3) as isize);
            all_of_3(___first, ___last, <Identity>::default())
        })
    );
}
pub fn test_4() -> bool {
    ({ test_for_each_2() });
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
    ({ test_4() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub trait ApplyAllTestImpl {
    fn check_num_2_0(&self, _a0: NumT_0_);
    fn check_num_2_1(&self, _a0: NumT_1_);
    fn check_num_2_2(&self, _a0: NumT_2_);
    fn operator_call_4_class_NumT_0_(&self);
    fn operator_call_4_class_NumT_1_(&self);
    fn operator_call_4_class_NumT_2_(&self);
}
impl ApplyAllTestImpl for Ptr<ApplyAllTest> {
    fn check_num_2_0(&self, mut _a0: NumT_0_) {
        assert!(!(elem!((*self).with(|__s| __s.is_called_array_.clone()), 0).read()));
        elem!((*self).with(|__s| __s.is_called_array_.clone()), 0).write(true);
    }
    fn check_num_2_1(&self, mut _a0: NumT_1_) {
        assert!(!(elem!((*self).with(|__s| __s.is_called_array_.clone()), 1).read()));
        elem!((*self).with(|__s| __s.is_called_array_.clone()), 1).write(true);
    }
    fn check_num_2_2(&self, mut _a0: NumT_2_) {
        assert!(!(elem!((*self).with(|__s| __s.is_called_array_.clone()), 2).read()));
        elem!((*self).with(|__s| __s.is_called_array_.clone()), 2).write(true);
    }
    fn operator_call_4_class_NumT_0_(&self) {
        ({ ApplyAllTestImpl::check_num_2_0(self, <NumT_0_>::default()) });
    }
    fn operator_call_4_class_NumT_1_(&self) {
        ({ ApplyAllTestImpl::check_num_2_1(self, <NumT_1_>::default()) });
    }
    fn operator_call_4_class_NumT_2_(&self) {
        ({ ApplyAllTestImpl::check_num_2_2(self, <NumT_2_>::default()) });
    }
}
pub trait IdentityImpl {
    fn operator_call(&self, b: bool) -> bool;
}
impl IdentityImpl for Ptr<Identity> {
    fn operator_call(&self, b: bool) -> bool {
        let b: Value<bool> = Rc::new(RefCell::new(b));
        return (*b.borrow());
    }
}
pub fn __cpp2rust_init_globals() {}
