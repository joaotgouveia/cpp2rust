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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_bool_ {}
impl CppMove for min_allocator_bool_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_unsigned_long_ {}
impl CppMove for min_allocator_unsigned_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn tests_5() -> bool {
    {
        let mut a: [bool; 7] = [
            (0 != 0),
            (1 != 0),
            (1 != 0),
            (0 != 0),
            (1 != 0),
            (0 != 0),
            (0 != 0),
        ];
        let mut N: u32 = (((::std::mem::size_of::<[bool; 7]>() as u64)
            .wrapping_div((::std::mem::size_of::<bool>() as u64)))
            as u32);
        let c: Value<std_vector_bool__std_allocator_bool__> = Rc::new(RefCell::new(
            <std_vector_bool__std_allocator_bool__>::default(),
        ));
        let mut i: u32 = 0_u32;
        'loop_: while (i < N) {
            ({ (*c.borrow()).push_back(a[(i) as usize]) });
            ({
                cpp2rust_assert_fail_4(
                    (({ (*c.borrow()).size() }) == (((i).wrapping_add(1_u32)) as u64)),
                )
            });
            let mut j: u64 = 0_u64;
            'loop_: while (j < ({ (*c.borrow()).size() })) {
                ({
                    cpp2rust_assert_fail_4(
                        ((({ (*c.borrow())[j]() }) as i32) == (a[(j) as usize] as i32)),
                    )
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let mut a: [bool; 7] = [
            (0 != 0),
            (1 != 0),
            (1 != 0),
            (0 != 0),
            (1 != 0),
            (0 != 0),
            (0 != 0),
        ];
        let mut N: u32 = (((::std::mem::size_of::<[bool; 7]>() as u64)
            .wrapping_div((::std::mem::size_of::<bool>() as u64)))
            as u32);
        let c: Value<std_vector_bool__min_allocator_bool__> = Rc::new(RefCell::new(
            <std_vector_bool__min_allocator_bool__>::default(),
        ));
        let mut i: u32 = 0_u32;
        'loop_: while (i < N) {
            ({ (*c.borrow()).push_back(a[(i) as usize]) });
            ({
                cpp2rust_assert_fail_4(
                    (({ (*c.borrow()).size() }) == (((i).wrapping_add(1_u32)) as u64)),
                )
            });
            let mut j: u64 = 0_u64;
            'loop_: while (j < ({ (*c.borrow()).size() })) {
                ({
                    cpp2rust_assert_fail_4(
                        ((({ (*c.borrow())[j]() }) as i32) == (a[(j) as usize] as i32)),
                    )
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
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
    ({ tests_5() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_bool__std_integral_constant_unsigned_long__0__;
pub trait min_allocator_bool_Impl {
    fn allocate(&self, mut n: u64) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_bool__std_integral_constant_unsigned_long__0__,
        mut n: u64,
    ) {
        unimplemented!()
    }
}
impl min_allocator_bool_Impl for Ptr<min_allocator_bool_> {}
pub trait min_allocator_unsigned_long_Impl {
    fn allocate(
        &self,
        n: u64,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__;
    fn deallocate(
        &self,
        p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        n: u64,
    );
}
impl min_allocator_unsigned_long_Impl for Ptr<min_allocator_unsigned_long_> {
    fn allocate(
        &self,
        mut n: u64,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        return min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__::new_1({
            ({ <std_allocator_unsigned_long_>::default().allocate(n) })
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        mut n: u64,
    ) {
        ({
            let ___p: Ptr<u64> = (p.ptr_).clone();
            let ___n: u64 = n;
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
