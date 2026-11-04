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
pub struct X {}
impl CppMove for X {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
    let p1: Value<std_shared_ptr_int_> = Rc::new(RefCell::new(std_shared_ptr_int_::new_1({
        Ptr::alloc(Default::default())
    })));
    let p2: Value<std_shared_ptr_int_> = Rc::new(RefCell::new((*p1.borrow()).clone()));
    let p3: Value<std_shared_ptr_int_> = Rc::new(RefCell::new(std_shared_ptr_int_::new_1({
        Ptr::alloc(Default::default())
    })));
    let w1: Value<std_weak_ptr_int_> =
        Rc::new(RefCell::new(std_weak_ptr_int_::new_1({ p1.as_pointer() })));
    let w2: Value<std_weak_ptr_int_> =
        Rc::new(RefCell::new(std_weak_ptr_int_::new_1({ p2.as_pointer() })));
    let w3: Value<std_weak_ptr_int_> =
        Rc::new(RefCell::new(std_weak_ptr_int_::new_1({ p3.as_pointer() })));
    {
        let cs: Value<std_owner_less_std_shared_ptr_int__> = Rc::new(RefCell::new(
            <std_owner_less_std_shared_ptr_int__>::default(),
        ));
        assert!(
            ({
                !({
                    ((*cs.borrow_mut())
                        as std__Sp_owner_less_std_shared_ptr_int___std_weak_ptr_int__)(
                        p1.as_pointer(),
                        p2.as_pointer(),
                    )
                })
            })
        );
        assert!(
            ({
                !({
                    ((*cs.borrow_mut())
                        as std__Sp_owner_less_std_shared_ptr_int___std_weak_ptr_int__)(
                        p2.as_pointer(),
                        p1.as_pointer(),
                    )
                })
            })
        );
        assert!(
            ({
                ({
                    ((*cs.borrow_mut())
                        as std__Sp_owner_less_std_shared_ptr_int___std_weak_ptr_int__)(
                        p1.as_pointer(),
                        p3.as_pointer(),
                    )
                }) || ({
                    ((*cs.borrow_mut())
                        as std__Sp_owner_less_std_shared_ptr_int___std_weak_ptr_int__)(
                        p3.as_pointer(),
                        p1.as_pointer(),
                    )
                })
            })
        );
        assert!(
            ({
                ((({
                    ((*cs.borrow_mut())
                        as std__Sp_owner_less_std_shared_ptr_int___std_weak_ptr_int__)(
                        p3.as_pointer(),
                        p1.as_pointer(),
                    )
                }) as i32)
                    == (({
                        ((*cs.borrow_mut())
                            as std__Sp_owner_less_std_shared_ptr_int___std_weak_ptr_int__)(
                            p3.as_pointer(),
                            p2.as_pointer(),
                        )
                    }) as i32))
            })
        );
        const _: () = assert!(true, "");;
        assert!(
            ({
                !({
                    ((*cs.borrow_mut())
                        as std__Sp_owner_less_std_shared_ptr_int___std_weak_ptr_int__)(
                        p1.as_pointer(),
                        w2.as_pointer(),
                    )
                })
            })
        );
        assert!(
            ({
                !({
                    ((*cs.borrow_mut())
                        as std__Sp_owner_less_std_shared_ptr_int___std_weak_ptr_int__)(
                        p2.as_pointer(),
                        w1.as_pointer(),
                    )
                })
            })
        );
        assert!(
            ({
                ({
                    ((*cs.borrow_mut())
                        as std__Sp_owner_less_std_shared_ptr_int___std_weak_ptr_int__)(
                        p1.as_pointer(),
                        w3.as_pointer(),
                    )
                }) || ({
                    ((*cs.borrow_mut())
                        as std__Sp_owner_less_std_shared_ptr_int___std_weak_ptr_int__)(
                        p3.as_pointer(),
                        w1.as_pointer(),
                    )
                })
            })
        );
        assert!(
            ({
                ((({
                    ((*cs.borrow_mut())
                        as std__Sp_owner_less_std_shared_ptr_int___std_weak_ptr_int__)(
                        p3.as_pointer(),
                        w1.as_pointer(),
                    )
                }) as i32)
                    == (({
                        ((*cs.borrow_mut())
                            as std__Sp_owner_less_std_shared_ptr_int___std_weak_ptr_int__)(
                            p3.as_pointer(),
                            w2.as_pointer(),
                        )
                    }) as i32))
            })
        );
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let cs: Value<std_owner_less_std_weak_ptr_int__> =
            Rc::new(RefCell::new(<std_owner_less_std_weak_ptr_int__>::default()));
        assert!(
            ({
                !({
                    ((*cs.borrow_mut())
                        as std__Sp_owner_less_std_weak_ptr_int___std_shared_ptr_int__)(
                        w1.as_pointer(),
                        w2.as_pointer(),
                    )
                })
            })
        );
        assert!(
            ({
                !({
                    ((*cs.borrow_mut())
                        as std__Sp_owner_less_std_weak_ptr_int___std_shared_ptr_int__)(
                        w2.as_pointer(),
                        w1.as_pointer(),
                    )
                })
            })
        );
        assert!(
            ({
                ({
                    ((*cs.borrow_mut())
                        as std__Sp_owner_less_std_weak_ptr_int___std_shared_ptr_int__)(
                        w1.as_pointer(),
                        w3.as_pointer(),
                    )
                }) || ({
                    ((*cs.borrow_mut())
                        as std__Sp_owner_less_std_weak_ptr_int___std_shared_ptr_int__)(
                        w3.as_pointer(),
                        w1.as_pointer(),
                    )
                })
            })
        );
        assert!(
            ({
                ((({
                    ((*cs.borrow_mut())
                        as std__Sp_owner_less_std_weak_ptr_int___std_shared_ptr_int__)(
                        w3.as_pointer(),
                        w1.as_pointer(),
                    )
                }) as i32)
                    == (({
                        ((*cs.borrow_mut())
                            as std__Sp_owner_less_std_weak_ptr_int___std_shared_ptr_int__)(
                            w3.as_pointer(),
                            w2.as_pointer(),
                        )
                    }) as i32))
            })
        );
        const _: () = assert!(true, "");;
        assert!(
            ({
                !({
                    ((*cs.borrow_mut())
                        as std__Sp_owner_less_std_weak_ptr_int___std_shared_ptr_int__)(
                        w1.as_pointer(),
                        p2.as_pointer(),
                    )
                })
            })
        );
        assert!(
            ({
                !({
                    ((*cs.borrow_mut())
                        as std__Sp_owner_less_std_weak_ptr_int___std_shared_ptr_int__)(
                        w2.as_pointer(),
                        p1.as_pointer(),
                    )
                })
            })
        );
        assert!(
            ({
                ({
                    ((*cs.borrow_mut())
                        as std__Sp_owner_less_std_weak_ptr_int___std_shared_ptr_int__)(
                        w1.as_pointer(),
                        p3.as_pointer(),
                    )
                }) || ({
                    ((*cs.borrow_mut())
                        as std__Sp_owner_less_std_weak_ptr_int___std_shared_ptr_int__)(
                        w3.as_pointer(),
                        p1.as_pointer(),
                    )
                })
            })
        );
        assert!(
            ({
                ((({
                    ((*cs.borrow_mut())
                        as std__Sp_owner_less_std_weak_ptr_int___std_shared_ptr_int__)(
                        w3.as_pointer(),
                        p1.as_pointer(),
                    )
                }) as i32)
                    == (({
                        ((*cs.borrow_mut())
                            as std__Sp_owner_less_std_weak_ptr_int___std_shared_ptr_int__)(
                            w3.as_pointer(),
                            p2.as_pointer(),
                        )
                    }) as i32))
            })
        );
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let sp1: Value<std_shared_ptr_int_> =
            Rc::new(RefCell::new(<std_shared_ptr_int_>::default()));
        let sp2: Value<std_shared_ptr_void_> =
            Rc::new(RefCell::new(<std_shared_ptr_void_>::default()));
        let sp3: Value<std_shared_ptr_long_> =
            Rc::new(RefCell::new(<std_shared_ptr_long_>::default()));
        let wp1: Value<std_weak_ptr_int_> = Rc::new(RefCell::new(<std_weak_ptr_int_>::default()));
        let cmp: Value<std_owner_less_void_> =
            Rc::new(RefCell::new(<std_owner_less_void_>::default()));
        assert!(
            ({
                !({
                    ((*cmp.borrow_mut()) as std__Sp_owner_less_void__void_)(
                        sp1.as_pointer(),
                        sp2.as_pointer(),
                    )
                })
            })
        );
        assert!(
            ({
                !({
                    ((*cmp.borrow_mut()) as std__Sp_owner_less_void__void_)(
                        sp1.as_pointer(),
                        wp1.as_pointer(),
                    )
                })
            })
        );
        assert!(
            ({
                !({
                    ((*cmp.borrow_mut()) as std__Sp_owner_less_void__void_)(
                        sp1.as_pointer(),
                        sp3.as_pointer(),
                    )
                })
            })
        );
        assert!(
            ({
                !({
                    ((*cmp.borrow_mut()) as std__Sp_owner_less_void__void_)(
                        wp1.as_pointer(),
                        sp1.as_pointer(),
                    )
                })
            })
        );
        assert!(
            ({
                !({
                    let ___lhs: Ptr<std_weak_ptr_int_> = wp1.as_pointer();
                    let ___rhs: Ptr<std_weak_ptr_int_> = wp1.as_pointer();
                    ((*cmp.borrow_mut()) as std__Sp_owner_less_void__void_)(___lhs, ___rhs)
                })
            })
        );
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let s : Value<std_set_std_shared_ptr_X___std_owner_less_void___std_allocator_std_shared_ptr_X___ > = Rc::new(RefCell::new(<std_set_std_shared_ptr_X___std_owner_less_void___std_allocator_std_shared_ptr_X___  >::default() )) ;
        let vp: Value<std_shared_ptr_void_> =
            Rc::new(RefCell::new(<std_shared_ptr_void_>::default()));
        assert!(
            ({ (*s.borrow()).find_61_class_std_shared_ptr_void_(vp.as_pointer(),) })
                == ({ (*s.borrow()).end() })
        );
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
