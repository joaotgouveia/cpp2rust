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
    let p : Value<__gnu_pbds_priority_queue_int__std_less_int_____gnu_pbds_pairing_heap_tag__std_allocator_char__ > = Rc::new(RefCell::new(<__gnu_pbds_priority_queue_int__std_less_int_____gnu_pbds_pairing_heap_tag__std_allocator_char__  >::default() )) ;
    let i: Value<()> = Rc::new(RefCell::new(0_()));
    'loop_: while ((*i.borrow()) < 1000_()) {
        ({
            ((*p.borrow())
                as __gnu_pbds_detail_pairing_heap_int__std_less_int___std_allocator_char__)
                .push(i.as_pointer())
        });
        (*i.borrow_mut()).prefix_inc();
    }
    unimplemented!();
    ({
        ((*p.borrow()) as __gnu_pbds_detail_pairing_heap_int__std_less_int___std_allocator_char__)
            .erase_if(
                ({
                    let ___fn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                    let ___x: Value<()> = Rc::new(RefCell::new(500_()));
                    bind1st_0(___fn.as_pointer(), ___x.as_pointer())
                }),
            )
    });
    unimplemented!();
    0
}
pub fn __cpp2rust_init_globals() {}
