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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!(
            (alloc_count_15.with(|rc| *rc.borrow()) >= dealloc_count_16.with(|rc| *rc.borrow()))
        );
        return ((alloc_count_15.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_16.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_17.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_14.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_15.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_16.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_14: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_15: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_16: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_17: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_MoveOnly_ {}
impl CppMove for min_allocator_MoveOnly_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_18() -> bool {
    {
        let m: Value<std_set_MoveOnly__std_less_MoveOnly___std_allocator_MoveOnly__> =
            Rc::new(RefCell::new(
                <std_set_MoveOnly__std_less_MoveOnly___std_allocator_MoveOnly__>::default(),
            ));
        let r: Value<BidirectionalIterator<MoveOnly>> = Rc::new(RefCell::new(
            ({
                let ___position: BidirectionalIterator<MoveOnly> = ({ (*m.borrow()).cend() });
                let ___x: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(2) })));
                (*m.borrow()).insert_39(___position, ___x.as_pointer())
            }),
        ));
        assert!((*r.borrow()) == ({ (*m.borrow()).begin() }));
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(
            ({
                let _x: Ptr<MoveOnly> = (*r.borrow()).iter_deref();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(2) })));
                operator_eq_0(_x, _y.as_pointer())
            })
        );
        (*r.borrow_mut()) = ({
            let ___position: BidirectionalIterator<MoveOnly> = ({ (*m.borrow()).cend() });
            let ___x: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(1) })));
            (*m.borrow()).insert_39(___position, ___x.as_pointer())
        });
        assert!((*r.borrow()) == ({ (*m.borrow()).begin() }));
        assert!(({ (({ (*m.borrow()).size() }) == 2_u64) }));
        assert!(
            ({
                let _x: Ptr<MoveOnly> = (*r.borrow()).iter_deref();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(1) })));
                operator_eq_0(_x, _y.as_pointer())
            })
        );
        (*r.borrow_mut()) = ({
            let ___position: BidirectionalIterator<MoveOnly> = ({ (*m.borrow()).cend() });
            let ___x: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(3) })));
            (*m.borrow()).insert_39(___position, ___x.as_pointer())
        });
        assert!((*r.borrow()) == libcc2rs::prev_refcount(({ ({ (*m.borrow()).end() }) }), 1_isize));
        assert!(({ (({ (*m.borrow()).size() }) == 3_u64) }));
        assert!(
            ({
                let _x: Ptr<MoveOnly> = (*r.borrow()).iter_deref();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(3) })));
                operator_eq_0(_x, _y.as_pointer())
            })
        );
        (*r.borrow_mut()) = ({
            let ___position: BidirectionalIterator<MoveOnly> = ({ (*m.borrow()).cend() });
            let ___x: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(3) })));
            (*m.borrow()).insert_39(___position, ___x.as_pointer())
        });
        assert!((*r.borrow()) == libcc2rs::prev_refcount(({ ({ (*m.borrow()).end() }) }), 1_isize));
        assert!(({ (({ (*m.borrow()).size() }) == 3_u64) }));
        assert!(
            ({
                let _x: Ptr<MoveOnly> = (*r.borrow()).iter_deref();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(3) })));
                operator_eq_0(_x, _y.as_pointer())
            })
        );
    }
    {
        let m: Value<std_set_MoveOnly__std_less_MoveOnly___min_allocator_MoveOnly__> =
            Rc::new(RefCell::new(
                <std_set_MoveOnly__std_less_MoveOnly___min_allocator_MoveOnly__>::default(),
            ));
        let r: Value<BidirectionalIterator<MoveOnly>> = Rc::new(RefCell::new(
            ({
                let ___position: BidirectionalIterator<MoveOnly> = ({ (*m.borrow()).cend() });
                let ___x: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(2) })));
                (*m.borrow()).insert_39(___position, ___x.as_pointer())
            }),
        ));
        assert!((*r.borrow()) == ({ (*m.borrow()).begin() }));
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(
            ({
                let _x: Ptr<MoveOnly> = (*r.borrow()).iter_deref();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(2) })));
                operator_eq_0(_x, _y.as_pointer())
            })
        );
        (*r.borrow_mut()) = ({
            let ___position: BidirectionalIterator<MoveOnly> = ({ (*m.borrow()).cend() });
            let ___x: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(1) })));
            (*m.borrow()).insert_39(___position, ___x.as_pointer())
        })
        .clone();
        assert!((*r.borrow()) == ({ (*m.borrow()).begin() }));
        assert!(({ (({ (*m.borrow()).size() }) == 2_u64) }));
        assert!(
            ({
                let _x: Ptr<MoveOnly> = (*r.borrow()).iter_deref();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(1) })));
                operator_eq_0(_x, _y.as_pointer())
            })
        );
        (*r.borrow_mut()) = ({
            let ___position: BidirectionalIterator<MoveOnly> = ({ (*m.borrow()).cend() });
            let ___x: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(3) })));
            (*m.borrow()).insert_39(___position, ___x.as_pointer())
        })
        .clone();
        assert!((*r.borrow()) == libcc2rs::prev_refcount(({ ({ (*m.borrow()).end() }) }), 1_isize));
        assert!(({ (({ (*m.borrow()).size() }) == 3_u64) }));
        assert!(
            ({
                let _x: Ptr<MoveOnly> = (*r.borrow()).iter_deref();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(3) })));
                operator_eq_0(_x, _y.as_pointer())
            })
        );
        (*r.borrow_mut()) = ({
            let ___position: BidirectionalIterator<MoveOnly> = ({ (*m.borrow()).cend() });
            let ___x: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(3) })));
            (*m.borrow()).insert_39(___position, ___x.as_pointer())
        })
        .clone();
        assert!((*r.borrow()) == libcc2rs::prev_refcount(({ ({ (*m.borrow()).end() }) }), 1_isize));
        assert!(({ (({ (*m.borrow()).size() }) == 3_u64) }));
        assert!(
            ({
                let _x: Ptr<MoveOnly> = (*r.borrow()).iter_deref();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(3) })));
                operator_eq_0(_x, _y.as_pointer())
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
    ({ test_18() });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__;
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
pub trait min_allocator_MoveOnly_Impl {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_MoveOnly_Impl for Ptr<min_allocator_MoveOnly_> {}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_14.with(|_| ());
    let _ = alloc_count_15.with(|_| ());
    let _ = dealloc_count_16.with(|_| ());
    let _ = disable_default_constructor_17.with(|_| ());
}
