extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn cmpxchg_weak_loop_0(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_bool__false_).compare_exchange_weak_15(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_1(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_bool__false_).compare_exchange_weak_16(___e, ___d, None)
        }) == (true))
        {
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
) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            let ___s: () = success;
            let ___f: () = failure;
            ((*atomic) as std___atomic_base_bool__false_)
                .compare_exchange_weak_11(___e, ___d, ___s, ___f)
        }) == (true))
        {
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
) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            let ___s: () = success;
            let ___f: () = failure;
            ((*atomic) as std___atomic_base_bool__false_)
                .compare_exchange_weak_12(___e, ___d, ___s, ___f)
        }) == (true))
        {
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    {
        let mut obj: () = unimplemented!();
        unimplemented!();
        {
            let mut lockfree: () =
                (unsafe { (obj as std___atomic_base_bool__false_).is_lock_free_1() });
            &(lockfree);
            {
                unimplemented!();
            }
        }
        (unsafe { (obj as std___atomic_base_bool__false_).store_3(false, None) });
        unimplemented!();
        (unsafe {
            (obj as std___atomic_base_bool__false_).store_3(
                true,
                Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_4))),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut x: () = (unsafe { (obj as std___atomic_base_bool__false_).to__7() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { (obj as std___atomic_base_bool__false_).store_3(true, None) });
        x = true;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        x = true;
        (unsafe { (obj as std___atomic_base_bool__false_).store_3(true, None) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        x = true;
        (unsafe { (obj as std___atomic_base_bool__false_).store_3(true, None) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut obj: () = unimplemented!();
        unimplemented!();
        {
            let mut lockfree: () = unimplemented!();
            &(lockfree);
            {
                unimplemented!();
            }
        }
        (unsafe { (obj as std___atomic_base_bool__false_).store_4(false, None) });
        unimplemented!();
        (unsafe {
            (obj as std___atomic_base_bool__false_).store_4(
                true,
                Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_4))),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut x: () = (unsafe { (obj as std___atomic_base_bool__false_).to__8() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { (obj as std___atomic_base_bool__false_).store_4(true, None) });
        x = true;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        x = true;
        (unsafe { (obj as std___atomic_base_bool__false_).store_4(true, None) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        x = true;
        (unsafe { (obj as std___atomic_base_bool__false_).store_4(true, None) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut obj: () = unimplemented!();
        unimplemented!();
        {
            let mut lockfree: () = unimplemented!();
            &(lockfree);
            {
                unimplemented!();
            }
        }
        (unsafe { (obj as std___atomic_base_bool__false_).store_4(false, None) });
        unimplemented!();
        (unsafe {
            (obj as std___atomic_base_bool__false_).store_4(
                true,
                Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_4))),
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut x: () = (unsafe { (obj as std___atomic_base_bool__false_).to__8() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { (obj as std___atomic_base_bool__false_).store_4(true, None) });
        x = true;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        x = true;
        (unsafe { (obj as std___atomic_base_bool__false_).store_4(true, None) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        x = true;
        (unsafe { (obj as std___atomic_base_bool__false_).store_4(true, None) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut storage: [(); 1] = [1_()];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
