extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut assigns_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, Default)]
pub struct Partial {
    pub v: i32,
    pub keep: i32,
}
impl Partial {
    pub unsafe fn new(mut v: i32, mut keep: i32) -> Self {
        let mut this = Self { v: v, keep: keep };
        this
    }
    pub unsafe fn copy_from(o: *const Partial) -> Self {
        let mut this = Self {
            v: (*o).v,
            keep: (*o).keep,
        };
        this
    }
    pub unsafe fn copy_assign(&mut self, o: *const Partial) -> *mut Partial {
        if (((self as *mut Partial).cast_const()) == (o)) {
            return &mut (*(self as *mut Partial));
        }
        self.v = (*o).v;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut assigns_0)).prefix_inc();
        return &mut (*(self as *mut Partial));
    }
}
impl Clone for Partial {
    fn clone(&self) -> Self {
        unsafe { Partial::copy_from(self as *const Partial) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct NonConstAssign {
    pub mark: i32,
}
impl NonConstAssign {
    pub unsafe fn new() -> Self {
        let mut this = Self { mark: 0 };
        this
    }
    pub unsafe fn operator_assign_2(&mut self, o: *mut NonConstAssign) -> *mut NonConstAssign {
        self.mark = (((*o).mark) + (1));
        return &mut (*(self as *mut NonConstAssign));
    }
    pub unsafe fn operator_assign_3(&mut self, o: *const NonConstAssign) -> *mut NonConstAssign {
        self.mark = (((*o).mark) + (10));
        return &mut (*(self as *mut NonConstAssign));
    }
}
impl Default for NonConstAssign {
    fn default() -> Self {
        unsafe { NonConstAssign::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct RefQualified {
    pub mark: i32,
}
impl RefQualified {
    pub unsafe fn new() -> Self {
        let mut this = Self { mark: 0 };
        this
    }
    pub unsafe fn copy_assign(&mut self, o: *const RefQualified) -> *mut RefQualified {
        self.mark = (((*o).mark) + (1));
        return &mut (*(self as *mut RefQualified));
    }
}
impl Default for RefQualified {
    fn default() -> Self {
        unsafe { RefQualified::new() }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct Holder {
    pub p: Partial,
    pub arr: [Partial; 2],
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: Partial = Partial::new({ 1 }, { 100 });
    let mut b: Partial = Partial::new({ 2 }, { 200 });
    let mut c: Partial = Partial::new({ 3 }, { 300 });
    (unsafe { Partial::copy_assign(&mut a, &b) });
    (if ((a.v) == (2)) && ((a.keep) == (100)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"a.v == 2 && a.keep == 100".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut assigns_0)) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"assigns == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { Partial::copy_assign(&mut c, &(*(unsafe { Partial::copy_assign(&mut a, &b) }))) });
    (if ((c.v) == (2)) && ((c.keep) == (300)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"c.v == 2 && c.keep == 300".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut assigns_0)) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"assigns == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe {
        let _o: *const Partial = &a;
        Partial::copy_assign(&mut a, _o)
    });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut assigns_0)) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"assigns == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe {
        let mut _o: Partial = Partial::new({ 9 }, { 900 });
        Partial::copy_assign(&mut a, &mut _o)
    });
    (if ((a.v) == (9)) && ((a.keep) == (100)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"a.v == 9 && a.keep == 100".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut assigns_0)) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"assigns == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let ra: *mut Partial = &mut a;
    (unsafe {
        let _o: *const Partial = &c;
        Partial::copy_assign(&mut (*ra), _o)
    });
    (if ((a.v) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"a.v == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut pa: *mut Partial = (&mut a as *mut Partial);
    (unsafe {
        let _o: *const Partial = &b;
        Partial::copy_assign(&mut (*pa), _o)
    });
    (if ((a.v) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"a.v == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut assigns_0)) == (6)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"assigns == 6".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut h: Holder = Holder {
        p: Partial::new({ 4 }, { 40 }),
        arr: [Partial::new({ 5 }, { 50 }), Partial::new({ 6 }, { 60 })],
    };
    (unsafe { Partial::copy_assign(&mut h.p, &b) });
    (unsafe { Partial::copy_assign(&mut h.arr[(1) as usize], &c) });
    (if ((h.p.v) == (2)) && ((h.p.keep) == (40)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"h.p.v == 2 && h.p.keep == 40".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((h.arr[(1) as usize].v) == (2)) && ((h.arr[(1) as usize].keep) == (60)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"h.arr[1].v == 2 && h.arr[1].keep == 60".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut assigns_0)) == (8)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"assigns == 8".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut n: NonConstAssign = NonConstAssign::new();
    let mut n1: NonConstAssign = NonConstAssign::new();
    let mut n2: NonConstAssign = NonConstAssign::new();
    let cn: NonConstAssign = NonConstAssign::new();
    (unsafe { NonConstAssign::operator_assign_2(&mut n1, &mut n) });
    (unsafe { NonConstAssign::operator_assign_3(&mut n2, &cn) });
    (if ((n1.mark) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"n1.mark == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((n2.mark) == (10)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"n2.mark == 10".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut r: RefQualified = RefQualified::new();
    let mut r1: RefQualified = RefQualified::new();
    (unsafe { RefQualified::copy_assign(&mut r1, &r) });
    (if ((r1.mark) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"r1.mark == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_1(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const assigns_0);
}
