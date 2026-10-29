#include <algorithm>
#include <string>
#include <utility>

using t1 = std::allocator_traits<std::allocator<void>>;

using t2 = std::allocator_traits<std::allocator<void>>::allocator_type;

using t3 = std::allocator_traits<std::allocator<void>>::value_type;

using t4 = std::allocator_traits<std::allocator<void>>::pointer;

using t5 = std::allocator_traits<std::allocator<void>>::const_pointer;

using t6 = std::allocator_traits<std::allocator<void>>::void_pointer;

using t7 = std::allocator_traits<std::allocator<void>>::const_void_pointer;

using t8 = std::allocator_traits<std::allocator<void>>::difference_type;

using t9 = std::allocator_traits<std::allocator<void>>::size_type;

using t10 = std::allocator_traits<
    std::allocator<void>>::propagate_on_container_copy_assignment;

using t11 = std::allocator_traits<
    std::allocator<void>>::propagate_on_container_move_assignment;

using t12 =
    std::allocator_traits<std::allocator<void>>::propagate_on_container_swap;

using t13 = std::allocator_traits<std::allocator<void>>::is_always_equal;

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

template <typename T1>
using t14 = std::allocator_traits<std::pmr::polymorphic_allocator<T1>>;

template <typename T1>
using t15 = typename std::allocator_traits<
    std::pmr::polymorphic_allocator<T1>>::allocator_type;

template <typename T1>
using t16 = typename std::allocator_traits<
    std::pmr::polymorphic_allocator<T1>>::value_type;

template <typename T1>
using t17 = typename std::allocator_traits<
    std::pmr::polymorphic_allocator<T1>>::pointer;

template <typename T1>
using t18 = typename std::allocator_traits<
    std::pmr::polymorphic_allocator<T1>>::const_pointer;

template <typename T1>
using t19 = typename std::allocator_traits<
    std::pmr::polymorphic_allocator<T1>>::void_pointer;

template <typename T1>
using t20 = typename std::allocator_traits<
    std::pmr::polymorphic_allocator<T1>>::const_void_pointer;

template <typename T1>
using t21 = typename std::allocator_traits<
    std::pmr::polymorphic_allocator<T1>>::difference_type;

template <typename T1>
using t22 = typename std::allocator_traits<
    std::pmr::polymorphic_allocator<T1>>::size_type;

template <typename T1>
using t23 =
    typename std::allocator_traits<std::pmr::polymorphic_allocator<T1>>::
        propagate_on_container_copy_assignment;

template <typename T1>
using t24 =
    typename std::allocator_traits<std::pmr::polymorphic_allocator<T1>>::
        propagate_on_container_move_assignment;

template <typename T1>
using t25 = typename std::allocator_traits<
    std::pmr::polymorphic_allocator<T1>>::propagate_on_container_swap;

template <typename T1>
using t26 = typename std::allocator_traits<
    std::pmr::polymorphic_allocator<T1>>::is_always_equal;

template <typename T1>
std::pmr::polymorphic_allocator<T1>
f9(const std::pmr::polymorphic_allocator<T1> &a0) {
  return std::allocator_traits<std::pmr::polymorphic_allocator<T1>>::
      select_on_container_copy_construction(a0);
}

template <typename T1>
T1 *f10(std::pmr::polymorphic_allocator<T1> &a0, std::size_t a1) {
  return std::allocator_traits<std::pmr::polymorphic_allocator<T1>>::allocate(
      a0, a1);
}

template <typename T1>
T1 *f11(std::pmr::polymorphic_allocator<T1> &a0, std::size_t a1,
        const void *a2) {
  return std::allocator_traits<std::pmr::polymorphic_allocator<T1>>::allocate(
      a0, a1, a2);
}

template <typename T1>
void f12(std::pmr::polymorphic_allocator<T1> &a0, T1 *a1, std::size_t a2) {
  return std::allocator_traits<std::pmr::polymorphic_allocator<T1>>::deallocate(
      a0, a1, a2);
}

template <typename T1>
std::size_t f13(const std::pmr::polymorphic_allocator<T1> &a0) {
  return std::allocator_traits<std::pmr::polymorphic_allocator<T1>>::max_size(
      a0);
}

template <typename T1, typename T2, typename... T3>
void f14(std::pmr::polymorphic_allocator<T1> &a0, T2 *a1, T3 &&...a2) {
  return std::allocator_traits<std::pmr::polymorphic_allocator<T1>>::construct(
      a0, a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2>
void f15(std::pmr::polymorphic_allocator<T1> &a0, T2 *a1) {
  return std::allocator_traits<std::pmr::polymorphic_allocator<T1>>::destroy(
      a0, a1);
}

template <typename T1> using t27 = std::allocator_traits<std::allocator<T1>>;

template <typename T1>
using t28 = typename std::allocator_traits<std::allocator<T1>>::allocator_type;

template <typename T1>
using t29 = typename std::allocator_traits<std::allocator<T1>>::value_type;

template <typename T1>
using t30 = typename std::allocator_traits<std::allocator<T1>>::pointer;

template <typename T1>
using t31 = typename std::allocator_traits<std::allocator<T1>>::const_pointer;

template <typename T1>
using t32 = typename std::allocator_traits<std::allocator<T1>>::void_pointer;

template <typename T1>
using t33 =
    typename std::allocator_traits<std::allocator<T1>>::const_void_pointer;

template <typename T1>
using t34 = typename std::allocator_traits<std::allocator<T1>>::difference_type;

template <typename T1>
using t35 = typename std::allocator_traits<std::allocator<T1>>::size_type;

template <typename T1>
using t36 = typename std::allocator_traits<
    std::allocator<T1>>::propagate_on_container_copy_assignment;

template <typename T1>
using t37 = typename std::allocator_traits<
    std::allocator<T1>>::propagate_on_container_move_assignment;

template <typename T1>
using t38 = typename std::allocator_traits<
    std::allocator<T1>>::propagate_on_container_swap;

template <typename T1>
using t39 = typename std::allocator_traits<std::allocator<T1>>::is_always_equal;

template <typename T1> T1 *f16(std::allocator<T1> &a0, std::size_t a1) {
  return std::allocator_traits<std::allocator<T1>>::allocate(a0, a1);
}

template <typename T1>
T1 *f17(std::allocator<T1> &a0, std::size_t a1, const void *a2) {
  return std::allocator_traits<std::allocator<T1>>::allocate(a0, a1, a2);
}

template <typename T1>
void f18(std::allocator<T1> &a0, T1 *a1, std::size_t a2) {
  return std::allocator_traits<std::allocator<T1>>::deallocate(a0, a1, a2);
}

template <typename T1> std::size_t f19(const std::allocator<T1> &a0) {
  return std::allocator_traits<std::allocator<T1>>::max_size(a0);
}

template <typename T1> std::allocator<T1> f20(const std::allocator<T1> &a0) {
  return std::allocator_traits<
      std::allocator<T1>>::select_on_container_copy_construction(a0);
}

template <typename T1, typename T2, typename... T3>
void f21(std::allocator<T1> &a0, T2 *a1, T3 &&...a2) {
  return std::allocator_traits<std::allocator<T1>>::construct(
      a0, a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2> void f22(std::allocator<T1> &a0, T2 *a1) {
  return std::allocator_traits<std::allocator<T1>>::destroy(a0, a1);
}

template <typename T1> using t40 = std::allocator_traits<T1>;

template <typename T1>
using t41 = typename std::allocator_traits<T1>::allocator_type;

template <typename T1>
using t42 = typename std::allocator_traits<T1>::value_type;

template <typename T1> using t43 = typename std::allocator_traits<T1>::pointer;

template <typename T1>
using t44 = typename std::allocator_traits<T1>::const_pointer;

template <typename T1>
using t45 = typename std::allocator_traits<T1>::void_pointer;

template <typename T1>
using t46 = typename std::allocator_traits<T1>::const_void_pointer;

template <typename T1>
using t47 = typename std::allocator_traits<T1>::difference_type;

template <typename T1>
using t48 = typename std::allocator_traits<T1>::size_type;

template <typename T1>
using t49 =
    typename std::allocator_traits<T1>::propagate_on_container_copy_assignment;

template <typename T1>
using t50 =
    typename std::allocator_traits<T1>::propagate_on_container_move_assignment;

template <typename T1>
using t51 = typename std::allocator_traits<T1>::propagate_on_container_swap;

template <typename T1>
using t52 = typename std::allocator_traits<T1>::is_always_equal;

template <typename T1>
typename std::allocator_traits<T1>::pointer
f23(T1 &a0, typename std::allocator_traits<T1>::size_type a1) {
  return std::allocator_traits<T1>::allocate(
      a0, std::forward<typename std::allocator_traits<T1>::size_type>(a1));
}

template <typename T1>
typename std::allocator_traits<T1>::pointer
f24(T1 &a0, typename std::allocator_traits<T1>::size_type a1,
    typename std::allocator_traits<T1>::const_void_pointer a2) {
  return std::allocator_traits<T1>::allocate(
      a0, std::forward<typename std::allocator_traits<T1>::size_type>(a1),
      std::forward<typename std::allocator_traits<T1>::const_void_pointer>(a2));
}

template <typename T1>
void f25(T1 &a0, typename std::allocator_traits<T1>::pointer a1,
         typename std::allocator_traits<T1>::size_type a2) {
  return std::allocator_traits<T1>::deallocate(
      a0, std::forward<typename std::allocator_traits<T1>::pointer>(a1),
      std::forward<typename std::allocator_traits<T1>::size_type>(a2));
}

template <typename T1>
typename std::allocator_traits<T1>::size_type f26(const T1 &a0) {
  return std::allocator_traits<T1>::max_size(a0);
}

template <typename T1> T1 f27(const T1 &a0) {
  return std::allocator_traits<T1>::select_on_container_copy_construction(a0);
}

template <typename T1, typename T2, typename... T3>
void f28(T1 &a0, T2 *a1, T3 &&...a2) {
  return std::allocator_traits<T1>::construct(a0, a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2> void f29(T1 &a0, T2 *a1) {
  return std::allocator_traits<T1>::destroy(a0, a1);
}
