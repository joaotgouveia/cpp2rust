#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <scoped_allocator>
#include <tuple>
#include <utility>

template <typename T1, typename... T2>
using t1 = std::scoped_allocator_adaptor<T1, T2...>;

template <typename T1, typename... T2>
using t2 =
    typename std::scoped_allocator_adaptor<T1, T2...>::outer_allocator_type;

template <typename T1, typename... T2>
using t3 =
    typename std::scoped_allocator_adaptor<T1, T2...>::inner_allocator_type;

template <typename T1, typename... T2>
using t4 = typename std::scoped_allocator_adaptor<T1, T2...>::size_type;

template <typename T1, typename... T2>
using t5 = typename std::scoped_allocator_adaptor<T1, T2...>::difference_type;

template <typename T1, typename... T2>
using t6 = typename std::scoped_allocator_adaptor<T1, T2...>::pointer;

template <typename T1, typename... T2>
using t7 = typename std::scoped_allocator_adaptor<T1, T2...>::const_pointer;

template <typename T1, typename... T2>
using t8 = typename std::scoped_allocator_adaptor<T1, T2...>::void_pointer;

template <typename T1, typename... T2>
using t9 =
    typename std::scoped_allocator_adaptor<T1, T2...>::const_void_pointer;

template <typename T1, typename... T2>
using t10 = typename std::scoped_allocator_adaptor<
    T1, T2...>::propagate_on_container_copy_assignment;

template <typename T1, typename... T2>
using t11 = typename std::scoped_allocator_adaptor<
    T1, T2...>::propagate_on_container_move_assignment;

template <typename T1, typename... T2>
using t12 =
    typename std::scoped_allocator_adaptor<T1,
                                           T2...>::propagate_on_container_swap;

template <typename T1, typename... T2>
using t13 = typename std::scoped_allocator_adaptor<T1, T2...>::is_always_equal;

template <typename T1, typename... T2>
std::scoped_allocator_adaptor<T1, T2...> f1() {
  return std::scoped_allocator_adaptor<T1, T2...>();
}

template <typename T1, typename... T2>
typename std::scoped_allocator_adaptor<T1, T2...>::inner_allocator_type &
f2(std::scoped_allocator_adaptor<T1, T2...> &a0) {
  return a0.inner_allocator();
}

template <typename T1, typename... T2>
const typename std::scoped_allocator_adaptor<T1, T2...>::inner_allocator_type &
f3(const std::scoped_allocator_adaptor<T1, T2...> &a0) {
  return a0.inner_allocator();
}

template <typename T1, typename... T2>
typename std::scoped_allocator_adaptor<T1, T2...>::outer_allocator_type &
f4(std::scoped_allocator_adaptor<T1, T2...> &a0) {
  return a0.outer_allocator();
}

template <typename T1, typename... T2>
const typename std::scoped_allocator_adaptor<T1, T2...>::outer_allocator_type &
f5(const std::scoped_allocator_adaptor<T1, T2...> &a0) {
  return a0.outer_allocator();
}

template <typename T1, typename... T2>
typename std::scoped_allocator_adaptor<T1, T2...>::pointer
f6(std::scoped_allocator_adaptor<T1, T2...> &a0,
   typename std::scoped_allocator_adaptor<T1, T2...>::size_type a1) {
  return a0.allocate(a1);
}

template <typename T1, typename... T2>
typename std::scoped_allocator_adaptor<T1, T2...>::pointer
f7(std::scoped_allocator_adaptor<T1, T2...> &a0,
   typename std::scoped_allocator_adaptor<T1, T2...>::size_type a1,
   typename std::scoped_allocator_adaptor<T1, T2...>::const_void_pointer a2) {
  return a0.allocate(a1, a2);
}

template <typename T1, typename... T2>
void f8(std::scoped_allocator_adaptor<T1, T2...> &a0,
        typename std::scoped_allocator_adaptor<T1, T2...>::pointer a1,
        typename std::scoped_allocator_adaptor<T1, T2...>::size_type a2) {
  return a0.deallocate(a1, a2);
}

template <typename T1, typename... T2>
typename std::scoped_allocator_adaptor<T1, T2...>::size_type
f9(const std::scoped_allocator_adaptor<T1, T2...> &a0) {
  return a0.max_size();
}

template <typename T1, typename... T2>
std::scoped_allocator_adaptor<T1, T2...>
f10(const std::scoped_allocator_adaptor<T1, T2...> &a0) {
  return a0.select_on_container_copy_construction();
}

template <typename T1, typename... T2, typename T3, typename... T4>
void f11(std::scoped_allocator_adaptor<T1, T2...> &a0, T3 *a1, T4 &&...a2) {
  return a0.template construct<T3, T4...>(a1, std::forward<T4>(a2)...);
}

template <typename T1, typename... T2, typename T3>
void f12(std::scoped_allocator_adaptor<T1, T2...> &a0, T3 *a1) {
  return a0.template destroy<T3>(a1);
}
