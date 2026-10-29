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
    let mut str: std_basic_string_char__std_char_traits_char___std_allocator_char__ =
        std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"Hello, world!\n".as_ptr() },
            None,
        );
    let file: [(); 25] = std::mem::transmute(*b"test_stdcopy_ostream.txt\0");
    {
        let mut ofs: std_basic_ofstream_char__std_char_traits_char__ =
            std_basic_ofstream_char__std_char_traits_char__::new_1({ file.as_ptr() }, {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut binary_0))
            });
        (unsafe {
            let ___first: __gnu_cxx___normal_iterator_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___  = ( unsafe { str . begin_59  ( ) } )  ;
            let ___last: __gnu_cxx___normal_iterator_char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___  = ( unsafe { str . end_61  ( ) } )  ;
            copy_1(
                ___first,
                ___last,
                std_ostream_iterator_char__char__std_char_traits_char__::new_1({ &mut ofs }),
            )
        });
    }
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
