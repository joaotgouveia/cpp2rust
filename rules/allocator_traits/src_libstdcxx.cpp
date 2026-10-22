#include "../common/init.h"
#include "../common/traits.h"
#include <string>
#include <utility>

template <typename T1>
using t13 = typename std::allocator_traits<T1>::propagate_on_container_swap;

template <typename T1>
typename std::allocator_traits<T1>::pointer
f1(T1 &a0, typename std::allocator_traits<T1>::size_type a1) {
  return std::allocator_traits<T1>::allocate(a0, a1);
}

template <typename T1>
typename std::allocator_traits<T1>::pointer
f2(T1 &a0, typename std::allocator_traits<T1>::size_type a1,
   typename std::allocator_traits<T1>::const_void_pointer a2) {
  return std::allocator_traits<T1>::allocate(a0, a1, a2);
}

template <typename T1>
void f3(T1 &a0, typename std::allocator_traits<T1>::pointer a1,
        typename std::allocator_traits<T1>::size_type a2) {
  return std::allocator_traits<T1>::deallocate(a0, a1, a2);
}

template <typename T1>
typename std::allocator_traits<T1>::size_type f4(const T1 &a0) {
  return std::allocator_traits<T1>::max_size(a0);
}

template <typename T1> T1 f5(const T1 &a0) {
  return std::allocator_traits<T1>::select_on_container_copy_construction(a0);
}

template <typename T1, typename T2> void f6(T1 &a0, T2 *a1) {
  return std::allocator_traits<T1>::destroy(a0, a1);
}
