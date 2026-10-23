extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_node_container_0(mut conv: FnPtr<fn(()) -> ()>) {
    let mut c: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) != (10_())) {
        unimplemented!();
        i.prefix_inc();
    }
    {
        let mut i: () = 0_();
        'loop_: while ((i) != (10_())) {
            let mut invoke_count: () = i;
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let mut i: () = 0_();
        'loop_: while ((i) != (10_())) {
            let mut invoke_count: () = 0_();
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        {
            let mut invoke_count: () = 0_();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut invoke_count: () = 0_();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut invoke_count: () = 0_();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_node_container_1(mut conv: FnPtr<fn(()) -> ()>) {
    let mut c: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) != (10_())) {
        unimplemented!();
        i.prefix_inc();
    }
    {
        let mut i: () = 0_();
        'loop_: while ((i) != (10_())) {
            let mut invoke_count: () = i;
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let mut i: () = 0_();
        'loop_: while ((i) != (10_())) {
            let mut invoke_count: () = 0_();
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        {
            let mut invoke_count: () = 0_();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut invoke_count: () = 0_();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut invoke_count: () = 0_();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_node_container_2(mut conv: FnPtr<fn(()) -> ()>) {
    let mut c: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) != (10_())) {
        unimplemented!();
        i.prefix_inc();
    }
    {
        let mut i: () = 0_();
        'loop_: while ((i) != (10_())) {
            let mut invoke_count: () = i;
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let mut i: () = 0_();
        'loop_: while ((i) != (10_())) {
            let mut invoke_count: () = 0_();
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        {
            let mut invoke_count: () = 0_();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut invoke_count: () = 0_();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut invoke_count: () = 0_();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_node_container_3(mut conv: FnPtr<fn(()) -> ()>) {
    let mut c: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) != (10_())) {
        unimplemented!();
        i.prefix_inc();
    }
    {
        let mut i: () = 0_();
        'loop_: while ((i) != (10_())) {
            let mut invoke_count: () = i;
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let mut i: () = 0_();
        'loop_: while ((i) != (10_())) {
            let mut invoke_count: () = 0_();
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        {
            let mut invoke_count: () = 0_();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut invoke_count: () = 0_();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut invoke_count: () = 0_();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_4() -> () {
    (unsafe {
        test_node_container_0(FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
            unsafe {
                return i;
            }
        }))
    });
    (unsafe {
        test_node_container_1(FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
            unsafe {
                return i;
            }
        }))
    });
    (unsafe {
        test_node_container_2(FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
            unsafe {
                return unimplemented!();
            }
        }))
    });
    (unsafe {
        test_node_container_3(FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
            unsafe {
                return unimplemented!();
            }
        }))
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
    (unsafe { test_4() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
