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
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_1.with(|rc| *rc.borrow()) >= dealloc_count_2.with(|rc| *rc.borrow())));
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_1: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_2: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_3: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_pair_const_int__int__ {}
impl CppMove for min_allocator_std_pair_const_int__int__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}pub fn node_factory_4 ( c : Ptr< Map<i32  , (Value<i32  >, Value<i32  >)>  > , key : Ptr< i32  > , mapped : Ptr< i32  > , ) -> std__Node_handle_int__std_pair_const_int__int___std_allocator_std__Rb_tree_node_std_pair_const_int__int____{
    let p: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        ({
            let ___x: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new(
                std_pair_const_int__int_::new_1({ (key).clone() }, { (mapped).clone() }),
            ));
            (*c.upgrade().deref()).insert_54(___x.as_pointer())
        }),
    ));
    assert!(({ ((*(*p.borrow()).1.borrow()) as bool) }));
    return ({
        let ___pos: BidirectionalIterator<(Value<i32>, Value<i32>)> =
            <BidirectionalIterator<(Value<i32>, Value<i32>)>>::from(
                ({ (*(*p.borrow()).0.borrow()).clone() }),
            );
        (*c.upgrade().deref()).extract_41(___pos)
    });
}pub fn node_factory_5 ( c : Ptr< Map<i32  , (Value<i32  >, Value<i32  >)>  > , key : Ptr< i32  > , mapped : Ptr< i32  > , ) -> std__Node_handle_int__std_pair_const_int__int___min_allocator_std___rb_tree__Node_min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0_____{
    let p: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        ({
            let ___x: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new(
                std_pair_const_int__int_::new_1({ (key).clone() }, { (mapped).clone() }),
            ));
            (*c.upgrade().deref()).insert_54(___x.as_pointer())
        }),
    ));
    assert!(({ ((*(*p.borrow()).1.borrow()) as bool) }));
    return ({
        let ___pos: BidirectionalIterator<(Value<i32>, Value<i32>)> =
            <BidirectionalIterator<(Value<i32>, Value<i32>)>>::from(
                ({ (*(*p.borrow()).0.borrow()).clone() }),
            );
        (*c.upgrade().deref()).extract_41(___pos)
    });
}
pub fn test_6(c: Ptr<Map<i32, (Value<i32>, Value<i32>)>>) {
    let mut nf : FnPtr<fn(Ptr< Map<i32  , (Value<i32  >, Value<i32  >)>  > ,Ptr< i32  > ,Ptr< i32  > ,) -> std__Node_handle_int__std_pair_const_int__int___std_allocator_std__Rb_tree_node_std_pair_const_int__int____  >  = ( FnPtr::<fn(Ptr< Map<i32  , (Value<i32  >, Value<i32  >)>  > ,Ptr< i32  > ,Ptr< i32  > ,) -> std__Node_handle_int__std_pair_const_int__int___std_allocator_std__Rb_tree_node_std_pair_const_int__int____  >::new(node_factory_4)  )  ;
    let c2: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) != 10) {
        let node : Value<std__Node_handle_int__std_pair_const_int__int___std_allocator_std__Rb_tree_node_std_pair_const_int__int____ > = Rc::new(RefCell::new(( { let _arg1: Ptr< i32  >  = i .as_pointer()  ; let _arg2 : Value<i32 > = Rc::new(RefCell::new(( (*i.borrow()) + 1 ) )); nf .call ( c2 .as_pointer()  , _arg1 , _arg2.as_pointer() , ) } )  )) ;
        assert!(
            ({
                !({
                    ( (*node.borrow()) as std__Node_handle_common_std_pair_const_int__int___std_allocator_std__Rb_tree_node_std_pair_const_int__int____ ) . empty ( )
                })
            })
        );
        let mut prev: usize = (*c.upgrade().deref()).len();
        let it: Value<BidirectionalIterator<(Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(
            ({
                let ___hint: BidirectionalIterator<(Value<i32>, Value<i32>)> =
                    <BidirectionalIterator<(Value<i32>, Value<i32>)>>::from(
                        c.with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end()),
                    );
                let ___nh: Ptr< std__Node_handle_int__std_pair_const_int__int___std_allocator_std__Rb_tree_node_std_pair_const_int__int____  >  = node .as_pointer()  ;
                (*c.upgrade().deref()).insert_44(___hint, ___nh)
            }),
        ));
        assert!(
            ({
                ({
                    ( (*node.borrow()) as std__Node_handle_common_std_pair_const_int__int___std_allocator_std__Rb_tree_node_std_pair_const_int__int____ ) . empty ( )
                })
            })
        );
        assert!(({ ({ (prev).wrapping_add(1_usize) } == { (*c.upgrade().deref()).len() }) }));
        assert!(
            ({ ((*(*(*it.borrow()).iter_deref().upgrade().deref()).0.borrow()) == (*i.borrow())) })
        );
        assert!(
            ({
                ((*(*(*it.borrow()).iter_deref().upgrade().deref()).1.borrow())
                    == ((*i.borrow()) + 1))
            })
        );
        (*i.borrow_mut()).prefix_inc();
    }
    assert!(({ ((*c.upgrade().deref()).len() == 10_usize) }));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) != 10) {
        assert!(
            ({
                (({
                    let ___x: Ptr<i32> = i.as_pointer();
                    (*c.upgrade().deref()).count(___x)
                }) == 1_usize)
            })
        );
        assert!(
            ({
                ({ (Map::subscript((c).clone(), i.as_pointer()).read()) } == {
                    ((*i.borrow()) + 1)
                })
            })
        );
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn test_7(c: Ptr<Map<i32, (Value<i32>, Value<i32>)>>) {
    let mut nf : FnPtr<fn(Ptr< Map<i32  , (Value<i32  >, Value<i32  >)>  > ,Ptr< i32  > ,Ptr< i32  > ,) -> std__Node_handle_int__std_pair_const_int__int___min_allocator_std___rb_tree__Node_min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0_____  >  = ( FnPtr::<fn(Ptr< Map<i32  , (Value<i32  >, Value<i32  >)>  > ,Ptr< i32  > ,Ptr< i32  > ,) -> std__Node_handle_int__std_pair_const_int__int___min_allocator_std___rb_tree__Node_min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0_____  >::new(node_factory_5)  )  ;
    let c2: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) != 10) {
        let node : Value<std__Node_handle_int__std_pair_const_int__int___min_allocator_std___rb_tree__Node_min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0_____ > = Rc::new(RefCell::new(( { let _arg1: Ptr< i32  >  = i .as_pointer()  ; let _arg2 : Value<i32 > = Rc::new(RefCell::new(( (*i.borrow()) + 1 ) )); nf .call ( c2 .as_pointer()  , _arg1 , _arg2.as_pointer() , ) } )  )) ;
        assert!(
            ({
                !({
                    ( (*node.borrow()) as std__Node_handle_common_std_pair_const_int__int___min_allocator_std___rb_tree__Node_min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0_____ ) . empty ( )
                })
            })
        );
        let mut prev: usize = (*c.upgrade().deref()).len();
        let it: Value<BidirectionalIterator<(Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(
            ({
                let ___hint: BidirectionalIterator<(Value<i32>, Value<i32>)> =
                    <BidirectionalIterator<(Value<i32>, Value<i32>)>>::from(
                        c.with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end()),
                    );
                let ___nh: Ptr< std__Node_handle_int__std_pair_const_int__int___min_allocator_std___rb_tree__Node_min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0_____  >  = node .as_pointer()  ;
                (*c.upgrade().deref()).insert_44(___hint, ___nh)
            }),
        ));
        assert!(
            ({
                ({
                    ( (*node.borrow()) as std__Node_handle_common_std_pair_const_int__int___min_allocator_std___rb_tree__Node_min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0_____ ) . empty ( )
                })
            })
        );
        assert!(({ ({ (prev).wrapping_add(1_usize) } == { (*c.upgrade().deref()).len() }) }));
        assert!(
            ({ ((*(*(*it.borrow()).iter_deref().upgrade().deref()).0.borrow()) == (*i.borrow())) })
        );
        assert!(
            ({
                ((*(*(*it.borrow()).iter_deref().upgrade().deref()).1.borrow())
                    == ((*i.borrow()) + 1))
            })
        );
        (*i.borrow_mut()).prefix_inc();
    }
    assert!(({ ((*c.upgrade().deref()).len() == 10_usize) }));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) != 10) {
        assert!(
            ({
                (({
                    let ___x: Ptr<i32> = i.as_pointer();
                    (*c.upgrade().deref()).count(___x)
                }) == 1_usize)
            })
        );
        assert!(
            ({
                ({ (Map::subscript((c).clone(), i.as_pointer()).read()) } == {
                    ((*i.borrow()) + 1)
                })
            })
        );
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn test_8() -> bool {
    let m: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    ({ test_6(m.as_pointer()) });
    let m2: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    ({ test_7(m2.as_pointer()) });
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_8() });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__;
pub trait min_allocator_std_pair_const_int__int__Impl {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_std_pair_const_int__int__Impl for Ptr<min_allocator_std_pair_const_int__int__> {}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
