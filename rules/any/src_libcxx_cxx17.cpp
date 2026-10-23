#include <any>
#include <initializer_list>
#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename... T2, typename T3 = std::decay_t<T1>,
          std::enable_if_t<std::is_constructible_v<std::decay_t<T1>, T2...> &&
                               std::is_copy_constructible_v<std::decay_t<T1>>,
                           int>
              T4 = 0>
std::any f33(std::in_place_type_t<T1> a0, T2 &&...a1) {
  return std::any(a0, std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3,
          typename T4 = std::decay_t<T1>,
          std::enable_if_t<
              std::is_constructible_v<std::decay_t<T1>,
                                      std::initializer_list<T2> &, T3...> &&
                  std::is_copy_constructible_v<std::decay_t<T1>>,
              int>
              T5 = 0>
std::any f34(std::in_place_type_t<T1> a0, std::initializer_list<T2> a1,
             T3 &&...a2) {
  return std::any(a0, a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2 = std::decay_t<T1>,
          std::enable_if_t<!std::is_same_v<std::decay_t<T1>, std::any> &&
                               std::is_copy_constructible_v<std::decay_t<T1>>,
                           int>
              T3 = 0>
std::any &f35(std::any &a0, T1 &&a1) {
  return a0.template operator= <T1>(std::forward<T1>(a1));
}

template <typename T1, typename... T2, typename T3 = std::decay_t<T1>,
          std::enable_if_t<std::is_constructible_v<std::decay_t<T1>, T2...> &&
                               std::is_copy_constructible_v<std::decay_t<T1>>,
                           int>
              T4 = 0>
std::decay_t<T1> &f36(std::any &a0, T2 &&...a1) {
  return a0.template emplace<T1, T2...>(std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3,
          typename T4 = std::decay_t<T1>,
          std::enable_if_t<
              std::is_constructible_v<std::decay_t<T1>,
                                      std::initializer_list<T2> &, T3...> &&
                  std::is_copy_constructible_v<std::decay_t<T1>>,
              int>
              T5 = 0>
std::decay_t<T1> &f37(std::any &a0, std::initializer_list<T2> a1, T3 &&...a2) {
  return a0.template emplace<T1, T2, T3...>(a1, std::forward<T3>(a2)...);
}

template <typename T1> std::add_pointer_t<T1> f38(std::any *a0) {
  return std::any_cast<T1>(a0);
}

template <typename T1>
std::add_pointer_t<std::add_const_t<T1>> f39(const std::any *a0) {
  return std::any_cast<T1>(a0);
}

template <typename T1, typename... T2> std::any f40(T2 &&...a0) {
  return std::make_any<T1, T2...>(std::forward<T2>(a0)...);
}

template <typename T1, typename T2, typename... T3>
std::any f41(std::initializer_list<T2> a0, T3 &&...a1) {
  return std::make_any<T1, T2, T3...>(a0, std::forward<T3>(a1)...);
}
