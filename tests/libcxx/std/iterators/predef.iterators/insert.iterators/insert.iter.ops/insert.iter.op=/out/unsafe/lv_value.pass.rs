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
pub struct nasty_vector_int_ {
    pub v_: (),
}
impl nasty_vector_int_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            v_: unimplemented!(),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {}
    pub unsafe fn begin_12(&mut self) -> () {
        return unimplemented!();
    }
    pub unsafe fn push_back_40(&mut self, x: *const ()) {
        unimplemented!();
    }
    pub unsafe fn insert_45(&mut self, mut pos: (), x: *const ()) -> () {
        return unimplemented!();
    }
}
impl std::cmp::PartialEq for nasty_vector_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_0(
                self as *const nasty_vector_int_,
                other as *const nasty_vector_int_,
            )
        }
    }
}
impl std::cmp::Eq for nasty_vector_int_ {}
impl Default for nasty_vector_int_ {
    fn default() -> Self {
        unsafe { nasty_vector_int_::new_1() }
    }
}
pub unsafe fn operator_eq_0(x: *const nasty_vector_int_, y: *const nasty_vector_int_) -> () {
    return unimplemented!();
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct nasty_mutex {}
impl nasty_mutex {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn destructor(&mut self) {}
    pub unsafe fn operator_addr(&mut self) -> *mut nasty_mutex {
        unimplemented!();
        return std::ptr::null_mut();
    }
    unsafe fn copy_from(_a0: *const nasty_mutex) -> Self {
        let mut this = Self {};
        unimplemented!();
        this
    }
    unsafe fn copy_assign(&mut self, _a0: *const nasty_mutex) -> *mut nasty_mutex {
        unimplemented!();
        return &mut (*(self as *mut nasty_mutex));
    }
    pub unsafe fn lock(&mut self) {}
    pub unsafe fn try_lock(&mut self) -> () {
        return true;
    }
    pub unsafe fn unlock(&mut self) {}
    pub unsafe fn lock_shared(&mut self) {}
    pub unsafe fn try_lock_shared(&mut self) -> () {
        return true;
    }
    pub unsafe fn unlock_shared(&mut self) {}
}
impl Clone for nasty_mutex {
    fn clone(&self) -> Self {
        unsafe { nasty_mutex::copy_from(self as *const nasty_mutex) }
    }
}
impl Default for nasty_mutex {
    fn default() -> Self {
        unsafe { nasty_mutex::new() }
    }
}
pub unsafe fn test_1(mut c1: (), mut j: (), mut x1: (), mut x2: (), mut x3: (), c2: *const ()) {
    let mut q: () = std_insert_iterator_std_vector_int__std_allocator_int___::new({ &mut c1 }, {
        unimplemented!()
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_2(
    mut c1: nasty_vector_int_,
    mut j: (),
    mut x1: (),
    mut x2: (),
    mut x3: (),
    c2: *const nasty_vector_int_,
) {
    let mut q: () = std_insert_iterator_nasty_vector_int__::new({ &mut c1 }, { unimplemented!() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn insert3at_3(c: *mut (), mut i: (), mut x1: (), mut x2: (), mut x3: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn insert3at_4(
    c: *mut nasty_vector_int_,
    mut i: (),
    mut x1: (),
    mut x2: (),
    mut x3: (),
) {
    unimplemented!();
    unimplemented!();
    (unsafe {
        let _pos: () = unimplemented!();
        let _x: *const () = &x3;
        nasty_vector_int_::insert_45(&mut (*c), _pos, _x)
    });
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
        let mut c1: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (3_())) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut c2: () = unimplemented!();
        (unsafe {
            let _c: *mut () = &mut c2;
            let _i: () = unimplemented!();
            insert3at_3(_c, _i, ('a' as ()), ('b' as ()), ('c' as ()))
        });
        (unsafe {
            test_1(
                unimplemented!(),
                0_(),
                ('a' as ()),
                ('b' as ()),
                ('c' as ()),
                &c2,
            )
        });
        unimplemented!();
        (unsafe {
            let _c: *mut () = &mut c2;
            let _i: () = unimplemented!();
            insert3at_3(_c, _i, ('a' as ()), ('b' as ()), ('c' as ()))
        });
        (unsafe {
            test_1(
                unimplemented!(),
                1_(),
                ('a' as ()),
                ('b' as ()),
                ('c' as ()),
                &c2,
            )
        });
        unimplemented!();
        (unsafe {
            let _c: *mut () = &mut c2;
            let _i: () = unimplemented!();
            insert3at_3(_c, _i, ('a' as ()), ('b' as ()), ('c' as ()))
        });
        (unsafe {
            test_1(
                unimplemented!(),
                2_(),
                ('a' as ()),
                ('b' as ()),
                ('c' as ()),
                &c2,
            )
        });
        unimplemented!();
        (unsafe {
            let _c: *mut () = &mut c2;
            let _i: () = unimplemented!();
            insert3at_3(_c, _i, ('a' as ()), ('b' as ()), ('c' as ()))
        });
        (unsafe {
            test_1(
                unimplemented!(),
                3_(),
                ('a' as ()),
                ('b' as ()),
                ('c' as ()),
                &c2,
            )
        });
    }
    {
        let mut c1: nasty_vector_int_ = nasty_vector_int_::new_1();
        let _dtor_c1 = ScopedDestructorUnsafe::new(&raw mut c1, nasty_vector_int_::destructor);
        let mut i: () = 0_();
        'loop_: while ((i) < (3_())) {
            (unsafe { nasty_vector_int_::push_back_40(&mut c1, &i) });
            i.prefix_inc();
        }
        let mut c2: nasty_vector_int_ = c1.clone();
        let _dtor_c2 = ScopedDestructorUnsafe::new(&raw mut c2, nasty_vector_int_::destructor);
        (unsafe {
            let _c: *mut nasty_vector_int_ = &mut c2;
            let _i: () = (unsafe { nasty_vector_int_::begin_12(&mut c2) });
            insert3at_4(_c, _i, ('a' as ()), ('b' as ()), ('c' as ()))
        });
        (unsafe { test_2(c1.clone(), 0_(), ('a' as ()), ('b' as ()), ('c' as ()), &c2) });
        c2 = (c1).clone();
        (unsafe {
            let _c: *mut nasty_vector_int_ = &mut c2;
            let _i: () = unimplemented!();
            insert3at_4(_c, _i, ('a' as ()), ('b' as ()), ('c' as ()))
        });
        (unsafe { test_2(c1.clone(), 1_(), ('a' as ()), ('b' as ()), ('c' as ()), &c2) });
        c2 = (c1).clone();
        (unsafe {
            let _c: *mut nasty_vector_int_ = &mut c2;
            let _i: () = unimplemented!();
            insert3at_4(_c, _i, ('a' as ()), ('b' as ()), ('c' as ()))
        });
        (unsafe { test_2(c1.clone(), 2_(), ('a' as ()), ('b' as ()), ('c' as ()), &c2) });
        c2 = (c1).clone();
        (unsafe {
            let _c: *mut nasty_vector_int_ = &mut c2;
            let _i: () = unimplemented!();
            insert3at_4(_c, _i, ('a' as ()), ('b' as ()), ('c' as ()))
        });
        (unsafe { test_2(c1.clone(), 3_(), ('a' as ()), ('b' as ()), ('c' as ()), &c2) });
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
