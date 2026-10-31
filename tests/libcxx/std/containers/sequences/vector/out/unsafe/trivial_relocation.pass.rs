extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Tracker {
    pub move_constructs: usize,
}
impl Default for Tracker {
    fn default() -> Self {
        unsafe {
            Tracker {
                move_constructs: 0_usize,
            }
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct Inner {
    pub tracker_: *mut Tracker,
}
impl Inner {
    pub unsafe fn new(mut tracker: *mut Tracker) -> Self {
        let mut this = Self { tracker_: tracker };
        this
    }
    pub unsafe fn copy_from(rhs: *const Inner) -> Self {
        let mut this = Self {
            tracker_: (*rhs).tracker_,
        };
        (*this.tracker_).move_constructs = ((*this.tracker_).move_constructs).wrapping_add(1_usize);
        this
    }
    pub unsafe fn move_from(rhs: *mut Inner) -> Self {
        let mut this = Self {
            tracker_: (*rhs).tracker_,
        };
        (*this.tracker_).move_constructs = ((*this.tracker_).move_constructs).wrapping_add(1_usize);
        this
    }
}
impl Clone for Inner {
    fn clone(&self) -> Self {
        unsafe { Inner::copy_from(self as *const Inner) }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct NotTriviallyMovable {
    pub inner_: Inner,
}
impl NotTriviallyMovable {
    pub unsafe fn new(mut tracker: *mut Tracker) -> Self {
        let mut this = Self {
            inner_: Inner::new({ tracker }),
        };
        this
    }
    pub unsafe fn move_from(other: *mut NotTriviallyMovable) -> Self {
        let mut this = Self {
            inner_: Inner::move_from({ &mut (*other).inner_ }),
        };
        this
    }
}
const _: () = assert!(
    true,
    "!std::is_trivially_copyable<NotTriviallyMovable>::value"
);
const _: () = assert!(true, "");
pub unsafe fn tests_0() -> bool {
    let mut track: Tracker = <Tracker>::default();
    let mut v: Vec<NotTriviallyMovable> = Vec::new();
    if 5_usize as usize > v.capacity() as usize {
        let len_0 = v.len();
        v.reserve_exact(5_usize as usize - len_0 as usize);
    };
    let capacity: usize = v.capacity();
    'loop_: while ((v.len()) < (v.capacity())) {
        {
            let __init = NotTriviallyMovable::new({ (&mut track as *mut Tracker) });
            v.push(__init)
        };
    }
    assert!(((track.move_constructs) == (0_usize)));
    assert!(((v.capacity()) == (capacity)));
    assert!(((v.len()) == (capacity)));
    {
        let __init = NotTriviallyMovable::new({ (&mut track as *mut Tracker) });
        v.push(__init)
    };
    assert!(((track.move_constructs) == (capacity)));
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
    (unsafe { tests_0() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
