extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(VaArg, FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, DestructorUnsafe, Default)]
#[cpp_from(Vec<bool> , new, mut_ptr)]
pub struct DestroyTracker {
    pub vec__: *mut Vec<bool>,
    pub index_: usize,
}
impl DestroyTracker {
    pub unsafe fn new(vec_: *mut Vec<bool>) -> Self {
        let mut this = Self {
            vec__: (vec_),
            index_: (&(*vec_)).len(),
        };
        unimplemented!();
        this
    }
    pub unsafe fn copy_from(other: *const DestroyTracker) -> Self {
        let mut this = Self {
            vec__: (*other).vec__,
            index_: (&(*(this.vec__).cast_const())).len(),
        };
        unimplemented!();
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const DestroyTracker) -> *mut DestroyTracker {
        return &mut (*(self as *mut DestroyTracker));
    }
    pub unsafe fn destructor(&mut self) {
        {
            *(&mut (*self.vec__)).as_mut_ptr().add(self.index_ as usize) = true;
            (&mut (*self.vec__)).as_mut_ptr().add(self.index_ as usize)
        };
    }
}
impl Clone for DestroyTracker {
    fn clone(&self) -> Self {
        unsafe { DestroyTracker::copy_from(self as *const DestroyTracker) }
    }
}
pub unsafe fn test_0(mut operation: FnPtr<fn(*mut Vec<DestroyTracker>, *mut Vec<bool>)>) {
    let mut all_destroyed: Vec<bool> = unimplemented!();
    {
        let mut v: Vec<DestroyTracker> = Vec::new();
        let mut i: usize = 0_usize;
        'loop_: while ((i) != (100_usize)) {
            (unsafe { operation.call(&mut v, &mut all_destroyed) });
            i.prefix_inc();
        }
    }
    assert!(unimplemented!());
}
pub unsafe fn test_1(mut operation: FnPtr<fn(*mut Vec<DestroyTracker>, *mut Vec<bool>)>) {
    let mut all_destroyed: Vec<bool> = unimplemented!();
    {
        let mut v: Vec<DestroyTracker> = Vec::new();
        let mut i: usize = 0_usize;
        'loop_: while ((i) != (100_usize)) {
            (unsafe { operation.call(&mut v, &mut all_destroyed) });
            i.prefix_inc();
        }
    }
    assert!(unimplemented!());
}
pub unsafe fn test_2(mut operation: FnPtr<fn(*mut Vec<DestroyTracker>, *mut Vec<bool>)>) {
    let mut all_destroyed: Vec<bool> = unimplemented!();
    {
        let mut v: Vec<DestroyTracker> = Vec::new();
        let mut i: usize = 0_usize;
        'loop_: while ((i) != (100_usize)) {
            (unsafe { operation.call(&mut v, &mut all_destroyed) });
            i.prefix_inc();
        }
    }
    assert!(unimplemented!());
}
pub unsafe fn test_3(mut operation: FnPtr<fn(*mut Vec<DestroyTracker>, *mut Vec<bool>)>) {
    let mut all_destroyed: Vec<bool> = unimplemented!();
    {
        let mut v: Vec<DestroyTracker> = Vec::new();
        let mut i: usize = 0_usize;
        'loop_: while ((i) != (100_usize)) {
            (unsafe { operation.call(&mut v, &mut all_destroyed) });
            i.prefix_inc();
        }
    }
    assert!(unimplemented!());
}
pub unsafe fn test_4(mut operation: FnPtr<fn(*mut Vec<DestroyTracker>, *mut Vec<bool>)>) {
    let mut all_destroyed: Vec<bool> = unimplemented!();
    {
        let mut v: Vec<DestroyTracker> = Vec::new();
        let mut i: usize = 0_usize;
        'loop_: while ((i) != (100_usize)) {
            (unsafe { operation.call(&mut v, &mut all_destroyed) });
            i.prefix_inc();
        }
    }
    assert!(unimplemented!());
}
pub unsafe fn test_5(mut operation: FnPtr<fn(*mut Vec<DestroyTracker>, *mut Vec<bool>)>) {
    let mut all_destroyed: Vec<bool> = unimplemented!();
    {
        let mut v: Vec<DestroyTracker> = Vec::new();
        let mut i: usize = 0_usize;
        'loop_: while ((i) != (100_usize)) {
            (unsafe { operation.call(&mut v, &mut all_destroyed) });
            i.prefix_inc();
        }
    }
    assert!(unimplemented!());
}
pub unsafe fn test_6(mut operation: FnPtr<fn(*mut Vec<DestroyTracker>, *mut Vec<bool>)>) {
    let mut all_destroyed: Vec<bool> = unimplemented!();
    {
        let mut v: Vec<DestroyTracker> = Vec::new();
        let mut i: usize = 0_usize;
        'loop_: while ((i) != (100_usize)) {
            (unsafe { operation.call(&mut v, &mut all_destroyed) });
            i.prefix_inc();
        }
    }
    assert!(unimplemented!());
}
pub unsafe fn test_7() -> bool {
    (unsafe {
        test_0(FnPtr::<fn(*mut Vec<DestroyTracker>, *mut Vec<bool>)>::new(
            |vec_: *mut Vec<DestroyTracker>, tracker: *mut Vec<bool>| unsafe {
                {
                    let __init = DestroyTracker::new({ tracker });
                    (*vec_).push(__init)
                };
            },
        ))
    });
    (unsafe {
        test_1(FnPtr::<fn(*mut Vec<DestroyTracker>, *mut Vec<bool>)>::new(
            |vec_: *mut Vec<DestroyTracker>, tracker: *mut Vec<bool>| unsafe {
                {
                    let __a1 = DestroyTracker::new({ tracker });
                    (*vec_).push(__a1)
                };
            },
        ))
    });
    (unsafe {
        test_2(FnPtr::<fn(*mut Vec<DestroyTracker>, *mut Vec<bool>)>::new(
            |vec_: *mut Vec<DestroyTracker>, tracker: *mut Vec<bool>| unsafe {
                unimplemented!();
            },
        ))
    });
    (unsafe {
        test_3(FnPtr::<fn(*mut Vec<DestroyTracker>, *mut Vec<bool>)>::new(
            |vec_: *mut Vec<DestroyTracker>, tracker: *mut Vec<bool>| unsafe {
                {
                    let __it = <UnsafeRandomAccessIterator<DestroyTracker>>::from((*vec_).begin());
                    let pos = __it
                        .iter_distance(&UnsafeRandomAccessIterator::Vec((*vec_).as_mut_ptr()))
                        as usize;
                    (*vec_).insert(pos, DestroyTracker::new({ tracker }));
                    __it
                };
            },
        ))
    });
    (unsafe {
        test_4(FnPtr::<fn(*mut Vec<DestroyTracker>, *mut Vec<bool>)>::new(
            |vec_: *mut Vec<DestroyTracker>, tracker: *mut Vec<bool>| unsafe {
                {
                    let __a0 = ((*vec_).len()).wrapping_add(1_usize) as usize;
                    (*vec_).resize(__a0, DestroyTracker::new({ tracker }))
                };
            },
        ))
    });
    (unsafe {
        test_5(FnPtr::<fn(*mut Vec<DestroyTracker>, *mut Vec<bool>)>::new(
            |vec_: *mut Vec<DestroyTracker>, tracker: *mut Vec<bool>| unsafe {
                unimplemented!();
            },
        ))
    });
    (unsafe {
        test_6(FnPtr::<fn(*mut Vec<DestroyTracker>, *mut Vec<bool>)>::new(
            |vec_: *mut Vec<DestroyTracker>, tracker: *mut Vec<bool>| unsafe {
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_7() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
