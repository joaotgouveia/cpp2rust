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
        let mut p0: [(); 1] = [1_()];
        let mut pa: std_discrete_distribution___param_type =
            std_discrete_distribution___param_type::new_1({ p0.as_mut_ptr() }, { p0.as_mut_ptr() });
        let mut p: () = (unsafe { pa.probabilities() });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p0: [(); 1] = [10_()];
        let mut pa: std_discrete_distribution___param_type =
            std_discrete_distribution___param_type::new_1({ p0.as_mut_ptr() }, {
                p0.as_mut_ptr().offset((1_()) as isize)
            });
        let mut p: () = (unsafe { pa.probabilities() });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p0: [(); 2] = [10_(), 30_()];
        let mut pa: std_discrete_distribution___param_type =
            std_discrete_distribution___param_type::new_1({ p0.as_mut_ptr() }, {
                p0.as_mut_ptr().offset((2_()) as isize)
            });
        let mut p: () = (unsafe { pa.probabilities() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p0: [(); 2] = [30_(), 10_()];
        let mut pa: std_discrete_distribution___param_type =
            std_discrete_distribution___param_type::new_1({ p0.as_mut_ptr() }, {
                p0.as_mut_ptr().offset((2_()) as isize)
            });
        let mut p: () = (unsafe { pa.probabilities() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p0: [(); 3] = [30_(), 0_(), 10_()];
        let mut pa: std_discrete_distribution___param_type =
            std_discrete_distribution___param_type::new_1({ p0.as_mut_ptr() }, {
                p0.as_mut_ptr().offset((3_()) as isize)
            });
        let mut p: () = (unsafe { pa.probabilities() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p0: [(); 3] = [0_(), 30_(), 10_()];
        let mut pa: std_discrete_distribution___param_type =
            std_discrete_distribution___param_type::new_1({ p0.as_mut_ptr() }, {
                p0.as_mut_ptr().offset((3_()) as isize)
            });
        let mut p: () = (unsafe { pa.probabilities() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p0: [(); 3] = [0_(), 0_(), 10_()];
        let mut pa: std_discrete_distribution___param_type =
            std_discrete_distribution___param_type::new_1({ p0.as_mut_ptr() }, {
                p0.as_mut_ptr().offset((3_()) as isize)
            });
        let mut p: () = (unsafe { pa.probabilities() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
