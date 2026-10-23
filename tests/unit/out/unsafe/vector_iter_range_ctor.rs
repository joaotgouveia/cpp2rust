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
    let mut src: [u32; 3] = [1_u32, 2_u32, 3_u32];
    let mut v1: () = std_vector_unsigned_int__std_allocator_unsigned_int__::new_1(
        { (src.as_mut_ptr() as *mut u32).cast::<()>() },
        { (src.as_mut_ptr().offset((3_()) as isize) as *mut u32).cast::<()>() },
        None,
    );
    unimplemented!();
    unimplemented!();
    let mut v2: () = std_vector_unsigned_long__std_allocator_unsigned_long__::new_1(
        { (src.as_mut_ptr() as *mut u32).cast::<()>() },
        { (src.as_mut_ptr().offset((3_()) as isize) as *mut u32).cast::<()>() },
        None,
    );
    unimplemented!();
    unimplemented!();
    let mut v3: () = std_vector_int__std_allocator_int__::new_1(
        { (src.as_mut_ptr() as *mut u32).cast::<()>() },
        { (src.as_mut_ptr().offset((3_()) as isize) as *mut u32).cast::<()>() },
        None,
    );
    unimplemented!();
    unimplemented!();
    let src1: [u32; 3] = [1_u32, 2_u32, 3_u32];
    let mut v4: () = std_vector_unsigned_int__std_allocator_unsigned_int__::new_2(
        { (src1.as_ptr() as *const u32).cast::<()>() },
        { unimplemented!() },
        None,
    );
    unimplemented!();
    unimplemented!();
    let mut buf: [u8; 5] = [10_u8, 20_u8, 30_u8, 40_u8, 50_u8];
    let mut start: *const u8 = (buf.as_mut_ptr()).cast_const();
    let mut len: () = 5_();
    let mut v5: () = std_vector_unsigned_char__std_allocator_unsigned_char__::new_1(
        { (start as *const u8).cast::<()>() },
        { (start.offset((len) as isize) as *const u8).cast::<()>() },
        None,
    );
    unimplemented!();
    unimplemented!();
    let mut m: () = unimplemented!();
    (*unimplemented!()) = 10_();
    (*unimplemented!()) = 20_();
    (*unimplemented!()) = 30_();
    let mut v6: () = std_vector_std_pair_int__int___std_allocator_std_pair_int__int___::new_1(
        { unimplemented!() },
        { unimplemented!() },
        None,
    );
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (unsafe {
        let ___position: () = unimplemented!();
        let ___first: () = unimplemented!();
        let ___last: () = unimplemented!();
        v6.insert_66_struct_std__Rb_tree_iterator_struct_std_pair_const_int__int____void(
            ___position,
            ___first,
            ___last,
        )
    });
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
