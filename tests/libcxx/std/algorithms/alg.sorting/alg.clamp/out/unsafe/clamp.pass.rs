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
    pub val: (),
    pub tag: *const (),
}
impl Tag {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            val: 0_(),
            tag: c"Default".as_ptr(),
        };
        this
    }
    pub unsafe fn new_2(mut a: (), mut b: *const ()) -> Self {
        let mut this = Self { val: a, tag: b };
        this
    }
    pub unsafe fn destructor(&mut self) {}
}
impl std::cmp::Ord for Tag {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_0(self as *const Tag, other as *const Tag) {
                std::cmp::Ordering::Less
            } else if operator_lt_0(other as *const Tag, self as *const Tag) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Tag {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_lt_0(self as *const Tag, other as *const Tag))
                && !(operator_lt_0(other as *const Tag, self as *const Tag))
        }
    }
}
impl std::cmp::Eq for Tag {}
impl Default for Tag {
    fn default() -> Self {
        unsafe { Tag::new_1() }
    }
}
pub unsafe fn eq_1(rhs: *const Tag, lhs: *const Tag) -> () {
    return (((*rhs).val) == ((*lhs).val)) && (((*rhs).tag) == ((*lhs).tag));
}
pub unsafe fn operator_lt_0(rhs: *const Tag, lhs: *const Tag) -> () {
    return (((*rhs).val) < ((*lhs).val));
}
pub unsafe fn test_2(a: *const (), lo: *const (), hi: *const (), x: *const ()) {
    unimplemented!();
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
    {
        let mut x: () = 0_();
        let mut y: () = 0_();
        let mut z: () = 0_();
        (unsafe {
            let _a: *const () = &x;
            let _x: *const () = &x;
            test_2(_a, &y, &z, _x)
        });
        (unsafe {
            let _a: *const () = &y;
            let _x: *const () = &y;
            test_2(_a, &x, &z, _x)
        });
    }
    {
        let mut x: () = 0_();
        let mut y: () = 1_();
        let mut z: () = 2_();
        (unsafe {
            let _lo: *const () = &y;
            let _x: *const () = &y;
            test_2(&x, _lo, &z, _x)
        });
        (unsafe {
            let _a: *const () = &y;
            let _x: *const () = &y;
            test_2(_a, &x, &z, _x)
        });
    }
    {
        let mut x: () = 1_();
        let mut y: () = 0_();
        let mut z: () = 1_();
        (unsafe {
            let _a: *const () = &x;
            let _x: *const () = &x;
            test_2(_a, &y, &z, _x)
        });
        (unsafe {
            let _lo: *const () = &x;
            let _x: *const () = &x;
            test_2(&y, _lo, &z, _x)
        });
    }
    {
        let mut x: Tag = Tag::new_2({ 0_() }, { c"Zero-x".as_ptr() });
        let _dtor_x = ScopedDestructorUnsafe::new(&raw mut x, Tag::destructor);
        let mut y: Tag = Tag::new_2({ 0_() }, { c"Zero-y".as_ptr() });
        let _dtor_y = ScopedDestructorUnsafe::new(&raw mut y, Tag::destructor);
        let mut z: Tag = Tag::new_2({ 0_() }, { c"Zero-z".as_ptr() });
        let _dtor_z = ScopedDestructorUnsafe::new(&raw mut z, Tag::destructor);
        unimplemented!();
        unimplemented!();
    }
    {
        let mut x: Tag = Tag::new_2({ 0_() }, { c"Zero-x".as_ptr() });
        let _dtor_x = ScopedDestructorUnsafe::new(&raw mut x, Tag::destructor);
        let mut y: Tag = Tag::new_2({ 0_() }, { c"Zero-y".as_ptr() });
        let _dtor_y = ScopedDestructorUnsafe::new(&raw mut y, Tag::destructor);
        let mut z: Tag = Tag::new_2({ 1_() }, { c"One-z".as_ptr() });
        let _dtor_z = ScopedDestructorUnsafe::new(&raw mut z, Tag::destructor);
        unimplemented!();
        unimplemented!();
    }
    {
        let mut x: Tag = Tag::new_2({ 1_() }, { c"One-x".as_ptr() });
        let _dtor_x = ScopedDestructorUnsafe::new(&raw mut x, Tag::destructor);
        let mut y: Tag = Tag::new_2({ 0_() }, { c"Zero-y".as_ptr() });
        let _dtor_y = ScopedDestructorUnsafe::new(&raw mut y, Tag::destructor);
        let mut z: Tag = Tag::new_2({ 1_() }, { c"One-z".as_ptr() });
        let _dtor_z = ScopedDestructorUnsafe::new(&raw mut z, Tag::destructor);
        unimplemented!();
        unimplemented!();
    }
    {
        let mut x: Tag = Tag::new_2({ 1_() }, { c"One-x".as_ptr() });
        let _dtor_x = ScopedDestructorUnsafe::new(&raw mut x, Tag::destructor);
        let mut y: Tag = Tag::new_2({ 0_() }, { c"Zero-y".as_ptr() });
        let _dtor_y = ScopedDestructorUnsafe::new(&raw mut y, Tag::destructor);
        let mut z: Tag = Tag::new_2({ 2_() }, { c"Two-z".as_ptr() });
        let _dtor_z = ScopedDestructorUnsafe::new(&raw mut z, Tag::destructor);
        unimplemented!();
        unimplemented!();
    }
    {
        let mut x: Tag = Tag::new_2({ 0_() }, { c"Zero-x".as_ptr() });
        let _dtor_x = ScopedDestructorUnsafe::new(&raw mut x, Tag::destructor);
        let mut y: Tag = Tag::new_2({ 1_() }, { c"One-y".as_ptr() });
        let _dtor_y = ScopedDestructorUnsafe::new(&raw mut y, Tag::destructor);
        let mut z: Tag = Tag::new_2({ 2_() }, { c"Two-z".as_ptr() });
        let _dtor_z = ScopedDestructorUnsafe::new(&raw mut z, Tag::destructor);
        unimplemented!();
        unimplemented!();
    }
    {
        let mut x: Tag = Tag::new_2({ 2_() }, { c"Two-x".as_ptr() });
        let _dtor_x = ScopedDestructorUnsafe::new(&raw mut x, Tag::destructor);
        let mut y: Tag = Tag::new_2({ 0_() }, { c"Zero-y".as_ptr() });
        let _dtor_y = ScopedDestructorUnsafe::new(&raw mut y, Tag::destructor);
        let mut z: Tag = Tag::new_2({ 1_() }, { c"One-z".as_ptr() });
        let _dtor_z = ScopedDestructorUnsafe::new(&raw mut z, Tag::destructor);
        unimplemented!();
        unimplemented!();
    }
    {
        let x: () = 1_();
        let y: () = 0_();
        let z: () = 1_();
        const _: () = assert!(true, "std::clamp(x, y, z) == x");;
        const _: () = assert!(true, "std::clamp(y, x, z) == x");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
