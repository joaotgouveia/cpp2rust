extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_less_int___std_less_int__ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_bool__bool_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_function_bool__int__int____std_function_bool__int__int___
{}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_less_void___std_less_void__ {}
pub fn test_4() -> () {
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "!std::is_same_v<M::value_compare, Comp>");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        let kc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        let vc: Value<std_flat_map_int__char__value_compare> =
            Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
    }
    {
        let comp: Value<()> = Rc::new(RefCell::new(std_function_bool__int__int__::new_1({
            unimplemented!()
        })));
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        let kc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        let vc: Value<std_flat_map_int__int__std_function_bool__int__int____value_compare> =
            Rc::new(RefCell::new(unimplemented!()));
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "!noexcept(vc(a, a))");;
        unimplemented!();
        unimplemented!();
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        let kc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        let vc: Value<std_flat_map_int__int__std_less_void___value_compare> =
            Rc::new(RefCell::new(unimplemented!()));
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
    }
    {
        let comp: Value<()> = Rc::new(RefCell::new(lambda!(
            Generic,
            {
                let i: Value<()> = Rc::new(RefCell::new(1_()));
            },
            |x: Ptr<()>, y: Ptr<()>| -> () {
                return ({ (unimplemented!().read()) } < { (unimplemented!().read()) });
            }
        )));
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let vc : Value<std_flat_map_std_vector_int___int__std_function_bool__const_std_vector_int__ref__const_std_vector_int__ref____value_compare > = Rc::new(RefCell::new(unimplemented!()  )) ;
        const _: () = assert!((48usize >= 48usize), "sizeof(vc) >= sizeof(Comp)");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let b: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
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
    ({ test_4() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
