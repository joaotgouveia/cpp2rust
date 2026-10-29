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
    pub vec__: *mut Vec<bool>,
    pub index_: usize,
}
impl DestroyTracker {
    pub unsafe fn new(vec_: *mut Vec<bool>) -> Self {
        let mut this = Self {
            vec__: (vec_),
            index_: (*vec_).len(),
        };
        {
            let a0_clone = false.clone();
            (*vec_).push(a0_clone)
        };
        this
    }
    pub unsafe fn copy_from(other: *const DestroyTracker) -> Self {
        let mut this = Self {
            vec__: (*other).vec__,
            index_: (*(this.vec__).cast_const()).len(),
        };
        {
            let a0_clone = false.clone();
            (*this.vec__).push(a0_clone)
        };
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const DestroyTracker) -> *mut DestroyTracker {
        return &mut (*(self as *mut DestroyTracker));
    }
    pub unsafe fn destructor(&mut self) {
        (&mut (*self.vec__))[(self.index_)] = true;
    }
}
impl Clone for DestroyTracker {
    fn clone(&self) -> Self {
        unsafe { DestroyTracker::copy_from(self as *const DestroyTracker) }
    }
}
pub unsafe fn test_0(mut operation: FnPtr<fn(*mut Vec<DestroyTracker>, *mut Vec<bool>)>) {
    let mut all_destroyed: Vec<bool> = Vec::new();
    {
        let mut v: Vec<DestroyTracker> = Vec::new();
        let mut i: usize = 0_usize;
        'loop_: while ((i) != (100_usize)) {
            (unsafe { operation.call(&mut v, &mut all_destroyed) });
            i.prefix_inc();
        }
    }
    assert!(
        (unsafe {
            let ___first: UnsafeRandomAccessIterator<bool> = all_destroyed.begin();
            let ___last: UnsafeRandomAccessIterator<bool> = all_destroyed.end();
            all_of_1(
                ___first,
                ___last,
                FnPtr::<fn(bool) -> bool>::new(|mut b: bool| -> bool {
                    unsafe {
                        return b;
                    }
                }),
            )
        })
    );
}
pub unsafe fn test_2(mut operation: FnPtr<fn(*mut Vec<DestroyTracker>, *mut Vec<bool>)>) {
    let mut all_destroyed: Vec<bool> = Vec::new();
    {
        let mut v: Vec<DestroyTracker> = Vec::new();
        let mut i: usize = 0_usize;
        'loop_: while ((i) != (100_usize)) {
            (unsafe { operation.call(&mut v, &mut all_destroyed) });
            i.prefix_inc();
        }
    }
    assert!(
        (unsafe {
            let ___first: UnsafeRandomAccessIterator<bool> = all_destroyed.begin();
            let ___last: UnsafeRandomAccessIterator<bool> = all_destroyed.end();
            all_of_3(
                ___first,
                ___last,
                FnPtr::<fn(bool) -> bool>::new(|mut b: bool| -> bool {
                    unsafe {
                        return b;
                    }
                }),
            )
        })
    );
}
pub unsafe fn test_4(mut operation: FnPtr<fn(*mut Vec<DestroyTracker>, *mut Vec<bool>)>) {
    let mut all_destroyed: Vec<bool> = Vec::new();
    {
        let mut v: Vec<DestroyTracker> = Vec::new();
        let mut i: usize = 0_usize;
        'loop_: while ((i) != (100_usize)) {
            (unsafe { operation.call(&mut v, &mut all_destroyed) });
            i.prefix_inc();
        }
    }
    assert!(
        (unsafe {
            let ___first: UnsafeRandomAccessIterator<bool> = all_destroyed.begin();
            let ___last: UnsafeRandomAccessIterator<bool> = all_destroyed.end();
            all_of_5(
                ___first,
                ___last,
                FnPtr::<fn(bool) -> bool>::new(|mut b: bool| -> bool {
                    unsafe {
                        return b;
                    }
                }),
            )
        })
    );
}
pub unsafe fn test_6(mut operation: FnPtr<fn(*mut Vec<DestroyTracker>, *mut Vec<bool>)>) {
    let mut all_destroyed: Vec<bool> = Vec::new();
    {
        let mut v: Vec<DestroyTracker> = Vec::new();
        let mut i: usize = 0_usize;
        'loop_: while ((i) != (100_usize)) {
            (unsafe { operation.call(&mut v, &mut all_destroyed) });
            i.prefix_inc();
        }
    }
    assert!(
        (unsafe {
            let ___first: UnsafeRandomAccessIterator<bool> = all_destroyed.begin();
            let ___last: UnsafeRandomAccessIterator<bool> = all_destroyed.end();
            all_of_7(
                ___first,
                ___last,
                FnPtr::<fn(bool) -> bool>::new(|mut b: bool| -> bool {
                    unsafe {
                        return b;
                    }
                }),
            )
        })
    );
}
pub unsafe fn test_8(mut operation: FnPtr<fn(*mut Vec<DestroyTracker>, *mut Vec<bool>)>) {
    let mut all_destroyed: Vec<bool> = Vec::new();
    {
        let mut v: Vec<DestroyTracker> = Vec::new();
        let mut i: usize = 0_usize;
        'loop_: while ((i) != (100_usize)) {
            (unsafe { operation.call(&mut v, &mut all_destroyed) });
            i.prefix_inc();
        }
    }
    assert!(
        (unsafe {
            let ___first: UnsafeRandomAccessIterator<bool> = all_destroyed.begin();
            let ___last: UnsafeRandomAccessIterator<bool> = all_destroyed.end();
            all_of_9(
                ___first,
                ___last,
                FnPtr::<fn(bool) -> bool>::new(|mut b: bool| -> bool {
                    unsafe {
                        return b;
                    }
                }),
            )
        })
    );
}
pub unsafe fn test_10(mut operation: FnPtr<fn(*mut Vec<DestroyTracker>, *mut Vec<bool>)>) {
    let mut all_destroyed: Vec<bool> = Vec::new();
    {
        let mut v: Vec<DestroyTracker> = Vec::new();
        let mut i: usize = 0_usize;
        'loop_: while ((i) != (100_usize)) {
            (unsafe { operation.call(&mut v, &mut all_destroyed) });
            i.prefix_inc();
        }
    }
    assert!(
        (unsafe {
            let ___first: UnsafeRandomAccessIterator<bool> = all_destroyed.begin();
            let ___last: UnsafeRandomAccessIterator<bool> = all_destroyed.end();
            all_of_11(
                ___first,
                ___last,
                FnPtr::<fn(bool) -> bool>::new(|mut b: bool| -> bool {
                    unsafe {
                        return b;
                    }
                }),
            )
        })
    );
}
pub unsafe fn test_12(mut operation: FnPtr<fn(*mut Vec<DestroyTracker>, *mut Vec<bool>)>) {
    let mut all_destroyed: Vec<bool> = Vec::new();
    {
        let mut v: Vec<DestroyTracker> = Vec::new();
        let mut i: usize = 0_usize;
        'loop_: while ((i) != (100_usize)) {
            (unsafe { operation.call(&mut v, &mut all_destroyed) });
            i.prefix_inc();
        }
    }
    assert!(
        (unsafe {
            let ___first: UnsafeRandomAccessIterator<bool> = all_destroyed.begin();
            let ___last: UnsafeRandomAccessIterator<bool> = all_destroyed.end();
            all_of_13(
                ___first,
                ___last,
                FnPtr::<fn(bool) -> bool>::new(|mut b: bool| -> bool {
                    unsafe {
                        return b;
                    }
                }),
            )
        })
    );
}
pub unsafe fn test_14() -> bool {
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
        test_2(FnPtr::<fn(*mut Vec<DestroyTracker>, *mut Vec<bool>)>::new(
            |vec_: *mut Vec<DestroyTracker>, tracker: *mut Vec<bool>| unsafe {
                {
                    let __a1 = DestroyTracker::new({ tracker });
                    (*vec_).push(__a1)
                };
            },
        ))
    });
    (unsafe {
        test_4(FnPtr::<fn(*mut Vec<DestroyTracker>, *mut Vec<bool>)>::new(
            |vec_: *mut Vec<DestroyTracker>, tracker: *mut Vec<bool>| unsafe {
                (unsafe {
                    let ___position: UnsafeRandomAccessIterator<DestroyTracker> =
                        <UnsafeRandomAccessIterator<DestroyTracker>>::from((*vec_).begin());
                    let ___args: *mut Vec<bool> = tracker;
                    (*vec_).emplace(___position, ___args)
                });
            },
        ))
    });
    (unsafe {
        test_6(FnPtr::<fn(*mut Vec<DestroyTracker>, *mut Vec<bool>)>::new(
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
        test_8(FnPtr::<fn(*mut Vec<DestroyTracker>, *mut Vec<bool>)>::new(
            |vec_: *mut Vec<DestroyTracker>, tracker: *mut Vec<bool>| unsafe {
                {
                    let __a0 = ((*vec_).len()).wrapping_add(1_usize) as usize;
                    (*vec_).resize(__a0, DestroyTracker::new({ tracker }))
                };
            },
        ))
    });
    (unsafe {
        test_10(FnPtr::<fn(*mut Vec<DestroyTracker>, *mut Vec<bool>)>::new(
            |vec_: *mut Vec<DestroyTracker>, tracker: *mut Vec<bool>| unsafe {
                (unsafe {
                    let ___pos: UnsafeRandomAccessIterator<DestroyTracker> =
                        <UnsafeRandomAccessIterator<DestroyTracker>>::from((*vec_).begin());
                    let mut ___rg: Vec<DestroyTracker> = Vec::from([
                        DestroyTracker::new({ tracker }),
                        DestroyTracker::new({ tracker }),
                    ]);
                    (*vec_).insert_range(___pos, &mut ___rg)
                });
            },
        ))
    });
    (unsafe {
        test_12(FnPtr::<fn(*mut Vec<DestroyTracker>, *mut Vec<bool>)>::new(
            |vec_: *mut Vec<DestroyTracker>, tracker: *mut Vec<bool>| unsafe {
                (unsafe {
                    let mut ___rg: Vec<DestroyTracker> = Vec::from([
                        DestroyTracker::new({ tracker }),
                        DestroyTracker::new({ tracker }),
                    ]);
                    (*vec_).append_range(&mut ___rg)
                });
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
    (unsafe { test_14() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
