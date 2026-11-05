extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn inc_0(mut p: Ptr<i32>) {
    {
        p.with_mut(|__v| *__v = *__v + 1)
    };
}
pub fn add_1(mut p: Ptr<i32>, mut n: i32) {
    {
        let __rhs = { n };
        p.with_mut(|__v| *__v = *__v + __rhs)
    };
}
pub fn twice_2(mut n: i32) -> i32 {
    return (n * 2);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Access_S_ {}
impl CppMove for Access_S_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub base: i32,
}
impl CppMove for S {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Box_ {
    #[offset(0)]
    pub v: i32,
}
impl CppMove for Box_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S { base: 100 }));
    ({ cpp2rust_assert_fail_3((({ SImpl::width_1_char(&s.as_pointer(), 3) }) == 103)) });
    ({ cpp2rust_assert_fail_3((({ SImpl::width_1_int(&s.as_pointer(), 3) }) == 112)) });
    ({ cpp2rust_assert_fail_3((({ SImpl::scale_2_2(&s.as_pointer(), 5) }) == 110)) });
    ({ cpp2rust_assert_fail_3((({ SImpl::scale_2_3(&s.as_pointer(), 5) }) == 115)) });
    ({ cpp2rust_assert_fail_3((({ SImpl::count_3(&s.as_pointer(), 1) }) == 101)) });
    ({ cpp2rust_assert_fail_3((({ SImpl::count_3_int_long(&s.as_pointer(), 1) }) == 103)) });
    ({ cpp2rust_assert_fail_3((({ SImpl::plain_4(&s.as_pointer(), 1) }) == 101)) });
    ({ cpp2rust_assert_fail_3((({ SImpl::plain_5(&s.as_pointer(), 1_i64) }) == 102)) });
    let y: Value<i32> = Rc::new(RefCell::new(1));
    ({ cpp2rust_assert_fail_3((({ SImpl::take_6(&s.as_pointer(), y.as_pointer()) }) == 102)) });
    ({
        cpp2rust_assert_fail_3(
            (({
                let _x: Value<i32> = Rc::new(RefCell::new(5));
                SImpl::take_7(&s.as_pointer(), _x.as_pointer())
            }) == 107),
        )
    });
    ({
        cpp2rust_assert_fail_3(
            (({
                SImpl::pick_8(&s.as_pointer(), {
                    let __tmp_0: Value<i32> = Rc::new(RefCell::new(1));
                    let __tmp_1: Value<i32> = Rc::new(RefCell::new(2));
                    std_pair_int__int_::new_1({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
                })
            }) == 101),
        )
    });
    ({
        cpp2rust_assert_fail_3(
            (({
                SImpl::pick_9(&s.as_pointer(), {
                    let __tmp_2: Value<i32> = Rc::new(RefCell::new(1));
                    let __tmp_3: Value<i64> = Rc::new(RefCell::new(2_i64));
                    std_pair_int__long_::new_1({ __tmp_2.as_pointer() }, { __tmp_3.as_pointer() })
                })
            }) == 102),
        )
    });
    ({
        cpp2rust_assert_fail_3(
            (({ SImpl::apply_10(&s.as_pointer(), FnPtr::<fn(Ptr<i32>)>::new(inc_0), 1) }) == 102),
        )
    });
    ({
        cpp2rust_assert_fail_3(
            (({ SImpl::apply_11(&s.as_pointer(), FnPtr::<fn(Ptr<i32>, i32)>::new(add_1), 1) })
                == 111),
        )
    });
    ({
        cpp2rust_assert_fail_3(
            (({ SImpl::apply_12(&s.as_pointer(), FnPtr::<fn(i32) -> i32>::new(twice_2), 3) })
                == 106),
        )
    });
    let c: Value<i32> = Rc::new(RefCell::new(3));
    ({
        cpp2rust_assert_fail_3(
            (({
                let _p: Value<std_pair_int__long_> = Rc::new(RefCell::new({
                    let __tmp_4: Value<i32> = Rc::new(RefCell::new(1));
                    let __tmp_5: Value<i64> = Rc::new(RefCell::new(2_i64));
                    std_pair_int__long_::new_1({ __tmp_4.as_pointer() }, { __tmp_5.as_pointer() })
                }));
                SImpl::combine_13(
                    &s.as_pointer(),
                    _p.as_pointer(),
                    FnPtr::<fn(i32) -> i32>::new(twice_2),
                    (c.as_pointer()),
                    4_u64,
                )
            }) == 112),
        )
    });
    let z: Value<i32> = Rc::new(RefCell::new(1));
    ({
        cpp2rust_assert_fail_3(
            (({
                let _p: Value<std_pair_int__int_> = Rc::new(RefCell::new({
                    let __tmp_6: Value<i32> = Rc::new(RefCell::new(1));
                    let __tmp_7: Value<i32> = Rc::new(RefCell::new(2));
                    std_pair_int__int_::new_1({ __tmp_6.as_pointer() }, { __tmp_7.as_pointer() })
                }));
                SImpl::combine_14(
                    &s.as_pointer(),
                    _p.as_pointer(),
                    FnPtr::<fn(Ptr<i32>, i32)>::new(add_1),
                    (z.as_pointer()),
                    5_u64,
                )
            }) == 107),
        )
    });
    ({ cpp2rust_assert_fail_3(((*z.borrow()) == 6)) });
    let a: Value<Access_S_> = Rc::new(RefCell::new(<Access_S_>::default()));
    let mut cs: Ptr<S> = (s.as_pointer());
    let cr: Ptr<S> = s.as_pointer();
    ({
        cpp2rust_assert_fail_3(
            (({ Access_S_Impl::get_1(&a.as_pointer(), (s.as_pointer())) }) == 100),
        )
    });
    ({
        cpp2rust_assert_fail_3((({ Access_S_Impl::get_2(&a.as_pointer(), (cs).clone()) }) == 101))
    });
    ({
        cpp2rust_assert_fail_3((({ Access_S_Impl::ref_3(&a.as_pointer(), s.as_pointer()) }) == 102))
    });
    ({
        cpp2rust_assert_fail_3(
            (({
                let _r: Ptr<S> = (cr).clone();
                Access_S_Impl::ref_4(&a.as_pointer(), _r)
            }) == 103),
        )
    });
    let mut b: Box_ = Box_ { v: 4 };
    ({ cpp2rust_assert_fail_3((b.v == 4)) });
    return 0;
}
pub trait Access_S_Impl {
    fn get_1(&self, p: Ptr<S>) -> i32;
    fn get_2(&self, p: Ptr<S>) -> i32;
    fn ref_3(&self, r: Ptr<S>) -> i32;
    fn ref_4(&self, r: Ptr<S>) -> i32;
}
impl Access_S_Impl for Ptr<Access_S_> {
    fn get_1(&self, mut p: Ptr<S>) -> i32 {
        return p.with(|__s| __s.base);
    }
    fn get_2(&self, mut p: Ptr<S>) -> i32 {
        return (p.with(|__s| __s.base) + 1);
    }
    fn ref_3(&self, r: Ptr<S>) -> i32 {
        return (r.with(|__s| __s.base) + 2);
    }
    fn ref_4(&self, r: Ptr<S>) -> i32 {
        return (r.with(|__s| __s.base) + 3);
    }
}
pub trait SImpl {
    fn plain_4(&self, x: i32) -> i32;
    fn plain_5(&self, x: i64) -> i32;
    fn take_6(&self, x: Ptr<i32>) -> i32;
    fn take_7(&self, x: Ptr<i32>) -> i32;
    fn pick_8(&self, p: std_pair_int__int_) -> i32;
    fn pick_9(&self, p: std_pair_int__long_) -> i32;
    fn apply_10(&self, f: FnPtr<fn(Ptr<i32>)>, x: i32) -> i32;
    fn apply_11(&self, f: FnPtr<fn(Ptr<i32>, i32)>, x: i32) -> i32;
    fn apply_12(&self, f: FnPtr<fn(i32) -> i32>, x: i32) -> i32;
    fn combine_13(
        &self,
        p: Ptr<std_pair_int__long_>,
        f: FnPtr<fn(i32) -> i32>,
        q: Ptr<i32>,
        n: u64,
    ) -> i32;
    fn combine_14(
        &self,
        p: Ptr<std_pair_int__int_>,
        f: FnPtr<fn(Ptr<i32>, i32)>,
        q: Ptr<i32>,
        n: u64,
    ) -> i32;
    fn width_1_char(&self, x: i32) -> i32;
    fn width_1_int(&self, x: i32) -> i32;
    fn scale_2_2(&self, x: i32) -> i32;
    fn scale_2_3(&self, x: i32) -> i32;
    fn count_3(&self, x: i32) -> i32;
    fn count_3_int_long(&self, x: i32) -> i32;
}
impl SImpl for Ptr<S> {
    fn plain_4(&self, mut x: i32) -> i32 {
        return ((*self).with(|__s| __s.base) + x);
    }
    fn plain_5(&self, mut x: i64) -> i32 {
        return (((*self).with(|__s| __s.base) + (x as i32)) + 1);
    }
    fn take_6(&self, x: Ptr<i32>) -> i32 {
        return (({ (*self).with(|__s| __s.base) } + { (x.read()) }) + 1);
    }
    fn take_7(&self, x: Ptr<i32>) -> i32 {
        return (({ (*self).with(|__s| __s.base) } + { (x.read()) }) + 2);
    }
    fn pick_8(&self, p: std_pair_int__int_) -> i32 {
        let p: Value<std_pair_int__int_> = Rc::new(RefCell::new(p));
        return ((*self).with(|__s| __s.base) + { (*p.borrow()).first });
    }
    fn pick_9(&self, p: std_pair_int__long_) -> i32 {
        let p: Value<std_pair_int__long_> = Rc::new(RefCell::new(p));
        return ((*self).with(|__s| __s.base) + ({ (*p.borrow()).second } as i32));
    }
    fn apply_10(&self, mut f: FnPtr<fn(Ptr<i32>)>, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        ({ f.call((x.as_pointer())) });
        return ((*self).with(|__s| __s.base) + (*x.borrow()));
    }
    fn apply_11(&self, mut f: FnPtr<fn(Ptr<i32>, i32)>, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        ({ f.call((x.as_pointer()), 10) });
        return ((*self).with(|__s| __s.base) + (*x.borrow()));
    }
    fn apply_12(&self, mut f: FnPtr<fn(i32) -> i32>, mut x: i32) -> i32 {
        return ({ (*self).with(|__s| __s.base) } + { ({ f.call(x) }) });
    }
    fn combine_13(
        &self,
        p: Ptr<std_pair_int__long_>,
        mut f: FnPtr<fn(i32) -> i32>,
        mut q: Ptr<i32>,
        mut n: u64,
    ) -> i32 {
        return ({
            ({ ({ (*self).with(|__s| __s.base) } + { (p.with(|__s| __s.second) as i32) }) } + {
                ({ f.call((q.read())) })
            })
        } + { (n as i32) });
    }
    fn combine_14(
        &self,
        p: Ptr<std_pair_int__int_>,
        mut f: FnPtr<fn(Ptr<i32>, i32)>,
        mut q: Ptr<i32>,
        mut n: u64,
    ) -> i32 {
        ({
            let _arg0: Ptr<i32> = (q).clone();
            let _arg1: i32 = (n as i32);
            f.call(_arg0, _arg1)
        });
        return ({ ({ (*self).with(|__s| __s.base) } + { p.with(|__s| __s.first) }) } + {
            (q.read())
        });
    }
    fn width_1_char(&self, mut x: i32) -> i32 {
        return ((*self).with(|__s| __s.base) + (x * (::std::mem::size_of::<i8>() as i32)));
    }
    fn width_1_int(&self, mut x: i32) -> i32 {
        return ((*self).with(|__s| __s.base) + (x * (::std::mem::size_of::<i32>() as i32)));
    }
    fn scale_2_2(&self, mut x: i32) -> i32 {
        return ((*self).with(|__s| __s.base) + (x * 2));
    }
    fn scale_2_3(&self, mut x: i32) -> i32 {
        return ((*self).with(|__s| __s.base) + (x * 3));
    }
    fn count_3(&self, mut x: i32) -> i32 {
        return (((*self).with(|__s| __s.base) + x) + (0 as i32));
    }
    fn count_3_int_long(&self, mut x: i32) -> i32 {
        return (((*self).with(|__s| __s.base) + x) + (2 as i32));
    }
}
pub fn __cpp2rust_init_globals() {}
