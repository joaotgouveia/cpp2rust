#include "../common/init.h"
#include "../common/traits.h"
#include <string>
#include <utility>

using t2 = std::logical_not<void>;

using t3 = std::logical_not<>::is_transparent;

std::logical_not<void> f2() { return std::logical_not<void>(); }

std::logical_not<void> f3(const std::logical_not<void> &a0) {
  return std::logical_not<void>(a0);
}

std::logical_not<void> f4(std::logical_not<void> &&a0) {
  return std::logical_not<void>(std::move(a0));
}

std::logical_not<void> &f5(std::logical_not<void> &a0,
                           const std::logical_not<void> &a1) {
  return a0.operator=(a1);
}

std::logical_not<void> &f6(std::logical_not<void> &a0,
                           std::logical_not<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> auto f7(const std::logical_not<void> &a0, T1 &&a1) {
  return a0.template operator()<T1>(std::forward<T1>(a1));
}
