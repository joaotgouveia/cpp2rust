#include <functional>
#include <utility>

template <typename T1, typename... T2> using t1 = std::__bind<T1, T2...>;

template <
    typename T1, typename... T2, typename T3, typename... T4,
    std::__enable_if_t<std::is_constructible<std::__decay_t<T1>, T3>::value &&
                           !std::is_same<std::__libcpp_remove_reference_t<T3>,
                                         std::__bind<T1, T2...>>::value,
                       int>
        T5 = 0>
std::__bind<T1, T2...> f1(T3 &&a0, T4 &&...a1) {
  return std::__bind<T1, T2...>(std::forward<T3>(a0), std::forward<T4>(a1)...);
}

template <typename T1, typename... T2, typename... T3>
auto f2(std::__bind<T1, T2...> &a0, T3 &&...a1)
    -> decltype(a0.template operator()<T3...>(std::forward<T3>(a1)...)) {
  return a0.template operator()<T3...>(std::forward<T3>(a1)...);
}

template <typename T1, typename... T2, typename... T3>
auto f3(const std::__bind<T1, T2...> &a0, T3 &&...a1)
    -> decltype(a0.template operator()<T3...>(std::forward<T3>(a1)...)) {
  return a0.template operator()<T3...>(std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename... T3>
using t2 = std::__bind_r<T1, T2, T3...>;

template <typename T1, typename T2, typename... T3>
using t3 = typename std::__bind_r<T1, T2, T3...>::result_type;

template <
    typename T1, typename T2, typename... T3, typename T4, typename... T5,
    std::__enable_if_t<std::is_constructible<
                           typename std::__bind<T2, T3...>::_Fd, T4>::value &&
                           !std::is_same<std::__libcpp_remove_reference_t<T4>,
                                         std::__bind_r<T1, T2, T3...>>::value,
                       int>
        T6 = 0>
std::__bind_r<T1, T2, T3...> f4(T4 &&a0, T5 &&...a1) {
  return std::__bind_r<T1, T2, T3...>(std::forward<T4>(a0),
                                      std::forward<T5>(a1)...);
}

template <typename T1, typename T2, typename... T3, typename... T4,
          std::__enable_if_t<
              std::is_convertible<typename std::__bind_return<
                                      typename std::__bind<T2, T3...>::_Fd,
                                      typename std::__bind<T2, T3...>::_Td,
                                      std::tuple<T4 &&...>>::type,
                                  T1>::value ||
                  std::is_void<T1>::value,
              int>
              T5 = 0>
T1 f5(std::__bind_r<T1, T2, T3...> &a0, T4 &&...a1) {
  return a0.template operator()<T4...>(std::forward<T4>(a1)...);
}

template <typename T1, typename T2, typename... T3, typename... T4,
          std::__enable_if_t<std::is_convertible<
                                 typename std::__bind_return<
                                     const typename std::__bind<T2, T3...>::_Fd,
                                     const typename std::__bind<T2, T3...>::_Td,
                                     std::tuple<T4 &&...>>::type,
                                 T1>::value ||
                                 std::is_void<T1>::value,
                             int>
              T5 = 0>
T1 f6(const std::__bind_r<T1, T2, T3...> &a0, T4 &&...a1) {
  return a0.template operator()<T4...>(std::forward<T4>(a1)...);
}

template <typename T1, typename... T2>
std::__bind<T1, T2...> f7(T1 &&a0, T2 &&...a1) {
  return std::bind<T1, T2...>(std::forward<T1>(a0), std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3>
std::__bind_r<T1, T2, T3...> f8(T2 &&a0, T3 &&...a1) {
  return std::bind<T1, T2, T3...>(std::forward<T2>(a0),
                                  std::forward<T3>(a1)...);
}
