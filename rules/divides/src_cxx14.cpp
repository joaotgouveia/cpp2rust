#include "../common/init.h"
#include "../common/traits.h"
#include <functional>
#include <string>
#include <utility>

using t2 = std::divides<void>;

using t3 = std::divides<>::is_transparent;

std::divides<void> f2() { return std::divides<void>(); }

std::divides<void> f3(const std::divides<void> &a0) {
  return std::divides<void>(a0);
}

std::divides<void> f4(std::divides<void> &&a0) {
  return std::divides<void>(std::move(a0));
}

std::divides<void> &f5(std::divides<void> &a0, const std::divides<void> &a1) {
  return a0.operator=(a1);
}

std::divides<void> &f6(std::divides<void> &a0, std::divides<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
auto f7(const std::divides<void> &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}
