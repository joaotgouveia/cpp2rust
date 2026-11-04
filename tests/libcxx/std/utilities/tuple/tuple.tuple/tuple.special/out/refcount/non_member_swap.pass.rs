extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_1(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_2(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_3(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_4(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_5(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_6(_a0: Ptr<MoveOnly>, _a1: Ptr<MoveOnly>) -> std::cmp::Ordering {
    {
        let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !(({ (*cmp.borrow()) }) == std::cmp::Ordering::Equal) {
            return ({ (*cmp.borrow()) });
        }
    }
    return ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) });
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, CppFrom, MoveAssign)]
#[cpp_from(i32, new, some_value)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    data_: i32,
}
impl MoveOnly {
    pub fn new(data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| 1);
        Self { data_: data }
    }
    pub fn move_from(x: Ptr<MoveOnly>) -> Self {
        let __this: MoveOnly = Self {
            data_: x.with(|__s| __s.data_),
        };
        field!(x, data_).write(0);
        __this
    }
}
impl CppLess for MoveOnly {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_2(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for MoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_0(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for MoveOnly {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for MoveOnly {
    fn default() -> Self {
        { MoveOnly::new(None) }
    }
}
pub fn operator_eq_7(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_8(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_9(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_10(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_11(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_12(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_13(_a0: Ptr<TrivialMoveOnly>, _a1: Ptr<TrivialMoveOnly>) -> std::cmp::Ordering {
    {
        let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !(({ (*cmp.borrow()) }) == std::cmp::Ordering::Equal) {
            return ({ (*cmp.borrow()) });
        }
    }
    return ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) });
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, CppFrom, MoveAssign)]
#[cpp_from(i32, new, some_value)]
#[byte_size(4)]
pub struct TrivialMoveOnly {
    #[offset(0)]
    data_: i32,
}
impl TrivialMoveOnly {
    pub fn new(data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| 1);
        Self { data_: data }
    }
    pub fn move_from(_a0: Ptr<TrivialMoveOnly>) -> Self {
        Self {
            data_: { (*_a0.upgrade().deref()).data_ },
        }
    }
}
impl CppLess for TrivialMoveOnly {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_9(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for TrivialMoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_7(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for TrivialMoveOnly {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        { TrivialMoveOnly::new(None) }
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
    {
        let t0: Value<std_tuple__> = Rc::new(RefCell::new(<std_tuple__>::default()));
        let t1: Value<std_tuple__> = Rc::new(RefCell::new(<std_tuple__>::default()));
        cpp_swap!(t0.as_pointer(), t1.as_pointer());
    }
    {
        let t0: Value<std_tuple_MoveOnly_> = Rc::new(RefCell::new(MoveOnly::new({ Some(0) })));
        let t1: Value<std_tuple_MoveOnly_> = Rc::new(RefCell::new(MoveOnly::new({ Some(1) })));
        cpp_swap!(t0.as_pointer(), t1.as_pointer());
        assert!(
            ({
                let _x: Ptr<MoveOnly> = ({ get_14(t0.as_pointer()) });
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(1) })));
                operator_eq_0(_x, _y.as_pointer())
            })
        );
        assert!(
            ({
                let _x: Ptr<MoveOnly> = ({ get_14(t1.as_pointer()) });
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(0) })));
                operator_eq_0(_x, _y.as_pointer())
            })
        );
    }
    {
        let t0: Value<std_tuple_MoveOnly__MoveOnly_> = Rc::new(RefCell::new({
            let __tmp_0: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(0) })));
            let __tmp_1: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(1) })));
            std_tuple_MoveOnly__MoveOnly_::new_1({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
        }));
        let t1: Value<std_tuple_MoveOnly__MoveOnly_> = Rc::new(RefCell::new({
            let __tmp_2: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(2) })));
            let __tmp_3: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(3) })));
            std_tuple_MoveOnly__MoveOnly_::new_1({ __tmp_2.as_pointer() }, { __tmp_3.as_pointer() })
        }));
        cpp_swap!(t0.as_pointer(), t1.as_pointer());
        assert!(
            ({
                let _x: Ptr<MoveOnly> = ({ get_15(t0.as_pointer()) });
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(2) })));
                operator_eq_0(_x, _y.as_pointer())
            })
        );
        assert!(
            ({
                let _x: Ptr<MoveOnly> = ({ get_16(t0.as_pointer()) });
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(3) })));
                operator_eq_0(_x, _y.as_pointer())
            })
        );
        assert!(
            ({
                let _x: Ptr<MoveOnly> = ({ get_15(t1.as_pointer()) });
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(0) })));
                operator_eq_0(_x, _y.as_pointer())
            })
        );
        assert!(
            ({
                let _x: Ptr<MoveOnly> = ({ get_16(t1.as_pointer()) });
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(1) })));
                operator_eq_0(_x, _y.as_pointer())
            })
        );
    }
    {
        let t0: Value<std_tuple_MoveOnly__MoveOnly__MoveOnly_> = Rc::new(RefCell::new({
            let __tmp_4: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(0) })));
            let __tmp_5: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(1) })));
            let __tmp_6: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(2) })));
            std_tuple_MoveOnly__MoveOnly__MoveOnly_::new_1(
                { __tmp_4.as_pointer() },
                { __tmp_5.as_pointer() },
                { __tmp_6.as_pointer() },
            )
        }));
        let t1: Value<std_tuple_MoveOnly__MoveOnly__MoveOnly_> = Rc::new(RefCell::new({
            let __tmp_7: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(3) })));
            let __tmp_8: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(4) })));
            let __tmp_9: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(5) })));
            std_tuple_MoveOnly__MoveOnly__MoveOnly_::new_1(
                { __tmp_7.as_pointer() },
                { __tmp_8.as_pointer() },
                { __tmp_9.as_pointer() },
            )
        }));
        cpp_swap!(t0.as_pointer(), t1.as_pointer());
        assert!(
            ({
                let _x: Ptr<MoveOnly> = ({ get_17(t0.as_pointer()) });
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(3) })));
                operator_eq_0(_x, _y.as_pointer())
            })
        );
        assert!(
            ({
                let _x: Ptr<MoveOnly> = ({ get_18(t0.as_pointer()) });
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(4) })));
                operator_eq_0(_x, _y.as_pointer())
            })
        );
        assert!(
            ({
                let _x: Ptr<MoveOnly> = ({ get_19(t0.as_pointer()) });
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(5) })));
                operator_eq_0(_x, _y.as_pointer())
            })
        );
        assert!(
            ({
                let _x: Ptr<MoveOnly> = ({ get_17(t1.as_pointer()) });
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(0) })));
                operator_eq_0(_x, _y.as_pointer())
            })
        );
        assert!(
            ({
                let _x: Ptr<MoveOnly> = ({ get_18(t1.as_pointer()) });
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(1) })));
                operator_eq_0(_x, _y.as_pointer())
            })
        );
        assert!(
            ({
                let _x: Ptr<MoveOnly> = ({ get_19(t1.as_pointer()) });
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(2) })));
                operator_eq_0(_x, _y.as_pointer())
            })
        );
    }
    return 0;
}
pub trait MoveOnlyImpl {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly>;
    fn get(&self) -> i32;
    fn operator_add(&self, x: Ptr<MoveOnly>) -> MoveOnly;
    fn operator_mul(&self, x: Ptr<MoveOnly>) -> MoveOnly;
}
impl MoveOnlyImpl for Ptr<MoveOnly> {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        field!((*self), data_).write({ x.with(|__s| __s.data_) });
        field!(x, data_).write(0);
        return (*self).clone();
    }
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.data_);
    }
    fn operator_add(&self, x: Ptr<MoveOnly>) -> MoveOnly {
        return MoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } + { x.with(|__s| __s.data_) }))
        });
    }
    fn operator_mul(&self, x: Ptr<MoveOnly>) -> MoveOnly {
        return MoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } * { x.with(|__s| __s.data_) }))
        });
    }
}
pub trait TrivialMoveOnlyImpl {
    fn move_assign(&self, _a0: Ptr<TrivialMoveOnly>) -> Ptr<TrivialMoveOnly>;
    fn get(&self) -> i32;
    fn operator_add(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
    fn operator_mul(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
}
impl TrivialMoveOnlyImpl for Ptr<TrivialMoveOnly> {
    fn move_assign(&self, _a0: Ptr<TrivialMoveOnly>) -> Ptr<TrivialMoveOnly> {
        field!((*self), data_).write({ { (*_a0.upgrade().deref()).data_ } });
        return (*self).clone();
    }
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.data_);
    }
    fn operator_add(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } + { x.with(|__s| __s.data_) }))
        });
    }
    fn operator_mul(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } * { x.with(|__s| __s.data_) }))
        });
    }
}
pub fn __cpp2rust_init_globals() {}
