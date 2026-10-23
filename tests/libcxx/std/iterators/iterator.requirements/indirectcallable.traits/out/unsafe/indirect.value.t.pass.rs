extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() {
    let mut ints: () = (unsafe {
        let mut ___start: () = 0_();
        let mut ___bound_sentinel: () = 5_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_1))(
            &mut ___start,
            &mut ___bound_sentinel,
        )
    });
    let mut unique_ptr_maker: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |mut v: ()| -> () {
            return unimplemented!();
        },
        |mut v: ()| -> () {
            return unimplemented!();
        }
    );
    {
        let mut consume: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: ()| {});
        const _: () = assert!(
            true,
            "std::indirectly_unary_invocable<decltype(consume), projected_iter>"
        );;
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut for_each_2))(
                &mut ints,
                consume.copy_from(),
                unique_ptr_maker.copy_from(),
            )
        });
        (unsafe {
            let ___first: () = unimplemented!();
            let ___last: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut for_each_2))(
                ___first,
                ___last,
                consume.copy_from(),
                unique_ptr_maker.copy_from(),
            )
        });
    }
    {
        const _: () = assert!(
            true,
            "std::indirectly_regular_unary_invocable<decltype([](auto) {{}}), projected_iter>"
        );;
    }
    {
        let mut unary_pred: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: ()| -> () {
            return false;
        });
        const _: () = assert!(
            true,
            "std::indirect_unary_predicate<decltype(unary_pred), projected_iter>"
        );;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut binary_pred: FnPtr<Generic> =
            lambda_unsafe!(Generic, {}, |mut _a0: (), mut _a1: ()| -> () {
                return false;
            });
        const _: () = assert!(
            true,
            "std::indirect_binary_predicate<decltype(binary_pred), projected_iter, projected_iter>"
        );;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut rel: FnPtr<Generic> =
            lambda_unsafe!(Generic, {}, |mut _a0: (), mut _a1: ()| -> () {
                return false;
            });
        const _: () = assert!(
            true,
            "std::indirect_equivalence_relation<decltype(rel), projected_iter>"
        );;
        let mut out: () = unimplemented!();
        &(unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut unique_copy_3))(
                &mut ints,
                unimplemented!(),
                rel.copy_from(),
                unique_ptr_maker.copy_from(),
            )
        });
        &(unsafe {
            let ___first: () = unimplemented!();
            let ___last: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut unique_copy_3))(
                ___first,
                ___last,
                unimplemented!(),
                rel.copy_from(),
                unique_ptr_maker.copy_from(),
            )
        });
    }
    {
        let mut rel: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut x: (), mut y: ()| -> () {
            return ((*x) < (*y));
        });
        const _: () = assert!(
            true,
            "std::indirect_strict_weak_order<decltype(rel), projected_iter>"
        );;
        unimplemented!();
        unimplemented!();
    }
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
    (unsafe { test_0() });
    const _: () = assert!(true, "(test(), true)");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
