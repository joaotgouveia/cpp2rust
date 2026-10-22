#include "../common/init.h"
#include "../common/traits.h"
#include <memory>
#include <utility>

template <typename T1>
typename std::allocator_traits<T1>::pointer
f8(typename std::allocator_traits<T1>::allocator_type &a0,
   typename std::allocator_traits<T1>::size_type a1) {
  return std::allocator_traits<T1>::allocate(a0, a1);
}

template <typename T1>
void f9(typename std::allocator_traits<T1>::allocator_type &a0,
        typename std::allocator_traits<T1>::pointer a1,
        typename std::allocator_traits<T1>::size_type a2) {
  return std::allocator_traits<T1>::deallocate(a0, a1, a2);
}

template <typename T1>
typename std::allocator_traits<T1>::pointer
f10(typename std::allocator_traits<T1>::allocator_type &a0,
    typename std::allocator_traits<T1>::size_type a1,
    typename std::allocator_traits<T1>::const_void_pointer a2) {
  return std::allocator_traits<T1>::allocate(a0, a1, a2);
}

template <typename T1, typename T2, typename... T3>
void f11(typename std::allocator_traits<T1>::allocator_type &a0, T2 *a1,
         T3 &&...a2) {
  return std::allocator_traits<T1>::construct(a0, a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2>
void f12(typename std::allocator_traits<T1>::allocator_type &a0, T2 *a1) {
  return std::allocator_traits<T1>::destroy(a0, a1);
}

template <typename T1>
typename std::allocator_traits<T1>::size_type
f13(const typename std::allocator_traits<T1>::allocator_type &a0) {
  return std::allocator_traits<T1>::max_size(a0);
}

template <typename T1>
typename std::allocator_traits<T1>::allocator_type
f14(const typename std::allocator_traits<T1>::allocator_type &a0) {
  return std::allocator_traits<T1>::select_on_container_copy_construction(a0);
}
