#include <scoped_allocator>
#include <tuple>
#include <utility>

template <typename T1, typename... T2, typename T3,
          std::__enable_if_t<std::is_constructible<T1, T3>::value, int> T4 = 0>
std::scoped_allocator_adaptor<T1, T2...> f27(T3 &&a0, const T2 &...a1) {
  return std::scoped_allocator_adaptor<T1, T2...>(std::forward<T3>(a0), a1...);
}

template <typename T1, typename... T2, typename T3,
          std::__enable_if_t<std::is_constructible<T1, const T3 &>::value, int>
              T4 = 0>
std::scoped_allocator_adaptor<T1, T2...>
f28(const std::scoped_allocator_adaptor<T3, T2...> &a0) {
  return std::scoped_allocator_adaptor<T1, T2...>(a0);
}

template <typename T1, typename... T2, typename T3,
          std::__enable_if_t<std::is_constructible<T1, T3>::value, int> T4 = 0>
std::scoped_allocator_adaptor<T1, T2...>
f29(std::scoped_allocator_adaptor<T3, T2...> &&a0) {
  return std::scoped_allocator_adaptor<T1, T2...>(std::move(a0));
}
