extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_1(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_2(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_3(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_4(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_5(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_6(_a0: Ptr<MoveOnly>, _a1: Ptr<MoveOnly>) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    data_: (),
}
impl MoveOnly {
    pub fn new(data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| 1_());
        Self { data_: data }
    }
    pub fn move_from(x: Ptr<MoveOnly>) -> Self {
        let __this: MoveOnly = Self {
            data_: x.with(|__s| __s.data_),
        };
        field!(x, data_).write(0_());
        __this
    }
}
impl std::cmp::Ord for MoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_6(
                Rc::new(RefCell::new(MoveOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(MoveOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for MoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MoveOnly {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_0(
                Rc::new(RefCell::new(MoveOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(MoveOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for MoveOnly {}
impl Default for MoveOnly {
    fn default() -> Self {
        { MoveOnly::new(None) }
    }
}
pub fn operator_eq_7(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_8(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_9(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_10(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_11(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_12(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_13(_a0: Ptr<TrivialMoveOnly>, _a1: Ptr<TrivialMoveOnly>) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(4)]
pub struct TrivialMoveOnly {
    #[offset(0)]
    data_: (),
}
impl TrivialMoveOnly {
    pub fn new(data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| 1_());
        Self { data_: data }
    }
    pub fn move_from(_a0: Ptr<TrivialMoveOnly>) -> Self {
        Self {
            data_: { (*_a0.upgrade().deref()).data_ },
        }
    }
}
impl std::cmp::Ord for TrivialMoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_13(
                Rc::new(RefCell::new(TrivialMoveOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(TrivialMoveOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for TrivialMoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for TrivialMoveOnly {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_7(
                Rc::new(RefCell::new(TrivialMoveOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(TrivialMoveOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for TrivialMoveOnly {}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        { TrivialMoveOnly::new(None) }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NonAssignable {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CopyAssignable {}
const _: () = assert!(true, "std::is_copy_assignable<CopyAssignable>::value");
#[derive(Record, ByteRepr, FnPtrArg, MoveAssign, Default)]
#[byte_size(1)]
pub struct MoveAssignable {}
#[derive(Record, ByteRepr, FnPtrArg, MoveAssign, Default)]
#[byte_size(1)]
pub struct NothrowMoveAssignable {}
#[derive(Record, ByteRepr, FnPtrArg, MoveAssign, Default)]
#[byte_size(1)]
pub struct PotentiallyThrowingMoveAssignable {}
#[derive(Record, ByteRepr, FnPtrArg, CopyAssign, MoveAssign, Default)]
#[byte_size(1)]
pub struct CountAssign {}
impl CountAssign {
    pub fn reset() {
        copied_14.with(|rc| {
            *rc.borrow_mut() = {
                moved_15.with(|rc| *rc.borrow_mut() = 0_());
                moved_15.with(|rc| *rc.borrow())
            }
        });
    }
}
thread_local!(
    pub static copied_14: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static moved_15: Value<()> = Rc::new(RefCell::new(0_()));
);
pub fn test_16() -> () {
    {
        let t0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*t.borrow_mut()) = (*t0.borrow()).clone();
    }
    {
        let t0: Value<()> = Rc::new(RefCell::new(MoveOnly::new({ Some(0_()) })));
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*t.borrow_mut()) = (*t0.borrow()).clone();
        unimplemented!();
    }
    {
        let t0: Value<()> = Rc::new(RefCell::new({
            let __tmp_0: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(0_()) })));
            let __tmp_1: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(1_()) })));
            std_tuple_MoveOnly__MoveOnly_::new_1({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
        }));
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*t.borrow_mut()) = (*t0.borrow()).clone();
        unimplemented!();
        unimplemented!();
    }
    {
        let t0: Value<()> = Rc::new(RefCell::new({
            let __tmp_2: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(0_()) })));
            let __tmp_3: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(1_()) })));
            let __tmp_4: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(2_()) })));
            std_tuple_MoveOnly__MoveOnly__MoveOnly_::new_1(
                { __tmp_2.as_pointer() },
                { __tmp_3.as_pointer() },
                { __tmp_4.as_pointer() },
            )
        }));
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*t.borrow_mut()) = (*t0.borrow()).clone();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let x: Value<()> = Rc::new(RefCell::new(42_()));
        let y: Value<()> = Rc::new(RefCell::new(100_()));
        let x2: Value<()> = Rc::new(RefCell::new(-1_()));
        let y2: Value<()> = Rc::new(RefCell::new(500_()));
        let t: Value<()> = Rc::new(RefCell::new(std_tuple_int_ref__int_refref_::new(
            { x.as_pointer() },
            { (y.as_pointer()).reinterpret_cast::<()>() },
        )));
        let t2: Value<()> = Rc::new(RefCell::new(std_tuple_int_ref__int_refref_::new(
            { x2.as_pointer() },
            { (y2.as_pointer()).reinterpret_cast::<()>() },
        )));
        (*t.borrow_mut()) = (*t2.borrow()).clone();
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
    ({ test_16() });
    const _: () = assert!(true, "test()");;
    {
        const _: () = assert!(true, "std::is_move_assignable<T>::value");;
        const _: () = assert!(true, "!std::is_copy_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_move_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "std::is_move_assignable<T>::value");;
    }
    {
        ({ CountAssign::reset() });
        const _: () = assert!(true, "std::is_move_assignable<T>::value");;
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*t1.borrow_mut()) = (*t2.borrow()).clone();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "!std::is_move_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "std::is_move_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "std::is_nothrow_move_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_nothrow_move_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_move_assignable<T1>::value");;
        const _: () = assert!(true, "std::is_move_assignable<T2>::value");;
        const _: () = assert!(true, "std::is_move_assignable<T3>::value");;
        const _: () = assert!(true, "std::is_move_assignable<T4>::value");;
        const _: () = assert!(true, "!std::is_move_assignable<T5>::value");;
        const _: () = assert!(true, "!std::is_move_assignable<T6>::value");;
    }
    return 0_();
}
pub trait CountAssignImpl {
    fn copy_assign(&self, _a0: Ptr<CountAssign>) -> Ptr<CountAssign>;
    fn move_assign(&self, _a0: Ptr<CountAssign>) -> Ptr<CountAssign>;
}
impl CountAssignImpl for Ptr<CountAssign> {
    fn copy_assign(&self, _a0: Ptr<CountAssign>) -> Ptr<CountAssign> {
        (*copied_14.with(Value::clone).borrow_mut()).prefix_inc();
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<CountAssign>) -> Ptr<CountAssign> {
        (*moved_15.with(Value::clone).borrow_mut()).prefix_inc();
        return (*self).clone();
    }
}
pub trait MoveAssignableImpl {
    fn move_assign(&self, _a0: Ptr<MoveAssignable>) -> Ptr<MoveAssignable>;
}
impl MoveAssignableImpl for Ptr<MoveAssignable> {
    fn move_assign(&self, _a0: Ptr<MoveAssignable>) -> Ptr<MoveAssignable> {
        return (*self).clone();
    }
}
pub trait MoveOnlyImpl {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly>;
    fn get(&self) -> ();
    fn operator_add(&self, x: Ptr<MoveOnly>) -> MoveOnly;
    fn operator_mul(&self, x: Ptr<MoveOnly>) -> MoveOnly;
}
impl MoveOnlyImpl for Ptr<MoveOnly> {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        field!((*self), data_).write({ x.with(|__s| __s.data_) });
        field!(x, data_).write(0_());
        return (*self).clone();
    }
    fn get(&self) -> () {
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
pub trait NothrowMoveAssignableImpl {
    fn move_assign(&self, _a0: Ptr<NothrowMoveAssignable>) -> Ptr<NothrowMoveAssignable>;
}
impl NothrowMoveAssignableImpl for Ptr<NothrowMoveAssignable> {
    fn move_assign(&self, _a0: Ptr<NothrowMoveAssignable>) -> Ptr<NothrowMoveAssignable> {
        return (*self).clone();
    }
}
pub trait PotentiallyThrowingMoveAssignableImpl {
    fn move_assign(
        &self,
        _a0: Ptr<PotentiallyThrowingMoveAssignable>,
    ) -> Ptr<PotentiallyThrowingMoveAssignable>;
}
impl PotentiallyThrowingMoveAssignableImpl for Ptr<PotentiallyThrowingMoveAssignable> {
    fn move_assign(
        &self,
        _a0: Ptr<PotentiallyThrowingMoveAssignable>,
    ) -> Ptr<PotentiallyThrowingMoveAssignable> {
        return (*self).clone();
    }
}
pub trait TrivialMoveOnlyImpl {
    fn move_assign(&self, _a0: Ptr<TrivialMoveOnly>) -> Ptr<TrivialMoveOnly>;
    fn get(&self) -> ();
    fn operator_add(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
    fn operator_mul(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
}
impl TrivialMoveOnlyImpl for Ptr<TrivialMoveOnly> {
    fn move_assign(&self, _a0: Ptr<TrivialMoveOnly>) -> Ptr<TrivialMoveOnly> {
        field!((*self), data_).write({ { (*_a0.upgrade().deref()).data_ } });
        return (*self).clone();
    }
    fn get(&self) -> () {
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
pub fn __cpp2rust_init_globals() {
    let _ = copied_14.with(|_| ());
    let _ = moved_15.with(|_| ());
}
