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
pub static mut moves_2: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(
    VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe, DestructorUnsafe, Default,
)]
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
    pub unsafe fn move_from(o: *mut Buffer) -> Self {
        let mut this = Self {
            data: [unimplemented!(); 4],
            size: (*o).size,
        };
        let mut i: () = 0_();
        'loop_: while ((i) < (4_())) {
            this.data[(i) as usize] = (*o).data[(i) as usize];
            (*o).data[(i) as usize] = -1_();
            i.prefix_inc();
        }
        (*o).size = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alive_0)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_2)).prefix_inc();
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
    pub unsafe fn move_assign(&mut self, o: *mut Buffer) -> *mut Buffer {
        if ((self as *mut Buffer) == (o)) {
            return &mut (*(self as *mut Buffer));
        }
        self.size = (*o).size;
        let mut i: () = 0_();
        'loop_: while ((i) < (4_())) {
            self.data[(i) as usize] = (*o).data[(i) as usize];
            (*o).data[(i) as usize] = -1_();
            i.prefix_inc();
        }
        (*o).size = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_2)).prefix_inc();
        return &mut (*(self as *mut Buffer));
    }
}
impl Clone for Buffer {
    fn clone(&self) -> Self {
        unsafe { Buffer::copy_from(self as *const Buffer) }
    }
}
pub unsafe fn make_3(mut size: ()) -> Buffer {
    let mut b: Buffer = Buffer::new({ size });
    let _dtor_b = ScopedDestructorUnsafe::new(&raw mut b, Buffer::destructor);
    return Buffer::move_from({ &mut (*unimplemented!()) });
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
        let mut c: Buffer = Buffer::move_from({ &mut (*unimplemented!()) });
        let _dtor_c = ScopedDestructorUnsafe::new(&raw mut c, Buffer::destructor);
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut d: Buffer = (unsafe { make_3(2_()) });
        let _dtor_d = ScopedDestructorUnsafe::new(&raw mut d, Buffer::destructor);
        unimplemented!();
        (unsafe { Buffer::copy_assign(&mut d, &b) });
        unimplemented!();
        (unsafe { Buffer::move_assign(&mut d, &mut (*unimplemented!())) });
        unimplemented!();
        (unsafe {
            let _o: *mut Buffer = &mut (*unimplemented!());
            Buffer::move_assign(&mut d, _o)
        });
        unimplemented!();
    }
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const alive_0);
    std::cell::LazyCell::force(&*&raw const copies_1);
    std::cell::LazyCell::force(&*&raw const moves_2);
}
