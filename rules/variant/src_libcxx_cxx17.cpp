#include "../common/init.h"
#include "../common/traits.h"
#include <cstddef>
#include <initializer_list>
#include <tuple>
#include <type_traits>
#include <utility>
#include <variant>

template <typename... T1>
auto f52(std::variant<T1...> &a0, std::variant<T1...> &a1) {
  return std::swap<T1...>(a0, a1);
}

template <typename... T1, std::size_t T2, typename... T3>
auto f53(std::variant<T1...> &a0, T3 &&...a1) {
  return a0.template emplace<T2, T3...>(std::forward<T3>(a1)...);
}

template <typename... T1, std::size_t T2, typename T3, typename... T4>
auto f54(std::variant<T1...> &a0, std::initializer_list<T3> a1, T4 &&...a2) {
  return a0.template emplace<T2, T3, T4...>(a1, std::forward<T4>(a2)...);
}

template <typename T1, typename... T2> decltype(auto) f55(T1 &&a0, T2 &&...a1) {
  return std::visit<T1, T2...>(std::forward<T1>(a0), std::forward<T2>(a1)...);
}
