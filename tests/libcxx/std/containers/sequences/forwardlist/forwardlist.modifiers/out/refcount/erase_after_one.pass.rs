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
        let t: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
        let c: Value<std_forward_list_int__std_allocator_int__> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1(
                { ({ begin_5(t.as_pointer()) }) },
                { (t.as_pointer() as Ptr<i32>).to_end() },
                None,
            ),
        ));
        let i: Value<std__Fwd_list_iterator_int_> = Rc::new(RefCell::new(
            ({
                let ___pos: std__Fwd_list_const_iterator_int_ = libcc2rs::next_refcount(
                    ({ ({ (*c.borrow()).cbefore_begin() }) }),
                    4_isize as isize,
                );
                (*c.borrow()).erase_after_48(___pos)
            }),
        ));
        assert!((*i.borrow()) == ({ (*c.borrow()).end_27() }));
        assert!(
            ({
                (({
                    let ___first: std__Fwd_list_iterator_int_ = ({ (*c.borrow()).begin_25() });
                    let ___last: std__Fwd_list_iterator_int_ = ({ (*c.borrow()).end_27() });
                    distance_6(___first, ___last)
                }) == 4_isize)
            })
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 0_isize as isize)
                .iter_deref()
                .read())
                == 0)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 1_isize as isize)
                .iter_deref()
                .read())
                == 1)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 2_isize as isize)
                .iter_deref()
                .read())
                == 2)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 3_isize as isize)
                .iter_deref()
                .read())
                == 3)
        );
        {
            let __a1 = ({
                let ___pos: std__Fwd_list_const_iterator_int_ = libcc2rs::next_refcount(
                    ({ ({ (*c.borrow()).cbefore_begin() }) }),
                    0_isize as isize,
                );
                (*c.borrow()).erase_after_48(___pos)
            });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        assert!((*i.borrow()) == ({ (*c.borrow()).begin_25() }));
        assert!(
            ({
                (({
                    let ___first: std__Fwd_list_iterator_int_ = ({ (*c.borrow()).begin_25() });
                    let ___last: std__Fwd_list_iterator_int_ = ({ (*c.borrow()).end_27() });
                    distance_6(___first, ___last)
                }) == 3_isize)
            })
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 0_isize as isize)
                .iter_deref()
                .read())
                == 1)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 1_isize as isize)
                .iter_deref()
                .read())
                == 2)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 2_isize as isize)
                .iter_deref()
                .read())
                == 3)
        );
        {
            let __a1 = ({
                let ___pos: std__Fwd_list_const_iterator_int_ = libcc2rs::next_refcount(
                    ({ ({ (*c.borrow()).cbefore_begin() }) }),
                    1_isize as isize,
                );
                (*c.borrow()).erase_after_48(___pos)
            });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        assert!(
            (*i.borrow())
                == libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 1_isize as isize)
        );
        assert!(
            ({
                (({
                    let ___first: std__Fwd_list_iterator_int_ = ({ (*c.borrow()).begin_25() });
                    let ___last: std__Fwd_list_iterator_int_ = ({ (*c.borrow()).end_27() });
                    distance_6(___first, ___last)
                }) == 2_isize)
            })
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 0_isize as isize)
                .iter_deref()
                .read())
                == 1)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 1_isize as isize)
                .iter_deref()
                .read())
                == 3)
        );
        {
            let __a1 = ({
                let ___pos: std__Fwd_list_const_iterator_int_ = libcc2rs::next_refcount(
                    ({ ({ (*c.borrow()).cbefore_begin() }) }),
                    1_isize as isize,
                );
                (*c.borrow()).erase_after_48(___pos)
            });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        assert!((*i.borrow()) == ({ (*c.borrow()).end_27() }));
        assert!(
            ({
                (({
                    let ___first: std__Fwd_list_iterator_int_ = ({ (*c.borrow()).begin_25() });
                    let ___last: std__Fwd_list_iterator_int_ = ({ (*c.borrow()).end_27() });
                    distance_6(___first, ___last)
                }) == 1_isize)
            })
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 0_isize as isize)
                .iter_deref()
                .read())
                == 1)
        );
        {
            let __a1 = ({
                let ___pos: std__Fwd_list_const_iterator_int_ = libcc2rs::next_refcount(
                    ({ ({ (*c.borrow()).cbefore_begin() }) }),
                    0_isize as isize,
                );
                (*c.borrow()).erase_after_48(___pos)
            });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        assert!((*i.borrow()) == ({ (*c.borrow()).begin_25() }));
        assert!((*i.borrow()) == ({ (*c.borrow()).end_27() }));
        assert!(
            ({
                (({
                    let ___first: std__Fwd_list_iterator_int_ = ({ (*c.borrow()).begin_25() });
                    let ___last: std__Fwd_list_iterator_int_ = ({ (*c.borrow()).end_27() });
                    distance_6(___first, ___last)
                }) == 0_isize)
            })
        );
    }
    {
        let t: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
        let c: Value<std_forward_list_int__min_allocator_int__> = Rc::new(RefCell::new(
            std_forward_list_int__min_allocator_int__::new_1(
                { ({ begin_5(t.as_pointer()) }) },
                { (t.as_pointer() as Ptr<i32>).to_end() },
                None,
            ),
        ));
        let i : Value<std___fwdlist__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___ > = Rc::new(RefCell::new(( { let ___pos: std___fwdlist__Iterator_true__min_pointer_int__std_integral_constant_unsigned_long__0___   = libcc2rs::next_refcount(({ ( { (*c.borrow()) . cbefore_begin ( ) } )   }), 4_isize  as isize)   ; (*c.borrow()) . erase_after_48 ( ___pos , ) } )  )) ;
        assert!((*i.borrow()) == ({ (*c.borrow()).end_27() }));
        assert!(
            ({
                (({
                    let ___first: std___fwdlist__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___   = ( { (*c.borrow()) . begin_25 ( ) } )   ;
                    let ___last: std___fwdlist__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___   = ( { (*c.borrow()) . end_27 ( ) } )   ;
                    distance_7(___first, ___last)
                }) == 4_isize)
            })
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 0_isize as isize)
                .iter_deref()
                .read())
                == 0)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 1_isize as isize)
                .iter_deref()
                .read())
                == 1)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 2_isize as isize)
                .iter_deref()
                .read())
                == 2)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 3_isize as isize)
                .iter_deref()
                .read())
                == 3)
        );
        {
            let __a1 = ({
                let ___pos: std___fwdlist__Iterator_true__min_pointer_int__std_integral_constant_unsigned_long__0___   = libcc2rs::next_refcount(({ ( { (*c.borrow()) . cbefore_begin ( ) } )   }), 0_isize  as isize)   ;
                (*c.borrow()).erase_after_48(___pos)
            });
            i.as_pointer().with_mut(|__it| *__it = __a1.clone());
            i.as_pointer()
        };
        assert!((*i.borrow()) == ({ (*c.borrow()).begin_25() }));
        assert!(
            ({
                (({
                    let ___first: std___fwdlist__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___   = ( { (*c.borrow()) . begin_25 ( ) } )   ;
                    let ___last: std___fwdlist__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___   = ( { (*c.borrow()) . end_27 ( ) } )   ;
                    distance_7(___first, ___last)
                }) == 3_isize)
            })
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 0_isize as isize)
                .iter_deref()
                .read())
                == 1)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 1_isize as isize)
                .iter_deref()
                .read())
                == 2)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 2_isize as isize)
                .iter_deref()
                .read())
                == 3)
        );
        {
            let __a1 = ({
                let ___pos: std___fwdlist__Iterator_true__min_pointer_int__std_integral_constant_unsigned_long__0___   = libcc2rs::next_refcount(({ ( { (*c.borrow()) . cbefore_begin ( ) } )   }), 1_isize  as isize)   ;
                (*c.borrow()).erase_after_48(___pos)
            });
            i.as_pointer().with_mut(|__it| *__it = __a1.clone());
            i.as_pointer()
        };
        assert!(
            (*i.borrow())
                == libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 1_isize as isize)
        );
        assert!(
            ({
                (({
                    let ___first: std___fwdlist__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___   = ( { (*c.borrow()) . begin_25 ( ) } )   ;
                    let ___last: std___fwdlist__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___   = ( { (*c.borrow()) . end_27 ( ) } )   ;
                    distance_7(___first, ___last)
                }) == 2_isize)
            })
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 0_isize as isize)
                .iter_deref()
                .read())
                == 1)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 1_isize as isize)
                .iter_deref()
                .read())
                == 3)
        );
        {
            let __a1 = ({
                let ___pos: std___fwdlist__Iterator_true__min_pointer_int__std_integral_constant_unsigned_long__0___   = libcc2rs::next_refcount(({ ( { (*c.borrow()) . cbefore_begin ( ) } )   }), 1_isize  as isize)   ;
                (*c.borrow()).erase_after_48(___pos)
            });
            i.as_pointer().with_mut(|__it| *__it = __a1.clone());
            i.as_pointer()
        };
        assert!((*i.borrow()) == ({ (*c.borrow()).end_27() }));
        assert!(
            ({
                (({
                    let ___first: std___fwdlist__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___   = ( { (*c.borrow()) . begin_25 ( ) } )   ;
                    let ___last: std___fwdlist__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___   = ( { (*c.borrow()) . end_27 ( ) } )   ;
                    distance_7(___first, ___last)
                }) == 1_isize)
            })
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 0_isize as isize)
                .iter_deref()
                .read())
                == 1)
        );
        {
            let __a1 = ({
                let ___pos: std___fwdlist__Iterator_true__min_pointer_int__std_integral_constant_unsigned_long__0___   = libcc2rs::next_refcount(({ ( { (*c.borrow()) . cbefore_begin ( ) } )   }), 0_isize  as isize)   ;
                (*c.borrow()).erase_after_48(___pos)
            });
            i.as_pointer().with_mut(|__it| *__it = __a1.clone());
            i.as_pointer()
        };
        assert!((*i.borrow()) == ({ (*c.borrow()).begin_25() }));
        assert!((*i.borrow()) == ({ (*c.borrow()).end_27() }));
        assert!(
            ({
                (({
                    let ___first: std___fwdlist__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___   = ( { (*c.borrow()) . begin_25 ( ) } )   ;
                    let ___last: std___fwdlist__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___   = ( { (*c.borrow()) . end_27 ( ) } )   ;
                    distance_7(___first, ___last)
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
