extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Empty {}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct S {
    pub a: (),
    pub k: (),
    pub e: Empty,
}
impl S {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            a: {
                let mut __tmp_0: () = 1_();
                let mut __tmp_1: Empty = <Empty>::default();
                std_tuple_int__Empty_::new({ &mut __tmp_0 }, { &mut __tmp_1 })
            },
            k: (*(unsafe { get_0(&mut this.a) })),
            e: (*(unsafe { get_1(&mut this.a) })),
        };
        this
    }
}
impl Default for S {
    fn default() -> Self {
        unsafe { S::new() }
    }
}
pub unsafe fn getP_2() -> () {
    return {
        let mut __tmp_2: () = 3_();
        let mut __tmp_3: () = 4_();
        std_tuple_int__int_::new({ &mut __tmp_2 }, { &mut __tmp_3 })
    };
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
        let mut t: () = 3_();
        unimplemented!();
        (*(unsafe { get_3(&mut t) })) = 2_();
        unimplemented!();
    }
    {
        let mut t: () = {
            let mut __tmp_4: () = 5_();
            std_tuple_std_string__int_::new({ &std::mem::transmute(*b"high\0") }, { &mut __tmp_4 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (*(unsafe { get_4(&mut t) })) = 4_();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut d: () = 1.5_f64;
        let mut t: () = {
            let mut __tmp_5: () = 5_();
            std_tuple_double_ref__std_string__int_::new(
                { &mut d },
                { &std::mem::transmute(*b"high\0") },
                { &mut __tmp_5 },
            )
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (*(unsafe { get_5(&mut t) })) = 2.5_f64;
        unimplemented!();
        (*(unsafe { get_6(&mut t) })) = 4_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(
            true,
            "std::get<0> ( std::make_tuple ( 0.0f, 1, 2.0, 3L )) == 0"
        );;
        const _: () = assert!(
            true,
            "std::get<1> ( std::make_tuple ( 0.0f, 1, 2.0, 3L )) == 1"
        );;
        const _: () = assert!(
            true,
            "std::get<2> ( std::make_tuple ( 0.0f, 1, 2.0, 3L )) == 2"
        );;
        const _: () = assert!(
            true,
            "std::get<3> ( std::make_tuple ( 0.0f, 1, 2.0, 3L )) == 3"
        );;
        const _: () = assert!(true, "S().k == 1");;
        const _: () = assert!(true, "std::get<1>(getP()) == 4");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
