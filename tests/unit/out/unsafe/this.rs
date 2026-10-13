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
pub struct S {
    pub a_: i32,
    pub self__: *mut S,
}
impl S {
    pub unsafe fn new_1(mut a: i32) -> Self {
        let mut this = Self {
            a_: a,
            self__: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_2(mut a: i32, mut other: *mut S) -> Self {
        let mut this = Self {
            a_: a,
            self__: std::ptr::null_mut(),
        };
        if ((&raw mut this) == (other)) {
            this.self__ = std::ptr::null_mut();
        } else {
            this.self__ = other;
        }
        this
    }
    pub unsafe fn new_3(mut a: i32, mut other: *const S) -> Self {
        let mut this = Self {
            a_: a,
            self__: std::ptr::null_mut(),
        };
        if (((&raw mut this).cast_const()) == (other)) {
            this.self__ = std::ptr::null_mut();
        }
        this
    }
    pub unsafe fn returns_this_reference(&mut self) -> *mut S {
        return &mut (*(self as *mut S));
    }
    pub unsafe fn returns_this_pointer(&mut self) -> *mut S {
        return (self as *mut S);
    }
    pub unsafe fn inc(&mut self) -> *mut S {
        self.a_.postfix_inc();
        return &mut (*(self as *mut S));
    }
    pub unsafe fn set_from_this(&mut self) {
        self.a_ = ((self.a_) + (1));
    }
    pub unsafe fn get(&mut self) -> i32 {
        return self.a_;
    }
    pub unsafe fn twice(&mut self) -> i32 {
        return ((unsafe { S::get(self) }) * (2));
    }
    pub unsafe fn link(&mut self) {
        self.self__ = (self as *mut S);
    }
    pub unsafe fn bump_me(&mut self) {
        (unsafe { bump_0((self as *mut S)) });
    }
    pub unsafe fn cref(&self) -> *const S {
        return &(*(self as *const S));
    }
    pub unsafe fn is(&self, mut o: *const S) -> bool {
        return ((o) == (self as *const S));
    }
    pub unsafe fn destroy(&mut self) {
        {
            let __p = (self as *mut S);
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(__p))
            }
        };
    }
    pub unsafe fn reset(&mut self) {
        (*(self as *mut S)) = S::new_1({ 0 });
    }
    pub unsafe fn copy_if_different_const(&mut self, mut other: *const S) -> bool {
        if (((self as *mut S).cast_const()) == (other)) {
            return false;
        }
        self.a_ = (*other).a_;
        self.self__ = (*other).self__;
        return true;
    }
    pub unsafe fn copy_if_different(&mut self, mut other: *mut S) -> bool {
        if ((self as *mut S) == (other)) {
            return false;
        }
        self.a_ = (*other).a_;
        self.self__ = (*other).self__;
        return true;
    }
}
pub unsafe fn bump_0(mut p: *mut S) {
    (*p).a_.postfix_inc();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct D {
    pub a_: i32,
}
impl D {
    pub unsafe fn new(mut a: i32) -> Self {
        let mut this = Self { a_: a };
        this.a_ *= 2;
        this
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S::new_1({ 1 });
    let ref_: *mut S = (unsafe { S::returns_this_reference(&mut s) });
    (*ref_).a_.postfix_inc();
    (if ((s.a_) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"s.a_ == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut ptr: *mut S = (unsafe { S::returns_this_pointer(&mut s) });
    (*ptr).a_.postfix_inc();
    (if ((s.a_) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"s.a_ == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { S::inc(&mut (*(unsafe { S::inc(&mut (*(unsafe { S::inc(&mut s) }))) }))) });
    (if ((s.a_) == (6)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"s.a_ == 6".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { S::set_from_this(&mut s) });
    (if ((s.a_) == (7)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"s.a_ == 7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { S::twice(&mut s) }) == (14)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"s.twice() == 14".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { S::link(&mut s) });
    (if ((s.self__) == (&mut s as *mut S)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"s.self_ == &s".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*s.self__).a_.postfix_inc();
    (if ((s.a_) == (8)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"s.a_ == 8".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { S::bump_me(&mut s) });
    (if ((s.a_) == (9)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"s.a_ == 9".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut d: D = D::new({ 3 });
    (if ((d.a_) == (6)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"d.a_ == 6".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let cr: *const S = (unsafe { S::cref(&s) });
    (if (((*cr).a_) == (9)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"cr.a_ == 9".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut t: S = S::new_1({ 0 });
    (if (unsafe {
        let _o: *const S = (&mut s as *mut S).cast_const();
        S::is(&s, _o)
    }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"s.is(&s)".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if !(unsafe { S::is(&s, (&mut t as *mut S).cast_const()) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"!s.is(&t)".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut p: *mut S = (Box::leak(Box::new(S::new_1({ 1 }))) as *mut S);
    let mut q: *mut S = (unsafe { S::returns_this_pointer(&mut (*p)) });
    (*q).a_.postfix_inc();
    (if (((*p).a_) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"p->a_ == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    {
        let __p = p;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    let mut h: *mut S = (Box::leak(Box::new(S::new_1({ 5 }))) as *mut S);
    (unsafe { S::destroy(&mut (*h)) });
    (unsafe { S::reset(&mut s) });
    (if ((s.a_) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"s.a_ == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (s.self__).is_null() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"s.self_ == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe {
        let _other: *mut S = (&mut s as *mut S);
        S::copy_if_different(&mut s, _other)
    }) as i32)
        == (false as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"s.copy_if_different(&s) == false".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe {
        let _other: *const S = (&mut s as *mut S).cast_const();
        S::copy_if_different_const(&mut s, _other)
    }) as i32)
        == (false as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"s.copy_if_different_const(&s) == false".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut other: S = S::new_1({ 22 });
    (if (((unsafe { S::copy_if_different(&mut s, (&mut other as *mut S)) }) as i32)
        == (true as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"s.copy_if_different(&other) == true".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((s.a_) == (other.a_)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"s.a_ == other.a_".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((s.self__) == (other.self__)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"s.self_ == other.self_".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut u: S = S::new_2({ 1 }, { (&mut s as *mut S) });
    (if ((u.self__) == (&mut s as *mut S)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"u.self_ == &s".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let s_const: S = S::new_1({ 100 });
    let mut u1: S = S::new_3({ 1 }, { (&s_const as *const S) });
    (if (u1.self__).is_null() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"u1.self_ == nullptr".as_ptr(),
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
pub unsafe fn __cpp2rust_init_globals() {}
