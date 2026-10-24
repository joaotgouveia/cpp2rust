#include <algorithm>
#include <cstddef>
#include <memory>
#include <string>
#include <utility>

template <typename T1> using t7 = std::allocator<T1>;

template <typename T1> using t8 = typename std::allocator<T1>::value_type;

template <typename T1> using t9 = typename std::allocator<T1>::size_type;

template <typename T1> using t10 = typename std::allocator<T1>::difference_type;

template <typename T1>
using t11 = typename std::allocator<T1>::propagate_on_container_move_assignment;

template <typename T1> using t12 = typename std::allocator<T1>::is_always_equal;

template <typename T1> std::allocator<T1> f8() { return std::allocator<T1>(); }

template <typename T1> void f9(std::allocator<T1> &a0, T1 *a1, std::size_t a2) {
  return a0.deallocate(a1, a2);
}

template <typename T1, typename T2>
bool f10(const std::allocator<T1> &a0, const std::allocator<T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}
