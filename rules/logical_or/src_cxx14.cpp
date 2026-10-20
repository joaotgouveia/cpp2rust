#include "../common/init.h"
#include "../common/traits.h"
#include <string>
#include <utility>

using t2 = std::logical_or<void>;

using t3 = std::logical_or<>::is_transparent;

std::logical_or<void> f2() { return std::logical_or<void>(); }

std::logical_or<void> f3(const std::logical_or<void> &a0) {
  return std::logical_or<void>(a0);
}

std::logical_or<void> f4(std::logical_or<void> &&a0) {
  return std::logical_or<void>(std::move(a0));
}

std::logical_or<void> &f5(std::logical_or<void> &a0,
                          const std::logical_or<void> &a1) {
  return a0.operator=(a1);
}

std::logical_or<void> &f6(std::logical_or<void> &a0,
                          std::logical_or<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
auto f7(const std::logical_or<void> &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}
