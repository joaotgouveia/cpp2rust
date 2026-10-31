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
pub struct test_macros_detail_is_same_std_less_int___std_less_int__ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std__Flat_map_impl_int__char__std_less_int___std_vector_int__std_allocator_int____std_vector_char__std_allocator_char____true__value_compare__std__Flat_map_impl_int__char__std_less_int___std_vector_int__std_allocator_int____std_vector_char__std_allocator_char____true__value_compare_
{}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_bool__bool_ {}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_function_bool__int__int____std_function_bool__int__int___
{}
pub type anon_4 = u32;
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std__Flat_map_impl_int__int__std_function_bool__int__int____std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____true__value_compare__std__Flat_map_impl_int__int__std_function_bool__int__int____std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____true__value_compare_
{}
pub type anon_5 = u32;
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_less_void___std_less_void__ {}
pub type anon_6 = u32;
pub const anon_6_value: anon_6 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std__Flat_map_impl_int__int__std_less_void___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____true__value_compare__std__Flat_map_impl_int__int__std_less_void___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____true__value_compare_
{}
pub fn test_7() -> bool {
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "!std::is_same_v<M::value_compare, Comp>");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        let kc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        let vc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!(
            ({
                (*vc.borrow_mut())(
                    (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(('2' as i8)))),
                    (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(('1' as i8)))),
                )
            })
        );
        assert!(
            !({
                (*vc.borrow_mut())(
                    (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(('1' as i8)))),
                    (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(('2' as i8)))),
                )
            })
        );
    }
    {
        let comp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        let kc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(!(unimplemented!()));
        assert!((unimplemented!() as bool));
        let vc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let a: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
            Rc::new(RefCell::new(1)),
            Rc::new(RefCell::new(2)),
        )));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "!noexcept(vc(a, a))");;
        assert!(
            !({
                (*vc.borrow_mut())(
                    (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(2))),
                    (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(1))),
                )
            })
        );
        assert!(
            ({
                (*vc.borrow_mut())(
                    (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(1))),
                    (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(2))),
                )
            })
        );
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        let kc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() as bool));
        assert!(!(unimplemented!()));
        let vc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let a: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
            Rc::new(RefCell::new(1)),
            Rc::new(RefCell::new(2)),
        )));
        const _: () = assert!(true, "");;
        assert!(
            ({
                (*vc.borrow_mut())(
                    (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(2))),
                    (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(1))),
                )
            })
        );
        assert!(
            !({
                (*vc.borrow_mut())(
                    (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(1))),
                    (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(2))),
                )
            })
        );
    }
    {
        let comp: Value<()> = Rc::new(RefCell::new(lambda!(
            Generic,
            {
                let i: Value<i32> = Rc::new(RefCell::new(1));
            },
            |x: Ptr<Vec<i32>>, y: Ptr<Vec<i32>>| -> bool {
                return ({
                    ({
                        let __idx = ((*i.borrow()) as usize);
                        if __idx as usize >= (*x.upgrade().deref()).len() {
                            panic!("out of bounds access")
                        } else {
                            x.decay().offset(__idx as isize)
                        }
                    }
                    .read())
                } < {
                    ({
                        let __idx = ((*i.borrow()) as usize);
                        if __idx as usize >= (*y.upgrade().deref()).len() {
                            panic!("out of bounds access")
                        } else {
                            y.decay().offset(__idx as isize)
                        }
                    }
                    .read())
                });
            }
        )));
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let vc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!((32usize >= 32usize), "sizeof(vc) >= sizeof(Comp)");;
        unimplemented!();
        (*m.borrow_mut()) = unimplemented!();
        assert!(unimplemented!());
        let a: Value<(Value<Vec<i32>>, Value<i32>)> = Rc::new(RefCell::new((
            Rc::new(RefCell::new(Vec::from([2, 1, 4]))),
            Rc::new(RefCell::new(42)),
        )));
        let b: Value<(Value<Vec<i32>>, Value<i32>)> = Rc::new(RefCell::new((
            Rc::new(RefCell::new(Vec::from([1, 2, 3]))),
            Rc::new(RefCell::new(42)),
        )));
        let c: Value<(Value<Vec<i32>>, Value<i32>)> = Rc::new(RefCell::new((
            Rc::new(RefCell::new(Vec::from([0, 3, 2]))),
            Rc::new(RefCell::new(42)),
        )));
        assert!(
            ({
                (*vc.borrow_mut())(
                    {
                        let __a0 = (*a.borrow()).clone();
                        (
                            Rc::new(RefCell::new(
                                <Ptr<Vec<i32>> as CppFrom<Vec<i32>>>::cpp_from(__a0.0.as_pointer()),
                            )),
                            Rc::new(RefCell::new(<Ptr<i32> as CppFrom<i32>>::cpp_from(
                                __a0.1.as_pointer(),
                            ))),
                        )
                    },
                    {
                        let __a0 = (*b.borrow()).clone();
                        (
                            Rc::new(RefCell::new(
                                <Ptr<Vec<i32>> as CppFrom<Vec<i32>>>::cpp_from(__a0.0.as_pointer()),
                            )),
                            Rc::new(RefCell::new(<Ptr<i32> as CppFrom<i32>>::cpp_from(
                                __a0.1.as_pointer(),
                            ))),
                        )
                    },
                )
            })
        );
        assert!(
            ({
                (*vc.borrow_mut())(
                    {
                        let __a0 = (*b.borrow()).clone();
                        (
                            Rc::new(RefCell::new(
                                <Ptr<Vec<i32>> as CppFrom<Vec<i32>>>::cpp_from(__a0.0.as_pointer()),
                            )),
                            Rc::new(RefCell::new(<Ptr<i32> as CppFrom<i32>>::cpp_from(
                                __a0.1.as_pointer(),
                            ))),
                        )
                    },
                    {
                        let __a0 = (*c.borrow()).clone();
                        (
                            Rc::new(RefCell::new(
                                <Ptr<Vec<i32>> as CppFrom<Vec<i32>>>::cpp_from(__a0.0.as_pointer()),
                            )),
                            Rc::new(RefCell::new(<Ptr<i32> as CppFrom<i32>>::cpp_from(
                                __a0.1.as_pointer(),
                            ))),
                        )
                    },
                )
            })
        );
        assert!(
            !({
                (*vc.borrow_mut())(
                    {
                        let __a0 = (*b.borrow()).clone();
                        (
                            Rc::new(RefCell::new(
                                <Ptr<Vec<i32>> as CppFrom<Vec<i32>>>::cpp_from(__a0.0.as_pointer()),
                            )),
                            Rc::new(RefCell::new(<Ptr<i32> as CppFrom<i32>>::cpp_from(
                                __a0.1.as_pointer(),
                            ))),
                        )
                    },
                    {
                        let __a0 = (*a.borrow()).clone();
                        (
                            Rc::new(RefCell::new(
                                <Ptr<Vec<i32>> as CppFrom<Vec<i32>>>::cpp_from(__a0.0.as_pointer()),
                            )),
                            Rc::new(RefCell::new(<Ptr<i32> as CppFrom<i32>>::cpp_from(
                                __a0.1.as_pointer(),
                            ))),
                        )
                    },
                )
            })
        );
        assert!(
            !({
                (*vc.borrow_mut())(
                    {
                        let __a0 = (*c.borrow()).clone();
                        (
                            Rc::new(RefCell::new(
                                <Ptr<Vec<i32>> as CppFrom<Vec<i32>>>::cpp_from(__a0.0.as_pointer()),
                            )),
                            Rc::new(RefCell::new(<Ptr<i32> as CppFrom<i32>>::cpp_from(
                                __a0.1.as_pointer(),
                            ))),
                        )
                    },
                    {
                        let __a0 = (*b.borrow()).clone();
                        (
                            Rc::new(RefCell::new(
                                <Ptr<Vec<i32>> as CppFrom<Vec<i32>>>::cpp_from(__a0.0.as_pointer()),
                            )),
                            Rc::new(RefCell::new(<Ptr<i32> as CppFrom<i32>>::cpp_from(
                                __a0.1.as_pointer(),
                            ))),
                        )
                    },
                )
            })
        );
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
    ({ test_7() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
