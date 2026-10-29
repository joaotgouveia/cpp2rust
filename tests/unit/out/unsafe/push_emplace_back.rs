extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Chunk {
    pub data: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Writer {
    pub output: *mut std_vector_Chunk__std_allocator_Chunk__,
    pub chunk: Chunk,
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct JPEGData { pub com_data : std_vector_std_vector_unsigned_char__std_allocator_unsigned_char____std_allocator_std_vector_unsigned_char__std_allocator_unsigned_char____ , pub app_data : std_vector_std_vector_unsigned_char__std_allocator_unsigned_char____std_allocator_std_vector_unsigned_char__std_allocator_unsigned_char____ , }
impl JPEGData {
    pub unsafe fn move_from(_a0: *mut JPEGData) -> Self {
        let mut this = Self {
            com_data: (*_a0).com_data,
            app_data: (*_a0).app_data,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut JPEGData) -> *mut JPEGData {
        self.com_data = ((*_a0).com_data).clone();
        self.app_data = ((*_a0).app_data).clone();
        return &mut (*(self as *mut JPEGData));
    }
}
pub unsafe fn push_param_0(
    mut dest : *mut std_vector_std_vector_unsigned_char__std_allocator_unsigned_char____std_allocator_std_vector_unsigned_char__std_allocator_unsigned_char____,
) {
    (unsafe {
        let mut ___x: std_vector_unsigned_char__std_allocator_unsigned_char__ =
            <std_vector_unsigned_char__std_allocator_unsigned_char__>::default();
        (*dest).push_back_58(&mut ___x)
    });
}
pub unsafe fn push_local_from_field_1(mut jpg: *mut JPEGData, mut cond: ()) {
    let mut head: [(); 3] = [1_(), 2_(), 3_()];
    let mut dest : *mut std_vector_std_vector_unsigned_char__std_allocator_unsigned_char____std_allocator_std_vector_unsigned_char__std_allocator_unsigned_char____ = std::ptr::null_mut() ;
    if cond {
        dest  = ( & mut ( * jpg  ) . com_data  as *mut std_vector_std_vector_unsigned_char__std_allocator_unsigned_char____std_allocator_std_vector_unsigned_char__std_allocator_unsigned_char____ )  ;
    } else {
        dest  = ( & mut ( * jpg  ) . app_data  as *mut std_vector_std_vector_unsigned_char__std_allocator_unsigned_char____std_allocator_std_vector_unsigned_char__std_allocator_unsigned_char____ )  ;
    }
    (unsafe {
        let mut ___x: std_vector_unsigned_char__std_allocator_unsigned_char__ =
            std_vector_unsigned_char__std_allocator_unsigned_char__::new_1(
                { head.as_mut_ptr() },
                { head.as_mut_ptr().offset((3_()) as isize) },
                None,
            );
        (*dest).push_back_58(&mut ___x)
    });
}
pub unsafe fn shrink_through_ptr_2(mut comps: *mut std_vector_Chunk__std_allocator_Chunk__) {
    (unsafe { (*comps).shrink_to_fit() });
}
pub unsafe fn nested_push_move_3(mut bw: *mut Writer) {
    (unsafe {
        let ___x: *mut Chunk = &mut (*bw).chunk;
        (*(*bw).output).push_back_58(___x)
    });
}
pub unsafe fn emplace_local_from_field_4(mut jpg: *mut JPEGData, mut cond: ()) {
    let mut head: [(); 3] = [1_(), 2_(), 3_()];
    let mut dest : *mut std_vector_std_vector_unsigned_char__std_allocator_unsigned_char____std_allocator_std_vector_unsigned_char__std_allocator_unsigned_char____ = std::ptr::null_mut() ;
    if cond {
        dest  = ( & mut ( * jpg  ) . com_data  as *mut std_vector_std_vector_unsigned_char__std_allocator_unsigned_char____std_allocator_std_vector_unsigned_char__std_allocator_unsigned_char____ )  ;
    } else {
        dest  = ( & mut ( * jpg  ) . app_data  as *mut std_vector_std_vector_unsigned_char__std_allocator_unsigned_char____std_allocator_std_vector_unsigned_char__std_allocator_unsigned_char____ )  ;
    }
    (unsafe {
        let ___args_0: *mut [(); 3] = &mut head;
        let mut ___args_1: *mut () = head.as_mut_ptr().offset((3_()) as isize);
        (*dest).emplace_back(___args_0, &mut ___args_1)
    });
}
pub unsafe fn nested_emplace_move_5(mut bw: *mut Writer) {
    (unsafe {
        let ___args: *mut Chunk = &mut (*bw).chunk;
        (*(*bw).output).emplace_back(___args)
    });
}
pub unsafe fn self_ref_push_6(mut comps: *mut std_vector_Chunk__std_allocator_Chunk__) {
    (unsafe {
        let ___x: *const Chunk = (unsafe { (*comps).front_51() });
        (*comps).push_back_57(___x)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Pair {
    pub first: (),
    pub second: (),
}
impl Pair {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            first: -1_(),
            second: -1_(),
        };
        this
    }
    pub unsafe fn new_2(mut a: ()) -> Self {
        let mut this = Self {
            first: a,
            second: 0_(),
        };
        this
    }
    pub unsafe fn new_3(mut a: (), mut b: ()) -> Self {
        let mut this = Self {
            first: a,
            second: ((b) * (2_())),
        };
        this
    }
}
impl Default for Pair {
    fn default() -> Self {
        unsafe { Pair::new_1() }
    }
}
pub unsafe fn emplace_ctor_args_7(mut pairs: *mut std_vector_Pair__std_allocator_Pair__) {
    (unsafe { (*pairs).emplace_back() });
    (unsafe {
        let mut ___args: () = 3_();
        (*pairs).emplace_back(&mut ___args)
    });
    (unsafe {
        let mut ___args_0: () = 4_();
        let mut ___args_1: () = 5_();
        (*pairs).emplace_back(&mut ___args_0, &mut ___args_1)
    });
}
pub unsafe fn emplace_deque_8(mut queue: *mut std_deque_Pair__std_allocator_Pair__) {
    (unsafe {
        let mut ___args_0: () = 6_();
        let mut ___args_1: () = 7_();
        (*queue).emplace_back(&mut ___args_0, &mut ___args_1)
    });
    (unsafe { (*queue).emplace_back() });
}
pub unsafe fn emplace_scalar_9(mut values: *mut std_vector_long__std_allocator_long__, mut x: ()) {
    (unsafe { (*values).emplace_back() });
    (unsafe { (*values).emplace_back(&mut x) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut vecs : std_vector_std_vector_unsigned_char__std_allocator_unsigned_char____std_allocator_std_vector_unsigned_char__std_allocator_unsigned_char____ = <std_vector_std_vector_unsigned_char__std_allocator_unsigned_char____std_allocator_std_vector_unsigned_char__std_allocator_unsigned_char____ >::default() ;
    (unsafe {
        push_param_0 ( ( & mut vecs as *mut std_vector_std_vector_unsigned_char__std_allocator_unsigned_char____std_allocator_std_vector_unsigned_char__std_allocator_unsigned_char____ ) , )
    });
    unimplemented!();
    unimplemented!();
    let mut jpg: JPEGData = <JPEGData>::default();
    (unsafe { push_local_from_field_1((&mut jpg as *mut JPEGData), true) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut chunks: std_vector_Chunk__std_allocator_Chunk__ =
        <std_vector_Chunk__std_allocator_Chunk__>::default();
    (unsafe {
        shrink_through_ptr_2((&mut chunks as *mut std_vector_Chunk__std_allocator_Chunk__))
    });
    unimplemented!();
    let mut w: Writer = <Writer>::default();
    w.chunk.data = 42_();
    w.output = (&mut chunks as *mut std_vector_Chunk__std_allocator_Chunk__);
    (unsafe { nested_push_move_3((&mut w as *mut Writer)) });
    unimplemented!();
    unimplemented!();
    (unsafe { emplace_local_from_field_4((&mut jpg as *mut JPEGData), false) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    w.chunk.data = 99_();
    w.output = (&mut chunks as *mut std_vector_Chunk__std_allocator_Chunk__);
    (unsafe { nested_emplace_move_5((&mut w as *mut Writer)) });
    unimplemented!();
    unimplemented!();
    (unsafe { self_ref_push_6((&mut chunks as *mut std_vector_Chunk__std_allocator_Chunk__)) });
    unimplemented!();
    unimplemented!();
    let mut pairs: std_vector_Pair__std_allocator_Pair__ =
        <std_vector_Pair__std_allocator_Pair__>::default();
    (unsafe { emplace_ctor_args_7((&mut pairs as *mut std_vector_Pair__std_allocator_Pair__)) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut queue: std_deque_Pair__std_allocator_Pair__ =
        <std_deque_Pair__std_allocator_Pair__>::default();
    (unsafe { emplace_deque_8((&mut queue as *mut std_deque_Pair__std_allocator_Pair__)) });
    unimplemented!();
    unimplemented!();
    let mut values: std_vector_long__std_allocator_long__ =
        <std_vector_long__std_allocator_long__>::default();
    (unsafe {
        emplace_scalar_9(
            (&mut values as *mut std_vector_long__std_allocator_long__),
            7_(),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
