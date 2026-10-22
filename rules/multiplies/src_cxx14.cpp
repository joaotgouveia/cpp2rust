#include "../common/init.h"
#include "../common/traits.h"
#include <functional>
#include <string>
#include <utility>

using t2 = std::multiplies<void>;

using t3 = std::multiplies<>::is_transparent;

std::multiplies<void> f2() { return std::multiplies<void>(); }

std::multiplies<void> f3(const std::multiplies<void> &a0) {
  return std::multiplies<void>(a0);
}

std::multiplies<void> f4(std::multiplies<void> &&a0) {
  return std::multiplies<void>(std::move(a0));
}

std::multiplies<void> &f5(std::multiplies<void> &a0,
                          const std::multiplies<void> &a1) {
  return a0.operator=(a1);
}

std::multiplies<void> &f6(std::multiplies<void> &a0,
                          std::multiplies<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
auto f7(const std::multiplies<void> &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}
