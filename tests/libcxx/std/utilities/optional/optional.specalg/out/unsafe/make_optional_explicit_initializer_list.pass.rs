extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_optional_TestT___std_optional_TestT__ {}
pub static mut value_2: std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { true });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct mandate_same_std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
pub static mut can_make_optional_explicit_impl_3: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut can_make_optional_explicit_impl_4: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_2))
    });
pub static mut can_make_optional_explicit_impl_5: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_2))
    });
pub static mut can_make_optional_explicit_impl_6: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut can_make_optional_explicit_7: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut can_make_optional_explicit_impl_3))
    });
pub static mut can_make_optional_explicit_8: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut can_make_optional_explicit_impl_4))
    });
pub static mut can_make_optional_explicit_9: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut can_make_optional_explicit_impl_5))
    });
pub static mut can_make_optional_explicit_10: std::cell::LazyCell<bool> =
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
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(Vec<i32 > , new_1, value)]
pub struct TestT {
    pub x: i32,
    pub size: i32,
    pub ptr: *mut i32,
}
impl TestT {
    pub unsafe fn new_1(mut il: Vec<i32>) -> Self {
        let mut this = Self {
            x: (*unimplemented!()),
            size: (il.len() as i32),
            ptr: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_2(mut il: Vec<i32>, mut p: *mut i32) -> Self {
        let mut this = Self {
            x: (*unimplemented!()),
            size: (il.len() as i32),
            ptr: p,
        };
        this
    }
}
pub unsafe fn test_11() -> bool {
    {
        let mut opt: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!((((*unimplemented!()).x) == (42)));
        assert!((((*unimplemented!()).size) == (3)));
        assert!(((*unimplemented!()).ptr).is_null());
    }
    {
        let mut i: i32 = 42;
        let mut opt: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!((((*unimplemented!()).x) == (42)));
        assert!((((*unimplemented!()).size) == (3)));
        assert!((((*unimplemented!()).ptr) == (&mut i as *mut i32)));
    }
    let test_string: FnPtr<fn()> = FnPtr::<fn()>::new(|| unsafe {
        {
            let mut opt: () = unimplemented!();
            assert!(
                (*unimplemented!())
                    == std::slice::from_raw_parts(
                        c"123".as_ptr(),
                        libcc2rs::c_str_len(c"123".as_ptr()) + 1
                    )
                    .to_vec()
            );
        }
        {
            let mut opt: () = unimplemented!();
            assert!(
                (*unimplemented!())
                    == std::slice::from_raw_parts(
                        c"abc".as_ptr(),
                        libcc2rs::c_str_len(c"abc".as_ptr()) + 1
                    )
                    .to_vec()
            );
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_11() });
    const _: () = assert!(true, "test()");;
    return 0;
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
