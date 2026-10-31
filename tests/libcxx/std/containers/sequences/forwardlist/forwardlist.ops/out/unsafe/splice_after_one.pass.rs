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
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_3)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_0)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)) = 0_usize;
    }
}
pub static mut outstanding_bytes_0: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_1: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_2: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_3: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut i32,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
pub static mut t1_4: std::cell::LazyCell<[i32; 8]> =
    std::cell::LazyCell::new(|| unsafe { [0, 1, 2, 3, 4, 5, 6, 7] });
pub static mut t2_5: std::cell::LazyCell<[i32; 3]> =
    std::cell::LazyCell::new(|| unsafe { [10, 11, 12] });
pub static mut size_t1_6: std::cell::LazyCell<isize> = std::cell::LazyCell::new(|| unsafe {
    ((((*std::cell::LazyCell::force_mut(&mut *&raw mut t1_4))
        .as_ptr()
        .add((*std::cell::LazyCell::force_mut(&mut *&raw mut t1_4)).len()) as usize
        - unimplemented!() as usize)
        / ::std::mem::size_of::<i32>()) as i64 as isize)
});
pub static mut size_t2_7: std::cell::LazyCell<isize> = std::cell::LazyCell::new(|| unsafe {
    ((((*std::cell::LazyCell::force_mut(&mut *&raw mut t2_5))
        .as_ptr()
        .add((*std::cell::LazyCell::force_mut(&mut *&raw mut t2_5)).len()) as usize
        - unimplemented!() as usize)
        / ::std::mem::size_of::<i32>()) as i64 as isize)
});
pub unsafe fn testd_8(c: *const (), mut p: i32, mut f: i32) {
    let mut i: () = unimplemented!();
    let mut n1: i32 = 0;
    'loop_: while ((n1) < (p)) {
        assert!(
            ((*i.iter_deref())
                == ((*std::cell::LazyCell::force_mut(&mut *&raw mut t1_4))[(n1) as usize]))
        );
        {
            n1.prefix_inc();
            i.iter_inc()
        };
    }
    let mut n2: i32 = f;
    'loop_: while ((n2) < ((f) + (1))) {
        assert!(
            ((*i.iter_deref())
                == ((*std::cell::LazyCell::force_mut(&mut *&raw mut t2_5))[(n2) as usize]))
        );
        {
            n2.prefix_inc();
            i.iter_inc()
        };
    }
    'loop_: while ((n1 as isize) < (*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_6))) {
        assert!(
            ((*i.iter_deref())
                == ((*std::cell::LazyCell::force_mut(&mut *&raw mut t1_4))[(n1) as usize]))
        );
        {
            n1.prefix_inc();
            i.iter_inc()
        };
    }
    assert!(
        ((unimplemented!())
            == ((*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_6)) + (1_isize)))
    );
}
pub unsafe fn testd_9(c: *const (), mut p: i32, mut f: i32) {
    let mut i: () = unimplemented!();
    let mut n1: i32 = 0;
    'loop_: while ((n1) < (p)) {
        assert!(
            ((*i.iter_deref())
                == ((*std::cell::LazyCell::force_mut(&mut *&raw mut t1_4))[(n1) as usize]))
        );
        {
            n1.prefix_inc();
            i.iter_inc()
        };
    }
    let mut n2: i32 = f;
    'loop_: while ((n2) < ((f) + (1))) {
        assert!(
            ((*i.iter_deref())
                == ((*std::cell::LazyCell::force_mut(&mut *&raw mut t2_5))[(n2) as usize]))
        );
        {
            n2.prefix_inc();
            i.iter_inc()
        };
    }
    'loop_: while ((n1 as isize) < (*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_6))) {
        assert!(
            ((*i.iter_deref())
                == ((*std::cell::LazyCell::force_mut(&mut *&raw mut t1_4))[(n1) as usize]))
        );
        {
            n1.prefix_inc();
            i.iter_inc()
        };
    }
    assert!(
        ((unimplemented!())
            == ((*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_6)) + (1_isize)))
    );
}
pub unsafe fn tests_10(c: *const (), mut p: i32, mut f: i32) {
    let mut i: () = unimplemented!();
    let mut n: i32 = 0;
    if ((p) == (f)) || ((p) == ((f) + (1))) {
        n = 0;
        'loop_: while ((n as isize) < (*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_6))) {
            assert!(
                ((*i.iter_deref())
                    == ((*std::cell::LazyCell::force_mut(&mut *&raw mut t1_4))[(n) as usize]))
            );
            {
                n.prefix_inc();
                i.iter_inc()
            };
        }
    } else if ((p) < (f)) {
        n = 0;
        'loop_: while ((n) < (p)) {
            assert!(
                ((*i.iter_deref())
                    == ((*std::cell::LazyCell::force_mut(&mut *&raw mut t1_4))[(n) as usize]))
            );
            {
                n.prefix_inc();
                i.iter_inc()
            };
        }
        n = f;
        'loop_: while ((n) < ((f) + (1))) {
            assert!(
                ((*i.iter_deref())
                    == ((*std::cell::LazyCell::force_mut(&mut *&raw mut t1_4))[(n) as usize]))
            );
            {
                n.prefix_inc();
                i.iter_inc()
            };
        }
        n = p;
        'loop_: while ((n) < (f)) {
            assert!(
                ((*i.iter_deref())
                    == ((*std::cell::LazyCell::force_mut(&mut *&raw mut t1_4))[(n) as usize]))
            );
            {
                n.prefix_inc();
                i.iter_inc()
            };
        }
        n = ((f) + (1));
        'loop_: while ((n as isize) < (*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_6))) {
            assert!(
                ((*i.iter_deref())
                    == ((*std::cell::LazyCell::force_mut(&mut *&raw mut t1_4))[(n) as usize]))
            );
            {
                n.prefix_inc();
                i.iter_inc()
            };
        }
    } else {
        n = 0;
        'loop_: while ((n) < (f)) {
            assert!(
                ((*i.iter_deref())
                    == ((*std::cell::LazyCell::force_mut(&mut *&raw mut t1_4))[(n) as usize]))
            );
            {
                n.prefix_inc();
                i.iter_inc()
            };
        }
        n = ((f) + (1));
        'loop_: while ((n) < (p)) {
            assert!(
                ((*i.iter_deref())
                    == ((*std::cell::LazyCell::force_mut(&mut *&raw mut t1_4))[(n) as usize]))
            );
            {
                n.prefix_inc();
                i.iter_inc()
            };
        }
        n = f;
        'loop_: while ((n) < ((f) + (1))) {
            assert!(
                ((*i.iter_deref())
                    == ((*std::cell::LazyCell::force_mut(&mut *&raw mut t1_4))[(n) as usize]))
            );
            {
                n.prefix_inc();
                i.iter_inc()
            };
        }
        n = p;
        'loop_: while ((n as isize) < (*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_6))) {
            assert!(
                ((*i.iter_deref())
                    == ((*std::cell::LazyCell::force_mut(&mut *&raw mut t1_4))[(n) as usize]))
            );
            {
                n.prefix_inc();
                i.iter_inc()
            };
        }
    }
    assert!(((unimplemented!()) == (*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_6))));
}
pub unsafe fn tests_11(c: *const (), mut p: i32, mut f: i32) {
    let mut i: () = unimplemented!();
    let mut n: i32 = 0;
    if ((p) == (f)) || ((p) == ((f) + (1))) {
        n = 0;
        'loop_: while ((n as isize) < (*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_6))) {
            assert!(
                ((*i.iter_deref())
                    == ((*std::cell::LazyCell::force_mut(&mut *&raw mut t1_4))[(n) as usize]))
            );
            {
                n.prefix_inc();
                i.iter_inc()
            };
        }
    } else if ((p) < (f)) {
        n = 0;
        'loop_: while ((n) < (p)) {
            assert!(
                ((*i.iter_deref())
                    == ((*std::cell::LazyCell::force_mut(&mut *&raw mut t1_4))[(n) as usize]))
            );
            {
                n.prefix_inc();
                i.iter_inc()
            };
        }
        n = f;
        'loop_: while ((n) < ((f) + (1))) {
            assert!(
                ((*i.iter_deref())
                    == ((*std::cell::LazyCell::force_mut(&mut *&raw mut t1_4))[(n) as usize]))
            );
            {
                n.prefix_inc();
                i.iter_inc()
            };
        }
        n = p;
        'loop_: while ((n) < (f)) {
            assert!(
                ((*i.iter_deref())
                    == ((*std::cell::LazyCell::force_mut(&mut *&raw mut t1_4))[(n) as usize]))
            );
            {
                n.prefix_inc();
                i.iter_inc()
            };
        }
        n = ((f) + (1));
        'loop_: while ((n as isize) < (*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_6))) {
            assert!(
                ((*i.iter_deref())
                    == ((*std::cell::LazyCell::force_mut(&mut *&raw mut t1_4))[(n) as usize]))
            );
            {
                n.prefix_inc();
                i.iter_inc()
            };
        }
    } else {
        n = 0;
        'loop_: while ((n) < (f)) {
            assert!(
                ((*i.iter_deref())
                    == ((*std::cell::LazyCell::force_mut(&mut *&raw mut t1_4))[(n) as usize]))
            );
            {
                n.prefix_inc();
                i.iter_inc()
            };
        }
        n = ((f) + (1));
        'loop_: while ((n) < (p)) {
            assert!(
                ((*i.iter_deref())
                    == ((*std::cell::LazyCell::force_mut(&mut *&raw mut t1_4))[(n) as usize]))
            );
            {
                n.prefix_inc();
                i.iter_inc()
            };
        }
        n = f;
        'loop_: while ((n) < ((f) + (1))) {
            assert!(
                ((*i.iter_deref())
                    == ((*std::cell::LazyCell::force_mut(&mut *&raw mut t1_4))[(n) as usize]))
            );
            {
                n.prefix_inc();
                i.iter_inc()
            };
        }
        n = p;
        'loop_: while ((n as isize) < (*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_6))) {
            assert!(
                ((*i.iter_deref())
                    == ((*std::cell::LazyCell::force_mut(&mut *&raw mut t1_4))[(n) as usize]))
            );
            {
                n.prefix_inc();
                i.iter_inc()
            };
        }
    }
    assert!(((unimplemented!()) == (*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_6))));
}
pub unsafe fn test_12() -> bool {
    {
        let mut f: i32 = 0;
        'loop_: while ((f as isize)
            <= ((*std::cell::LazyCell::force_mut(&mut *&raw mut size_t2_7)) - (1_isize)))
        {
            let mut p: i32 = 0;
            'loop_: while ((p as isize)
                <= (*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_6)))
            {
                let mut c1: () = unimplemented!();
                let mut c2: () = unimplemented!();
                unimplemented!();
                (unsafe { testd_8(&c1, p, f) });
                p.prefix_inc();
            }
            f.prefix_inc();
        }
        let mut f: i32 = 0;
        'loop_: while ((f as isize)
            <= ((*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_6)) - (1_isize)))
        {
            let mut p: i32 = 0;
            'loop_: while ((p as isize)
                <= (*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_6)))
            {
                let mut c1: () = unimplemented!();
                unimplemented!();
                (unsafe { tests_10(&c1, p, f) });
                p.prefix_inc();
            }
            f.prefix_inc();
        }
    }
    {
        let mut f: i32 = 0;
        'loop_: while ((f as isize)
            <= ((*std::cell::LazyCell::force_mut(&mut *&raw mut size_t2_7)) - (1_isize)))
        {
            let mut p: i32 = 0;
            'loop_: while ((p as isize)
                <= (*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_6)))
            {
                let mut c1: () = unimplemented!();
                let mut c2: () = unimplemented!();
                unimplemented!();
                (unsafe { testd_9(&c1, p, f) });
                p.prefix_inc();
            }
            f.prefix_inc();
        }
        let mut f: i32 = 0;
        'loop_: while ((f as isize)
            <= ((*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_6)) - (1_isize)))
        {
            let mut p: i32 = 0;
            'loop_: while ((p as isize)
                <= (*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_6)))
            {
                let mut c1: () = unimplemented!();
                unimplemented!();
                (unsafe { tests_11(&c1, p, f) });
                p.prefix_inc();
            }
            f.prefix_inc();
        }
    }
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
    assert!((unsafe { test_12() }));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
    std::cell::LazyCell::force(&*&raw const t1_4);
    std::cell::LazyCell::force(&*&raw const t2_5);
    std::cell::LazyCell::force(&*&raw const size_t1_6);
    std::cell::LazyCell::force(&*&raw const size_t2_7);
}
