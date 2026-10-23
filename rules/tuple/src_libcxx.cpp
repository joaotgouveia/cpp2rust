#include <array>
#include <tuple>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> std::tuple<> f83(std::array<T1, 0> a0) {
  return std::tuple<>(a0);
}

template <typename T1, typename T2>
std::tuple<> f84(std::allocator_arg_t a0, const T1 &a1, std::array<T2, 0> a2) {
  return std::tuple<>(std::move(a0), a1, a2);
}

std::tuple<> f85() { return std::tuple_cat(); }

template <typename T1, typename... T2>
auto f86(T1 &&a0, T2 &&...a1)
    -> decltype(std::tuple_cat<T1, T2...>(std::forward<T1>(a0),
                                          std::forward<T2>(a1)...)) {
  return std::tuple_cat<T1, T2...>(std::forward<T1>(a0),
                                   std::forward<T2>(a1)...);
}
