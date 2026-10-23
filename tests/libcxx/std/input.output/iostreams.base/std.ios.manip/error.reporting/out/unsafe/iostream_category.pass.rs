extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct StaticInit {
    pub ec: *const (),
}
impl StaticInit {
    pub unsafe fn destructor(&mut self) {
        let mut str: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { unimplemented!() },
            None,
        );
        unimplemented!();
    }
}
pub static mut foo_0: std::cell::LazyCell<StaticInit> =
    std::cell::LazyCell::new(|| unsafe { <StaticInit>::default() });

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
        let e_cat1: *const () = unimplemented!();
        let mut m1: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { unimplemented!() },
            None,
        );
        unimplemented!();
    }
    {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut foo_0)).ec = (unimplemented!());
        let mut m2: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { unimplemented!() },
            None,
        );
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const foo_0);
}
