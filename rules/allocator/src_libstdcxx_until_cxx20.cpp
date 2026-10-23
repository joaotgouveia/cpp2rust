#include <string>
#include <utility>

using t17 = std::allocator<void>::pointer;

using t18 = std::allocator<void>::const_pointer;

using t19 = std::allocator<void>::is_always_equal;

template <typename T1> using t20 = typename std::allocator<T1>::pointer;

template <typename T1> using t21 = typename std::allocator<T1>::const_pointer;

template <typename T1> using t22 = typename std::allocator<T1>::reference;

template <typename T1> using t23 = typename std::allocator<T1>::const_reference;

template <typename T1> using t24 = typename std::allocator<T1>::is_always_equal;

template <typename T1>
bool f19(const std::allocator<T1> &a0, const std::allocator<T1> &a1) {
  return a0 != a1;
}
