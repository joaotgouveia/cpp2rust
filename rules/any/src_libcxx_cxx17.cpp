#include <any>
#include <initializer_list>
#include <type_traits>
#include <utility>

template <typename T1, typename... T2, typename T3 = std::decay_t<T1>,
          std::enable_if_t<std::is_constructible_v<std::decay_t<T1>, T2...> &&
                               std::is_copy_constructible_v<std::decay_t<T1>>,
                           int>
              T4 = 0>
std::any f40(std::in_place_type_t<T1> a0, T2 &&...a1) {
  return std::any(std::move(a0), std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3,
          typename T4 = std::decay_t<T1>,
          std::enable_if_t<
              std::is_constructible_v<std::decay_t<T1>,
                                      std::initializer_list<T2> &, T3...> &&
                  std::is_copy_constructible_v<std::decay_t<T1>>,
              int>
              T5 = 0>
std::any f41(std::in_place_type_t<T1> a0, std::initializer_list<T2> a1,
             T3 &&...a2) {
  return std::any(std::move(a0), std::move(a1), std::forward<T3>(a2)...);
}

template <typename T1, typename... T2, typename T3 = std::decay_t<T1>,
          std::enable_if_t<std::is_constructible_v<std::decay_t<T1>, T2...> &&
                               std::is_copy_constructible_v<std::decay_t<T1>>,
                           int>
              T4 = 0>
std::decay_t<T1> &f42(std::any &a0, T2 &&...a1) {
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
std::decay_t<T1> &f43(std::any &a0, std::initializer_list<T2> a1, T3 &&...a2) {
  return a0.template emplace<T1, T2, T3...>(std::move(a1),
                                            std::forward<T3>(a2)...);
}

template <typename T1> std::add_pointer_t<T1> f44(std::any *a0) {
  return std::any_cast<T1>(a0);
}

template <typename T1>
std::add_pointer_t<std::add_const_t<T1>> f45(const std::any *a0) {
  return std::any_cast<T1>(a0);
}

template <typename T1, typename... T2> std::any f46(T2 &&...a0) {
  return std::make_any<T1, T2...>(std::forward<T2>(a0)...);
}

template <typename T1, typename T2, typename... T3>
std::any f47(std::initializer_list<T2> a0, T3 &&...a1) {
  return std::make_any<T1, T2, T3...>(std::move(a0), std::forward<T3>(a1)...);
}
