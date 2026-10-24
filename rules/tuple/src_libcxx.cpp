#include <array>
#include <tuple>
#include <utility>

template <
    typename... T1,
    std::__enable_if_t<std::__all<std::__is_swappable_v<T1>...>::value, int>
        T2 = 0>
void f120(std::tuple<T1...> &a0, std::tuple<T1...> &a1) {
  return std::swap<T1...>(a0, a1);
}

template <typename T1> std::tuple<> f121(std::array<T1, 0> a0) {
  return std::tuple<>(a0);
}

template <typename T1, typename T2>
std::tuple<> f122(std::allocator_arg_t a0, const T1 &a1, std::array<T2, 0> a2) {
  return std::tuple<>(std::move(a0), a1, a2);
}

template <typename... T1, typename... T2,
          std::__enable_if_t<
              std::_And<std::_BoolConstant<sizeof...(T2) == sizeof...(T1)>,
                        typename std::tuple<T1...>::template _EnableUTypesCtor<
                            T2...>>::value,
              int>
              T3 = 0>
std::tuple<T1...> f123(T2 &&...a0) {
  return std::tuple<T1...>(std::forward<T2>(a0)...);
}

template <typename... T1, typename T2, typename... T3,
          std::__enable_if_t<
              std::_And<std::_BoolConstant<sizeof...(T3) == sizeof...(T1)>,
                        typename std::tuple<T1...>::template _EnableUTypesCtor<
                            T3...>>::value,
              int>
              T4 = 0>
std::tuple<T1...> f124(std::allocator_arg_t a0, const T2 &a1, T3 &&...a2) {
  return std::tuple<T1...>(std::move(a0), a1, std::forward<T3>(a2)...);
}

template <
    typename... T1, typename... T2,
    std::__enable_if_t<std::_And<typename std::tuple<T1...>::
                                     template _EnableCtorFromUTypesTuple<
                                         const std::tuple<T2...> &>>::value,
                       int>
        T3 = 0>
std::tuple<T1...> f125(const std::tuple<T2...> &a0) {
  return std::tuple<T1...>(a0);
}

template <typename... T1, typename... T2,
          std::__enable_if_t<std::_And<typename std::tuple<T1...>::
                                           template _EnableCtorFromUTypesTuple<
                                               std::tuple<T2...> &&>>::value,
                             int>
              T3 = 0>
std::tuple<T1...> f126(std::tuple<T2...> &&a0) {
  return std::tuple<T1...>(std::move(a0));
}

template <typename... T1, typename T2, typename... T3,
          std::__enable_if_t<std::_And<typename std::tuple<T1...>::
                                           template _EnableCtorFromUTypesTuple<
                                               std::tuple<T3...> &&>>::value,
                             int>
              T4 = 0>
std::tuple<T1...> f127(std::allocator_arg_t a0, const T2 &a1,
                       std::tuple<T3...> &&a2) {
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}

template <typename... T1, typename... T2,
          std::__enable_if_t<
              std::_And<std::_BoolConstant<sizeof...(T1) == sizeof...(T2)>,
                        std::is_assignable<T1 &, const T2 &>...>::value,
              int>
              T3 = 0>
std::tuple<T1...> &f128(std::tuple<T1...> &a0, const std::tuple<T2...> &a1) {
  return a0.template operator= <T2...>(a1);
}

template <typename... T1, typename... T2,
          std::__enable_if_t<
              std::_And<std::_BoolConstant<sizeof...(T1) == sizeof...(T2)>,
                        std::is_assignable<T1 &, T2>...>::value,
              int>
              T3 = 0>
std::tuple<T1...> &f129(std::tuple<T1...> &a0, std::tuple<T2...> &&a1) {
  return a0.template operator= <T2...>(std::move(a1));
}

template <typename... T1, typename T2, typename T3,
          std::__enable_if_t<std::tuple<T1...>::template _EnableAssignFromPair<
                                 false, const std::pair<T2, T3> &>::value,
                             int>
              T4 = 0>
std::tuple<T1...> &f130(std::tuple<T1...> &a0, const std::pair<T2, T3> &a1) {
  return a0.template operator= <T2, T3>(a1);
}

template <typename... T1, typename T2, typename T3,
          std::__enable_if_t<std::tuple<T1...>::template _EnableAssignFromPair<
                                 false, std::pair<T2, T3> &&>::value,
                             int>
              T4 = 0>
std::tuple<T1...> &f131(std::tuple<T1...> &a0, std::pair<T2, T3> &&a1) {
  return a0.template operator= <T2, T3>(std::move(a1));
}

template <typename... T1, typename T2, std::size_t T3,
          std::__enable_if_t<
              std::_And<std::_BoolConstant<T3 == sizeof...(T1)>,
                        std::is_assignable<T1 &, const T2 &>...>::value,
              int>
              T4 = 0>
std::tuple<T1...> &f132(std::tuple<T1...> &a0, const std::array<T2, T3> &a1) {
  return a0.template operator= <T2, T3>(a1);
}

template <typename... T1, typename T2, std::size_t T3, typename T4 = void,
          std::__enable_if_t<std::_And<std::_BoolConstant<T3 == sizeof...(T1)>,
                                       std::is_assignable<T1 &, T2>...>::value,
                             int>
              T5 = 0>
std::tuple<T1...> &f133(std::tuple<T1...> &a0, std::array<T2, T3> &&a1) {
  return a0.template operator= <T2, T3>(std::move(a1));
}

std::tuple<> f134() { return std::tuple_cat(); }

template <typename T1, typename... T2>
auto f135(T1 &&a0, T2 &&...a1)
    -> decltype(std::tuple_cat<T1, T2...>(std::forward<T1>(a0),
                                          std::forward<T2>(a1)...)) {
  return std::tuple_cat<T1, T2...>(std::forward<T1>(a0),
                                   std::forward<T2>(a1)...);
}
