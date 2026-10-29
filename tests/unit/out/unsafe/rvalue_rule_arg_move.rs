extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut copies_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut moves_1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, Default)]
pub struct Elem {
    pub v: (),
}
impl Elem {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn copy_from(o: *const Elem) -> Self {
        let mut this = Self { v: (*o).v };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        this
    }
    pub unsafe fn move_from(o: *mut Elem) -> Self {
        let mut this = Self { v: (*o).v };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)).prefix_inc();
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Elem) -> *mut Elem {
        self.v = (*_a0).v;
        return &mut (*(self as *mut Elem));
    }
}
impl Clone for Elem {
    fn clone(&self) -> Self {
        unsafe { Elem::copy_from(self as *const Elem) }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut v: std_vector_Elem__std_allocator_Elem__ =
        <std_vector_Elem__std_allocator_Elem__>::default();
    (unsafe { v.reserve(4_()) });
    let mut a: Elem = Elem::new({ 1_() });
    let mut b: Elem = Elem::new({ 2_() });
    (unsafe {
        let ___position: __gnu_cxx___normal_iterator_const_Elem_ptr__std_vector_Elem__std_allocator_Elem___  = { let mut __tmp_0 : __gnu_cxx___normal_iterator_Elem_ptr__std_vector_Elem__std_allocator_Elem___  = ( unsafe { v . end_28  ( ) } )  ; __gnu_cxx___normal_iterator_const_Elem_ptr__std_vector_Elem__std_allocator_Elem___ :: new_1 ( { & mut __tmp_0 } , )  } ;
        v.insert_63(___position, &mut a)
    });
    (unsafe {
        let ___position: __gnu_cxx___normal_iterator_const_Elem_ptr__std_vector_Elem__std_allocator_Elem___  = { let mut __tmp_1 : __gnu_cxx___normal_iterator_Elem_ptr__std_vector_Elem__std_allocator_Elem___  = ( unsafe { v . end_28  ( ) } )  ; __gnu_cxx___normal_iterator_const_Elem_ptr__std_vector_Elem__std_allocator_Elem___ :: new_1 ( { & mut __tmp_1 } , )  } ;
        v.insert_62(___position, &b)
    });
    let mut ok: () = ((((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (1_()))
        && ((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)) == (1_())))
        && ((v[(0_()) as usize].v) == (1_())))
        && ((v[(1_()) as usize].v) == (2_()));
    let mut pa: *mut Elem = (&mut a as *mut Elem);
    unimplemented!();
    return if ok { 0_() } else { 1_() };
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copies_0);
    std::cell::LazyCell::force(&*&raw const moves_1);
}
