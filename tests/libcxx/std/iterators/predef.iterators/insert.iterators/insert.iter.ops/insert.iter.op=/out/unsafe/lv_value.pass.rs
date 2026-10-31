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
    pub v_: Vec<i32>,
}
impl nasty_vector_int_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { v_: Vec::new() };
        this
    }
    pub unsafe fn destructor(&mut self) {}
    pub unsafe fn begin_12(&mut self) -> UnsafeRandomAccessIterator<i32> {
        return self.v_.begin();
    }
    pub unsafe fn push_back_40(&mut self, x: *const i32) {
        {
            let a0_clone = (*x).clone();
            self.v_.push(a0_clone)
        };
    }
    pub unsafe fn insert_45(
        &mut self,
        mut pos: UnsafeRandomAccessIterator<i32>,
        x: *const i32,
    ) -> UnsafeRandomAccessIterator<i32> {
        return {
            let __it = pos.clone();
            let pos =
                __it.iter_distance(&UnsafeRandomAccessIterator::Vec(self.v_.as_mut_ptr())) as usize;
            self.v_.insert(pos, (*x));
            __it
        };
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
pub unsafe fn operator_eq_0(x: *const nasty_vector_int_, y: *const nasty_vector_int_) -> bool {
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
        assert!(false);
        return std::ptr::null_mut();
    }
    unsafe fn copy_from(_a0: *const nasty_mutex) -> Self {
        let mut this = Self {};
        assert!(false);
        this
    }
    unsafe fn copy_assign(&mut self, _a0: *const nasty_mutex) -> *mut nasty_mutex {
        assert!(false);
        return &mut (*(self as *mut nasty_mutex));
    }
    pub unsafe fn lock(&mut self) {}
    pub unsafe fn try_lock(&mut self) -> bool {
        return true;
    }
    pub unsafe fn unlock(&mut self) {}
    pub unsafe fn lock_shared(&mut self) {}
    pub unsafe fn try_lock_shared(&mut self) -> bool {
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
pub unsafe fn test_1(
    mut c1: Vec<i32>,
    mut j: (),
    mut x1: i32,
    mut x2: i32,
    mut x3: i32,
    c2: *const Vec<i32>,
) {
    let mut q: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_2(
    mut c1: nasty_vector_int_,
    mut j: (),
    mut x1: i32,
    mut x2: i32,
    mut x3: i32,
    c2: *const nasty_vector_int_,
) {
    let mut q: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    assert!(
        (unsafe {
            let _x: *const nasty_vector_int_ = &c1;
            let _y: *const nasty_vector_int_ = c2;
            operator_eq_0(_x, _y)
        })
    );
}
pub unsafe fn insert3at_3(
    c: *mut Vec<i32>,
    mut i: UnsafeRandomAccessIterator<i32>,
    mut x1: i32,
    mut x2: i32,
    mut x3: i32,
) {
    i = {
        let __it = <UnsafeRandomAccessIterator<i32>>::from((i).clone());
        let pos = __it.iter_distance(&UnsafeRandomAccessIterator::Vec((*c).as_mut_ptr())) as usize;
        (*c).insert(pos, x1);
        __it
    };
    i = {
        let __it = <UnsafeRandomAccessIterator<i32>>::from((*i.iter_inc()).clone());
        let pos = __it.iter_distance(&UnsafeRandomAccessIterator::Vec((*c).as_mut_ptr())) as usize;
        (*c).insert(pos, x2);
        __it
    };
    {
        let __it = <UnsafeRandomAccessIterator<i32>>::from((*i.iter_inc()).clone());
        let pos = __it.iter_distance(&UnsafeRandomAccessIterator::Vec((*c).as_mut_ptr())) as usize;
        (*c).insert(pos, x3);
        __it
    };
}
pub unsafe fn insert3at_4(
    c: *mut nasty_vector_int_,
    mut i: UnsafeRandomAccessIterator<i32>,
    mut x1: i32,
    mut x2: i32,
    mut x3: i32,
) {
    i = (unsafe {
        let _pos: UnsafeRandomAccessIterator<i32> =
            <UnsafeRandomAccessIterator<i32>>::from((i).clone());
        let _x: *const i32 = &x1;
        nasty_vector_int_::insert_45(&mut (*c), _pos, _x)
    });
    i = (unsafe {
        let _pos: UnsafeRandomAccessIterator<i32> =
            <UnsafeRandomAccessIterator<i32>>::from((*i.iter_inc()).clone());
        let _x: *const i32 = &x2;
        nasty_vector_int_::insert_45(&mut (*c), _pos, _x)
    });
    (unsafe {
        let _pos: UnsafeRandomAccessIterator<i32> =
            <UnsafeRandomAccessIterator<i32>>::from((*i.iter_inc()).clone());
        let _x: *const i32 = &x3;
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    {
        let mut c1: Vec<i32> = Vec::new();
        let mut i: i32 = 0;
        'loop_: while ((i) < (3)) {
            {
                let a0_clone = i.clone();
                c1.push(a0_clone)
            };
            i.prefix_inc();
        }
        let mut c2: Vec<i32> = c1.clone();
        (unsafe {
            let _c: *mut Vec<i32> = &mut c2;
            let _i: UnsafeRandomAccessIterator<i32> = c2.begin();
            insert3at_3(
                _c,
                _i,
                (('a' as libc::c_char) as i32),
                (('b' as libc::c_char) as i32),
                (('c' as libc::c_char) as i32),
            )
        });
        (unsafe {
            test_1(
                c1.clone(),
                0_(),
                (('a' as libc::c_char) as i32),
                (('b' as libc::c_char) as i32),
                (('c' as libc::c_char) as i32),
                &c2,
            )
        });
        c2 = c1.clone();
        (unsafe {
            let _c: *mut Vec<i32> = &mut c2;
            let _i: UnsafeRandomAccessIterator<i32> = c2.begin().iter_add(1_isize);
            insert3at_3(
                _c,
                _i,
                (('a' as libc::c_char) as i32),
                (('b' as libc::c_char) as i32),
                (('c' as libc::c_char) as i32),
            )
        });
        (unsafe {
            test_1(
                c1.clone(),
                1_(),
                (('a' as libc::c_char) as i32),
                (('b' as libc::c_char) as i32),
                (('c' as libc::c_char) as i32),
                &c2,
            )
        });
        c2 = c1.clone();
        (unsafe {
            let _c: *mut Vec<i32> = &mut c2;
            let _i: UnsafeRandomAccessIterator<i32> = c2.begin().iter_add(2_isize);
            insert3at_3(
                _c,
                _i,
                (('a' as libc::c_char) as i32),
                (('b' as libc::c_char) as i32),
                (('c' as libc::c_char) as i32),
            )
        });
        (unsafe {
            test_1(
                c1.clone(),
                2_(),
                (('a' as libc::c_char) as i32),
                (('b' as libc::c_char) as i32),
                (('c' as libc::c_char) as i32),
                &c2,
            )
        });
        c2 = c1.clone();
        (unsafe {
            let _c: *mut Vec<i32> = &mut c2;
            let _i: UnsafeRandomAccessIterator<i32> = c2.begin().iter_add(3_isize);
            insert3at_3(
                _c,
                _i,
                (('a' as libc::c_char) as i32),
                (('b' as libc::c_char) as i32),
                (('c' as libc::c_char) as i32),
            )
        });
        (unsafe {
            test_1(
                c1.clone(),
                3_(),
                (('a' as libc::c_char) as i32),
                (('b' as libc::c_char) as i32),
                (('c' as libc::c_char) as i32),
                &c2,
            )
        });
    }
    {
        let mut c1: nasty_vector_int_ = nasty_vector_int_::new_1();
        let _dtor_c1 = ScopedDestructorUnsafe::new(&raw mut c1, nasty_vector_int_::destructor);
        let mut i: i32 = 0;
        'loop_: while ((i) < (3)) {
            (unsafe { nasty_vector_int_::push_back_40(&mut c1, &i) });
            i.prefix_inc();
        }
        let mut c2: nasty_vector_int_ = c1.clone();
        let _dtor_c2 = ScopedDestructorUnsafe::new(&raw mut c2, nasty_vector_int_::destructor);
        (unsafe {
            let _c: *mut nasty_vector_int_ = &mut c2;
            let _i: UnsafeRandomAccessIterator<i32> =
                (unsafe { nasty_vector_int_::begin_12(&mut c2) });
            insert3at_4(
                _c,
                _i,
                (('a' as libc::c_char) as i32),
                (('b' as libc::c_char) as i32),
                (('c' as libc::c_char) as i32),
            )
        });
        (unsafe {
            test_2(
                c1.clone(),
                0_(),
                (('a' as libc::c_char) as i32),
                (('b' as libc::c_char) as i32),
                (('c' as libc::c_char) as i32),
                &c2,
            )
        });
        c2 = (c1).clone();
        (unsafe {
            let _c: *mut nasty_vector_int_ = &mut c2;
            let _i: UnsafeRandomAccessIterator<i32> =
                (unsafe { nasty_vector_int_::begin_12(&mut c2) }).iter_add(1_isize);
            insert3at_4(
                _c,
                _i,
                (('a' as libc::c_char) as i32),
                (('b' as libc::c_char) as i32),
                (('c' as libc::c_char) as i32),
            )
        });
        (unsafe {
            test_2(
                c1.clone(),
                1_(),
                (('a' as libc::c_char) as i32),
                (('b' as libc::c_char) as i32),
                (('c' as libc::c_char) as i32),
                &c2,
            )
        });
        c2 = (c1).clone();
        (unsafe {
            let _c: *mut nasty_vector_int_ = &mut c2;
            let _i: UnsafeRandomAccessIterator<i32> =
                (unsafe { nasty_vector_int_::begin_12(&mut c2) }).iter_add(2_isize);
            insert3at_4(
                _c,
                _i,
                (('a' as libc::c_char) as i32),
                (('b' as libc::c_char) as i32),
                (('c' as libc::c_char) as i32),
            )
        });
        (unsafe {
            test_2(
                c1.clone(),
                2_(),
                (('a' as libc::c_char) as i32),
                (('b' as libc::c_char) as i32),
                (('c' as libc::c_char) as i32),
                &c2,
            )
        });
        c2 = (c1).clone();
        (unsafe {
            let _c: *mut nasty_vector_int_ = &mut c2;
            let _i: UnsafeRandomAccessIterator<i32> =
                (unsafe { nasty_vector_int_::begin_12(&mut c2) }).iter_add(3_isize);
            insert3at_4(
                _c,
                _i,
                (('a' as libc::c_char) as i32),
                (('b' as libc::c_char) as i32),
                (('c' as libc::c_char) as i32),
            )
        });
        (unsafe {
            test_2(
                c1.clone(),
                3_(),
                (('a' as libc::c_char) as i32),
                (('b' as libc::c_char) as i32),
                (('c' as libc::c_char) as i32),
                &c2,
            )
        });
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
