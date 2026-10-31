extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(x: *const X, y: *const X) -> bool {
    return (((*x).i_) == ((*y).i_)) && (((*x).j_) == ((*y).j_));
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CppFromUnsafe, DestructorUnsafe)]
#[cpp_from(bool, new_1, mut_ptr)]
pub struct X {
    i_: i32,
    j_: i32,
    dtor_called_: *mut bool,
}
impl X {
    pub unsafe fn new_1(dtor_called: *mut bool) -> Self {
        let mut this = Self {
            i_: 0,
            j_: 0,
            dtor_called_: (dtor_called),
        };
        this
    }
    pub unsafe fn new_2(mut i: i32, dtor_called: *mut bool) -> Self {
        let mut this = Self {
            i_: i,
            j_: 0,
            dtor_called_: (dtor_called),
        };
        this
    }
    pub unsafe fn new_3(mut il: Vec<i32>, dtor_called: *mut bool) -> Self {
        let mut this = Self {
            i_: (*unimplemented!().offset((0) as isize)),
            j_: (*unimplemented!().offset((1) as isize)),
            dtor_called_: (dtor_called),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*self.dtor_called_) = true;
    }
}
impl std::cmp::PartialEq for X {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_0(self as *const X, other as *const X) }
    }
}
impl std::cmp::Eq for X {}
impl Default for X {
    fn default() -> Self {
        unsafe {
            X {
                i_: 0_i32,
                j_: 0,
                dtor_called_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_1(x: *const Y, y: *const Y) -> bool {
    return (((*x).i_) == ((*y).i_)) && (((*x).j_) == ((*y).j_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(i32, new_1, value)]
#[cpp_from(Vec<i32 > , new_2, value)]
pub struct Y {
    i_: i32,
    j_: i32,
}
impl Y {
    pub unsafe fn new_3() -> Self {
        let mut this = Self { i_: 0, j_: 0 };
        this
    }
    pub unsafe fn new_1(mut i: i32) -> Self {
        let mut this = Self { i_: i, j_: 0 };
        this
    }
    pub unsafe fn new_2(mut il: Vec<i32>) -> Self {
        let mut this = Self {
            i_: (*unimplemented!().offset((0) as isize)),
            j_: (*unimplemented!().offset((1) as isize)),
        };
        this
    }
}
impl std::cmp::PartialEq for Y {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_1(self as *const Y, other as *const Y) }
    }
}
impl std::cmp::Eq for Y {}
impl Default for Y {
    fn default() -> Self {
        unsafe { Y::new_3() }
    }
}
pub unsafe fn operator_eq_3(x: *const Z, y: *const Z) -> bool {
    return (((*x).i_) == ((*y).i_)) && (((*x).j_) == ((*y).j_));
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CppFromUnsafe, DestructorUnsafe)]
#[cpp_from(i32, new_1, value)]
#[cpp_from(Vec<i32 > , new_2, value)]
pub struct Z {
    i_: i32,
    j_: i32,
}
impl Z {
    pub unsafe fn new_3() -> Self {
        let mut this = Self { i_: 0, j_: 0 };
        this
    }
    pub unsafe fn new_1(mut i: i32) -> Self {
        let mut this = Self { i_: i, j_: 0 };
        this
    }
    pub unsafe fn new_2(mut il: Vec<i32>) -> Self {
        let mut this = Self {
            i_: (*unimplemented!().offset((0) as isize)),
            j_: (*unimplemented!().offset((1) as isize)),
        };
        std::process::abort();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dtor_called_2)) = true;
    }
}
impl std::cmp::PartialEq for Z {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_3(self as *const Z, other as *const Z) }
    }
}
impl std::cmp::Eq for Z {}
impl Default for Z {
    fn default() -> Self {
        unsafe { Z::new_3() }
    }
}
pub static mut dtor_called_2: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub unsafe fn check_X_4() -> bool {
    let mut dtor_called: bool = false;
    let mut x: X = X::new_1({ &mut dtor_called });
    let _dtor_x = ScopedDestructorUnsafe::new(&raw mut x, X::destructor);
    let mut opt: () = unimplemented!();
    assert!(((dtor_called as i32) == (false as i32)));
    let v: *mut X = unimplemented!();
    const _: () = assert!(true, "std::is_same_v<X&, decltype(v)>");;
    assert!(dtor_called);
    assert!(
        (unsafe {
            let _x: *const X = &(*unimplemented!());
            let mut _y: X = X::new_3({ Vec::from([1, 2]) }, { &mut dtor_called });
            operator_eq_0(_x, &mut _y)
        })
    );
    assert!(((v) == (&mut (*unimplemented!()) as *mut X)));
    return true;
}
pub unsafe fn check_Y_5() -> bool {
    let mut opt: () = unimplemented!();
    let v: *mut Y = unimplemented!();
    const _: () = assert!(true, "std::is_same_v<Y&, decltype(v)>");;
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(
        (unsafe {
            let _x: *const Y = &(*unimplemented!());
            let mut _y: Y = Y::new_2({ Vec::from([1, 2]) });
            operator_eq_1(_x, &mut _y)
        })
    );
    assert!(((v) == (&mut (*unimplemented!()) as *mut Y)));
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
    {
        (unsafe { check_X_4() });
        const _: () = assert!(true, "check_X()");;
    }
    {
        let mut opt: () = unimplemented!();
        let v: *mut Vec<i32> = unimplemented!();
        const _: () = assert!(true, "std::is_same_v<std::vector<int>&, decltype(v)>");;
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(unimplemented!());
        assert!(((v) == (&mut (*unimplemented!()) as *mut Vec<i32>)));
    }
    {
        (unsafe { check_Y_5() });
        const _: () = assert!(true, "check_Y()");;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const dtor_called_2);
}
