extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(mut x: ()) -> () {
    return x;
}
pub unsafe fn ptr_1(mut x: *mut ()) -> *mut () {
    return x;
}
pub unsafe fn bar_2(x: *mut ()) -> *mut () {
    return x;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct X1 {
    pub v: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct X2 {
    pub v: *mut X1,
}
impl X2 {
    pub unsafe fn get(&mut self) -> *mut X1 {
        return self.v;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct X3 {
    pub v: *mut X2,
}
impl X3 {
    pub unsafe fn get(&mut self) -> *mut X2 {
        return self.v;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct X4 {
    pub v: X3,
}
impl X4 {
    pub unsafe fn get(&mut self) -> *mut X3 {
        return &mut self.v;
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut x1: () = unimplemented!();
    let mut x2: () = (unsafe { foo_0(x1) });
    let mut x3: () = (((unsafe { foo_0(x2) }) + (unsafe { foo_0(x1) })) + (1_()));
    x2 += 1_();
    x2 += (unsafe { foo_0(x1) });
    x3 += (((unsafe { foo_0(x2) }) + (unsafe { foo_0(x3) })) + (1_()));
    let mut p1: *mut () = (&mut x1 as *mut ());
    let mut p2: *mut () = (unsafe { ptr_1(p1) });
    p1 = p2;
    p2 = (unsafe { ptr_1(p1) });
    let r1: *mut () = &mut x1;
    let r2: *mut () = (unsafe { bar_2(&mut x1) });
    let r3: *mut () = (unsafe { bar_2(r1) });
    (*r2) += x1;
    (*r3) += (*r1);
    let mut x4: () = (((unsafe { foo_0(x3) }) + (*(unsafe { ptr_1((&mut x3 as *mut ())) })))
        + (*(unsafe { bar_2(&mut x2) })));
    let mut a: X1 = X1 {
        v: unimplemented!(),
    };
    let mut b: X2 = X2 { v: &mut a };
    let mut c: X3 = X3 {
        v: (&mut b as *mut X2),
    };
    let mut d: X4 = X4 { v: c };
    (*(*d.v.v).v).v = unimplemented!();
    (*(unsafe { X2::get(&mut (*(unsafe { X3::get(&mut (*(unsafe { X4::get(&mut d) }))) }))) })).v =
        unimplemented!();
    d.v.v = (&mut b as *mut X2);
    let r4: *const () = &(*(unsafe {
        X2::get(&mut (*(unsafe { X3::get(&mut (*(unsafe { X4::get(&mut d) }))) })))
    }))
    .v;
    let r5: *mut X1 =
        (unsafe { X2::get(&mut (*(unsafe { X3::get(&mut (*(unsafe { X4::get(&mut d) }))) }))) });
    let mut p: *mut X2 = (unsafe { X3::get(&mut (*(unsafe { X4::get(&mut d) }))) });
    let r6: *mut X3 = (unsafe { X4::get(&mut d) });
    let r7: *mut X3 = &mut d.v;
    let r8: *mut () = &mut (*(unsafe { X2::get(&mut (*(unsafe { X3::get(&mut d.v) }))) })).v;
    let mut x5: () =
        (*(unsafe { X2::get(&mut (*(unsafe { X3::get(&mut (*(unsafe { X4::get(&mut d) }))) }))) }))
            .v;
    (*(unsafe { bar_2(&mut x1) })) += 10_();
    (*(unsafe { bar_2(&mut x1) })).postfix_inc();
    let mut bar_out: () = (*(unsafe {
        bar_2(
            &mut (*(unsafe {
                X2::get(&mut (*(unsafe { X3::get(&mut (*(unsafe { X4::get(&mut d) }))) })))
            }))
            .v,
        )
    }));
    let mut bar_inc: () = (*(unsafe { bar_2(&mut x1) })).prefix_inc();
    bar_inc = (*(unsafe { bar_2(&mut x1) })).postfix_inc();
    bar_inc = (((*(unsafe { bar_2(&mut x1) })) + (unsafe { foo_0(x4) })) + (1_()));
    (*(unsafe {
        bar_2(
            &mut (*(unsafe {
                X2::get(&mut (*(unsafe { X3::get(&mut (*(unsafe { X4::get(&mut d) }))) })))
            }))
            .v,
        )
    })) += 10_();
    (*(unsafe {
        bar_2(
            &mut (*(unsafe {
                X2::get(&mut (*(unsafe { X3::get(&mut (*(unsafe { X4::get(&mut d) }))) })))
            }))
            .v,
        )
    }))
    .postfix_inc();
    let mut bar_inc2: () = (*(unsafe {
        bar_2(
            &mut (*(unsafe {
                X2::get(&mut (*(unsafe { X3::get(&mut (*(unsafe { X4::get(&mut d) }))) })))
            }))
            .v,
        )
    }))
    .prefix_inc();
    bar_inc2 = (*(unsafe {
        bar_2(
            &mut (*(unsafe {
                X2::get(&mut (*(unsafe { X3::get(&mut (*(unsafe { X4::get(&mut d) }))) })))
            }))
            .v,
        )
    }))
    .postfix_inc();
    (*(unsafe { ptr_1((&mut x1 as *mut ())) })).prefix_inc();
    (*(unsafe { ptr_1((&mut x1 as *mut ())) })) += 1_();
    (*(unsafe {
        ptr_1(
            (&mut (*(unsafe {
                X2::get(&mut (*(unsafe { X3::get(&mut (*(unsafe { X4::get(&mut d) }))) })))
            }))
            .v as *mut ()),
        )
    }))
    .prefix_inc();
    (*(unsafe {
        ptr_1(
            (&mut (*(unsafe {
                X2::get(&mut (*(unsafe { X3::get(&mut (*(unsafe { X4::get(&mut d) }))) })))
            }))
            .v as *mut ()),
        )
    })) += 1_();
    (*(&mut (*(unsafe {
        ptr_1(
            (&mut (*(unsafe {
                X2::get(&mut (*(unsafe { X3::get(&mut (*(unsafe { X4::get(&mut d) }))) })))
            }))
            .v as *mut ()),
        )
    })) as *mut ())) += 1_();
    let mut ptr1: () = (*(unsafe {
        ptr_1(
            (&mut (*(unsafe {
                X2::get(&mut (*(unsafe { X3::get(&mut (*(unsafe { X4::get(&mut d) }))) })))
            }))
            .v as *mut ()),
        )
    }))
    .postfix_inc();
    let ptr2: *mut () = &mut (*(unsafe {
        ptr_1(
            (&mut (*(unsafe {
                X2::get(&mut (*(unsafe { X3::get(&mut (*(unsafe { X4::get(&mut d) }))) })))
            }))
            .v as *mut ()),
        )
    }));
    let mut ptr3: *mut () = (&mut (*(unsafe {
        ptr_1(
            (&mut (*(unsafe {
                X2::get(&mut (*(unsafe { X3::get(&mut (*(unsafe { X4::get(&mut d) }))) })))
            }))
            .v as *mut ()),
        )
    })) as *mut ());
    let mut vptr: () = (*(unsafe {
        ptr_1(
            (&mut (*(unsafe {
                X2::get(&mut (*(unsafe { X3::get(&mut (*(unsafe { X4::get(&mut d) }))) })))
            }))
            .v as *mut ()),
        )
    }));
    let mut pref: *mut () = (unsafe {
        bar_2(
            &mut (*(unsafe {
                X2::get(&mut (*(unsafe { X3::get(&mut (*(unsafe { X4::get(&mut d) }))) })))
            }))
            .v,
        )
    });
    (*(unsafe {
        bar_2(
            &mut (*(unsafe {
                X2::get(&mut (*(unsafe { X3::get(&mut (*(unsafe { X4::get(&mut d) }))) })))
            }))
            .v,
        )
    }))
    .postfix_inc();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
