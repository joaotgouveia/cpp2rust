extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Traits_char__Kind = u32;
pub const Traits_char__Kind_Small: Traits_char__Kind = 0;
pub const Traits_char__Kind_Large: Traits_char__Kind = 1;
pub type anon_1 = u32;
pub const anon_1_Size: anon_1 = 1;
pub static mut Traits_char__Width_2: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe {
        (((::std::mem::size_of::<libc::c_char>() as usize).wrapping_mul(8_usize)) as i32)
    });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Traits_char__Box {
    pub value: libc::c_char,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Traits_0 {}
impl Traits_0 {
    pub unsafe fn kind() -> Traits_char__Kind {
        return if ((::std::mem::size_of::<libc::c_char>()) > (2_usize)) {
            Traits_char__Kind_Large
        } else {
            Traits_char__Kind_Small
        };
    }
    pub unsafe fn box_(&mut self, mut v: libc::c_char) -> Traits_char__Box {
        return Traits_char__Box { value: v };
    }
}
pub type Traits_int__Kind = u32;
pub const Traits_int__Kind_Small: Traits_int__Kind = 0;
pub const Traits_int__Kind_Large: Traits_int__Kind = 1;
pub type anon_4 = u32;
pub const anon_4_Size: anon_4 = 4;
pub static mut Traits_int__Width_5: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe {
        (((::std::mem::size_of::<i32>() as usize).wrapping_mul(8_usize)) as i32)
    });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Traits_int__Box {
    pub value: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Traits_3 {}
impl Traits_3 {
    pub unsafe fn kind() -> Traits_int__Kind {
        return if ((::std::mem::size_of::<i32>()) > (2_usize)) {
            Traits_int__Kind_Large
        } else {
            Traits_int__Kind_Small
        };
    }
    pub unsafe fn box_(&mut self, mut v: i32) -> Traits_int__Box {
        return Traits_int__Box { value: v };
    }
}
pub type anon_7 = u32;
pub const anon_7_value: anon_7 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct IsSame_6 {}
pub type anon_9 = u32;
pub const anon_9_value: anon_9 = 0;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct IsSame_8 {}
pub type anon_11 = u32;
pub const anon_11_value: anon_11 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct IsSame_10 {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (if ((anon_1_Size as i32) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
                c"Traits<char>::Size == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((anon_4_Size as i32) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
                c"Traits<int>::Size == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut Traits_char__Width_2)) == (8)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
                c"Traits<char>::Width == 8".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut Traits_int__Width_5)) == (32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
                c"Traits<int>::Width == 32".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe { Traits_0::kind() }) as i32) == (Traits_char__Kind_Small as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
                c"Traits<char>::kind() == Traits<char>::Small".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe { Traits_3::kind() }) as i32) == (Traits_int__Kind_Large as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
                c"Traits<int>::kind() == Traits<int>::Large".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut tc: Traits_0 = <Traits_0>::default();
    let mut ti: Traits_3 = <Traits_3>::default();
    let mut bc: Traits_char__Box = (unsafe { Traits_0::box_(&mut tc, ('a' as libc::c_char)) });
    let mut bi: Traits_int__Box = (unsafe { Traits_3::box_(&mut ti, 7) });
    (if ((bc.value as i32) == (('a' as libc::c_char) as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
                c"bc.value == \'a\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((bi.value) == (7)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
                c"bi.value == 7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((anon_7_value as i32) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
                c"(IsSame<int, int>::value == 1)".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((anon_9_value as i32) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
                c"(IsSame<int, char>::value == 0)".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((anon_11_value as i32) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
                c"(IsSame<char, char>::value == 1)".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_13(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const Traits_char__Width_2);
    std::cell::LazyCell::force(&*&raw const Traits_int__Width_5);
}
