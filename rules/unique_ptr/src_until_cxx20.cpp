#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <condition_variable>
#include <utility>

template <typename T1, typename T2>
bool f55(const std::unique_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f56(std::nullptr_t a0, const std::unique_ptr<T1, T2> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f57(const std::unique_ptr<T1, T2> &a0, const std::unique_ptr<T3, T4> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f58(std::nullptr_t a0, const std::unique_ptr<T1, T2> &a1) {
  return a0 == a1;
}
