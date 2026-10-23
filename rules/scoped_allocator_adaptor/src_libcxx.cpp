#include <scoped_allocator>
#include <tuple>
#include <utility>

template <typename T1, typename... T2, typename T3,
          typename std::enable_if<std::is_constructible<T1, T3>::value,
                                  int>::type T4 = 0>
std::scoped_allocator_adaptor<T1, T2...> f25(T3 &&a0, const T2 &...a1) {
  return std::scoped_allocator_adaptor<T1, T2...>(std::forward<T3>(a0), a1...);
}

template <typename T1, typename... T2, typename T3,
          typename std::enable_if<std::is_constructible<T1, const T3 &>::value,
                                  int>::type T4 = 0>
std::scoped_allocator_adaptor<T1, T2...>
f26(const std::scoped_allocator_adaptor<T3, T2...> &a0) {
  return std::scoped_allocator_adaptor<T1, T2...>(a0);
}

template <typename T1, typename... T2, typename T3,
          typename std::enable_if<std::is_constructible<T1, T3>::value,
                                  int>::type T4 = 0>
std::scoped_allocator_adaptor<T1, T2...>
f27(std::scoped_allocator_adaptor<T3, T2...> &&a0) {
  return std::scoped_allocator_adaptor<T1, T2...>(std::move(a0));
}

template <typename T1, typename... T2>
using t15 = typename std::scoped_allocator_adaptor<
    T1, T2...>::propagate_on_container_copy_assignment::value_type;

template <typename T1, typename... T2>
using t16 = typename std::scoped_allocator_adaptor<
    T1, T2...>::propagate_on_container_copy_assignment::type;

template <typename T1, typename... T2>
typename std::scoped_allocator_adaptor<
    T1, T2...>::propagate_on_container_copy_assignment::value_type
f28(const typename std::scoped_allocator_adaptor<
    T1, T2...>::propagate_on_container_copy_assignment &a0) {
  return static_cast<typename std::scoped_allocator_adaptor<
      T1, T2...>::propagate_on_container_copy_assignment::value_type>(a0);
}

template <typename T1, typename... T2>
using t17 = typename std::scoped_allocator_adaptor<
    T1, T2...>::propagate_on_container_move_assignment::value_type;

template <typename T1, typename... T2>
using t18 = typename std::scoped_allocator_adaptor<
    T1, T2...>::propagate_on_container_move_assignment::type;

template <typename T1, typename... T2>
typename std::scoped_allocator_adaptor<
    T1, T2...>::propagate_on_container_move_assignment::value_type
f29(const typename std::scoped_allocator_adaptor<
    T1, T2...>::propagate_on_container_move_assignment &a0) {
  return static_cast<typename std::scoped_allocator_adaptor<
      T1, T2...>::propagate_on_container_move_assignment::value_type>(a0);
}

template <typename T1, typename... T2>
using t19 = typename std::scoped_allocator_adaptor<
    T1, T2...>::propagate_on_container_swap::value_type;

template <typename T1, typename... T2>
using t20 = typename std::scoped_allocator_adaptor<
    T1, T2...>::propagate_on_container_swap::type;

template <typename T1, typename... T2>
typename std::scoped_allocator_adaptor<
    T1, T2...>::propagate_on_container_swap::value_type
f30(const typename std::scoped_allocator_adaptor<
    T1, T2...>::propagate_on_container_swap &a0) {
  return static_cast<typename std::scoped_allocator_adaptor<
      T1, T2...>::propagate_on_container_swap::value_type>(a0);
}

template <typename T1, typename... T2>
using t21 =
    typename std::scoped_allocator_adaptor<T1,
                                           T2...>::is_always_equal::value_type;

template <typename T1, typename... T2>
using t22 =
    typename std::scoped_allocator_adaptor<T1, T2...>::is_always_equal::type;

template <typename T1, typename... T2>
typename std::scoped_allocator_adaptor<T1, T2...>::is_always_equal::value_type
f31(const typename std::scoped_allocator_adaptor<T1, T2...>::is_always_equal
        &a0) {
  return static_cast<typename std::scoped_allocator_adaptor<
      T1, T2...>::is_always_equal::value_type>(a0);
}
