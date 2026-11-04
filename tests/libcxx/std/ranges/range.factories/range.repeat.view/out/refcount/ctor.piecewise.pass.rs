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
pub struct C {}
impl CppMove for C {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct B {
    #[offset(0)]
    pub v: i32,
}
impl CppMove for B {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(B, new_1, value)]
#[byte_size(8)]
pub struct A {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl A {
    pub fn new_1(mut b: B) -> Self {
        Self {
            x: b.v,
            y: (b.v + 1),
        }
    }
    pub fn new_2(mut _x: i32, mut _y: i32) -> Self {
        Self { x: _x, y: _y }
    }
}
impl CppMove for A {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for A {
    fn default() -> Self {
        { A { x: 111, y: 222 } }
    }
}
const _: () = assert!(
    true,
    "std::constructible_from<std::ranges::repeat_view<A, int>,\n                                      std::piecewise_construct_t,\n                                      std::tuple<int, int>,\n                                      std::tuple<int>>"
);
const _: () = assert!(
    true,
    "std::constructible_from<std::ranges::repeat_view<A, int>,\n                                      std::piecewise_construct_t,\n                                      std::tuple<B>,\n                                      std::tuple<int>>"
);
const _: () = assert!(
    true,
    "std::constructible_from<std::ranges::repeat_view<A, int>,\n                                      std::piecewise_construct_t,\n                                      std::tuple<>,\n                                      std::tuple<int>>"
);
const _: () = assert!(
    true,
    "std::constructible_from<std::ranges::repeat_view<A>,\n                                      std::piecewise_construct_t,\n                                      std::tuple<int, int>,\n                                      std::tuple<std::unreachable_sentinel_t>>"
);
const _: () = assert!(
    true,
    "std::constructible_from<std::ranges::repeat_view<A>,\n                                      std::piecewise_construct_t,\n                                      std::tuple<B>,\n                                      std::tuple<std::unreachable_sentinel_t>>"
);
const _: () = assert!(
    true,
    "std::constructible_from<std::ranges::repeat_view<A>,\n                                      std::piecewise_construct_t,\n                                      std::tuple<>,\n                                      std::tuple<std::unreachable_sentinel_t>>"
);
const _: () = assert!(
    !(false),
    "!std::constructible_from<std::ranges::repeat_view<A, int>,\n                                       std::piecewise_construct_t,\n                                       std::tuple<C>,\n                                       std::tuple<int>>"
);
const _: () = assert!(
    !(false),
    "!std::constructible_from<std::ranges::repeat_view<A>,\n                                       std::piecewise_construct_t,\n                                       std::tuple<C>,\n                                       std::tuple<std::unreachable_sentinel_t>>"
);
const _: () = assert!(
    !(false),
    "!std::constructible_from<std::ranges::repeat_view<A, int>,\n                                       std::piecewise_construct_t,\n                                       std::tuple<int, int, int>,\n                                       std::tuple<int>>"
);
const _: () = assert!(
    !(false),
    "!std::constructible_from<std::ranges::repeat_view<A>,\n                                       std::piecewise_construct_t,\n                                       std::tuple<int, int, int>,\n                                       std::tuple<std::unreachable_sentinel_t>>"
);
const _: () = assert!(
    !(false),
    "!std::constructible_from<std::ranges::repeat_view<A>, std::piecewise_construct_t, std::tuple<B>, std::tuple<int>>"
);
pub fn test_0() -> bool {
    {
        let rv: Value<std_ranges_repeat_view_A__int_> =
            Rc::new(RefCell::new(std_ranges_repeat_view_A__int_::new_1(
                { (*piecewise_construct_1.with(Value::clone).borrow()).clone() },
                { <std_tuple__>::default() },
                { 3 },
            )));
        assert!(({ (({ (*rv.borrow()).size() }) == 3_u32) }));
        assert!(
            ({
                ({
                    ((*rv.borrow()) as std_ranges_view_interface_std_ranges_repeat_view_A__int__)[0]
                        .x
                } == 111)
            })
        );
        assert!(
            ({
                ({
                    ((*rv.borrow()) as std_ranges_view_interface_std_ranges_repeat_view_A__int__)[0]
                        .y
                } == 222)
            })
        );
        assert!(({ ({ (*rv.borrow()).begin() }) }).iter_add(3) == ({ (*rv.borrow()).end_6() }));
    }
    {
        let rv: Value<std_ranges_repeat_view_A__std_unreachable_sentinel_t_> =
            Rc::new(RefCell::new(
                std_ranges_repeat_view_A__std_unreachable_sentinel_t_::new_1(
                    { (*piecewise_construct_1.with(Value::clone).borrow()).clone() },
                    { <std_tuple__>::default() },
                    {
                        std_tuple_std_unreachable_sentinel_t_::new_1({
                            unreachable_sentinel_2.with(|v| v.as_pointer())
                        })
                    },
                ),
            ));
        assert!(
            ({
                ({
                    ( (*rv.borrow()) as std_ranges_view_interface_std_ranges_repeat_view_A__std_unreachable_sentinel_t__ ) [0_i64 ] . x
                } == 111)
            })
        );
        assert!(
            ({
                ({
                    ( (*rv.borrow()) as std_ranges_view_interface_std_ranges_repeat_view_A__std_unreachable_sentinel_t__ ) [0_i64 ] . y
                } == 222)
            })
        );
        assert!(
            ({ ({ (*rv.borrow()).begin() }) }).iter_add(300_isize)
                != ({ ({ (*rv.borrow()).end_7() }) })
        );
    }
    {
        let rv: Value<std_ranges_repeat_view_A__int_> =
            Rc::new(RefCell::new(std_ranges_repeat_view_A__int_::new_2(
                { (*piecewise_construct_1.with(Value::clone).borrow()).clone() },
                {
                    {
                        let __tmp_0: Value<i32> = Rc::new(RefCell::new(1));
                        let __tmp_1: Value<i32> = Rc::new(RefCell::new(2));
                        std_tuple_int__int_::new_1({ __tmp_0.as_pointer() }, {
                            __tmp_1.as_pointer()
                        })
                    }
                },
                { 3 },
            )));
        assert!(({ (({ (*rv.borrow()).size() }) == 3_u32) }));
        assert!(
            ({
                ({
                    ((*rv.borrow()) as std_ranges_view_interface_std_ranges_repeat_view_A__int__)[0]
                        .x
                } == 1)
            })
        );
        assert!(
            ({
                ({
                    ((*rv.borrow()) as std_ranges_view_interface_std_ranges_repeat_view_A__int__)[0]
                        .y
                } == 2)
            })
        );
        assert!(({ ({ (*rv.borrow()).begin() }) }).iter_add(3) == ({ (*rv.borrow()).end_6() }));
    }
    {
        let rv: Value<std_ranges_repeat_view_A__std_unreachable_sentinel_t_> =
            Rc::new(RefCell::new(
                std_ranges_repeat_view_A__std_unreachable_sentinel_t_::new_2(
                    { (*piecewise_construct_1.with(Value::clone).borrow()).clone() },
                    {
                        {
                            let __tmp_2: Value<i32> = Rc::new(RefCell::new(1));
                            let __tmp_3: Value<i32> = Rc::new(RefCell::new(2));
                            std_tuple_int__int_::new_1({ __tmp_2.as_pointer() }, {
                                __tmp_3.as_pointer()
                            })
                        }
                    },
                    {
                        std_tuple_std_unreachable_sentinel_t_::new_1({
                            unreachable_sentinel_2.with(|v| v.as_pointer())
                        })
                    },
                ),
            ));
        assert!(
            ({
                ({
                    ( (*rv.borrow()) as std_ranges_view_interface_std_ranges_repeat_view_A__std_unreachable_sentinel_t__ ) [0_i64 ] . x
                } == 1)
            })
        );
        assert!(
            ({
                ({
                    ( (*rv.borrow()) as std_ranges_view_interface_std_ranges_repeat_view_A__std_unreachable_sentinel_t__ ) [0_i64 ] . y
                } == 2)
            })
        );
        assert!(
            ({ ({ (*rv.borrow()).begin() }) }).iter_add(300_isize)
                != ({ ({ (*rv.borrow()).end_7() }) })
        );
    }
    {
        let rv: Value<std_ranges_repeat_view_A__int_> =
            Rc::new(RefCell::new(std_ranges_repeat_view_A__int_::new_3(
                { (*piecewise_construct_1.with(Value::clone).borrow()).clone() },
                { B { v: 11 } },
                { 3 },
            )));
        assert!(({ (({ (*rv.borrow()).size() }) == 3_u32) }));
        assert!(
            ({
                ({
                    ((*rv.borrow()) as std_ranges_view_interface_std_ranges_repeat_view_A__int__)[0]
                        .x
                } == 11)
            })
        );
        assert!(
            ({
                ({
                    ((*rv.borrow()) as std_ranges_view_interface_std_ranges_repeat_view_A__int__)[0]
                        .y
                } == 12)
            })
        );
        assert!(({ ({ (*rv.borrow()).begin() }) }).iter_add(3) == ({ (*rv.borrow()).end_6() }));
    }
    {
        let rv: Value<std_ranges_repeat_view_A__std_unreachable_sentinel_t_> =
            Rc::new(RefCell::new(
                std_ranges_repeat_view_A__std_unreachable_sentinel_t_::new_3(
                    { (*piecewise_construct_1.with(Value::clone).borrow()).clone() },
                    { B { v: 10 } },
                    {
                        std_tuple_std_unreachable_sentinel_t_::new_1({
                            unreachable_sentinel_2.with(|v| v.as_pointer())
                        })
                    },
                ),
            ));
        assert!(
            ({
                ({
                    ( (*rv.borrow()) as std_ranges_view_interface_std_ranges_repeat_view_A__std_unreachable_sentinel_t__ ) [0_i64 ] . x
                } == 10)
            })
        );
        assert!(
            ({
                ({
                    ( (*rv.borrow()) as std_ranges_view_interface_std_ranges_repeat_view_A__std_unreachable_sentinel_t__ ) [0_i64 ] . y
                } == 11)
            })
        );
        assert!(
            ({ ({ (*rv.borrow()).begin() }) }).iter_add(300_isize)
                != ({ ({ (*rv.borrow()).end_7() }) })
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
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
