#include "../common/init.h"
#include "../common/traits.h"
#include <functional>
#include <string>
#include <utility>

using t2 = std::bit_xor<void>;

using t3 = std::bit_xor<>::is_transparent;

std::bit_xor<void> f2() { return std::bit_xor<void>(); }

std::bit_xor<void> f3(const std::bit_xor<void> &a0) {
  return std::bit_xor<void>(a0);
}

std::bit_xor<void> f4(std::bit_xor<void> &&a0) {
  return std::bit_xor<void>(std::move(a0));
}

std::bit_xor<void> &f5(std::bit_xor<void> &a0, const std::bit_xor<void> &a1) {
  return a0.operator=(a1);
}

std::bit_xor<void> &f6(std::bit_xor<void> &a0, std::bit_xor<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
auto f7(const std::bit_xor<void> &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}
