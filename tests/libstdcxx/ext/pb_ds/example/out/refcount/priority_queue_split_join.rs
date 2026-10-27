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
    let even_p : Value<__gnu_pbds_priority_queue_int__std_less_int_____gnu_pbds_pairing_heap_tag__std_allocator_char__ > = Rc::new(RefCell::new(<__gnu_pbds_priority_queue_int__std_less_int_____gnu_pbds_pairing_heap_tag__std_allocator_char__  >::default() )) ;
    let odd_p : Value<__gnu_pbds_priority_queue_int__std_less_int_____gnu_pbds_pairing_heap_tag__std_allocator_char__ > = Rc::new(RefCell::new(<__gnu_pbds_priority_queue_int__std_less_int_____gnu_pbds_pairing_heap_tag__std_allocator_char__  >::default() )) ;
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        ({
            let _r_val: Value<()> = Rc::new(RefCell::new((2_()).wrapping_mul(i)));
            ((*even_p.borrow())
                as __gnu_pbds_detail_pairing_heap_int__std_less_int___std_allocator_char__)
                .push(_r_val.as_pointer())
        });
        ({
            let _r_val: Value<()> =
                Rc::new(RefCell::new(((2_()).wrapping_mul(i)).wrapping_add(1_())));
            ((*odd_p.borrow())
                as __gnu_pbds_detail_pairing_heap_int__std_less_int___std_allocator_char__)
                .push(_r_val.as_pointer())
        });
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    'loop_: while unimplemented!() {
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    'loop_: while unimplemented!() {
        unimplemented!();
        unimplemented!();
    }
    ({
        ((*even_p.borrow())
            as __gnu_pbds_detail_pairing_heap_int__std_less_int___std_allocator_char__)
            .join(odd_p.as_pointer())
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    'loop_: while unimplemented!() {
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    'loop_: while unimplemented!() {
        unimplemented!();
        unimplemented!();
    }
    ({
        ((*even_p.borrow())
            as __gnu_pbds_detail_pairing_heap_int__std_less_int___std_allocator_char__)
            .split(unimplemented!(), odd_p.as_pointer())
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    'loop_: while unimplemented!() {
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    'loop_: while unimplemented!() {
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
