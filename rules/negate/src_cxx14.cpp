#include <functional>
#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t2 = std::negate<void>;

using t3 = std::negate<>::is_transparent;

std::negate<void> f2() { return std::negate<void>(); }

std::negate<void> f3(const std::negate<void> &a0) {
  return std::negate<void>(a0);
}

std::negate<void> f4(std::negate<void> &&a0) {
  return std::negate<void>(std::move(a0));
}

std::negate<void> &f5(std::negate<void> &a0, const std::negate<void> &a1) {
  return a0.operator=(a1);
}

std::negate<void> &f6(std::negate<void> &a0, std::negate<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> auto f7(const std::negate<void> &a0, T1 &&a1) {
  return a0.template operator()<T1>(std::forward<T1>(a1));
}
