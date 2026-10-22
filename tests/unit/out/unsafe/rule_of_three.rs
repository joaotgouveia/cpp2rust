extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut alive_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut copies_1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe, Default)]
pub struct Buffer {
    pub data: [(); 4],
    pub size: (),
}
impl Buffer {
    pub unsafe fn new(mut size: ()) -> Self {
        let mut this = Self {
            data: [unimplemented!(); 4],
            size: size,
        };
        let mut i: () = 0_();
        'loop_: while ((i) < (4_())) {
            this.data[(i) as usize] = if ((i) < (size)) { i } else { -1_() };
            i.prefix_inc();
        }
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alive_0)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alive_0)).prefix_dec();
    }
    pub unsafe fn copy_from(o: *const Buffer) -> Self {
        let mut this = Self {
            data: [unimplemented!(); 4],
            size: (*o).size,
        };
        let mut i: () = 0_();
        'loop_: while ((i) < (4_())) {
            this.data[(i) as usize] = (*o).data[(i) as usize];
            i.prefix_inc();
        }
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alive_0)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_1)).prefix_inc();
        this
    }
    pub unsafe fn copy_assign(&mut self, o: *const Buffer) -> *mut Buffer {
        if (((self as *mut Buffer).cast_const()) == (o)) {
            return &mut (*(self as *mut Buffer));
        }
        self.size = (*o).size;
        let mut i: () = 0_();
        'loop_: while ((i) < (4_())) {
            self.data[(i) as usize] = (*o).data[(i) as usize];
            i.prefix_inc();
        }
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_1)).prefix_inc();
        return &mut (*(self as *mut Buffer));
    }
}
impl Clone for Buffer {
    fn clone(&self) -> Self {
        unsafe { Buffer::copy_from(self as *const Buffer) }
    }
}
pub unsafe fn sum_2(b: *const Buffer) -> () {
    let mut s: () = 0_();
    let mut i: () = 0_();
    'loop_: while ((i) < ((*b).size)) {
        s += (*b).data[(i) as usize];
        i.prefix_inc();
    }
    return s;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    {
        let mut a: Buffer = Buffer::new({ 4_() });
        let _dtor_a = ScopedDestructorUnsafe::new(&raw mut a, Buffer::destructor);
        let mut b: Buffer = Buffer::copy_from({ &a });
        let _dtor_b = ScopedDestructorUnsafe::new(&raw mut b, Buffer::destructor);
        unimplemented!();
        b.data[(0_()) as usize] = 100_();
        unimplemented!();
        let mut c: Buffer = Buffer::new({ 2_() });
        let _dtor_c = ScopedDestructorUnsafe::new(&raw mut c, Buffer::destructor);
        (unsafe { Buffer::copy_assign(&mut c, &a) });
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _o: *const Buffer = &c;
            Buffer::copy_assign(&mut c, _o)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut d: Buffer = Buffer::copy_from({ &(*unimplemented!()) });
        let _dtor_d = ScopedDestructorUnsafe::new(&raw mut d, Buffer::destructor);
        unimplemented!();
        unimplemented!();
        (unsafe { Buffer::copy_assign(&mut d, &(*unimplemented!())) });
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const alive_0);
    std::cell::LazyCell::force(&*&raw const copies_1);
}
