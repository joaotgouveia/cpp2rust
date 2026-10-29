extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(mut c1: (), mut j: (), mut x1: (), mut x2: (), mut x3: (), c2: *const ()) {
    let mut q : () = std_insert_iterator_std_vector_std_unique_ptr_int__do_nothing___std_allocator_std_unique_ptr_int__do_nothing____ :: new ( { & mut c1 } , { unimplemented!()  } , )  ;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn insert3at_1(c: *mut (), mut i: (), mut x1: (), mut x2: (), mut x3: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct do_nothing {}
impl do_nothing {
    pub unsafe fn operator_call(&self, mut _a0: ()) {}
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
        let mut c1: () = unimplemented!();
        let mut x: [(); 6] = [
            0_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut i: () = 0_();
        'loop_: while ((i) < (3_())) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut c2: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (3_())) {
            unimplemented!();
            i.prefix_inc();
        }
        (unsafe {
            let _c: *mut () = &mut c2;
            let _i: () = unimplemented!();
            let _x1: () = unimplemented!();
            let _x2: () = unimplemented!();
            let _x3: () = unimplemented!();
            insert3at_1(_c, _i, _x1, _x2, _x3)
        });
        (unsafe {
            let _x1: () = unimplemented!();
            let _x2: () = unimplemented!();
            let _x3: () = unimplemented!();
            test_0(unimplemented!(), 0_(), _x1, _x2, _x3, &c2)
        });
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (3_())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (3_())) {
            unimplemented!();
            i.prefix_inc();
        }
        (unsafe {
            let _c: *mut () = &mut c2;
            let _i: () = unimplemented!();
            let _x1: () = unimplemented!();
            let _x2: () = unimplemented!();
            let _x3: () = unimplemented!();
            insert3at_1(_c, _i, _x1, _x2, _x3)
        });
        (unsafe {
            let _x1: () = unimplemented!();
            let _x2: () = unimplemented!();
            let _x3: () = unimplemented!();
            test_0(unimplemented!(), 1_(), _x1, _x2, _x3, &c2)
        });
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (3_())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (3_())) {
            unimplemented!();
            i.prefix_inc();
        }
        (unsafe {
            let _c: *mut () = &mut c2;
            let _i: () = unimplemented!();
            let _x1: () = unimplemented!();
            let _x2: () = unimplemented!();
            let _x3: () = unimplemented!();
            insert3at_1(_c, _i, _x1, _x2, _x3)
        });
        (unsafe {
            let _x1: () = unimplemented!();
            let _x2: () = unimplemented!();
            let _x3: () = unimplemented!();
            test_0(unimplemented!(), 2_(), _x1, _x2, _x3, &c2)
        });
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (3_())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (3_())) {
            unimplemented!();
            i.prefix_inc();
        }
        (unsafe {
            let _c: *mut () = &mut c2;
            let _i: () = unimplemented!();
            let _x1: () = unimplemented!();
            let _x2: () = unimplemented!();
            let _x3: () = unimplemented!();
            insert3at_1(_c, _i, _x1, _x2, _x3)
        });
        (unsafe {
            let _x1: () = unimplemented!();
            let _x2: () = unimplemented!();
            let _x3: () = unimplemented!();
            test_0(unimplemented!(), 3_(), _x1, _x2, _x3, &c2)
        });
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
