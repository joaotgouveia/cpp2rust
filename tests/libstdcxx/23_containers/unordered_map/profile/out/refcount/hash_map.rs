extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut tmp: Ptr<
        __gnu_cxx_hash_map_int__int____gnu_cxx_hash_int___std_equal_to_int___std_allocator_int__,
    > = Ptr::<
        __gnu_cxx_hash_map_int__int____gnu_cxx_hash_int___std_equal_to_int___std_allocator_int__,
    >::null();
    let mut j: () = 1_();
    'loop_: while (j <= 10_()) {
        tmp  = Ptr::alloc( <__gnu_cxx_hash_map_int__int____gnu_cxx_hash_int___std_equal_to_int___std_allocator_int__  >::default() )  ;
        let i: Value<()> = Rc::new(RefCell::new(0_()));
        'loop_: while ((*i.borrow()) < (10000_() * j)) {
            let __rhs = (*i.borrow());
            elem!((tmp), (*i.borrow())).write(__rhs);
            (*i.borrow_mut()).postfix_inc();
        }
        tmp.delete();
        j.postfix_inc();
    }
    0
}
pub fn __cpp2rust_init_globals() {}
