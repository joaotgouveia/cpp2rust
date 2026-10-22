#include "../common/init.h"
#include "../common/traits.h"
#include <cstddef>
#include <cxxabi.h>
#include <functional>
#include <memory>
#include <utility>

template <typename T1, typename... T2>
bool f17(const std::function<T1(T2...)> &a0, std::nullptr_t a1) {
  return a0 != a1;
}

template <typename T1, typename... T2>
bool f18(std::nullptr_t a0, const std::function<T1(T2...)> &a1) {
  return a0 != a1;
}

template <typename T1, typename... T2>
bool f19(std::nullptr_t a0, const std::function<T1(T2...)> &a1) {
  return a0 == a1;
}
