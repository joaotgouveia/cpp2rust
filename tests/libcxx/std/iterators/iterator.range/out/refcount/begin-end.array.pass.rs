extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
impl CppMove for test_macros_detail_is_same_int_ptr__int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {}
impl CppMove for test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_2() -> bool {
    let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    let ca: Ptr<i32> = a.as_pointer();
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!((({ begin_3(a.as_pointer(),) }) == (a.as_pointer() as Ptr::<i32>)));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(
            ((a.as_pointer() as Ptr<i32>).to_end()
                == (a.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(({ ({ begin_4((ca).clone(),) }) } == { (ca).clone() }));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(({ ((ca).clone() as Ptr<i32>).to_end() } == { (ca).offset((3) as isize) }));
    }
    return true;
}
pub fn test_r_5() -> bool {
    let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    let ca: Ptr<i32> = a.as_pointer();
    {
        const _: () = assert!(true, "");;
        assert!(
            (({ ({ rbegin_6(a.as_pointer(),) }).base() })
                == (a.as_pointer() as Ptr::<i32>).offset((3) as isize))
        );
        const _: () = assert!(true, "");;
        assert!((({ ({ rend_7(a.as_pointer(),) }).base() }) == (a.as_pointer() as Ptr::<i32>)));
        const _: () = assert!(true, "");;
        assert!(({ ({ ({ rbegin_8((ca).clone(),) }).base() }) } == { (ca).offset((3) as isize) }));
        const _: () = assert!(true, "");;
        assert!(({ ({ ({ rend_9((ca).clone(),) }).base() }) } == { (ca).clone() }));
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_2() });
    ({ test_r_5() });
    const _: () = assert!(true, "test()");;
    const _: () = assert!(true, "test_r()");;
    {
        let mut b: Ptr<i32> = ({ begin_4(a_10.with(|v| v.as_pointer())) });
        assert!(({ (b).clone() } == { (a_10.with(|v| v.as_pointer()) as Ptr::<i32>) }));
        let mut e: Ptr<i32> = (a_10.with(|v| v.as_pointer()) as Ptr<i32>).to_end();
        assert!(
            ({ (e).clone() } == {
                (a_10.with(|v| v.as_pointer()) as Ptr<i32>).offset((3) as isize)
            })
        );
    }
    return 0;
}
thread_local!(
    static a_10: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
);
pub fn __cpp2rust_init_globals() {}
