#include <functional>
#include <type_traits>
#include <utility>

template <typename T1> using t4 = std::_Not_fn<T1>;

template <typename T1> std::_Not_fn<T1> f4(const std::_Not_fn<T1> &a0) {
  return std::_Not_fn<T1>(a0);
}

template <typename T1> std::_Not_fn<T1> f5(std::_Not_fn<T1> &&a0) {
  return std::_Not_fn<T1>(std::move(a0));
}

template <typename T1, typename T2> std::_Not_fn<T1> f6(T2 &&a0, int a1) {
  return std::_Not_fn<T1>(std::forward<T2>(a0), a1);
}

template <
    typename T1, typename... T2,
    typename T3 = std::enable_if_t<std::__is_invocable<T1 &, T2...>::value>>
auto f7(std::_Not_fn<T1> &a0, T2 &&...a1) {
  return a0.template operator()<T2...>(std::forward<T2>(a1)...);
}

template <typename T1, typename... T2,
          typename T3 =
              std::enable_if_t<std::__is_invocable<const T1 &, T2...>::value>>
auto f8(const std::_Not_fn<T1> &a0, T2 &&...a1) {
  return a0.template operator()<T2...>(std::forward<T2>(a1)...);
}

template <
    typename T1, typename... T2,
    typename T3 = std::enable_if_t<std::__is_invocable<T1 &&, T2...>::value>>
auto f9(std::_Not_fn<T1> &&a0, T2 &&...a1) {
  return std::move(a0).template operator()<T2...>(std::forward<T2>(a1)...);
}

template <typename T1, typename... T2,
          typename T3 =
              std::enable_if_t<std::__is_invocable<const T1 &&, T2...>::value>>
auto f10(const std::_Not_fn<T1> &&a0, T2 &&...a1) {
  return std::move(a0).template operator()<T2...>(std::forward<T2>(a1)...);
}

template <typename T1, typename... T2>
std::invoke_result_t<T1, T2...> f11(T1 &&a0, T2 &&...a1) {
  return std::invoke<T1, T2...>(std::forward<T1>(a0), std::forward<T2>(a1)...);
}

template <typename T1> auto f12(T1 &&a0) {
  return std::not_fn<T1>(std::forward<T1>(a0));
}
