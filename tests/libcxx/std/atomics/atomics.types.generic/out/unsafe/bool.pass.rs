extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn cmpxchg_weak_loop_0(atomic: *mut (), expected: *mut (), mut desired: ()) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_1(atomic: *mut (), expected: *mut (), mut desired: ()) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_2(
    atomic: *mut (),
    expected: *mut (),
    mut desired: (),
    mut success: (),
    mut failure: (),
) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_3(
    atomic: *mut (),
    expected: *mut (),
    mut desired: (),
    mut success: (),
    mut failure: (),
) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
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
        let mut obj: () = unimplemented!();
        assert!(((unimplemented!() as i32) == (true as i32)));
        {
            let mut lockfree: bool = unimplemented!();
            &(lockfree);
            {
                assert!(lockfree);
            }
        }
        unimplemented!();
        assert!(((unimplemented!() as i32) == (false as i32)));
        unimplemented!();
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        let mut x: bool = unimplemented!();
        assert!(
            (((unsafe {
                cmpxchg_weak_loop_0(&mut obj, &mut *(&raw mut x).cast::<()>(), (false as ()))
            }) as i32)
                == (true as i32))
        );
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((x as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((x as i32) == (false as i32)));
        unimplemented!();
        x = true;
        assert!(
            (((unsafe {
                let _success: () = unimplemented!();
                let _failure: () = unimplemented!();
                cmpxchg_weak_loop_2(
                    &mut obj,
                    &mut *(&raw mut x).cast::<()>(),
                    (false as ()),
                    _success,
                    _failure,
                )
            }) as i32)
                == (true as i32))
        );
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((x as i32) == (true as i32)));
        x = true;
        unimplemented!();
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((x as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((x as i32) == (false as i32)));
        x = true;
        unimplemented!();
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((x as i32) == (true as i32)));
        assert!((((unimplemented!()) as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!((((unimplemented!()) as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
    }
    {
        let mut obj: () = unimplemented!();
        assert!(((unimplemented!() as i32) == (true as i32)));
        {
            let mut lockfree: bool = unimplemented!();
            &(lockfree);
            {
                assert!(lockfree);
            }
        }
        unimplemented!();
        assert!(((unimplemented!() as i32) == (false as i32)));
        unimplemented!();
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        let mut x: bool = unimplemented!();
        assert!(
            (((unsafe {
                cmpxchg_weak_loop_1(&mut obj, &mut *(&raw mut x).cast::<()>(), (false as ()))
            }) as i32)
                == (true as i32))
        );
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((x as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((x as i32) == (false as i32)));
        unimplemented!();
        x = true;
        assert!(
            (((unsafe {
                let _success: () = unimplemented!();
                let _failure: () = unimplemented!();
                cmpxchg_weak_loop_3(
                    &mut obj,
                    &mut *(&raw mut x).cast::<()>(),
                    (false as ()),
                    _success,
                    _failure,
                )
            }) as i32)
                == (true as i32))
        );
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((x as i32) == (true as i32)));
        x = true;
        unimplemented!();
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((x as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((x as i32) == (false as i32)));
        x = true;
        unimplemented!();
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((x as i32) == (true as i32)));
        assert!((((unimplemented!()) as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!((((unimplemented!()) as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
    }
    {
        let mut obj: () = unimplemented!();
        assert!(((unimplemented!() as i32) == (true as i32)));
        {
            let mut lockfree: bool = unimplemented!();
            &(lockfree);
            {
                assert!(lockfree);
            }
        }
        unimplemented!();
        assert!(((unimplemented!() as i32) == (false as i32)));
        unimplemented!();
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        let mut x: bool = unimplemented!();
        assert!(
            (((unsafe {
                cmpxchg_weak_loop_1(&mut obj, &mut *(&raw mut x).cast::<()>(), (false as ()))
            }) as i32)
                == (true as i32))
        );
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((x as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((x as i32) == (false as i32)));
        unimplemented!();
        x = true;
        assert!(
            (((unsafe {
                let _success: () = unimplemented!();
                let _failure: () = unimplemented!();
                cmpxchg_weak_loop_3(
                    &mut obj,
                    &mut *(&raw mut x).cast::<()>(),
                    (false as ()),
                    _success,
                    _failure,
                )
            }) as i32)
                == (true as i32))
        );
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((x as i32) == (true as i32)));
        x = true;
        unimplemented!();
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((x as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((x as i32) == (false as i32)));
        x = true;
        unimplemented!();
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((x as i32) == (true as i32)));
        assert!((((unimplemented!()) as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!((((unimplemented!()) as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
    }
    {
        let mut storage: [libc::c_char; 1] = [(1 as libc::c_char)];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        assert!(((unimplemented!() as i32) == (false as i32)));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
