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
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_1: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_2: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_3: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub fn operator_eq_4(
    mut _a0: min_allocator_std_pair_const_int__double__,
    mut _a1: min_allocator_std_pair_const_int__double__,
) -> () {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_pair_const_int__double__ {}
impl std::cmp::PartialEq for min_allocator_std_pair_const_int__double__ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_4(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_std_pair_const_int__double__ {}
pub fn test_5() -> () {
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            {
                let __tmp_0: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_1: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_0.as_pointer() }, {
                    __tmp_1.as_pointer()
                })
            },
            {
                let __tmp_2: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_3: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_2.as_pointer() }, {
                    __tmp_3.as_pointer()
                })
            },
            {
                let __tmp_4: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_5: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_4.as_pointer() }, {
                    __tmp_5.as_pointer()
                })
            },
            {
                let __tmp_6: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_7: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_6.as_pointer() }, {
                    __tmp_7.as_pointer()
                })
            },
            {
                let __tmp_8: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_9: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_8.as_pointer() }, {
                    __tmp_9.as_pointer()
                })
            },
            {
                let __tmp_10: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_11: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_10.as_pointer() }, {
                    __tmp_11.as_pointer()
                })
            },
            {
                let __tmp_12: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_13: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_12.as_pointer() }, {
                    __tmp_13.as_pointer()
                })
            },
            {
                let __tmp_14: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_15: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_14.as_pointer() }, {
                    __tmp_15.as_pointer()
                })
            },
            {
                let __tmp_16: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_17: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_16.as_pointer() }, {
                    __tmp_17.as_pointer()
                })
            },
        ])));
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            {
                let __tmp_18: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_19: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_18.as_pointer() }, {
                    __tmp_19.as_pointer()
                })
            },
            {
                let __tmp_20: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_21: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_20.as_pointer() }, {
                    __tmp_21.as_pointer()
                })
            },
            {
                let __tmp_22: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_23: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_22.as_pointer() }, {
                    __tmp_23.as_pointer()
                })
            },
            {
                let __tmp_24: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_25: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_24.as_pointer() }, {
                    __tmp_25.as_pointer()
                })
            },
            {
                let __tmp_26: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_27: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_26.as_pointer() }, {
                    __tmp_27.as_pointer()
                })
            },
            {
                let __tmp_28: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_29: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_28.as_pointer() }, {
                    __tmp_29.as_pointer()
                })
            },
            {
                let __tmp_30: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_31: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_30.as_pointer() }, {
                    __tmp_31.as_pointer()
                })
            },
            {
                let __tmp_32: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_33: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_32.as_pointer() }, {
                    __tmp_33.as_pointer()
                })
            },
            {
                let __tmp_34: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_35: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_34.as_pointer() }, {
                    __tmp_35.as_pointer()
                })
            },
        ])));
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            {
                let __tmp_36: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_37: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_36.as_pointer() }, {
                    __tmp_37.as_pointer()
                })
            },
            {
                let __tmp_38: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_39: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_38.as_pointer() }, {
                    __tmp_39.as_pointer()
                })
            },
            {
                let __tmp_40: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_41: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_40.as_pointer() }, {
                    __tmp_41.as_pointer()
                })
            },
            {
                let __tmp_42: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_43: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_42.as_pointer() }, {
                    __tmp_43.as_pointer()
                })
            },
            {
                let __tmp_44: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_45: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_44.as_pointer() }, {
                    __tmp_45.as_pointer()
                })
            },
            {
                let __tmp_46: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_47: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_46.as_pointer() }, {
                    __tmp_47.as_pointer()
                })
            },
            {
                let __tmp_48: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_49: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_48.as_pointer() }, {
                    __tmp_49.as_pointer()
                })
            },
            {
                let __tmp_50: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_51: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_50.as_pointer() }, {
                    __tmp_51.as_pointer()
                })
            },
            {
                let __tmp_52: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_53: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_52.as_pointer() }, {
                    __tmp_53.as_pointer()
                })
            },
        ])));
        let mut a: min_allocator_std_pair_const_int__double__ =
            <min_allocator_std_pair_const_int__double__>::default();
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return true;
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_5() });
    return 0_();
}
pub trait min_allocator_std_pair_const_int__double__Impl {
    fn allocate(
        &self,
        mut n: (),
    ) -> min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_std_pair_const_int__double__Impl
    for Ptr<min_allocator_std_pair_const_int__double__>
{
}
pub trait min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
