#include <algorithm>
#include <iosfwd>
#include <string>
#include <utility>

using t1 = std::allocator<void>;

using t2 = std::allocator<void>::value_type;

using t3 = std::allocator<void>::size_type;

using t4 = std::allocator<void>::difference_type;

using t5 = std::allocator<void>::propagate_on_container_move_assignment;

using t6 = std::allocator<void>::is_always_equal;

std::allocator<void> f1() { return std::allocator<void>(); }

std::allocator<void> f2(const std::allocator<void> &a0) {
  return std::allocator<void>(a0);
}

std::allocator<void> &f3(std::allocator<void> &a0,
                         const std::allocator<void> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::allocator<void> f4(const std::allocator<T1> &a0)
  requires(!(std::is_same<T1, void>::value))
{
  return std::allocator<void>(a0);
}

template <typename T1> using t7 = std::allocator<const T1>;

template <typename T1> using t8 = typename std::allocator<const T1>::value_type;

template <typename T1> std::allocator<const T1> f5() {
  return std::allocator<const T1>();
}

template <typename T1, typename T2>
std::allocator<const T1> f6(const std::allocator<T2> &a0) {
  return std::allocator<const T1>(a0);
}

template <typename T1> using t9 = std::allocator<volatile T1>;

template <typename T1>
using t10 = typename std::allocator<volatile T1>::value_type;

template <typename T1> std::allocator<volatile T1> f7() {
  return std::allocator<volatile T1>();
}

template <typename T1, typename T2>
std::allocator<volatile T1> f8(const std::allocator<T2> &a0) {
  return std::allocator<volatile T1>(a0);
}

template <typename T1> using t11 = std::allocator<const volatile T1>;

template <typename T1>
using t12 = typename std::allocator<const volatile T1>::value_type;

template <typename T1> std::allocator<const volatile T1> f9() {
  return std::allocator<const volatile T1>();
}

template <typename T1, typename T2>
std::allocator<const volatile T1> f10(const std::allocator<T2> &a0) {
  return std::allocator<const volatile T1>(a0);
}

template <typename T1> using t13 = std::allocator<T1>;

template <typename T1> using t14 = typename std::allocator<T1>::value_type;

template <typename T1> using t15 = typename std::allocator<T1>::size_type;

template <typename T1> using t16 = typename std::allocator<T1>::difference_type;

template <typename T1>
using t17 = typename std::allocator<T1>::propagate_on_container_move_assignment;

template <typename T1> using t18 = typename std::allocator<T1>::is_always_equal;

template <typename T1> std::allocator<T1> f11() { return std::allocator<T1>(); }

template <typename T1> std::allocator<T1> f12(const std::allocator<T1> &a0) {
  return std::allocator<T1>(a0);
}

template <typename T1>
std::allocator<T1> &f13(std::allocator<T1> &a0, const std::allocator<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1> T1 *f14(std::allocator<T1> &a0, std::size_t a1) {
  return a0.allocate(a1);
}

template <typename T1>
void f15(std::allocator<T1> &a0, T1 *a1, std::size_t a2) {
  return a0.deallocate(a1, a2);
}

template <typename T1, typename T2>
std::allocator<T1> f16(const std::allocator<T2> &a0)
  requires(!(std::is_same<T2, T1>::value))
{
  return std::allocator<T1>(a0);
}

template <typename T1>
bool f17(const std::allocator<T1> &a0, const std::allocator<T1> &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2>
bool f18(const std::allocator<T1> &a0, const std::allocator<T2> &a1) {
  return std::operator==(a0, a1);
}
