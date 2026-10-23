#include <algorithm>
#include <string>
#include <utility>

using t13 = std::allocator_traits<std::allocator<void>>;

using t14 = std::allocator_traits<std::allocator<void>>::allocator_type;

using t15 = std::allocator_traits<std::allocator<void>>::value_type;

using t16 = std::allocator_traits<std::allocator<void>>::pointer;

using t17 = std::allocator_traits<std::allocator<void>>::const_pointer;

using t18 = std::allocator_traits<std::allocator<void>>::void_pointer;

using t19 = std::allocator_traits<std::allocator<void>>::const_void_pointer;

using t20 = std::allocator_traits<std::allocator<void>>::difference_type;

using t21 = std::allocator_traits<std::allocator<void>>::size_type;

using t22 = std::allocator_traits<
    std::allocator<void>>::propagate_on_container_copy_assignment;

using t23 = std::allocator_traits<
    std::allocator<void>>::propagate_on_container_move_assignment;

using t24 =
    std::allocator_traits<std::allocator<void>>::propagate_on_container_swap;

using t25 = std::allocator_traits<std::allocator<void>>::is_always_equal;

std::allocator_traits<std::allocator<void>>::allocator_type
f1(const std::allocator_traits<std::allocator<void>>::allocator_type &a0) {
  return std::allocator_traits<
      std::allocator<void>>::select_on_container_copy_construction(a0);
}

std::allocator_traits<std::allocator<void>> f2() {
  return std::allocator_traits<std::allocator<void>>();
}

std::allocator_traits<std::allocator<void>>
f3(const std::allocator_traits<std::allocator<void>> &a0) {
  return std::allocator_traits<std::allocator<void>>(a0);
}

std::allocator_traits<std::allocator<void>>
f4(std::allocator_traits<std::allocator<void>> &&a0) {
  return std::allocator_traits<std::allocator<void>>(std::move(a0));
}

std::allocator_traits<std::allocator<void>> &
f5(std::allocator_traits<std::allocator<void>> &a0,
   const std::allocator_traits<std::allocator<void>> &a1) {
  return a0.operator=(a1);
}

std::allocator_traits<std::allocator<void>> &
f6(std::allocator_traits<std::allocator<void>> &a0,
   std::allocator_traits<std::allocator<void>> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename... T2>
void f7(std::allocator<void> &a0, T1 *a1, T2 &&...a2) {
  return std::allocator_traits<std::allocator<void>>::construct(
      a0, a1, std::forward<T2>(a2)...);
}

template <typename T1> void f8(std::allocator<void> &a0, T1 *a1) {
  return std::allocator_traits<std::allocator<void>>::destroy(a0, a1);
}

template <typename T1> using t26 = std::allocator_traits<std::allocator<T1>>;

template <typename T1>
using t27 = typename std::allocator_traits<std::allocator<T1>>::allocator_type;

template <typename T1>
using t28 = typename std::allocator_traits<std::allocator<T1>>::value_type;

template <typename T1>
using t29 = typename std::allocator_traits<std::allocator<T1>>::pointer;

template <typename T1>
using t30 = typename std::allocator_traits<std::allocator<T1>>::const_pointer;

template <typename T1>
using t31 = typename std::allocator_traits<std::allocator<T1>>::void_pointer;

template <typename T1>
using t32 =
    typename std::allocator_traits<std::allocator<T1>>::const_void_pointer;

template <typename T1>
using t33 = typename std::allocator_traits<std::allocator<T1>>::difference_type;

template <typename T1>
using t34 = typename std::allocator_traits<std::allocator<T1>>::size_type;

template <typename T1>
using t35 = typename std::allocator_traits<
    std::allocator<T1>>::propagate_on_container_copy_assignment;

template <typename T1>
using t36 = typename std::allocator_traits<
    std::allocator<T1>>::propagate_on_container_move_assignment;

template <typename T1>
using t37 = typename std::allocator_traits<
    std::allocator<T1>>::propagate_on_container_swap;

template <typename T1>
using t38 = typename std::allocator_traits<std::allocator<T1>>::is_always_equal;

template <typename T1> T1 *f9(std::allocator<T1> &a0, std::size_t a1) {
  return std::allocator_traits<std::allocator<T1>>::allocate(a0, a1);
}

template <typename T1>
T1 *f10(std::allocator<T1> &a0, std::size_t a1, const void *a2) {
  return std::allocator_traits<std::allocator<T1>>::allocate(a0, a1, a2);
}

template <typename T1>
void f11(std::allocator<T1> &a0, T1 *a1, std::size_t a2) {
  return std::allocator_traits<std::allocator<T1>>::deallocate(a0, a1, a2);
}

template <typename T1> std::size_t f12(const std::allocator<T1> &a0) {
  return std::allocator_traits<std::allocator<T1>>::max_size(a0);
}

template <typename T1> std::allocator<T1> f13(const std::allocator<T1> &a0) {
  return std::allocator_traits<
      std::allocator<T1>>::select_on_container_copy_construction(a0);
}

template <typename T1, typename T2, typename... T3>
void f14(std::allocator<T1> &a0, T2 *a1, T3 &&...a2) {
  return std::allocator_traits<std::allocator<T1>>::construct(
      a0, a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2> void f15(std::allocator<T1> &a0, T2 *a1) {
  return std::allocator_traits<std::allocator<T1>>::destroy(a0, a1);
}

template <typename T1>
using t39 = typename std::allocator_traits<T1>::propagate_on_container_swap;

template <typename T1>
typename std::allocator_traits<T1>::pointer
f16(T1 &a0, typename std::allocator_traits<T1>::size_type a1) {
  return std::allocator_traits<T1>::allocate(a0, a1);
}

template <typename T1>
typename std::allocator_traits<T1>::pointer
f17(T1 &a0, typename std::allocator_traits<T1>::size_type a1,
    typename std::allocator_traits<T1>::const_void_pointer a2) {
  return std::allocator_traits<T1>::allocate(a0, a1, a2);
}

template <typename T1>
void f18(T1 &a0, typename std::allocator_traits<T1>::pointer a1,
         typename std::allocator_traits<T1>::size_type a2) {
  return std::allocator_traits<T1>::deallocate(a0, a1, a2);
}

template <typename T1>
typename std::allocator_traits<T1>::size_type f19(const T1 &a0) {
  return std::allocator_traits<T1>::max_size(a0);
}

template <typename T1> T1 f20(const T1 &a0) {
  return std::allocator_traits<T1>::select_on_container_copy_construction(a0);
}

template <typename T1, typename T2> void f21(T1 &a0, T2 *a1) {
  return std::allocator_traits<T1>::destroy(a0, a1);
}
