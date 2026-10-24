#include <algorithm>
#include <type_traits>
#include <utility>
#include <variant>

template <typename... T1, typename T2,
          typename std::enable_if<
              (!(std::is_same<T2, const std::variant<T1...> &>::value) &&
               !(std::is_same<T2, std::variant<T1...>>::value)),
              int>::type T3 = 0>
std::enable_if_t<
    std::variant<T1...>::template __exactly_once<typename std::_Nth_type<
        std::variant<T1...>::template __accepted_index<T2 &&>, T1...>::type> &&
        std::is_constructible_v<
            typename std::_Nth_type<
                std::variant<T1...>::template __accepted_index<T2 &&>,
                T1...>::type,
            T2> &&
        std::is_assignable_v<
            typename std::_Nth_type<
                std::variant<T1...>::template __accepted_index<T2 &&>,
                T1...>::type &,
            T2>,
    std::variant<T1...> &>
f42(std::variant<T1...> &a0, T2 &&a1) {
  return a0.template operator= <T2>(std::forward<T2>(a1));
}
