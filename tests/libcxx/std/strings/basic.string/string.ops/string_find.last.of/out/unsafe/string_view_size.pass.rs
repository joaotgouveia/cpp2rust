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
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_3)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_0)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)) = 0_();
    }
}
pub static mut outstanding_bytes_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_char__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_char__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
        return self.ptr_;
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.ptr_.offset((n) as isize));
    }
    pub unsafe fn pointer_to(
        t: *mut (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    ptr_: *const (),
}
impl min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *const ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn new_2(mut p: min_pointer_char__std_integral_constant_unsigned_long__0__) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *const () {
        return self.ptr_;
    }
    pub unsafe fn pointer_to(
        t: *const (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_char_ {}
impl min_allocator_char_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_char__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub unsafe fn test_4(s: *const (), mut sv: (), mut pos: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if ((x) != (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5))) {
        unimplemented!();
    }
}
pub unsafe fn test_6(s: *const (), mut sv: (), mut pos: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if ((x) != (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7))) {
        unimplemented!();
    }
}
pub unsafe fn test_8(s: *const (), mut sv: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if ((x) != (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5))) {
        unimplemented!();
    }
}
pub unsafe fn test_9(s: *const (), mut sv: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if ((x) != (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7))) {
        unimplemented!();
    }
}
pub unsafe fn test0_10() {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"fodgq".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"qanej".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"clbao".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"mekdn".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"srdfq".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"oemth".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"cdaih".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"qohtk".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"cshmd".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"lhcdo".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"qnsoh".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 2_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"pkrof".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 2_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"fmtsp".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            4_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"khbpm".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 4_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"pbsji".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 4_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"mprdj".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 4_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"eqmpa".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"omigs".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 5_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"onmje".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 5_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"oqmrj".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 5_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"schfa".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            6_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"igdsc".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 6_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"brqgo".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 6_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"tnrph".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 6_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"hcjitbfapl".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"daiprenocl".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"litpcfdghe".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"aidjksrolc".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"qpghtfbaji".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"gfshlcmdjr".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"nkodajteqp".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"gbmetiprqd".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"crnklpmegd".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"jsbtafedoc".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"qnmodrtkeb".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"cpebqsfmnj".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"lmofqdhpki".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            9_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"hnefkqimca".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 9_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"drtasbgmfp".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 9_(), 7_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"lsaijeqhtr".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 9_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"elgofjmbrq".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"mjqdgalkpc".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 10_(), 8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"kthqnfcerm".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 10_(), 6_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"dfsjhanorc".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 10_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"eqsgalomhb".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            11_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"akiteljmoh".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 11_(), 8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"hlbdfreqjo".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 11_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"taqobhlerg".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 11_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"snafbdlghrjkpqtoceim".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"aemtbrgcklhndjisfpoq".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"pnracgfkjdiholtbqsem".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"dicfltehbsgrmojnpkaq".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"jlnkraeodhcspfgbqitm".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"lhosrngtmfjikbqpcade".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"rbtaqjhgkneisldpmfoc".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"gpifsqlrdkbonjtmheca".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"hdpkobnsalmcfijregtq".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"jtlshdgqaiprkbcoenfm".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"fkdrbqltsgmcoiphneaj".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"crsplifgtqedjohnabmk".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"niptglfbosehkamrdqcj".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"copqdhstbingamjfkler".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 19_(), 16_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"mrtaefilpdsgocnhqbjk".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 19_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"kojatdhlcmigpbfrqnes".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 19_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"eaintpchlqsbdgrkjofm".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"gjnhidfsepkrtaqbmclo".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 20_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"spocfaktqdbiejlhngmr".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 20_(), 18_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"rphmlekgfscndtaobiqj".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 20_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"liatsqdoegkmfcnbhrpj".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            unimplemented!(),
            21_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"binjagtfldkrspcomqeh".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 21_(), 12_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"latkmisecnorjbfhqpdg".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 21_(), 17_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"lecfratdjkhnsmqpoigb".as_ptr() },
            None,
        );
        test_4(&mut _s, unimplemented!(), 21_(), 19_())
    });
}
pub unsafe fn test0_11() {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"fodgq".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"qanej".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"clbao".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"mekdn".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"srdfq".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"oemth".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"cdaih".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"qohtk".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"cshmd".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"lhcdo".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"qnsoh".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 2_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"pkrof".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 2_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"fmtsp".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            4_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"khbpm".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 4_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"pbsji".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 4_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"mprdj".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 4_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"eqmpa".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"omigs".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 5_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"onmje".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 5_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"oqmrj".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 5_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"schfa".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            6_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"igdsc".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 6_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"brqgo".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 6_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"tnrph".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 6_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"hcjitbfapl".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"daiprenocl".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"litpcfdghe".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"aidjksrolc".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"qpghtfbaji".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"gfshlcmdjr".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"nkodajteqp".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"gbmetiprqd".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"crnklpmegd".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"jsbtafedoc".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"qnmodrtkeb".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"cpebqsfmnj".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"lmofqdhpki".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            9_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"hnefkqimca".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 9_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"drtasbgmfp".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 9_(), 7_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"lsaijeqhtr".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 9_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"elgofjmbrq".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"mjqdgalkpc".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 10_(), 8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"kthqnfcerm".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 10_(), 6_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"dfsjhanorc".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 10_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"eqsgalomhb".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            11_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"akiteljmoh".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 11_(), 8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"hlbdfreqjo".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 11_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"taqobhlerg".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 11_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"snafbdlghrjkpqtoceim".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"aemtbrgcklhndjisfpoq".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"pnracgfkjdiholtbqsem".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"dicfltehbsgrmojnpkaq".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"jlnkraeodhcspfgbqitm".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"lhosrngtmfjikbqpcade".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"rbtaqjhgkneisldpmfoc".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"gpifsqlrdkbonjtmheca".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"hdpkobnsalmcfijregtq".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"jtlshdgqaiprkbcoenfm".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"fkdrbqltsgmcoiphneaj".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"crsplifgtqedjohnabmk".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"niptglfbosehkamrdqcj".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"copqdhstbingamjfkler".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 19_(), 16_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"mrtaefilpdsgocnhqbjk".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 19_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"kojatdhlcmigpbfrqnes".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 19_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"eaintpchlqsbdgrkjofm".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"gjnhidfsepkrtaqbmclo".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 20_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"spocfaktqdbiejlhngmr".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 20_(), 18_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"rphmlekgfscndtaobiqj".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 20_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"liatsqdoegkmfcnbhrpj".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            unimplemented!(),
            21_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"binjagtfldkrspcomqeh".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 21_(), 12_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"latkmisecnorjbfhqpdg".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 21_(), 17_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"lecfratdjkhnsmqpoigb".as_ptr() },
            None,
        );
        test_6(&mut _s, unimplemented!(), 21_(), 19_())
    });
}
pub unsafe fn test1_12() {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_8(
            &mut _s,
            unimplemented!(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_8(
            &mut _s,
            unimplemented!(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_8(
            &mut _s,
            unimplemented!(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_8(
            &mut _s,
            unimplemented!(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"nhmko".as_ptr() },
            None,
        );
        test_8(
            &mut _s,
            unimplemented!(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"lahfb".as_ptr() },
            None,
        );
        test_8(&mut _s, unimplemented!(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"gmfhd".as_ptr() },
            None,
        );
        test_8(&mut _s, unimplemented!(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"odaft".as_ptr() },
            None,
        );
        test_8(&mut _s, unimplemented!(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"eolhfgpjqk".as_ptr() },
            None,
        );
        test_8(
            &mut _s,
            unimplemented!(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"nbatdlmekr".as_ptr() },
            None,
        );
        test_8(&mut _s, unimplemented!(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"jdmciepkaq".as_ptr() },
            None,
        );
        test_8(&mut _s, unimplemented!(), 8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"hkbgspoflt".as_ptr() },
            None,
        );
        test_8(&mut _s, unimplemented!(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"gprdcokbnjhlsfmtieqa".as_ptr() },
            None,
        );
        test_8(
            &mut _s,
            unimplemented!(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"qjghlnftcaismkropdeb".as_ptr() },
            None,
        );
        test_8(&mut _s, unimplemented!(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"pnalfrdtkqcmojiesbhg".as_ptr() },
            None,
        );
        test_8(&mut _s, unimplemented!(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"pniotcfrhqsmgdkjbael".as_ptr() },
            None,
        );
        test_8(&mut _s, unimplemented!(), 19_())
    });
}
pub unsafe fn test1_13() {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_9(
            &mut _s,
            unimplemented!(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_9(
            &mut _s,
            unimplemented!(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_9(
            &mut _s,
            unimplemented!(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_9(
            &mut _s,
            unimplemented!(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"nhmko".as_ptr() },
            None,
        );
        test_9(
            &mut _s,
            unimplemented!(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"lahfb".as_ptr() },
            None,
        );
        test_9(&mut _s, unimplemented!(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"gmfhd".as_ptr() },
            None,
        );
        test_9(&mut _s, unimplemented!(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"odaft".as_ptr() },
            None,
        );
        test_9(&mut _s, unimplemented!(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"eolhfgpjqk".as_ptr() },
            None,
        );
        test_9(
            &mut _s,
            unimplemented!(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"nbatdlmekr".as_ptr() },
            None,
        );
        test_9(&mut _s, unimplemented!(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"jdmciepkaq".as_ptr() },
            None,
        );
        test_9(&mut _s, unimplemented!(), 8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"hkbgspoflt".as_ptr() },
            None,
        );
        test_9(&mut _s, unimplemented!(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"gprdcokbnjhlsfmtieqa".as_ptr() },
            None,
        );
        test_9(
            &mut _s,
            unimplemented!(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"qjghlnftcaismkropdeb".as_ptr() },
            None,
        );
        test_9(&mut _s, unimplemented!(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"pnalfrdtkqcmojiesbhg".as_ptr() },
            None,
        );
        test_9(&mut _s, unimplemented!(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"pniotcfrhqsmgdkjbael".as_ptr() },
            None,
        );
        test_9(&mut _s, unimplemented!(), 19_())
    });
}
pub unsafe fn test_string_14() {
    (unsafe { test0_10() });
    (unsafe { test1_12() });
}
pub unsafe fn test_string_15() {
    (unsafe { test0_11() });
    (unsafe { test1_13() });
}
pub unsafe fn test_16() -> () {
    (unsafe { test_string_14() });
    (unsafe { test_string_15() });
    return true;
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_16() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
