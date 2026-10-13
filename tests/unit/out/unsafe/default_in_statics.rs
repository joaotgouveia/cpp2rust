extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: i64,
    pub __clang_max_align_nonce2: f64,
}
impl Clone for max_align_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for max_align_t {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type std_byte = u8;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Inner {
    pub v: i32,
    pub name: *const libc::c_char,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Outer {
    pub p1: *mut i32,
    pub p2: *const i32,
    pub arr: [*mut i32; 3],
    pub cp: *const libc::c_char,
    pub pp: *mut *mut i32,
    pub inner: Inner,
    pub x: i32,
    pub fn_: Option<unsafe fn(i32) -> i32>,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Foo {
    pub s1: *const libc::c_char,
    pub s2: *const libc::c_char,
    pub fn1: Option<unsafe fn(i32) -> i32>,
    pub fn2: Option<unsafe fn(i32) -> i32>,
    pub n: i32,
}
pub static mut static_fn_0: std::cell::LazyCell<Option<unsafe fn(i32) -> i32>> =
    std::cell::LazyCell::new(|| unsafe { None });
pub static mut static_outer_1: std::cell::LazyCell<Outer> =
    std::cell::LazyCell::new(|| unsafe { <Outer>::default() });
pub static mut static_inner_array_2: std::cell::LazyCell<[Inner; 2]> =
    std::cell::LazyCell::new(|| unsafe { [<Inner>::default(); 2] });
pub static mut static_foo_3: std::cell::LazyCell<Foo> = std::cell::LazyCell::new(|| unsafe {
    Foo {
        s1: c"hello".as_ptr(),
        s2: std::ptr::null(),
        fn1: None,
        fn2: None,
        n: 42,
    }
});
pub static mut static_foo_array_4: std::cell::LazyCell<[Foo; 2]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            Foo {
                s1: c"first".as_ptr(),
                s2: std::ptr::null(),
                fn1: None,
                fn2: None,
                n: 1,
            },
            Foo {
                s1: c"second".as_ptr(),
                s2: std::ptr::null(),
                fn1: None,
                fn2: None,
                n: 2,
            },
        ]
    });
pub unsafe fn check_local_static_5() {
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut local_outer_6)).p1).is_null() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_9(
                c"local_outer.p1 == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void check_local_static()".as_ptr(),
            )
        });
    });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut local_outer_6)).fn_).is_none() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_9(
                c"local_outer.fn == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void check_local_static()".as_ptr(),
            )
        });
    });
    (if (*std::cell::LazyCell::force_mut(&mut *&raw mut local_fn_7)).is_none() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_9(
                c"local_fn == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void check_local_static()".as_ptr(),
            )
        });
    });
    (if (*std::cell::LazyCell::force_mut(&mut *&raw mut local_p_8)).is_null() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_9(
                c"local_p == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void check_local_static()".as_ptr(),
            )
        });
    });
}
static mut local_outer_6: std::cell::LazyCell<Outer> =
    std::cell::LazyCell::new(|| unsafe { <Outer>::default() });
static mut local_fn_7: std::cell::LazyCell<Option<unsafe fn(i32) -> i32>> =
    std::cell::LazyCell::new(|| unsafe { None });
static mut local_p_8: std::cell::LazyCell<*mut i32> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null_mut() });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (if (*std::cell::LazyCell::force_mut(&mut *&raw mut static_fn_0)).is_none() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_9(
                c"static_fn == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut static_outer_1)).p1).is_null() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_9(
                c"static_outer.p1 == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut static_outer_1)).p2).is_null() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_9(
                c"static_outer.p2 == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut static_outer_1)).cp).is_null() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_9(
                c"static_outer.cp == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut static_outer_1)).pp).is_null() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_9(
                c"static_outer.pp == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut static_outer_1)).fn_).is_none() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_9(
                c"static_outer.fn == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut i: i32 = 0;
    'loop_: while ((i) < (3)) {
        (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut static_outer_1)).arr[(i) as usize])
            .is_null()
        {
            &(0);
        } else {
            (unsafe {
                __assert_fail_9(
                    c"static_outer.arr[i] == nullptr".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
        i.prefix_inc();
    }
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut static_outer_1))
        .inner
        .name)
        .is_null()
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_9(
                c"static_outer.inner.name == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut i: i32 = 0;
    'loop_: while ((i) < (2)) {
        (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut static_inner_array_2))[(i) as usize]
            .name)
            .is_null()
        {
            &(0);
        } else {
            (unsafe {
                __assert_fail_9(
                    c"static_inner_array[i].name == nullptr".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
        i.prefix_inc();
    }
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut static_foo_3)).s2).is_null() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_9(
                c"static_foo.s2 == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut static_foo_3)).fn1).is_none() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_9(
                c"static_foo.fn1 == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut static_foo_3)).fn2).is_none() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_9(
                c"static_foo.fn2 == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*std::cell::LazyCell::force_mut(&mut *&raw mut static_foo_3)).n) == (42)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_9(
                c"static_foo.n == 42".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut i: i32 = 0;
    'loop_: while ((i) < (2)) {
        (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut static_foo_array_4))[(i) as usize].s2)
            .is_null()
        {
            &(0);
        } else {
            (unsafe {
                __assert_fail_9(
                    c"static_foo_array[i].s2 == nullptr".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
        (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut static_foo_array_4))[(i) as usize].fn1)
            .is_none()
        {
            &(0);
        } else {
            (unsafe {
                __assert_fail_9(
                    c"static_foo_array[i].fn1 == nullptr".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
        (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut static_foo_array_4))[(i) as usize].fn2)
            .is_none()
        {
            &(0);
        } else {
            (unsafe {
                __assert_fail_9(
                    c"static_foo_array[i].fn2 == nullptr".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
        i.prefix_inc();
    }
    (unsafe { check_local_static_5() });
    return 0;
}
pub unsafe fn __assert_fail_9(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const static_fn_0);
    std::cell::LazyCell::force(&*&raw const static_outer_1);
    std::cell::LazyCell::force(&*&raw const static_inner_array_2);
    std::cell::LazyCell::force(&*&raw const static_foo_3);
    std::cell::LazyCell::force(&*&raw const static_foo_array_4);
}
