#include "../common/init.h"
#include "../common/traits.h"
#include <string>
#include <utility>

using t1 = std::allocator<void>;

using t2 = std::allocator<void>::value_type;

using t3 = std::allocator<void>::size_type;

using t4 = std::allocator<void>::difference_type;

using t5 = std::allocator<void>::propagate_on_container_move_assignment;

std::allocator<void> f1() { return std::allocator<void>(); }

std::allocator<void> f2(const std::allocator<void> &a0) {
  return std::allocator<void>(a0);
}

std::allocator<void> &f3(std::allocator<void> &a0,
                         const std::allocator<void> &a1) {
  return a0.operator=(a1);
}

template <typename T1> using t6 = std::allocator<const T1>;

template <typename T1> using t7 = typename std::allocator<const T1>::value_type;

template <typename T1> std::allocator<const T1> f4() {
  return std::allocator<const T1>();
}

template <typename T1, typename T2>
std::allocator<const T1> f5(const std::allocator<T2> &a0) {
  return std::allocator<const T1>(a0);
}

template <typename T1> using t8 = std::allocator<volatile T1>;

template <typename T1>
using t9 = typename std::allocator<volatile T1>::value_type;

template <typename T1> std::allocator<volatile T1> f6() {
  return std::allocator<volatile T1>();
}

template <typename T1, typename T2>
std::allocator<volatile T1> f7(const std::allocator<T2> &a0) {
  return std::allocator<volatile T1>(a0);
}

template <typename T1> using t10 = std::allocator<const volatile T1>;

template <typename T1>
using t11 = typename std::allocator<const volatile T1>::value_type;

template <typename T1> std::allocator<const volatile T1> f8() {
  return std::allocator<const volatile T1>();
}

template <typename T1, typename T2>
std::allocator<const volatile T1> f9(const std::allocator<T2> &a0) {
  return std::allocator<const volatile T1>(a0);
}

template <typename T1> using t12 = std::allocator<T1>;

template <typename T1> using t13 = typename std::allocator<T1>::value_type;

template <typename T1> using t14 = typename std::allocator<T1>::size_type;

template <typename T1> using t15 = typename std::allocator<T1>::difference_type;

template <typename T1>
using t16 = typename std::allocator<T1>::propagate_on_container_move_assignment;

template <typename T1> std::allocator<T1> f10() { return std::allocator<T1>(); }

template <typename T1> std::allocator<T1> f11(const std::allocator<T1> &a0) {
  return std::allocator<T1>(a0);
}

template <typename T1>
std::allocator<T1> &f12(std::allocator<T1> &a0, const std::allocator<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::allocator<T1> f13(const std::allocator<T2> &a0) {
  return std::allocator<T1>(a0);
}

template <typename T1>
bool f14(const std::allocator<T1> &a0, const std::allocator<T1> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
bool f15(const std::allocator<T1> &a0, const std::allocator<T2> &a1) {
  return a0 == a1;
}
