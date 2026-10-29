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
    let mut vec_: std_vector_unsigned_char__std_allocator_unsigned_char__ =
        std_vector_unsigned_char__std_allocator_unsigned_char__::new_1({ unimplemented!() }, None);
    let mut i: () = 27_();
    let mut str: std_basic_string_char__std_char_traits_char___std_allocator_char__ =
        std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"foo.".as_ptr() },
            None,
        );
    write!(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_0)),
        "{:} a",
        i,
    );
    (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_0)).write_all(
        &([
            (&[vec_[(0_()) as usize] as u8] as &[u8]),
            (&[vec_[(1_()) as usize] as u8] as &[u8]),
            (&[('o' as ()) as u8] as &[u8]),
        ]
        .concat()),
    );
    write!(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_0)),
        "{:}\n0x{:x}",
        str,
        27_(),
    );
    (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_0)).write_all(
        &([
            (b" a\xc3\xa7ordas?" as &[u8]),
            (&[('\n' as ()) as u8] as &[u8]),
            (b"Sim, 0x" as &[u8]),
        ]
        .concat()),
    );
    write!(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_0)),
        "{:x}.\n",
        i,
    );
    write!(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_0)),
        "Hello, World!\n",
    );
    (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_0)).write_all(
        &([
            (&[vec_[(0_()) as usize] as u8] as &[u8]),
            (&[('\n' as ()) as u8] as &[u8]),
            (&[vec_[(1_()) as usize] as u8] as &[u8]),
            (&[('\n' as ()) as u8] as &[u8]),
        ]
        .concat()),
    );
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
