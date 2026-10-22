#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

using t4 = std::bad_optional_access;

std::bad_optional_access f37() { return std::bad_optional_access(); }

const char *f38(const std::bad_optional_access &a0) { return a0.what(); }

std::bad_optional_access f39(const std::bad_optional_access &a0) {
  return std::bad_optional_access(a0);
}

std::bad_optional_access &f40(std::bad_optional_access &a0,
                              const std::bad_optional_access &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::enable_if_t<std::is_constructible_v<std::decay_t<T1>, T1>,
                 std::optional<std::decay_t<T1>>>
f41(T1 &&a0) {
  return std::make_optional<T1>(std::forward<T1>(a0));
}

template <typename T1, typename... T2>
std::enable_if_t<std::is_constructible_v<T1, T2...>, std::optional<T1>>
f42(T2 &&...a0) {
  return std::make_optional<T1, T2...>(std::forward<T2>(a0)...);
}

template <typename T1, typename T2, typename... T3>
std::enable_if_t<
    std::is_constructible_v<T1, std::initializer_list<T2> &, T3...>,
    std::optional<T1>>
f43(std::initializer_list<T2> a0, T3 &&...a1) {
  return std::make_optional<T1, T2, T3...>(a0, std::forward<T3>(a1)...);
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() !=
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f44(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() !=
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f45(const std::optional<T1> &a0, const T2 &a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() !=
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f46(const T1 &a0, const std::optional<T2> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() <
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f47(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() <
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f48(const std::optional<T1> &a0, const T2 &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() <
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f49(const T1 &a0, const std::optional<T2> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() <=
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f50(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() <=
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f51(const std::optional<T1> &a0, const T2 &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() <=
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f52(const T1 &a0, const std::optional<T2> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() ==
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f53(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() ==
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f54(const std::optional<T1> &a0, const T2 &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() ==
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f55(const T1 &a0, const std::optional<T2> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() >
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f56(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() >
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f57(const std::optional<T1> &a0, const T2 &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() >
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f58(const T1 &a0, const std::optional<T2> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() >=
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f59(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() >=
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f60(const std::optional<T1> &a0, const T2 &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() >=
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f61(const T1 &a0, const std::optional<T2> &a1) {
  return a0 >= a1;
}

template <typename T1> const T1 *f62(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1> T1 *f63(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1> const T1 &f64(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1> T1 &f65(std::optional<T1> &a0) { return a0.operator*(); }

template <typename T1> T1 &&f66(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1> const T1 &&f67(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1> bool f68(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1> const T1 &f69(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1> T1 &f70(std::optional<T1> &a0) { return a0.value(); }

template <typename T1> T1 &&f71(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1> const T1 &&f72(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1> void f73(std::optional<T1> &a0) { return a0.reset(); }

template <
    typename T1, typename... T2,
    std::enable_if_t<std::conjunction_v<std::is_constructible<T1, T2...>>, bool>
        T3 = false>
std::optional<T1> f74(std::in_place_t a0, T2 &&...a1) {
  return std::optional<T1>(std::move(a0), std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3,
          std::enable_if_t<std::conjunction_v<std::is_constructible<
                               T1, std::initializer_list<T2> &, T3...>>,
                           bool>
              T4 = false>
std::optional<T1> f75(std::in_place_t a0, std::initializer_list<T2> a1,
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
f76(std::optional<T1> &a0, T2 &&a1) {
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
f77(std::optional<T1> &a0, const std::optional<T2> &a1) {
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
f78(std::optional<T1> &a0, std::optional<T2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}

template <typename T1, typename... T2>
std::enable_if_t<std::is_constructible_v<T1, T2...>, T1 &>
f79(std::optional<T1> &a0, T2 &&...a1) {
  return a0.template emplace<T2...>(std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3>
std::enable_if_t<
    std::is_constructible_v<T1, std::initializer_list<T2> &, T3...>, T1 &>
f80(std::optional<T1> &a0, std::initializer_list<T2> a1, T3 &&...a2) {
  return a0.template emplace<T2, T3...>(a1, std::forward<T3>(a2)...);
}

template <typename T1>
std::enable_if_t<std::is_move_constructible_v<T1> && std::is_swappable_v<T1>>
f81(std::optional<T1> &a0, std::optional<T1> &a1) {
  return std::swap<T1>(a0, a1);
}
