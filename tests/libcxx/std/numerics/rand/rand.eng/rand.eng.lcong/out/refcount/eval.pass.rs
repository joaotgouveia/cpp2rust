extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn randu_0() {
    let e: Value<std_linear_congruential_engine_unsigned_int__65539__0__2147483648_> =
        Rc::new(RefCell::new(
            std_linear_congruential_engine_unsigned_int__65539__0__2147483648_::new_1({ 1_u32 }),
        ));
    assert!(({ (({ (*e.borrow_mut())() }) == 65539_u32) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 393225_u32) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1769499_u32) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 7077969_u32) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 26542323_u32) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 95552217_u32) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 334432395_u32) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1146624417_u32) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1722371299_u32) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 14608041_u32) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1766175739_u32) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1875647473_u32) }));
}
pub fn randu_1() {
    let e: Value<std_linear_congruential_engine_unsigned_long__65539__0__2147483648_> =
        Rc::new(RefCell::new(
            std_linear_congruential_engine_unsigned_long__65539__0__2147483648_::new_1({ 1_u64 }),
        ));
    assert!(({ (({ (*e.borrow_mut())() }) == 65539_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 393225_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1769499_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 7077969_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 26542323_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 95552217_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 334432395_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1146624417_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1722371299_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 14608041_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1766175739_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1875647473_u64) }));
}
pub fn randu_2() {
    let e: Value<std_linear_congruential_engine_unsigned_long_long__65539__0__2147483648_> =
        Rc::new(RefCell::new(
            std_linear_congruential_engine_unsigned_long_long__65539__0__2147483648_::new_1({
                1_u64
            }),
        ));
    assert!(({ (({ (*e.borrow_mut())() }) == 65539_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 393225_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1769499_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 7077969_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 26542323_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 95552217_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 334432395_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1146624417_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1722371299_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 14608041_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1766175739_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1875647473_u64) }));
}
pub fn minstd_3() {
    let e: Value<std_linear_congruential_engine_unsigned_int__16807__0__2147483647_> =
        Rc::new(RefCell::new(
            std_linear_congruential_engine_unsigned_int__16807__0__2147483647_::new_1({ 1_u32 }),
        ));
    assert!(({ (({ (*e.borrow_mut())() }) == 16807_u32) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 282475249_u32) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1622650073_u32) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 984943658_u32) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1144108930_u32) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 470211272_u32) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 101027544_u32) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1457850878_u32) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1458777923_u32) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 2007237709_u32) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 823564440_u32) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1115438165_u32) }));
}
pub fn minstd_4() {
    let e: Value<std_linear_congruential_engine_unsigned_long__16807__0__2147483647_> =
        Rc::new(RefCell::new(
            std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::new_1({ 1_u64 }),
        ));
    assert!(({ (({ (*e.borrow_mut())() }) == 16807_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 282475249_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1622650073_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 984943658_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1144108930_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 470211272_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 101027544_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1457850878_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1458777923_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 2007237709_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 823564440_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1115438165_u64) }));
}
pub fn minstd_5() {
    let e: Value<std_linear_congruential_engine_unsigned_long_long__16807__0__2147483647_> =
        Rc::new(RefCell::new(
            std_linear_congruential_engine_unsigned_long_long__16807__0__2147483647_::new_1({
                1_u64
            }),
        ));
    assert!(({ (({ (*e.borrow_mut())() }) == 16807_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 282475249_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1622650073_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 984943658_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1144108930_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 470211272_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 101027544_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1457850878_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1458777923_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 2007237709_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 823564440_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1115438165_u64) }));
}
pub fn Haldir_6() {
    let e: Value<std_linear_congruential_engine_unsigned_int__16807__78125__2147483647_> =
        Rc::new(RefCell::new(
            std_linear_congruential_engine_unsigned_int__16807__78125__2147483647_::new_1({
                207560540_u32
            }),
        ));
    assert!(({ (({ (*e.borrow_mut())() }) == 956631177_u32) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 2037688522_u32) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1509348670_u32) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1546336451_u32) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 429714088_u32) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 217250280_u32) }));
}
pub fn Haldir_7() {
    let e: Value<std_linear_congruential_engine_unsigned_long__16807__78125__2147483647_> =
        Rc::new(RefCell::new(
            std_linear_congruential_engine_unsigned_long__16807__78125__2147483647_::new_1({
                207560540_u64
            }),
        ));
    assert!(({ (({ (*e.borrow_mut())() }) == 956631177_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 2037688522_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1509348670_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1546336451_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 429714088_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 217250280_u64) }));
}
pub fn Haldir_8() {
    let e: Value<std_linear_congruential_engine_unsigned_long_long__16807__78125__2147483647_> =
        Rc::new(RefCell::new(
            std_linear_congruential_engine_unsigned_long_long__16807__78125__2147483647_::new_1({
                207560540_u64
            }),
        ));
    assert!(({ (({ (*e.borrow_mut())() }) == 956631177_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 2037688522_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1509348670_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 1546336451_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 429714088_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 217250280_u64) }));
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ randu_0() });
    ({ randu_1() });
    ({ randu_2() });
    ({ minstd_3() });
    ({ minstd_4() });
    ({ minstd_5() });
    ({ Haldir_6() });
    ({ Haldir_7() });
    ({ Haldir_8() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
