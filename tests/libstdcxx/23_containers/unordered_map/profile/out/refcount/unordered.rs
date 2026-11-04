extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_unordered_set_0() {
    let mut tmp2: Ptr<
        std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int__,
    > = Ptr::<std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int__>::null();
    tmp2 = Ptr::alloc(
        <std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int__>::default(),
    );
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(1));
        (*tmp2.upgrade().deref()).insert_36(___x.as_pointer())
    });
    tmp2.delete();
}
pub fn test_unordered_map_1() {
    let mut tmp : Ptr< std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int___  >  = Ptr::<std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int___>::null() ;
    let mut i: i32 = 0;
    'loop_: while (i < 20) {
        tmp  = Ptr::alloc( std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int___ :: new_1 ( { ( ( ( i + 2 ) ) as u64 )  } , None , None , None , )  )  ;
        let j: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*j.borrow()) < 10000) {
            let __rhs = (*j.borrow());
            elem!((tmp), (*j.borrow())).write(__rhs);
            (*j.borrow_mut()).postfix_inc();
        }
        tmp.delete();
        i.postfix_inc();
    }
    tmp  = Ptr::alloc( <std_unordered_map_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int___  >::default() )  ;
    let mut i: i32 = 0;
    'loop_: while (i < 150000) {
        ({
            let ___x: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
                Rc::new(RefCell::new(i)),
                Rc::new(RefCell::new(i)),
            )));
            (*tmp.upgrade().deref()).insert_44(___x.as_pointer())
        });
        i.postfix_inc();
    }
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(1));
        (*tmp.upgrade().deref()).erase_58(___x.as_pointer())
    });
    tmp.delete();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_unordered_set_0() });
    ({ test_unordered_map_1() });
    0
}
pub fn __cpp2rust_init_globals() {}
