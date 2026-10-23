#include <algorithm>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

template <
    typename T1, typename... T2,
    std::enable_if_t<std::conjunction_v<std::is_constructible<T1, T2...>>, bool>
        T3 = false>
std::optional<T1> f77(std::in_place_t a0, T2 &&...a1) {
  return std::optional<T1>(std::move(a0), std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3,
          std::enable_if_t<std::conjunction_v<std::is_constructible<
                               T1, std::initializer_list<T2> &, T3...>>,
                           bool>
              T4 = false>
std::optional<T1> f78(std::in_place_t a0, std::initializer_list<T2> a1,
                      T3 &&...a2) {
  return std::optional<T1>(std::move(a0), a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2>
std::enable_if_t<
    std::conjunction_v<
        std::integral_constant<
            bool,
            !std::is_same<std::optional<T1>, typename std::remove_cv<
                                                 typename std::remove_reference<
                                                     T2>::type>::type>::value>,
        std::integral_constant<
            bool,
            !std::integral_constant<
                bool, std::is_scalar<T1>::value &&
                          std::is_same<T1, std::decay_t<T2>>::value>::value>,
        std::is_constructible<T1, T2>, std::is_assignable<T1 &, T2>>,
    std::optional<T1> &>
f79(std::optional<T1> &a0, T2 &&a1) {
  return a0.template operator= <T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
std::enable_if_t<
    std::conjunction_v<
        std::integral_constant<bool, !std::is_same<T1, T2>::value>,
        std::is_constructible<T1, const T2 &>,
        std::is_assignable<T1 &, const T2 &>,
        std::integral_constant<
            bool,
            !std::integral_constant<
                bool,
                std::is_constructible<T1, std::optional<T2> &>::value ||
                    std::is_convertible<std::optional<T2> &, T1>::value ||
                    std::is_constructible<T1, std::optional<T2>>::value ||
                    std::is_convertible<std::optional<T2>, T1>::value ||
                    std::is_constructible<T1,
                                          const std::optional<T2> &>::value ||
                    std::is_convertible<const std::optional<T2> &, T1>::value ||
                    std::is_constructible<T1, const std::optional<T2>>::value ||
                    std::is_convertible<const std::optional<T2>,
                                        T1>::value>::value>,
        std::integral_constant<
            bool,
            !std::integral_constant<
                bool,
                std::is_assignable<T1 &, const std::optional<T2> &>::value ||
                    std::is_assignable<T1 &, std::optional<T2> &>::value ||
                    std::is_assignable<T1 &,
                                       const std::optional<T2> &&>::value ||
                    std::is_assignable<T1 &,
                                       std::optional<T2> &&>::value>::value>>,
    std::optional<T1> &>
f80(std::optional<T1> &a0, const std::optional<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
std::enable_if_t<
    std::conjunction_v<
        std::integral_constant<bool, !std::is_same<T1, T2>::value>,
        std::is_constructible<T1, T2>, std::is_assignable<T1 &, T2>,
        std::integral_constant<
            bool,
            !std::integral_constant<
                bool,
                std::is_constructible<T1, std::optional<T2> &>::value ||
                    std::is_convertible<std::optional<T2> &, T1>::value ||
                    std::is_constructible<T1, std::optional<T2>>::value ||
                    std::is_convertible<std::optional<T2>, T1>::value ||
                    std::is_constructible<T1,
                                          const std::optional<T2> &>::value ||
                    std::is_convertible<const std::optional<T2> &, T1>::value ||
                    std::is_constructible<T1, const std::optional<T2>>::value ||
                    std::is_convertible<const std::optional<T2>,
                                        T1>::value>::value>,
        std::integral_constant<
            bool,
            !std::integral_constant<
                bool,
                std::is_assignable<T1 &, const std::optional<T2> &>::value ||
                    std::is_assignable<T1 &, std::optional<T2> &>::value ||
                    std::is_assignable<T1 &,
                                       const std::optional<T2> &&>::value ||
                    std::is_assignable<T1 &,
                                       std::optional<T2> &&>::value>::value>>,
    std::optional<T1> &>
f81(std::optional<T1> &a0, std::optional<T2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}
