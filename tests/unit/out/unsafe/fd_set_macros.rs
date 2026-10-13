extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg)]
pub struct __fsid_t {
    pub __val: [i32; 2],
}
impl Clone for __fsid_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __fsid_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
impl Clone for __sigset_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __sigset_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}
impl Clone for timeval {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for timeval {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}
impl Clone for timespec {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for timespec {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct fd_set {
    pub __fds_bits: [i64; 16],
}
impl Clone for fd_set {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for fd_set {
    fn default() -> Self {
        unimplemented!()
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut set: fd_set = unsafe { std::mem::zeroed::<fd_set>() };
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut __i: u32 = 0_u32;
        let mut __arr: *mut fd_set = (&mut set as *mut fd_set);
        __i = 0_u32;
        'loop_: while ((((__i as usize)
            < ((::std::mem::size_of::<fd_set>() as usize)
                .wrapping_div((::std::mem::size_of::<i64>() as usize))))
            as i32)
            != 0)
        {
            ((*(__arr)).__fds_bits)[(__i) as usize] = 0_i64;
            __i.prefix_inc();
        }
    }
    {
        &(::std::mem::size_of::<i32>());
        {
            if !(((((((*(&mut set as *mut fd_set)).__fds_bits)
                [((3) / ((8) * (::std::mem::size_of::<i64>() as i32))) as usize])
                & (((1_u64) << ((3) % ((8) * (::std::mem::size_of::<i64>() as i32)))) as i64))
                != (0_i64)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"!FD_ISSET(3, &set)".as_ptr().cast_mut()).cast_const(),
                        (c"fd_set_macros.c".as_ptr().cast_mut()).cast_const(),
                        7_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    ({
        ((*(&mut set as *mut fd_set)).__fds_bits)
            [((3) / ((8) * (::std::mem::size_of::<i64>() as i32))) as usize] |=
            (((1_u64) << ((3) % ((8) * (::std::mem::size_of::<i64>() as i32)))) as i64)
    });
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((((*(&mut set as *mut fd_set)).__fds_bits)
                [((3) / ((8) * (::std::mem::size_of::<i64>() as i32))) as usize])
                & (((1_u64) << ((3) % ((8) * (::std::mem::size_of::<i64>() as i32)))) as i64))
                != (0_i64)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"FD_ISSET(3, &set)".as_ptr().cast_mut()).cast_const(),
                        (c"fd_set_macros.c".as_ptr().cast_mut()).cast_const(),
                        9_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    ({
        ((*(&mut set as *mut fd_set)).__fds_bits)
            [((3) / ((8) * (::std::mem::size_of::<i64>() as i32))) as usize] &=
            !(((1_u64) << ((3) % ((8) * (::std::mem::size_of::<i64>() as i32)))) as i64)
    });
    {
        &(::std::mem::size_of::<i32>());
        {
            if !(((((((*(&mut set as *mut fd_set)).__fds_bits)
                [((3) / ((8) * (::std::mem::size_of::<i64>() as i32))) as usize])
                & (((1_u64) << ((3) % ((8) * (::std::mem::size_of::<i64>() as i32)))) as i64))
                != (0_i64)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"!FD_ISSET(3, &set)".as_ptr().cast_mut()).cast_const(),
                        (c"fd_set_macros.c".as_ptr().cast_mut()).cast_const(),
                        11_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    return 0;
}
pub unsafe fn __assert_fail_0(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
