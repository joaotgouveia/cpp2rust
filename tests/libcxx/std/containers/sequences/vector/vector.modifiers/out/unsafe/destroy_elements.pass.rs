extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe, Default)]
pub struct DestroyTracker {
    pub vec__: *mut (),
    pub index_: (),
}
impl DestroyTracker {
    pub unsafe fn new(vec_: *mut ()) -> Self {
        let mut this = Self {
            vec__: (vec_),
            index_: unimplemented!(),
        };
        unimplemented!();
        this
    }
    pub unsafe fn copy_from(other: *const DestroyTracker) -> Self {
        let mut this = Self {
            vec__: (*other).vec__,
            index_: unimplemented!(),
        };
        unimplemented!();
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const DestroyTracker) -> *mut DestroyTracker {
        return &mut (*(self as *mut DestroyTracker));
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
    }
}
impl Clone for DestroyTracker {
    fn clone(&self) -> Self {
        unsafe { DestroyTracker::copy_from(self as *const DestroyTracker) }
    }
}
pub unsafe fn test_0(mut operation: FnPtr<fn(*mut (), *mut ())>) {
    let mut all_destroyed: () = unimplemented!();
    {
        let mut v: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) != (100_())) {
            (unsafe { operation.call(&mut v, &mut all_destroyed) });
            i.prefix_inc();
        }
    }
    unimplemented!();
}
pub unsafe fn test_1(mut operation: FnPtr<fn(*mut (), *mut ())>) {
    let mut all_destroyed: () = unimplemented!();
    {
        let mut v: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) != (100_())) {
            (unsafe { operation.call(&mut v, &mut all_destroyed) });
            i.prefix_inc();
        }
    }
    unimplemented!();
}
pub unsafe fn test_2(mut operation: FnPtr<fn(*mut (), *mut ())>) {
    let mut all_destroyed: () = unimplemented!();
    {
        let mut v: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) != (100_())) {
            (unsafe { operation.call(&mut v, &mut all_destroyed) });
            i.prefix_inc();
        }
    }
    unimplemented!();
}
pub unsafe fn test_3(mut operation: FnPtr<fn(*mut (), *mut ())>) {
    let mut all_destroyed: () = unimplemented!();
    {
        let mut v: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) != (100_())) {
            (unsafe { operation.call(&mut v, &mut all_destroyed) });
            i.prefix_inc();
        }
    }
    unimplemented!();
}
pub unsafe fn test_4(mut operation: FnPtr<fn(*mut (), *mut ())>) {
    let mut all_destroyed: () = unimplemented!();
    {
        let mut v: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) != (100_())) {
            (unsafe { operation.call(&mut v, &mut all_destroyed) });
            i.prefix_inc();
        }
    }
    unimplemented!();
}
pub unsafe fn test_5(mut operation: FnPtr<fn(*mut (), *mut ())>) {
    let mut all_destroyed: () = unimplemented!();
    {
        let mut v: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) != (100_())) {
            (unsafe { operation.call(&mut v, &mut all_destroyed) });
            i.prefix_inc();
        }
    }
    unimplemented!();
}
pub unsafe fn test_6(mut operation: FnPtr<fn(*mut (), *mut ())>) {
    let mut all_destroyed: () = unimplemented!();
    {
        let mut v: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) != (100_())) {
            (unsafe { operation.call(&mut v, &mut all_destroyed) });
            i.prefix_inc();
        }
    }
    unimplemented!();
}
pub unsafe fn test_7() -> () {
    (unsafe {
        test_0(FnPtr::<fn(*mut (), *mut ())>::new(
            |vec_: *mut (), tracker: *mut ()| unsafe {
                unimplemented!();
            },
        ))
    });
    (unsafe {
        test_1(FnPtr::<fn(*mut (), *mut ())>::new(
            |vec_: *mut (), tracker: *mut ()| unsafe {
                unimplemented!();
            },
        ))
    });
    (unsafe {
        test_2(FnPtr::<fn(*mut (), *mut ())>::new(
            |vec_: *mut (), tracker: *mut ()| unsafe {
                unimplemented!();
            },
        ))
    });
    (unsafe {
        test_3(FnPtr::<fn(*mut (), *mut ())>::new(
            |vec_: *mut (), tracker: *mut ()| unsafe {
                unimplemented!();
            },
        ))
    });
    (unsafe {
        test_4(FnPtr::<fn(*mut (), *mut ())>::new(
            |vec_: *mut (), tracker: *mut ()| unsafe {
                unimplemented!();
            },
        ))
    });
    (unsafe {
        test_5(FnPtr::<fn(*mut (), *mut ())>::new(
            |vec_: *mut (), tracker: *mut ()| unsafe {
                unimplemented!();
            },
        ))
    });
    (unsafe {
        test_6(FnPtr::<fn(*mut (), *mut ())>::new(
            |vec_: *mut (), tracker: *mut ()| unsafe {
                unimplemented!();
            },
        ))
    });
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_7() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
