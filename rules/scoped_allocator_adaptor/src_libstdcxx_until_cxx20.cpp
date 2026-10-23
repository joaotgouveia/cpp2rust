#include <algorithm>
#include <scoped_allocator>
#include <tuple>
#include <utility>

template <
    typename T1, typename... T2, typename T3,
    typename T4 = typename std::enable_if<
        std::is_constructible<T1, const T3 &>::value>::type,
    typename std::enable_if<(!(std::is_same<T3, T1>::value)), int>::type T5 = 0>
std::scoped_allocator_adaptor<T1, T2...>
f23(const std::scoped_allocator_adaptor<T3, T2...> &a0) {
  return std::scoped_allocator_adaptor<T1, T2...>(a0);
}

template <
    typename T1, typename... T2, typename T3,
    typename T4 =
        typename std::enable_if<std::is_constructible<T1, T3>::value>::type,
    typename std::enable_if<(!(std::is_same<T3, T1>::value)), int>::type T5 = 0>
std::scoped_allocator_adaptor<T1, T2...>
f24(std::scoped_allocator_adaptor<T3, T2...> &&a0) {
  return std::scoped_allocator_adaptor<T1, T2...>(std::move(a0));
}
