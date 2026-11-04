extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn DoNotOptimize_0(value: Ptr<AnyPtr>) -> Ptr<AnyPtr> {
    b"";
    return (value).clone();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(40)]
pub struct test_allocator_statistics {
    #[offset(0)]
    pub time_to_throw: i32,
    #[offset(4)]
    pub throw_after: i32,
    #[offset(8)]
    pub count: i32,
    #[offset(12)]
    pub alloc_count: i32,
    #[offset(16)]
    pub allocated_size: i32,
    #[offset(20)]
    pub construct_count: i32,
    #[offset(24)]
    pub destroy_count: i32,
    #[offset(28)]
    pub copied: i32,
    #[offset(32)]
    pub moved: i32,
    #[offset(36)]
    pub converted: i32,
}
impl CppMove for test_allocator_statistics {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        {
            test_allocator_statistics {
                time_to_throw: 0,
                throw_after: 2147483647,
                count: 0,
                alloc_count: 0,
                allocated_size: 0,
                construct_count: 0,
                destroy_count: 0,
                copied: 0,
                moved: 0,
                converted: 0,
            }
        }
    }
}
thread_local!(
    pub static destructed_value_1: Value<i32> = Rc::new(RefCell::new(-1_i32));
);
thread_local!(
    pub static moved_value_2: Value<i32> = Rc::new(RefCell::new(2147483647));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
impl CppMove for test_alloc_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_3(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_4(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_3(_x, _y)
    });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(16)]
pub struct test_allocator_void_ {
    #[offset(0)]
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_void_ {
    pub fn new_1(mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: 0,
            id_: 0,
            stats_: (stats).clone(),
        }
    }
    pub fn new_2(mut data: i32) -> Self {
        Self {
            data_: data,
            id_: 0,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_3(mut data: i32, mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: data,
            id_: 0,
            stats_: (stats).clone(),
        }
    }
    pub fn new_4(mut data: i32, mut id: i32) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_5(mut data: i32, mut id: i32, mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: (stats).clone(),
        }
    }
    pub fn copy_from(a: Ptr<test_allocator_void_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }
    }
}
impl CppEq for test_allocator_void_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_3(__lhs.clone(), __rhs.clone())
    }
}
impl Clone for test_allocator_void_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_void_> = Rc::new(RefCell::new(test_allocator_void_ {
            data_: self.data_.clone(),
            id_: self.id_.clone(),
            stats_: self.stats_.clone(),
        }));
        test_allocator_void_::copy_from(__src.as_pointer())
    }
}
impl CppMove for test_allocator_void_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for test_allocator_void_ {
    fn default() -> Self {
        {
            test_allocator_void_ {
                data_: 0,
                id_: 0,
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
pub fn operator_eq_5(
    x: Ptr<test_allocator_NotConstructible_>,
    y: Ptr<test_allocator_NotConstructible_>,
) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_NotConstructible__rebind_NotConstructible_ {}
impl CppMove for test_allocator_NotConstructible__rebind_NotConstructible_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_NotConstructible__rebind_NotConstructible_ptr_ {}
impl CppMove for test_allocator_NotConstructible__rebind_NotConstructible_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_NotConstructible_ {
    #[offset(0)]
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_NotConstructible_ {
    pub fn new_1(mut data: i32) -> Self {
        Self {
            data_: data,
            id_: 0,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn copy_from(a: Ptr<test_allocator_NotConstructible_>) -> Self {
        let __this: Value<test_allocator_NotConstructible_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_NotConstructible_> = __this.as_pointer();
        assert!(
            (({ a.with(|__s| __s.data_) } != { destructed_value_1.with(|rc| *rc.borrow()) })
                && ({ a.with(|__s| __s.id_) } != { destructed_value_1.with(|rc| *rc.borrow()) }))
                && (!(Ptr::<i8>::from_string_literal(b"copying from destroyed allocator"))
                    .is_null())
        );
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_inc();
            });
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| {
                __v.prefix_inc();
            });
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl CppEq for test_allocator_NotConstructible_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_5(__lhs.clone(), __rhs.clone())
    }
}
impl Clone for test_allocator_NotConstructible_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_NotConstructible_> =
            Rc::new(RefCell::new(test_allocator_NotConstructible_ {
                data_: self.data_.clone(),
                id_: self.id_.clone(),
                stats_: self.stats_.clone(),
            }));
        test_allocator_NotConstructible_::copy_from(__src.as_pointer())
    }
}
impl CppMove for test_allocator_NotConstructible_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for test_allocator_NotConstructible_ {
    fn default() -> Self {
        {
            test_allocator_NotConstructible_ {
                data_: 0_i32,
                id_: 0,
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
impl test_allocator_NotConstructible_ptr_ {
    pub fn new_1(a: Ptr<test_allocator_NotConstructible_>) -> Self {
        let __this: Value<test_allocator_NotConstructible_ptr_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_NotConstructible_ptr_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_inc();
            });
            field!(this.with(|__s| __s.stats_.clone()), converted).with_mut(|__v| {
                __v.prefix_inc();
            });
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Record, ByteRepr, FnPtrArg, CppFrom, Destructor)]
#[cpp_from(test_allocator_NotConstructible_, new_1, ptr)]
#[byte_size(16)]
pub struct test_allocator_NotConstructible_ptr_ {
    #[offset(0)]
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_NotConstructible_ptr_ {}
impl Default for test_allocator_NotConstructible_ptr_ {
    fn default() -> Self {
        {
            test_allocator_NotConstructible_ptr_ {
                data_: 0_i32,
                id_: 0_i32,
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Ctor_Tag {}
impl CppMove for Ctor_Tag {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, FnPtrArg, CppFrom, CopyAssign, MoveAssign, Default)]
#[cpp_from(Ctor_Tag, new_1, value)]
#[byte_size(1)]
pub struct Tag_X {}
impl Tag_X {
    pub fn new_1(mut _a0: Ctor_Tag) -> Self {
        Self {}
    }
    pub fn new_2(mut _a0: Ctor_Tag, _a1: Ptr<Tag_X>) -> Self {
        Self {}
    }
    pub fn new_3(mut _a0: Ctor_Tag, _a1: Ptr<Tag_X>) -> Self {
        Self {}
    }
}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NotConstructible {}
impl NotConstructible {}
impl CppEq for NotConstructible {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_6(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_eq_6(_a0: Ptr<NotConstructible>, _a1: Ptr<NotConstructible>) -> bool {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_8.with(|rc| *rc.borrow()) >= dealloc_count_9.with(|rc| *rc.borrow())));
        return ((alloc_count_8.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_9.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_10.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_7.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_8.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_9.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_7: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_8: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_9: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_10: Value<bool> = Rc::new(RefCell::new(false));
);
pub fn operator_eq_11(mut _a0: min_allocator_int_, mut _a1: min_allocator_int_) -> bool {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
impl CppEq for min_allocator_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_11(__lhs.read(), __rhs.read())
    }
}
impl CppMove for min_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_12(
    mut _a0: min_allocator_NotConstructible_,
    mut _a1: min_allocator_NotConstructible_,
) -> bool {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_NotConstructible_ {}
impl CppEq for min_allocator_NotConstructible_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_12(__lhs.read(), __rhs.read())
    }
}
impl CppMove for min_allocator_NotConstructible_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
    pub fn new_1(mut _a0: min_allocator_int_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(min_allocator_int_, new_1, value)]
#[byte_size(1)]
pub struct min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
impl CppMove for min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl min_allocator_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0___ {
    pub fn new_1(mut _a0: min_allocator_NotConstructible_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(min_allocator_NotConstructible_, new_1, value)]
#[byte_size(1)]
pub struct min_allocator_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0___ {
}
impl CppMove
    for min_allocator_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0___
{
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0___ {}
pub fn operator_eq_13(mut _a0: explicit_allocator_int_, mut _a1: explicit_allocator_int_) -> bool {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct explicit_allocator_int_ {}
impl explicit_allocator_int_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl CppEq for explicit_allocator_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_13(__lhs.read(), __rhs.read())
    }
}
impl CppMove for explicit_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for explicit_allocator_int_ {
    fn default() -> Self {
        { explicit_allocator_int_::new() }
    }
}
pub fn operator_eq_14(
    mut _a0: explicit_allocator_NotConstructible_,
    mut _a1: explicit_allocator_NotConstructible_,
) -> bool {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct explicit_allocator_NotConstructible_ {}
impl explicit_allocator_NotConstructible_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl CppEq for explicit_allocator_NotConstructible_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_14(__lhs.read(), __rhs.read())
    }
}
impl CppMove for explicit_allocator_NotConstructible_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for explicit_allocator_NotConstructible_ {
    fn default() -> Self {
        { explicit_allocator_NotConstructible_::new() }
    }
}
impl explicit_allocator_int_ptr_ {
    pub fn new_1(mut _a0: explicit_allocator_int_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct explicit_allocator_int_ptr_ {}
impl explicit_allocator_int_ptr_ {}
impl CppMove for explicit_allocator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl explicit_allocator_NotConstructible_ptr_ {
    pub fn new_1(mut _a0: explicit_allocator_NotConstructible_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct explicit_allocator_NotConstructible_ptr_ {}
impl explicit_allocator_NotConstructible_ptr_ {}
impl CppMove for explicit_allocator_NotConstructible_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_15(mut _a0: safe_allocator_int_, mut _a1: safe_allocator_int_) -> bool {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct safe_allocator_int_ {}
impl safe_allocator_int_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl CppEq for safe_allocator_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_15(__lhs.read(), __rhs.read())
    }
}
impl CppMove for safe_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for safe_allocator_int_ {
    fn default() -> Self {
        { safe_allocator_int_::new() }
    }
}
pub fn operator_eq_16(
    mut _a0: safe_allocator_NotConstructible_,
    mut _a1: safe_allocator_NotConstructible_,
) -> bool {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct safe_allocator_NotConstructible_ {}
impl safe_allocator_NotConstructible_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl CppEq for safe_allocator_NotConstructible_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_16(__lhs.read(), __rhs.read())
    }
}
impl CppMove for safe_allocator_NotConstructible_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for safe_allocator_NotConstructible_ {
    fn default() -> Self {
        { safe_allocator_NotConstructible_::new() }
    }
}
impl safe_allocator_int_ptr_ {
    pub fn new_1(mut _a0: safe_allocator_int_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct safe_allocator_int_ptr_ {}
impl safe_allocator_int_ptr_ {}
impl CppMove for safe_allocator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl safe_allocator_NotConstructible_ptr_ {
    pub fn new_1(mut _a0: safe_allocator_NotConstructible_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct safe_allocator_NotConstructible_ptr_ {}
impl safe_allocator_NotConstructible_ptr_ {}
impl CppMove for safe_allocator_NotConstructible_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_17(a: Ptr<test_allocator_NotConstructible_>) {
    let d: Value<Vec<NotConstructible>> = Rc::new(RefCell::new(
        std_deque_NotConstructible__test_allocator_NotConstructible__::new_1({ (a).clone() }),
    ));
    assert!(({ (({ (*d.borrow()).size() }) == 0_usize) }));
    assert!(
        ({
            let _x: Value<test_allocator_NotConstructible_> =
                Rc::new(RefCell::new(({ (*d.borrow()).get_allocator() })));
            let _y: Ptr<test_allocator_NotConstructible_> = (a).clone();
            operator_eq_5(_x.as_pointer(), _y)
        })
    );
    const _: () = assert!(true, "");;
}
pub fn test_18(a: Ptr<min_allocator_int_>) {
    let d: Value<Vec<i32>> = Rc::new(RefCell::new(std_deque_int__min_allocator_int__::new_1({
        (a).clone()
    })));
    assert!(({ (({ (*d.borrow()).size() }) == 0_usize) }));
    assert!(
        ({
            ({
                let _arg0: min_allocator_int_ = ({ (*d.borrow()).get_allocator() });
                let _arg1: min_allocator_int_ = (*a.upgrade().deref()).clone();
                operator_eq_11(_arg0, _arg1)
            })
        })
    );
    const _: () = assert!(true, "");;
}
pub fn test_19(a: Ptr<safe_allocator_int_>) {
    let d: Value<Vec<i32>> = Rc::new(RefCell::new(std_deque_int__safe_allocator_int__::new_1({
        (a).clone()
    })));
    assert!(({ (({ (*d.borrow()).size() }) == 0_usize) }));
    assert!(
        ({
            ({
                let _arg0: safe_allocator_int_ = ({ (*d.borrow()).get_allocator() });
                let _arg1: safe_allocator_int_ = (*a.upgrade().deref()).clone();
                operator_eq_15(_arg0, _arg1)
            })
        })
    );
    const _: () = assert!(true, "");;
}
pub fn test_20(a: Ptr<min_allocator_NotConstructible_>) {
    let d: Value<Vec<NotConstructible>> = Rc::new(RefCell::new(
        std_deque_NotConstructible__min_allocator_NotConstructible__::new_1({ (a).clone() }),
    ));
    assert!(({ (({ (*d.borrow()).size() }) == 0_usize) }));
    assert!(
        ({
            ({
                let _arg0: min_allocator_NotConstructible_ = ({ (*d.borrow()).get_allocator() });
                let _arg1: min_allocator_NotConstructible_ = (*a.upgrade().deref()).clone();
                operator_eq_12(_arg0, _arg1)
            })
        })
    );
    const _: () = assert!(true, "");;
}
pub fn test_21(a: Ptr<safe_allocator_NotConstructible_>) {
    let d: Value<Vec<NotConstructible>> = Rc::new(RefCell::new(
        std_deque_NotConstructible__safe_allocator_NotConstructible__::new_1({ (a).clone() }),
    ));
    assert!(({ (({ (*d.borrow()).size() }) == 0_usize) }));
    assert!(
        ({
            ({
                let _arg0: safe_allocator_NotConstructible_ = ({ (*d.borrow()).get_allocator() });
                let _arg1: safe_allocator_NotConstructible_ = (*a.upgrade().deref()).clone();
                operator_eq_16(_arg0, _arg1)
            })
        })
    );
    const _: () = assert!(true, "");;
}
pub fn test_22(a: Ptr<explicit_allocator_int_>) {
    let d: Value<Vec<i32>> = Rc::new(RefCell::new(
        std_deque_int__explicit_allocator_int__::new_1({ (a).clone() }),
    ));
    assert!(({ (({ (*d.borrow()).size() }) == 0_usize) }));
    assert!(
        ({
            ({
                let _arg0: explicit_allocator_int_ = ({ (*d.borrow()).get_allocator() });
                let _arg1: explicit_allocator_int_ = (*a.upgrade().deref()).clone();
                operator_eq_13(_arg0, _arg1)
            })
        })
    );
    const _: () = assert!(true, "");;
}
pub fn test_23(a: Ptr<explicit_allocator_NotConstructible_>) {
    let d: Value<Vec<NotConstructible>> = Rc::new(RefCell::new(
        std_deque_NotConstructible__explicit_allocator_NotConstructible__::new_1({ (a).clone() }),
    ));
    assert!(({ (({ (*d.borrow()).size() }) == 0_usize) }));
    assert!(
        ({
            ({
                let _arg0: explicit_allocator_NotConstructible_ =
                    ({ (*d.borrow()).get_allocator() });
                let _arg1: explicit_allocator_NotConstructible_ = (*a.upgrade().deref()).clone();
                operator_eq_14(_arg0, _arg1)
            })
        })
    );
    const _: () = assert!(true, "");;
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
    ({
        let _a: Value<std_allocator_int_> = Rc::new(RefCell::new(<std_allocator_int_>::default()));
        test_24(_a.as_pointer())
    });
    ({
        let _a: Value<test_allocator_NotConstructible_> =
            Rc::new(RefCell::new(test_allocator_NotConstructible_::new_1({ 3 })));
        test_17(_a.as_pointer())
    });
    ({
        let _a: Value<min_allocator_int_> = Rc::new(RefCell::new(<min_allocator_int_>::default()));
        test_18(_a.as_pointer())
    });
    ({
        let _a: Value<safe_allocator_int_> = Rc::new(RefCell::new(safe_allocator_int_::new()));
        test_19(_a.as_pointer())
    });
    ({
        let _a: Value<min_allocator_NotConstructible_> =
            Rc::new(RefCell::new(<min_allocator_NotConstructible_>::default()));
        test_20(_a.as_pointer())
    });
    ({
        let _a: Value<safe_allocator_NotConstructible_> =
            Rc::new(RefCell::new(safe_allocator_NotConstructible_::new()));
        test_21(_a.as_pointer())
    });
    ({
        let _a: Value<explicit_allocator_int_> =
            Rc::new(RefCell::new(explicit_allocator_int_::new()));
        test_22(_a.as_pointer())
    });
    ({
        let _a: Value<explicit_allocator_NotConstructible_> =
            Rc::new(RefCell::new(explicit_allocator_NotConstructible_::new()));
        test_23(_a.as_pointer())
    });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
pub trait NotConstructibleImpl {
    fn copy_assign(&self, _a0: Ptr<NotConstructible>) -> Ptr<NotConstructible> {
        unimplemented!()
    }
}
impl NotConstructibleImpl for Ptr<NotConstructible> {}
pub trait Tag_XImpl {
    fn copy_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X>;
    fn move_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X>;
}
impl Tag_XImpl for Ptr<Tag_X> {
    fn copy_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X> {
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X> {
        return (*self).clone();
    }
}
pub trait explicit_allocator_NotConstructible_Impl {
    fn allocate(&self, n: usize) -> Ptr<NotConstructible>;
    fn deallocate(&self, p: Ptr<NotConstructible>, n: usize);
}
impl explicit_allocator_NotConstructible_Impl for Ptr<explicit_allocator_NotConstructible_> {
    fn allocate(&self, mut n: usize) -> Ptr<NotConstructible> {
        return ({ <std_allocator_NotConstructible_>::default().allocate(n) })
            .reinterpret_cast::<NotConstructible>();
    }
    fn deallocate(&self, mut p: Ptr<NotConstructible>, mut n: usize) {
        ({ <std_allocator_NotConstructible_>::default().deallocate((p).clone(), n) });
    }
}
pub trait explicit_allocator_NotConstructible_ptr_Impl {
    fn allocate(&self, n: usize) -> Ptr<Ptr<NotConstructible>>;
    fn deallocate(&self, p: Ptr<Ptr<NotConstructible>>, n: usize);
}
impl explicit_allocator_NotConstructible_ptr_Impl
    for Ptr<explicit_allocator_NotConstructible_ptr_>
{
    fn allocate(&self, mut n: usize) -> Ptr<Ptr<NotConstructible>> {
        return ({ <std_allocator_NotConstructible_ptr_>::default().allocate(n) })
            .reinterpret_cast::<Ptr<NotConstructible>>();
    }
    fn deallocate(&self, mut p: Ptr<Ptr<NotConstructible>>, mut n: usize) {
        ({ <std_allocator_NotConstructible_ptr_>::default().deallocate((p).clone(), n) });
    }
}
pub trait explicit_allocator_int_Impl {
    fn allocate(&self, n: usize) -> Ptr<i32>;
    fn deallocate(&self, p: Ptr<i32>, n: usize);
}
impl explicit_allocator_int_Impl for Ptr<explicit_allocator_int_> {
    fn allocate(&self, mut n: usize) -> Ptr<i32> {
        return ({ <std_allocator_int_>::default().allocate(n) }).reinterpret_cast::<i32>();
    }
    fn deallocate(&self, mut p: Ptr<i32>, mut n: usize) {
        ({ <std_allocator_int_>::default().deallocate((p).clone(), n) });
    }
}
pub trait explicit_allocator_int_ptr_Impl {
    fn allocate(&self, n: usize) -> Ptr<Ptr<i32>>;
    fn deallocate(&self, p: Ptr<Ptr<i32>>, n: usize);
}
impl explicit_allocator_int_ptr_Impl for Ptr<explicit_allocator_int_ptr_> {
    fn allocate(&self, mut n: usize) -> Ptr<Ptr<i32>> {
        return ({ <std_allocator_int_ptr_>::default().allocate(n) })
            .reinterpret_cast::<Ptr<i32>>();
    }
    fn deallocate(&self, mut p: Ptr<Ptr<i32>>, mut n: usize) {
        ({ <std_allocator_int_ptr_>::default().deallocate((p).clone(), n) });
    }
}
pub trait min_allocator_NotConstructible_Impl {
    fn allocate(
        &self,
        n: usize,
    ) -> min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__;
    fn deallocate(
        &self,
        p: min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
        n: usize,
    );
}
impl min_allocator_NotConstructible_Impl for Ptr<min_allocator_NotConstructible_> {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ {
        return min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__::new_1({
            ({ <std_allocator_NotConstructible_>::default().allocate(n) })
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        ({ <std_allocator_NotConstructible_>::default().deallocate((p.ptr_).clone(), n) });
    }
}
pub trait min_allocator_int_Impl {
    fn allocate(&self, n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_int__std_integral_constant_unsigned_long__0__, n: usize);
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {
    fn allocate(&self, mut n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_1({
            ({ <std_allocator_int_>::default().allocate(n) })
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        ({ <std_allocator_int_>::default().deallocate((p.ptr_).clone(), n) });
    }
}
pub trait min_allocator_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0___Impl
{
    fn allocate ( &self , n : usize  , ) -> min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn deallocate(
        &self,
        p : min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        n: usize,
    );
}
impl min_allocator_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0___Impl
    for Ptr<min_allocator_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0___>
{
    fn allocate ( &self , mut n : usize  , ) -> min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { ( { <std_allocator_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0___  >::default() . allocate ( n  , ) } )   } , )    ;
    }
    fn deallocate(
        &self,
        mut p : min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        ({
            <std_allocator_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0___  >::default() . deallocate ( (p . ptr_  ).clone() , n  , )
        });
    }
}
pub trait min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___Impl {
    fn allocate ( &self , n : usize  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn deallocate(
        &self,
        p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        n: usize,
    );
}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___Impl
    for Ptr<min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___>
{
    fn allocate ( &self , mut n : usize  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { ( { <std_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___  >::default() . allocate ( n  , ) } )   } , )    ;
    }
    fn deallocate(
        &self,
        mut p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        ({
            <std_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___>::default()
                .deallocate((p.ptr_).clone(), n)
        });
    }
}
pub trait safe_allocator_NotConstructible_Impl {
    fn allocate(&self, n: usize) -> Ptr<NotConstructible>;
    fn deallocate(&self, p: Ptr<NotConstructible>, n: usize);
}
impl safe_allocator_NotConstructible_Impl for Ptr<safe_allocator_NotConstructible_> {
    fn allocate(&self, mut n: usize) -> Ptr<NotConstructible> {
        let memory: Value<Ptr<NotConstructible>> = Rc::new(RefCell::new(
            ({ <std_allocator_NotConstructible_>::default().allocate(n) }),
        ));
        {
            match ({ (*memory.borrow()).to_any() },) {
                (mut __tmp_2,) => {
                    __tmp_2.memset(
                        (0) as u8,
                        ((1usize as u64).wrapping_mul((n as u64)) as usize) as usize,
                    );
                    (__tmp_2).clone()
                }
            };
        }
        return (*memory.borrow()).clone();
    }
    fn deallocate(&self, mut p: Ptr<NotConstructible>, mut n: usize) {
        {
            ({
                let _value: Value<AnyPtr> = Rc::new(RefCell::new(match ({ (p).to_any() },) {
                    (mut __tmp_3,) => {
                        __tmp_3.memset(
                            (0) as u8,
                            ((1usize as u64).wrapping_mul((n as u64)) as usize) as usize,
                        );
                        (__tmp_3).clone()
                    }
                }));
                DoNotOptimize_0(_value.as_pointer())
            });
        }
        ({ <std_allocator_NotConstructible_>::default().deallocate((p).clone(), n) });
    }
}
pub trait safe_allocator_NotConstructible_ptr_Impl {
    fn allocate(&self, n: usize) -> Ptr<Ptr<NotConstructible>>;
    fn deallocate(&self, p: Ptr<Ptr<NotConstructible>>, n: usize);
}
impl safe_allocator_NotConstructible_ptr_Impl for Ptr<safe_allocator_NotConstructible_ptr_> {
    fn allocate(&self, mut n: usize) -> Ptr<Ptr<NotConstructible>> {
        let memory: Value<Ptr<Ptr<NotConstructible>>> = Rc::new(RefCell::new(
            ({ <std_allocator_NotConstructible_ptr_>::default().allocate(n) }),
        ));
        {
            match ({ (*memory.borrow()).to_any() },) {
                (mut __tmp_6,) => {
                    __tmp_6.memset(
                        (0) as u8,
                        ((8usize as u64).wrapping_mul((n as u64)) as usize) as usize,
                    );
                    (__tmp_6).clone()
                }
            };
        }
        return (*memory.borrow()).clone();
    }
    fn deallocate(&self, mut p: Ptr<Ptr<NotConstructible>>, mut n: usize) {
        {
            ({
                let _value: Value<AnyPtr> = Rc::new(RefCell::new(match ({ (p).to_any() },) {
                    (mut __tmp_7,) => {
                        __tmp_7.memset(
                            (0) as u8,
                            ((8usize as u64).wrapping_mul((n as u64)) as usize) as usize,
                        );
                        (__tmp_7).clone()
                    }
                }));
                DoNotOptimize_0(_value.as_pointer())
            });
        }
        ({ <std_allocator_NotConstructible_ptr_>::default().deallocate((p).clone(), n) });
    }
}
pub trait safe_allocator_int_Impl {
    fn allocate(&self, n: usize) -> Ptr<i32>;
    fn deallocate(&self, p: Ptr<i32>, n: usize);
}
impl safe_allocator_int_Impl for Ptr<safe_allocator_int_> {
    fn allocate(&self, mut n: usize) -> Ptr<i32> {
        let memory: Value<Ptr<i32>> = Rc::new(RefCell::new(
            ({ <std_allocator_int_>::default().allocate(n) }),
        ));
        {
            match ({ (*memory.borrow()).to_any() },) {
                (mut __tmp_0,) => {
                    __tmp_0.memset(
                        (0) as u8,
                        ((::std::mem::size_of::<i32>() as u64).wrapping_mul((n as u64)) as usize)
                            as usize,
                    );
                    (__tmp_0).clone()
                }
            };
        }
        return (*memory.borrow()).clone();
    }
    fn deallocate(&self, mut p: Ptr<i32>, mut n: usize) {
        {
            ({
                let _value: Value<AnyPtr> = Rc::new(RefCell::new(match ({ (p).to_any() },) {
                    (mut __tmp_1,) => {
                        __tmp_1.memset(
                            (0) as u8,
                            ((::std::mem::size_of::<i32>() as u64).wrapping_mul((n as u64))
                                as usize) as usize,
                        );
                        (__tmp_1).clone()
                    }
                }));
                DoNotOptimize_0(_value.as_pointer())
            });
        }
        ({ <std_allocator_int_>::default().deallocate((p).clone(), n) });
    }
}
pub trait safe_allocator_int_ptr_Impl {
    fn allocate(&self, n: usize) -> Ptr<Ptr<i32>>;
    fn deallocate(&self, p: Ptr<Ptr<i32>>, n: usize);
}
impl safe_allocator_int_ptr_Impl for Ptr<safe_allocator_int_ptr_> {
    fn allocate(&self, mut n: usize) -> Ptr<Ptr<i32>> {
        let memory: Value<Ptr<Ptr<i32>>> = Rc::new(RefCell::new(
            ({ <std_allocator_int_ptr_>::default().allocate(n) }),
        ));
        {
            match ({ (*memory.borrow()).to_any() },) {
                (mut __tmp_4,) => {
                    __tmp_4.memset(
                        (0) as u8,
                        ((8usize as u64).wrapping_mul((n as u64)) as usize) as usize,
                    );
                    (__tmp_4).clone()
                }
            };
        }
        return (*memory.borrow()).clone();
    }
    fn deallocate(&self, mut p: Ptr<Ptr<i32>>, mut n: usize) {
        {
            ({
                let _value: Value<AnyPtr> = Rc::new(RefCell::new(match ({ (p).to_any() },) {
                    (mut __tmp_5,) => {
                        __tmp_5.memset(
                            (0) as u8,
                            ((8usize as u64).wrapping_mul((n as u64)) as usize) as usize,
                        );
                        (__tmp_5).clone()
                    }
                }));
                DoNotOptimize_0(_value.as_pointer())
            });
        }
        ({ <std_allocator_int_ptr_>::default().deallocate((p).clone(), n) });
    }
}
pub trait test_allocator_NotConstructible_Impl {
    fn destructor(&self);
    fn address_11(&self, x: Ptr<NotConstructible>) -> Ptr<NotConstructible> {
        unimplemented!()
    }
    fn address_12(&self, x: Ptr<NotConstructible>) -> Ptr<NotConstructible> {
        unimplemented!()
    }
    fn allocate(&self, n: u32, _a1: Option<AnyPtr>) -> Ptr<NotConstructible>;
    fn deallocate(&self, p: Ptr<NotConstructible>, s: u32);
    fn max_size(&self) -> u32;
    fn destroy(&self, p: Ptr<NotConstructible>);
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
    fn get_id(&self) -> i32 {
        unimplemented!()
    }
}
impl test_allocator_NotConstructible_Impl for Ptr<test_allocator_NotConstructible_> {
    fn destructor(&self) {
        assert!(((*self).with(|__s| __s.data_) != destructed_value_1.with(|rc| *rc.borrow())));
        assert!(((*self).with(|__s| __s.id_) != destructed_value_1.with(|rc| *rc.borrow())));
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_dec();
            });
        }
        field!((*self), data_).write(destructed_value_1.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_1.with(|rc| *rc.borrow()));
    }
    fn allocate(&self, mut n: u32, _a1: Option<AnyPtr>) -> Ptr<NotConstructible> {
        let mut _a1: AnyPtr = _a1.unwrap_or_else(|| AnyPtr::default());
        assert!(((*self).with(|__s| __s.data_) != destructed_value_1.with(|rc| *rc.borrow())));
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            if ({
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.time_to_throw)
            } >= {
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.throw_after)
            }) {
                std::process::abort();
            }
            field!((*self).with(|__s| __s.stats_.clone()), time_to_throw).with_mut(|__v| {
                __v.prefix_inc();
            });
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count).with_mut(|__v| {
                __v.prefix_inc();
            });
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                (((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size) as u32)
                    .wrapping_add(n)) as i32
            });
        }
        return ({ <std_allocator_NotConstructible_>::default().allocate((n as usize)) });
    }
    fn deallocate(&self, mut p: Ptr<NotConstructible>, mut s: u32) {
        assert!(((*self).with(|__s| __s.data_) != destructed_value_1.with(|rc| *rc.borrow())));
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count).with_mut(|__v| {
                __v.prefix_dec();
            });
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                (((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size) as u32)
                    .wrapping_sub(s)) as i32
            });
        }
        ({ <std_allocator_NotConstructible_>::default().deallocate((p).clone(), (s as usize)) });
    }
    fn max_size(&self) -> u32 {
        return ((((((2147483647_u32).wrapping_mul((2_u32 as u32)) as u32)
            .wrapping_add((1_u32 as u32))) as usize)
            .wrapping_div((1usize as usize))) as u32);
    }
    fn destroy(&self, mut p: Ptr<NotConstructible>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), destroy_count).with_mut(|__v| {
                __v.prefix_inc();
            });
        };
    }
}
pub trait test_allocator_NotConstructible_ptr_Impl {
    fn destructor(&self);
    fn address_11(&self, x: Ptr<Ptr<NotConstructible>>) -> Ptr<Ptr<NotConstructible>> {
        unimplemented!()
    }
    fn address_12(&self, x: Ptr<Ptr<NotConstructible>>) -> Ptr<Ptr<NotConstructible>> {
        unimplemented!()
    }
    fn allocate(&self, n: u32, _a1: Option<AnyPtr>) -> Ptr<Ptr<NotConstructible>>;
    fn deallocate(&self, p: Ptr<Ptr<NotConstructible>>, s: u32);
    fn max_size(&self) -> u32 {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<Ptr<NotConstructible>>) {
        unimplemented!()
    }
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
    fn get_id(&self) -> i32 {
        unimplemented!()
    }
}
impl test_allocator_NotConstructible_ptr_Impl for Ptr<test_allocator_NotConstructible_ptr_> {
    fn destructor(&self) {
        assert!(((*self).with(|__s| __s.data_) != destructed_value_1.with(|rc| *rc.borrow())));
        assert!(((*self).with(|__s| __s.id_) != destructed_value_1.with(|rc| *rc.borrow())));
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_dec();
            });
        }
        field!((*self), data_).write(destructed_value_1.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_1.with(|rc| *rc.borrow()));
    }
    fn allocate(&self, mut n: u32, _a1: Option<AnyPtr>) -> Ptr<Ptr<NotConstructible>> {
        let mut _a1: AnyPtr = _a1.unwrap_or_else(|| AnyPtr::default());
        assert!(((*self).with(|__s| __s.data_) != destructed_value_1.with(|rc| *rc.borrow())));
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            if ({
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.time_to_throw)
            } >= {
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.throw_after)
            }) {
                std::process::abort();
            }
            field!((*self).with(|__s| __s.stats_.clone()), time_to_throw).with_mut(|__v| {
                __v.prefix_inc();
            });
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count).with_mut(|__v| {
                __v.prefix_inc();
            });
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                (((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size) as u32)
                    .wrapping_add(n)) as i32
            });
        }
        return ({ <std_allocator_NotConstructible_ptr_>::default().allocate((n as usize)) });
    }
    fn deallocate(&self, mut p: Ptr<Ptr<NotConstructible>>, mut s: u32) {
        assert!(((*self).with(|__s| __s.data_) != destructed_value_1.with(|rc| *rc.borrow())));
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count).with_mut(|__v| {
                __v.prefix_dec();
            });
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                (((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size) as u32)
                    .wrapping_sub(s)) as i32
            });
        }
        ({
            <std_allocator_NotConstructible_ptr_>::default().deallocate((p).clone(), (s as usize))
        });
    }
}
pub trait test_allocator_statisticsImpl {
    fn clear(&self);
    fn clear_ctor_counters(&self);
}
impl test_allocator_statisticsImpl for Ptr<test_allocator_statistics> {
    fn clear(&self) {
        assert!(
            ((*self).with(|__s| __s.count) == 0)
                && (!(Ptr::<i8>::from_string_literal(b"clearing leaking allocator data?"))
                    .is_null())
        );
        field!((*self), count).write(0);
        field!((*self), time_to_throw).write(0);
        field!((*self), alloc_count).write(0);
        field!((*self), allocated_size).write(0);
        field!((*self), construct_count).write(0);
        field!((*self), destroy_count).write(0);
        field!((*self), throw_after).write(2147483647);
        ({ test_allocator_statisticsImpl::clear_ctor_counters(self) });
    }
    fn clear_ctor_counters(&self) {
        field!((*self), copied).write(0);
        field!((*self), moved).write(0);
        field!((*self), converted).write(0);
    }
}
pub trait test_allocator_void_Impl {
    fn destructor(&self);
    fn get_id(&self) -> i32;
    fn get_data(&self) -> i32;
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_>;
}
impl test_allocator_void_Impl for Ptr<test_allocator_void_> {
    fn destructor(&self) {
        field!((*self), data_).write(destructed_value_1.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_1.with(|rc| *rc.borrow()));
    }
    fn get_id(&self) -> i32 {
        return (*self).with(|__s| __s.id_);
    }
    fn get_data(&self) -> i32 {
        return (*self).with(|__s| __s.data_);
    }
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_> {
        field!((*self), data_).write({ _a0.with(|__s| __s.data_) });
        field!((*self), id_).write({ _a0.with(|__s| __s.id_) });
        field!((*self), stats_).write({ _a0.with(|__s| __s.stats_.clone()) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = destructed_value_1.with(|_| ());
    let _ = moved_value_2.with(|_| ());
    let _ = outstanding_bytes_7.with(|_| ());
    let _ = alloc_count_8.with(|_| ());
    let _ = dealloc_count_9.with(|_| ());
    let _ = disable_default_constructor_10.with(|_| ());
}
