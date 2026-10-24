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
bool f112(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() !=
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f113(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() !=
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f114(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() ==
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f115(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() ==
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f116(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() ==
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f117(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, std::enable_if_t<(std::is_move_constructible_v<T1> &&
                                         std::is_swappable_v<T1>),
                                        int>
                           T2 = 0>
void f118(std::optional<T1> &a0, std::optional<T1> &a1) {
  return std::swap<T1>(a0, a1);
}

template <
    typename T1,
    std::enable_if_t<std::is_constructible_v<std::decay_t<T1>, T1>, int> T2 = 0>
std::optional<std::decay_t<T1>> f119(T1 &&a0) {
  return std::make_optional<T1>(std::forward<T1>(a0));
}

template <
    typename T1, typename... T2,
    std::enable_if_t<std::__is_constructible_for_optional_v<T1, T2...>, int>
        T3 = 0>
std::optional<T1> f120(T2 &&...a0) {
  return std::make_optional<T1, T2...>(std::forward<T2>(a0)...);
}

template <
    typename T1, typename T2, typename... T3,
    std::enable_if_t<
        std::__is_constructible_for_optional_initializer_list_v<T1, T2, T3...>,
        int>
        T4 = 0>
std::optional<T1> f121(std::initializer_list<T2> a0, T3 &&...a1) {
  return std::make_optional<T1, T2, T3...>(a0, std::forward<T3>(a1)...);
}

template <typename T1, typename T2> std::nullopt_t f122(T1 &&a0, T2 &&a1) {
  return std::nullopt_t(std::forward<T1>(a0), std::forward<T2>(a1));
}

template <typename T1, typename T2,
          std::enable_if_t<
              std::__is_core_convertible_v<decltype(std::declval<const T1 &>() <
                                                    std::declval<const T2 &>()),
                                           bool>,
              int>
              T3 = 0>
bool f123(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<
              std::__is_core_convertible_v<decltype(std::declval<const T1 &>() <
                                                    std::declval<const T2 &>()),
                                           bool>,
              int>
              T3 = 0>
bool f124(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<
              std::__is_core_convertible_v<decltype(std::declval<const T1 &>() <
                                                    std::declval<const T2 &>()),
                                           bool>,
              int>
              T3 = 0>
bool f125(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() <=
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f126(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() <=
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f127(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() <=
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f128(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<
              std::__is_core_convertible_v<decltype(std::declval<const T1 &>() >
                                                    std::declval<const T2 &>()),
                                           bool>,
              int>
              T3 = 0>
bool f129(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<
              std::__is_core_convertible_v<decltype(std::declval<const T1 &>() >
                                                    std::declval<const T2 &>()),
                                           bool>,
              int>
              T3 = 0>
bool f130(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<
              std::__is_core_convertible_v<decltype(std::declval<const T1 &>() >
                                                    std::declval<const T2 &>()),
                                           bool>,
              int>
              T3 = 0>
bool f131(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() >=
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f132(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() >=
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f133(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() >=
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f134(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1> std::optional<T1> f135(const std::optional<T1> &a0) {
  return std::optional<T1>(a0);
}

template <typename T1> std::optional<T1> f136(std::optional<T1> &&a0) {
  return std::optional<T1>(std::move(a0));
}

template <typename T1>
std::optional<T1> &f137(std::optional<T1> &a0, const std::optional<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::optional<T1> &f138(std::optional<T1> &a0, std::optional<T1> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename... T3,
          std::enable_if_t<std::_And<std::_IsSame<T2, std::in_place_t>,
                                     std::is_constructible<T1, T3...>>::value,
                           int>
              T4 = 0>
std::optional<T1> f139(T2 a0, T3 &&...a1) {
  return std::optional<T1>(a0, std::forward<T3>(a1)...);
}

template <
    typename T1, typename T2, typename... T3,
    std::enable_if_t<
        std::is_constructible_v<T1, std::initializer_list<T2> &, T3...>, int>
        T4 = 0>
std::optional<T1> f140(std::in_place_t a0, std::initializer_list<T2> a1,
                       T3 &&...a2) {
  return std::optional<T1>(std::move(a0), a1, std::forward<T3>(a2)...);
}

template <typename T1, typename... T2,
          std::enable_if_t<std::is_constructible_v<T1, T2...>, int> T3 = 0>
T1 &f141(std::optional<T1> &a0, T2 &&...a1) {
  return a0.template emplace<T2...>(std::forward<T2>(a1)...);
}

template <
    typename T1, typename T2, typename... T3,
    std::enable_if_t<
        std::is_constructible_v<T1, std::initializer_list<T2> &, T3...>, int>
        T4 = 0>
T1 &f142(std::optional<T1> &a0, std::initializer_list<T2> a1, T3 &&...a2) {
  return a0.template emplace<T2, T3...>(a1, std::forward<T3>(a2)...);
}

template <typename T1>
std::add_pointer_t<T1 const> f143(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1> std::add_pointer_t<T1> f144(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1> T1 const &f145(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1> T1 const &&f146(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1> T1 const &f147(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1> T1 const &&f148(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}
