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
        T3 = true>
std::optional<T1> f59(T2 &&a0) {
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
        T3 = false>
std::optional<T1> f60(T2 &&a0) {
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
std::optional<T1> f61(const std::optional<T2> &a0) {
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
std::optional<T1> f62(const std::optional<T2> &a0) {
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
std::optional<T1> f63(std::optional<T2> &&a0) {
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
std::optional<T1> f64(std::optional<T2> &&a0) {
  return std::optional<T1>(std::move(a0));
}

template <typename T1, typename... T2,
          std::enable_if_t<std::__and_v<std::is_constructible<T1, T2...>>, bool>
              T3 = false>
std::optional<T1> f65(std::in_place_t a0, T2 &&...a1) {
  return std::optional<T1>(std::move(a0), std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3,
          std::enable_if_t<std::__and_v<std::is_constructible<
                               T1, std::initializer_list<T2> &, T3...>>,
                           bool>
              T4 = false>
std::optional<T1> f66(std::in_place_t a0, std::initializer_list<T2> a1,
                      T3 &&...a2) {
  return std::optional<T1>(std::move(a0), a1, std::forward<T3>(a2)...);
}
