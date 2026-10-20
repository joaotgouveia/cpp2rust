#include "../common/init.h"
#include "../common/traits.h"
#include <string>
#include <utility>

using t2 = std::bit_not<void>;

using t3 = std::bit_not<>::is_transparent;

std::bit_not<void> f2() { return std::bit_not<void>(); }

std::bit_not<void> f3(const std::bit_not<void> &a0) {
  return std::bit_not<void>(a0);
}

std::bit_not<void> f4(std::bit_not<void> &&a0) {
  return std::bit_not<void>(std::move(a0));
}

std::bit_not<void> &f5(std::bit_not<void> &a0, const std::bit_not<void> &a1) {
  return a0.operator=(a1);
}

std::bit_not<void> &f6(std::bit_not<void> &a0, std::bit_not<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> auto f7(const std::bit_not<void> &a0, T1 &&a1) {
  return a0.template operator()<T1>(std::forward<T1>(a1));
}
