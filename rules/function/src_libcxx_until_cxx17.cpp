#include "../common/init.h"
#include "../common/traits.h"
#include <cstddef>
#include <cxxabi.h>
#include <functional>
#include <memory>
#include <utility>

template <typename T1, typename... T2, typename T3>
std::function<T1(T2...)> f24(std::allocator_arg_t a0, const T3 &a1) {
  return std::function<T1(T2...)>(std::move(a0), a1);
}

template <typename T1, typename... T2, typename T3>
std::function<T1(T2...)> f25(std::allocator_arg_t a0, const T3 &a1,
                             std::nullptr_t a2) {
  return std::function<T1(T2...)>(std::move(a0), a1, a2);
}

template <typename T1, typename... T2, typename T3>
std::function<T1(T2...)> f26(std::allocator_arg_t a0, const T3 &a1,
                             const std::function<T1(T2...)> &a2) {
  return std::function<T1(T2...)>(std::move(a0), a1, a2);
}

template <typename T1, typename... T2, typename T3>
std::function<T1(T2...)> f27(std::allocator_arg_t a0, const T3 &a1,
                             std::function<T1(T2...)> &&a2) {
  return std::function<T1(T2...)>(std::move(a0), a1, std::move(a2));
}

template <typename T1, typename... T2, typename T3, typename T4>
void f28(std::function<T1(T2...)> &a0, T3 &&a1, const T4 &a2) {
  return a0.template assign<T3, T4>(std::forward<T3>(a1), a2);
}
