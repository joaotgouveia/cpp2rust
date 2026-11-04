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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
impl CppMove for min_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_4() -> bool {
    {
        let c: Value<std_forward_list_int__std_allocator_int__> = Rc::new(RefCell::new(
            <std_forward_list_int__std_allocator_int__>::default(),
        ));
        let i: Value<std__Fwd_list_iterator_int_> =
            Rc::new(RefCell::new(({ (*c.borrow()).begin_25() })));
        let j: Value<std__Fwd_list_iterator_int_> =
            Rc::new(RefCell::new(({ (*c.borrow()).end_27() })));
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        assert!(({ (*i.borrow()) == (*j.borrow()) }));
    }
    {
        let c: Value<std_forward_list_int__std_allocator_int__> = Rc::new(RefCell::new(
            <std_forward_list_int__std_allocator_int__>::default(),
        ));
        let i: Value<std__Fwd_list_const_iterator_int_> =
            Rc::new(RefCell::new(({ (*c.borrow()).begin_26() })));
        let j: Value<std__Fwd_list_const_iterator_int_> =
            Rc::new(RefCell::new(({ (*c.borrow()).end_28() })));
        assert!(({ (({ distance_6((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        assert!(({ (*i.borrow()) == (*j.borrow()) }));
    }
    {
        let c: Value<std_forward_list_int__std_allocator_int__> = Rc::new(RefCell::new(
            <std_forward_list_int__std_allocator_int__>::default(),
        ));
        let i: Value<std__Fwd_list_const_iterator_int_> =
            Rc::new(RefCell::new(({ (*c.borrow()).cbegin() })));
        let j: Value<std__Fwd_list_const_iterator_int_> =
            Rc::new(RefCell::new(({ (*c.borrow()).cend() })));
        assert!(({ (({ distance_6((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        assert!(({ (*i.borrow()) == (*j.borrow()) }));
        assert!(
            (*i.borrow())
                == <std__Fwd_list_const_iterator_int_>::from(({ (*c.borrow()).end_27() }))
        );
    }
    {
        let t: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9])));
        let c: Value<std_forward_list_int__std_allocator_int__> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1(
                { ({ begin_7(t.as_pointer()) }) },
                { (t.as_pointer() as Ptr<i32>).to_end() },
                None,
            ),
        ));
        let i: Value<std__Fwd_list_iterator_int_> =
            Rc::new(RefCell::new(({ (*c.borrow()).begin_25() })));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 0) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        (*i.borrow()).iter_deref().write(10);
        assert!(({ (((*i.borrow()).iter_deref().read()) == 10) }));
        assert!(
            ({
                (({
                    let ___first: std__Fwd_list_iterator_int_ = ({ (*c.borrow()).begin_25() });
                    let ___last: std__Fwd_list_iterator_int_ = ({ (*c.borrow()).end_27() });
                    distance_5(___first, ___last)
                }) == 10_isize)
            })
        );
    }
    {
        let i: Value<std__Fwd_list_iterator_int_> =
            Rc::new(RefCell::new(<std__Fwd_list_iterator_int_>::default()));
        let j: Value<std__Fwd_list_const_iterator_int_> =
            Rc::new(RefCell::new(<std__Fwd_list_const_iterator_int_>::default()));
        &(*i.borrow_mut());
        &(*j.borrow_mut());
    }
    {
        let c: Value<std_forward_list_int__min_allocator_int__> = Rc::new(RefCell::new(
            <std_forward_list_int__min_allocator_int__>::default(),
        ));
        let i : Value<std___fwdlist__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___ > = Rc::new(RefCell::new(( { (*c.borrow()) . begin_25 ( ) } )  )) ;
        let j : Value<std___fwdlist__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___ > = Rc::new(RefCell::new(( { (*c.borrow()) . end_27 ( ) } )  )) ;
        assert!(({ (({ distance_8((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        assert!(({ (*i.borrow()) == (*j.borrow()) }));
    }
    {
        let c: Value<std_forward_list_int__min_allocator_int__> = Rc::new(RefCell::new(
            <std_forward_list_int__min_allocator_int__>::default(),
        ));
        let i : Value<std___fwdlist__Iterator_true__min_pointer_int__std_integral_constant_unsigned_long__0___ > = Rc::new(RefCell::new(( { (*c.borrow()) . begin_26 ( ) } )  )) ;
        let j : Value<std___fwdlist__Iterator_true__min_pointer_int__std_integral_constant_unsigned_long__0___ > = Rc::new(RefCell::new(( { (*c.borrow()) . end_28 ( ) } )  )) ;
        assert!(({ (({ distance_9((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        assert!(({ (*i.borrow()) == (*j.borrow()) }));
    }
    {
        let c: Value<std_forward_list_int__min_allocator_int__> = Rc::new(RefCell::new(
            <std_forward_list_int__min_allocator_int__>::default(),
        ));
        let i : Value<std___fwdlist__Iterator_true__min_pointer_int__std_integral_constant_unsigned_long__0___ > = Rc::new(RefCell::new(( { (*c.borrow()) . cbegin ( ) } )  )) ;
        let j : Value<std___fwdlist__Iterator_true__min_pointer_int__std_integral_constant_unsigned_long__0___ > = Rc::new(RefCell::new(( { (*c.borrow()) . cend ( ) } )  )) ;
        assert!(({ (({ distance_9((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        assert!(({ (*i.borrow()) == (*j.borrow()) }));
        assert!((*i.borrow())  == <std___fwdlist__Iterator_true__min_pointer_int__std_integral_constant_unsigned_long__0___  >::from(( { (*c.borrow()) . end_27 ( ) } )  )   );
    }
    {
        let t: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9])));
        let c: Value<std_forward_list_int__min_allocator_int__> = Rc::new(RefCell::new(
            std_forward_list_int__min_allocator_int__::new_1(
                { ({ begin_7(t.as_pointer()) }) },
                { (t.as_pointer() as Ptr<i32>).to_end() },
                None,
            ),
        ));
        let i : Value<std___fwdlist__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___ > = Rc::new(RefCell::new(( { (*c.borrow()) . begin_25 ( ) } )  )) ;
        assert!(({ (((*i.borrow()).iter_deref().read()) == 0) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        (*i.borrow()).iter_deref().write(10);
        assert!(({ (((*i.borrow()).iter_deref().read()) == 10) }));
        assert!(
            ({
                (({
                    let ___first: std___fwdlist__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___   = ( { (*c.borrow()) . begin_25 ( ) } )   ;
                    let ___last: std___fwdlist__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___   = ( { (*c.borrow()) . end_27 ( ) } )   ;
                    distance_8(___first, ___last)
                }) == 10_isize)
            })
        );
    }
    {
        let i : Value<std___fwdlist__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___ > = Rc::new(RefCell::new(<std___fwdlist__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___  >::default() )) ;
        let j : Value<std___fwdlist__Iterator_true__min_pointer_int__std_integral_constant_unsigned_long__0___ > = Rc::new(RefCell::new(<std___fwdlist__Iterator_true__min_pointer_int__std_integral_constant_unsigned_long__0___  >::default() )) ;
        &(*i.borrow_mut());
        &(*j.borrow_mut());
    }
    {
        let ii1: Value<std__Fwd_list_iterator_int_> =
            Rc::new(RefCell::new(<std__Fwd_list_iterator_int_>::default()));
        let ii2: Value<std__Fwd_list_iterator_int_> =
            Rc::new(RefCell::new(<std__Fwd_list_iterator_int_>::default()));
        let ii4: Value<std__Fwd_list_iterator_int_> =
            Rc::new(RefCell::new((*ii1.borrow()).clone()));
        let cii: Value<std__Fwd_list_const_iterator_int_> =
            Rc::new(RefCell::new(<std__Fwd_list_const_iterator_int_>::default()));
        assert!(({ (*ii1.borrow()) == (*ii2.borrow()) }));
        assert!(({ (*ii1.borrow()) == (*ii4.borrow()) }));
        assert!(({ !((*ii1.borrow()) != (*ii2.borrow())) }));
        assert!(
            (<std__Fwd_list_const_iterator_int_>::from(({ (*ii1.borrow()).clone() }))
                == (*cii.borrow()))
        );
        assert!(
            ((*cii.borrow())
                == <std__Fwd_list_const_iterator_int_>::from(({ (*ii1.borrow()).clone() })))
        );
        assert!(
            !(<std__Fwd_list_const_iterator_int_>::from(({ (*ii1.borrow()).clone() }))
                != (*cii.borrow()))
        );
        assert!(
            !((*cii.borrow())
                != <std__Fwd_list_const_iterator_int_>::from(({ (*ii1.borrow()).clone() })))
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
    assert!(({ test_4() }));
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
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
}
