extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct ConstSwappable {
    #[offset(0)]
    pub i: i32,
}
impl CppMove for ConstSwappable {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn swap_0(lhs: Ptr<ConstSwappable>, rhs: Ptr<ConstSwappable>) {
    cpp_swap!(field_ptr!(lhs, i), field_ptr!(rhs, i));
}
pub fn test_1() -> bool {
    {
        let t0: Value<std_tuple_const_ConstSwappable_> =
            Rc::new(RefCell::new(ConstSwappable { i: 0 }));
        let t1: Value<std_tuple_const_ConstSwappable_> =
            Rc::new(RefCell::new(ConstSwappable { i: 1 }));
        ({ (*t0.borrow()).swap_79(t1.as_pointer()) });
        assert!((({ get_2(t0.as_pointer(),) }).with(|__s| __s.i) == 1));
        assert!((({ get_3(t1.as_pointer(),) }).with(|__s| __s.i) == 0));
    }
    {
        let t0: Value<std_tuple_ConstSwappable__ConstSwappable_> = Rc::new(RefCell::new({
            let __tmp_0: Value<ConstSwappable> = Rc::new(RefCell::new(ConstSwappable { i: 0 }));
            let __tmp_1: Value<ConstSwappable> = Rc::new(RefCell::new(ConstSwappable { i: 1 }));
            std_tuple_ConstSwappable__ConstSwappable_::new_1({ __tmp_0.as_pointer() }, {
                __tmp_1.as_pointer()
            })
        }));
        let t1: Value<std_tuple_ConstSwappable__ConstSwappable_> = Rc::new(RefCell::new({
            let __tmp_2: Value<ConstSwappable> = Rc::new(RefCell::new(ConstSwappable { i: 2 }));
            let __tmp_3: Value<ConstSwappable> = Rc::new(RefCell::new(ConstSwappable { i: 3 }));
            std_tuple_ConstSwappable__ConstSwappable_::new_1({ __tmp_2.as_pointer() }, {
                __tmp_3.as_pointer()
            })
        }));
        ({ (*t0.borrow()).swap_79(t1.as_pointer()) });
        assert!((({ get_4(t0.as_pointer(),) }).with(|__s| __s.i) == 2));
        assert!((({ get_5(t0.as_pointer(),) }).with(|__s| __s.i) == 3));
        assert!((({ get_4(t1.as_pointer(),) }).with(|__s| __s.i) == 0));
        assert!((({ get_5(t1.as_pointer(),) }).with(|__s| __s.i) == 1));
    }
    {
        let t0: Value<std_tuple_ConstSwappable__const_ConstSwappable__const_ConstSwappable_> =
            Rc::new(RefCell::new({
                let __tmp_4: Value<ConstSwappable> = Rc::new(RefCell::new(ConstSwappable { i: 0 }));
                let __tmp_5: Value<ConstSwappable> = Rc::new(RefCell::new(ConstSwappable { i: 1 }));
                let __tmp_6: Value<ConstSwappable> = Rc::new(RefCell::new(ConstSwappable { i: 2 }));
                std_tuple_ConstSwappable__const_ConstSwappable__const_ConstSwappable_::new_1(
                    { __tmp_4.as_pointer() },
                    { __tmp_5.as_pointer() },
                    { __tmp_6.as_pointer() },
                )
            }));
        let t1: Value<std_tuple_ConstSwappable__const_ConstSwappable__const_ConstSwappable_> =
            Rc::new(RefCell::new({
                let __tmp_7: Value<ConstSwappable> = Rc::new(RefCell::new(ConstSwappable { i: 3 }));
                let __tmp_8: Value<ConstSwappable> = Rc::new(RefCell::new(ConstSwappable { i: 4 }));
                let __tmp_9: Value<ConstSwappable> = Rc::new(RefCell::new(ConstSwappable { i: 5 }));
                std_tuple_ConstSwappable__const_ConstSwappable__const_ConstSwappable_::new_1(
                    { __tmp_7.as_pointer() },
                    { __tmp_8.as_pointer() },
                    { __tmp_9.as_pointer() },
                )
            }));
        ({ (*t0.borrow()).swap_79(t1.as_pointer()) });
        assert!((({ get_6(t0.as_pointer(),) }).with(|__s| __s.i) == 3));
        assert!((({ get_7(t0.as_pointer(),) }).with(|__s| __s.i) == 4));
        assert!((({ get_8(t0.as_pointer(),) }).with(|__s| __s.i) == 5));
        assert!((({ get_6(t1.as_pointer(),) }).with(|__s| __s.i) == 0));
        assert!((({ get_7(t1.as_pointer(),) }).with(|__s| __s.i) == 1));
        assert!((({ get_8(t1.as_pointer(),) }).with(|__s| __s.i) == 2));
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
    ({ test_1() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
