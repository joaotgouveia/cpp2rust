#include <cstddef>
#include <tuple>
#include <type_traits>
#include <utility>
#include <variant>

#include "../common/init.h"
#include "../common/traits.h"

template <typename... T1>
auto f44(std::variant<T1...> &a0, std::variant<T1...> &a1) {
  return std::swap<T1...>(a0, a1);
}

template <
    typename... T1,
    typename T2 = std::variant_alternative_t<0, std::variant<T1...>>,
    std::enable_if_t<std::is_default_constructible_v<
                         std::variant_alternative_t<0, std::variant<T1...>>>,
                     int>
        T3 = 0>
std::variant<T1...> f45() {
  return std::variant<T1...>();
}

template <typename... T1, std::size_t T2, typename... T3,
          typename T4 = std::enable_if_t<(T2 < sizeof...(T1)), int>,
          typename T5 = std::variant_alternative_t<T2, std::variant<T1...>>,
          std::enable_if_t<
              std::is_constructible_v<
                  std::variant_alternative_t<T2, std::variant<T1...>>, T3...>,
              int>
              T6 = 0>
std::variant<T1...> f46(std::in_place_index_t<T2> a0, T3 &&...a1) {
  return std::variant<T1...>(a0, std::forward<T3>(a1)...);
}
