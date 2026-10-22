#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <scoped_allocator>
#include <tuple>
#include <utility>

template <typename T1, typename... T2>
using t14 = typename std::scoped_allocator_adaptor<T1, T2...>::value_type;

template <typename T1, typename... T2>
std::scoped_allocator_adaptor<T1, T2...>
f21(const std::scoped_allocator_adaptor<T1, T2...> &a0) {
  return std::scoped_allocator_adaptor<T1, T2...>(a0);
}

template <typename T1, typename... T2>
std::scoped_allocator_adaptor<T1, T2...>
f22(std::scoped_allocator_adaptor<T1, T2...> &&a0) {
  return std::scoped_allocator_adaptor<T1, T2...>(std::move(a0));
}

template <typename T1, typename... T2>
std::scoped_allocator_adaptor<T1, T2...> &
f23(std::scoped_allocator_adaptor<T1, T2...> &a0,
    const std::scoped_allocator_adaptor<T1, T2...> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename... T2>
std::scoped_allocator_adaptor<T1, T2...> &
f24(std::scoped_allocator_adaptor<T1, T2...> &a0,
    std::scoped_allocator_adaptor<T1, T2...> &&a1) {
  return a0.operator=(std::move(a1));
}
