#include <algorithm>
#include <functional>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename... T2>
bool f7(const std::function<T1(T2...)> &a0, std::nullptr_t a1) {
  return a0 != a1;
}

template <typename T1, typename... T2>
bool f8(std::nullptr_t a0, const std::function<T1(T2...)> &a1) {
  return a0 != a1;
}

template <typename T1, typename... T2>
bool f9(std::nullptr_t a0, const std::function<T1(T2...)> &a1) {
  return a0 == a1;
}
