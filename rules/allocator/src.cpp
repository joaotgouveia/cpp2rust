#include "../common/init.h"
#include "../common/traits.h"
#include <cstddef>
#include <memory>
#include <string>
#include <utility>

template <typename T1> using t1 = std::allocator<T1>;

template <typename T1> using t2 = typename std::allocator<T1>::value_type;

template <typename T1> using t3 = typename std::allocator<T1>::size_type;

template <typename T1> using t4 = typename std::allocator<T1>::difference_type;

template <typename T1>
using t5 = typename std::allocator<T1>::propagate_on_container_move_assignment;

template <typename T1> std::allocator<T1> f4() { return std::allocator<T1>(); }

template <typename T1, typename T2>
std::allocator<T1> f5(const std::allocator<T2> &a0) {
  return std::allocator<T1>(a0);
}

template <typename T1, typename T2>
bool f6(const std::allocator<T1> &a0, const std::allocator<T2> &a1) {
  return a0 == a1;
}
