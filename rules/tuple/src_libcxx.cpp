#include <array>
#include <tuple>
#include <utility>

template <
    typename... T1,
    std::__enable_if_t<std::__all<std::__is_swappable_v<T1>...>::value, int>
        T2 = 0>
void f142(std::tuple<T1...> &a0, std::tuple<T1...> &a1) {
  return std::swap<T1...>(a0, a1);
}

template <typename T1> std::tuple<> f143(std::array<T1, 0> a0) {
  return std::tuple<>(std::move(a0));
}

template <typename T1, typename T2>
std::tuple<> f144(std::allocator_arg_t a0, const T1 &a1, std::array<T2, 0> a2) {
  return std::tuple<>(std::move(a0), a1, std::move(a2));
}

template <typename... T1,
          std::__enable_if_t<
              std::_And<std::is_default_constructible<T1>...>::value, int>
              T4 = 0>
std::tuple<T1...> f145() {
  return std::tuple<T1...>();
}

template <typename... T1, typename T2,
          std::__enable_if_t<
              std::_And<std::is_default_constructible<T1>...>::value, int>
              T5 = 0>
std::tuple<T1...> f146(std::allocator_arg_t a0, const T2 &a1) {
  return std::tuple<T1...>(std::move(a0), a1);
}

template <
    typename... T1,
    std::__enable_if_t<std::_And<std::_BoolConstant<sizeof...(T1) >= 1>,
                                 std::is_copy_constructible<T1>...>::value,
                       int>
        T3 = 0>
std::tuple<T1...> f147(const T1 &...a0) {
  return std::tuple<T1...>(a0...);
}

template <
    typename... T1, typename T2,
    std::__enable_if_t<std::_And<std::_BoolConstant<sizeof...(T1) >= 1>,
                                 std::is_copy_constructible<T1>...>::value,
                       int>
        T4 = 0>
std::tuple<T1...> f148(std::allocator_arg_t a0, const T2 &a1, const T1 &...a2) {
  return std::tuple<T1...>(std::move(a0), a1, a2...);
}

template <typename... T1, typename... T2,
          std::__enable_if_t<
              std::_And<std::_BoolConstant<sizeof...(T2) == sizeof...(T1)>,
                        typename std::tuple<T1...>::template _EnableUTypesCtor<
                            T2...>>::value,
              int>
              T3 = 0>
std::tuple<T1...> f149(T2 &&...a0) {
  return std::tuple<T1...>(std::forward<T2>(a0)...);
}

template <typename... T1, typename T2, typename... T3,
          std::__enable_if_t<
              std::_And<std::_BoolConstant<sizeof...(T3) == sizeof...(T1)>,
                        typename std::tuple<T1...>::template _EnableUTypesCtor<
                            T3...>>::value,
              int>
              T4 = 0>
std::tuple<T1...> f150(std::allocator_arg_t a0, const T2 &a1, T3 &&...a2) {
  return std::tuple<T1...>(std::move(a0), a1, std::forward<T3>(a2)...);
}

template <
    typename... T1, typename T2,
    std::__enable_if_t<std::_And<std::is_copy_constructible<T1>...>::value, int>
        T4 = 0>
std::tuple<T1...> f151(std::allocator_arg_t a0, const T2 &a1,
                       const std::tuple<T1...> &a2) {
  return std::tuple<T1...>(std::move(a0), a1, a2);
}

template <
    typename... T1, typename T2,
    std::__enable_if_t<std::_And<std::is_move_constructible<T1>...>::value, int>
        T4 = 0>
std::tuple<T1...> f152(std::allocator_arg_t a0, const T2 &a1,
                       std::tuple<T1...> &&a2) {
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}

template <
    typename... T1, typename... T2,
    std::__enable_if_t<std::_And<typename std::tuple<T1...>::
                                     template _EnableCtorFromUTypesTuple<
                                         const std::tuple<T2...> &>>::value,
                       int>
        T3 = 0>
std::tuple<T1...> f153(const std::tuple<T2...> &a0) {
  return std::tuple<T1...>(a0);
}

template <typename... T1, typename... T2,
          std::__enable_if_t<std::_And<typename std::tuple<T1...>::
                                           template _EnableCtorFromUTypesTuple<
                                               std::tuple<T2...> &&>>::value,
                             int>
              T3 = 0>
std::tuple<T1...> f154(std::tuple<T2...> &&a0) {
  return std::tuple<T1...>(std::move(a0));
}

template <typename... T1, typename T2, typename... T3,
          std::__enable_if_t<std::_And<typename std::tuple<T1...>::
                                           template _EnableCtorFromUTypesTuple<
                                               std::tuple<T3...> &&>>::value,
                             int>
              T4 = 0>
std::tuple<T1...> f155(std::allocator_arg_t a0, const T2 &a1,
                       std::tuple<T3...> &&a2) {
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}

template <
    typename... T1, typename T2, typename T3,
    std::__enable_if_t<
        std::_And<typename std::tuple<T1...>::template _EnableCtorFromPair<
            const std::pair<T2, T3> &>>::value,
        int>
        T5 = 0>
std::tuple<T1...> f156(const std::pair<T2, T3> &a0) {
  return std::tuple<T1...>(a0);
}

template <
    typename... T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<
        std::_And<typename std::tuple<T1...>::template _EnableCtorFromPair<
            const std::pair<T3, T4> &>>::value,
        int>
        T6 = 0>
std::tuple<T1...> f157(std::allocator_arg_t a0, const T2 &a1,
                       const std::pair<T3, T4> &a2) {
  return std::tuple<T1...>(std::move(a0), a1, a2);
}

template <
    typename... T1, typename T2, typename T3,
    std::__enable_if_t<
        std::_And<typename std::tuple<T1...>::template _EnableCtorFromPair<
            std::pair<T2, T3> &&>>::value,
        int>
        T5 = 0>
std::tuple<T1...> f158(std::pair<T2, T3> &&a0) {
  return std::tuple<T1...>(std::move(a0));
}

template <
    typename... T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<
        std::_And<typename std::tuple<T1...>::template _EnableCtorFromPair<
            std::pair<T3, T4> &&>>::value,
        int>
        T6 = 0>
std::tuple<T1...> f159(std::allocator_arg_t a0, const T2 &a1,
                       std::pair<T3, T4> &&a2) {
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}

template <typename... T1, typename... T2,
          std::__enable_if_t<
              std::_And<std::_BoolConstant<sizeof...(T1) == sizeof...(T2)>,
                        std::is_assignable<T1 &, const T2 &>...>::value,
              int>
              T3 = 0>
std::tuple<T1...> &f160(std::tuple<T1...> &a0, const std::tuple<T2...> &a1) {
  return a0.template operator= <T2...>(a1);
}

template <typename... T1, typename... T2,
          std::__enable_if_t<
              std::_And<std::_BoolConstant<sizeof...(T1) == sizeof...(T2)>,
                        std::is_assignable<T1 &, T2>...>::value,
              int>
              T3 = 0>
std::tuple<T1...> &f161(std::tuple<T1...> &a0, std::tuple<T2...> &&a1) {
  return a0.template operator= <T2...>(std::move(a1));
}

template <typename... T1, typename T2, typename T3,
          std::__enable_if_t<std::tuple<T1...>::template _EnableAssignFromPair<
                                 false, const std::pair<T2, T3> &>::value,
                             int>
              T4 = 0>
std::tuple<T1...> &f162(std::tuple<T1...> &a0, const std::pair<T2, T3> &a1) {
  return a0.template operator= <T2, T3>(a1);
}

template <typename... T1, typename T2, typename T3,
          std::__enable_if_t<std::tuple<T1...>::template _EnableAssignFromPair<
                                 false, std::pair<T2, T3> &&>::value,
                             int>
              T4 = 0>
std::tuple<T1...> &f163(std::tuple<T1...> &a0, std::pair<T2, T3> &&a1) {
  return a0.template operator= <T2, T3>(std::move(a1));
}

template <typename... T1, typename T2, std::size_t T3,
          std::__enable_if_t<
              std::_And<std::_BoolConstant<T3 == sizeof...(T1)>,
                        std::is_assignable<T1 &, const T2 &>...>::value,
              int>
              T4 = 0>
std::tuple<T1...> &f164(std::tuple<T1...> &a0, const std::array<T2, T3> &a1) {
  return a0.template operator= <T2, T3>(a1);
}

template <typename... T1, typename T2, std::size_t T3, typename T4 = void,
          std::__enable_if_t<std::_And<std::_BoolConstant<T3 == sizeof...(T1)>,
                                       std::is_assignable<T1 &, T2>...>::value,
                             int>
              T5 = 0>
std::tuple<T1...> &f165(std::tuple<T1...> &a0, std::array<T2, T3> &&a1) {
  return a0.template operator= <T2, T3>(std::move(a1));
}

std::tuple<> f166() { return std::tuple_cat(); }

template <typename T1, typename... T2>
auto f167(T1 &&a0, T2 &&...a1)
    -> decltype(std::tuple_cat<T1, T2...>(std::forward<T1>(a0),
                                          std::forward<T2>(a1)...)) {
  return std::tuple_cat<T1, T2...>(std::forward<T1>(a0),
                                   std::forward<T2>(a1)...);
}
