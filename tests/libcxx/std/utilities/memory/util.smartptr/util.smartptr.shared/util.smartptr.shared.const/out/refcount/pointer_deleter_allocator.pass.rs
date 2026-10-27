extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct bare_allocator_void_ {}
impl bare_allocator_void_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for bare_allocator_void_ {
    fn default() -> Self {
        { bare_allocator_void_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_1: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_2: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_3: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_void_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct complete_type_allocator_int_ {}
impl complete_type_allocator_int_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for complete_type_allocator_int_ {
    fn default() -> Self {
        { complete_type_allocator_int_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_deleter_base {}
thread_local!(
    pub static count_4: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_5: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(4)]
pub struct test_deleter_int_ {
    #[offset(0)]
    state_: (),
}
impl test_deleter_int_ {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(4)]
pub struct test_deleter_base_ {
    #[offset(0)]
    state_: (),
}
impl test_deleter_base_ {}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct test_deleter_A_ {
    #[offset(0)]
    state_: (),
}
impl test_deleter_A_ {
    pub fn new_1(mut s: ()) -> Self {
        let __this: test_deleter_A_ = Self { state_: s };
        (*count_4.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(d: Ptr<test_deleter_A_>) -> Self {
        let __this: test_deleter_A_ = Self {
            state_: d.with(|__s| __s.state_),
        };
        (*count_4.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for test_deleter_A_ {
    fn clone(&self) -> Self {
        let __src: Value<test_deleter_A_> = Rc::new(RefCell::new(test_deleter_A_ {
            state_: self.state_.clone(),
        }));
        test_deleter_A_::copy_from(__src.as_pointer())
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct test_deleter_const_A_ {
    #[offset(0)]
    state_: (),
}
impl test_deleter_const_A_ {
    pub fn new_1(mut s: ()) -> Self {
        let __this: test_deleter_const_A_ = Self { state_: s };
        (*count_4.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(d: Ptr<test_deleter_const_A_>) -> Self {
        let __this: test_deleter_const_A_ = Self {
            state_: d.with(|__s| __s.state_),
        };
        (*count_4.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for test_deleter_const_A_ {
    fn clone(&self) -> Self {
        let __src: Value<test_deleter_const_A_> = Rc::new(RefCell::new(test_deleter_const_A_ {
            state_: self.state_.clone(),
        }));
        test_deleter_const_A_::copy_from(__src.as_pointer())
    }
}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(4)]
pub struct test_deleter_derivedarr4arr_ {
    #[offset(0)]
    state_: (),
}
impl test_deleter_derivedarr4arr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(40)]
pub struct test_allocator_statistics {
    #[offset(0)]
    pub time_to_throw: (),
    #[offset(4)]
    pub throw_after: (),
    #[offset(8)]
    pub count: (),
    #[offset(12)]
    pub alloc_count: (),
    #[offset(16)]
    pub allocated_size: (),
    #[offset(20)]
    pub construct_count: (),
    #[offset(24)]
    pub destroy_count: (),
    #[offset(28)]
    pub copied: (),
    #[offset(32)]
    pub moved: (),
    #[offset(36)]
    pub converted: (),
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        {
            test_allocator_statistics {
                time_to_throw: 0_(),
                throw_after: 2147483647_(),
                count: 0_(),
                alloc_count: 0_(),
                allocated_size: 0_(),
                construct_count: 0_(),
                destroy_count: 0_(),
                copied: 0_(),
                moved: 0_(),
                converted: 0_(),
            }
        }
    }
}
thread_local!(
    pub static destructed_value_6: Value<()> = Rc::new(RefCell::new(-1_()));
);
thread_local!(
    pub static moved_value_7: Value<()> = Rc::new(RefCell::new(2147483647_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_8(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_9(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_8(_x, _y)
    });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(16)]
pub struct test_allocator_void_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_void_ {
    pub fn new_1(mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: 0_(),
            id_: 0_(),
            stats_: (stats).clone(),
        }
    }
    pub fn new_2(mut data: ()) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_3(mut data: (), mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: (stats).clone(),
        }
    }
    pub fn new_4(mut data: (), mut id: ()) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_5(mut data: (), mut id: (), mut stats: Ptr<test_allocator_statistics>) -> Self {
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
impl std::cmp::PartialEq for test_allocator_void_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_8(
                Rc::new(RefCell::new(test_allocator_void_ {
                    data_: self.data_.clone(),
                    id_: self.id_.clone(),
                    stats_: self.stats_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(test_allocator_void_ {
                    data_: other.data_.clone(),
                    id_: other.id_.clone(),
                    stats_: other.stats_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_void_ {}
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
impl Default for test_allocator_void_ {
    fn default() -> Self {
        {
            test_allocator_void_ {
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(16)]
pub struct test_allocator_int_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_int_ {}
impl Default for test_allocator_int_ {
    fn default() -> Self {
        {
            test_allocator_int_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(16)]
pub struct test_allocator_base_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_base_ {}
impl Default for test_allocator_base_ {
    fn default() -> Self {
        {
            test_allocator_base_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Destructor)]
#[byte_size(16)]
pub struct test_allocator_A_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_A_ {
    pub fn new_1(mut data: (), mut stats: Ptr<test_allocator_statistics>) -> Self {
        let __this: Value<test_allocator_A_> = Rc::new(RefCell::new(Self {
            data_: data,
            id_: 0_(),
            stats_: (stats).clone(),
        }));
        let this: Ptr<test_allocator_A_> = __this.as_pointer();
        if !((stats).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(a: Ptr<test_allocator_A_>) -> Self {
        let __this: Value<test_allocator_A_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_A_> = __this.as_pointer();
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(a: Ptr<test_allocator_A_>) -> Self {
        let __this: Value<test_allocator_A_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_A_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), moved).with_mut(|__v| __v.prefix_inc());
        }
        unimplemented!();
        field!(a, id_).write(moved_value_7.with(|rc| *rc.borrow()));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for test_allocator_A_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_A_> = Rc::new(RefCell::new(test_allocator_A_ {
            data_: self.data_.clone(),
            id_: self.id_.clone(),
            stats_: self.stats_.clone(),
        }));
        test_allocator_A_::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_A_ {
    fn default() -> Self {
        {
            test_allocator_A_ {
                data_: unimplemented!(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(16)]
pub struct test_allocator_derivedarr4arr_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_derivedarr4arr_ {}
impl Default for test_allocator_derivedarr4arr_ {
    fn default() -> Self {
        {
            test_allocator_derivedarr4arr_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Ctor_Tag {}
#[derive(Record, ByteRepr, FnPtrArg, CopyAssign, MoveAssign, Default)]
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct bad_type {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct bad_deleter {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct no_move_deleter {}
const _: () = assert!(true, "!std::is_move_constructible<no_move_deleter>::value");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct no_nullptr_deleter {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct base {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct derived {}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(1)]
pub struct move_deleter_int_ {}
impl move_deleter_int_ {
    pub fn move_from(_a0: Ptr<move_deleter_int_>) -> Self {
        Self {}
    }
    pub fn new_1(mut _a0: ()) -> Self {
        Self {}
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(1)]
pub struct A {}
impl A {
    pub fn new() -> Self {
        let __this: A = Self {};
        (*count_10.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(_a0: Ptr<A>) -> Self {
        let __this: A = Self {};
        (*count_10.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for A {
    fn clone(&self) -> Self {
        let __src: Value<A> = Rc::new(RefCell::new(A {}));
        A::copy_from(__src.as_pointer())
    }
}
impl Default for A {
    fn default() -> Self {
        { A::new() }
    }
}
thread_local!(
    pub static count_10: Value<()> = Rc::new(RefCell::new(0_()));
);
const _: () = assert!(
    true,
    "std::is_constructible<std::shared_ptr<int>,  int*, test_deleter<int>, test_allocator<int> >::value"
);
const _: () = assert!(
    true,
    "!std::is_constructible<std::shared_ptr<int>,  int*, bad_deleter, test_allocator<int> >::value"
);
const _: () = assert!(
    true,
    "std::is_constructible<std::shared_ptr<base>,  derived*, test_deleter<base>, test_allocator<base> >::value"
);
const _: () = assert!(
    true,
    "!std::is_constructible<std::shared_ptr<A>,  int*, test_deleter<A>, test_allocator<A> >::value"
);
const _: () = assert!(
    true,
    "std::is_constructible<std::shared_ptr<int[]>,  int*, test_deleter<int>, test_allocator<int> >::value"
);
const _: () = assert!(
    true,
    "!std::is_constructible<std::shared_ptr<int[]>,  int*, bad_deleter, test_allocator<int> >::value"
);
const _: () = assert!(
    true,
    "!std::is_constructible<std::shared_ptr<int[]>,  int(*)[], test_deleter<int>, test_allocator<int> >::value"
);
const _: () = assert!(
    true,
    "std::is_constructible<std::shared_ptr<int[5]>, int*, test_deleter<int>, test_allocator<int> >::value"
);
const _: () = assert!(
    true,
    "!std::is_constructible<std::shared_ptr<int[5]>, int*, bad_deleter, test_allocator<int> >::value"
);
const _: () = assert!(
    true,
    "!std::is_constructible<std::shared_ptr<int[5]>, int(*)[5], test_deleter<int>, test_allocator<int> >::value"
);
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
    {
        let alloc_stats: Value<test_allocator_statistics> =
            Rc::new(RefCell::new(<test_allocator_statistics>::default()));
        {
            let mut ptr: Ptr<A> = Ptr::alloc(A::new());
            let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            let mut d: Ptr<test_deleter_A_> = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        dealloc_count_5.with(|rc| *rc.borrow_mut() = 0_());
    }
    {
        let alloc_stats: Value<test_allocator_statistics> =
            Rc::new(RefCell::new(<test_allocator_statistics>::default()));
        {
            let mut ptr: Ptr<A> = Ptr::alloc(A::new());
            let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            let mut d: Ptr<test_deleter_const_A_> = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        dealloc_count_5.with(|rc| *rc.borrow_mut() = 0_());
    }
    {
        let mut ptr: Ptr<A> = Ptr::alloc(A::new());
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut d: Ptr<test_deleter_A_> = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    dealloc_count_5.with(|rc| *rc.borrow_mut() = 0_());
    {
        let mut ptr: Ptr<A> = Ptr::alloc(A::new());
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut d: Ptr<test_deleter_A_> = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut d: move_deleter_int_ = move_deleter_int_::new_1({ 0_() });
        let p2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible<std::shared_ptr<int>, int*, bad_deleter,\n                                             test_allocator<A> >::value"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible<std::shared_ptr<int>, int*, no_move_deleter,\n                                             test_allocator<A> >::value"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible<std::shared_ptr<derived[4]>,\n                                             base[4], test_deleter<derived[4]>,\n                                             test_allocator<derived[4]> >::value"
        );;
    }
    {
        let ptr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    }
    return 0_();
}
pub trait AImpl {
    fn destructor(&self);
    fn copy_assign(&self, _a0: Ptr<A>) -> Ptr<A>;
}
impl AImpl for Ptr<A> {
    fn destructor(&self) {
        (*count_10.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn copy_assign(&self, _a0: Ptr<A>) -> Ptr<A> {
        return (*self).clone();
    }
}
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
pub trait bad_deleterImpl {
    fn operator_call(&self, _a0: bad_type);
}
impl bad_deleterImpl for Ptr<bad_deleter> {
    fn operator_call(&self, mut _a0: bad_type) {}
}
pub trait bare_allocator_void_Impl {
    fn allocate(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn deallocate(&self, mut p: (), mut _a1: ()) {
        unimplemented!()
    }
}
impl bare_allocator_void_Impl for Ptr<bare_allocator_void_> {}
pub trait complete_type_allocator_int_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!()
    }
}
impl complete_type_allocator_int_Impl for Ptr<complete_type_allocator_int_> {}
pub trait min_allocator_void_Impl {
    fn allocate(&self, mut n: ()) -> min_pointer_void__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_void__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_void_Impl for Ptr<min_allocator_void_> {}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait move_deleter_int_Impl {
    fn operator_call(&self, ptr: Ptr<()>);
}
impl move_deleter_int_Impl for Ptr<move_deleter_int_> {
    fn operator_call(&self, mut ptr: Ptr<()>) {
        ptr.delete();
    }
}
pub trait no_move_deleterImpl {
    fn operator_call(&self, _a0: Ptr<()>);
}
impl no_move_deleterImpl for Ptr<no_move_deleter> {
    fn operator_call(&self, mut _a0: Ptr<()>) {}
}
pub trait no_nullptr_deleterImpl {
    fn operator_call_1(&self, _a0: Ptr<()>);
}
impl no_nullptr_deleterImpl for Ptr<no_nullptr_deleter> {
    fn operator_call_1(&self, mut _a0: Ptr<()>) {}
}
pub trait test_allocator_A_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<A> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<A> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), mut _a1: ()) -> Ptr<A> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<A>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<A>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_A_Impl for Ptr<test_allocator_A_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_6.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_6.with(|rc| *rc.borrow()));
    }
}
pub trait test_allocator_base_Impl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn address_11(&self, x: ()) -> Ptr<base> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<base> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), mut _a1: ()) -> Ptr<base> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<base>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<base>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_base_Impl for Ptr<test_allocator_base_> {}
pub trait test_allocator_derivedarr4arr_Impl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn address_11(&self, x: ()) -> Ptr<Box<[derived]>> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<Box<[derived]>> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), mut _a1: ()) -> Ptr<Box<[derived]>> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<Box<[derived]>>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<Box<[derived]>>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_derivedarr4arr_Impl for Ptr<test_allocator_derivedarr4arr_> {}
pub trait test_allocator_int_Impl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn address_11(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), mut _a1: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<()>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_int_Impl for Ptr<test_allocator_int_> {}
pub trait test_allocator_statisticsImpl {
    fn clear(&self);
    fn clear_ctor_counters(&self);
}
impl test_allocator_statisticsImpl for Ptr<test_allocator_statistics> {
    fn clear(&self) {
        unimplemented!();
        field!((*self), count).write(0_());
        field!((*self), time_to_throw).write(0_());
        field!((*self), alloc_count).write(0_());
        field!((*self), allocated_size).write(0_());
        field!((*self), construct_count).write(0_());
        field!((*self), destroy_count).write(0_());
        field!((*self), throw_after).write(2147483647_());
        ({ test_allocator_statisticsImpl::clear_ctor_counters(self) });
    }
    fn clear_ctor_counters(&self) {
        field!((*self), copied).write(0_());
        field!((*self), moved).write(0_());
        field!((*self), converted).write(0_());
    }
}
pub trait test_allocator_void_Impl {
    fn destructor(&self);
    fn get_id(&self) -> ();
    fn get_data(&self) -> ();
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_>;
}
impl test_allocator_void_Impl for Ptr<test_allocator_void_> {
    fn destructor(&self) {
        field!((*self), data_).write(destructed_value_6.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_6.with(|rc| *rc.borrow()));
    }
    fn get_id(&self) -> () {
        return (*self).with(|__s| __s.id_);
    }
    fn get_data(&self) -> () {
        return (*self).with(|__s| __s.data_);
    }
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_> {
        field!((*self), data_).write({ _a0.with(|__s| __s.data_) });
        field!((*self), id_).write({ _a0.with(|__s| __s.id_) });
        field!((*self), stats_).write({ _a0.with(|__s| __s.stats_.clone()) });
        return (*self).clone();
    }
}
pub trait test_deleter_A_Impl {
    fn destructor(&self);
    fn state(&self) -> ();
    fn set_state(&self, mut i: ()) {
        unimplemented!()
    }
    fn operator_call(&self, p: Ptr<A>);
}
impl test_deleter_A_Impl for Ptr<test_deleter_A_> {
    fn destructor(&self) {
        unimplemented!();
        (*count_4.with(Value::clone).borrow_mut()).prefix_dec();
        field!((*self), state_).write(-1_());
    }
    fn state(&self) -> () {
        return (*self).with(|__s| __s.state_);
    }
    fn operator_call(&self, mut p: Ptr<A>) {
        unimplemented!();
        (*dealloc_count_5.with(Value::clone).borrow_mut()).prefix_inc();
        {
            let __p = p;
            __p.destructor();
            __p.delete();
        };
    }
}
pub trait test_deleter_base_Impl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn state(&self) -> () {
        unimplemented!()
    }
    fn set_state(&self, mut i: ()) {
        unimplemented!()
    }
    fn operator_call(&self, mut p: Ptr<base>) {
        unimplemented!()
    }
}
impl test_deleter_base_Impl for Ptr<test_deleter_base_> {}
pub trait test_deleter_const_A_Impl {
    fn destructor(&self);
    fn state(&self) -> ();
    fn set_state(&self, mut i: ()) {
        unimplemented!()
    }
    fn operator_call(&self, p: Ptr<A>);
}
impl test_deleter_const_A_Impl for Ptr<test_deleter_const_A_> {
    fn destructor(&self) {
        unimplemented!();
        (*count_4.with(Value::clone).borrow_mut()).prefix_dec();
        field!((*self), state_).write(-1_());
    }
    fn state(&self) -> () {
        return (*self).with(|__s| __s.state_);
    }
    fn operator_call(&self, mut p: Ptr<A>) {
        unimplemented!();
        (*dealloc_count_5.with(Value::clone).borrow_mut()).prefix_inc();
        {
            let __p = p;
            __p.destructor();
            __p.delete();
        };
    }
}
pub trait test_deleter_derivedarr4arr_Impl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn state(&self) -> () {
        unimplemented!()
    }
    fn set_state(&self, mut i: ()) {
        unimplemented!()
    }
    fn operator_call(&self, mut p: Ptr<Box<[derived]>>) {
        unimplemented!()
    }
}
impl test_deleter_derivedarr4arr_Impl for Ptr<test_deleter_derivedarr4arr_> {}
pub trait test_deleter_int_Impl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn state(&self) -> () {
        unimplemented!()
    }
    fn set_state(&self, mut i: ()) {
        unimplemented!()
    }
    fn operator_call(&self, mut p: Ptr<()>) {
        unimplemented!()
    }
}
impl test_deleter_int_Impl for Ptr<test_deleter_int_> {}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
    let _ = count_4.with(|_| ());
    let _ = dealloc_count_5.with(|_| ());
    let _ = destructed_value_6.with(|_| ());
    let _ = moved_value_7.with(|_| ());
    let _ = count_10.with(|_| ());
}
