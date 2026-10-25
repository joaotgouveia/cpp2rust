#include <cstddef>
#include <initializer_list>
#include <tuple>
#include <type_traits>
#include <utility>
#include <variant>

template <typename... T1>
auto f57(std::variant<T1...> &a0, std::variant<T1...> &a1) {
  return std::swap<T1...>(a0, a1);
}

template <
    typename... T1,
    typename T2 = std::variant_alternative_t<0, std::variant<T1...>>,
    std::enable_if_t<std::is_default_constructible_v<
                         std::variant_alternative_t<0, std::variant<T1...>>>,
                     int>
        T3 = 0>
std::variant<T1...> f58() {
  return std::variant<T1...>();
}

template <typename... T1, std::size_t T2, typename... T3,
          typename T4 = std::enable_if_t<(T2 < sizeof...(T1)), int>,
          typename T5 = std::variant_alternative_t<T2, std::variant<T1...>>,
          std::enable_if_t<
              std::is_constructible_v<
                  std::variant_alternative_t<T2, std::variant<T1...>>, T3...>,
              int>
              T6 = 0>
std::variant<T1...> f59(std::in_place_index_t<T2> a0, T3 &&...a1) {
  return std::variant<T1...>(std::move(a0), std::forward<T3>(a1)...);
}

template <
    typename... T1, std::size_t T2, typename T3, typename... T4,
    std::enable_if_t<(T2 < sizeof...(T1)), int> T5 = 0,
    typename T6 = std::variant_alternative_t<T2, std::variant<T1...>>,
    std::enable_if_t<std::is_constructible_v<
                         std::variant_alternative_t<T2, std::variant<T1...>>,
                         std::initializer_list<T3> &, T4...>,
                     int>
        T7 = 0>
std::variant<T1...> f60(std::in_place_index_t<T2> a0,
                        std::initializer_list<T3> a1, T4 &&...a2) {
  return std::variant<T1...>(std::move(a0), std::move(a1),
                             std::forward<T4>(a2)...);
}

template <typename... T1, typename T2, typename... T3,
          std::size_t T4 = std::__find_detail::__find_unambiguous_index_sfinae<
              T2, T1...>::value,
          std::enable_if_t<std::is_constructible_v<T2, T3...>, int> T5 = 0>
std::variant<T1...> f61(std::in_place_type_t<T2> a0, T3 &&...a1) {
  return std::variant<T1...>(std::move(a0), std::forward<T3>(a1)...);
}

template <
    typename... T1, typename T2, typename T3, typename... T4,
    std::size_t T5 =
        std::__find_detail::__find_unambiguous_index_sfinae<T2, T1...>::value,
    std::enable_if_t<
        std::is_constructible_v<T2, std::initializer_list<T3> &, T4...>, int>
        T6 = 0>
std::variant<T1...> f62(std::in_place_type_t<T2> a0,
                        std::initializer_list<T3> a1, T4 &&...a2) {
  return std::variant<T1...>(std::move(a0), std::move(a1),
                             std::forward<T4>(a2)...);
}

template <typename... T1, std::size_t T2, typename... T3,
          std::enable_if_t<(T2 < sizeof...(T1)), int> T4 = 0,
          typename T5 = std::variant_alternative_t<T2, std::variant<T1...>>,
          std::enable_if_t<
              std::is_constructible_v<
                  std::variant_alternative_t<T2, std::variant<T1...>>, T3...>,
              int>
              T6 = 0>
std::variant_alternative_t<T2, std::variant<T1...>> &
f63(std::variant<T1...> &a0, T3 &&...a1) {
  return a0.template emplace<T2, T3...>(std::forward<T3>(a1)...);
}

template <
    typename... T1, std::size_t T2, typename T3, typename... T4,
    std::enable_if_t<(T2 < sizeof...(T1)), int> T5 = 0,
    typename T6 = std::variant_alternative_t<T2, std::variant<T1...>>,
    std::enable_if_t<std::is_constructible_v<
                         std::variant_alternative_t<T2, std::variant<T1...>>,
                         std::initializer_list<T3> &, T4...>,
                     int>
        T7 = 0>
std::variant_alternative_t<T2, std::variant<T1...>> &
f64(std::variant<T1...> &a0, std::initializer_list<T3> a1, T4 &&...a2) {
  return a0.template emplace<T2, T3, T4...>(std::move(a1),
                                            std::forward<T4>(a2)...);
}

template <typename... T1, typename T2, typename... T3,
          std::size_t T4 = std::__find_detail::__find_unambiguous_index_sfinae<
              T2, T1...>::value,
          std::enable_if_t<std::is_constructible_v<T2, T3...>, int> T5 = 0>
T2 &f65(std::variant<T1...> &a0, T3 &&...a1) {
  return a0.template emplace<T2, T3...>(std::forward<T3>(a1)...);
}

template <
    typename... T1, typename T2, typename T3, typename... T4,
    std::size_t T5 =
        std::__find_detail::__find_unambiguous_index_sfinae<T2, T1...>::value,
    std::enable_if_t<
        std::is_constructible_v<T2, std::initializer_list<T3> &, T4...>, int>
        T6 = 0>
T2 &f66(std::variant<T1...> &a0, std::initializer_list<T3> a1, T4 &&...a2) {
  return a0.template emplace<T2, T3, T4...>(std::move(a1),
                                            std::forward<T4>(a2)...);
}

template <
    typename... T1, bool T2 = true,
    std::enable_if_t<std::_And<std::_And<std::bool_constant<true>,
                                         std::is_move_constructible<T1>,
                                         std::is_swappable<T1>>...>::value,
                     int>
        T3 = 0>
void f67(std::variant<T1...> &a0, std::variant<T1...> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename... T2,
          typename T3 =
              std::void_t<decltype(std::__as_variant(std::declval<T2>()))...>>
decltype(auto) f68(T1 &&a0, T2 &&...a1) {
  return std::visit<T1, T2...>(std::forward<T1>(a0), std::forward<T2>(a1)...);
}
