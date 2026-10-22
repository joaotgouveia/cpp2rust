#include "../common/init.h"
#include "../common/traits.h"
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

template <typename T1, std::enable_if_t<(std::is_move_constructible_v<T1> &&
                                         std::is_swappable_v<T1>),
                                        int>
                           T2 = 0>
void f94(std::optional<T1> &a0, std::optional<T1> &a1) {
  return std::swap<T1>(a0, a1);
}

template <
    typename T1,
    std::enable_if_t<std::is_constructible_v<std::decay_t<T1>, T1>, int> T2 = 0>
std::optional<std::decay_t<T1>> f95(T1 &&a0) {
  return std::make_optional<T1>(std::forward<T1>(a0));
}

template <typename T1, typename T2> std::nullopt_t f96(T1 &&a0, T2 &&a1) {
  return std::nullopt_t(std::forward<T1>(a0), std::forward<T2>(a1));
}

template <typename T1> std::optional<T1> f97(const std::optional<T1> &a0) {
  return std::optional<T1>(a0);
}

template <typename T1> std::optional<T1> f98(std::optional<T1> &&a0) {
  return std::optional<T1>(std::move(a0));
}

template <typename T1>
std::optional<T1> &f99(std::optional<T1> &a0, const std::optional<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::optional<T1> &f100(std::optional<T1> &a0, std::optional<T1> &&a1) {
  return a0.operator=(std::move(a1));
}

template <
    typename T1, typename T2, typename... T3,
    std::enable_if_t<
        std::integral_constant<
            bool, std::integral_constant<
                      bool, std::is_same<T2, std::in_place_t>::value>::value &&
                      std::is_constructible<T1, T3...>::value>::value,
        int>
        T4 = 0>
std::optional<T1> f101(T2 a0, T3 &&...a1) {
  return std::optional<T1>(a0, std::forward<T3>(a1)...);
}

template <
    typename T1, typename T2, typename... T3,
    std::enable_if_t<
        std::is_constructible_v<T1, std::initializer_list<T2> &, T3...>, int>
        T4 = 0>
std::optional<T1> f102(std::in_place_t a0, std::initializer_list<T2> a1,
                       T3 &&...a2) {
  return std::optional<T1>(std::move(a0), a1, std::forward<T3>(a2)...);
}

template <
    typename T1, typename T2,
    std::enable_if_t<
        std::integral_constant<
            bool,
            std::integral_constant<
                bool, !std::is_same<
                          typename std::remove_cv<
                              typename std::remove_reference<T2>::type>::type,
                          std::optional<T1>>::value>::value &&
                std::integral_constant<
                    bool,
                    std::integral_constant<
                        bool, !std::is_same<typename std::remove_cv<
                                                typename std::remove_reference<
                                                    T2>::type>::type,
                                            T1>::value>::value ||
                        std::integral_constant<
                            bool, !std::is_scalar<T1>::value>::value>::value &&
                std::is_constructible<T1, T2>::value &&
                std::is_assignable<T1 &, T2>::value>::value,
        int>
        T3 = 0>
std::optional<T1> &f103(std::optional<T1> &a0, T2 &&a1) {
  return a0.template operator= <T2>(std::forward<T2>(a1));
}

template <typename T1, typename... T2,
          std::enable_if_t<std::is_constructible_v<T1, T2...>, int> T3 = 0>
T1 &f104(std::optional<T1> &a0, T2 &&...a1) {
  return a0.template emplace<T2...>(std::forward<T2>(a1)...);
}

template <
    typename T1, typename T2, typename... T3,
    std::enable_if_t<
        std::is_constructible_v<T1, std::initializer_list<T2> &, T3...>, int>
        T4 = 0>
T1 &f105(std::optional<T1> &a0, std::initializer_list<T2> a1, T3 &&...a2) {
  return a0.template emplace<T2, T3...>(a1, std::forward<T3>(a2)...);
}
