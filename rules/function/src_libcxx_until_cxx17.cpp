#include <cstddef>
#include <cxxabi.h>
#include <functional>
#include <memory>
#include <utility>

template <typename T1, typename... T2, typename T3>
std::function<T1(T2...)> f27(std::allocator_arg_t a0, const T3 &a1) {
  return std::function<T1(T2...)>(std::move(a0), a1);
}

template <typename T1, typename... T2, typename T3>
std::function<T1(T2...)> f28(std::allocator_arg_t a0, const T3 &a1,
                             std::nullptr_t a2) {
  return std::function<T1(T2...)>(std::move(a0), a1, a2);
}

template <typename T1, typename... T2, typename T3>
std::function<T1(T2...)> f29(std::allocator_arg_t a0, const T3 &a1,
                             const std::function<T1(T2...)> &a2) {
  return std::function<T1(T2...)>(std::move(a0), a1, a2);
}

template <typename T1, typename... T2, typename T3>
std::function<T1(T2...)> f30(std::allocator_arg_t a0, const T3 &a1,
                             std::function<T1(T2...)> &&a2) {
  return std::function<T1(T2...)>(std::move(a0), a1, std::move(a2));
}

template <
    typename T1, typename... T2, typename T3, typename T4,
    typename T5 = std::__enable_if_t<std::_And<
        std::_IsNotSame<std::__remove_cvref_t<T3>, std::function<T1(T2...)>>,
        std::__is_invocable_r<T1, T3 &, T2...>>::value>>
std::function<T1(T2...)> f31(std::allocator_arg_t a0, const T4 &a1, T3 a2) {
  return std::function<T1(T2...)>(std::move(a0), a1, a2);
}

template <typename T1, typename... T2, typename T3, typename T4>
void f32(std::function<T1(T2...)> &a0, T3 &&a1, const T4 &a2) {
  return a0.template assign<T3, T4>(std::forward<T3>(a1), a2);
}
