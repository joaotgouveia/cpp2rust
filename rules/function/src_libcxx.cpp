#include <cstddef>
#include <cxxabi.h>
#include <functional>
#include <memory>
#include <utility>

template <typename T1, typename... T2>
bool f19(const std::function<T1(T2...)> &a0, std::nullptr_t a1) {
  return std::operator== <T1, T2...>(a0, a1);
}

template <typename T1, typename... T2>
void f20(std::function<T1(T2...)> &a0, std::function<T1(T2...)> &a1) {
  return std::swap<T1, T2...>(a0, a1);
}

using t3 = std::bad_function_call;

std::bad_function_call f21() { return std::bad_function_call(); }

std::bad_function_call f22(const std::bad_function_call &a0) {
  return std::bad_function_call(a0);
}

std::bad_function_call &f23(std::bad_function_call &a0,
                            const std::bad_function_call &a1) {
  return a0.operator=(a1);
}

const char *f24(const std::bad_function_call &a0) { return a0.what(); }

template <
    typename T1, typename... T2, typename T3,
    typename T4 = std::__enable_if_t<std::_And<
        std::_IsNotSame<std::__remove_cvref_t<T3>, std::function<T1(T2...)>>,
        std::__is_invocable_r<T1, T3 &, T2...>>::value>>
std::function<T1(T2...)> f25(T3 a0) {
  return std::function<T1(T2...)>(a0);
}

template <typename T1, typename... T2, typename T3,
          typename T4 = std::__enable_if_t<std::_And<
              std::_IsNotSame<std::__remove_cvref_t<std::__decay_t<T3>>,
                              std::function<T1(T2...)>>,
              std::__is_invocable_r<T1, std::__decay_t<T3> &, T2...>>::value>>
std::function<T1(T2...)> &f26(std::function<T1(T2...)> &a0, T3 &&a1) {
  return a0.template operator= <T3>(std::forward<T3>(a1));
}
