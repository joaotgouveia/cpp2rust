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
    pub fn outstanding_alloc() -> u64 {
        ({
            cpp2rust_assert_fail_4(
                (alloc_count_1.with(|rc| *rc.borrow()) >= dealloc_count_2.with(|rc| *rc.borrow())),
            )
        });
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        ({ cpp2rust_assert_fail_4((({ malloc_allocator_base::outstanding_alloc() }) == 0_u64)) });
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_u64);
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_u64);
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_u64);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<u64> = Rc::new(RefCell::new(0_u64));
);
thread_local!(
    pub static alloc_count_1: Value<u64> = Rc::new(RefCell::new(0_u64));
);
thread_local!(
    pub static dealloc_count_2: Value<u64> = Rc::new(RefCell::new(0_u64));
);
thread_local!(
    pub static disable_default_constructor_3: Value<bool> = Rc::new(RefCell::new(false));
);
pub fn test_5() -> bool {
    {
        let m : Value<std_map_int__double__std_less_int___std_allocator_std_pair_const_int__double___ > = Rc::new(RefCell::new(<std_map_int__double__std_less_int___std_allocator_std_pair_const_int__double___  >::default() )) ;
        ({ cpp2rust_assert_fail_4(({ (*m.borrow()).empty() })) });
        ({
            let ___x: Value<std_pair_const_int__double_> = Rc::new(RefCell::new({
                let __tmp_0: Value<i32> = Rc::new(RefCell::new(1));
                let __tmp_1: Value<f64> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_1({ __tmp_0.as_pointer() }, {
                    __tmp_1.as_pointer()
                })
            }));
            (*m.borrow()).insert_54(___x.as_pointer())
        });
        ({ cpp2rust_assert_fail_4(!({ (*m.borrow()).empty() })) });
        ({ (*m.borrow()).clear() });
        ({ cpp2rust_assert_fail_4(({ (*m.borrow()).empty() })) });
    }
    {
        let m : Value<std_map_int__double__std_less_int___min_allocator_std_pair_const_int__double___ > = Rc::new(RefCell::new(<std_map_int__double__std_less_int___min_allocator_std_pair_const_int__double___  >::default() )) ;
        ({ cpp2rust_assert_fail_4(({ (*m.borrow()).empty() })) });
        ({
            let ___x: Value<std_pair_const_int__double_> = Rc::new(RefCell::new({
                let __tmp_2: Value<i32> = Rc::new(RefCell::new(1));
                let __tmp_3: Value<f64> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_1({ __tmp_2.as_pointer() }, {
                    __tmp_3.as_pointer()
                })
            }));
            (*m.borrow()).insert_54(___x.as_pointer())
        });
        ({ cpp2rust_assert_fail_4(!({ (*m.borrow()).empty() })) });
        ({ (*m.borrow()).clear() });
        ({ cpp2rust_assert_fail_4(({ (*m.borrow()).empty() })) });
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
    ({ test_5() });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
