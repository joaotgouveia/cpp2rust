extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Counter_base {}
impl CppMove for Counter_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Destructor, Default)]
#[cpp_from(i32, new_1, ptr)]
#[byte_size(4)]
pub struct Counter_int_ {
    #[offset(0)]
    data_: i32,
}
impl Counter_int_ {
    pub fn new_1(data: Ptr<i32>) -> Self {
        let __this: Counter_int_ = Self {
            data_: (data.read()),
        };
        (*gConstructed_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(rhs: Ptr<Counter_int_>) -> Self {
        let __this: Counter_int_ = Self {
            data_: rhs.with(|__s| __s.data_),
        };
        (*gConstructed_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl CppLess for Counter_int_ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        Counter_int_Impl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for Counter_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        Counter_int_Impl::operator_eq(__lhs, __rhs.clone())
    }
}
impl Clone for Counter_int_ {
    fn clone(&self) -> Self {
        let __src: Value<Counter_int_> = Rc::new(RefCell::new(Counter_int_ {
            data_: self.data_.clone(),
        }));
        Counter_int_::copy_from(__src.as_pointer())
    }
}
impl CppMove for Counter_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static gConstructed_0: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn test_1() -> bool {
    {
        let src: Value<std_set_int__std_less_int___std_allocator_int__> = Rc::new(RefCell::new(
            std_set_int__std_less_int___std_allocator_int__::new_1(
                { Vec::from([1, 3, 5]) },
                None,
                None,
            ),
        ));
        let dst: Value<std_set_int__std_less_int___std_allocator_int__> = Rc::new(RefCell::new(
            std_set_int__std_less_int___std_allocator_int__::new_1(
                { Vec::from([2, 4, 5]) },
                None,
                None,
            ),
        ));
        ({ (*dst.borrow()).merge(src.as_pointer()) });
        assert!(
            ({
                set_equal_2(
                    src.as_pointer(),
                    std_set_int__std_less_int___std_allocator_int__::new_1(
                        { Vec::from([5]) },
                        None,
                        None,
                    ),
                )
            })
        );
        assert!(
            ({
                set_equal_2(
                    dst.as_pointer(),
                    std_set_int__std_less_int___std_allocator_int__::new_1(
                        { Vec::from([1, 2, 3, 4, 5]) },
                        None,
                        None,
                    ),
                )
            })
        );
    }
    {
        assert!((gConstructed_0.with(|rc| *rc.borrow()) == 0));
    };
    {
        {
            let first: Value<
                std_set_Counter_int___std_less_Counter_int____std_allocator_Counter_int___,
            > = Rc::new(RefCell::new(
                std_set_Counter_int___std_less_Counter_int____std_allocator_Counter_int___::new_1(
                    {
                        Vec::from([
                            {
                                let __tmp_0: Value<i32> = Rc::new(RefCell::new(1));
                                Counter_int_::new_1({ __tmp_0.as_pointer() })
                            },
                            {
                                let __tmp_1: Value<i32> = Rc::new(RefCell::new(2));
                                Counter_int_::new_1({ __tmp_1.as_pointer() })
                            },
                            {
                                let __tmp_2: Value<i32> = Rc::new(RefCell::new(3));
                                Counter_int_::new_1({ __tmp_2.as_pointer() })
                            },
                        ])
                    },
                    None,
                    None,
                ),
            ));
            let second: Value<std_set_Counter_int___comparator__std_allocator_Counter_int___> =
                Rc::new(RefCell::new(
                    std_set_Counter_int___comparator__std_allocator_Counter_int___::new_1(
                        {
                            Vec::from([
                                {
                                    let __tmp_3: Value<i32> = Rc::new(RefCell::new(2));
                                    Counter_int_::new_1({ __tmp_3.as_pointer() })
                                },
                                {
                                    let __tmp_4: Value<i32> = Rc::new(RefCell::new(3));
                                    Counter_int_::new_1({ __tmp_4.as_pointer() })
                                },
                                {
                                    let __tmp_5: Value<i32> = Rc::new(RefCell::new(4));
                                    Counter_int_::new_1({ __tmp_5.as_pointer() })
                                },
                            ])
                        },
                        None,
                        None,
                    ),
                ));
            let third: Value<std_multiset_Counter_int___comparator__std_allocator_Counter_int___> =
                Rc::new(RefCell::new(
                    std_multiset_Counter_int___comparator__std_allocator_Counter_int___::new_1(
                        {
                            Vec::from([
                                {
                                    let __tmp_6: Value<i32> = Rc::new(RefCell::new(1));
                                    Counter_int_::new_1({ __tmp_6.as_pointer() })
                                },
                                {
                                    let __tmp_7: Value<i32> = Rc::new(RefCell::new(3));
                                    Counter_int_::new_1({ __tmp_7.as_pointer() })
                                },
                            ])
                        },
                        None,
                        None,
                    ),
                ));
            assert!((gConstructed_0.with(|rc| *rc.borrow()) == 8));
            ({ (*first.borrow()).merge(second.as_pointer()) });
            ({ (*first.borrow()).merge(third.as_pointer()) });
            assert!(
                ({
                    set_equal_4 ( first .as_pointer()  , std_set_Counter_int___std_less_Counter_int____std_allocator_Counter_int___ :: new_1 ( { Vec::from([ { let __tmp_8 : Value<i32 > = Rc::new(RefCell::new(1 )); Counter_int_ :: new_1 ( { __tmp_8.as_pointer() } , )  }  , { let __tmp_9 : Value<i32 > = Rc::new(RefCell::new(2 )); Counter_int_ :: new_1 ( { __tmp_9.as_pointer() } , )  }  , { let __tmp_10 : Value<i32 > = Rc::new(RefCell::new(3 )); Counter_int_ :: new_1 ( { __tmp_10.as_pointer() } , )  }  , { let __tmp_11 : Value<i32 > = Rc::new(RefCell::new(4 )); Counter_int_ :: new_1 ( { __tmp_11.as_pointer() } , )  }  , ] )  } , None , None , )   , )
                })
            );
            assert!(
                ({
                    set_equal_5(
                        second.as_pointer(),
                        std_set_Counter_int___comparator__std_allocator_Counter_int___::new_1(
                            {
                                Vec::from([
                                    {
                                        let __tmp_12: Value<i32> = Rc::new(RefCell::new(2));
                                        Counter_int_::new_1({ __tmp_12.as_pointer() })
                                    },
                                    {
                                        let __tmp_13: Value<i32> = Rc::new(RefCell::new(3));
                                        Counter_int_::new_1({ __tmp_13.as_pointer() })
                                    },
                                ])
                            },
                            None,
                            None,
                        ),
                    )
                })
            );
            assert!(
                ({
                    set_equal_6(
                        third.as_pointer(),
                        std_multiset_Counter_int___comparator__std_allocator_Counter_int___::new_1(
                            {
                                Vec::from([
                                    {
                                        let __tmp_14: Value<i32> = Rc::new(RefCell::new(1));
                                        Counter_int_::new_1({ __tmp_14.as_pointer() })
                                    },
                                    {
                                        let __tmp_15: Value<i32> = Rc::new(RefCell::new(3));
                                        Counter_int_::new_1({ __tmp_15.as_pointer() })
                                    },
                                ])
                            },
                            None,
                            None,
                        ),
                    )
                })
            );
            assert!((gConstructed_0.with(|rc| *rc.borrow()) == 8));
        }
        assert!((gConstructed_0.with(|rc| *rc.borrow()) == 0));
        {
            let first: Value<
                std_set_Counter_int___std_less_Counter_int____std_allocator_Counter_int___,
            > = Rc::new(RefCell::new(
                std_set_Counter_int___std_less_Counter_int____std_allocator_Counter_int___::new_1(
                    {
                        Vec::from([
                            {
                                let __tmp_16: Value<i32> = Rc::new(RefCell::new(1));
                                Counter_int_::new_1({ __tmp_16.as_pointer() })
                            },
                            {
                                let __tmp_17: Value<i32> = Rc::new(RefCell::new(2));
                                Counter_int_::new_1({ __tmp_17.as_pointer() })
                            },
                            {
                                let __tmp_18: Value<i32> = Rc::new(RefCell::new(3));
                                Counter_int_::new_1({ __tmp_18.as_pointer() })
                            },
                        ])
                    },
                    None,
                    None,
                ),
            ));
            let second: Value<std_set_Counter_int___comparator__std_allocator_Counter_int___> =
                Rc::new(RefCell::new(
                    std_set_Counter_int___comparator__std_allocator_Counter_int___::new_1(
                        {
                            Vec::from([
                                {
                                    let __tmp_19: Value<i32> = Rc::new(RefCell::new(2));
                                    Counter_int_::new_1({ __tmp_19.as_pointer() })
                                },
                                {
                                    let __tmp_20: Value<i32> = Rc::new(RefCell::new(3));
                                    Counter_int_::new_1({ __tmp_20.as_pointer() })
                                },
                                {
                                    let __tmp_21: Value<i32> = Rc::new(RefCell::new(4));
                                    Counter_int_::new_1({ __tmp_21.as_pointer() })
                                },
                            ])
                        },
                        None,
                        None,
                    ),
                ));
            let third: Value<std_multiset_Counter_int___comparator__std_allocator_Counter_int___> =
                Rc::new(RefCell::new(
                    std_multiset_Counter_int___comparator__std_allocator_Counter_int___::new_1(
                        {
                            Vec::from([
                                {
                                    let __tmp_22: Value<i32> = Rc::new(RefCell::new(1));
                                    Counter_int_::new_1({ __tmp_22.as_pointer() })
                                },
                                {
                                    let __tmp_23: Value<i32> = Rc::new(RefCell::new(3));
                                    Counter_int_::new_1({ __tmp_23.as_pointer() })
                                },
                            ])
                        },
                        None,
                        None,
                    ),
                ));
            assert!((gConstructed_0.with(|rc| *rc.borrow()) == 8));
            ({ (*first.borrow()).merge(second.as_pointer()) });
            ({ (*first.borrow()).merge(third.as_pointer()) });
            assert!(
                ({
                    set_equal_4 ( first .as_pointer()  , std_set_Counter_int___std_less_Counter_int____std_allocator_Counter_int___ :: new_1 ( { Vec::from([ { let __tmp_24 : Value<i32 > = Rc::new(RefCell::new(1 )); Counter_int_ :: new_1 ( { __tmp_24.as_pointer() } , )  }  , { let __tmp_25 : Value<i32 > = Rc::new(RefCell::new(2 )); Counter_int_ :: new_1 ( { __tmp_25.as_pointer() } , )  }  , { let __tmp_26 : Value<i32 > = Rc::new(RefCell::new(3 )); Counter_int_ :: new_1 ( { __tmp_26.as_pointer() } , )  }  , { let __tmp_27 : Value<i32 > = Rc::new(RefCell::new(4 )); Counter_int_ :: new_1 ( { __tmp_27.as_pointer() } , )  }  , ] )  } , None , None , )   , )
                })
            );
            assert!(
                ({
                    set_equal_5(
                        second.as_pointer(),
                        std_set_Counter_int___comparator__std_allocator_Counter_int___::new_1(
                            {
                                Vec::from([
                                    {
                                        let __tmp_28: Value<i32> = Rc::new(RefCell::new(2));
                                        Counter_int_::new_1({ __tmp_28.as_pointer() })
                                    },
                                    {
                                        let __tmp_29: Value<i32> = Rc::new(RefCell::new(3));
                                        Counter_int_::new_1({ __tmp_29.as_pointer() })
                                    },
                                ])
                            },
                            None,
                            None,
                        ),
                    )
                })
            );
            assert!(
                ({
                    set_equal_6(
                        third.as_pointer(),
                        std_multiset_Counter_int___comparator__std_allocator_Counter_int___::new_1(
                            {
                                Vec::from([
                                    {
                                        let __tmp_30: Value<i32> = Rc::new(RefCell::new(1));
                                        Counter_int_::new_1({ __tmp_30.as_pointer() })
                                    },
                                    {
                                        let __tmp_31: Value<i32> = Rc::new(RefCell::new(3));
                                        Counter_int_::new_1({ __tmp_31.as_pointer() })
                                    },
                                ])
                            },
                            None,
                            None,
                        ),
                    )
                })
            );
            assert!((gConstructed_0.with(|rc| *rc.borrow()) == 8));
        }
        assert!((gConstructed_0.with(|rc| *rc.borrow()) == 0));
    }
    {
        let first: Value<std_set_int__std_less_int___std_allocator_int__> = Rc::new(RefCell::new(
            <std_set_int__std_less_int___std_allocator_int__>::default(),
        ));
        {
            let second: Value<std_set_int__std_less_int___std_allocator_int__> = Rc::new(
                RefCell::new(<std_set_int__std_less_int___std_allocator_int__>::default()),
            );
            ({ (*first.borrow()).merge(second.as_pointer()) });
            ({ (*first.borrow()).merge(second.as_pointer()) });
        }
        {
            let second: Value<std_multiset_int__std_less_int___std_allocator_int__> = Rc::new(
                RefCell::new(<std_multiset_int__std_less_int___std_allocator_int__>::default()),
            );
            ({ (*first.borrow()).merge(second.as_pointer()) });
            ({ (*first.borrow()).merge(second.as_pointer()) });
        }
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct comparator_3 {}
impl Callable2<Ptr<Counter_int_>, Ptr<Counter_int_>, bool> for comparator_3 {
    fn call(&self, a1: Ptr<Counter_int_>, a2: Ptr<Counter_int_>) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            comparator_3Impl::operator_call(__it, a1, a2)
        })
    }
}
impl CppMove for comparator_3 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
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
    return 0;
}
pub trait Counter_int_Impl {
    fn copy_assign(&self, rhs: Ptr<Counter_int_>) -> Ptr<Counter_int_> {
        unimplemented!()
    }
    fn move_assign(&self, rhs: Ptr<Counter_int_>) -> Ptr<Counter_int_> {
        unimplemented!()
    }
    fn destructor(&self);
    fn get(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_eq(&self, x: Ptr<Counter_int_>) -> bool;
    fn operator_lt(&self, x: Ptr<Counter_int_>) -> bool;
}
impl Counter_int_Impl for Ptr<Counter_int_> {
    fn destructor(&self) {
        (*gConstructed_0.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn operator_eq(&self, x: Ptr<Counter_int_>) -> bool {
        return ({ (*self).with(|__s| __s.data_) } == { x.with(|__s| __s.data_) });
    }
    fn operator_lt(&self, x: Ptr<Counter_int_>) -> bool {
        return ({ (*self).with(|__s| __s.data_) } < { x.with(|__s| __s.data_) });
    }
}
pub trait comparator_3Impl {
    fn operator_call(&self, lhs: Ptr<Counter_int_>, rhs: Ptr<Counter_int_>) -> bool;
}
impl comparator_3Impl for Ptr<comparator_3> {
    fn operator_call(&self, lhs: Ptr<Counter_int_>, rhs: Ptr<Counter_int_>) -> bool {
        return ({
            let _x: Ptr<Counter_int_> = (rhs).clone();
            Counter_int_Impl::operator_lt(&lhs, _x)
        });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = gConstructed_0.with(|_| ());
}
