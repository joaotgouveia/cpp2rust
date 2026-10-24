#include <algorithm>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

template <
    typename T1, typename T2,
    std::enable_if_t<
        std::__and_v<std::__not_<std::is_same<std::optional<T1>,
                                              std::__remove_cvref_t<T2>>>,
                     std::__not_<std::is_same<std::in_place_t,
                                              std::__remove_cvref_t<T2>>>,
                     std::is_constructible<T1, T2>, std::is_convertible<T2, T1>,
                     typename std::optional<T1>::
                         template __not_constructing_bool_from_optional<T2>>,
        bool>
        T3 = true,
    typename std::enable_if<(!(std::is_same<T2, std::nullopt_t>::value)),
                            int>::type T4 = 0>
std::optional<T1> f56(T2 &&a0) {
  return std::optional<T1>(std::forward<T2>(a0));
}

template <
    typename T1, typename T2,
    std::enable_if_t<
        std::__and_v<std::__not_<std::is_same<std::optional<T1>,
                                              std::__remove_cvref_t<T2>>>,
                     std::__not_<std::is_same<std::in_place_t,
                                              std::__remove_cvref_t<T2>>>,
                     std::is_constructible<T1, T2>,
                     std::__not_<std::is_convertible<T2, T1>>,
                     typename std::optional<T1>::
                         template __not_constructing_bool_from_optional<T2>>,
        bool>
        T3 = false,
    typename std::enable_if<(!(std::is_same<T2, std::nullopt_t>::value)),
                            int>::type T4 = 0>
std::optional<T1> f57(T2 &&a0) {
  return std::optional<T1>(std::forward<T2>(a0));
}

template <typename T1, typename T2,
          std::enable_if_t<
              std::__and_v<std::__not_<std::is_same<T1, T2>>,
                           std::is_constructible<T1, const T2 &>,
                           std::is_convertible<const T2 &, T1>,
                           typename std::optional<T1>::
                               template __construct_from_contained_value<T2>>,
              bool>
              T3 = true>
std::optional<T1> f58(const std::optional<T2> &a0) {
  return std::optional<T1>(a0);
}

template <typename T1, typename T2,
          std::enable_if_t<
              std::__and_v<std::__not_<std::is_same<T1, T2>>,
                           std::is_constructible<T1, const T2 &>,
                           std::__not_<std::is_convertible<const T2 &, T1>>,
                           typename std::optional<T1>::
                               template __construct_from_contained_value<T2>>,
              bool>
              T3 = false>
std::optional<T1> f59(const std::optional<T2> &a0) {
  return std::optional<T1>(a0);
}

template <
    typename T1, typename T2,
    std::enable_if_t<
        std::__and_v<std::__not_<std::is_same<T1, T2>>,
                     std::is_constructible<T1, T2>, std::is_convertible<T2, T1>,
                     typename std::optional<
                         T1>::template __construct_from_contained_value<T2>>,
        bool>
        T3 = true>
std::optional<T1> f60(std::optional<T2> &&a0) {
  return std::optional<T1>(std::move(a0));
}

template <typename T1, typename T2,
          std::enable_if_t<
              std::__and_v<std::__not_<std::is_same<T1, T2>>,
                           std::is_constructible<T1, T2>,
                           std::__not_<std::is_convertible<T2, T1>>,
                           typename std::optional<T1>::
                               template __construct_from_contained_value<T2>>,
              bool>
              T3 = false>
std::optional<T1> f61(std::optional<T2> &&a0) {
  return std::optional<T1>(std::move(a0));
}

template <typename T1, typename... T2,
          std::enable_if_t<std::__and_v<std::is_constructible<T1, T2...>>, bool>
              T3 = false>
std::optional<T1> f62(std::in_place_t a0, T2 &&...a1) {
  return std::optional<T1>(std::move(a0), std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3,
          std::enable_if_t<std::__and_v<std::is_constructible<
                               T1, std::initializer_list<T2> &, T3...>>,
                           bool>
              T4 = false>
std::optional<T1> f63(std::in_place_t a0, std::initializer_list<T2> a1,
                      T3 &&...a2) {
  return std::optional<T1>(std::move(a0), a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2,
          typename std::enable_if<(!(std::is_same<T2, std::nullopt_t>::value)),
                                  int>::type T3 = 0>
std::enable_if_t<
    std::__and_v<
        std::__not_<std::is_same<std::optional<T1>, std::__remove_cvref_t<T2>>>,
        std::__not_<std::__and_<std::is_scalar<T1>,
                                std::is_same<T1, std::decay_t<T2>>>>,
        std::is_constructible<T1, T2>, std::is_assignable<T1 &, T2>>,
    std::optional<T1> &>
f64(std::optional<T1> &a0, T2 &&a1) {
  return a0.template operator= <T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
std::enable_if_t<
    std::__and_v<std::__not_<std::is_same<T1, T2>>,
                 std::is_constructible<T1, const T2 &>,
                 std::is_assignable<T1 &, const T2 &>,
                 std::__not_<std::__converts_from_optional<T1, T2>>,
                 std::__not_<std::__assigns_from_optional<T1, T2>>>,
    std::optional<T1> &>
f65(std::optional<T1> &a0, const std::optional<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
std::enable_if_t<
    std::__and_v<std::__not_<std::is_same<T1, T2>>,
                 std::is_constructible<T1, T2>, std::is_assignable<T1 &, T2>,
                 std::__not_<std::__converts_from_optional<T1, T2>>,
                 std::__not_<std::__assigns_from_optional<T1, T2>>>,
    std::optional<T1> &>
f66(std::optional<T1> &a0, std::optional<T2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}
