extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn begin_0(_a0: *mut [ArrayHijacker; 3]) -> i32 {
    return 42;
}
pub unsafe fn end_1(_a0: *mut [ArrayHijacker; 3]) -> i32 {
    return 42;
}
pub unsafe fn begin_2(_a0: *const [ArrayHijacker; 3]) -> i32 {
    return 42;
}
pub unsafe fn end_3(_a0: *const [ArrayHijacker; 3]) -> i32 {
    return 42;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ArrayHijacker {}
pub unsafe fn begin_4(_a0: *mut ContainerHijacker) -> i32 {
    return 42;
}
pub unsafe fn end_5(_a0: *mut ContainerHijacker) -> i32 {
    return 42;
}
pub unsafe fn begin_6(_a0: *const ContainerHijacker) -> i32 {
    return 42;
}
pub unsafe fn end_7(_a0: *const ContainerHijacker) -> i32 {
    return 42;
}
pub unsafe fn cbegin_8(_a0: *mut ContainerHijacker) -> i32 {
    return 42;
}
pub unsafe fn cend_9(_a0: *mut ContainerHijacker) -> i32 {
    return 42;
}
pub unsafe fn cbegin_10(_a0: *const ContainerHijacker) -> i32 {
    return 42;
}
pub unsafe fn cend_11(_a0: *const ContainerHijacker) -> i32 {
    return 42;
}
pub unsafe fn rbegin_12(_a0: *mut ContainerHijacker) -> i32 {
    return 42;
}
pub unsafe fn rend_13(_a0: *mut ContainerHijacker) -> i32 {
    return 42;
}
pub unsafe fn rbegin_14(_a0: *const ContainerHijacker) -> i32 {
    return 42;
}
pub unsafe fn rend_15(_a0: *const ContainerHijacker) -> i32 {
    return 42;
}
pub unsafe fn crbegin_16(_a0: *mut ContainerHijacker) -> i32 {
    return 42;
}
pub unsafe fn crend_17(_a0: *mut ContainerHijacker) -> i32 {
    return 42;
}
pub unsafe fn crbegin_18(_a0: *const ContainerHijacker) -> i32 {
    return 42;
}
pub unsafe fn crend_19(_a0: *const ContainerHijacker) -> i32 {
    return 42;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ContainerHijacker {
    pub a_: *mut i32,
}
impl ContainerHijacker {
    pub unsafe fn begin(&self) -> *mut i32 {
        return self.a_;
    }
    pub unsafe fn end(&self) -> *mut i32 {
        return self.a_.offset((3) as isize);
    }
    pub unsafe fn rbegin(&self) -> *mut i32 {
        return self.a_;
    }
    pub unsafe fn rend(&self) -> *mut i32 {
        return self.a_.offset((3) as isize);
    }
}
pub unsafe fn test_20() -> bool {
    {
        let mut a: [ArrayHijacker; 3] = [
            <ArrayHijacker>::default(),
            <ArrayHijacker>::default(),
            <ArrayHijacker>::default(),
        ];
        assert!(((unsafe { begin_0(&mut a,) }) == (42)));
        assert!(((unsafe { end_1(&mut a,) }) == (42)));
        assert!(((unimplemented!()) == (a.as_mut_ptr())));
        assert!(((a.as_ptr().add(a.len())) == (a.as_mut_ptr().offset((3) as isize))));
        assert!(((unimplemented!()) == ((a.as_mut_ptr()).cast_const())));
        assert!(((unimplemented!()) == ((a.as_mut_ptr().offset((3) as isize)).cast_const())));
        assert!(((unimplemented!()) == (a.as_mut_ptr().offset((3) as isize))));
        assert!(((unimplemented!()) == (a.as_mut_ptr())));
        assert!(((unimplemented!()) == ((a.as_mut_ptr().offset((3) as isize)).cast_const())));
        assert!(((unimplemented!()) == ((a.as_mut_ptr()).cast_const())));
    }
    {
        let mut a: [i32; 3] = [0_i32, 0_i32, 0_i32];
        let mut c: ContainerHijacker = ContainerHijacker { a_: a.as_mut_ptr() };
        assert!(((unsafe { begin_4(&mut c,) }) == (42)));
        assert!(((unsafe { end_5(&mut c,) }) == (42)));
        assert!(((unimplemented!()) == (a.as_mut_ptr())));
        assert!(((unimplemented!()) == (a.as_mut_ptr().offset((3) as isize))));
        assert!(((unimplemented!()) == (a.as_mut_ptr())));
        assert!(((unimplemented!()) == (a.as_mut_ptr().offset((3) as isize))));
        assert!(((unimplemented!()) == (a.as_mut_ptr())));
        assert!(((unimplemented!()) == (a.as_mut_ptr().offset((3) as isize))));
        assert!(((unimplemented!()) == (a.as_mut_ptr())));
        assert!(((unimplemented!()) == (a.as_mut_ptr().offset((3) as isize))));
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
    (unsafe { test_20() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
