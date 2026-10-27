extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_int__long_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_int__long__long_long_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_NumT_0___NumT_1___NumT_2__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list___ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_int__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_int___types_type_list_long__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_int___types_type_list_long___types_type_list_long_long__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_int__long___types_type_list_long_long__ {}
pub unsafe fn for_each_0(mut _a0: types_type_list_NumT_0___NumT_1___NumT_2__, mut f: ApplyAllTest) {
    (unsafe {
        let _arg0: () = {
            (unsafe { ApplyAllTest::operator_call_4_class_NumT_0_(&mut f) });
            0_()
        };
        let _arg1: () = {
            (unsafe { ApplyAllTest::operator_call_4_class_NumT_1_(&mut f) });
            0_()
        };
        let _arg2: () = {
            (unsafe { ApplyAllTest::operator_call_4_class_NumT_2_(&mut f) });
            0_()
        };
        swallow_1(_arg0, _arg1, _arg2)
    });
}
pub unsafe fn swallow_1(mut _a0: (), mut _a1: (), mut _a2: ()) {}
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NumT_0_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NumT_1_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NumT_2_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ApplyAllTest {
    pub is_called_array_: *mut (),
}
impl ApplyAllTest {
    pub unsafe fn new(mut is_called_array: *mut ()) -> Self {
        let mut this = Self {
            is_called_array_: is_called_array,
        };
        this
    }
    pub unsafe fn check_num_2_0(&mut self, mut _a0: NumT_0_) {
        unimplemented!();
        (*self.is_called_array_.offset((0_()) as isize)) = true;
    }
    pub unsafe fn check_num_2_1(&mut self, mut _a0: NumT_1_) {
        unimplemented!();
        (*self.is_called_array_.offset((1_()) as isize)) = true;
    }
    pub unsafe fn check_num_2_2(&mut self, mut _a0: NumT_2_) {
        unimplemented!();
        (*self.is_called_array_.offset((2_()) as isize)) = true;
    }
    pub unsafe fn operator_call_4_class_NumT_0_(&mut self) {
        (unsafe { ApplyAllTest::check_num_2_0(self, <NumT_0_>::default()) });
    }
    pub unsafe fn operator_call_4_class_NumT_1_(&mut self) {
        (unsafe { ApplyAllTest::check_num_2_1(self, <NumT_1_>::default()) });
    }
    pub unsafe fn operator_call_4_class_NumT_2_(&mut self) {
        (unsafe { ApplyAllTest::check_num_2_2(self, <NumT_2_>::default()) });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Identity {}
impl Identity {
    pub unsafe fn operator_call(&self, mut b: ()) -> () {
        return b;
    }
}
pub unsafe fn test_for_each_2() {
    let mut is_called_array: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
    (unsafe {
        for_each_0(
            <types_type_list_NumT_0___NumT_1___NumT_2__>::default(),
            ApplyAllTest::new({ is_called_array.as_mut_ptr() }),
        )
    });
    unimplemented!();
}
pub unsafe fn test_3() -> () {
    (unsafe { test_for_each_2() });
    return true;
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_3() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
