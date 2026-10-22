#include "../common/init.h"
#include "../common/traits.h"
#include <functional>
#include <string>
#include <utility>

using t2 = std::greater<void>;

using t3 = std::greater<>::is_transparent;

std::greater<void> f2() { return std::greater<void>(); }

std::greater<void> f3(const std::greater<void> &a0) {
  return std::greater<void>(a0);
}

std::greater<void> f4(std::greater<void> &&a0) {
  return std::greater<void>(std::move(a0));
}

std::greater<void> &f5(std::greater<void> &a0, const std::greater<void> &a1) {
  return a0.operator=(a1);
}

std::greater<void> &f6(std::greater<void> &a0, std::greater<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
auto f7(const std::greater<void> &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}
