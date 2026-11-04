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
pub struct N_X {}
impl CppMove for N_X {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<N_X>> = Rc::new(RefCell::new(
        (0..(5_usize) as usize)
            .map(|_| <N_X>::default())
            .collect::<Vec<_>>(),
    ));
    let w: Value<Vec<N_X>> = Rc::new(RefCell::new(
        (0..(1_usize) as usize)
            .map(|_| <N_X>::default())
            .collect::<Vec<_>>(),
    ));
    &(v.decay().offset(0_usize as isize));
    &(if 0_usize as usize >= (*w.as_pointer().upgrade().deref()).len() {
        panic!("out of bounds access")
    } else {
        w.as_pointer().decay().offset(0_usize as isize)
    });
    &((*v.borrow()).len());
    &((*v.borrow()).capacity());
    {
        let __a0 = 1_usize as usize;
        (*v.borrow_mut()).resize_with(__a0, || <N_X>::default())
    };
    match ({
        <__gnu_cxx___normal_iterator___gnu_cxx__Pointer_adapter___gnu_cxx__Relative_pointer_impl_const_N_X____std_vector_N_X____gnu_cxx__ExtPtr_allocator_N_X___  >::from(CppRange::begin(&v .decay() )  )
    },)
    {
        (mut __tmp_0,) => {
            let __off = __tmp_0.index();
            (*v.borrow_mut()).insert(__off, <N_X>::default());
            (__tmp_0).clone()
        }
    };
    match ({
        <__gnu_cxx___normal_iterator___gnu_cxx__Pointer_adapter___gnu_cxx__Relative_pointer_impl_const_N_X____std_vector_N_X____gnu_cxx__ExtPtr_allocator_N_X___  >::from(CppRange::begin(&v .decay() )  )
    },)
    {
        (mut __tmp_1,) => {
            let __off = __tmp_1.index();
            (*v.borrow_mut()).splice(__off..__off, std::iter::repeat_n(<N_X>::default(), 1_usize));
            (__tmp_1).clone()
        }
    };
    match (
        {
            <__gnu_cxx___normal_iterator___gnu_cxx__Pointer_adapter___gnu_cxx__Relative_pointer_impl_const_N_X____std_vector_N_X____gnu_cxx__ExtPtr_allocator_N_X___  >::from(CppRange::begin(&v .decay() )  )
        },
        { CppRange::begin(&w.decay()) },
        { CppRange::end(&w.decay()) },
    ) {
        (mut __tmp_2, mut __tmp_3, mut __tmp_4) => {
            let __start = __tmp_2.index();
            let mut __tmp: Vec<N_X> = Vec::new();
            let mut __it = (__tmp_3).clone();
            while __it != __tmp_4 {
                __tmp.push(<N_X as CppFrom<N_X>>::cpp_from(__it.iter_deref()));
                __it.iter_inc();
            }
            v.as_pointer().with_mut(|__v: &mut Vec<N_X>| {
                __v.splice(__start..__start, __tmp);
            });
            (__tmp_2).clone()
        }
    };
    v.as_pointer().write(({ (*w.borrow()).clone() }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
