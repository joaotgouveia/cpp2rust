extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_node_container_0(mut conv: FnPtr<fn(i32) -> i32>) {
    let mut c: () = unimplemented!();
    let mut i: i32 = 0;
    'loop_: while ((i) != (10)) {
        unimplemented!();
        i.prefix_inc();
    }
    {
        let mut i: i32 = 0;
        'loop_: while ((i) != (10)) {
            let mut invoke_count: i32 = i;
            unimplemented!();
            assert!(((invoke_count) == (10)));
            i.prefix_inc();
        }
    }
    {
        let mut i: i32 = 0;
        'loop_: while ((i) != (10)) {
            let mut invoke_count: i32 = 0;
            unimplemented!();
            assert!(((invoke_count) == ((10) - (i))));
            i.prefix_inc();
        }
    }
    {
        {
            let mut invoke_count: i32 = 0;
            unimplemented!();
            assert!(((invoke_count) == (0)));
        }
        {
            let mut invoke_count: i32 = 0;
            unimplemented!();
            assert!(((invoke_count) == (0)));
        }
    }
    {
        let mut invoke_count: i32 = 0;
        unimplemented!();
        assert!(((invoke_count) == (1)));
    }
}
pub unsafe fn test_node_container_1(mut conv: FnPtr<fn(i32) -> i32>) {
    let mut c: () = unimplemented!();
    let mut i: i32 = 0;
    'loop_: while ((i) != (10)) {
        unimplemented!();
        i.prefix_inc();
    }
    {
        let mut i: i32 = 0;
        'loop_: while ((i) != (10)) {
            let mut invoke_count: i32 = i;
            unimplemented!();
            assert!(((invoke_count) == (10)));
            i.prefix_inc();
        }
    }
    {
        let mut i: i32 = 0;
        'loop_: while ((i) != (10)) {
            let mut invoke_count: i32 = 0;
            unimplemented!();
            assert!(((invoke_count) == ((10) - (i))));
            i.prefix_inc();
        }
    }
    {
        {
            let mut invoke_count: i32 = 0;
            unimplemented!();
            assert!(((invoke_count) == (0)));
        }
        {
            let mut invoke_count: i32 = 0;
            unimplemented!();
            assert!(((invoke_count) == (0)));
        }
    }
    {
        let mut invoke_count: i32 = 0;
        unimplemented!();
        assert!(((invoke_count) == (1)));
    }
}
pub unsafe fn test_node_container_2(mut conv: FnPtr<fn(i32) -> (i32, i32)>) {
    let mut c: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
    let mut i: i32 = 0;
    'loop_: while ((i) != (10)) {
        unimplemented!();
        i.prefix_inc();
    }
    {
        let mut i: i32 = 0;
        'loop_: while ((i) != (10)) {
            let mut invoke_count: i32 = i;
            unimplemented!();
            assert!(((invoke_count) == (10)));
            i.prefix_inc();
        }
    }
    {
        let mut i: i32 = 0;
        'loop_: while ((i) != (10)) {
            let mut invoke_count: i32 = 0;
            unimplemented!();
            assert!(((invoke_count) == ((10) - (i))));
            i.prefix_inc();
        }
    }
    {
        {
            let mut invoke_count: i32 = 0;
            unimplemented!();
            assert!(((invoke_count) == (0)));
        }
        {
            let mut invoke_count: i32 = 0;
            unimplemented!();
            assert!(((invoke_count) == (0)));
        }
    }
    {
        let mut invoke_count: i32 = 0;
        unimplemented!();
        assert!(((invoke_count) == (1)));
    }
}
pub unsafe fn test_node_container_3(mut conv: FnPtr<fn(i32) -> (i32, i32)>) {
    let mut c: () = unimplemented!();
    let mut i: i32 = 0;
    'loop_: while ((i) != (10)) {
        unimplemented!();
        i.prefix_inc();
    }
    {
        let mut i: i32 = 0;
        'loop_: while ((i) != (10)) {
            let mut invoke_count: i32 = i;
            unimplemented!();
            assert!(((invoke_count) == (10)));
            i.prefix_inc();
        }
    }
    {
        let mut i: i32 = 0;
        'loop_: while ((i) != (10)) {
            let mut invoke_count: i32 = 0;
            unimplemented!();
            assert!(((invoke_count) == ((10) - (i))));
            i.prefix_inc();
        }
    }
    {
        {
            let mut invoke_count: i32 = 0;
            unimplemented!();
            assert!(((invoke_count) == (0)));
        }
        {
            let mut invoke_count: i32 = 0;
            unimplemented!();
            assert!(((invoke_count) == (0)));
        }
    }
    {
        let mut invoke_count: i32 = 0;
        unimplemented!();
        assert!(((invoke_count) == (1)));
    }
}
pub unsafe fn test_4() -> bool {
    (unsafe {
        test_node_container_0(FnPtr::<fn(i32) -> i32>::new(|mut i: i32| -> i32 {
            unsafe {
                return i;
            }
        }))
    });
    (unsafe {
        test_node_container_1(FnPtr::<fn(i32) -> i32>::new(|mut i: i32| -> i32 {
            unsafe {
                return i;
            }
        }))
    });
    (unsafe {
        test_node_container_2(FnPtr::<fn(i32) -> (i32, i32)>::new(
            |mut i: i32| -> (i32, i32) {
                unsafe {
                    return (i, i);
                }
            },
        ))
    });
    (unsafe {
        test_node_container_3(FnPtr::<fn(i32) -> (i32, i32)>::new(
            |mut i: i32| -> (i32, i32) {
                unsafe {
                    return (i, i);
                }
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
    (unsafe { test_4() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
