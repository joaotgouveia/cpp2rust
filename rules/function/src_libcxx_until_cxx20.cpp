#include <cstddef>
#include <cxxabi.h>
#include <functional>
#include <memory>
#include <utility>

template <typename T1, typename... T2>
bool f19(const std::function<T1(T2...)> &a0, std::nullptr_t a1) {
  return std::operator!= <T1, T2...>(a0, a1);
}

template <typename T1, typename... T2>
bool f20(std::nullptr_t a0, const std::function<T1(T2...)> &a1) {
  return std::operator!= <T1, T2...>(a0, a1);
}

template <typename T1, typename... T2>
bool f21(std::nullptr_t a0, const std::function<T1(T2...)> &a1) {
  return std::operator== <T1, T2...>(a0, a1);
}

template <
    typename T1, typename... T2, typename T3,
    typename T4 = std::__enable_if_t<std::_And<
        std::_IsNotSame<std::__remove_cvref_t<T3>, std::function<T1(T2...)>>,
        std::__is_invocable_r<T1, T3 &, T2...>>::value>,
    typename std::enable_if<
        (!(std::is_same<T3, std::nullptr_t>::value) &&
         !(std::is_same<T3, const std::function<T1(T2...)> &>::value) &&
         !(std::is_same<T3, std::function<T1(T2...)> &&>::value)),
        int>::type T5 = 0>
std::function<T1(T2...)> f22(T3 a0) {
  return std::function<T1(T2...)>(std::move(a0));
}

template <typename T1, typename... T2, typename T3,
          typename T4 = std::__enable_if_t<std::_And<
              std::_IsNotSame<std::__remove_cvref_t<std::__decay_t<T3>>,
                              std::function<T1(T2...)>>,
              std::__is_invocable_r<T1, std::__decay_t<T3> &, T2...>>::value>,
          typename std::enable_if<
              (!(std::is_same<T3, const std::function<T1(T2...)> &>::value) &&
               !(std::is_same<T3, std::function<T1(T2...)>>::value) &&
               !(std::is_same<T3, std::nullptr_t>::value)),
              int>::type T5 = 0>
std::function<T1(T2...)> &f23(std::function<T1(T2...)> &a0, T3 &&a1) {
  return a0.template operator= <T3>(std::forward<T3>(a1));
}
