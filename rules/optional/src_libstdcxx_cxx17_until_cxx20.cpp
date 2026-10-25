#include <algorithm>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

template <typename T1, typename T2>
std::__optional_ne_t<T1, T2> f63(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::__optional_ne_t<T1, T2> f64(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::__optional_lt_t<T1, T2> f65(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::__optional_lt_t<T1, T2> f66(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::__optional_le_t<T1, T2> f67(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::__optional_le_t<T1, T2> f68(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::__optional_eq_t<T1, T2> f69(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::__optional_eq_t<T1, T2> f70(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::__optional_gt_t<T1, T2> f71(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::__optional_gt_t<T1, T2> f72(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::__optional_ge_t<T1, T2> f73(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::__optional_ge_t<T1, T2> f74(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <
    typename T1, typename T2 = std::remove_cv_t<T1>,
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
std::optional<T1> f75(T2 &&a0) {
  return std::optional<T1>(std::forward<T2>(a0));
}

template <
    typename T1, typename T2 = std::remove_cv_t<T1>,
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
std::optional<T1> f76(T2 &&a0) {
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
std::optional<T1> f77(const std::optional<T2> &a0) {
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
std::optional<T1> f78(const std::optional<T2> &a0) {
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
std::optional<T1> f79(std::optional<T2> &&a0) {
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
std::optional<T1> f80(std::optional<T2> &&a0) {
  return std::optional<T1>(std::move(a0));
}

template <typename T1, typename... T2,
          std::enable_if_t<std::__and_v<std::is_constructible<T1, T2...>>, bool>
              T3 = false>
std::optional<T1> f81(std::in_place_t a0, T2 &&...a1) {
  return std::optional<T1>(std::move(a0), std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3,
          std::enable_if_t<std::__and_v<std::is_constructible<
                               T1, std::initializer_list<T2> &, T3...>>,
                           bool>
              T4 = false>
std::optional<T1> f82(std::in_place_t a0, std::initializer_list<T2> a1,
                      T3 &&...a2) {
  return std::optional<T1>(std::move(a0), std::move(a1),
                           std::forward<T3>(a2)...);
}

template <typename T1, typename T2 = std::remove_cv_t<T1>,
          typename std::enable_if<(!(std::is_same<T2, std::nullopt_t>::value)),
                                  int>::type T3 = 0>
std::enable_if_t<
    std::__and_v<
        std::__not_<std::is_same<std::optional<T1>, std::__remove_cvref_t<T2>>>,
        std::__not_<std::__and_<std::is_scalar<T1>,
                                std::is_same<T1, std::decay_t<T2>>>>,
        std::is_constructible<T1, T2>, std::is_assignable<T1 &, T2>>,
    std::optional<T1> &>
f83(std::optional<T1> &a0, T2 &&a1) {
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
f84(std::optional<T1> &a0, const std::optional<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
std::enable_if_t<
    std::__and_v<std::__not_<std::is_same<T1, T2>>,
                 std::is_constructible<T1, T2>, std::is_assignable<T1 &, T2>,
                 std::__not_<std::__converts_from_optional<T1, T2>>,
                 std::__not_<std::__assigns_from_optional<T1, T2>>>,
    std::optional<T1> &>
f85(std::optional<T1> &a0, std::optional<T2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}
