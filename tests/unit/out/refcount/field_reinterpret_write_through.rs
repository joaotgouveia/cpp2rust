extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct In {
    #[offset(0)]
    pub a: (),
    #[offset(4)]
    pub b: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(40)]
pub struct S {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    #[byte_size(8)]
    pub in_: In,
    #[offset(12)]
    #[byte_size(4)]
    pub bytes: Box<[()]>,
    #[offset(16)]
    #[byte_size(12)]
    pub arr: Box<[()]>,
    #[offset(32)]
    pub tail: (),
}
impl Default for S {
    fn default() -> Self {
        {
            S {
                x: unimplemented!(),
                in_: <In>::default(),
                bytes: (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
                arr: (0..3).map(|_| unimplemented!()).collect::<Box<[()]>>(),
                tail: unimplemented!(),
            }
        }
    }
}
pub fn set_bytes_0(mut p: Ptr<()>, mut n: (), mut v: ()) {
    let mut i: () = 0_();
    'loop_: while (((i < n) as ()) != 0) {
        elem!(p, i).write({ v });
        i.postfix_inc();
    }
}
pub fn all_bytes_1(mut p: Ptr<()>, mut n: (), mut v: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (((i < n) as ()) != 0) {
        if ((({ (elem!(p, i).read()) } != { v }) as ()) != 0) {
            return 0_();
        }
        i.postfix_inc();
    }
    return 1_();
}
pub fn check_final_2(mut s: Ptr<S>) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn check_struct_3(mut s: Ptr<S>) {
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut fb: Ptr<()> = (field_ptr!((s), x)).reinterpret_cast::<()>();
        let mut sb: Ptr<()> = (s).reinterpret_cast::<()>().offset((0_usize) as isize);
        ({
            let _p: Ptr<()> = (fb).clone();
            set_bytes_0(_p, ::std::mem::size_of::<()>(), (16_()))
        });
        unimplemented!();
        unimplemented!();
        field!((s), x)
            .write((((72340172838076673_() as ()).wrapping_mul((((16_()) + 1_()) as ()))) as ()));
        unimplemented!();
        unimplemented!();
        ({
            let _p: Ptr<()> = (sb).clone();
            let _v: () = ((16_()) + 2_());
            set_bytes_0(_p, ::std::mem::size_of::<()>(), _v)
        });
        unimplemented!();
        unimplemented!();
        field!((s), x)
            .write((((72340172838076673_() as ()).wrapping_mul((((16_()) + 3_()) as ()))) as ()));
        unimplemented!();
        unimplemented!();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut fb: Ptr<()> = (field_ptr!(field_ptr!((s), in_), a)).reinterpret_cast::<()>();
        let mut sb: Ptr<()> = (s)
            .reinterpret_cast::<()>()
            .offset(((4_usize as ()).wrapping_add((0_usize as ()))) as isize);
        ({
            let _p: Ptr<()> = (fb).clone();
            set_bytes_0(_p, ::std::mem::size_of::<()>(), (32_()))
        });
        unimplemented!();
        unimplemented!();
        field!(field!((s), in_), a)
            .write((((72340172838076673_() as ()).wrapping_mul((((32_()) + 1_()) as ()))) as ()));
        unimplemented!();
        unimplemented!();
        ({
            let _p: Ptr<()> = (sb).clone();
            let _v: () = ((32_()) + 2_());
            set_bytes_0(_p, ::std::mem::size_of::<()>(), _v)
        });
        unimplemented!();
        unimplemented!();
        field!(field!((s), in_), a)
            .write((((72340172838076673_() as ()).wrapping_mul((((32_()) + 3_()) as ()))) as ()));
        unimplemented!();
        unimplemented!();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut fb: Ptr<()> = (field_ptr!(field_ptr!((s), in_), b)).reinterpret_cast::<()>();
        let mut sb: Ptr<()> = (s)
            .reinterpret_cast::<()>()
            .offset(((4_usize as ()).wrapping_add((4_usize as ()))) as isize);
        ({
            let _p: Ptr<()> = (fb).clone();
            set_bytes_0(_p, ::std::mem::size_of::<()>(), (48_()))
        });
        unimplemented!();
        unimplemented!();
        field!(field!((s), in_), b)
            .write((((72340172838076673_() as ()).wrapping_mul((((48_()) + 1_()) as ()))) as ()));
        unimplemented!();
        unimplemented!();
        ({
            let _p: Ptr<()> = (sb).clone();
            let _v: () = ((48_()) + 2_());
            set_bytes_0(_p, ::std::mem::size_of::<()>(), _v)
        });
        unimplemented!();
        unimplemented!();
        field!(field!((s), in_), b)
            .write((((72340172838076673_() as ()).wrapping_mul((((48_()) + 3_()) as ()))) as ()));
        unimplemented!();
        unimplemented!();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut fb: Ptr<()> = ((array_field_ptr!((s), bytes) as Ptr<()>).offset((0_()) as isize))
            .reinterpret_cast::<()>();
        let mut sb: Ptr<()> = (s)
            .reinterpret_cast::<()>()
            .offset(((12_usize as ()).wrapping_add(0_())) as isize);
        ({
            let _p: Ptr<()> = (fb).clone();
            set_bytes_0(_p, ::std::mem::size_of::<()>(), (64_()))
        });
        unimplemented!();
        unimplemented!();
        array_elem!((s), bytes, 0_())
            .write((((72340172838076673_() as ()).wrapping_mul((((64_()) + 1_()) as ()))) as ()));
        unimplemented!();
        unimplemented!();
        ({
            let _p: Ptr<()> = (sb).clone();
            let _v: () = ((64_()) + 2_());
            set_bytes_0(_p, ::std::mem::size_of::<()>(), _v)
        });
        unimplemented!();
        unimplemented!();
        array_elem!((s), bytes, 0_())
            .write((((72340172838076673_() as ()).wrapping_mul((((64_()) + 3_()) as ()))) as ()));
        unimplemented!();
        unimplemented!();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut fb: Ptr<()> = ((array_field_ptr!((s), bytes) as Ptr<()>).offset((1_()) as isize))
            .reinterpret_cast::<()>();
        let mut sb: Ptr<()> = (s)
            .reinterpret_cast::<()>()
            .offset(((12_usize as ()).wrapping_add(1_())) as isize);
        ({
            let _p: Ptr<()> = (fb).clone();
            set_bytes_0(_p, ::std::mem::size_of::<()>(), (80_()))
        });
        unimplemented!();
        unimplemented!();
        array_elem!((s), bytes, 1_())
            .write((((72340172838076673_() as ()).wrapping_mul((((80_()) + 1_()) as ()))) as ()));
        unimplemented!();
        unimplemented!();
        ({
            let _p: Ptr<()> = (sb).clone();
            let _v: () = ((80_()) + 2_());
            set_bytes_0(_p, ::std::mem::size_of::<()>(), _v)
        });
        unimplemented!();
        unimplemented!();
        array_elem!((s), bytes, 1_())
            .write((((72340172838076673_() as ()).wrapping_mul((((80_()) + 3_()) as ()))) as ()));
        unimplemented!();
        unimplemented!();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut fb: Ptr<()> = ((array_field_ptr!((s), bytes) as Ptr<()>).offset((2_()) as isize))
            .reinterpret_cast::<()>();
        let mut sb: Ptr<()> = (s)
            .reinterpret_cast::<()>()
            .offset(((12_usize as ()).wrapping_add(2_())) as isize);
        ({
            let _p: Ptr<()> = (fb).clone();
            set_bytes_0(_p, ::std::mem::size_of::<()>(), (96_()))
        });
        unimplemented!();
        unimplemented!();
        array_elem!((s), bytes, 2_())
            .write((((72340172838076673_() as ()).wrapping_mul((((96_()) + 1_()) as ()))) as ()));
        unimplemented!();
        unimplemented!();
        ({
            let _p: Ptr<()> = (sb).clone();
            let _v: () = ((96_()) + 2_());
            set_bytes_0(_p, ::std::mem::size_of::<()>(), _v)
        });
        unimplemented!();
        unimplemented!();
        array_elem!((s), bytes, 2_())
            .write((((72340172838076673_() as ()).wrapping_mul((((96_()) + 3_()) as ()))) as ()));
        unimplemented!();
        unimplemented!();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut fb: Ptr<()> = ((array_field_ptr!((s), bytes) as Ptr<()>).offset((3_()) as isize))
            .reinterpret_cast::<()>();
        let mut sb: Ptr<()> = (s)
            .reinterpret_cast::<()>()
            .offset(((12_usize as ()).wrapping_add(3_())) as isize);
        ({
            let _p: Ptr<()> = (fb).clone();
            set_bytes_0(_p, ::std::mem::size_of::<()>(), (112_()))
        });
        unimplemented!();
        unimplemented!();
        array_elem!((s), bytes, 3_())
            .write((((72340172838076673_() as ()).wrapping_mul((((112_()) + 1_()) as ()))) as ()));
        unimplemented!();
        unimplemented!();
        ({
            let _p: Ptr<()> = (sb).clone();
            let _v: () = ((112_()) + 2_());
            set_bytes_0(_p, ::std::mem::size_of::<()>(), _v)
        });
        unimplemented!();
        unimplemented!();
        array_elem!((s), bytes, 3_())
            .write((((72340172838076673_() as ()).wrapping_mul((((112_()) + 3_()) as ()))) as ()));
        unimplemented!();
        unimplemented!();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut fb: Ptr<()> = ((array_field_ptr!((s), arr) as Ptr<()>).offset((0_()) as isize))
            .reinterpret_cast::<()>();
        let mut sb: Ptr<()> = (s).reinterpret_cast::<()>().offset(
            ((16_usize as ())
                .wrapping_add(((0_()).wrapping_mul((::std::mem::size_of::<()>() as ())) as ())))
                as isize,
        );
        ({
            let _p: Ptr<()> = (fb).clone();
            set_bytes_0(_p, ::std::mem::size_of::<()>(), (128_()))
        });
        unimplemented!();
        unimplemented!();
        array_elem!((s), arr, 0_())
            .write((((72340172838076673_() as ()).wrapping_mul((((128_()) + 1_()) as ()))) as ()));
        unimplemented!();
        unimplemented!();
        ({
            let _p: Ptr<()> = (sb).clone();
            let _v: () = ((128_()) + 2_());
            set_bytes_0(_p, ::std::mem::size_of::<()>(), _v)
        });
        unimplemented!();
        unimplemented!();
        array_elem!((s), arr, 0_())
            .write((((72340172838076673_() as ()).wrapping_mul((((128_()) + 3_()) as ()))) as ()));
        unimplemented!();
        unimplemented!();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut fb: Ptr<()> = ((array_field_ptr!((s), arr) as Ptr<()>).offset((1_()) as isize))
            .reinterpret_cast::<()>();
        let mut sb: Ptr<()> = (s).reinterpret_cast::<()>().offset(
            ((16_usize as ())
                .wrapping_add(((1_()).wrapping_mul((::std::mem::size_of::<()>() as ())) as ())))
                as isize,
        );
        ({
            let _p: Ptr<()> = (fb).clone();
            set_bytes_0(_p, ::std::mem::size_of::<()>(), (144_()))
        });
        unimplemented!();
        unimplemented!();
        array_elem!((s), arr, 1_())
            .write((((72340172838076673_() as ()).wrapping_mul((((144_()) + 1_()) as ()))) as ()));
        unimplemented!();
        unimplemented!();
        ({
            let _p: Ptr<()> = (sb).clone();
            let _v: () = ((144_()) + 2_());
            set_bytes_0(_p, ::std::mem::size_of::<()>(), _v)
        });
        unimplemented!();
        unimplemented!();
        array_elem!((s), arr, 1_())
            .write((((72340172838076673_() as ()).wrapping_mul((((144_()) + 3_()) as ()))) as ()));
        unimplemented!();
        unimplemented!();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut fb: Ptr<()> = ((array_field_ptr!((s), arr) as Ptr<()>).offset((2_()) as isize))
            .reinterpret_cast::<()>();
        let mut sb: Ptr<()> = (s).reinterpret_cast::<()>().offset(
            ((16_usize as ())
                .wrapping_add(((2_()).wrapping_mul((::std::mem::size_of::<()>() as ())) as ())))
                as isize,
        );
        ({
            let _p: Ptr<()> = (fb).clone();
            set_bytes_0(_p, ::std::mem::size_of::<()>(), (8_()))
        });
        unimplemented!();
        unimplemented!();
        array_elem!((s), arr, 2_())
            .write((((72340172838076673_() as ()).wrapping_mul((((8_()) + 1_()) as ()))) as ()));
        unimplemented!();
        unimplemented!();
        ({
            let _p: Ptr<()> = (sb).clone();
            let _v: () = ((8_()) + 2_());
            set_bytes_0(_p, ::std::mem::size_of::<()>(), _v)
        });
        unimplemented!();
        unimplemented!();
        array_elem!((s), arr, 2_())
            .write((((72340172838076673_() as ()).wrapping_mul((((8_()) + 3_()) as ()))) as ()));
        unimplemented!();
        unimplemented!();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut fb: Ptr<()> = (field_ptr!((s), tail)).reinterpret_cast::<()>();
        let mut sb: Ptr<()> = (s).reinterpret_cast::<()>().offset((32_usize) as isize);
        ({
            let _p: Ptr<()> = (fb).clone();
            set_bytes_0(_p, ::std::mem::size_of::<()>(), (24_()))
        });
        unimplemented!();
        unimplemented!();
        field!((s), tail)
            .write((((72340172838076673_() as ()).wrapping_mul((((24_()) + 1_()) as ()))) as ()));
        unimplemented!();
        unimplemented!();
        ({
            let _p: Ptr<()> = (sb).clone();
            let _v: () = ((24_()) + 2_());
            set_bytes_0(_p, ::std::mem::size_of::<()>(), _v)
        });
        unimplemented!();
        unimplemented!();
        field!((s), tail)
            .write((((72340172838076673_() as ()).wrapping_mul((((24_()) + 3_()) as ()))) as ()));
        unimplemented!();
        unimplemented!();
    }
    ({ check_final_2((s).clone()) });
    ({ set_bytes_0((field_ptr!(s, in_)).reinterpret_cast::<()>(), 8usize, 33_()) });
    unimplemented!();
    let mut in_: In = In {
        a: (((72340172838076673_() as ()).wrapping_mul(((35_()) as ()))) as ()),
        b: (((72340172838076673_() as ()).wrapping_mul(((51_()) as ()))) as ()),
    };
    field!(s, in_).write((in_).clone());
    unimplemented!();
    unimplemented!();
    ({
        let _p: Ptr<()> = (array_field_ptr!(s, arr) as Ptr<()>).reinterpret_cast::<()>();
        let _n: () = ::std::mem::size_of::<[(); 3]>();
        set_bytes_0(_p, _n, 133_())
    });
    unimplemented!();
    ({
        let _p: Ptr<()> = ((array_field_ptr!(s, arr)) as Ptr<()>).reinterpret_cast::<()>();
        let _n: () = ::std::mem::size_of::<[(); 3]>();
        set_bytes_0(_p, _n, 134_())
    });
    unimplemented!();
    array_elem!(s, arr, 0_())
        .write((((72340172838076673_() as ()).wrapping_mul(((131_()) as ()))) as ()));
    array_elem!(s, arr, 1_())
        .write((((72340172838076673_() as ()).wrapping_mul(((147_()) as ()))) as ()));
    array_elem!(s, arr, 2_())
        .write((((72340172838076673_() as ()).wrapping_mul(((11_()) as ()))) as ()));
    let mut ib: Ptr<()> = (array_field_ptr!(s, bytes) as Ptr<()>).reinterpret_cast::<()>();
    ib.write((((72340172838076673_() as ()).wrapping_mul(((69_()) as ()))) as ()));
    unimplemented!();
    array_elem!(s, bytes, 0_()).write(67_());
    array_elem!(s, bytes, 1_()).write(83_());
    array_elem!(s, bytes, 2_()).write(99_());
    array_elem!(s, bytes, 3_()).write(115_());
    unimplemented!();
    let mut v: () = ((field_ptr!(field_ptr!(s, in_), b)) as Ptr<()>).to_any();
    v.reinterpret_cast::<()>().write(11_());
    unimplemented!();
    let twelve: Value<()> = Rc::new(RefCell::new(12_()));
    ({
        memcpy_4(
            ((field_ptr!(s, x)) as Ptr<()>).to_any(),
            ((twelve.as_pointer()) as Ptr<()>).to_any(),
            ::std::mem::size_of::<()>(),
        )
    });
    unimplemented!();
    ({
        memset_5(
            ((field_ptr!(field_ptr!(s, in_), b)) as Ptr<()>).to_any(),
            51_(),
            ::std::mem::size_of::<()>(),
        )
    });
    ({
        memset_5(
            ((field_ptr!(s, x)) as Ptr<()>).to_any(),
            19_(),
            ::std::mem::size_of::<()>(),
        )
    });
    ({ check_final_2((s).clone()) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let local: Value<S> = <Value<S>>::default();
    ({
        let ___s: () = ((local.as_pointer()) as Ptr<S>).to_any();
        let ___n: () = 40usize;
        memset_5(___s, 0_(), ___n)
    });
    ({ check_struct_3((local.as_pointer())) });
    let arr: Value<Box<[S]>> = Rc::new(RefCell::new(
        (0..2).map(|_| <S>::default()).collect::<Box<[S]>>(),
    ));
    ({
        let ___s: () = ((arr.as_pointer() as Ptr<S>) as Ptr<S>).to_any();
        let ___n: () = 80usize;
        memset_5(___s, 0_(), ___n)
    });
    ({ check_struct_3(((arr.as_pointer() as Ptr<S>).offset(1_()))) });
    unimplemented!();
    let mut heap: Ptr<S> = unimplemented!().reinterpret_cast::<S>();
    unimplemented!();
    ({ check_struct_3((heap).clone()) });
    unimplemented!();
    let mut raw_: Ptr<()> = unimplemented!().reinterpret_cast::<()>();
    unimplemented!();
    ({ memset_5((raw_).to_any(), 0_(), 40usize) });
    let mut view: Ptr<S> = raw_.reinterpret_cast::<S>();
    ({ check_struct_3((view).clone()) });
    unimplemented!();
    ({
        let _p: Ptr<()> = raw_.offset((16_usize) as isize);
        set_bytes_0(_p, ::std::mem::size_of::<()>(), 119_())
    });
    unimplemented!();
    array_elem!(view, bytes, 3_()).write(17_());
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
