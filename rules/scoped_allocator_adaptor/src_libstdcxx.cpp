#include <algorithm>
#include <scoped_allocator>
#include <tuple>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename... T2>
using t14 = typename std::scoped_allocator_adaptor<T1, T2...>::value_type;

template <typename T1, typename... T2>
std::scoped_allocator_adaptor<T1, T2...>
f18(const std::scoped_allocator_adaptor<T1, T2...> &a0) {
  return std::scoped_allocator_adaptor<T1, T2...>(a0);
}

template <typename T1, typename... T2>
std::scoped_allocator_adaptor<T1, T2...>
f19(std::scoped_allocator_adaptor<T1, T2...> &&a0) {
  return std::scoped_allocator_adaptor<T1, T2...>(std::move(a0));
}

template <typename T1, typename... T2>
std::scoped_allocator_adaptor<T1, T2...> &
f20(std::scoped_allocator_adaptor<T1, T2...> &a0,
    const std::scoped_allocator_adaptor<T1, T2...> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename... T2>
std::scoped_allocator_adaptor<T1, T2...> &
f21(std::scoped_allocator_adaptor<T1, T2...> &a0,
    std::scoped_allocator_adaptor<T1, T2...> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename... T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_constructible<T1, T3>::value>::type>
std::scoped_allocator_adaptor<T1, T2...> f22(T3 &&a0, const T2 &...a1) {
  return std::scoped_allocator_adaptor<T1, T2...>(std::forward<T3>(a0), a1...);
}

template <typename T1, typename... T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_constructible<T1, const T3 &>::value>::type>
std::scoped_allocator_adaptor<T1, T2...>
f23(const std::scoped_allocator_adaptor<T3, T2...> &a0) {
  return std::scoped_allocator_adaptor<T1, T2...>(a0);
}

template <typename T1, typename... T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_constructible<T1, T3>::value>::type>
std::scoped_allocator_adaptor<T1, T2...>
f24(std::scoped_allocator_adaptor<T3, T2...> &&a0) {
  return std::scoped_allocator_adaptor<T1, T2...>(std::move(a0));
}
