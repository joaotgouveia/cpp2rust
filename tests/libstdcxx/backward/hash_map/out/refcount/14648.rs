extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let m : Value<__gnu_cxx_hash_map_char____gnu_cxx_rope_char__std_allocator_char______gnu_cxx_hash_char___std_equal_to_char___std_allocator___gnu_cxx_rope_char__std_allocator_char____ > = Rc::new(RefCell::new(<__gnu_cxx_hash_map_char____gnu_cxx_rope_char__std_allocator_char______gnu_cxx_hash_char___std_equal_to_char___std_allocator___gnu_cxx_rope_char__std_allocator_char____  >::default() )) ;
    (*m.borrow_mut())[('l' as ())] = __gnu_cxx_rope_char__std_allocator_char__::new_1(
        { Ptr::<()>::from_string_literal(b"50") },
        None,
    );
    (*m.borrow_mut())[('x' as ())] = __gnu_cxx_rope_char__std_allocator_char__::new_1(
        { Ptr::<()>::from_string_literal(b"10") },
        None,
    );
    unimplemented!();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test01_0() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
