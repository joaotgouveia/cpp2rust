#include <any>
#include <initializer_list>
#include <type_traits>
#include <utility>

template <
    typename T1, typename T2 = std::decay_t<T1>,
    std::enable_if_t<
        std::_And<std::_Not<std::is_same<std::decay_t<T1>, std::any>>,
                  std::_Not<std::__is_inplace_type<T1>>,
                  std::is_copy_constructible<std::decay_t<T1>>>::value,
        int>
        T3 = 0,
    typename std::enable_if<(!(std::is_same<T1, const std::any &>::value) &&
                             !(std::is_same<T1, std::any>::value)),
                            int>::type T4 = 0>
std::any f38(T1 &&a0) {
  return std::any(std::forward<T1>(a0));
}

template <
    typename T1, typename T2 = std::decay_t<T1>,
    std::enable_if_t<!std::is_same_v<std::decay_t<T1>, std::any> &&
                         std::is_copy_constructible_v<std::decay_t<T1>>,
                     int>
        T3 = 0,
    typename std::enable_if<(!(std::is_same<T1, const std::any &>::value) &&
                             !(std::is_same<T1, std::any>::value)),
                            int>::type T4 = 0>
std::any &f39(std::any &a0, T1 &&a1) {
  return a0.template operator= <T1>(std::forward<T1>(a1));
}
