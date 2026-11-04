extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NotConstructible {}
impl NotConstructible {}
impl CppEq for NotConstructible {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_0(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_eq_0(_a0: Ptr<NotConstructible>, _a1: Ptr<NotConstructible>) -> bool {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_2.with(|rc| *rc.borrow()) >= dealloc_count_3.with(|rc| *rc.borrow())));
        return ((alloc_count_2.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_3.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_4.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_1.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_2.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_3.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_1: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_2: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_3: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_4: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_NotConstructible_ {}
impl CppMove for min_allocator_NotConstructible_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
impl CppMove for min_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_5() -> bool {
    {
        let c: Value<std_forward_list_NotConstructible__std_allocator_NotConstructible__> =
            Rc::new(RefCell::new(
                <std_forward_list_NotConstructible__std_allocator_NotConstructible__>::default(),
            ));
        const _: () = assert!(true, "");;
        ({ (*c.borrow()).clear() });
        assert!(
            ({
                (({
                    let ___first: std__Fwd_list_iterator_NotConstructible_ =
                        ({ (*c.borrow()).begin_25() });
                    let ___last: std__Fwd_list_iterator_NotConstructible_ =
                        ({ (*c.borrow()).end_27() });
                    distance_6(___first, ___last)
                }) == 0_isize)
            })
        );
    }
    {
        let t: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
        let c: Value<std_forward_list_int__std_allocator_int__> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1(
                { ({ begin_7(t.as_pointer()) }) },
                { (t.as_pointer() as Ptr<i32>).to_end() },
                None,
            ),
        ));
        const _: () = assert!(true, "");;
        ({ (*c.borrow()).clear() });
        assert!(
            ({
                (({
                    let ___first: std__Fwd_list_iterator_int_ = ({ (*c.borrow()).begin_25() });
                    let ___last: std__Fwd_list_iterator_int_ = ({ (*c.borrow()).end_27() });
                    distance_8(___first, ___last)
                }) == 0_isize)
            })
        );
        ({ (*c.borrow()).clear() });
        assert!(
            ({
                (({
                    let ___first: std__Fwd_list_iterator_int_ = ({ (*c.borrow()).begin_25() });
                    let ___last: std__Fwd_list_iterator_int_ = ({ (*c.borrow()).end_27() });
                    distance_8(___first, ___last)
                }) == 0_isize)
            })
        );
    }
    {
        let c: Value<std_forward_list_NotConstructible__min_allocator_NotConstructible__> =
            Rc::new(RefCell::new(
                <std_forward_list_NotConstructible__min_allocator_NotConstructible__>::default(),
            ));
        const _: () = assert!(true, "");;
        ({ (*c.borrow()).clear() });
        assert!(
            ({
                (({
                    let ___first: std___fwdlist__Iterator_false__min_pointer_NotConstructible__std_integral_constant_unsigned_long__0___   = ( { (*c.borrow()) . begin_25 ( ) } )   ;
                    let ___last: std___fwdlist__Iterator_false__min_pointer_NotConstructible__std_integral_constant_unsigned_long__0___   = ( { (*c.borrow()) . end_27 ( ) } )   ;
                    distance_9(___first, ___last)
                }) == 0_isize)
            })
        );
    }
    {
        let t: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
        let c: Value<std_forward_list_int__min_allocator_int__> = Rc::new(RefCell::new(
            std_forward_list_int__min_allocator_int__::new_1(
                { ({ begin_7(t.as_pointer()) }) },
                { (t.as_pointer() as Ptr<i32>).to_end() },
                None,
            ),
        ));
        const _: () = assert!(true, "");;
        ({ (*c.borrow()).clear() });
        assert!(
            ({
                (({
                    let ___first: std___fwdlist__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___   = ( { (*c.borrow()) . begin_25 ( ) } )   ;
                    let ___last: std___fwdlist__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___   = ( { (*c.borrow()) . end_27 ( ) } )   ;
                    distance_10(___first, ___last)
                }) == 0_isize)
            })
        );
        ({ (*c.borrow()).clear() });
        assert!(
            ({
                (({
                    let ___first: std___fwdlist__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___   = ( { (*c.borrow()) . begin_25 ( ) } )   ;
                    let ___last: std___fwdlist__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___   = ( { (*c.borrow()) . end_27 ( ) } )   ;
                    distance_10(___first, ___last)
                }) == 0_isize)
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
    assert!(({ test_5() }));
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__;
pub trait NotConstructibleImpl {
    fn copy_assign(&self, _a0: Ptr<NotConstructible>) -> Ptr<NotConstructible> {
        unimplemented!()
    }
}
impl NotConstructibleImpl for Ptr<NotConstructible> {}
pub trait min_allocator_NotConstructible_Impl {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_NotConstructible_Impl for Ptr<min_allocator_NotConstructible_> {}
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
    let _ = outstanding_bytes_1.with(|_| ());
    let _ = alloc_count_2.with(|_| ());
    let _ = dealloc_count_3.with(|_| ());
    let _ = disable_default_constructor_4.with(|_| ());
}
