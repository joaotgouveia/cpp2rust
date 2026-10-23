extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;

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
        let c: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut a: [(); 8] = [
            {
                let mut __tmp_0: () = 10_();
                std_pair_int__std_string_::new_1({ &mut __tmp_0 }, {
                    &std::mem::transmute(*b"ten\0")
                })
            },
            {
                let mut __tmp_1: () = 20_();
                std_pair_int__std_string_::new_2({ &mut __tmp_1 }, {
                    &std::mem::transmute(*b"twenty\0")
                })
            },
            {
                let mut __tmp_2: () = 30_();
                std_pair_int__std_string_::new_2({ &mut __tmp_2 }, {
                    &std::mem::transmute(*b"thirty\0")
                })
            },
            {
                let mut __tmp_3: () = 40_();
                std_pair_int__std_string_::new_3({ &mut __tmp_3 }, {
                    &std::mem::transmute(*b"forty\0")
                })
            },
            {
                let mut __tmp_4: () = 50_();
                std_pair_int__std_string_::new_3({ &mut __tmp_4 }, {
                    &std::mem::transmute(*b"fifty\0")
                })
            },
            {
                let mut __tmp_5: () = 60_();
                std_pair_int__std_string_::new_3({ &mut __tmp_5 }, {
                    &std::mem::transmute(*b"sixty\0")
                })
            },
            {
                let mut __tmp_6: () = 70_();
                std_pair_int__std_string_::new_4({ &mut __tmp_6 }, {
                    &std::mem::transmute(*b"seventy\0")
                })
            },
            {
                let mut __tmp_7: () = 80_();
                std_pair_int__std_string_::new_2({ &mut __tmp_7 }, {
                    &std::mem::transmute(*b"eighty\0")
                })
            },
        ];
        let c: () = unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
