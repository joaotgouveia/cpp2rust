#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() !=
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f79(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() !=
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f80(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() !=
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f81(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() ==
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f82(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() ==
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f83(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() ==
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f84(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, std::enable_if_t<(std::is_move_constructible_v<T1> &&
                                         std::is_swappable_v<T1>),
                                        int>
                           T2 = 0>
void f85(std::optional<T1> &a0, std::optional<T1> &a1) {
  return std::swap<T1>(a0, a1);
}

template <
    typename T1,
    std::enable_if_t<std::is_constructible_v<std::decay_t<T1>, T1>, int> T2 = 0>
std::optional<std::decay_t<T1>> f86(T1 &&a0) {
  return std::make_optional<T1>(std::forward<T1>(a0));
}

template <
    typename T1, typename... T2,
    std::enable_if_t<std::__is_constructible_for_optional_v<T1, T2...>, int>
        T3 = 0>
std::optional<T1> f87(T2 &&...a0) {
  return std::make_optional<T1, T2...>(std::forward<T2>(a0)...);
}

template <
    typename T1, typename T2, typename... T3,
    std::enable_if_t<
        std::__is_constructible_for_optional_initializer_list_v<T1, T2, T3...>,
        int>
        T4 = 0>
std::optional<T1> f88(std::initializer_list<T2> a0, T3 &&...a1) {
  return std::make_optional<T1, T2, T3...>(a0, std::forward<T3>(a1)...);
}

template <typename T1, typename T2> std::nullopt_t f89(T1 &&a0, T2 &&a1) {
  return std::nullopt_t(std::forward<T1>(a0), std::forward<T2>(a1));
}

template <typename T1, typename T2,
          std::enable_if_t<
              std::__is_core_convertible_v<decltype(std::declval<const T1 &>() <
                                                    std::declval<const T2 &>()),
                                           bool>,
              int>
              T3 = 0>
bool f90(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<
              std::__is_core_convertible_v<decltype(std::declval<const T1 &>() <
                                                    std::declval<const T2 &>()),
                                           bool>,
              int>
              T3 = 0>
bool f91(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<
              std::__is_core_convertible_v<decltype(std::declval<const T1 &>() <
                                                    std::declval<const T2 &>()),
                                           bool>,
              int>
              T3 = 0>
bool f92(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() <=
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f93(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() <=
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f94(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() <=
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f95(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<
              std::__is_core_convertible_v<decltype(std::declval<const T1 &>() >
                                                    std::declval<const T2 &>()),
                                           bool>,
              int>
              T3 = 0>
bool f96(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<
              std::__is_core_convertible_v<decltype(std::declval<const T1 &>() >
                                                    std::declval<const T2 &>()),
                                           bool>,
              int>
              T3 = 0>
bool f97(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<
              std::__is_core_convertible_v<decltype(std::declval<const T1 &>() >
                                                    std::declval<const T2 &>()),
                                           bool>,
              int>
              T3 = 0>
bool f98(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() >=
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f99(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() >=
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f100(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() >=
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f101(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1> std::optional<T1> f102(const std::optional<T1> &a0) {
  return std::optional<T1>(a0);
}

template <typename T1> std::optional<T1> f103(std::optional<T1> &&a0) {
  return std::optional<T1>(std::move(a0));
}

template <typename T1>
std::optional<T1> &f104(std::optional<T1> &a0, const std::optional<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::optional<T1> &f105(std::optional<T1> &a0, std::optional<T1> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename... T3,
          std::enable_if_t<std::_And<std::_IsSame<T2, std::in_place_t>,
                                     std::is_constructible<T1, T3...>>::value,
                           int>
              T4 = 0>
std::optional<T1> f106(T2 a0, T3 &&...a1) {
  return std::optional<T1>(a0, std::forward<T3>(a1)...);
}

template <
    typename T1, typename T2, typename... T3,
    std::enable_if_t<
        std::is_constructible_v<T1, std::initializer_list<T2> &, T3...>, int>
        T4 = 0>
std::optional<T1> f107(std::in_place_t a0, std::initializer_list<T2> a1,
                       T3 &&...a2) {
  return std::optional<T1>(std::move(a0), a1, std::forward<T3>(a2)...);
}

template <typename T1, typename... T2,
          std::enable_if_t<std::is_constructible_v<T1, T2...>, int> T3 = 0>
T1 &f108(std::optional<T1> &a0, T2 &&...a1) {
  return a0.template emplace<T2...>(std::forward<T2>(a1)...);
}

template <
    typename T1, typename T2, typename... T3,
    std::enable_if_t<
        std::is_constructible_v<T1, std::initializer_list<T2> &, T3...>, int>
        T4 = 0>
T1 &f109(std::optional<T1> &a0, std::initializer_list<T2> a1, T3 &&...a2) {
  return a0.template emplace<T2, T3...>(a1, std::forward<T3>(a2)...);
}

template <typename T1>
std::add_pointer_t<T1 const> f110(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1> std::add_pointer_t<T1> f111(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1> T1 const &f112(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1> T1 const &&f113(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1> T1 const &f114(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1> T1 const &&f115(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}
