extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct Tag {
    pub val: i32,
    pub tag: *const libc::c_char,
}
impl Tag {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            val: 0,
            tag: c"Default".as_ptr(),
        };
        this
    }
    pub unsafe fn new_2(mut a: i32, mut b: *const libc::c_char) -> Self {
        let mut this = Self { val: a, tag: b };
        this
    }
    pub unsafe fn destructor(&mut self) {}
}
impl Default for Tag {
    fn default() -> Self {
        unsafe { Tag::new_1() }
    }
}
pub unsafe fn eq_0(rhs: *const Tag, lhs: *const Tag) -> bool {
    return (((*rhs).val) == ((*lhs).val)) && (((*rhs).tag) == ((*lhs).tag));
}
pub unsafe fn comp_1(rhs: *const Tag, lhs: *const Tag) -> bool {
    return (((*rhs).val) < ((*lhs).val));
}
pub unsafe fn test_2(v: *const i32, lo: *const i32, hi: *const i32, mut c: (), x: *const i32) {
    assert!(((unimplemented!()) == (x)));
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
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut z: i32 = 0;
        (unsafe {
            let _v: *const i32 = &x;
            let _x: *const i32 = &x;
            test_2(_v, &y, &z, unimplemented!(), _x)
        });
        (unsafe {
            let _v: *const i32 = &y;
            let _x: *const i32 = &y;
            test_2(_v, &x, &z, unimplemented!(), _x)
        });
    }
    {
        let mut x: i32 = 0;
        let mut y: i32 = 1;
        let mut z: i32 = -1_i32;
        (unsafe {
            let _v: *const i32 = &x;
            let _x: *const i32 = &x;
            test_2(_v, &y, &z, unimplemented!(), _x)
        });
        (unsafe {
            let _lo: *const i32 = &x;
            let _x: *const i32 = &x;
            test_2(&y, _lo, &z, unimplemented!(), _x)
        });
    }
    {
        let mut x: i32 = 1;
        let mut y: i32 = 0;
        let mut z: i32 = 0;
        (unsafe {
            let _lo: *const i32 = &y;
            let _x: *const i32 = &y;
            test_2(&x, _lo, &z, unimplemented!(), _x)
        });
        (unsafe {
            let _v: *const i32 = &y;
            let _x: *const i32 = &y;
            test_2(_v, &x, &z, unimplemented!(), _x)
        });
    }
    {
        let mut x: Tag = Tag::new_2({ 0 }, { c"Zero-x".as_ptr() });
        let _dtor_x = ScopedDestructorUnsafe::new(&raw mut x, Tag::destructor);
        let mut y: Tag = Tag::new_2({ 0 }, { c"Zero-y".as_ptr() });
        let _dtor_y = ScopedDestructorUnsafe::new(&raw mut y, Tag::destructor);
        let mut z: Tag = Tag::new_2({ 0 }, { c"Zero-z".as_ptr() });
        let _dtor_z = ScopedDestructorUnsafe::new(&raw mut z, Tag::destructor);
        assert!(
            (unsafe {
                let _rhs: *const Tag = unimplemented!();
                let _lhs: *const Tag = &x;
                eq_0(_rhs, _lhs)
            })
        );
        assert!(
            (unsafe {
                let _rhs: *const Tag = unimplemented!();
                let _lhs: *const Tag = &y;
                eq_0(_rhs, _lhs)
            })
        );
    }
    {
        let mut x: Tag = Tag::new_2({ 0 }, { c"Zero-x".as_ptr() });
        let _dtor_x = ScopedDestructorUnsafe::new(&raw mut x, Tag::destructor);
        let mut y: Tag = Tag::new_2({ 0 }, { c"Zero-y".as_ptr() });
        let _dtor_y = ScopedDestructorUnsafe::new(&raw mut y, Tag::destructor);
        let mut z: Tag = Tag::new_2({ 1 }, { c"One-z".as_ptr() });
        let _dtor_z = ScopedDestructorUnsafe::new(&raw mut z, Tag::destructor);
        assert!(
            (unsafe {
                let _rhs: *const Tag = unimplemented!();
                let _lhs: *const Tag = &x;
                eq_0(_rhs, _lhs)
            })
        );
        assert!(
            (unsafe {
                let _rhs: *const Tag = unimplemented!();
                let _lhs: *const Tag = &y;
                eq_0(_rhs, _lhs)
            })
        );
    }
    {
        let mut x: Tag = Tag::new_2({ 1 }, { c"One-x".as_ptr() });
        let _dtor_x = ScopedDestructorUnsafe::new(&raw mut x, Tag::destructor);
        let mut y: Tag = Tag::new_2({ 0 }, { c"Zero-y".as_ptr() });
        let _dtor_y = ScopedDestructorUnsafe::new(&raw mut y, Tag::destructor);
        let mut z: Tag = Tag::new_2({ 1 }, { c"One-z".as_ptr() });
        let _dtor_z = ScopedDestructorUnsafe::new(&raw mut z, Tag::destructor);
        assert!(
            (unsafe {
                let _rhs: *const Tag = unimplemented!();
                let _lhs: *const Tag = &x;
                eq_0(_rhs, _lhs)
            })
        );
        assert!(
            (unsafe {
                let _rhs: *const Tag = unimplemented!();
                let _lhs: *const Tag = &z;
                eq_0(_rhs, _lhs)
            })
        );
    }
    {
        let mut x: Tag = Tag::new_2({ 1 }, { c"One-x".as_ptr() });
        let _dtor_x = ScopedDestructorUnsafe::new(&raw mut x, Tag::destructor);
        let mut y: Tag = Tag::new_2({ 0 }, { c"Zero-y".as_ptr() });
        let _dtor_y = ScopedDestructorUnsafe::new(&raw mut y, Tag::destructor);
        let mut z: Tag = Tag::new_2({ 2 }, { c"Two-z".as_ptr() });
        let _dtor_z = ScopedDestructorUnsafe::new(&raw mut z, Tag::destructor);
        assert!(
            (unsafe {
                let _rhs: *const Tag = unimplemented!();
                let _lhs: *const Tag = &x;
                eq_0(_rhs, _lhs)
            })
        );
        assert!(
            (unsafe {
                let _rhs: *const Tag = unimplemented!();
                let _lhs: *const Tag = &x;
                eq_0(_rhs, _lhs)
            })
        );
    }
    {
        let mut x: Tag = Tag::new_2({ 0 }, { c"Zero-x".as_ptr() });
        let _dtor_x = ScopedDestructorUnsafe::new(&raw mut x, Tag::destructor);
        let mut y: Tag = Tag::new_2({ 1 }, { c"One-y".as_ptr() });
        let _dtor_y = ScopedDestructorUnsafe::new(&raw mut y, Tag::destructor);
        let mut z: Tag = Tag::new_2({ 2 }, { c"Two-z".as_ptr() });
        let _dtor_z = ScopedDestructorUnsafe::new(&raw mut z, Tag::destructor);
        assert!(
            (unsafe {
                let _rhs: *const Tag = unimplemented!();
                let _lhs: *const Tag = &y;
                eq_0(_rhs, _lhs)
            })
        );
        assert!(
            (unsafe {
                let _rhs: *const Tag = unimplemented!();
                let _lhs: *const Tag = &y;
                eq_0(_rhs, _lhs)
            })
        );
    }
    {
        let mut x: Tag = Tag::new_2({ 2 }, { c"Two-x".as_ptr() });
        let _dtor_x = ScopedDestructorUnsafe::new(&raw mut x, Tag::destructor);
        let mut y: Tag = Tag::new_2({ 0 }, { c"Zero-y".as_ptr() });
        let _dtor_y = ScopedDestructorUnsafe::new(&raw mut y, Tag::destructor);
        let mut z: Tag = Tag::new_2({ 1 }, { c"One-z".as_ptr() });
        let _dtor_z = ScopedDestructorUnsafe::new(&raw mut z, Tag::destructor);
        assert!(
            (unsafe {
                let _rhs: *const Tag = unimplemented!();
                let _lhs: *const Tag = &z;
                eq_0(_rhs, _lhs)
            })
        );
        assert!(
            (unsafe {
                let _rhs: *const Tag = unimplemented!();
                let _lhs: *const Tag = &z;
                eq_0(_rhs, _lhs)
            })
        );
    }
    {
        let x: i32 = 1;
        let y: i32 = 0;
        let z: i32 = 0;
        const _: () = assert!(true, "std::clamp(x, y, z, std::greater<T>()) == y");;
        const _: () = assert!(true, "std::clamp(y, x, z, std::greater<T>()) == y");;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
