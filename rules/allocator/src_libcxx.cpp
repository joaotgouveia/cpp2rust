#include <cstddef>
#include <memory>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> T1 *f33(std::allocator<T1> &a0, std::size_t a1) {
  return a0.allocate(a1);
}

template <typename T1>
void f34(std::allocator<T1> &a0, T1 *a1, std::size_t a2) {
  return a0.deallocate(a1, a2);
}

template <typename T1>
using t36 = typename std::allocator<
    T1>::propagate_on_container_move_assignment::value_type;

template <typename T1>
using t37 =
    typename std::allocator<T1>::propagate_on_container_move_assignment::type;

template <typename T1>
typename std::allocator<T1>::propagate_on_container_move_assignment::value_type
f35(const typename std::allocator<T1>::propagate_on_container_move_assignment
        &a0) {
  return static_cast<typename std::allocator<
      T1>::propagate_on_container_move_assignment::value_type>(a0);
}
