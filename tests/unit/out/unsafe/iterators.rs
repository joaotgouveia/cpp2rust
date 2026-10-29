extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut x: std_basic_string_char__std_char_traits_char___std_allocator_char__ =
        std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"hello".as_ptr() },
            None,
        );
    'loop_: for c in IterRange::new((unsafe { x.begin_59() }), (unsafe { x.end_61() })) {
        let c: *mut () = &mut (*c);
        (*c).prefix_inc();
    }
    'loop_: for c in IterRange::new((unsafe { x.begin_59() }), (unsafe { x.end_61() })) {
        let c: *mut () = &mut (*c);
        unimplemented!();
    }
    'loop_: for c in IterRange::new((unsafe { x.begin_59() }), (unsafe { x.end_61() })) {
        let mut c: () = (*c);
        unimplemented!();
    }
    let mut v: std_vector_int_ptr__std_allocator_int_ptr__ =
        <std_vector_int_ptr__std_allocator_int_ptr__>::default();
    (unsafe {
        let mut ___x: *mut () = (Box::leak(Box::new(2_())) as *mut ());
        v.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: *mut () = (Box::leak(Box::new(3_())) as *mut ());
        v.push_back_58(&mut ___x)
    });
    'loop_: for p in IterRange::new((unsafe { v.begin_26() }), (unsafe { v.end_28() })) {
        let mut p: *mut () = (*p);
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
