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
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_1.with(|rc| *rc.borrow()) >= dealloc_count_2.with(|rc| *rc.borrow())));
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_1: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_2: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_3: Value<bool> = Rc::new(RefCell::new(false));
);
pub fn operator_eq_4(mut _a0: min_allocator_int_, mut _a1: min_allocator_int_) -> bool {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
impl CppEq for min_allocator_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_4(__lhs.read(), __rhs.read())
    }
}
impl CppMove for min_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
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
        (*gConstructed_5.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(rhs: Ptr<Counter_int_>) -> Self {
        let __this: Counter_int_ = Self {
            data_: rhs.with(|__s| __s.data_),
        };
        (*gConstructed_5.with(Value::clone).borrow_mut()).prefix_inc();
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
    pub static gConstructed_5: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn test_6() -> bool {
    {
        let m: Value<std_set_int__std_less_int___std_allocator_int__> = Rc::new(RefCell::new(
            std_set_int__std_less_int___std_allocator_int__::new_1(
                { Vec::from([1, 2, 3, 4, 5, 6]) },
                None,
                None,
            ),
        ));
        ({ test_7(m.as_pointer()) });
    }
    {
        let m: Value<std_set_Counter_int___std_less_Counter_int____std_allocator_Counter_int___> =
            Rc::new(RefCell::new(
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
                            {
                                let __tmp_3: Value<i32> = Rc::new(RefCell::new(4));
                                Counter_int_::new_1({ __tmp_3.as_pointer() })
                            },
                            {
                                let __tmp_4: Value<i32> = Rc::new(RefCell::new(5));
                                Counter_int_::new_1({ __tmp_4.as_pointer() })
                            },
                            {
                                let __tmp_5: Value<i32> = Rc::new(RefCell::new(6));
                                Counter_int_::new_1({ __tmp_5.as_pointer() })
                            },
                        ])
                    },
                    None,
                    None,
                ),
            ));
        assert!((gConstructed_5.with(|rc| *rc.borrow()) == 6));
        ({ test_8(m.as_pointer()) });
        assert!((gConstructed_5.with(|rc| *rc.borrow()) == 0));
    }
    {
        let m: Value<std_set_int__std_less_int___min_allocator_int__> = Rc::new(RefCell::new(
            std_set_int__std_less_int___min_allocator_int__::new_1(
                { Vec::from([1, 2, 3, 4, 5, 6]) },
                None,
                None,
            ),
        ));
        ({ test_9(m.as_pointer()) });
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
    ({ test_6() });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
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
        (*gConstructed_5.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn operator_eq(&self, x: Ptr<Counter_int_>) -> bool {
        return ({ (*self).with(|__s| __s.data_) } == { x.with(|__s| __s.data_) });
    }
    fn operator_lt(&self, x: Ptr<Counter_int_>) -> bool {
        return ({ (*self).with(|__s| __s.data_) } < { x.with(|__s| __s.data_) });
    }
}
pub trait min_allocator_int_Impl {
    fn allocate(&self, mut n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
    let _ = gConstructed_5.with(|_| ());
}
