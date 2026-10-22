#include "../common/init.h"
#include "../common/traits.h"
#include <functional>
#include <string>
#include <utility>

using t2 = std::greater_equal<void>;

using t3 = std::greater_equal<>::is_transparent;

std::greater_equal<void> f2() { return std::greater_equal<void>(); }

std::greater_equal<void> f3(const std::greater_equal<void> &a0) {
  return std::greater_equal<void>(a0);
}

std::greater_equal<void> f4(std::greater_equal<void> &&a0) {
  return std::greater_equal<void>(std::move(a0));
}

std::greater_equal<void> &f5(std::greater_equal<void> &a0,
                             const std::greater_equal<void> &a1) {
  return a0.operator=(a1);
}

std::greater_equal<void> &f6(std::greater_equal<void> &a0,
                             std::greater_equal<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
auto f7(const std::greater_equal<void> &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}
