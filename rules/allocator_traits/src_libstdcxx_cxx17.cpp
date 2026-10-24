#include <algorithm>
#include <string>
#include <utility>

template <typename T1>
using t40 = std::allocator_traits<std::pmr::polymorphic_allocator<T1>>;

template <typename T1>
using t41 = typename std::allocator_traits<
    std::pmr::polymorphic_allocator<T1>>::allocator_type;

template <typename T1>
using t42 = typename std::allocator_traits<
    std::pmr::polymorphic_allocator<T1>>::value_type;

template <typename T1>
using t43 = typename std::allocator_traits<
    std::pmr::polymorphic_allocator<T1>>::pointer;

template <typename T1>
using t44 = typename std::allocator_traits<
    std::pmr::polymorphic_allocator<T1>>::const_pointer;

template <typename T1>
using t45 = typename std::allocator_traits<
    std::pmr::polymorphic_allocator<T1>>::void_pointer;

template <typename T1>
using t46 = typename std::allocator_traits<
    std::pmr::polymorphic_allocator<T1>>::const_void_pointer;

template <typename T1>
using t47 = typename std::allocator_traits<
    std::pmr::polymorphic_allocator<T1>>::difference_type;

template <typename T1>
using t48 = typename std::allocator_traits<
    std::pmr::polymorphic_allocator<T1>>::size_type;

template <typename T1>
using t49 =
    typename std::allocator_traits<std::pmr::polymorphic_allocator<T1>>::
        propagate_on_container_copy_assignment;

template <typename T1>
using t50 =
    typename std::allocator_traits<std::pmr::polymorphic_allocator<T1>>::
        propagate_on_container_move_assignment;

template <typename T1>
using t51 = typename std::allocator_traits<
    std::pmr::polymorphic_allocator<T1>>::propagate_on_container_swap;

template <typename T1>
using t52 = typename std::allocator_traits<
    std::pmr::polymorphic_allocator<T1>>::is_always_equal;

template <typename T1>
std::pmr::polymorphic_allocator<T1>
f23(const std::pmr::polymorphic_allocator<T1> &a0) {
  return std::allocator_traits<std::pmr::polymorphic_allocator<T1>>::
      select_on_container_copy_construction(a0);
}

template <typename T1>
T1 *f24(std::pmr::polymorphic_allocator<T1> &a0, std::size_t a1) {
  return std::allocator_traits<std::pmr::polymorphic_allocator<T1>>::allocate(
      a0, a1);
}

template <typename T1>
T1 *f25(std::pmr::polymorphic_allocator<T1> &a0, std::size_t a1,
        const void *a2) {
  return std::allocator_traits<std::pmr::polymorphic_allocator<T1>>::allocate(
      a0, a1, a2);
}

template <typename T1>
void f26(std::pmr::polymorphic_allocator<T1> &a0, T1 *a1, std::size_t a2) {
  return std::allocator_traits<std::pmr::polymorphic_allocator<T1>>::deallocate(
      a0, a1, a2);
}

template <typename T1>
std::size_t f27(const std::pmr::polymorphic_allocator<T1> &a0) {
  return std::allocator_traits<std::pmr::polymorphic_allocator<T1>>::max_size(
      a0);
}

template <typename T1, typename T2, typename... T3>
void f28(std::pmr::polymorphic_allocator<T1> &a0, T2 *a1, T3 &&...a2) {
  return std::allocator_traits<std::pmr::polymorphic_allocator<T1>>::construct(
      a0, a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2>
void f29(std::pmr::polymorphic_allocator<T1> &a0, T2 *a1) {
  return std::allocator_traits<std::pmr::polymorphic_allocator<T1>>::destroy(
      a0, a1);
}
