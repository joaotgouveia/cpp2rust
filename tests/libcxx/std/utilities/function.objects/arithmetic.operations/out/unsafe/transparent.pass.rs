extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut value_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_plus_int___two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_plus_int___two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_plus_int__ {}
pub static mut value_1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_plus_std_basic_string_char____two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_plus_std_basic_string_char____two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_plus_std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
pub static mut value_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { ((::std::mem::size_of::<()>()) == (1_())) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_plus_void__ {}
pub static mut value_3: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_minus_int___two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_minus_int___two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_minus_int__ {}
pub static mut value_4: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_minus_std_basic_string_char____two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_minus_std_basic_string_char____two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_minus_std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
pub static mut value_5: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { ((::std::mem::size_of::<()>()) == (1_())) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_minus_void__ {}
pub static mut value_6: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_multiplies_int___two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_multiplies_int___two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_multiplies_int__ {}
pub static mut value_7: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_multiplies_std_basic_string_char____two>())
        == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_multiplies_std_basic_string_char____two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_multiplies_std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
pub static mut value_8: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { ((::std::mem::size_of::<()>()) == (1_())) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_multiplies_void__ {}
pub static mut value_9: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_divides_int___two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_divides_int___two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_divides_int__ {}
pub static mut value_10: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_divides_std_basic_string_char____two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_divides_std_basic_string_char____two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_divides_std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
pub static mut value_11: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { ((::std::mem::size_of::<()>()) == (1_())) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_divides_void__ {}
pub static mut value_12: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_modulus_int___two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_modulus_int___two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_modulus_int__ {}
pub static mut value_13: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_modulus_std_basic_string_char____two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_modulus_std_basic_string_char____two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_modulus_std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
pub static mut value_14: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { ((::std::mem::size_of::<()>()) == (1_())) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_modulus_void__ {}
pub static mut value_15: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_negate_int___two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_negate_int___two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_negate_int__ {}
pub static mut value_16: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_negate_std_basic_string_char____two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_negate_std_basic_string_char____two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_negate_std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
pub static mut value_17: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { ((::std::mem::size_of::<()>()) == (1_())) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_negate_void__ {}
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
    const _: () = assert!(true, "!is_transparent<std::plus<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::plus<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::plus<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::plus<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::minus<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::minus<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::minus<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::minus<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::multiplies<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::multiplies<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::multiplies<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::multiplies<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::divides<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::divides<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::divides<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::divides<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::modulus<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::modulus<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::modulus<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::modulus<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::negate<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::negate<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::negate<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::negate<>>::value");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const value_0);
    std::cell::LazyCell::force(&*&raw const value_1);
    std::cell::LazyCell::force(&*&raw const value_2);
    std::cell::LazyCell::force(&*&raw const value_3);
    std::cell::LazyCell::force(&*&raw const value_4);
    std::cell::LazyCell::force(&*&raw const value_5);
    std::cell::LazyCell::force(&*&raw const value_6);
    std::cell::LazyCell::force(&*&raw const value_7);
    std::cell::LazyCell::force(&*&raw const value_8);
    std::cell::LazyCell::force(&*&raw const value_9);
    std::cell::LazyCell::force(&*&raw const value_10);
    std::cell::LazyCell::force(&*&raw const value_11);
    std::cell::LazyCell::force(&*&raw const value_12);
    std::cell::LazyCell::force(&*&raw const value_13);
    std::cell::LazyCell::force(&*&raw const value_14);
    std::cell::LazyCell::force(&*&raw const value_15);
    std::cell::LazyCell::force(&*&raw const value_16);
    std::cell::LazyCell::force(&*&raw const value_17);
}
