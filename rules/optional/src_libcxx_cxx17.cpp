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
bool f113(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() !=
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f114(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() !=
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f115(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() ==
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f116(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() ==
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f117(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() ==
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f118(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, std::enable_if_t<(std::is_move_constructible_v<T1> &&
                                         std::is_swappable_v<T1>),
                                        int>
                           T2 = 0>
void f119(std::optional<T1> &a0, std::optional<T1> &a1) {
  return std::swap<T1>(a0, a1);
}

template <
    typename T1,
    std::enable_if_t<std::is_constructible_v<std::decay_t<T1>, T1>, int> T2 = 0>
std::optional<std::decay_t<T1>> f120(T1 &&a0) {
  return std::make_optional<T1>(std::forward<T1>(a0));
}

template <
    typename T1, typename... T2,
    std::enable_if_t<std::__is_constructible_for_optional_v<T1, T2...>, int>
        T3 = 0>
std::optional<T1> f121(T2 &&...a0) {
  return std::make_optional<T1, T2...>(std::forward<T2>(a0)...);
}

template <
    typename T1, typename T2, typename... T3,
    std::enable_if_t<
        std::__is_constructible_for_optional_initializer_list_v<T1, T2, T3...>,
        int>
        T4 = 0>
std::optional<T1> f122(std::initializer_list<T2> a0, T3 &&...a1) {
  return std::make_optional<T1, T2, T3...>(std::move(a0),
                                           std::forward<T3>(a1)...);
}

template <typename T1, typename T2> std::nullopt_t f123(T1 &&a0, T2 &&a1) {
  return std::nullopt_t(std::forward<T1>(a0), std::forward<T2>(a1));
}

using t6 = std::nullopt_t::__secret_tag;

std::nullopt_t::__secret_tag f124() { return std::nullopt_t::__secret_tag(); }

template <typename T1, typename T2,
          std::enable_if_t<
              std::__is_core_convertible_v<decltype(std::declval<const T1 &>() <
                                                    std::declval<const T2 &>()),
                                           bool>,
              int>
              T3 = 0>
bool f125(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<
              std::__is_core_convertible_v<decltype(std::declval<const T1 &>() <
                                                    std::declval<const T2 &>()),
                                           bool>,
              int>
              T3 = 0>
bool f126(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<
              std::__is_core_convertible_v<decltype(std::declval<const T1 &>() <
                                                    std::declval<const T2 &>()),
                                           bool>,
              int>
              T3 = 0>
bool f127(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() <=
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f128(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() <=
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f129(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() <=
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f130(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<
              std::__is_core_convertible_v<decltype(std::declval<const T1 &>() >
                                                    std::declval<const T2 &>()),
                                           bool>,
              int>
              T3 = 0>
bool f131(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<
              std::__is_core_convertible_v<decltype(std::declval<const T1 &>() >
                                                    std::declval<const T2 &>()),
                                           bool>,
              int>
              T3 = 0>
bool f132(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<
              std::__is_core_convertible_v<decltype(std::declval<const T1 &>() >
                                                    std::declval<const T2 &>()),
                                           bool>,
              int>
              T3 = 0>
bool f133(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() >=
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f134(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() >=
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f135(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::enable_if_t<std::__is_core_convertible_v<
                               decltype(std::declval<const T1 &>() >=
                                        std::declval<const T2 &>()),
                               bool>,
                           int>
              T3 = 0>
bool f136(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1> std::optional<T1> f137(const std::optional<T1> &a0) {
  return std::optional<T1>(a0);
}

template <typename T1> std::optional<T1> f138(std::optional<T1> &&a0) {
  return std::optional<T1>(std::move(a0));
}

template <typename T1>
std::optional<T1> &f139(std::optional<T1> &a0, const std::optional<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::optional<T1> &f140(std::optional<T1> &a0, std::optional<T1> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename... T3,
          std::enable_if_t<std::_And<std::_IsSame<T2, std::in_place_t>,
                                     std::is_constructible<T1, T3...>>::value,
                           int>
              T4 = 0>
std::optional<T1> f141(T2 a0, T3 &&...a1) {
  return std::optional<T1>(std::move(a0), std::forward<T3>(a1)...);
}

template <
    typename T1, typename T2, typename... T3,
    std::enable_if_t<
        std::is_constructible_v<T1, std::initializer_list<T2> &, T3...>, int>
        T4 = 0>
std::optional<T1> f142(std::in_place_t a0, std::initializer_list<T2> a1,
                       T3 &&...a2) {
  return std::optional<T1>(std::move(a0), std::move(a1),
                           std::forward<T3>(a2)...);
}

template <typename T1, typename... T2,
          std::enable_if_t<std::is_constructible_v<T1, T2...>, int> T3 = 0>
T1 &f143(std::optional<T1> &a0, T2 &&...a1) {
  return a0.template emplace<T2...>(std::forward<T2>(a1)...);
}

template <
    typename T1, typename T2, typename... T3,
    std::enable_if_t<
        std::is_constructible_v<T1, std::initializer_list<T2> &, T3...>, int>
        T4 = 0>
T1 &f144(std::optional<T1> &a0, std::initializer_list<T2> a1, T3 &&...a2) {
  return a0.template emplace<T2, T3...>(std::move(a1), std::forward<T3>(a2)...);
}

template <typename T1>
std::add_pointer_t<T1 const> f145(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1> std::add_pointer_t<T1> f146(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1> T1 const &f147(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1> T1 const &&f148(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1> T1 const &f149(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1> T1 const &&f150(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}
