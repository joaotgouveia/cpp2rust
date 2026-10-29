#include <scoped_allocator>
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
using t4 = typename std::scoped_allocator_adaptor<T1, T2...>::value_type;

template <typename T1, typename... T2>
using t5 = typename std::scoped_allocator_adaptor<T1, T2...>::size_type;

template <typename T1, typename... T2>
using t6 = typename std::scoped_allocator_adaptor<T1, T2...>::difference_type;

template <typename T1, typename... T2>
using t7 = typename std::scoped_allocator_adaptor<T1, T2...>::pointer;

template <typename T1, typename... T2>
using t8 = typename std::scoped_allocator_adaptor<T1, T2...>::const_pointer;

template <typename T1, typename... T2>
using t9 = typename std::scoped_allocator_adaptor<T1, T2...>::void_pointer;

template <typename T1, typename... T2>
using t10 =
    typename std::scoped_allocator_adaptor<T1, T2...>::const_void_pointer;

template <typename T1, typename... T2>
using t11 = typename std::scoped_allocator_adaptor<
    T1, T2...>::propagate_on_container_copy_assignment;

template <typename T1, typename... T2>
using t12 = typename std::scoped_allocator_adaptor<
    T1, T2...>::propagate_on_container_move_assignment;

template <typename T1, typename... T2>
using t13 =
    typename std::scoped_allocator_adaptor<T1,
                                           T2...>::propagate_on_container_swap;

template <typename T1, typename... T2>
using t14 = typename std::scoped_allocator_adaptor<T1, T2...>::is_always_equal;

template <typename T1, typename... T2>
std::scoped_allocator_adaptor<T1, T2...> f1() {
  return std::scoped_allocator_adaptor<T1, T2...>();
}

template <typename T1, typename... T2>
std::scoped_allocator_adaptor<T1, T2...>
f2(const std::scoped_allocator_adaptor<T1, T2...> &a0) {
  return std::scoped_allocator_adaptor<T1, T2...>(a0);
}

template <typename T1, typename... T2>
std::scoped_allocator_adaptor<T1, T2...>
f3(std::scoped_allocator_adaptor<T1, T2...> &&a0) {
  return std::scoped_allocator_adaptor<T1, T2...>(std::move(a0));
}

template <typename T1, typename... T2>
std::scoped_allocator_adaptor<T1, T2...> &
f4(std::scoped_allocator_adaptor<T1, T2...> &a0,
   const std::scoped_allocator_adaptor<T1, T2...> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename... T2>
std::scoped_allocator_adaptor<T1, T2...> &
f5(std::scoped_allocator_adaptor<T1, T2...> &a0,
   std::scoped_allocator_adaptor<T1, T2...> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename... T2>
typename std::scoped_allocator_adaptor<T1, T2...>::inner_allocator_type &
f6(std::scoped_allocator_adaptor<T1, T2...> &a0) {
  return a0.inner_allocator();
}

template <typename T1, typename... T2>
const typename std::scoped_allocator_adaptor<T1, T2...>::inner_allocator_type &
f7(const std::scoped_allocator_adaptor<T1, T2...> &a0) {
  return a0.inner_allocator();
}

template <typename T1, typename... T2>
T1 &f8(std::scoped_allocator_adaptor<T1, T2...> &a0) {
  return a0.outer_allocator();
}

template <typename T1, typename... T2>
const T1 &f9(const std::scoped_allocator_adaptor<T1, T2...> &a0) {
  return a0.outer_allocator();
}

template <typename T1, typename... T2>
typename std::scoped_allocator_adaptor<T1, T2...>::pointer
f10(std::scoped_allocator_adaptor<T1, T2...> &a0,
    typename std::scoped_allocator_adaptor<T1, T2...>::size_type a1) {
  return a0.allocate(
      std::forward<
          typename std::scoped_allocator_adaptor<T1, T2...>::size_type>(a1));
}

template <typename T1, typename... T2>
typename std::scoped_allocator_adaptor<T1, T2...>::pointer
f11(std::scoped_allocator_adaptor<T1, T2...> &a0,
    typename std::scoped_allocator_adaptor<T1, T2...>::size_type a1,
    typename std::scoped_allocator_adaptor<T1, T2...>::const_void_pointer a2) {
  return a0.allocate(
      std::forward<
          typename std::scoped_allocator_adaptor<T1, T2...>::size_type>(a1),
      std::forward<typename std::scoped_allocator_adaptor<
          T1, T2...>::const_void_pointer>(a2));
}

template <typename T1, typename... T2>
void f12(std::scoped_allocator_adaptor<T1, T2...> &a0,
         typename std::scoped_allocator_adaptor<T1, T2...>::pointer a1,
         typename std::scoped_allocator_adaptor<T1, T2...>::size_type a2) {
  return a0.deallocate(
      std::forward<typename std::scoped_allocator_adaptor<T1, T2...>::pointer>(
          a1),
      std::forward<
          typename std::scoped_allocator_adaptor<T1, T2...>::size_type>(a2));
}

template <typename T1, typename... T2>
typename std::scoped_allocator_adaptor<T1, T2...>::size_type
f13(const std::scoped_allocator_adaptor<T1, T2...> &a0) {
  return a0.max_size();
}

template <typename T1, typename... T2>
std::scoped_allocator_adaptor<T1, T2...>
f14(const std::scoped_allocator_adaptor<T1, T2...> &a0) {
  return a0.select_on_container_copy_construction();
}

template <typename T1, typename... T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_constructible<T1, T3>::value>::type>
std::scoped_allocator_adaptor<T1, T2...> f15(T3 &&a0, const T2 &...a1) {
  return std::scoped_allocator_adaptor<T1, T2...>(std::forward<T3>(a0), a1...);
}

template <typename T1, typename... T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_constructible<T1, const T3 &>::value>::type>
std::scoped_allocator_adaptor<T1, T2...>
f16(const std::scoped_allocator_adaptor<T3, T2...> &a0)
  requires(!(std::is_same<T3, T1>::value))
{
  return std::scoped_allocator_adaptor<T1, T2...>(a0);
}

template <typename T1, typename... T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_constructible<T1, T3>::value>::type>
std::scoped_allocator_adaptor<T1, T2...>
f17(std::scoped_allocator_adaptor<T3, T2...> &&a0)
  requires(!(std::is_same<T3, T1>::value))
{
  return std::scoped_allocator_adaptor<T1, T2...>(std::move(a0));
}

template <typename T1, typename... T2, typename T3, typename... T4>
void f18(std::scoped_allocator_adaptor<T1, T2...> &a0, T3 *a1, T4 &&...a2) {
  return a0.template construct<T3, T4...>(a1, std::forward<T4>(a2)...);
}

template <typename T1, typename... T2, typename T3>
void f19(std::scoped_allocator_adaptor<T1, T2...> &a0, T3 *a1) {
  return a0.template destroy<T3>(a1);
}
