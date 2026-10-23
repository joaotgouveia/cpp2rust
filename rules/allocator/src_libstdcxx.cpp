#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t6 = std::allocator<void>;

using t7 = std::allocator<void>::value_type;

using t8 = std::allocator<void>::size_type;

using t9 = std::allocator<void>::difference_type;

using t10 = std::allocator<void>::propagate_on_container_move_assignment;

std::allocator<void> f7() { return std::allocator<void>(); }

std::allocator<void> f8(const std::allocator<void> &a0) {
  return std::allocator<void>(a0);
}

std::allocator<void> &f9(std::allocator<void> &a0,
                         const std::allocator<void> &a1) {
  return a0.operator=(a1);
}

template <typename T1> using t11 = std::allocator<const T1>;

template <typename T1>
using t12 = typename std::allocator<const T1>::value_type;

template <typename T1> std::allocator<const T1> f10() {
  return std::allocator<const T1>();
}

template <typename T1, typename T2>
std::allocator<const T1> f11(const std::allocator<T2> &a0) {
  return std::allocator<const T1>(a0);
}

template <typename T1> using t13 = std::allocator<volatile T1>;

template <typename T1>
using t14 = typename std::allocator<volatile T1>::value_type;

template <typename T1> std::allocator<volatile T1> f12() {
  return std::allocator<volatile T1>();
}

template <typename T1, typename T2>
std::allocator<volatile T1> f13(const std::allocator<T2> &a0) {
  return std::allocator<volatile T1>(a0);
}

template <typename T1> using t15 = std::allocator<const volatile T1>;

template <typename T1>
using t16 = typename std::allocator<const volatile T1>::value_type;

template <typename T1> std::allocator<const volatile T1> f14() {
  return std::allocator<const volatile T1>();
}

template <typename T1, typename T2>
std::allocator<const volatile T1> f15(const std::allocator<T2> &a0) {
  return std::allocator<const volatile T1>(a0);
}

template <typename T1> std::allocator<T1> f16(const std::allocator<T1> &a0) {
  return std::allocator<T1>(a0);
}

template <typename T1>
std::allocator<T1> &f17(std::allocator<T1> &a0, const std::allocator<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
bool f18(const std::allocator<T1> &a0, const std::allocator<T1> &a1) {
  return a0 == a1;
}
