extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_adjacent_pairs_0() {
    let mut v: () = unimplemented!();
    let expected_index: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let __tmp_0: Value<
        std_ranges_adjacent_view_std_ranges_ref_view_std_vector_int__std_allocator_int_____2_,
    > = Rc::new(RefCell::new(unimplemented!()));
    let __range1: Ptr<()> = __tmp_0.as_pointer();
    'loop_: for __decomp_1 in IterRange::new(unimplemented!(), unimplemented!()) {
        let __decomp_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let x: Ptr<()> = ({ get_2((__decomp_1.as_pointer()).reinterpret_cast::<()>()) });
        let y: Ptr<()> = ({ get_3((__decomp_1.as_pointer()).reinterpret_cast::<()>()) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (*expected_index.borrow_mut()).first.prefix_inc();
        (*expected_index.borrow_mut()).second.prefix_inc();
    }
}
pub fn test_string_view_4() {
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let __decomp_5: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let a: Ptr<()> = ({ get_6((__decomp_5.as_pointer()).reinterpret_cast::<()>()) });
    let b: Ptr<()> = ({ get_7((__decomp_5.as_pointer()).reinterpret_cast::<()>()) });
    let c: Ptr<()> = ({ get_8((__decomp_5.as_pointer()).reinterpret_cast::<()>()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_9() -> () {
    ({ test_adjacent_pairs_0() });
    ({ test_string_view_4() });
    return true;
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_9() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
