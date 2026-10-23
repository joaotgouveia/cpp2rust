extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test01_0() {
    let mut m : __gnu_cxx_hash_map_char____gnu_cxx_rope_char__std_allocator_char______gnu_cxx_hash_char___std_equal_to_char___std_allocator___gnu_cxx_rope_char__std_allocator_char____ = <__gnu_cxx_hash_map_char____gnu_cxx_rope_char__std_allocator_char______gnu_cxx_hash_char___std_equal_to_char___std_allocator___gnu_cxx_rope_char__std_allocator_char____ >::default() ;
    m[('l' as ()) as usize] =
        __gnu_cxx_rope_char__std_allocator_char__::new_1({ c"50".as_ptr() }, None);
    m[('x' as ()) as usize] =
        __gnu_cxx_rope_char__std_allocator_char__::new_1({ c"10".as_ptr() }, None);
    unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test01_0() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
