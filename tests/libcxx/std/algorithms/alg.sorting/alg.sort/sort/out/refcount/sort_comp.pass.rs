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
pub struct indirect_less {}
impl CppMove for indirect_less {
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
    {
        let v: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
            std_vector_int__std_allocator_int__::new_1({ 1000_u64 }, None),
        ));
        let mut i: i32 = 0;
        'loop_: while ((i as u64) < ({ (*v.borrow()).size() })) {
            let __rhs = i;
            (*v.borrow_mut())[(i as u64)] = __rhs;
            i.prefix_inc();
        }
        ({
            let ___first: __gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___   = ( { (*v.borrow()) . begin_28 ( ) } )   ;
            let ___last: __gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___ =
                ({ (*v.borrow()).end_30() });
            sort_0(___first, ___last, <std_greater_int_>::default())
        });
        ({
            let ___first: __gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___   = ( { (*v.borrow()) . begin_28 ( ) } )   ;
            let ___last: __gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___ =
                ({ (*v.borrow()).end_30() });
            reverse_1(___first, ___last)
        });
        ({
            cpp2rust_assert_fail_2(
                ({
                    let ___first: __gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___   = ( { (*v.borrow()) . begin_28 ( ) } )   ;
                    let ___last: __gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___   = ( { (*v.borrow()) . end_30 ( ) } )   ;
                    is_sorted_3(___first, ___last)
                }),
            )
        });
    }
    {
        let v : Value<std_vector_std_unique_ptr_int__std_default_delete_int____std_allocator_std_unique_ptr_int__std_default_delete_int____ > = Rc::new(RefCell::new(std_vector_std_unique_ptr_int__std_default_delete_int____std_allocator_std_unique_ptr_int__std_default_delete_int____ :: new_1 ( { 1000_u64  } , None , )  )) ;
        let mut i: i32 = 0;
        'loop_: while ((i as u64) < ({ (*v.borrow()).size() })) {
            ({
                let ___p: Ptr<i32> = Ptr::alloc(i);
                (*v.borrow())[(i as u64)].reset(___p)
            });
            i.prefix_inc();
        }
        ({
            let ___first: __gnu_cxx___normal_iterator_std_unique_ptr_int__std_default_delete_int___ptr__std_vector_std_unique_ptr_int__std_default_delete_int____std_allocator_std_unique_ptr_int__std_default_delete_int_____   = ( { (*v.borrow()) . begin_28 ( ) } )   ;
            let ___last: __gnu_cxx___normal_iterator_std_unique_ptr_int__std_default_delete_int___ptr__std_vector_std_unique_ptr_int__std_default_delete_int____std_allocator_std_unique_ptr_int__std_default_delete_int_____   = ( { (*v.borrow()) . end_30 ( ) } )   ;
            sort_4(___first, ___last, <indirect_less>::default())
        });
        ({
            cpp2rust_assert_fail_2(
                ({
                    let ___first: __gnu_cxx___normal_iterator_std_unique_ptr_int__std_default_delete_int___ptr__std_vector_std_unique_ptr_int__std_default_delete_int____std_allocator_std_unique_ptr_int__std_default_delete_int_____   = ( { (*v.borrow()) . begin_28 ( ) } )   ;
                    let ___last: __gnu_cxx___normal_iterator_std_unique_ptr_int__std_default_delete_int___ptr__std_vector_std_unique_ptr_int__std_default_delete_int____std_allocator_std_unique_ptr_int__std_default_delete_int_____   = ( { (*v.borrow()) . end_30 ( ) } )   ;
                    is_sorted_5(___first, ___last, <indirect_less>::default())
                }),
            )
        });
        ({ cpp2rust_assert_fail_2((((*v.borrow())[0_u64].read()) == 0)) });
        ({ cpp2rust_assert_fail_2((((*v.borrow())[1_u64].read()) == 1)) });
        ({ cpp2rust_assert_fail_2((((*v.borrow())[2_u64].read()) == 2)) });
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
