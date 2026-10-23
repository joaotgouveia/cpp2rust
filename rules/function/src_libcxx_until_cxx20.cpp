#include <cstddef>
#include <cxxabi.h>
#include <functional>
#include <memory>
#include <utility>

template <typename T1, typename... T2>
bool f15(const std::function<T1(T2...)> &a0, std::nullptr_t a1) {
  return std::operator!= <T1, T2...>(a0, a1);
}

template <typename T1, typename... T2>
bool f16(std::nullptr_t a0, const std::function<T1(T2...)> &a1) {
  return std::operator!= <T1, T2...>(a0, a1);
}

template <typename T1, typename... T2>
bool f17(std::nullptr_t a0, const std::function<T1(T2...)> &a1) {
  return std::operator== <T1, T2...>(a0, a1);
}
