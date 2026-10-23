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
    let src: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([1_u32, 2_u32, 3_u32])));
    let mut v1: () = std_vector_unsigned_int__std_allocator_unsigned_int__::new_1(
        { (src.as_pointer() as Ptr<u32>).reinterpret_cast::<()>() },
        { ((src.as_pointer() as Ptr<u32>).offset((3_()) as isize)).reinterpret_cast::<()>() },
        None,
    );
    unimplemented!();
    unimplemented!();
    let mut v2: () = std_vector_unsigned_long__std_allocator_unsigned_long__::new_1(
        { (src.as_pointer() as Ptr<u32>).reinterpret_cast::<()>() },
        { ((src.as_pointer() as Ptr<u32>).offset((3_()) as isize)).reinterpret_cast::<()>() },
        None,
    );
    unimplemented!();
    unimplemented!();
    let mut v3: () = std_vector_int__std_allocator_int__::new_1(
        { (src.as_pointer() as Ptr<u32>).reinterpret_cast::<()>() },
        { ((src.as_pointer() as Ptr<u32>).offset((3_()) as isize)).reinterpret_cast::<()>() },
        None,
    );
    unimplemented!();
    unimplemented!();
    let src1: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([1_u32, 2_u32, 3_u32])));
    let mut v4: () = std_vector_unsigned_int__std_allocator_unsigned_int__::new_2(
        { (src1.as_pointer() as Ptr<u32>).reinterpret_cast::<()>() },
        { unimplemented!() },
        None,
    );
    unimplemented!();
    unimplemented!();
    let buf: Value<Box<[u8]>> =
        Rc::new(RefCell::new(Box::new([10_u8, 20_u8, 30_u8, 40_u8, 50_u8])));
    let mut start: Ptr<u8> = (buf.as_pointer() as Ptr<u8>);
    let mut len: () = 5_();
    let mut v5: () = std_vector_unsigned_char__std_allocator_unsigned_char__::new_1(
        { (start).reinterpret_cast::<()>() },
        { (start.offset((len) as isize)).reinterpret_cast::<()>() },
        None,
    );
    unimplemented!();
    unimplemented!();
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!().write(10_());
    unimplemented!().write(20_());
    unimplemented!().write(30_());
    let v6: Value<()> = Rc::new(RefCell::new(
        std_vector_std_pair_int__int___std_allocator_std_pair_int__int___::new_1(
            { unimplemented!() },
            { unimplemented!() },
            None,
        ),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    ({
        let ___position: () = unimplemented!();
        let ___first: () = unimplemented!();
        let ___last: () = unimplemented!();
        (*v6.borrow())
            .insert_66_struct_std__Rb_tree_iterator_struct_std_pair_const_int__int____void(
                ___position,
                ___first,
                ___last,
            )
    });
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
