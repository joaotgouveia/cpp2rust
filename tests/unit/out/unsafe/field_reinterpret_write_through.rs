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
pub struct In {
    pub a: (),
    pub b: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub x: (),
    pub in_: In,
    pub bytes: [(); 4],
    pub arr: [(); 3],
    pub tail: (),
}
pub unsafe fn set_bytes_0(mut p: *mut (), mut n: (), mut v: ()) {
    let mut i: () = 0_();
    'loop_: while ((((i) < (n)) as ()) != 0) {
        (*p.offset((i) as isize)) = v;
        i.postfix_inc();
    }
}
pub unsafe fn all_bytes_1(mut p: *const (), mut n: (), mut v: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((((i) < (n)) as ()) != 0) {
        if ((((*p.offset((i) as isize)) != (v)) as ()) != 0) {
            return 0_();
        }
        i.postfix_inc();
    }
    return 1_();
}
pub unsafe fn check_final_2(mut s: *mut S) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn check_struct_3(mut s: *mut S) {
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut fb: *mut () = ((&mut (*(s)).x as *mut ()) as *mut ());
        let mut sb: *mut () = ((s) as *mut ()).offset((::std::mem::offset_of!(S, x)) as isize);
        (unsafe {
            let _p: *mut () = fb;
            set_bytes_0(_p, ::std::mem::size_of::<()>(), (16_()))
        });
        unimplemented!();
        unimplemented!();
        (*(s)).x = (((72340172838076673_() as ()).wrapping_mul((((16_()) + (1_())) as ()))) as ());
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _p: *mut () = sb;
            let _v: () = ((16_()) + (2_()));
            set_bytes_0(_p, ::std::mem::size_of::<()>(), _v)
        });
        unimplemented!();
        unimplemented!();
        (*(s)).x = (((72340172838076673_() as ()).wrapping_mul((((16_()) + (3_())) as ()))) as ());
        unimplemented!();
        unimplemented!();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut fb: *mut () = ((&mut (*(s)).in_.a as *mut ()) as *mut ());
        let mut sb: *mut () = ((s) as *mut ()).offset(
            ((::std::mem::offset_of!(S, in_) as ())
                .wrapping_add((::std::mem::offset_of!(In, a) as ()))) as isize,
        );
        (unsafe {
            let _p: *mut () = fb;
            set_bytes_0(_p, ::std::mem::size_of::<()>(), (32_()))
        });
        unimplemented!();
        unimplemented!();
        (*(s)).in_.a =
            (((72340172838076673_() as ()).wrapping_mul((((32_()) + (1_())) as ()))) as ());
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _p: *mut () = sb;
            let _v: () = ((32_()) + (2_()));
            set_bytes_0(_p, ::std::mem::size_of::<()>(), _v)
        });
        unimplemented!();
        unimplemented!();
        (*(s)).in_.a =
            (((72340172838076673_() as ()).wrapping_mul((((32_()) + (3_())) as ()))) as ());
        unimplemented!();
        unimplemented!();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut fb: *mut () = ((&mut (*(s)).in_.b as *mut ()) as *mut ());
        let mut sb: *mut () = ((s) as *mut ()).offset(
            ((::std::mem::offset_of!(S, in_) as ())
                .wrapping_add((::std::mem::offset_of!(In, b) as ()))) as isize,
        );
        (unsafe {
            let _p: *mut () = fb;
            set_bytes_0(_p, ::std::mem::size_of::<()>(), (48_()))
        });
        unimplemented!();
        unimplemented!();
        (*(s)).in_.b =
            (((72340172838076673_() as ()).wrapping_mul((((48_()) + (1_())) as ()))) as ());
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _p: *mut () = sb;
            let _v: () = ((48_()) + (2_()));
            set_bytes_0(_p, ::std::mem::size_of::<()>(), _v)
        });
        unimplemented!();
        unimplemented!();
        (*(s)).in_.b =
            (((72340172838076673_() as ()).wrapping_mul((((48_()) + (3_())) as ()))) as ());
        unimplemented!();
        unimplemented!();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut fb: *mut () = (&mut (*(s)).bytes[(0_()) as usize] as *mut ());
        let mut sb: *mut () = ((s) as *mut ())
            .offset(((::std::mem::offset_of!(S, bytes) as ()).wrapping_add(0_())) as isize);
        (unsafe {
            let _p: *mut () = fb;
            set_bytes_0(_p, ::std::mem::size_of::<()>(), (64_()))
        });
        unimplemented!();
        unimplemented!();
        (*(s)).bytes[(0_()) as usize] =
            (((72340172838076673_() as ()).wrapping_mul((((64_()) + (1_())) as ()))) as ());
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _p: *mut () = sb;
            let _v: () = ((64_()) + (2_()));
            set_bytes_0(_p, ::std::mem::size_of::<()>(), _v)
        });
        unimplemented!();
        unimplemented!();
        (*(s)).bytes[(0_()) as usize] =
            (((72340172838076673_() as ()).wrapping_mul((((64_()) + (3_())) as ()))) as ());
        unimplemented!();
        unimplemented!();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut fb: *mut () = (&mut (*(s)).bytes[(1_()) as usize] as *mut ());
        let mut sb: *mut () = ((s) as *mut ())
            .offset(((::std::mem::offset_of!(S, bytes) as ()).wrapping_add(1_())) as isize);
        (unsafe {
            let _p: *mut () = fb;
            set_bytes_0(_p, ::std::mem::size_of::<()>(), (80_()))
        });
        unimplemented!();
        unimplemented!();
        (*(s)).bytes[(1_()) as usize] =
            (((72340172838076673_() as ()).wrapping_mul((((80_()) + (1_())) as ()))) as ());
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _p: *mut () = sb;
            let _v: () = ((80_()) + (2_()));
            set_bytes_0(_p, ::std::mem::size_of::<()>(), _v)
        });
        unimplemented!();
        unimplemented!();
        (*(s)).bytes[(1_()) as usize] =
            (((72340172838076673_() as ()).wrapping_mul((((80_()) + (3_())) as ()))) as ());
        unimplemented!();
        unimplemented!();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut fb: *mut () = (&mut (*(s)).bytes[(2_()) as usize] as *mut ());
        let mut sb: *mut () = ((s) as *mut ())
            .offset(((::std::mem::offset_of!(S, bytes) as ()).wrapping_add(2_())) as isize);
        (unsafe {
            let _p: *mut () = fb;
            set_bytes_0(_p, ::std::mem::size_of::<()>(), (96_()))
        });
        unimplemented!();
        unimplemented!();
        (*(s)).bytes[(2_()) as usize] =
            (((72340172838076673_() as ()).wrapping_mul((((96_()) + (1_())) as ()))) as ());
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _p: *mut () = sb;
            let _v: () = ((96_()) + (2_()));
            set_bytes_0(_p, ::std::mem::size_of::<()>(), _v)
        });
        unimplemented!();
        unimplemented!();
        (*(s)).bytes[(2_()) as usize] =
            (((72340172838076673_() as ()).wrapping_mul((((96_()) + (3_())) as ()))) as ());
        unimplemented!();
        unimplemented!();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut fb: *mut () = (&mut (*(s)).bytes[(3_()) as usize] as *mut ());
        let mut sb: *mut () = ((s) as *mut ())
            .offset(((::std::mem::offset_of!(S, bytes) as ()).wrapping_add(3_())) as isize);
        (unsafe {
            let _p: *mut () = fb;
            set_bytes_0(_p, ::std::mem::size_of::<()>(), (112_()))
        });
        unimplemented!();
        unimplemented!();
        (*(s)).bytes[(3_()) as usize] =
            (((72340172838076673_() as ()).wrapping_mul((((112_()) + (1_())) as ()))) as ());
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _p: *mut () = sb;
            let _v: () = ((112_()) + (2_()));
            set_bytes_0(_p, ::std::mem::size_of::<()>(), _v)
        });
        unimplemented!();
        unimplemented!();
        (*(s)).bytes[(3_()) as usize] =
            (((72340172838076673_() as ()).wrapping_mul((((112_()) + (3_())) as ()))) as ());
        unimplemented!();
        unimplemented!();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut fb: *mut () = ((&mut (*(s)).arr[(0_()) as usize] as *mut ()) as *mut () as *mut ());
        let mut sb: *mut () = ((s) as *mut ()).offset(
            ((::std::mem::offset_of!(S, arr) as ())
                .wrapping_add(((0_()).wrapping_mul((::std::mem::size_of::<()>() as ())) as ())))
                as isize,
        );
        (unsafe {
            let _p: *mut () = fb;
            set_bytes_0(_p, ::std::mem::size_of::<()>(), (128_()))
        });
        unimplemented!();
        unimplemented!();
        (*(s)).arr[(0_()) as usize] =
            (((72340172838076673_() as ()).wrapping_mul((((128_()) + (1_())) as ()))) as ());
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _p: *mut () = sb;
            let _v: () = ((128_()) + (2_()));
            set_bytes_0(_p, ::std::mem::size_of::<()>(), _v)
        });
        unimplemented!();
        unimplemented!();
        (*(s)).arr[(0_()) as usize] =
            (((72340172838076673_() as ()).wrapping_mul((((128_()) + (3_())) as ()))) as ());
        unimplemented!();
        unimplemented!();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut fb: *mut () = ((&mut (*(s)).arr[(1_()) as usize] as *mut ()) as *mut () as *mut ());
        let mut sb: *mut () = ((s) as *mut ()).offset(
            ((::std::mem::offset_of!(S, arr) as ())
                .wrapping_add(((1_()).wrapping_mul((::std::mem::size_of::<()>() as ())) as ())))
                as isize,
        );
        (unsafe {
            let _p: *mut () = fb;
            set_bytes_0(_p, ::std::mem::size_of::<()>(), (144_()))
        });
        unimplemented!();
        unimplemented!();
        (*(s)).arr[(1_()) as usize] =
            (((72340172838076673_() as ()).wrapping_mul((((144_()) + (1_())) as ()))) as ());
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _p: *mut () = sb;
            let _v: () = ((144_()) + (2_()));
            set_bytes_0(_p, ::std::mem::size_of::<()>(), _v)
        });
        unimplemented!();
        unimplemented!();
        (*(s)).arr[(1_()) as usize] =
            (((72340172838076673_() as ()).wrapping_mul((((144_()) + (3_())) as ()))) as ());
        unimplemented!();
        unimplemented!();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut fb: *mut () = ((&mut (*(s)).arr[(2_()) as usize] as *mut ()) as *mut () as *mut ());
        let mut sb: *mut () = ((s) as *mut ()).offset(
            ((::std::mem::offset_of!(S, arr) as ())
                .wrapping_add(((2_()).wrapping_mul((::std::mem::size_of::<()>() as ())) as ())))
                as isize,
        );
        (unsafe {
            let _p: *mut () = fb;
            set_bytes_0(_p, ::std::mem::size_of::<()>(), (8_()))
        });
        unimplemented!();
        unimplemented!();
        (*(s)).arr[(2_()) as usize] =
            (((72340172838076673_() as ()).wrapping_mul((((8_()) + (1_())) as ()))) as ());
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _p: *mut () = sb;
            let _v: () = ((8_()) + (2_()));
            set_bytes_0(_p, ::std::mem::size_of::<()>(), _v)
        });
        unimplemented!();
        unimplemented!();
        (*(s)).arr[(2_()) as usize] =
            (((72340172838076673_() as ()).wrapping_mul((((8_()) + (3_())) as ()))) as ());
        unimplemented!();
        unimplemented!();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut fb: *mut () = ((&mut (*(s)).tail as *mut ()) as *mut ());
        let mut sb: *mut () = ((s) as *mut ()).offset((::std::mem::offset_of!(S, tail)) as isize);
        (unsafe {
            let _p: *mut () = fb;
            set_bytes_0(_p, ::std::mem::size_of::<()>(), (24_()))
        });
        unimplemented!();
        unimplemented!();
        (*(s)).tail =
            (((72340172838076673_() as ()).wrapping_mul((((24_()) + (1_())) as ()))) as ());
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _p: *mut () = sb;
            let _v: () = ((24_()) + (2_()));
            set_bytes_0(_p, ::std::mem::size_of::<()>(), _v)
        });
        unimplemented!();
        unimplemented!();
        (*(s)).tail =
            (((72340172838076673_() as ()).wrapping_mul((((24_()) + (3_())) as ()))) as ());
        unimplemented!();
        unimplemented!();
    }
    (unsafe { check_final_2(s) });
    (unsafe {
        set_bytes_0(
            ((&mut (*s).in_ as *mut In) as *mut ()),
            ::std::mem::size_of::<In>(),
            33_(),
        )
    });
    unimplemented!();
    let mut in_: In = In {
        a: (((72340172838076673_() as ()).wrapping_mul(((35_()) as ()))) as ()),
        b: (((72340172838076673_() as ()).wrapping_mul(((51_()) as ()))) as ()),
    };
    (*s).in_ = in_;
    unimplemented!();
    unimplemented!();
    (unsafe {
        let _p: *mut () = ((*s).arr.as_mut_ptr() as *mut ());
        let _n: () = ::std::mem::size_of::<[(); 3]>();
        set_bytes_0(_p, _n, 133_())
    });
    unimplemented!();
    (unsafe {
        let _p: *mut () = ((&mut (*s).arr as *mut [(); 3]) as *mut ());
        let _n: () = ::std::mem::size_of::<[(); 3]>();
        set_bytes_0(_p, _n, 134_())
    });
    unimplemented!();
    (*s).arr[(0_()) as usize] =
        (((72340172838076673_() as ()).wrapping_mul(((131_()) as ()))) as ());
    (*s).arr[(1_()) as usize] =
        (((72340172838076673_() as ()).wrapping_mul(((147_()) as ()))) as ());
    (*s).arr[(2_()) as usize] =
        (((72340172838076673_() as ()).wrapping_mul(((11_()) as ()))) as ());
    let mut ib: *mut () = ((*s).bytes.as_mut_ptr() as *mut ());
    (*ib) = (((72340172838076673_() as ()).wrapping_mul(((69_()) as ()))) as ());
    unimplemented!();
    (*s).bytes[(0_()) as usize] = 67_();
    (*s).bytes[(1_()) as usize] = 83_();
    (*s).bytes[(2_()) as usize] = 99_();
    (*s).bytes[(3_()) as usize] = 115_();
    unimplemented!();
    let mut v: () = ((&mut (*s).in_.b as *mut ()) as ());
    (*(v as *mut ())) = 11_();
    unimplemented!();
    let mut twelve: () = 12_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (unsafe { check_final_2(s) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut local: S = <S>::default();
    unimplemented!();
    (unsafe { check_struct_3((&mut local as *mut S)) });
    let mut arr: [S; 2] = [<S>::default(); 2];
    unimplemented!();
    (unsafe { check_struct_3((&mut arr[(1_()) as usize] as *mut S)) });
    unimplemented!();
    let mut heap: *mut S = (unimplemented!() as *mut S);
    unimplemented!();
    (unsafe { check_struct_3(heap) });
    unimplemented!();
    let mut raw_: *mut () = (unimplemented!() as *mut ());
    unimplemented!();
    unimplemented!();
    let mut view: *mut S = (raw_ as *mut S);
    (unsafe { check_struct_3(view) });
    unimplemented!();
    (unsafe {
        let _p: *mut () = raw_.offset((::std::mem::offset_of!(S, arr)) as isize);
        set_bytes_0(_p, ::std::mem::size_of::<()>(), 119_())
    });
    unimplemented!();
    (*view).bytes[(3_()) as usize] = 17_();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
