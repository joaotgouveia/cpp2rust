#include <cstddef>
#include <initializer_list>
#include <tuple>
#include <type_traits>
#include <utility>
#include <variant>

template <
    typename... T1, typename T2,
    std::enable_if_t<
        !std::is_same_v<std::__remove_cvref_t<T2>, std::variant<T1...>>, int>
        T3 = 0,
    std::enable_if_t<!std::__is_inplace_type<std::__remove_cvref_t<T2>>::value,
                     int>
        T4 = 0,
    std::enable_if_t<!std::__is_inplace_index<std::__remove_cvref_t<T2>>::value,
                     int>
        T5 = 0,
    typename T6 = std::__variant_detail::__best_match_t<T2, T1...>,
    std::size_t T7 = std::__find_detail::__find_unambiguous_index_sfinae<
        std::__variant_detail::__best_match_t<T2, T1...>, T1...>::value,
    std::enable_if_t<std::is_constructible_v<
                         std::__variant_detail::__best_match_t<T2, T1...>, T2>,
                     int>
        T8 = 0,
    typename std::enable_if<
        (!(std::is_same<T2, const std::variant<T1...> &>::value) &&
         !(std::is_same<T2, std::variant<T1...>>::value)),
        int>::type T9 = 0>
std::variant<T1...> f69(T2 &&a0) {
  return std::variant<T1...>(std::forward<T2>(a0));
}

template <
    typename... T1, typename T2,
    std::enable_if_t<
        !std::is_same_v<std::__remove_cvref_t<T2>, std::variant<T1...>>, int>
        T3 = 0,
    typename T4 = std::__variant_detail::__best_match_t<T2, T1...>,
    std::size_t T5 = std::__find_detail::__find_unambiguous_index_sfinae<
        std::__variant_detail::__best_match_t<T2, T1...>, T1...>::value,
    std::enable_if_t<
        std::is_assignable_v<std::__variant_detail::__best_match_t<T2, T1...> &,
                             T2> &&
            std::is_constructible_v<
                std::__variant_detail::__best_match_t<T2, T1...>, T2>,
        int>
        T6 = 0,
    typename std::enable_if<
        (!(std::is_same<T2, const std::variant<T1...> &>::value) &&
         !(std::is_same<T2, std::variant<T1...>>::value)),
        int>::type T7 = 0>
std::variant<T1...> &f70(std::variant<T1...> &a0, T2 &&a1) {
  return a0.template operator= <T2>(std::forward<T2>(a1));
}
