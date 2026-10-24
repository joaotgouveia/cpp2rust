#include <algorithm>
#include <string>
#include <utility>

using t13 = std::allocator<void>;

using t14 = std::allocator<void>::value_type;

using t15 = std::allocator<void>::size_type;

using t16 = std::allocator<void>::difference_type;

using t17 = std::allocator<void>::propagate_on_container_move_assignment;

using t18 = std::allocator<void>::is_always_equal;

std::allocator<void> f11() { return std::allocator<void>(); }

std::allocator<void> f12(const std::allocator<void> &a0) {
  return std::allocator<void>(a0);
}

std::allocator<void> &f13(std::allocator<void> &a0,
                          const std::allocator<void> &a1) {
  return a0.operator=(a1);
}

template <typename T1> using t19 = std::allocator<const T1>;

template <typename T1>
using t20 = typename std::allocator<const T1>::value_type;

template <typename T1> std::allocator<const T1> f14() {
  return std::allocator<const T1>();
}

template <typename T1, typename T2>
std::allocator<const T1> f15(const std::allocator<T2> &a0) {
  return std::allocator<const T1>(a0);
}

template <typename T1> using t21 = std::allocator<volatile T1>;

template <typename T1>
using t22 = typename std::allocator<volatile T1>::value_type;

template <typename T1> std::allocator<volatile T1> f16() {
  return std::allocator<volatile T1>();
}

template <typename T1, typename T2>
std::allocator<volatile T1> f17(const std::allocator<T2> &a0) {
  return std::allocator<volatile T1>(a0);
}

template <typename T1> using t23 = std::allocator<const volatile T1>;

template <typename T1>
using t24 = typename std::allocator<const volatile T1>::value_type;

template <typename T1> std::allocator<const volatile T1> f18() {
  return std::allocator<const volatile T1>();
}

template <typename T1, typename T2>
std::allocator<const volatile T1> f19(const std::allocator<T2> &a0) {
  return std::allocator<const volatile T1>(a0);
}

template <typename T1> std::allocator<T1> f20(const std::allocator<T1> &a0) {
  return std::allocator<T1>(a0);
}

template <typename T1>
std::allocator<T1> &f21(std::allocator<T1> &a0, const std::allocator<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
bool f22(const std::allocator<T1> &a0, const std::allocator<T1> &a1) {
  return operator==(a0, a1);
}
