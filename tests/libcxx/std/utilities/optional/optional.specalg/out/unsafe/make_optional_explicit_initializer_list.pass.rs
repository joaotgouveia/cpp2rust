extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_optional_TestT___std_optional_TestT__ {}
pub static mut value_2: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { true });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct mandate_same_std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
pub static mut can_make_optional_explicit_impl_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut can_make_optional_explicit_impl_4: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_2))
    });
pub static mut can_make_optional_explicit_impl_5: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_2))
    });
pub static mut can_make_optional_explicit_impl_6: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut can_make_optional_explicit_7: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut can_make_optional_explicit_impl_3))
    });
pub static mut can_make_optional_explicit_8: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut can_make_optional_explicit_impl_4))
    });
pub static mut can_make_optional_explicit_9: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut can_make_optional_explicit_impl_5))
    });
pub static mut can_make_optional_explicit_10: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut can_make_optional_explicit_impl_6))
    });
const _: () = assert!(
    true,
    "!can_make_optional_explicit<int, std::initializer_list<int>&>"
);
const _: () = assert!(
    true,
    "can_make_optional_explicit<std::string, std::initializer_list<char>&>"
);
const _: () = assert!(
    true,
    "can_make_optional_explicit<std::string, std::initializer_list<char>&, std::allocator<char>>"
);
const _: () = assert!(
    true,
    "!can_make_optional_explicit<std::string, std::initializer_list<std::string>&>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestT {
    pub x: (),
    pub size: (),
    pub ptr: *mut (),
}
impl TestT {
    pub unsafe fn new_1(mut il: ()) -> Self {
        let mut this = Self {
            x: (*unimplemented!()),
            size: unimplemented!(),
            ptr: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_2(mut il: (), mut p: *mut ()) -> Self {
        let mut this = Self {
            x: (*unimplemented!()),
            size: unimplemented!(),
            ptr: p,
        };
        this
    }
}
pub unsafe fn test_11() -> () {
    {
        let mut opt: () = unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut i: () = 42_();
        let mut opt: () = unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    let test_string: FnPtr<fn()> = FnPtr::<fn()>::new(|| unsafe {
        {
            let mut opt: () = unimplemented!();
            unimplemented!();
        }
        {
            let mut opt: () = unimplemented!();
            unimplemented!();
        }
    });
    {
        (unsafe { test_string.call() });
    }
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
    (unsafe { test_11() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const value_2);
    std::cell::LazyCell::force(&*&raw const can_make_optional_explicit_impl_3);
    std::cell::LazyCell::force(&*&raw const can_make_optional_explicit_impl_4);
    std::cell::LazyCell::force(&*&raw const can_make_optional_explicit_impl_5);
    std::cell::LazyCell::force(&*&raw const can_make_optional_explicit_impl_6);
    std::cell::LazyCell::force(&*&raw const can_make_optional_explicit_7);
    std::cell::LazyCell::force(&*&raw const can_make_optional_explicit_8);
    std::cell::LazyCell::force(&*&raw const can_make_optional_explicit_9);
    std::cell::LazyCell::force(&*&raw const can_make_optional_explicit_10);
}
