#include <array>
#include <tuple>
#include <utility>

template <typename... T1, typename... T2,
          typename std::enable_if<
              std::conjunction<
                  std::integral_constant<bool, sizeof...(T1) == sizeof...(T2)>,
                  std::is_assignable<T1 &, const T2 &>...>::value,
              int>::type T3 = 0>
std::tuple<T1...> &f87(std::tuple<T1...> &a0, const std::tuple<T2...> &a1) {
  return a0.template operator= <T2...>(a1);
}

template <typename... T1, typename... T2,
          typename std::enable_if<
              std::conjunction<
                  std::integral_constant<bool, sizeof...(T1) == sizeof...(T2)>,
                  std::is_assignable<T1 &, T2>...>::value,
              int>::type T3 = 0>
std::tuple<T1...> &f88(std::tuple<T1...> &a0, std::tuple<T2...> &&a1) {
  return a0.template operator= <T2...>(std::move(a1));
}

template <
    typename... T1, typename T2, std::size_t T3,
    typename std::enable_if<
        std::conjunction<std::integral_constant<bool, T3 == sizeof...(T1)>,
                         std::is_assignable<T1 &, const T2 &>...>::value,
        int>::type T4 = 0>
std::tuple<T1...> &f89(std::tuple<T1...> &a0, const std::array<T2, T3> &a1) {
  return a0.template operator= <T2, T3>(a1);
}

template <
    typename... T1, typename T2, std::size_t T3, typename T4 = void,
    typename std::enable_if<
        std::conjunction<std::integral_constant<bool, T3 == sizeof...(T1)>,
                         std::is_assignable<T1 &, T2>...>::value,
        int>::type T5 = 0>
std::tuple<T1...> &f90(std::tuple<T1...> &a0, std::array<T2, T3> &&a1) {
  return a0.template operator= <T2, T3>(std::move(a1));
}
