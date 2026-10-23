#include <array>
#include <tuple>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <
    typename... T1, typename... T2,
    std::enable_if_t<std::conjunction<
        std::integral_constant<bool, sizeof...(T1) == sizeof...(T2)>,
        std::is_assignable<const T1 &, const T2 &>...>::value> *T3 = nullptr>
const std::tuple<T1...> &f92(const std::tuple<T1...> &a0,
                             const std::tuple<T2...> &a1) {
  return a0.template operator= <T2...>(a1);
}

template <typename... T1, typename... T2,
          std::enable_if_t<std::conjunction<
              std::integral_constant<bool, sizeof...(T1) == sizeof...(T2)>,
              std::is_assignable<const T1 &, T2>...>::value> *T3 = nullptr>
const std::tuple<T1...> &f93(const std::tuple<T1...> &a0,
                             std::tuple<T2...> &&a1) {
  return a0.template operator= <T2...>(std::move(a1));
}
