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
pub struct Inner {
    pub value: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Outer {
    pub p: *mut Inner,
}
pub static mut alpha_0: std::cell::LazyCell<Inner> =
    std::cell::LazyCell::new(|| unsafe { Inner { value: 1 } });
pub static mut beta_1: std::cell::LazyCell<Inner> =
    std::cell::LazyCell::new(|| unsafe { Inner { value: 2 } });
pub static mut shared_2: std::cell::LazyCell<Inner> =
    std::cell::LazyCell::new(|| unsafe { Inner { value: 42 } });
pub static mut items_3: std::cell::LazyCell<[*mut Inner; 2]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut alpha_0)) as *mut Inner),
            (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut beta_1)) as *mut Inner),
        ]
    });
pub static mut obj_4: std::cell::LazyCell<Outer> = std::cell::LazyCell::new(|| unsafe {
    Outer {
        p: (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut shared_2)) as *mut Inner),
    }
});
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (if (((*(*std::cell::LazyCell::force_mut(&mut *&raw mut items_3))[(0) as usize]).value) == (1))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"items[0]->value == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(*std::cell::LazyCell::force_mut(&mut *&raw mut items_3))[(1) as usize]).value) == (2))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"items[1]->value == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(*std::cell::LazyCell::force_mut(&mut *&raw mut obj_4)).p).value) == (42)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"obj.p->value == 42".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(*std::cell::LazyCell::force_mut(&mut *&raw mut cache_6))[(0) as usize]).value) == (1))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"cache[0]->value == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(*std::cell::LazyCell::force_mut(&mut *&raw mut cache_6))[(1) as usize]).value) == (2))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"cache[1]->value == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
static mut cache_6: std::cell::LazyCell<[*mut Inner; 2]> = std::cell::LazyCell::new(|| unsafe {
    [
        (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut alpha_0)) as *mut Inner),
        (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut beta_1)) as *mut Inner),
    ]
});
pub unsafe fn __assert_fail_5(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const alpha_0);
    std::cell::LazyCell::force(&*&raw const beta_1);
    std::cell::LazyCell::force(&*&raw const shared_2);
    std::cell::LazyCell::force(&*&raw const items_3);
    std::cell::LazyCell::force(&*&raw const obj_4);
}
