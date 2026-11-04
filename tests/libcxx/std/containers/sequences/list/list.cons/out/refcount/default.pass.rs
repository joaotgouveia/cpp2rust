extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_1(x: Ptr<DefaultOnly>, y: Ptr<DefaultOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_lt_2(x: Ptr<DefaultOnly>, y: Ptr<DefaultOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(4)]
pub struct DefaultOnly {
    #[offset(0)]
    data_: i32,
}
impl DefaultOnly {
    pub fn new() -> Self {
        let __this: DefaultOnly = Self { data_: -1_i32 };
        (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl CppLess for DefaultOnly {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_2(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for DefaultOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_1(__lhs.clone(), __rhs.clone())
    }
}
impl Default for DefaultOnly {
    fn default() -> Self {
        { DefaultOnly::new() }
    }
}
thread_local!(
    pub static count_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> u64 {
        ({
            cpp2rust_assert_fail_7(
                (alloc_count_4.with(|rc| *rc.borrow()) >= dealloc_count_5.with(|rc| *rc.borrow())),
            )
        });
        return ((alloc_count_4.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_5.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        ({ cpp2rust_assert_fail_7((({ malloc_allocator_base::outstanding_alloc() }) == 0_u64)) });
        disable_default_constructor_6.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_3.with(|rc| *rc.borrow_mut() = 0_u64);
        alloc_count_4.with(|rc| *rc.borrow_mut() = 0_u64);
        dealloc_count_5.with(|rc| *rc.borrow_mut() = 0_u64);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_3: Value<u64> = Rc::new(RefCell::new(0_u64));
);
thread_local!(
    pub static alloc_count_4: Value<u64> = Rc::new(RefCell::new(0_u64));
);
thread_local!(
    pub static dealloc_count_5: Value<u64> = Rc::new(RefCell::new(0_u64));
);
thread_local!(
    pub static disable_default_constructor_6: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
impl CppMove for min_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_DefaultOnly_ {}
impl CppMove for min_allocator_DefaultOnly_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct explicit_allocator_int_ {}
impl explicit_allocator_int_ {
    pub fn new() -> Self {
        Self {}
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
pub fn test_8() -> bool {
    {
        let l: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(<std_list_int__std_allocator_int__>::default()));
        ({ cpp2rust_assert_fail_7((({ (*l.borrow()).size() }) == 0_u64)) });
        ({
            cpp2rust_assert_fail_7(
                (({
                    let ___first: std__List_iterator_int_ = ({ (*l.borrow()).begin_24() });
                    let ___last: std__List_iterator_int_ = ({ (*l.borrow()).end_26() });
                    distance_9(___first, ___last)
                }) == 0_i64),
            )
        });
    }
    {
        let l: Value<std_list_DefaultOnly__std_allocator_DefaultOnly__> = Rc::new(RefCell::new(
            <std_list_DefaultOnly__std_allocator_DefaultOnly__>::default(),
        ));
        ({ cpp2rust_assert_fail_7((({ (*l.borrow()).size() }) == 0_u64)) });
        ({
            cpp2rust_assert_fail_7(
                (({
                    let ___first: std__List_iterator_DefaultOnly_ = ({ (*l.borrow()).begin_24() });
                    let ___last: std__List_iterator_DefaultOnly_ = ({ (*l.borrow()).end_26() });
                    distance_10(___first, ___last)
                }) == 0_i64),
            )
        });
    }
    {
        let l: Value<std_list_int__std_allocator_int__> = Rc::new(RefCell::new({
            let __tmp_0: Value<std_allocator_int_> =
                Rc::new(RefCell::new((<std_allocator_int_>::default())));
            std_list_int__std_allocator_int__::new_1({ __tmp_0.as_pointer() })
        }));
        ({ cpp2rust_assert_fail_7((({ (*l.borrow()).size() }) == 0_u64)) });
        ({
            cpp2rust_assert_fail_7(
                (({
                    let ___first: std__List_iterator_int_ = ({ (*l.borrow()).begin_24() });
                    let ___last: std__List_iterator_int_ = ({ (*l.borrow()).end_26() });
                    distance_9(___first, ___last)
                }) == 0_i64),
            )
        });
    }
    {
        let l: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(<std_list_int__min_allocator_int__>::default()));
        ({ cpp2rust_assert_fail_7((({ (*l.borrow()).size() }) == 0_u64)) });
        ({
            cpp2rust_assert_fail_7(
                (({
                    let ___first: std___list__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___   = ( { (*l.borrow()) . begin_24 ( ) } )   ;
                    let ___last: std___list__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___   = ( { (*l.borrow()) . end_26 ( ) } )   ;
                    distance_11(___first, ___last)
                }) == 0_i64),
            )
        });
    }
    {
        let l: Value<std_list_DefaultOnly__min_allocator_DefaultOnly__> = Rc::new(RefCell::new(
            <std_list_DefaultOnly__min_allocator_DefaultOnly__>::default(),
        ));
        ({ cpp2rust_assert_fail_7((({ (*l.borrow()).size() }) == 0_u64)) });
        ({
            cpp2rust_assert_fail_7(
                (({
                    let ___first: std___list__Iterator_false__min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0___   = ( { (*l.borrow()) . begin_24 ( ) } )   ;
                    let ___last: std___list__Iterator_false__min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0___   = ( { (*l.borrow()) . end_26 ( ) } )   ;
                    distance_12(___first, ___last)
                }) == 0_i64),
            )
        });
    }
    {
        let l: Value<std_list_int__min_allocator_int__> = Rc::new(RefCell::new({
            let __tmp_1: Value<min_allocator_int_> =
                Rc::new(RefCell::new((<min_allocator_int_>::default())));
            std_list_int__min_allocator_int__::new_1({ __tmp_1.as_pointer() })
        }));
        ({ cpp2rust_assert_fail_7((({ (*l.borrow()).size() }) == 0_u64)) });
        ({
            cpp2rust_assert_fail_7(
                (({
                    let ___first: std___list__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___   = ( { (*l.borrow()) . begin_24 ( ) } )   ;
                    let ___last: std___list__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___   = ( { (*l.borrow()) . end_26 ( ) } )   ;
                    distance_11(___first, ___last)
                }) == 0_i64),
            )
        });
    }
    {
        let l: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(<std_list_int__std_allocator_int__>::default()));
        ({ cpp2rust_assert_fail_7((({ (*l.borrow()).size() }) == 0_u64)) });
        ({
            cpp2rust_assert_fail_7(
                (({
                    let ___first: std__List_iterator_int_ = ({ (*l.borrow()).begin_24() });
                    let ___last: std__List_iterator_int_ = ({ (*l.borrow()).end_26() });
                    distance_9(___first, ___last)
                }) == 0_i64),
            )
        });
    }
    {
        let l: Value<std_list_int__explicit_allocator_int__> = Rc::new(RefCell::new(
            <std_list_int__explicit_allocator_int__>::default(),
        ));
        ({ cpp2rust_assert_fail_7((({ (*l.borrow()).size() }) == 0_u64)) });
        ({
            cpp2rust_assert_fail_7(
                (({
                    let ___first: std__List_iterator_int_ = ({ (*l.borrow()).begin_24() });
                    let ___last: std__List_iterator_int_ = ({ (*l.borrow()).end_26() });
                    distance_9(___first, ___last)
                }) == 0_i64),
            )
        });
    }
    {
        let l: Value<std_list_int__explicit_allocator_int__> = Rc::new(RefCell::new({
            let __tmp_2: Value<explicit_allocator_int_> =
                Rc::new(RefCell::new((explicit_allocator_int_::new())));
            std_list_int__explicit_allocator_int__::new_1({ __tmp_2.as_pointer() })
        }));
        ({ cpp2rust_assert_fail_7((({ (*l.borrow()).size() }) == 0_u64)) });
        ({
            cpp2rust_assert_fail_7(
                (({
                    let ___first: std__List_iterator_int_ = ({ (*l.borrow()).begin_24() });
                    let ___last: std__List_iterator_int_ = ({ (*l.borrow()).end_26() });
                    distance_9(___first, ___last)
                }) == 0_i64),
            )
        });
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
    ({ cpp2rust_assert_fail_7(({ test_8() })) });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
pub trait DefaultOnlyImpl {
    fn copy_assign(&self, _a0: Ptr<DefaultOnly>) -> Ptr<DefaultOnly> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl DefaultOnlyImpl for Ptr<DefaultOnly> {
    fn destructor(&self) {
        field!((*self), data_).write(0);
        (*count_0.with(Value::clone).borrow_mut()).prefix_dec();
    }
}
pub trait explicit_allocator_int_Impl {
    fn allocate(&self, mut n: u64) -> Ptr<i32> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<i32>, mut n: u64) {
        unimplemented!()
    }
}
impl explicit_allocator_int_Impl for Ptr<explicit_allocator_int_> {}
pub trait min_allocator_DefaultOnly_Impl {
    fn allocate(
        &self,
        mut n: u64,
    ) -> min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__,
        mut n: u64,
    ) {
        unimplemented!()
    }
}
impl min_allocator_DefaultOnly_Impl for Ptr<min_allocator_DefaultOnly_> {}
pub trait min_allocator_int_Impl {
    fn allocate(&self, mut n: u64) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: u64,
    ) {
        unimplemented!()
    }
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {}
pub fn __cpp2rust_init_globals() {
    let _ = count_0.with(|_| ());
    let _ = outstanding_bytes_3.with(|_| ());
    let _ = alloc_count_4.with(|_| ());
    let _ = dealloc_count_5.with(|_| ());
    let _ = disable_default_constructor_6.with(|_| ());
}
