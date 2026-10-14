use crate::reinterpret::OriginalAlloc;
use crate::{Ptr, Root, Value};
use std::cell::{Ref, RefMut};
use std::rc::Weak;

/// Allows extending Ptr through the shim.rs files. Required methods have no default,
/// opt-in methods panic by default and are overridden by seams that support them.
pub trait PtrSeam<T> {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn address(&self) -> usize;
    fn upgrade(&self, offset: usize) -> Box<dyn StrongPtrSeam<T>>;

    /// Opt-in, enables Ptr::reinterpret_cast.
    fn as_original_alloc(&self, _offset: usize) -> OriginalAlloc {
        panic!("reinterpret_cast is not supported by this seam")
    }

    /// Opt-in, enables Ptr::sort_with_cmp.
    fn sort_with_cmp(
        &self,
        _from: usize,
        _last: usize,
        _cmp: &mut dyn FnMut(Ptr<T>, Ptr<T>) -> bool,
    ) {
        panic!("sort_with_cmp is not supported by this seam")
    }

    /// Opt-in, enables Ptr::sort.
    fn sort(&self, _from: usize, _last: usize)
    where
        T: Ord,
    {
        panic!("sort is not supported by this seam")
    }

    /// Opt-in, enables Ptr<T: CChar>::with_slice.
    fn with_slice(&self, _offset: usize, _len: usize, _f: &mut dyn FnMut(&[T])) {
        panic!("with_slice is not supported by this seam")
    }

    /// Opt-in, enables Ptr<T: CChar>::with_slice_mut.
    fn with_slice_mut(&self, _offset: usize, _len: usize, _f: &mut dyn FnMut(&mut [T])) {
        panic!("with_slice_mut is not supported by this seam")
    }

    /// Opt-in, enables Ptr::delete.
    fn delete(&self, _offset: usize) {
        panic!("delete is not supported by this seam")
    }

    /// Opt-in: enables Ptr::to_owned_opt.
    fn to_owned_single(&self, _offset: usize) -> Option<Value<T>> {
        panic!("to_owned_single is not supported by this seam")
    }

    /// Opt-in: enables Ptr::to_owned_opt.
    fn to_owned_array(&self, _offset: usize) -> Option<Value<Box<[T]>>> {
        panic!("to_owned_array is not supported by this seam")
    }

    /// Opt-in: enables Ptr::decay
    fn decay(&self, _offset: usize) -> Ptr<T::Decayed>
    where
        T: DecayResult,
    {
        panic!("decay is not supported by this seam")
    }

    /// Opt-in: override Ptr<T: CChar>::with_c_str, returning false causes the default
    /// element-by-element scan to be used
    fn with_c_str(&self, _offset: usize, _f: &mut dyn FnMut(&[T])) -> bool {
        false
    }

    /// Opt-in: enables field_ptr! on seam pointers. The returned root must
    /// address the seam's elements contiguously, so that element `i` lies at
    /// byte `i * T::byte_size()` within it (e.g. a `Weak<RefCell<Vec<T>>>` or
    /// `Weak<RefCell<Box<[T]>>>`, which implement `Root` when `T: Record`).
    fn field_root(&self) -> Weak<dyn Root> {
        panic!("field pointers are not supported by this seam")
    }
}

pub trait DecayResult {
    type Decayed;
}

impl<D> DecayResult for Vec<D> {
    type Decayed = D;
}

impl<D> DecayResult for Box<[D]> {
    type Decayed = D;
}

pub trait StrongPtrSeam<T> {
    fn deref(&self) -> Ref<'_, T>;
    fn deref_mut(&self) -> RefMut<'_, T>;
}

pub trait PtrDynSeam<T: ?Sized> {
    fn upgrade(&self, offset: usize) -> Box<dyn StrongPtrDynSeam<T>>;
}

impl<T: ?Sized> std::fmt::Debug for dyn PtrDynSeam<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Seam")
    }
}

pub trait StrongPtrDynSeam<T: ?Sized> {
    fn deref(&self) -> Ref<'_, T>;
    fn deref_mut(&self) -> RefMut<'_, T>;
}

#[cfg(test)]
mod tests {
    use crate::{AsPointer, ByteRepr, Ptr, PtrSeam, Record, Root, StrongPtrSeam, Value};
    use std::cell::{Ref, RefCell, RefMut};
    use std::rc::{Rc, Weak};

    struct Seg<T>(Vec<T>);

    impl<T: 'static> PtrSeam<T> for Weak<RefCell<Seg<T>>> {
        fn len(&self) -> usize {
            Weak::upgrade(self)
                .expect("ub: dangling pointer")
                .borrow()
                .0
                .len()
        }

        fn is_empty(&self) -> bool {
            Weak::upgrade(self)
                .expect("ub: dangling pointer")
                .borrow()
                .0
                .is_empty()
        }

        fn address(&self) -> usize {
            self.as_ptr() as usize
        }

        fn upgrade(&self, offset: usize) -> Box<dyn StrongPtrSeam<T>> {
            Box::new((Weak::upgrade(self).expect("ub: dangling pointer"), offset))
        }
    }

    impl<T> StrongPtrSeam<T> for (Rc<RefCell<Seg<T>>>, usize) {
        fn deref(&self) -> Ref<'_, T> {
            let i = self.1;
            Ref::map(self.0.borrow(), move |c| &c.0[i])
        }

        fn deref_mut(&self) -> RefMut<'_, T> {
            let i = self.1;
            RefMut::map(self.0.borrow_mut(), move |c| &mut c.0[i])
        }
    }

    impl<T: 'static> AsPointer<T> for Rc<RefCell<Seg<T>>> {
        fn as_pointer(&self) -> Ptr<T> {
            Ptr::seam(Rc::new(Rc::downgrade(self)))
        }
    }

    fn seg(v: Vec<i32>) -> Value<Seg<i32>> {
        Rc::new(RefCell::new(Seg(v)))
    }

    #[derive(Default, Record, ByteRepr)]
    #[byte_size(8)]
    struct Pair {
        #[offset(0)]
        a: i32,
        #[offset(4)]
        b: i32,
    }

    impl PtrSeam<Pair> for Weak<RefCell<Vec<Pair>>> {
        fn len(&self) -> usize {
            Weak::upgrade(self)
                .expect("ub: dangling pointer")
                .borrow()
                .len()
        }

        fn is_empty(&self) -> bool {
            Weak::upgrade(self)
                .expect("ub: dangling pointer")
                .borrow()
                .is_empty()
        }

        fn address(&self) -> usize {
            self.as_ptr() as usize
        }

        fn upgrade(&self, offset: usize) -> Box<dyn StrongPtrSeam<Pair>> {
            Box::new((Weak::upgrade(self).expect("ub: dangling pointer"), offset))
        }

        fn field_root(&self) -> Weak<dyn Root> {
            self.clone()
        }
    }

    impl StrongPtrSeam<Pair> for (Rc<RefCell<Vec<Pair>>>, usize) {
        fn deref(&self) -> Ref<'_, Pair> {
            let i = self.1;
            Ref::map(self.0.borrow(), move |v| &v[i])
        }

        fn deref_mut(&self) -> RefMut<'_, Pair> {
            let i = self.1;
            RefMut::map(self.0.borrow_mut(), move |v| &mut v[i])
        }
    }

    fn pairs(v: Vec<Pair>) -> Rc<RefCell<Vec<Pair>>> {
        Rc::new(RefCell::new(v))
    }

    #[test]
    fn seam_len() {
        let seam = seg(vec![1, 2, 3]);
        let ptr: Ptr<i32> = seam.as_pointer();
        assert_eq!(ptr.len(), 3);
        assert!(!ptr.is_empty());
    }

    #[test]
    fn seam_elements() {
        let seam = seg(vec![1, 2, 3]);
        let ptr: Ptr<i32> = seam.as_pointer();
        assert_eq!(ptr.with(|v| *v), 1);
        let back: Ptr<i32> = &ptr + 2;
        assert_eq!(back.with(|v| *v), 3);
    }

    #[test]
    fn seam_with_mut() {
        let seam = seg(vec![1, 2, 3]);
        let ptr: Ptr<i32> = seam.as_pointer();
        ptr.with_mut(|v| *v = 10);
        assert_eq!(ptr.with(|v| *v), 10);
    }

    #[test]
    fn seam_pointer() {
        let seam = seg(vec![1, 2, 3]);
        let ptr1: Ptr<i32> = seam.as_pointer();
        let ptr2: Ptr<i32> = seam.as_pointer();
        assert_eq!(ptr1, ptr2);

        let ptr3 = &ptr1 + 1;
        assert_ne!(ptr1, ptr3);
        assert!(ptr3 > ptr1);
        assert_eq!(ptr3 - ptr1, 1);
    }

    #[test]
    #[should_panic(expected = "dangling")]
    fn seam_dangling_ptr() {
        let seam = seg(vec![1, 2, 3]);
        let ptr: Ptr<i32> = seam.as_pointer();
        drop(seam);
        let _ = ptr.len();
    }

    #[test]
    #[should_panic(expected = "not supported by this seam")]
    fn seam_reinterpret_cast_panics() {
        let seam = seg(vec![1, 2, 3]);
        let ptr: Ptr<i32> = seam.as_pointer();
        let _ = ptr.reinterpret_cast::<u8>();
    }

    #[test]
    #[should_panic(expected = "not supported by this seam")]
    fn seam_sort_panics() {
        let seam = seg(vec![3, 1, 2]);
        let ptr: Ptr<i32> = seam.as_pointer();
        ptr.sort(3);
    }

    #[test]
    #[should_panic(expected = "not supported by this seam")]
    fn seam_delete_panics() {
        let seam = seg(vec![1, 2, 3]);
        let ptr: Ptr<i32> = seam.as_pointer();
        ptr.delete();
    }

    #[test]
    #[should_panic(expected = "not supported by this seam")]
    fn seam_to_owned_panics() {
        use crate::ToOwnedOption;
        let seam = seg(vec![1, 2, 3]);
        let ptr: Ptr<i32> = seam.as_pointer();
        let _: Option<Value<i32>> = ptr.to_owned_opt();
    }

    #[test]
    fn seam_field_ptr() {
        let store = pairs(vec![Pair { a: 1, b: 2 }, Pair { a: 3, b: 4 }]);
        let ptr: Ptr<Pair> = Ptr::seam(Rc::new(Rc::downgrade(&store)));

        let b0 = crate::field_ptr!(ptr, b);
        assert_eq!(b0.read(), 2);

        let a1 = crate::field_ptr!(ptr.offset(1), a);
        assert_eq!(a1.read(), 3);
        assert_eq!(crate::field_ptr!(ptr.offset(1), b).read(), 4);

        a1.write(30);
        assert_eq!(store.borrow()[1].a, 30);
    }

    #[test]
    #[should_panic(expected = "field pointers are not supported by this seam")]
    fn seam_field_ptr_default_panics() {
        let store: Value<Seg<Pair>> = Rc::new(RefCell::new(Seg(vec![Pair { a: 1, b: 2 }])));
        let ptr: Ptr<Pair> = store.as_pointer();
        let _ = crate::field_ptr!(ptr, a);
    }
}
