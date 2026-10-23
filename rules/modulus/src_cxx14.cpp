#include <functional>
#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t2 = std::modulus<void>;

using t3 = std::modulus<>::is_transparent;

std::modulus<void> f2() { return std::modulus<void>(); }

std::modulus<void> f3(const std::modulus<void> &a0) {
  return std::modulus<void>(a0);
}

std::modulus<void> f4(std::modulus<void> &&a0) {
  return std::modulus<void>(std::move(a0));
}

std::modulus<void> &f5(std::modulus<void> &a0, const std::modulus<void> &a1) {
  return a0.operator=(a1);
}

std::modulus<void> &f6(std::modulus<void> &a0, std::modulus<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
auto f7(const std::modulus<void> &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}
