#include <memory>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

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
